load("//bazel:envoy_build_system.bzl", "envoy_cc_extension", "envoy_cc_library", "envoy_extension_package")

licenses(["notice"])
envoy_extension_package()

envoy_cc_library(
    name = "core",
    srcs = ["src/crypto.cc", "src/mailbox.cc", "src/aws_pci.cc", "src/pq_codec.cc", "src/pq_loader.cc", "src/boringssl_private_key.cc", "src/boringssl_records.cc"],
    hdrs = glob(["include/fpga/*.h"]),
    strip_include_prefix = "include",
    external_deps = ["ssl"],
    linkopts = [
        "-ldl",
        "-Wl,--wrap=EVP_AEAD_CTX_init_with_direction",
        "-Wl,--wrap=EVP_AEAD_CTX_cleanup",
        "-Wl,--wrap=EVP_AEAD_CTX_seal_scatter",
        "-Wl,--wrap=EVP_AEAD_CTX_open",
    ],
)

envoy_cc_extension(
    name = "config",
    srcs = ["envoy/extension.cc", "envoy/pq_filter.cc"],
    deps = [
        ":core",
        "//envoy/event:dispatcher_interface",
        "//envoy/http:filter_interface",
        "//envoy/http:header_map_interface",
        "//envoy/registry",
        "//envoy/server:filter_config_interface",
        "//envoy/ssl/private_key:private_key_config_interface",
        "//envoy/ssl/private_key:private_key_interface",
        "//envoy/common:exception_lib",
        "//source/common/protobuf:protobuf",
        "@envoy_api//envoy/extensions/transport_sockets/tls/v3:pkg_cc_proto",
    ],
)
