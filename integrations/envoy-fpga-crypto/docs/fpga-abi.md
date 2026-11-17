# FPC1 accelerator interface, versions 1 and 2

The original classical interface below is ABI 1. ABI 2 adds PQ operations,
catalog discovery and larger windows as specified at the end of this file.
The host accepts both versions; ABI 1 devices cannot receive PQ requests.

This is a **custom accelerator protocol**, not an AWS-provided crypto API.
An F2 AFI must implement it on application PF BAR0. The host SDK uses
`fpga_pci_attach(slot, FPGA_APP_PF, APP_PF_BAR0, 0, &handle)`, followed by
32-bit `fpga_pci_poke`/`fpga_pci_peek`. The mapped BAR must cover at least
`0x21000` bytes. Offsets and payload integers are little-endian.

AWS supplies the shell, PCIe access, SDK and AFI lifecycle. This repository
supplies the host extension and behavioral emulator. It does **not** supply
synthesized RSA/ECC/AES IP, a deployable AFI, or a key provisioning service.

## BAR0 registers

| Offset | Access | Meaning |
|---|---|---|
| `0x00` | RO | Identity `0x46504331` |
| `0x04` | RO | ABI version `1` |
| `0x08` | RO | Capabilities: bit `operation - 1` |
| `0x0c` | RO | State: idle `0`, busy `1`, done `2`, failed `3` |
| `0x10` | RW | Request byte length, at most 65,536 |
| `0x14` | RO | Response byte length |
| `0x18`, `0x1c` | RW | Request sequence, low/high 32 bits |
| `0x20`, `0x24` | RO | Completed sequence, low/high 32 bits |
| `0x28` | WO | Doorbell: submit `1`, acknowledge/erase `2` |
| `0x2c` | RO | Failure: GCM authentication `1`; other errors `2+` |
| `0x1000..0x10fff` | RW | Request window, 64 KiB |
| `0x11000..0x20fff` | RO | Response window, 64 KiB |

Only one command is in flight per slot. A process-wide queue serializes all
users; a host `flock` excludes other cooperating processes. The FPGA must not
accept a second submit while busy/done/failed. Publish BUSY atomically with
accepting the doorbell. Publish the response, length, error and completed
sequence before publishing DONE/FAILED. Completion registers remain stable
until ACK. Reads must obey PCIe ordering; request-length readback drains
posted writes before submission.

ACK must clear **both entire windows**, including inline keys, plaintext,
partial results and data from prior longer requests, before publishing IDLE.
Provisioned long-lived key slots remain. On accepting ACK, publish BUSY while
scrubbing the windows, then IDLE when erasure is complete. The host polls
until IDLE within the original request deadline. Zero-cycle erasure may
publish IDLE immediately. DONE/FAILED after ACK is a protocol failure.

Startup accepts only the exact identity, supported version and IDLE. A timeout, bad
sequence, malformed length, unexpected state, MMIO error or failed ACK
permanently poisons that host engine. It does not reset the FPGA or accept a
late completion. Stop all users, perform operator-controlled image/device
recovery and reprovisioning as needed, then restart Envoy. Merely restarting
Envoy cannot recover a mailbox left BUSY/DONE.

## Request payload

The first 32 bytes contain eight unsigned 32-bit words:

| Byte offset | Field |
|---|---|
| `0` | Version, `1` |
| `4` | Operation ID |
| `8` | Key slot; `0` means an inline TLS traffic key for AES |
| `12` | Input length |
| `16` | AAD length |
| `20` | Nonce length |
| `24` | Inline key length |
| `28` | Reserved; must be zero |

Append `nonce || inline_key || AAD || input`, without alignment padding.
Round the final host MMIO write up to four bytes with zero padding; the
request length excludes that padding. Maximum plaintext/sign/hash input is
49,152 bytes; AES open accepts 49,168 bytes including its tag. AAD is at most
8,192 bytes. GCM nonce length is exactly 12 and tag length exactly 16.

| ID | Operation | Input | Output |
|---|---|---|---|
| 1 | SHA-256 | Message | 32-byte digest |
| 2 | AES-128-GCM seal | Plaintext | Ciphertext followed by 16-byte tag |
| 3 | AES-128-GCM open | Ciphertext and tag | Authenticated plaintext |
| 4 | AES-256-GCM seal | Plaintext | Ciphertext followed by 16-byte tag |
| 5 | AES-256-GCM open | Ciphertext and tag | Authenticated plaintext |
| 6 | ECDSA P-256, SHA-256 | Message, **not a digest** | ASN.1 DER ECDSA signature, at most 72 bytes |
| 7 | RSA-PSS, SHA-256 | Message, **not a digest** | Modulus-sized signature, 256..512 bytes |
| 8 | RSA PKCS#1 v1.5, SHA-256 | Message, **not a digest** | Modulus-sized signature, 256..512 bytes |

RSA keys are 2048..4096 bits. PSS uses SHA-256 for MGF1 and a 32-byte random
salt. ECDSA must use a cryptographically secure per-signature nonce or a
correct deterministic construction; never reuse one. Hashing and signature
encoding/padding are the accelerator's responsibility. Implement constant-time,
fault-resistant private-key operations and an appropriate hardware entropy
source when integrating real crypto IP.

Non-AES requests must have empty AAD/nonce/inline-key. SHA-256 uses slot zero.
Signing uses a nonzero provisioned slot and rejects the wrong key type. AES
uses either a nonzero provisioned AES slot with no inline key, or slot zero
and exactly 16/32 inline key bytes. Never allow a request to overwrite a
provisioned slot. There is deliberately no key export/provision command in
this data plane ABI.

Only authenticated plaintext may appear in a successful open response. On
authentication failure, publish FAILED with error 1 and response length zero;
erase speculative plaintext internally. No response bytes are valid on a
failure. The host acknowledges normal operation failures and keeps the engine
available, but never retries them in software.

## Hardware integration and performance

The repository now includes [Keccak/SHAKE and NTT RTL kernels](rtl-kernels.md).
They are not connected to this mailbox and do not implement complete signing
or KEM commands. An AFI must expose only complete, validated operations in its
capabilities/catalog; these kernels alone do not satisfy that contract.

Connect this register/memory interface to reviewed crypto IP behind the F2
shell's application AXI interface, then simulate it against the host sequence
above. `tests/emulated_registers.h` is an executable behavioral reference.
Key provisioning, access control, key destruction across AFI reload/reset,
entropy, side channels and image attestation belong to the hardware integration.
An F2 FPGA is not automatically an HSM.

This mailbox is intentionally a correctness-first PIO transport. Large
payloads require many 32-bit PCIe transactions. Before expecting an advantage
over AES-NI, measure end-to-end latency, throughput and Envoy tail latency.
A production bulk engine should replace the PIO windows with DMA queues and
batching while retaining bounded lengths, deadlines, ownership, sequence
validation and authenticated-output semantics.

## ABI 2: post-quantum operations

ABI register `0x04` reports **2**. The register offsets and state machine
remain the same. The request window starts at `0x1000` and is **8 MiB**;
the response window moves to **`0x801000`** and is also 8 MiB. BAR0 must
therefore cover at least **`0x1001000` bytes**. All responses, including
classical responses, use the ABI 2 response window. Classical requests retain
their ABI 1 payload encoding and bounds. PQ requests use encoding version 2.
ACK must erase both entire 8 MiB windows before publishing IDLE; erasure
time counts toward the operation deadline.

PQ request words (all unsigned little-endian 32-bit):

| Byte offset | Meaning |
|---|---|
| 0 | Payload version 2 |
| 4 | Operation ID |
| 8 | Nonzero private-key slot, or zero for catalog/verify/encapsulate |
| 12 | Data byte length, at most 3 MiB |
| 16 | Signature context byte length, at most 255 |
| 20, 24 | Reserved zero; no nonce or inline secret key |
| 28 | Exact catalog algorithm-name length, at most 96 |
| 32 | Auxiliary byte length, at most 3 MiB |

Append `algorithm_name || context || auxiliary || data` without padding.
Algorithm names are ASCII letters, digits, hyphens and underscores, with no
NUL terminator. Total encoded length is at most 8 MiB. Catalog requests have
empty algorithm/context/auxiliary/data and slot zero. Non-signature operations
have empty contexts. Only verify has auxiliary bytes.

| ID | Operation | Data / auxiliary | Response |
|---|---|---|---|
| 9 | Catalog | Empty | Framed algorithm descriptors |
| 10 | Keygen | Empty | Public key; newly generated private key remains in slot |
| 11 | Sign | Message / empty | Signature, bounded by catalog maximum |
| 12 | Verify | Message / framed public key, signature | Empty on success |
| 13 | Encapsulate | Public key / empty | Framed ciphertext, shared secret |
| 14 | Decapsulate | Ciphertext / empty | Shared secret |
| 15 | Public key | Empty | Public key for algorithm-bound slot |
| 16 | Erase key | Empty | Empty |

The capabilities bitmap uses the same bit `ID-1` convention. A device may
advertise only its implemented operations and algorithms. Catalog support is
required for PQ operations: the host discovers/caches it before the first
non-catalog PQ request, checks algorithm kind/context/input lengths, and
validates key/ciphertext/shared-secret response lengths against it. Unsupported
algorithms never trigger a CPU fallback. Firmware must additionally enforce
slot ownership, algorithm bindings, policy and cryptographic validity.

Framing used throughout PQ responses is `u32 version=1`, `u32 count`,
`count` unsigned 32-bit lengths, followed by concatenated byte strings.
No padding or trailing bytes is accepted. A catalog is a frame containing
one record per enabled mode, at most 512 records. Each record is a frame
of three parts: a 32-byte descriptor, the algorithm name, and the implementation
version string (at most 4096 bytes). Descriptor words are:

| Word | Meaning |
|---|---|
| 0 | Kind: stateless signature 1, KEM 2 |
| 1 | Flags: standardized scheme/mode bit 0, context support bit 1; other bits zero |
| 2 | Claimed NIST security level, 1..5 |
| 3 | Public-key bytes |
| 4 | Secret-key bytes (metadata only; no export operation) |
| 5 | Maximum signature bytes, zero for KEMs |
| 6 | Ciphertext bytes, zero for signatures |
| 7 | Shared-secret bytes, zero for signatures |

Every nonzero key/signature/ciphertext/secret size must fit 3 MiB. Names must
be unique. The implementation version identifies wire compatibility, not
the AFI deployment version. `encodeCatalog`/`decodeCatalog` in `src/pq_codec.cc`
and `tests/emulated_registers.h` are executable host references.

ABI 2 failure codes are authentication failure 1, device failure 2, invalid
request or slot 3, unavailable algorithm/operation 4, and key/queue capacity
5. FAILED must always publish response length zero. ML-KEM implicit rejection
is successful decapsulation with a pseudorandom secret, not failure code 1.
The host acknowledges ordinary operation failures and poisons the engine for
malformed responses or ambiguous device state.

Keygen must reject occupied slots; sign/decapsulate/public/erase must require
the exact algorithm bound to that slot. No private-key import/export is
defined. Keygen/erase may have committed if a client times out or disconnects;
do not retry automatically. Stateful signature algorithms are outside this
ABI's stateless signing model. Production FPGA integration also needs reviewed
PQ cores, entropy, management-plane authorization and persistence policy.

The larger PIO windows provide interoperability with multi-megabyte PQ keys;
they do not provide an efficient transport. Use DMA/batching for acceleration
and measure the complete host-to-device operation before claiming speedup.
