# Local audit checks, 2026-11-16

All assertions below remain `[pcc:unverified]` until an independent signed
repository verifier accepts them.

| Check | Observed result |
|---|---|
| Original raw SHA-256 | Exact match, e0f7050d...efa32f, 78,049 bytes |
| Hash-checked original Rust source assembly | Four required source digests match |
| Full original replay | 144/144 rows; 36 per c/v/w/R; zero retained-counter mismatches; all certified |
| Independent new Rust scalar arithmetic on original replay | 144/144 checks pass, including nonzero r-torsion witnesses |
| Historical validator dumps, new Rust arithmetic | 105 main-dump and 24 follow-up-dump checks pass; repeated instances are not distinct targets |
| Complete p8209/cs363/seed1/w relation-check replay | 3,927 relations, 169,325 trials; zero bad relations; zero shared-counter mismatches |
| New independent relation arithmetic | All 3,927 accepted relations pass place, split-sum and caG+cbT checks |
| Original pricing recalculation | 0.528398 at w=1; 0.098959 at w=0.1 |
| Follow-up recalculation | Q1/Q3/Q4 at w=1: 0.468526 / 0.141821 / 0.123273 |
| Recorded calibration recalculation | 0.007687 at recorded rates; 0.048648 at ten times those rates; incomplete timing boundary disclosed |
| Transfer/order arithmetic | Nine independent r-primality and unique-multiple Hasse-Weil checks; same-field odd-prime source homomorphisms trivial |
| Window-free plain rho | First case passes: p8209/cs363/ts363/seed1; 38,001,348 steps, 38,006,519 group ops; counts equal registered plain rho, zero window compares; remaining cases recorded in final addendum |
| Focused release build/check/test | Pass; one new independent arithmetic test, no root-workspace pass asserted |

Original replay completed in 608.628 shared-host wall seconds, 1707.513 user
CPU seconds and 0.234 system CPU seconds. These are execution practicality
receipts, not a wall-clock solver speedup. Three replay workers. Local host
reports AMD EPYC 9V74; its physical host/core isolation is not established.

The focused crate was rebuilt after the replay to add stronger certificate
input guards and recorded-calibration recalculation. Full replay binary hash:
`99a27fac3a8d826ee01c962fcdb1877e43b2f7fa09066c2955a65b34e8669af8`.
Original production slices and the replay algorithm are unchanged. The later
auditor source hashes are retained in `auditor-source.sha256`.

Errors and deviations retained: `/usr/bin/time` unavailable, so the initial
resource-wrapper command exited 127 before starting any solver. It was
replaced by Bash time and separate `/proc/self/status` RSS receipts. A
follow-up input filter initially selected zero runs due to a different arm
name; corrected to `R_rho_plain` before the case was counted. A first audit
calculation looked for the wrong compares field; corrected to
`compares_uncharged`, and recalculated before reporting. Missing original
compare counters now cause the Q3/Q4 reading to be omitted, not treated as
zero. No frozen producer file was edited.

The final tail of the original replay overlapped with the added three-thread
window-free follow-up check (at most six workers). This exceeded the original
plan's three-worker aggregate limit. Only deterministic counts are interpreted;
no isolation or timing claim is made from the overlap. The source/hash and
count-matching acceptance conditions were retained.

The follow-up 17-case command exited 124 at its 900-second bound after
16 matching rows. The first separately run case makes 17/18 complete at that
point. No completed row is excluded. The longest reference case,
p8209/cs393/target1000396/seed1 (198,717,560 steps), is retried separately
under the explicit 480-second single-worker protocol amendment. Its result
or censoring is reported in the final addendum. The earlier timeout remains
in the evidence and is not counted as a solver failure or a mathematical
counterexample.

## Final follow-up addendum

The single-case retry exited 0 within its 480-second bound. Its 198,717,560
steps and 198,742,318 group operations equal the registered plain-rho row;
window compares are zero. Combining the separate first case, the 16
completed rows of the censored attempt and this retry gives **18/18** unique
curve/target/seed cases with zero retained-counter mismatches and zero
window compares. All 18 recovered scalars pass the separate new Rust
arithmetic checker (`lean-rho-combined-certificates.json`). The censored
attempt is retained as an execution/resource event. Peak RSS of the retry:
2,756 KiB. No running audit job remains.

This makes the aggregate window-free sensitivities exact readings of
replayed counts for the whole follow-up panel. They still omit other native
work as disclosed in REPORT.md, so they do not establish a fully calibrated
rho lower cost or a universal absence of a Jacobian IC advantage.
