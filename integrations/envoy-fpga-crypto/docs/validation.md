# Validation

Validated in the managed Linux workspace. The original 2026-10-17 results
below are preserved in [their receipt](receipts/2026-10-17/README.md).

## Hardware-kernel validation, 2026-10-18

| Check | Result |
|---|---|
| Verilator source RTL lint, fatal `-Wall`, assertions enabled | Passed |
| Keccak/SHAKE, modular multiplication and forward/inverse NTT | 15/15 native C++ test groups passed |
| sv2v lowering and Yosys generic synthesis | Passed structural checks; zero combinational loops and no runtime divider/modulo/latch cells |
| Same external correctness suite against the synthesized Verilog | 15/15 groups passed |
| Existing core, classical TLS, PQ and PQ TLS functional regressions | 36/36 groups passed again |
| Fully linked minimal Envoy v1.39.3 executable | Passed |
| Actual Envoy HTTP listener with the explicit liboqs CPU backend | 5/5 groups passed |
| Required root `cargo test --release --lib --locked` | Failed before tests: 26 compilation errors in unchanged Rust sources |

The hardware suite uses Verilator 5.032, sv2v 0.0.13, Yosys 0.52 and GCC
14.2.0, with public fixture seed `0x4650433252544c`. Source simulation enables
randomized register initialization and runtime assertions. Independent
oracles include OpenSSL SHAKE, direct polynomial evaluation/interpolation,
and schoolbook negacyclic convolution. The same 15 groups are replayed after
generic synthesis; these are two representations of the same tests, not
30 independent test specifications. See [kernel interfaces and test scope](rtl-kernels.md).

The generic design has 14,953 coarse cells across the simulation top's
independent cores and their nested permutation/multiplier instances. This
is not a Xilinx utilization or timing result. Yosys reports register
replacement warnings for writable arrays, retained in the synthesis log;
its structural checks report no problems. Initial tooling attempts failed
on generated-C++ warning settings, generated-netlist warnings, and an
oversized generated model that was killed for memory use. The final
byte-addressed SHAKE state and generic synthesis flow passed both suites.
Failed attempts are retained with the successful evidence.

The root Rust failure is outside this module. For example, the duplicate
`Fp4` variants in `src/cryptanalysis/ecbench/workload.rs` are present in the
unchanged base blob `0433fa620596bd24cdf1dbfd06c1ae67e841b2e1`. No Rust code or
existing workflow was changed to suppress that failure.

The scoped `Envoy FPGA crypto` workflow runs the 16 core groups, fatal RTL
lint, 15 source RTL groups and the same 15 synthesis-replay groups. Its local
actionlint 1.7.7 check passed. It does not build the linked Envoy or run the
optional BoringSSL/liboqs suites.

## Linked Envoy and live HTTP, 2026-10-18

The minimal profile in `envoy/minimal_build_config` linked the complete
`//source/exe:envoy-static` executable with the custom extension. This was a
development fastbuild using Bazel 8.7.0, Clang 18.1.8 and libc++, with
`--define=google_grpc=disabled --//bazel:http3=false`. The legacy
`--define=http3=disabled` flag is ignored by this Envoy version and was
corrected before the successful build. Installed production sources, headers,
`BUILD` and `config.proto` were compared with the module snapshot.

`make test-envoy` launches actual Envoy processes using the software example
on ephemeral loopback and admin ports. All five groups passed:

- Catalog policy, routing, nonmatching prefixes and method handling.
- ML-DSA-65 key generation, public-key retrieval, signing, independent
  BoringSSL verification, message tampering and key erasure.
- Signature contexts, a 256 KiB binary message, and malformed/duplicate
  context headers.
- ML-KEM-768 interoperability with BoringSSL in both directions, implicit
  rejection and key erasure.
- Malformed requests followed by a successful request on the same listener.

The first run passed 2/5 groups: the harness incorrectly required
`Content-Type` on empty verify/erase acknowledgments. Envoy deliberately
removes that header for zero-length local replies. The corrected harness
still checks status, body, `no-store` and the CPU-backend label; nonempty
responses retain their required content types. No production filter behavior
was changed to obtain that pass.

The executable's successful startup also executes both HTTP factory
registrations, closing the original object-only check's gap. The live test
uses CPU liboqs, not an FPGA. Actual Envoy TLS listeners, the default full
extension selection, complete hardware signature/KEM commands, physical F2
operation and performance remain unverified. See the
[receipt](receipts/2026-10-18/README.md) and
[report with diagram and PDF](reports/2026-10-18/README.md).

## Original host and TLS validation, 2026-10-17

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
| Production C++ objects against Envoy v1.39.3 | Compiled, including the three factory definitions; no runtime registration check then |
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

The original object check did not build a fully linked Envoy executable or
execute the factory registry. It therefore missed the duplicate HTTP factory
configuration type: both filters returned `google.protobuf.Struct`.
`config.proto` now gives the filters distinct `FpgaHttp` and `PqHttp` message
types; both example HTTP configurations use their new type URLs. The TLS
private-key provider retains its existing configuration in its separate
registry category. The TLS harness separately verifies the four linker
wrappers against the exact BoringSSL pin. At the Oct 17 stage, full Envoy
linking and live HTTP integration had not been performed. Custom AFI
synthesis, device interoperability and F2 performance testing remain
deployment validation steps; no F2 hardware or AWS identity was attached.

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

The original 2026-10-17 run did not test a live Envoy HTTP listener,
synthesized PQ core or physical F2 operation. The subsequent kernel tests
exercise hardware building blocks, not complete hardware signature/KEM
commands. No deployed Envoy/F2 system has been tested end to end.
