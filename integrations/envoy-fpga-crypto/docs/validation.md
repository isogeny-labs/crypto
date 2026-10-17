# Validation

Validated on 2026-10-17 in the managed Linux workspace.

| Check | Result |
|---|---|
| Core crypto, mailbox, deadlines, cancellation, concurrency and erasure | 16/16 passed |
| Actual BoringSSL TLS 1.2/1.3 handshakes, records and failure paths | 8/8 passed |
| PQ catalog, slot policy, malformed input/output, large keys and failure paths | 8/8 passed |
| Every enabled liboqs signature/KEM algorithm and parameter mode through ABI 2 | 262/262 passed |
| ML-DSA-44/65/87 TLS 1.3 and independent ML-KEM-768 interoperability | 4/4 passed |
| Core with AddressSanitizer and UndefinedBehaviorSanitizer | 16/16 passed |
| TLS adapters with AddressSanitizer and UndefinedBehaviorSanitizer | 8/8 passed |
| PQ adapter/plugin with AddressSanitizer and UndefinedBehaviorSanitizer | 8/8 passed |
| Production C++ objects against Envoy v1.39.3 | Passed, including all three factory registrations |
| Example YAML parsing and Python helper syntax | Passed |

Standalone builds used GCC 14.2.0 and OpenSSL 3.5.7. TLS tests linked the
checksum-verified BoringSSL 0.20260929.0 libraries. Native Envoy object
compilation used Bazel 8.7.0 and Envoy's Clang 18.1.8/libc++ toolchain. PQ
operations used the checksum-pinned liboqs 0.16.0 build with OpenSSL disabled.
The plugin exports only `fpga_pq_plugin_v1`; `ldd` confirms no OpenSSL/BoringSSL
dependency. The default catalog has 18 pure standardized parameter sets;
explicit opt-in enables the full 262 library modes.

Sanitizers instrumented this project's core, emulator and adapter code;
the BoringSSL and liboqs static libraries were normal Release builds. The
PQ plugin wrapper itself was instrumented. LeakSanitizer
was disabled (`ASAN_OPTIONS=detect_leaks=0`) because the workspace's ptrace
environment prevents its operation. Address/undefined-behavior checks
remained enabled.

## Reproduce the runtime tests

```sh
make test
make sanitize ASAN_OPTIONS=detect_leaks=0
python3 tools/test_boringssl.py
make sanitize-boringssl ASAN_OPTIONS=detect_leaks=0 \
    BORINGSSL_SRC="$PWD/.cache/boringssl-0.20260929.0" \
    BORINGSSL_BUILD="$PWD/.cache/boringssl-build"
```

On a host without ptrace restrictions, omit `ASAN_OPTIONS` to enable the
default leak checks too. The TLS harness verifies the server certificate
and uses no private key on the server's SSL context; signing goes through
the asynchronous provider and emulated BAR0 mailbox. AES-GCM record work
passes through the ELF wrappers and the same mailbox transport.

## Reproduce the native API compilation

After running the installer against Envoy v1.39.3, run in that checkout:

```sh
bazel build --config=clang --@io_bazel_rules_go//go/config:pure=true \
    --output_groups=compilation_outputs \
    //source/extensions/fpga_crypto:config \
    //source/extensions/fpga_crypto:core
```

This compiles all seven production core/bridge translation units and both
native Envoy adapters against real generated Envoy headers. For the local
release-archive checkout, workspace stamping was disabled with
`--workspace_status_command=/bin/true`. The missing Clang `libtinfo.so.5`
compatibility library was downloaded from Debian's package repository,
checksum-verified, and installed in the workspace toolchain directory.
No system directories were modified.

The object check does not build a fully linked Envoy executable. The TLS
harness separately verifies the four linker wrappers against the exact
BoringSSL pin. Full Envoy linking, live HTTP integration, custom AFI
synthesis, device interoperability and F2 performance testing remain
deployment validation steps. No F2 hardware or AWS identity was attached
to this workspace.

## Post-quantum test scope

```sh
python3 tools/build_pq.py --test
PQ_TEST_ALL=1 ./build/pq_test "$PWD/build/libfpga_pq_oqs.so"
make test-pq-boringssl OQS_BUILD="$PWD/.cache/liboqs-build" \
    BORINGSSL_SRC="$PWD/.cache/boringssl-0.20260929.0" \
    BORINGSSL_BUILD="$PWD/.cache/boringssl-build"
make sanitize-pq ASAN_OPTIONS=detect_leaks=0 OQS_BUILD="$PWD/.cache/liboqs-build"
```

Each of the 262 enabled algorithm/parameter modes generates a key, reads its
public key, rejects overwriting the occupied slot, and runs either a signature
round trip with message/signature tampering or a KEM encapsulation/decapsulation
round trip. Tests then erase the slot and confirm it is no longer usable.
All operations pass through the real host codec, BAR0 emulator, ABI 2 windows,
sequence checks and erasure acknowledgment. This is functional testing against
the library, not an independent cryptographic audit or NIST validation.

The default PQ groups additionally check experimental/prehash policy, FN-DSA
name rejection, algorithm/slot binding, key-count limits, contexts up to 255
bytes, empty messages, ML-KEM implicit rejection, malformed frames, 10,000
fuzzed frames, no software fallback and poisoned mailboxes. Large-key tests
include Classic McEliece-8192128f and UOV OV-V; a 256 KiB ML-DSA message crosses
the former ABI 1 limits. The exhaustive harness uses a 60-second deadline
per operation; shorter production deadlines can reject slow software modes.

BoringSSL independently verifies ML-DSA signatures for all three parameter
sets. It completes TLS 1.3 handshakes with certificate verification and no
server private key in the SSL context; signing traverses the emulated ABI 2
mailbox and the plugin. It also encapsulates to a plugin ML-KEM-768 key and
decapsulates plugin ciphertexts with its own key. The hybrid X25519MLKEM768 TLS
key share is BoringSSL CPU work and is not counted as FPGA offload.

No live Envoy HTTP listener, synthesized PQ core or physical F2 operation was
tested. Native object compilation and BoringSSL integration tests cover
different boundaries; neither proves a deployed Envoy/F2 system end to end.
