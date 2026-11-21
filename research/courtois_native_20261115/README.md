# Courtois study on the existing native Rust backends

This replaces the historical standalone Python execution path with calls to
the maintained Rust code in `crypto` and `cryptanalysis`. It is a native
integration/correctness study, with performance admission still open.

The current producer records report eight study tests passing, including a
complete solve on the 17-bit fixture, and a separate Rust replay of 256 field
vectors using cryptanalysis's `Field` implementation. These are producer
checks, not independently signed verdicts. The unpublished local claim record
is retained at `evidence/unpublished-native-check-claim.json`.
[pcc:unverified]

## Reused code

| Component | Existing source |
|---|---|
| Word field arithmetic | `crypto:src/cryptanalysis/semaev_decomp.rs`, `Gf2` |
| General binary-curve group law and lifting | `crypto:src/cryptanalysis/koblitz_fast_arith.rs`, `FastBinaryCurve` |
| Binary RREF | `crypto:src/cryptanalysis/gf2_elim.rs`, `rref_counted` |
| Modular relation matrix | `crypto:src/cryptanalysis/ic_boundary.rs`, `IncrementalGauss` |
| Generic rho control | `crypto:src/cryptanalysis/pollard_rho.rs`, `pollard_rho_dlp` |
| Separate point/scalar replay | `crypto:src/binary_ecc/curve.rs` |
| Cross-repository field replay | `cryptanalysis:bindings/rust/hashed-ecdl-tensor/src/binary.rs`, `Field` |

The new code builds the partial-trace constraints, controls the searches, and
connects these components. It does not introduce another field or curve
arithmetic implementation. Addition and scalar replay use the existing general
representation, separately from the word backend used during the search.

## Status against the five workstreams

| Requested work | Current status | Evidence / remaining obligation |
|---|---|---|
| 1. Reproduce and audit splitting | Implemented; producer checks only | Exact pair enumeration, common-divisor target, identity/zero-x branches, computed partial-trace rank |
| 2. Account for the complete attack | Partial | Existing group counters, binary word XORs, row operations/scans/pivot inversions retained; complete calibrated field/bit pricing and admitted cold cost still owed |
| 3. Four/five-point extensions | Implemented; producer checks only | Exact reachable-point-set comparisons for the fixed final base |
| 4. Optimize and compare | Controls implemented; measurement blocked | Plain/cached/symmetry/pair-table variants; no admitted timing ratios |
| 5. Complete solver and rho | Native correctness implemented; producer checks only | Base-log and target recovery, cofactor projection, slow-library replay; strongest matched-rho performance reference and independent gate still owed |

## Reproduce the development checks

Run from `crypto` with its sibling `cryptanalysis` checkout present:

```sh
cargo test --locked --manifest-path research/courtois_native_20261115/Cargo.toml -- --test-threads=1
cargo test --locked --manifest-path research/courtois_native_20261115/replay-cryptanalysis/Cargo.toml -- --nocapture
```

The separate replay package is intentional: it compiles cryptanalysis's actual
Rust crate without substituting the copy that crypto also vendors. Its baseline
manifest declares `getrandom` 0.4 while its source calls the removed
`getrandom::getrandom` function. The companion compatibility change uses
`getrandom::fill` and refreshes its already-stale lockfile. The before/after
build output is preserved in `evidence/`. [pcc:unverified]

The field-vector corpus is frozen in `evidence/field-vectors.json`. To export a
fresh, separate corpus during the unit tests, set `COURTOIS_VECTOR_EXPORT` to a
new path; the test refuses to replace an existing file. The native command
`courtois-native-study check OUTPUT.json` is a correctness producer and records
`performance_eligible: false`; it is not an admitted experiment launcher.

## Versions and evidence

- Crypto baseline: `b42bc6d1ae547d9cbc84d8a512c894db396c89af`.
- Cryptanalysis baseline: `9f467175fd5a0fe8fe6623a8715f9cb80e987305`, plus the
  companion one-line OS-randomness API compatibility edit.
- Toolchain used: Rust 1.100.0. The package lockfiles pin the resolved crates.
- `fixtures.json`: exact frozen public curve/model/generator/subgroup inputs;
  field degree alone is never treated as curve identity.
- `evidence/native-unit-tests-final.txt`: final development test output.
- `evidence/cryptanalysis-field-replay.txt`: separate Rust replay output.
- `evidence/cryptanalysis-build-before.txt` and `...-after.txt`: retained API
  incompatibility and corrected build outputs.
- `evidence/checkpoint-gate.txt`: actual failed S3 checkpoint gate.
- `evidence/root-release-tests-before-inputs.txt`: the initial full-library
  build failure before its tracked fixture inputs were materialized.
- `evidence/root-release-tests-after-inputs.txt`: the retry after materializing
  those inputs. It reached the declared 600-second development limit while
  compiling and returned exit 124; the full library suite has not passed.
- `PROTOCOL.md`: scope, controls, units, explicit bounds, and acceptance criteria.

## Measurement access and interpretation

The repository checkpoint gate failed because this runtime has neither its
configured AWS profile nor a checkpoint upload token. No native performance
experiment was launched. Local unit tests are development work, which the
repository permits without routing them to EKS. [pcc:unverified]

The maintained GitHub Iso Lab workflow was also inspected: it builds ecbench
and the fresh-owner fixture, not this standalone executable. After checkpoint
access is restored, the native study still needs either an ecbench adapter or
a separately admitted native image/command route, complete pricing, and the
matched strong reference. Those are implementation/admission obligations, not
mathematical obstructions. See the [problem record](../../problems/courtois-native-checkpoint-access.md).

`Dockerfile` supplies the two-sibling native build recipe for that custom-command
route. It is an unbuilt development recipe, not a qualified image or evidence
of a submitted job. Freeze the base-image digests, both source commits, binary
hashes and final OCI digest before submitting through the existing admission
path. Both executables run as UID 1000. The current correctness command remains
distinct from an admitted, completely priced performance protocol.

Historical Python timings and assertions of verification remain historical;
they are not native evidence or a PCC standing. This change does not report a
new full-ECDLP complexity theorem or a performance ranking.

## PCC publication state

The sparse checkout initially omitted the tracked PCC ledger. The first local
claim commands therefore initialized an unpublished two-entry ledger rather
than appending to the repository history. That local ledger and both claim
records are preserved in `evidence/unpublished-initial-*`. The exact tracked
baseline was restored and the checks were registered again using the official
tool. During this work, current main's canonical ledger changed its prefix;
the pending local claim and its append entry are retained separately instead
of choosing a side or changing published ledger history. The baseline is
`b42bc6d1ae547d9cbc84d8a512c894db396c89af`. This change makes no canonical PCC
ledger or claim-file update, and does not assert registered or signed standing.

## Merge follow-up

The [2026-11-15 merge review](review-20261115T2234Z/README.md) retains fresh producer checks on both updated PR heads. The compatibility PR's automatic ledger reconciliation rewrote a published event; its auto-merge is disabled until canonical PCC maintainers provide an authorized provenance-preserving route. No frozen evidence or claim is changed. [pcc:unverified]

## Compatibility merged-head follow-up

The [post-merge review](postmerge-20261115T2341Z/README.md) records PR 1572 merged and fresh producer binary/vector checks passing on its final head. Its PCC gate fails because the frozen claim incorrectly labels raw text logs as recompute proofs; a maintainer-authorized versioned correction is required. The final merge also rechained the published ledger event again. The claim, canonical ledger and prior evidence are unchanged by this review. [pcc:unverified]
