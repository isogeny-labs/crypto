# Line-FFT algebra identity records

The separate issuer and checker passed for all three elementary identities.
Both programs use the unchanged production `identity.certificate/v1` and
SHA-256 modules. Build, issuer and checker exit codes were zero. All source,
binary, certificate, locked-archive and index custody checks passed.
[pcc:unverified]

| Identity | Statement ID | Certificate SHA-256 |
| --- | --- | --- |
| `(E+XO)+(E-XO)=2E` | `IDC1h202115d385061d50` | `1357d5159c9ff963fc9a8cda96d7a5ca47bcffe8b603d292d65d1f173c4973ac` |
| `(E+XO)-(E-XO)=2XO` | `IDC1hc0b9bda3040f6fb4` | `da5afa43f513e880562b6080e8bc9a4435ae1bedf3fe1ba32d86f0ab0a5f1362` |
| `(Z^2+1)^2-2Z^2=Z^4+1` | `IDC1h6bb2bd5b305d062c` | `4c8d15a0b318a059f5773788aadabb133c7309a2ae7ac04cabd5682324cf1495` |

Each record contains 32 evaluations with issuer seed `20261123`. The separate
checker replays those evaluations and checks 32 extra evaluations using each
of `20261124` and `20261125`. The right-side-plus-one control failed at index
zero for every statement in both issuer and checker. All three digest mutations
and all three underdeclared-degree controls were rejected as specified.
[pcc:unverified]

## What the evidence means

| Layer | Field or domain | What it supports |
| --- | --- | --- |
| These certificates | `F_(2^61-1)` | Fixed finite evaluations of three explicit polynomial expression trees and successful controls. |
| Written algebra in `../DERIVATION.md` | Ordinary polynomial algebra; odd-characteristic fields with stated nonzero denominators | The mathematical butterfly and trace-doubling argument. This has no kernel-checked proof artifact. |
| Separate native FFT qualification | Prime31; full fibres at `k=1,...,8` | Agreement of the specified transform cases with independent polynomial references. These certificates do not verify that executable. |
| Root-count and larger-depth statements | Norm-one torus and Dickson mathematics | Follow from the written mathematical argument and cited primary literature; they are not certified by these finite records. |
| Elliptic relation solving | Actual curve, subgroup, signs, relation rank and cost | Not established by either this certificate layer or the FFT qualifier. |

All formal-proof fields are null. These are same-team unsigned finite-evaluation
records; a separate checker process is not an independent verifier. The API's
numeric false-accept output is retained literally but is not asserted to be a
rigorous failure probability for these deterministic seeds. Dividing the
cleared trace identity by `Z^2` requires `Z != 0`; deriving the butterfly inverse
requires `2X != 0`. Those domain conditions are outside these certificates.
[pcc:unverified]

## Reproduction and custody

The frozen method is [PROTOCOL.md](PROTOCOL.md). The concrete certificate files
are in [evidence/attempt-01/certificates](evidence/attempt-01/certificates).
The attempt retains raw [issuer output](evidence/attempt-01/issuer.stdout.jsonl),
[checker output](evidence/attempt-01/checker.stdout.jsonl), all commands, exit
codes, source copies and hash checks. Its `PRE_BUILD.sha256` binds every source
and protocol before compilation. The frozen protocol SHA-256 is
`6ad51ef67e8490e023b9a8af959918ca67d8a586b62d1a4aa60ae97a8e423414`.
The witness SHA-256 is
`0cc7c986563845f09e9c5f306e1deec88df975f23c357a3edd0933bfc9c88753`.
[pcc:unverified]

Run `bash qualify.sh attempt-02` from this directory for a new immutable
attempt. Optional `PPLUS_RUSTC`, `PPLUS_CARGO` and `PPLUS_PARENT_CACHE` select
the available compiler, Cargo and archive/index cache. The driver copies only
12 locked archives and index records to a fresh local CARGO_HOME; it uses fresh
Cargo extraction and a dedicated target. It builds offline with a frozen
lockfile and does not modify the source task or its cache. All arithmetic and
certificate evaluation are native Rust. No performance experiment is included.
[pcc:unverified]

The local attempt also retains the copied archive/index cache, build target and
frozen native binaries. They are reproducible build inputs or products; source,
lock, hash manifests, certificates and raw outputs are the portable evidence.
[pcc:unverified]

## Task log

### What was done

Added three explicit expression-tree witnesses and dedicated issuer/checker
wrappers around unchanged production modules; built offline and retained one
successful attempt with all controls and custody checks. [pcc:unverified]

### Decisions made

Keep this evidence separate from native Prime31 FFT correctness, root-count
proofs, formal verification and ECDLP claims. Preserve exact production module
bytes and locked dependencies. [pcc:unverified]

### Open threads

Signed independent validation or a formal proof would require separate work.
Neither is implied by the successful finite checks. [pcc:unverified]

### Errors

A manually transcribed dependency-checksum typo was corrected during source
preparation, before freezing or building. No compile or native arithmetic
failure occurred; no failed attempt was discarded. [pcc:unverified]
