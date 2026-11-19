# R00–R99 registry closure audit

Audit snapshot: `2026-11-19T05:42:42Z` on branch
`codex/ghs-rho-parallel-20261118`, Git HEAD
`67e251c8c60bf8e3a9d5bea5da3fc6c480a16bd8`. The worktree was shared and
dirty; the SHA-256 input manifest in `receipts/r00_r99_closure.txt` is the
authority for this audit. All observations below are `[pcc:unverified]`.

## Requested scope and disposition

| Requirement | Disposition | Evidence |
|:--|:--|:--|
| Audit exact R00–R99 uniqueness and contiguity across the six fragments | Verified complete for the frozen input hashes | Native validator reports 100 lanes with spans R00–R16, R17–R33, R34–R50, R51–R66, R67–R83, and R84–R99; exact-set comparison against integers 0 through 99 passed. |
| Require exactly one canonical protocol state per lane | Verified complete | Every lane has one string-valued `state` from the six-value protocol. Counts are 15 `blocked`, 30 `executed-pass`, two `executed-reject`, 12 `implemented-unmeasured`, 22 `queued-after-dependency`, and 19 `screened`. |
| Preserve detailed disposition | Verified complete | Every lane retains a nonempty free-form `status`; the 100 lanes use 51 detailed status strings. `state` is the aggregation field and `status` remains descriptive evidence. |
| Require nonempty evidence and resolve every path/anchor | Verified complete | All 100 evidence arrays are nonempty. All 161 references resolve locally, including Markdown heading anchors. |
| Check the R17/R36 production-integration evidence | Verified present; performance unmeasured | Registry entries and the six integration artifacts listed below resolve and their deterministic receipts report passing controls. No isolated timing, admitted end-to-end comparison, independent replay, PCC decision, or frontier promotion is established. |
| Produce timing or update a leaderboard | Not attempted by design | This closure is a structural/evidence audit. No timing was run and no canonical leaderboard was edited. |

## Whole-registry result

The rustfmt-formatted native validator passed `rustfmt --check`, compiled with
Rust 1.100.0 under `-D warnings`, and exited zero. It reported:

- six fragments and exactly 100 globally unique lanes;
- exact ordered spans `(0,16)`, `(17,33)`, `(34,50)`, `(51,66)`, `(67,83)`,
  and `(84,99)`;
- exact global equality with R00 through R99, with no missing or extra ID;
- schema, claim-scope, required-field, canonical-state, nonempty-status, and
  nonempty-evidence checks passing;
- canonical state counts of 15 blocked, 30 executed-pass, two
  executed-reject, 12 implemented-unmeasured, 22 queued-after-dependency,
  and 19 screened;
- 111 ECBench policy declarations, 21 solver registry entries, nine PDP3
  engines, and four PKM tower engines in the frozen source-contract inputs;
- 161 evidence references and zero unresolved file paths or Markdown anchors.

The validator is fail-closed for malformed or duplicate-key JSON, incorrect
schema or claim scope, invalid or overlapping ranges, out-of-order or
duplicate IDs, missing required fields, a missing/empty/unknown `state`,
empty required strings, mistyped arrays, an empty evidence array, incomplete
exact-set coverage, an unresolved evidence reference, or missing ECBench
contract markers. Its source and complete invocation/output are frozen in the
receipt. Two in-memory negative fixtures also confirmed fail-closed behavior:
an unknown state and an empty evidence array were each rejected with exit
status one.

## State and detailed-status protocol

`state` now supplies the controlled disposition vocabulary:

| Canonical state | Lanes |
|:--|--:|
| `blocked` | 15 |
| `executed-pass` | 30 |
| `executed-reject` | 2 |
| `implemented-unmeasured` | 12 |
| `queued-after-dependency` | 22 |
| `screened` | 19 |
| **Total** | **100** |

The 51 free-form `status` values are intentionally retained. They record the
specific control outcome or open gate without being used as the canonical
aggregation key.

## Evidence closure

The six formerly evidence-empty R34–R50 lanes now have the following
references, and the native path/anchor resolver accepts each one:

| ID | Canonical state | Evidence |
|:--|:--|:--|
| R40 | `queued-after-dependency` | `ranges/R34_R50.md#registry` |
| R41 | `queued-after-dependency` | `ranges/R34_R50.md#registry` |
| R42 | `queued-after-dependency` | `ranges/R34_R50.md#registry` |
| R43 | `blocked` | `ranges/R34_R50.md#integration-order`; `prototypes/r34_50.rs`; `receipts/r34_50.txt` |
| R45 | `queued-after-dependency` | `ranges/R34_R50.md#registry`; `research/quartic_ghs_strong_rho_20261114/strong_rho.rs` |
| R46 | `blocked` | `ranges/R34_R50.md#registry`; `research/quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md#13-r9--projective-or-x-only-redesign-screen` |

“Resolved” means the frozen local file exists and, where supplied, the
Markdown heading anchor resolves. It is not independent validation of the
evidence contents.

## R17/R36 integration closure

R17–R20, R33, R36, and R50 all have canonical state
`implemented-unmeasured`. Their detailed statuses preserve the distinctions
between production control, dispatch, counter-contract, and composition
results.

The core integration evidence and the added production/analyzer reviews
resolve:

- `R17_PRODUCTION.md`;
- `R36_PRODUCTION.md`;
- `receipts/r17_r36_q223.jsonl`;
- `receipts/r17_r36_validation.txt`;
- `receipts/r36_q223.jsonl`;
- `receipts/r36_validation.txt`;
- `ranges/R17_PRODUCTION_REVIEW.md`;
- `ranges/R17_NATIVE_ANALYZER.md`;
- `receipts/r17_native_analyzer.txt`;
- `ranges/R36_PRODUCTION_REVIEW.md`;
- `receipts/r36_production_review.txt`.

The retained receipts report successful standalone and production controls,
q=223 recovery/trajectory equality, and expected counter identities. The R17
record also retains the initial E0063 build failure and excludes the
subsequent stale-binary output before documenting a fail-fast rerun. These are
producer-side deterministic controls only.

R16 is canonically `blocked`: its detailed status records that independent
content validation passed while source binding and signed standing remain
blocked. This prevents the content check from being misreported as an
independently source-bound promotion result.

The open R17/R36 gates remain:

- integration of the standalone native `fq_small_div` authority checker into
  the canonical analyzer and publication gate;
- complete modeled-cost treatment and frozen sensitivity/pricing for exact
  small divisions;
- admitted source-bound isolated A/A and ABAB measurement;
- independent replay, validator, matched-reference, and PCC standing;
- leaderboard publication only after those gates and the benchmark-record
  contract pass.

## Closure boundary

This audit establishes exact registry coverage, canonical state validity,
nonempty evidence coverage, and local reference integrity at the listed
hashes. No registry-level structural or reference gap remains in this
snapshot. It does not establish an end-to-end speedup, a rho/IC frontier
change, external-tool equality, statistical admission, or a new leaderboard
result `[pcc:unverified]`.
