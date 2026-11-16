# Post-quantum operations

The extension implements a generic signature/KEM API and an explicit CPU
reference backend using **liboqs 0.16.0**. It does not implement new lattice
arithmetic. The FPGA host transport supports the same operations through
FPC1 ABI 2; a matching FPGA implementation is still required. There is no
PQ RTL, synthesized accelerator, AFI or F2 performance result in this project.

## Coverage and standards

The pinned, default x86-64 liboqs build exposes **262 algorithm/parameter
modes**, not 262 independent cryptographic schemes. Its catalog includes:

| Family | Operations | Default policy |
|---|---|---|
| ML-DSA-44, -65, -87 (FIPS 204) | Key generation, sign, verify | Enabled |
| ML-KEM-512, -768, -1024 (FIPS 203) | Key generation, encapsulate, decapsulate | Enabled |
| Pure SLH-DSA (FIPS 205) | 12 parameter sets, sign/verify/keygen | Enabled |
| HashSLH-DSA | 144 library prehash/hash/parameter combinations | Explicit opt-in |
| Falcon-512/-1024, padded variants | Key generation, sign, verify | Experimental opt-in |
| MAYO, CROSS, UOV, SNOVA, MQOM | Signature modes present in the library build | Experimental opt-in |
| Classic McEliece, BIKE, HQC, Kyber, NTRU, NTRU Prime, FrodoKEM, eFrodoKEM | KEM modes present in the library build | Experimental opt-in |

The conservative default policy exposes 18 pure standardized parameter
sets. The remaining 244 modes require `allow_experimental: true`, including
the prehash combinations. Some prehash/parameter combinations reduce the
security strength below that of their underlying SLH-DSA parameter set;
the catalog's level is the library's claimed parameter-set level, not an
assurance for a weaker prehash. See FIPS 205 Section 10.2 when choosing a
prehash. Only algorithms actually enabled in the linked
library are reported. Exact names, implementation versions, key sizes,
signature/ciphertext sizes, claimed security levels and context support
are discoverable; unavailable schemes are rejected. A future library upgrade
requires changing the version pin and rerunning interoperability tests.

NIST's standardization page, updated August 5, 2026, still lists **FIPS 206
(FN-DSA) as in development**. The implementation therefore offers `Falcon-512`
and `Falcon-1024` under their library names and rejects `FN-DSA-512` rather
than claiming an unfinished standard. Kyber and ML-KEM are also distinct.
`standardized` identifies the FIPS algorithm format, not FIPS module
validation or approval of every prehash/security-level combination.

Stateful LMS/XMSS are excluded. Supporting them requires durable, atomic
signing-state management and recovery across restarts; a volatile stateless
key-slot wrapper would risk reusing one-time signing keys. Composite/hybrid
application schemes and every published PQ candidate are not provided.

Sources: [NIST standards status](https://csrc.nist.gov/Projects/Post-Quantum-Cryptography/Post-Quantum-Cryptography-Standardization),
[liboqs algorithms](https://openquantumsafe.org/liboqs/algorithms/),
[liboqs 0.16.0 release](https://github.com/open-quantum-safe/liboqs/releases/tag/0.16.0).
The library's experimental candidates are intended for research; enabling
them is an explicit application choice, not a production recommendation.

## Build and test

```sh
python3 tools/build_pq.py --test
# Or use an existing build made with OQS_USE_OPENSSL=OFF and PIC:
make test-pq OQS_BUILD=/path/to/liboqs-build
make sanitize-pq OQS_BUILD=/path/to/liboqs-build

# Exercise every enabled algorithm/parameter mode:
PQ_TEST_ALL=1 ./build/pq_test "$PWD/build/libfpga_pq_oqs.so"

# Independent implementation checks and real TLS 1.3 handshakes:
make test-pq-boringssl OQS_BUILD=/path/to/liboqs-build \
    BORINGSSL_SRC=/path/to/boringssl-0.20260929.0 \
    BORINGSSL_BUILD=/path/to/boringssl-build
```

The helper downloads a checksum-pinned source archive, builds liboqs with
OpenSSL disabled, and statically links it into `build/libfpga_pq_oqs.so`.
Only `fpga_pq_plugin_v1` is exported. A versioned C ABI and caller-owned byte
buffers isolate GCC/libstdc++ plugin code from Envoy's Clang/libc++ and
BoringSSL. The native extension loads the operator-selected absolute path
with `RTLD_LOCAL`; it has no liboqs header or OpenSSL dependency. Install the
plugin and its runtime C++ library dependencies alongside the custom Envoy
binary. This plugin does not make the extension loadable by stock Envoy.

## Envoy HTTP configuration

[pq-software.yaml](../examples/pq-software.yaml) is a loopback CPU demo.
Set `plugin_library` to the installed absolute path. The filter name is
`envoy.filters.http.pq_crypto`; its typed configuration is
`type.googleapis.com/envoy.extensions.fpga_crypto.v3.PqHttp`, with the validated
configuration fields under `value`. The classical HTTP filter uses the distinct
`FpgaHttp` message from `config.proto`. Using the same generic Struct type for
both factories would collide in Envoy's factory-by-type registry. The TLS
private-key provider keeps its existing Struct configuration in its separate
factory category.

`backend: software-liboqs` explicitly selects CPU operation. For actual
hardware, use `backend: aws-fpga` (the default), remove `plugin_library` and
`max_keys`, and configure `slot_id`, `sdk_library` and `lock_directory` as
in the classical filter. The hardware must implement ABI 2 and return its
own catalog. Configuration fails at startup if catalog discovery fails or
a route names an unavailable/disallowed algorithm. There is no automatic
software fallback, and CPU library capabilities are never advertised as
hardware capabilities.

Each configured route binds an operation, exact algorithm and optional
key slot. Clients cannot choose algorithm names or private-key slots via
request headers. Key-generating/signing/decapsulating routes use nonzero
slots; verify and encapsulate use external public keys and slot zero.
`algorithms` is always the catalog route. Nonmatching prefixes continue
through the filter chain; matching prefixes never forward upstream.

Every endpoint requires POST. Successful replies carry
`X-Crypto-Backend: software-liboqs` or `aws-fpga`, `Cache-Control: no-store`,
and `application/octet-stream` (catalog uses JSON). A nonempty signature
context is supplied as hex in `x-pq-context`, up to 255 decoded bytes.
Contexts are rejected by schemes that do not support them; they are never
silently dropped. Prehash algorithm variants receive the original message;
the selected liboqs variant performs its specified prehash.

| Operation | Request body | Successful response |
|---|---|---|
| `algorithms` | Empty | JSON catalog for the selected backend and policy |
| `keygen` | Empty | Raw public key; private key stays in the slot |
| `public-key` | Empty | Raw public key for the matching algorithm/slot |
| `sign` | Message | Raw signature |
| `verify` | Framed public key, signature, message | Empty, HTTP 200 |
| `encapsulate` | Raw public key | Framed ciphertext, shared secret |
| `decapsulate` | Raw ciphertext | Raw shared secret |
| `erase` | Empty | Empty, HTTP 200 |

Envoy omits `Content-Type` on empty local replies, including successful
`verify` and `erase` acknowledgments. Nonempty binary responses use
`application/octet-stream`; the catalog uses `application/json`.

Binary framing is little-endian: `u32 version=1`, `u32 part_count`, one
`u32 byte_length` per part, then the parts concatenated in table order,
without padding. Truncation, trailing bytes, excessive counts and lengths
are rejected. Messages, keys and ciphertexts are each bounded to 3 MiB;
the complete mailbox frame is bounded to 8 MiB. Individual algorithms
enforce their exact key/ciphertext sizes and maximum signature sizes.

Invalid framing, wrong key slots/algorithms, and invalid signatures return
400; unknown paths return 404; oversized bodies 413; queue/device/backend
failures 503; deadlines 504. ML-KEM's implicit rejection is different:
a correctly sized but tampered ciphertext produces a pseudorandom shared
secret with success, as FIPS 203 specifies. Applications must authenticate
their protocol transcript; decapsulation success is not authentication.

Example requests against the software demo:

```sh
curl -fsS -X POST http://127.0.0.1:8088/_pq/algorithms
curl -fsS -X POST http://127.0.0.1:8088/_pq/mldsa/keygen -o public.bin
printf 'message' | curl -fsS --data-binary @- \
    http://127.0.0.1:8088/_pq/mldsa/sign -o signature.bin
```

Python framing for verification:

```python
import struct
from pathlib import Path
from urllib.request import Request, urlopen
parts = [Path("public.bin").read_bytes(), Path("signature.bin").read_bytes(), b"message"]
body = struct.pack("<II", 1, len(parts))
body += struct.pack("<" + "I" * len(parts), *(len(p) for p in parts)) + b"".join(parts)
with urlopen(Request("http://127.0.0.1:8088/_pq/mldsa/verify", data=body, method="POST")) as reply:
    assert reply.status == 200
```

Expose only the routes an application needs and put remotely accessible
routes behind the intended authentication/authorization filters. A client
authorized for `keygen` or `erase` controls that configured slot. These are
explicit management operations, not implicit actions during signing.

## Key and worker lifecycle

Software keys are volatile per filter configuration/engine instance. They
are wiped on explicit erase and instance destruction; they are not exported,
persisted or protected by an FPGA/HSM. Key generation refuses to overwrite
an occupied slot. Bindings include the exact algorithm, so a slot cannot be
used with another parameter set or operation family. The software backend
limits key count (default 32, configurable 1..1024) and total stored key bytes
to 64 MiB. Hardware key provisioning/persistence remains its implementation's
responsibility.

Library calls run on the bounded Engine worker, not an Envoy request thread.
Deadlines include queue time. An in-progress liboqs call cannot be forcibly
interrupted: an expired caller receives no cryptographic output, but the
worker remains occupied until the call returns. Shutdown joins that worker.
The filter's dispatcher timer enforces the response deadline independently.
Keygen/erase can have committed when a caller times out or disconnects;
query the public-key route to reconcile slot state before retrying. There
are no automatic retries. Cancellation drops responses and pending work,
and does not roll back completed management operations.

## TLS integration boundary

The pinned BoringSSL recognizes ML-DSA TLS signature algorithms 0x0904,
0x0905 and 0x0906. The private-key provider maps those to `PqSign` with the
matching ML-DSA catalog name and an empty FIPS 204 context; BoringSSL supplies
the TLS CertificateVerify message. An ABI 2 accelerator with a matching
provisioned key and certificate can serve these requests. Configure TLS 1.3
and `mldsa44`, `mldsa65` or `mldsa87` signing preferences. ML-DSA TLS 1.2,
Falcon/FN-DSA TLS, and SLH-DSA TLS are not implemented.
Use a dedicated TLS 1.3 listener with one matching ML-DSA certificate;
mixed classical/PQ certificate selection in Envoy has not been validated.

Tests perform all three ML-DSA TLS 1.3 handshakes with certificate verification,
asynchronous signing through the emulated ABI 2 mailbox and the CPU plugin,
and a BoringSSL X25519MLKEM768 key exchange. They also independently verify
plugin ML-DSA signatures and exchange ML-KEM-768 ciphertexts in both directions
with BoringSSL. **The TLS ML-KEM key exchange runs inside BoringSSL on the
CPU.** Exposing application KEM operations does not offload TLS key shares;
that requires an additional BoringSSL key-share integration.
