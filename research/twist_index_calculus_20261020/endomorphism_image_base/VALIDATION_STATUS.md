# Validation status of the twist image-base control study

![Verified relation-finding stages](figures/mechanism.svg)

Scientific results and repository software gates are separate obligations.
The following records preserve failures and interruptions as well as passes.

## Scientific verification

| Requirement | Recorded outcome |
| --- | --- |
| Frozen input integrity | Exact SHA-256 checked before execution |
| Original 189 cases and deterministic A/A | All exits 0, byte-identical non-timing trees |
| Doubling control | All 21 cases, reference and polynomial first/A/A complete |
| Independent signed sets, matrices and recovery | 210 passing audits; 189 full-rank target/factor recoveries and 21 deliberately inactive-column censored controls |
| Frozen baseline parity | All 621 queries of seven old S4 prefixes equal in sets, rank and system count |
| Negative controls | Omitted relation, changed input hash, corrupt eigenvalue, changed coefficient and false Lean theorem rejected |
| Image coordinate identity | Exact integer remainder zero, IDC1h7e0104841476f369; pinned Lean 4.34.1 conditional theorem and positive axiom audit |
| Cold resources and phase sums | 105 complete same-origin eleven-execution timed rounds plus 630 non-timing records; all 210 cost rows admitted, all deterministic traces and exact sums checked |
| Resume control | Complete units preserved, incomplete unit rejected, partial inputs retained, copied bytes unchanged |
| External complete GB | 378 fixed systems × eight passes = 3,024 equal bases; raw input and S-pair reductions check every pair |

These are known-answer fixtures with scalar 271. Source auditing excludes
use of that scalar by the candidate; physical isolation of the scalar input
is not established. The supplied public coordinate encodings are used for
alignment while every required lift is recomputed and charged.

## Software gates and retained failures

The final serial native suite passes **57/57** with one Rayon and one test
thread. An earlier parallel run passes 56/57: unchanged shared F4 test
`hilbert_stop_gives_the_same_reduced_basis_as_the_plain_run` reports
`cheaper=19`, below threshold 20. The failure is retained without weakening
the test. This study does not edit `src/cryptanalysis/f4_fp.rs`.

The local parallel library run passes 4,482 tests, fails six in unchanged
`jv_cover`, and ignores 119. Its serial continuation was interrupted by
migration; no pass is credited. First managed gate
`20261023-200308-twist-image-release-gate-854444` fails in LLVM compilation
under the 8 GiB address limit; no test executed.

Replacement `20261023-211022-twist-image-gate-v2-5c8a60` builds separately,
then runs serial tests under the original 8 GiB / 1,200 s limit. All six
compile/test exits are zero: **4,488 library tests passed, zero failed,
119 ignored; examples passed 1 and 3 tests; all 57 native tests passed**.
Library execution took 977.90 seconds; the job completed at 21:54 UTC on
2026-10-23. Saved receipts and logs establish these outcomes after migration;
tests were not repeated. Compilation is not performance evidence and
changes no scientific arm's frozen source or resource envelope.

PR head `c7cecbd07f437b600b5893be61ad0eedf38d2368` fails Clippy's
`manual_is_multiple_of` in untouched `curve_traits/walk.rs` and
`prime_small_x.rs`; QuarticTwistSymmetry Lean CI fails toolchain extraction.
Logs are retained; neither check is bypassed or credited as passing.
The merge conflict was referred to Copilot. Every later head requires
its own review and CI.

## Missing claim gates

No complete GAE operation price, 0/0.1/1 GAE band, protocol-matched strongest
rho reference, independent sampling of target scalars, or growing-size
inference follows from these fixture timings. Magma, FGb, Singular and
OpenF4 were not reproduced. msolve is generally faster on the GB component;
it has not been integrated and priced as an external complete DLP solver.

Accordingly this directory has no asserted `VALIDATION.json` and contributes
no new row to the gated leaderboard or benchmark index. Honest finite
positive and negative observations remain in the report with their exact
boundary. See [the unresolved problem](../../../problems/quartic-twist-polynomial-ic-pricing.md)
and [the raw evidence](ARTIFACTS.md).
