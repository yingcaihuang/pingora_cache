use async_trait::async_trait;
use pingora::prelude::*;
use pingora::proxy::{ProxyHttp, Session};
use pingora::http::ResponseHeader;
use http::header::HeaderValue;
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
        // Rewrite rule: /a/1.html -> /a/403.html
        {
            let req = session.req_header_mut();
            if req.uri.path() == "/a/1.html" {
                if let Ok(new_uri) = "/a/403.html".parse::<http::Uri>() {
                    req.set_uri(new_uri);
                }
            }
        }

        // Enable cache for GET and HEAD requests
        let req = session.req_header();
        if req.method != http::Method::GET && req.method != http::Method::HEAD {
            return Ok(false);
        }

        // Create a cache key based on the request path and host
        // In a real CDN, you might want to include query params, headers, etc.
        let path = req.uri.path();

        // Force no cache for /noc/nodelete.gif
        if path == "/noc/nodelete.gif" {
            return Ok(false);
        }

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

    fn upstream_response_filter(
        &self,
        session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut (),
    ) -> Result<()> {
        let path = session.req_header().uri.path();
        if path.starts_with("/img/") {
            let _ = upstream_response.insert_header("Cache-Control", "max-age=3600");
        } else if path.starts_with("/css/") {
            let _ = upstream_response.insert_header("Cache-Control", "max-age=21600");
        } else if path.starts_with("/js/") {
            let _ = upstream_response.insert_header("Cache-Control", "max-age=2592000");
        }
        Ok(())
    }

    async fn response_filter(
        &self,
        session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut (),
    ) -> Result<()> {
        if let Some(client_addr) = session.client_addr() {
            if let Some(inet_addr) = client_addr.as_inet() {
                let ip = inet_addr.ip().to_string();
                if let Ok(header_value) = HeaderValue::from_str(&ip) {
                    let _ = upstream_response.insert_header("MYCX", header_value);
                }
            }
        }
        Ok(())
    }
}

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
