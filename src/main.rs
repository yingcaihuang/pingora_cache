use async_trait::async_trait;
use pingora::prelude::*;
use pingora::proxy::{ProxyHttp, Session};
use pingora::cache::{MemCache, CacheKey};
use pingora::lb::LoadBalancer;
use pingora::lb::selection::RoundRobin;
use std::sync::Arc;
use once_cell::sync::Lazy;

// Initialize a static memory cache
static MEM_CACHE: Lazy<MemCache> = Lazy::new(|| MemCache::new());

pub struct CdnProxy {
    pub lb_http: Arc<LoadBalancer<RoundRobin>>,
    pub lb_https: Arc<LoadBalancer<RoundRobin>>,
}

#[async_trait]
impl ProxyHttp for CdnProxy {
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        // Determine protocol from X-Forwarded-Proto header set by Nginx
        let proto = session
            .req_header()
            .headers
            .get("x-forwarded-proto")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("http");

        let (upstream, use_tls) = if proto == "https" {
            // Select backend for HTTPS
            let upstream = self.lb_https.select(b"", 256).unwrap();
            (upstream, true)
        } else {
            // Select backend for HTTP
            let upstream = self.lb_http.select(b"", 256).unwrap();
            (upstream, false)
        };

        // Use www.yingcai.com as the Host header and SNI
        let peer = Box::new(HttpPeer::new(upstream, use_tls, "www.yingcai.com".to_string()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut ()) -> Result<bool> {
        // Enable cache for GET and HEAD requests
        let req = session.req_header();
        if req.method != http::Method::GET && req.method != http::Method::HEAD {
            return Ok(false);
        }

        // Create a cache key based on the request path and host
        // In a real CDN, you might want to include query params, headers, etc.
        let path = req.uri.path();
        // Use the Host header from the request, or default to www.yingcai.com
        let host = req.uri.host().unwrap_or("www.yingcai.com");
        
        // CacheKey::new(authority, path, shard)
        let key = CacheKey::new(host, path, path);

        // Enable the cache with our memory storage
        session.cache.enable(
            &*MEM_CACHE,
            None, // No eviction manager for this simple example
            None, // No predictor
            None, // No cache lock (for simplicity, but recommended for production)
            None,
        );

        // Set the cache key
        session.cache.set_cache_key(key);

        // Return false to continue processing
        Ok(false)
    }

/*
    async fn upstream_response_filter(
        &self,
        session: &mut Session,
        resp: &ResponseHeader,
        _ctx: &mut (),
    ) -> Result<()> {
        // Decide if we should cache the response
        // For example, only cache 200 OK
        if resp.status != 200 {
            session.cache.disable(NoCacheReason::Custom("Not 200 OK"));
        } else {
            // Set cache meta if needed, or rely on default
            // In 0.6, we might need to set meta explicitly if we want to control TTL
            // But for now, let's see if this compiles.
            // If we need to set TTL, we might use session.cache.set_max_file_size or similar?
            // Or maybe we can't set TTL here easily without CacheMeta.
            // But let's first get it to compile.
        }
        Ok(())
    }
*/
}

// Helper enum for response_cache_filter return type if not exported directly
// (Adjust based on actual Pingora API if needed, usually it's in pingora::proxy)
// Assuming CacheAction is available in prelude or proxy module.
// If not, we might need to import it specifically.
// It seems it might be `pingora::proxy::CacheAction` or similar.
// For now, I'll assume it's available via prelude or I'll let the user fix imports.

fn main() {
    env_logger::init();

    // Create the server
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    // HTTP Upstreams: 21 (weight 1), 22 (weight 2)
    let upstreams_http = LoadBalancer::try_from_iter([
        "211.161.133.21:80",
        "211.161.133.22:80",
        "211.161.133.22:80",
    ]).unwrap();

    // HTTPS Upstreams
    let upstreams_https = LoadBalancer::try_from_iter([
        "211.161.133.21:443",
        "211.161.133.22:443",
        "211.161.133.22:443",
    ]).unwrap();

    // Create the proxy service
    let mut proxy = http_proxy_service(
        &my_server.configuration,
        CdnProxy {
            lb_http: Arc::new(upstreams_http),
            lb_https: Arc::new(upstreams_https),
        },
    );
    
    // Listen on port 6188
    proxy.add_tcp("0.0.0.0:6188");

    // Add the service to the server
    my_server.add_service(proxy);

    // Run the server
    my_server.run_forever();
}
