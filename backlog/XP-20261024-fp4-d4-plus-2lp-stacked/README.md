# Stacked D4 symmetrisation plus double large primes on E(F_{p^4}), end to end (launch card)

Status: **draft**. The arm is not built (see "Open inputs"); its completion rule is an open design choice that the sparse-LA null made decisive.

Question: on E(F_{p^4}), does the stacked arm (D4 symmetrised oracle plus two-large-prime relations) lower the whole-solve cost S below the plain D4 arm, with a fitted exponent in #E that is separated from D4's?

## Sources (every number cited)

- [research/ic_next10_20261014/01-fp4-ladder/README.md](../../research/ic_next10_20261014/01-fp4-ladder/README.md): the ladder. Sizes p in {101, 181, 269, 397, 521, 769, 1033}, log2 #E from 26.6 to 40.1, three curves per size (seeds 1 to 3), 21 curves. Whole-cost fits (21 curves, OLS of log2 cost on log2 #E): plain 0.238 [0.212, 0.262], D4 0.217 [0.192, 0.247], DLP 0.333 [0.313, 0.353]; linear algebra exponents plain 0.495, D4 0.504, DLP 0.420 [0.405, 0.440]. The 2LP arm's whole solve is 2.31 times plain (ecbench, 4 curves at p = 101 to 397, CI [2.10, 2.62], session ECBS1hf49e5346dc2b). DLP is 1.8 to 4.5 times plain across the ladder and the gap widens.
- [research/gaudry_quartic_d4_ecbench_20261013/README.md](../../research/gaudry_quartic_d4_ecbench_20261013/README.md): the D4 solve on E(F_{29^4}), one curve, S = 7.17e8 (6.49e8 with the Y = 0 amendment); the README reports no 18.2 figure.
- The plain-to-D4 ratio of 18.2 used in REVIEW.md M3 is not stated in either README. It equals 1/0.0548, where 0.0548 [0.048, 0.061] is ic-k4-d4 / ic-k4-plain in the ecbench table of 01-fp4-ladder/README.md. Treat 18.2 as derived, not as a recorded figure.
- [research/sparse_la_timing_20261018/README.md](../../research/sparse_la_timing_20261018/README.md): the sparse null. Deferred sparse solvers cost 1.206 times baseline in total on S1 (1.398 times on one workload). The cause is the completion rule: the solvers answer only at full column rank, while structured-gauss and incremental-gauss stop when the target column is pinned. Full-rank arms draw 1.75 times more relations on S1 workload 2 and 5.5 to 10.5 times more on S2. The sparse-cado stage alone counts 1.47 to 1.97 times the row operations of its full-rank matrix.

## Design

Arms on the ladder sizes p in {101, 181, 269, 397, 521, 769, 1033} (3 curves each, seeds 1 to 3, one planted target each, shared by all arms):

1. plain (reference), 2. D4 (reference, the cheapest recorded arm), 3. DLP (2LP, reference), 4. **stacked** (D4 oracle with two-large-prime relations), 5. dlp-null (control: DLP code path with F' = F, must reproduce plain).

Matrix: `sparse-cado` (= filter+wiedemann in ecbench). The completion rule is a pre-registered parameter with two values, and the protocol must name one before the run: (a) stop at full rank, (b) stop when the target column is pinned. The card runs both on the stacked arm and reports which one the plain D4 arm is compared with. Rule (a) is the one the sparse null blamed, so it is an explicit arm, not a default.

## Decision rule

Whole-solve S of the stacked arm against plain D4, at each of the ladder sizes (at least 4 sizes), with the ratio's bootstrap CI over curves and a fitted exponent in #E with CI for both arms.

- **Gain** over D4 only if the fitted whole-solve exponent of the stacked arm is below D4's 0.217 with CIs separated, and the ratio stacked/D4 is below 1 with CI excluding 1 at the largest size.
- **No gain** if the ratio CI covers 1 at the largest size, or the exponent CIs overlap.
- **Loss** if the ratio is above 1 with CI excluding 1 at the largest size.
- The dlp-null must reproduce plain exactly, or the run is void. The D4 positive control (6 of 6 oracle matches in the ladder) must reproduce.

Relevance: DLP lowers the linear-algebra exponent (0.420 against plain 0.495) but its whole-solve exponent is higher (0.333 against plain 0.238), so the whole-solve gain has to come from elsewhere. Stacking is the only unrun combination in the regime with a proven sub-rho exponent (REVIEW.md M3).

## Frozen inputs (pinned vs open)

| input | status |
|:--|:--|
| ladder sizes, seeds, arms | pinned: 01-fp4-ladder/README.md |
| D4 oracle cost C4(D4) = 2.54e10 F_p mults | pinned: 01-fp4-ladder/README.md |
| plain oracle cost C4(S4) = 1.213e12 F_p mults | pinned: same |
| LA charge 16 F_p per multiplication mod r | pinned: same |
| ecbench spec file and invocation for the ladder | **open**: not located in the sources read |
| stacked arm implementation | **open**: no code for D4 plus DLP was found in the sources read |
| completion rule | **open**: a design choice to be named in PROTOCOL.md |
| code commit | **open**: pin at launch |

## Cost estimate (ESTIMATE, not measured)

Plain S4 solve: 25 min and 3.7 GB per solve (01-fp4-ladder/README.md). D4 solve: 34 to 49 s single-threaded. The ladder ran 700,701 oracle trials. A stacked-arm run at 4 to 7 sizes with 3 curves each is dominated by the solver cost and is not derivable from these numbers; the protocol must state it after a 1-curve pilot.

## Open inputs

1. Stacked-arm code (D4 oracle with DLP graph) in ecbench.
2. The ecbench invocation and spec file for the ladder.
3. The completion-rule choice in PROTOCOL.md.
4. A pilot curve for the cost estimate.
