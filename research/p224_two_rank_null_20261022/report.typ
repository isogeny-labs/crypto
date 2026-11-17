#set page(paper: "a4", margin: 14mm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt + luma(170), inset: 3pt)
#show heading: set block(above: 1.1em, below: 0.6em)
#show table: set text(size: 7.5pt)

= Is a class-group 2-rank of 6 unusual for a prime-order curve's CM discriminant? (P-224)

Date 2026-10-22. Frozen protocol: `PROTOCOL.md` (committed before the run; SHA-256 `7b857376a897560b30ed964b0ddc7e064798227969b4db70475a5b310268090e`). Code: `src/cryptanalysis/two_rank_null.rs`, `examples/p224_two_rank_null.rs`. Raw rows: `run/rows.jsonl` (6822 rows); everything below is recomputed from them by `analyze`. Seed 20261022, trial bound $10^6$, Pollard-Brent budget $2^21$ iterations, 4 threads, 54.6 min wall, 127 CPU-min (estimate: 60 min wall). This is a structural statistic, not an ECDLP cost measurement: no rho ratio, no scoreboard row, no improvement of the index-calculus baseline.

== Answer (frozen decision rule, measured)

*P-224's $omega(D_K) = 7$ (2-rank 6) is within the control distribution.* Against random 224-bit primes (R224u, N = 1000) 12.3% of controls have $omega >= 7$ (Clopper-Pearson 95%: 10.3% to 14.5%); the mid-rank percentile of 7 is 91.5% (control median 4, 2.5% to 97.5%: 2 to 8). Against the fixed P-224 prime (F224u) 14.5% (12.4% to 16.8%) of controls reach 7, mid-rank 90.0%. Two-sided add-one $p$ (minimum over censoring versions) 0.136 and 0.162, against the corrected threshold $alpha/m = 0.005$. Effect size: P-224 is 3 above the control median of 4 (2 above the fixed-prime median of 5) (null mean 4.47).
The part of $omega$ made of primes $<= 10^6$ ($omega_"small"$, never censored) is 5 for P-224 against a control median of 2 (95.5th mid-rank percentile in R224u, upper tail 6.9%, 5.4% to 8.7%); also within the control distribution at the corrected threshold ($p = 0.140$, and $0.162$ for F224u).

*P-256 is inconclusive under the frozen instrument* (its 258-bit discriminant leaves a 215-bit unsplit cofactor: $omega in [4, 13]$), and its $omega_"small"$ (2) is at the control median (50.2%). The published factorisation (5 primes) puts $omega = 5$ at a point 48.9% of R256u controls reach or exceed on $omega_"mid"$ (post hoc, below).

Class of each statement: $omega$ values of P-224, brainpoolP224r1, brainpoolP256r1, secp224k1, secp256k1: exact computation (complete factorisations, product and primality checked by the code). Percentiles and verdicts: comparative, from this controlled experiment. The 2-rank identity: theorem (genus theory).

#figure(image("run/omega_null.svg", width: 100%), caption: [Null distribution of $omega(D_K)$ for the three uniform-trace primary arms, N = 1000 each (run `20261022`). Grey bars: $omega_"mid"$; blue and red lines: the rigorous lower ($omega_"lo"$) and upper ($omega_"hi"$) versions. Dashed: targets (P-256 is at the midpoint of its bound 4 to 13). Source rows: `run/rows.jsonl`.])

== Null arms and censoring

Censoring is the fraction of samples whose discriminant keeps an unsplit composite cofactor after the shared budget. Those samples were kept and bounded, never dropped.

#table(columns: (auto, auto, auto, auto, auto, auto), align: left,
[arm], [N], [censored (95% CP)], [fully factored], [$omega_"mid"$ median [2.5%, 97.5%]], [$omega_"small"$ median [2.5%, 97.5%]],
[R224u], [1000], [577 = 57.7% (54.6 to 60.8)], [423], [4 [2, 8]], [2 [0, 5]],
[R224s], [1000], [596 = 59.6% (56.5 to 62.7)], [404], [4 [2, 8]], [2 [0, 5]],
[F224u], [1000], [611 = 61.1% (58.0 to 64.1)], [389], [5 [2, 8]], [2 [0, 6]],
[F224s], [1000], [597 = 59.7% (56.6 to 62.8)], [403], [5 [2, 8]], [2 [0, 6]],
[R256u], [1000], [658 = 65.8% (62.8 to 68.7)], [342], [4 [2, 8]], [2 [0, 5]],
[R256s], [1000], [663 = 66.3% (63.3 to 69.2)], [337], [4 [2, 8]], [2 [0, 5]],
[R384u], [100], [80 = 80.0% (70.8 to 87.3)], [20], [5 [2, 9]], [2 [0, 6]],
[R521u], [100], [85 = 85.0% (76.5 to 91.4)], [15], [5 [2, 9]], [2 [0, 6]],
)

The rigorous upper bound $omega_"hi"$ is loose (median 11 to 13 for 224- and 256-bit arms), so a verdict of "unusual" on a censored metric is only reachable for extreme targets; "within the control distribution" is decided by the favourable bound $p_min$. No overlap flag was raised in any arm. Samples never resampled or dropped: 0.

== Pre-registered family (uniform trace, $alpha/m = 0.005$, $m = 10$)

#table(columns: (auto, auto, auto, auto, auto, auto, auto, auto), align: left,
[target], [null], [metric], [value], [control median [2.5%, 97.5%]], [mid-rank pct; upper tail (95% CP)], [$p$ min / mid / max], [verdict],
[P-224], [R224u], [$omega$], [7], [4 [2, 8]], [91.5%; 12.3% (10.3 to 14.5)], [0.136 / 0.248 / 1.000], [within the control distribution],
[P-224], [R224u], [$omega_"small"$], [5], [2 [0, 5]], [95.5%; 6.9% (5.4 to 8.7)], [0.140], [within the control distribution],
[P-224], [F224u], [$omega$], [7], [5 [2, 8]], [90.0%; 14.5% (12.4 to 16.8)], [0.162 / 0.292 / 1.000], [within the control distribution],
[P-224], [F224u], [$omega_"small"$], [5], [2 [0, 6]], [94.5%; 8.0% (6.4 to 9.9)], [0.162], [within the control distribution],
[P-256], [R256u], [$omega$], [4 to 13], [4 [2, 8]], [62.2%; 48.9% (45.8 to 52.0)], [0.002 / 0.979 / 1.000], [inconclusive: censoring],
[P-256], [R256u], [$omega_"small"$], [2], [2 [0, 5]], [50.2%; 63.5% (60.4 to 66.5)], [1.000], [within the control distribution],
[brainpoolP224r1], [R224u], [$omega$], [5], [4 [2, 8]], [64.8%; 45.6% (42.5 to 48.7)], [0.651 / 0.913 / 1.000], [within the control distribution],
[brainpoolP224r1], [R224u], [$omega_"small"$], [3], [2 [0, 5]], [73.2%; 36.2% (33.2 to 39.3)], [0.725], [within the control distribution],
[brainpoolP256r1], [R256u], [$omega$], [4], [4 [2, 8]], [39.6%; 71.9% (69.0 to 74.7)], [0.448 / 1.000 / 1.000], [within the control distribution],
[brainpoolP256r1], [R256u], [$omega_"small"$], [1], [2 [0, 5]], [23.5%; 89.5% (87.4 to 91.3)], [0.731], [within the control distribution],
)

Sensitivity (semicircle-weighted trace, not in $m$): P-224 mid-rank 92.5% (R224s) and 88.8% (F224s) on $omega$, 96.3% and 94.6% on $omega_"small"$, all within the control distribution (minimum $p$ 0.120 to 0.200); P-256 inconclusive, brainpool within, the same as the uniform arms. The conclusion does not depend on the trace scheme. Full tables: `run/summary.md`.

== Descriptive comparison points (outside the tested family; no verdict)

#table(columns: (auto, auto, auto, auto), align: left,
[target], [measured under the same instrument], [null], [note],
[P-384], [$omega in [2, 19]$, $omega_"small" = 0$, unsplit cofactor 362 bits], [R384u, N = 100], [99% of controls have $omega_"hi" >= 2$; no percentile resolvable (power check fails)],
[brainpoolP384r1], [$omega in [3, 18]$, $omega_"small" = 1$, unsplit 326 bits], [R384u], [as above],
[P-521], [$omega in [2, 27]$, $omega_"small" = 1$, unsplit 521 bits], [R521u, N = 100], [as above],
[secp224k1, secp256k1], [$omega = 1$ (exactly)], [R224u, R256u], [$j = 0$: $Delta = -3 v^2$, $D_K = -3$ by CM theory. Not a member of the generic null; 4.7% and 4.6% of controls have $omega_"lo" <= 1$ only because the control population has no $j = 0$ curves. No percentile is claimed.],
)

== Positive control (run first): passed

(a) Population: 300 fresh R224u samples with $k$ planted small primes through the same code path, Mann-Whitney against the R224u null. $k = 2$: mean $omega_"mid"$ shift +2.05, $z = 15.9$, $p = 8 times 10^(-57)$. $k = 4$: shift +4.05, $z = 23.8$, $p = 4 times 10^(-125)$. Both detected (rule: $p < 0.001$ and shift within 0.5 of $k$).
(b) Target sweep: P-224's discriminant times the first $k$ primes $>= 5$ not dividing it. On $omega_"small"$ the detector flags (corrected $p_max < 0.005$) from $k = 3$ ($omega_"small" = 8$); on the censored $omega$ it flags from $k = 9$ ($omega = 16$, $p_max = 0.004$; $k = 3$ to $8$ are "inconclusive: censoring" because $p_max$ reaches 1.0 to 0.022 while $p_min = 0.002$). Pre-registered rule (flagged at some $k <= 10$ on both metrics): *met*. Reading: the instrument flags a planted excess of 9 primes on the censored metric (planted $omega$ = 16) and of 3 on the exact one; an excess of 3 over the control median on the censored $omega$ (P-224's actual excess) would not be flagged at this $N$ by the rigorous bound, which is what "within the control distribution" with $p_min = 0.14$ rests on, not a claim of equivalence.

== Observations outside the frozen rule (post hoc, labelled, not part of any verdict)

+ P-256 with its published factorisation ($omega = 5$, factors in `research/p224_class_number_20261022/PROTOCOL.md`): 48.9% of R256u controls have $omega_"mid" >= 5$ (unequal instrument: published factors for the target, budgeted rho for the controls, so no verdict).
+ Conditioning on being fully factored by the instrument (as P-224 is): 33 of 423 R224u controls (7.8%) have $omega >= 7$. Complete factorability selects smoother, higher-$omega$ numbers, so this is the like-for-like upper-tail fraction for a P-224-like input; P-224 sits at the 92nd percentile of that subpopulation, not beyond it.

== Scope: what this does not show

- The null is not a point-counted sample. $N$ is drawn uniformly (and, as sensitivity, semicircle-weighted) from the Hasse interval subject to primality, with random or fixed $p$; `random b` curves were not generated because point counting is unavailable. Class-number weighting of traces is not modelled.
- $omega$ of 58 to 66% of controls is bounded, not known. Verdicts on $omega$ rest on the pre-registered bound rule; the favourable bound ($p_min$) carries every "within" verdict.
- P-256, P-384, P-521 and brainpoolP384r1 have censored discriminants under this instrument, so no percentile claim is made for them (P-256 inconclusive; the others descriptive).
- The study is about the count of distinct primes (the genus-theory 2-rank). It says nothing about the 4-rank, the size of the class group, or the hardness of any discrete-logarithm problem. "Within the control distribution" is not "random" and not "secure".
- No earlier claim is weakened. PR 2408's exact value (2-rank 6 for P-224, 4 for P-256, from complete published factorisations) is unchanged; this report only adds that 6 is not extreme for the population.
