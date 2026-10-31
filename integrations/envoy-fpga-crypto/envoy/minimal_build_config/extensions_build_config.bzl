# Small, fully linked Envoy profile for this extension's integration tests.
# The default installer/build retains Envoy's normal extension selection.
EXTENSIONS = {
    "envoy.access_loggers.file": "//source/extensions/access_loggers/file:config",
    "envoy.access_loggers.stderr": "//source/extensions/access_loggers/stream:config",
    "envoy.access_loggers.stdout": "//source/extensions/access_loggers/stream:config",
    "envoy.clusters.static": "//source/extensions/clusters/static:static_cluster_lib",
    "envoy.filters.http.fpga_crypto": "//source/extensions/fpga_crypto:config",
    "envoy.filters.http.pq_crypto": "//source/extensions/fpga_crypto:config",
    "envoy.filters.http.router": "//source/extensions/filters/http/router:config",
    "envoy.filters.network.http_connection_manager": "//source/extensions/filters/network/http_connection_manager:config",
    "envoy.network.dns_resolver.cares": "//source/extensions/network/dns_resolver/cares:config",
    "envoy.transport_sockets.raw_buffer": "//source/extensions/transport_sockets/raw_buffer:config",
    "envoy.transport_sockets.tls": "//source/extensions/transport_sockets/tls:config",
    "envoy.upstreams.http.http": "//source/extensions/upstreams/http/http:config",
}

EXTENSION_CONFIG_VISIBILITY = ["//:extension_config", "//:contrib_library", "//:mobile_library"]
EXTENSION_PACKAGE_VISIBILITY = ["//:extension_library", "//:contrib_library", "//:mobile_library"]
CONTRIB_EXTENSION_PACKAGE_VISIBILITY = ["//:contrib_library"]
MOBILE_PACKAGE_VISIBILITY = ["//:mobile_library"]
LEGACY_ALWAYSLINK = 1
