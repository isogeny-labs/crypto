load("//bazel:envoy_build_system.bzl", "envoy_cc_extension", "envoy_cc_library", "envoy_extension_package")
load("@com_google_protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@com_google_protobuf//bazel:proto_library.bzl", "proto_library")

licenses(["notice"])
envoy_extension_package()

proto_library(
    name = "config_proto",
    srcs = ["config.proto"],
    deps = ["@com_google_protobuf//:struct_proto"],
)

cc_proto_library(
    name = "config_cc_proto",
    deps = [":config_proto"],
)

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
        ":config_cc_proto",
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
