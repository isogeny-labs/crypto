## Why

The requested graph, structured Semaev/F4 and RR5 components needed executable interfaces with explicit source identity, replay and stopping contracts. This implements all three in the existing Rust source ecosystem and retains local correctness evidence.

## What changed

- **Graph:** fresh-process eager/lazy forest, pivot and MST comparison on an admitted fixed-curve stream; exact forward/transpose projection tape; input/source/binary custody, caps, witness replay, rank and target-outcome checks. Full group admission runs inside bounded children; coordinator startup is separately recorded.
- **Structured solver:** constrained M31 three-summand expanded S4 and two chained S3 orders, explicit cancellation chart, unchanged leaf selectors, replayable selector-normal-form cofactors, bounded fallback, guarded original F4 visitor and signed rational witnesses.
- **RR5 integration:** three incremental visitor entry points and a caller-driven public-query adapter. Rows reach the existing relation adapter after quartic root recovery, signed lift and point replay. Consumer stopping, caps, unsupported charts and exhaustive completion remain distinct. Existing batch entry points remain available.
- Added component READMEs, a bounded development-check wrapper, source/lock manifests, independent review, a generic resultant identity record, and the implementation report with editable diagram/PDF.

## Validation

| Final release gate | Result |
| --- | --- |
| Graph 06 | 7 integration tests; includes 128 dense-oracle systems and four fresh CLI workers |
| Structured final | 10 library + 3 CLI tests; both binary targets compile |
| RR5 all-targets | 42 library + 8 public-example tests; existing binary targets compile |

All three component gates returned exit 0: **70 distinct tests pass**. An independent reviewer also executed the ten structured producer-authored debug unit tests, without an independent rebuild. These are unsigned development observations. [pcc:unverified]

Auxiliary generic identity `IDC1hd6568d78855aef4c` was issued and replayed at 32+16 points in the repository format's fixed M61 field; both issuer and checker rejected the formula-plus-one corruption. This auxiliary record does not replace M31 controls or supply a formal proof. Producer claim `PCC-20261124-db5264` binds the aggregate evidence and report and remains unverified. [pcc:unverified]

## Remaining gates and scope

The **full root-library gate is not passed**. The dependency-restored test build lost its execution handle without a terminal status; a separately retained cached release-test retry returned 124 at its cap. A focused compiler check reached `src/cryptanalysis/prime_root_trace.rs` and emitted E0425 for `powers`, `gcds`, `previous`, `r` and `g`, then returned 124 at its cap. That source file is byte-identical to main `ceba74bb64854027c4dd16f00428bf676383bfdb`; the identities and all attempt logs are retained. Sparse dependencies were hydrated as encountered. [pcc:unverified]

The tape is an operator, not a complete Krylov solver. Structured extraction is m=3; RR5 covers its existing distinct-four chart. Callback deadlines are cooperative, and complete cold-cost accounting remains unavailable. Rustfmt is not installed; diff-check notes retained raw-log EOF blank lines and a final blank line in the tested graph Cargo manifest.

**Retain the existing no-merge and no-paid-compute holds.** No scientific comparison, natural collector rerun or B677 extraction panel was launched. No performance result or canonical scoreboard value changes in this PR.

## Final wrapper and CI follow-up

The final compatibility check found that the aggregate wrapper selected inherited graph unit tests requiring the full root telemetry API. The corrected wrapper uses the qualified graph integration target and keeps structured/RR5 on all targets. Its full cached release invocation returned zero for all three components, repeating the same **70 distinct tests**. The failed all-targets attempt and successful wrapper invocation are both retained. Supplemental claim `PCC-20261124-9f9ff4` pins the wrapper correction and its logs; its standing remains unverified. Original claim-bound source/report artifacts remain unchanged. [pcc:unverified]

Automatic [Round123](https://github.com/isogeny-labs/crypto/actions/runs/45323167283) and [Round124](https://github.com/isogeny-labs/crypto/actions/runs/45323167354) checks at head `42cbbf52` failed at existing source-precondition assertions before compilation or benchmark execution. The scripts, exact source inputs and workflows are byte-identical to the PR base. Read-only inspection reproduced the failing string predicates; baseline CI itself was not run. These observations do not establish a passing whole-repository gate. No manual rerun or scientific workload was dispatched. [pcc:unverified]

- [Wrapper correction and raw-evidence guide](https://github.com/isogeny-labs/crypto/blob/lane/m31-integrated-build-20261123/research/m31_integrated_build_20261123/evidence/WRAPPER_CORRECTION.md)
- [Final wrapper receipt](https://github.com/isogeny-labs/crypto/blob/lane/m31-integrated-build-20261123/research/m31_integrated_build_20261123/evidence/wrapper-final-v1/receipt.json)

## Evidence

- [Implementation report](https://github.com/isogeny-labs/crypto/blob/709a68ad6367751e0d2b7362de22bca35cb3d631/research/m31_integrated_build_20261123/reports/REPORT.md)
- [Four-page report PDF](https://github.com/isogeny-labs/crypto/blob/709a68ad6367751e0d2b7362de22bca35cb3d631/research/m31_integrated_build_20261123/reports/m31-integrated-build.pdf)
- [Aggregate source/evidence manifest](https://github.com/isogeny-labs/crypto/blob/709a68ad6367751e0d2b7362de22bca35cb3d631/research/m31_integrated_build_20261123/evidence/verification.json)
- [Independent review and auxiliary authorship scope](https://github.com/isogeny-labs/crypto/blob/709a68ad6367751e0d2b7362de22bca35cb3d631/research/m31_integrated_build_20261123/evidence/INDEPENDENT_REVIEW.md)

The published source tree exactly matches the reviewed local Git index, including six relative symlinks and both new Cargo lockfiles. Sources checked (KB): unavailable in this runtime; repository source/active handoffs and original Semaev, F4 and double-large-prime papers supplied the fallback. [pcc:unverified]
