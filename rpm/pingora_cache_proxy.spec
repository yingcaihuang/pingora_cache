Name:           pingora_cache_proxy
Version:        0.1.0
Release:        1%{?dist}
Summary:        High performance CDN cache proxy based on Pingora

License:        Proprietary
URL:            http://www.yingcai.com
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  systemd-rpm-macros
# 如果是在 CI 环境中构建，可能需要 openssl-devel
BuildRequires:  openssl-devel

Requires:       systemd
Requires:       openssl

%description
A high performance CDN cache proxy built with Cloudflare's Pingora framework.
Features include:
- HTTP/HTTPS caching
- Load balancing
- Custom upstream logic

%prep
%setup -q

%build
# 确保使用 release 模式构建
cargo build --release

%install
rm -rf $RPM_BUILD_ROOT
# 安装二进制文件到 /usr/bin
install -D -m 0755 target/release/pingora_cache_proxy $RPM_BUILD_ROOT%{_bindir}/pingora_cache_proxy
# 安装 systemd 服务文件
install -D -m 0644 pingora.service $RPM_BUILD_ROOT%{_unitdir}/pingora_cache_proxy.service

%post
%systemd_post pingora_cache_proxy.service

%preun
%systemd_preun pingora_cache_proxy.service

%postun
%systemd_postun_with_restart pingora_cache_proxy.service

%files
%defattr(-,root,root,-)
%{_bindir}/pingora_cache_proxy
%{_unitdir}/pingora_cache_proxy.service

%changelog
* Wed Nov 26 2025 Developer <dev@yingcai.com> - 0.1.0-1
- Initial package release
