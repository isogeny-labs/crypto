# Envoy FPGA crypto extension

C++ prototype targeting **Envoy v1.39.3**, its **BoringSSL 0.20260929.0** pin,
Linux ELF, and AWS F2's userspace PCIe SDK. It implements all three requested
categories:

| Use | Integration | Supported operations |
|---|---|---|
| TLS handshake | Native Envoy `PrivateKeyMethodProvider`, named `fpga` | RSA-PSS/SHA-256, RSA PKCS#1/SHA-256, ECDSA P-256/SHA-256; TLS 1.3 ML-DSA-44/65/87 via ABI 2 |
| TLS record encryption | Opt-in BoringSSL link-time bridge | TLS 1.2/1.3 AES-128-GCM and AES-256-GCM seal/open |
| Application crypto | Native HTTP decoder filter `envoy.filters.http.fpga_crypto` | SHA-256, the same signatures, AES-128/256-GCM seal/open |
| Post-quantum application crypto | Native HTTP filter `envoy.filters.http.pq_crypto` | ML-DSA, ML-KEM, SLH-DSA and discoverable liboqs signature/KEM modes; explicit CPU backend or ABI 2 hardware |

**Hardware prerequisite:** F2 does not expose generic crypto commands by
default. Load an AFI implementing [the FPC1 BAR0 ABI](docs/fpga-abi.md), backed
by crypto IP, and provision its key slots. This repository includes the host
implementation, a CPU behavioral emulator, and independently simulated
[FPGA hash/arithmetic kernels](docs/rtl-kernels.md), **not an FPGA bitstream**.
There is no F2 device or AWS account attached to this workspace; real hardware
performance and interoperability have not been measured.

See [post-quantum coverage, build, API and TLS boundaries](docs/post-quantum.md).
The CPU plugin supports 262 enabled algorithm/parameter modes in the pinned
build, including experimental Falcon. This pinned backend does not implement
final FN-DSA; experimental Falcon is not claimed as final FN-DSA. The RTL includes
Keccak/SHAKE and ML-KEM/ML-DSA NTT building blocks; complete hardware
signature/KEM engines and their F2 shell/mailbox integration remain unfinished.

## Run the tests

The standalone suite needs a C++17 compiler, Make, and OpenSSL development
headers/libraries. Its classical CPU backend exists only in `tests/` and is
excluded from the production Envoy target. PQ software mode is a separate,
explicitly selected liboqs plugin.

```sh
make test
make sanitize
```

The TLS suite builds and uses the exact pinned BoringSSL source. It needs
Python 3.12+, CMake 3.22+, Make, and network access for the first download.
The download is checked against Envoy's published SHA-256 pin.

```sh
python3 tools/test_boringssl.py
# Or use an existing verified build:
make test-boringssl BORINGSSL_SRC=/path/to/boringssl-0.20260929.0 \
    BORINGSSL_BUILD=/path/to/boringssl-build
```

Tests cover NIST AES-GCM known answers, SHA-256, independently verified RSA
and ECDSA signatures, invalid tags/AAD, maximum payloads, bounded queue
capacity, concurrent callers, cancellation, queue expiry, MMIO errors,
malformed device responses, wrong sequence IDs and stuck hardware. Actual
BoringSSL client/server handshakes cover TLS 1.2/1.3, RSA/ECDSA, AES-128/256,
scatter buffers, TLS 1.3 KeyUpdate, certificate verification, disconnects
during signing and failure without a CPU fallback.

See [validation results and exact test scope](docs/validation.md).
The [implementation report and diagram](docs/reports/2026-10-18/README.md)
show the verified components and remaining F2 integration work.

To build and test the optional PQ plugin:

```sh
python3 tools/build_pq.py --test
```

The optional hardware-kernel checks need Verilator and OpenSSL development
libraries; the additional generic synthesis check needs sv2v and Yosys:

```sh
make lint-rtl test-rtl
make test-rtl-synth
```

These simulate the actual RTL, including backpressure, cancellation and
erasure. They do not run an AFI or establish FPGA performance.

## Build Envoy

This is a source extension, not a `.so` that a stock Envoy image can load.
Use Envoy's normal native build prerequisites and the exact pinned version.
Its pinned Linux Clang needs `libtinfo.so.5`, which Envoy's build image supplies:

```sh
git clone --depth 1 --branch v1.39.3 https://github.com/envoyproxy/envoy.git /path/to/envoy
python3 tools/install_envoy.py /path/to/envoy
cd /path/to/envoy
bazel build --config=clang -c opt --jobs=6 //source/exe:envoy-static
```

The installer copies the production source to `source/extensions/fpga_crypto`
and adds its registration target to Envoy's extension map. It can be run
again after editing this repository. HTTP configurations use the distinct
`envoy.extensions.fpga_crypto.v3.FpgaHttp` and `PqHttp` messages from
`config.proto`. When upgrading the original Struct-based examples, replace
the HTTP `@type` URL and retain the fields under `value`; the TLS private-key
provider's configuration is unchanged. To compile only the extension:

```sh
bazel build --config=clang //source/extensions/fpga_crypto:config
```

A smaller extension selection is included for linked HTTP integration tests:

```sh
bazel build --config=clang --define=google_grpc=disabled --//bazel:http3=false \
    --override_repository=envoy_build_config=/absolute/path/to/this/project/envoy/minimal_build_config \
    //source/exe:envoy-static
```

After building that binary and the PQ plugin, run from this project's root:

```sh
make test-envoy ENVOY_BIN=/path/to/envoy/bazel-bin/source/exe/envoy-static \
    OQS_BUILD=/path/to/liboqs-build \
    BORINGSSL_SRC=/path/to/boringssl-0.20260929.0 BORINGSSL_BUILD=/path/to/boringssl-build
```

The native C++ harness launches Envoy with the software example on ephemeral
loopback ports. It checks HTTP routing, ML-DSA signatures, ML-KEM interoperability,
contexts, malformed requests and key erasure. Per-process startup logs and
rendered configurations remain under `build/envoy-http/`. This exercises the
explicit CPU plugin; it does not substitute software for an unavailable FPGA.

Use the default non-FIPS BoringSSL build. OpenSSL, AWS-LC, symbol-prefixing,
LTO and shared BoringSSL builds are outside this bridge's supported build
contract. Four linker `--wrap` options in `BUILD` intercept calls from the
TLS implementation to `EVP_AEAD_CTX_init_with_direction`, `cleanup`,
`seal_scatter`, and `open`. The pin matters: changes to those call sites
require rerunning the TLS integration suite and reviewing the bridge.

## Run on F2

1. Install the official [AWS F2 SDK](https://github.com/aws/aws-fpga/tree/f2).
   Load your compatible AFI using the SDK's image-management tools.
2. Provision RSA/ECDSA and AES keys through your accelerator's management
   plane. Place the corresponding public TLS certificate chain on the host.
3. Create `/run/lock/envoy-fpga-crypto` owned by the Envoy service user with
   mode `0700`, and grant that user access to the FPGA BAR device. Set
   `sdk_library` in [the example](examples/envoy.yaml) to the installed
   `libfpga_mgmt.so` that exports the PCI API. No AWS credentials are needed
   for the extension's local MMIO operations. Disable SDK register-value debug
   logging: mailbox reads/writes contain traffic keys and plaintext.
4. Start the custom Envoy binary with the example configuration:

```sh
bazel-bin/source/exe/envoy-static --disable-hot-restart -c /path/to/examples/envoy.yaml
```

The example binds to loopback. Its private-key slot is RSA slot 1; switch
the certificate, signature algorithms and TLS 1.2 cipher suites together
when selecting ECDSA slot 2. Do not supply `private_key` in the TLS certificate
configuration. `fallback: false` prevents silently using a host private key.
Keep any remote crypto endpoints behind your intended authentication and
authorization filters; signing endpoints are signing oracles by design.

All consumers of a slot must use identical SDK path, lock directory and queue
capacity. The queue is shared across listeners and filters. Cross-process
exclusion intentionally prevents Envoy hot restart from overlapping two
owners of the same single-mailbox accelerator. A failed startup or unhealthy
device does not fall back to CPU.

## Application HTTP API

Configure the filter before the router. Nonmatching paths continue normally.
Only paths enabled by `key_slots` are available; clients cannot choose a
hardware key slot or supply raw keys. Every operation accepts `POST` with a
binary body and returns a binary body, `application/octet-stream`, with
`Cache-Control: no-store`.

| Default path | Request | Response |
|---|---|---|
| `/_fpga/sha256` | Message | Raw 32-byte digest |
| `/_fpga/sign/ecdsa` | Message | DER P-256 signature |
| `/_fpga/sign/rsa-pss` | Message | RSA-PSS signature |
| `/_fpga/sign/rsa-pkcs1` | Message | RSA PKCS#1 signature |
| `/_fpga/aes128/seal`, `aes256/seal` | Plaintext | Ciphertext followed by 16-byte tag |
| `/_fpga/aes128/open`, `aes256/open` | Ciphertext and tag | Authenticated plaintext |

AES accepts optional hex AAD in `x-fpga-aad` (at most 8 KiB decoded). Seal
generates a fresh cryptographic 96-bit random nonce and returns its hex
encoding in `x-fpga-nonce`. Open requires that header and the original AAD.
Keep nonce/key pairs unique across all writers, persist the nonce with the
ciphertext, and rotate keys within the limits of your GCM usage policy.
The seal endpoint does not accept a caller-selected nonce. The 48 KiB input
limit includes plaintext/message bytes; open additionally allows its 16-byte
tag. Streaming hashing and larger application objects are not implemented.

```sh
printf abc | curl --cacert server-ca.pem --data-binary @- \
    https://localhost:8443/_fpga/sha256 --output digest.bin
```

Invalid requests/tags return 400, unknown operations 404, oversized payloads
413, device/queue failures 503 and deadlines 504. Requests never resume
upstream after matching the crypto path.

## Execution and limits

TLS signing and application operations use a dedicated worker and bounded
queue, with deadlines that include time waiting in the queue. Envoy checks
completion using a 1 ms dispatcher timer; cancellation never posts callbacks
to a destroyed connection. The host validates the image ABI, capabilities,
lengths and sequence IDs. It clears owned request key/input buffers and
requires the AFI to erase its mailbox on acknowledgment. Ambiguous device
state poisons the engine; recovery is operator-controlled.

The record bridge is **process-wide and opt-in** via
`offload_tls_records: true`. It applies to newly initialized TLS AES-GCM
contexts, including upstream TLS. BoringSSL continues to own handshake
transcripts, HKDF, ephemeral key agreement, record framing, sequence numbers,
AAD and nonce construction. The bridge preserves TLS AEAD nonce-reuse checks
and zeroes failed outputs. Traffic keys are derived on the CPU and sent
inline to the FPGA; provisioned signing keys stay behind their hardware
slot interface.

Record operations are **synchronous** because BoringSSL's record API is
synchronous. Waiting for the FPGA therefore blocks that Envoy worker up to
the configured deadline. The PIO transport is a functional prototype, not
a throughput claim. Use DMA/batching and an asynchronous transport design
before treating this as production bulk TLS acceleration. ChaCha20, CBC,
QUIC, TLS key-share offload, other hash families, FIPS certification and
hardware key provisioning are outside the implementation. Non-AES TLS
record algorithms are handled by the ordinary TLS stack and are not
reported as offloaded. PQ application operations and ML-DSA TLS signing
are described separately in [the PQ guide](docs/post-quantum.md).

The public hooks are documented by [Envoy's private key provider API](https://www.envoyproxy.io/docs/envoy/v1.39.3/api-v3/extensions/transport_sockets/tls/v3/common.proto)
and [AWS's F2 PCIe SDK](https://github.com/aws/aws-fpga/blob/f2/sdk/userspace/include/fpga_pci.h).
