#set page(paper: "us-letter", margin: (x: 0.45in, y: 0.40in), numbering: "1 / 1")
#set text(font: "DejaVu Sans", size: 7.6pt, fill: rgb("#111827"))
#set par(leading: 0.41em, justify: true)
#show heading.where(level: 1): set text(size: 11.0pt, weight: "bold")
#show heading.where(level: 2): set text(size: 8.9pt, weight: "bold", fill: rgb("#1e3a8a"))

= Multi-Round Algorithmic Hillclimb Until Frontier Stall: Graph-Anchored Extension ECDLP

*Protocol:* `EXT-IC-GRAPH-HILLCLIMB-STALL-20261126` (`protocol.json`, sealed in `pre-call.sha256` prior to execution) #h(1fr) *Standing:* `[pcc:unverified]` \
*Scope:* 16-round falsification-driven hillclimb (`R0..R15`) across Phase-1 Sieve/Graph, Phase-2 Sparse LA, and Phase-3 Traced Multi-Pattern Descent on the Joux--Vitse genus-3 Weil cover over $FF_(p^6)$ (`foreign_hook_calls = 0`, all artifacts $< 60$ KiB) (`[pcc:unverified]`).

== 1. Experimental Design, Promotion Gate & Sub-Phase Bottleneck Diagnosis

Starting from the Round-1 `signed-matroid-2lp:traced-anchored` baseline (`R1`, `mean4 = 1,925.05G` bitops across frozen seeds `{389, 101, 211, 307}` at $p = 1009$; `Run50 = 2,064.45G`), profiling exact `bitop/v1` sub-totals revealed three coupled bottlenecks (`[pcc:unverified]`): (1) Phase-1 `relations` (`1,133.25G`, `58.9%`) was dominated by `3|cal(B)|` Montgomery batch-inversion multiplications, degree-8 forward differences, and Horner Frobenius compositions `compose_mod`; (2) Phase-2 `linear_algebra` (`447.35G`, `23.2%`) re-executed full 3-checkpoint Randomized Lanczos solves with `40,542`-bitop `ModrMul` modular multiplications on tiny integer entries; and (3) Phase-3 `descent` (`343.68G`, `17.9%`) replayed one redundant Macaulay degree step under `StopMode::Staircase(64)` across all `~467` cold-traced F4 calls (`[pcc:unverified]`).

Under `EXT-IC-GRAPH-HILLCLIMB-STALL-20261126`, each round modifies one explicit subsystem against the reigning champion and promotes *iff* all 4 seeds recover $[d]G = Q$ (`foreign_hook_calls = 0`) and `mean4` `observed_bitops` drops by $>= 1.00$%; rejected candidates (`STALL`) are reverted before testing the next hypothesis until 5 consecutive orthogonal attempts stall (`[pcc:unverified]`).

#figure(
  image("figures/hillclimb_trajectory.svg", width: 93%),
  caption: [16-round `p = 1009` 4-seed mean `bitop/v1` trajectory (`R0..R15`), showing 8 promoted advances beyond `R1`, an intermediate falsification (`R6`), and a 5-round frontier stall block (`R11..R15`) (`[pcc:unverified]`).],
)

== 2. Complete 16-Round Hillclimb Trajectory (`p = 1009`, Seeds `{389, 101, 211, 307}`)

#table(
  columns: (0.42fr, 0.44fr, 2.02fr, 0.72fr, 0.68fr, 0.62fr, 0.65fr, 0.72fr, 0.62fr, 0.56fr, 0.78fr),
  inset: 2.8pt,
  align: (left, left, left, right, right, right, right, right, right, right, center),
  stroke: 0.4pt + rgb("#d1d5db"),
  [*ID*], [*Base*], [*Candidate Mechanism*], [*Run50 (G)*], [*Rel4 (G)*], [*LA4 (G)*], [*Desc4 (G)*], [*Mean4 (G)*], [*$Delta$ (%)*], [*vs R1*], [*Verdict*],
  [`R0`], [`--`], [Deferred 2-LP + `(1^6)` + Lanczos], [`6,718.20`], [`1,897.78`], [`444.82`], [`1,749.58`], [`4,092.95`], [`+0.00`], [`0.47x`], [`BASELINE`],
  [`R1`], [`R0`], [Matroid 2-LP + TracedMP + Lanczos], [`2,064.45`], [`1,133.25`], [`447.35`], [`343.68`], [`1,925.05`], [`-52.97`], [`1.00x`], [`PROMOTED`],
  [`R2`], [`R1`], [+ Online Markowitz SGE LA], [`1,904.90`], [`1,133.25`], [`269.32`], [`343.68`], [`1,747.02`], [`-9.25`], [`1.10x`], [`PROMOTED`],
  [`R3`], [`R2`], [+ Partition `m=9` + InvTable Sieve], [`1,865.46`], [`1,094.06`], [`269.32`], [`343.68`], [`1,707.83`], [`-2.24`], [`1.13x`], [`PROMOTED`],
  [`R4`], [`R3`], [+ Stickelberger Stack 2-LP Split], [`1,845.69`], [`1,074.54`], [`269.32`], [`343.68`], [`1,688.31`], [`-1.14`], [`1.14x`], [`PROMOTED`],
  [`R5`], [`R4`], [+ `4096`-Incidence / `64`-SubRank / FastSextic], [`1,794.95`], [`1,042.76`], [`251.29`], [`343.36`], [`1,638.19`], [`-2.97`], [`1.18x`], [`PROMOTED`],
  [`R6`], [`R5`], [? Flat 5-Var $d_0$ Elim (`6->5` Eqs)], [`1,811.52`], [`1,042.76`], [`251.29`], [`355.79`], [`1,650.62`], [`+0.76`], [`1.17x`], [`STALL #1`],
  [`R7`], [`R5`], [+ `Hilbert(8)` Early F4 Stop], [`1,635.39`], [`1,042.76`], [`251.29`], [`230.94`], [`1,525.77`], [`-6.86`], [`1.26x`], [`PROMOTED`],
  [`R8`], [`R7`], [+ AddChain Scalar Wiedemann LA], [`1,413.75`], [`1,043.19`], [`43.35`], [`230.94`], [`1,318.25`], [`-13.60`], [`1.46x`], [`PROMOTED`],
  [`R9`], [`R8`], [+ Zero-Mul `inv_2w_sq` + Deg-4 $A_1'$ Sieve], [`1,203.92`], [`833.66`], [`43.35`], [`230.94`], [`1,108.72`], [`-15.89`], [`1.74x`], [`PROMOTED`],
  [*`R10`*], [*`R9`*], [*+ `FrobBasis` + Stickelberger $G$-Screen*], [*`1,141.61`*], [*`770.48`*], [*`43.35`*], [*`230.94`*], [*`1,045.54`*], [*`-5.70`*], [*`1.84x`*], [*`CHAMPION`*],
  [`R11`], [`R10`], [? Post-Rank `+1,024 B` Anchor Margin], [`1,146.57`], [`775.55`], [`43.35`], [`230.94`], [`1,050.61`], [`+0.48`], [`1.83x`], [`STALL #1`],
  [`R12`], [`R10`], [? Post-Rank `+3,072 B` Anchor Margin], [`1,156.92`], [`785.83`], [`43.35`], [`227.05`], [`1,057.01`], [`+1.10`], [`1.82x`], [`STALL #2`],
  [`R13`], [`R10`], [? Conditional `3-LP` on `2-LP` Lines], [`1,207.75`], [`836.00`], [`43.68`], [`230.94`], [`1,111.40`], [`+6.30`], [`1.73x`], [`STALL #3`],
  [`R14`], [`R10`], [? Sub-Sampled `92%` Small Factor Base], [`3,209.90`], [`3,259.63`], [`0.45`], [`0.00`], [`3,260.80`], [`+211.88`], [`0.59x`], [`STALL #4`],
  [`R15`], [`R10`], [? Narrow `512`-Incidence / `16`-SubRank], [`1,193.35`], [`822.38`], [`43.35`], [`230.94`], [`1,097.44`], [`+4.96`], [`1.75x`], [`STALL #5`],
)

#pagebreak()

== 3. Algorithmic Mechanisms Behind Promoted Breakthroughs & Falsified Stalls

#figure(
  image("figures/phase_waterfall.svg", width: 92%),
  caption: [Phase-decomposed `bitop/v1` waterfall across `R0`, `R1`, and all 8 promoted hillclimb steps (`R2..R5, R7..R10`) (`[pcc:unverified]`).],
)

*Promoted Advances (`R2..R5, R7..R10`, cumulative `1.841x` beyond `R1`, `3.915x` beyond `R0`, `[pcc:unverified]`):*
- *Phase-2 LA (`447.35G -> 251.29G -> 43.35G`, `10.32x` speedup):* `R2` (`SparseMarkowitzTracker`) replaced 3x Lanczos restarts with incremental minimum-Markowitz SGE (`-39.8%` LA), exposing fill-in inside a `~300 x 300` dense core. `R8` (`WiedemannTracker`) replaced Gaussian elimination altogether with *factored Scalar Wiedemann* (`Berlekamp--Massey` over $FF_ell$) keeping every relation row in original sparse $plus.minus 1, plus.minus 2$ form: `accum_small_coef_mod_l` applies $|c| <= 8$ via pure `ModrAdd` addition chains (`754` bitops, *`0` `ModrMul`s*, vs `40,542` bitops per `ModrMul`), projects via $u in {-1, +1}^N$ (`0` `ModrMul`s), and verifies exact full-matrix orthogonality $M h equiv 0 mod ell$, cutting LA `bitop/v1` by *`90.31%`* (`447.35G -> 43.35G`) (`[pcc:unverified]`).
- *Phase-1 Sieve & Line Enumeration (`1,133.25G -> 770.48G` vs `R1`, `2.46x` vs `R0`):* `R3` replaced full Yun/Frobenius line splitting on degree-8/7 $G(x)$ with root-free partition tests (`parity_selftest.json`: `507/507` exact line match across `1,024` planes) and an $FF_p$ inverse table; `R4` added stack-array Stickelberger `(2, 2)` quartic screening; `R5` widened `roots_by_incidence` amortization to `4,096` planes with `64`-plane sub-rank checks (`B_mean: 156,928 -> 155,520`). `R9` (`sieve_line_buckets_zero_mul` + `SharedBFast`) advanced degree-4 $A_1'(x)$ instead of degree-8 $w A_1'(x)^2$ (halving difference-table `FieldAdd`s), pre-multiplied $N(B)(x) rho(x)$ once per multi-line plane, and replaced all `3|cal(B)| = 1,527` per-line Montgomery batch multiplications with an $O(1)$ table lookup `inv_2w_sq[A_1'(x)]` (`-209.5G`, `-15.89%` total!). `R10` (`lines_for_b_frob_stickelberger_m9`) fused `disc(rest)` Legendre evaluation into the Euclidean squarefreeness loop (`0` extra GCD passes, skipping `50%` of impossible partitions on $deg in {4..8}$ by Stickelberger parity) and replaced `735`-mul Horner `compose_mod` with a squaring-built Frobenius matrix (`<= 56` muls), reaching `mean4 = 1,045.54G` (`8.07s` wall; `Run50 = 1,141.61G`, `8.36s` wall) (`[pcc:unverified]`).
- *Phase-3 Traced Descent (`343.68G -> 230.94G` vs `R1`, `7.58x` vs `R0`):* After `R6` falsified linear $d_0$ substitution (`+0.76%` `STALL #1`: substituting $d_0$ into 5 degree-2 Weil equations creates degree-4 equations whose Macaulay fill exceeds the 1-variable saving), `R7` switched cold-traced F4 from `Staircase(64)` to `Hilbert(8)`, eliminating one redundant high-degree Macaulay step across all `~467` trace replays (`-32.74%` Phase-3 cost) (`[pcc:unverified]`).

*Five Consecutive Orthogonal Frontier Stalls (`R11..R15` on `R10`, `[pcc:unverified]`):*
Post-rank over-collection (`R11: +1,024 B => +0.48%`; `R12: +3,072 B => +1.10%`) proved that at `R10`'s `5.0M`-bitop/plane sieve speed vs `494M`-bitop/residual descent speed, marginal anchor gain $Delta f = +0.33$% saves only `8.0` mean residuals (`-3.89G`) against `+15.35G` sieve cost (`3.95x` negative ROI). Conditional `3-LP` extraction on productive `2-LP` lines (`R13 => +6.30%`) confirmed cubic $(2^3)$ splitting remains net-negative even under smooth-line conditioning. Restricting small-FB admissions to `92%` (`R14 => +211.88%`, `0/4` verified) left `41` unpopulated rational columns (`rank <= 467 < 508`), while even full `92%` FB truncation inflates required planes by $0.92 / 0.92^5 = 1.396 times$. Narrow `512/16` batch cadence (`R15 => +4.96%`) confirmed `4096/64` amortization sits at the minimum (`[pcc:unverified]`).

== 4. Multi-Prime Scaling (`p = 307..1009`), Controls & Standalone Qualification

#figure(
  image("figures/multiprime_frontier.svg", width: 91%),
  caption: [Multi-prime total `bitop/v1` scaling (`p in {307, 383, 503, 661, 1009}`, fresh seeds `{101, 211, 307}`) for `R0`, `R1`, `R10` Champion, and Pollard rho (`[pcc:unverified]`).],
)

#table(
  columns: (0.48fr, 0.56fr, 0.72fr, 0.78fr, 0.78fr, 0.80fr, 0.68fr, 0.68fr, 0.86fr, 0.96fr),
  inset: 3.0pt,
  align: (right, center, right, right, right, right, right, right, right, right),
  stroke: 0.4pt + rgb("#d1d5db"),
  [*$p$*], [*Solved*], [*Mean $ell$*], [*`R0` Mean*], [*`R1` Mean*], [*`R10` Champ*], [*vs `R0`*], [*vs `R1`*], [*Kara-Rho*], [*Champ / Kara-Rho*],
  [`307`], [`2/3`], [`2.09e14`], [`0.993T`], [`0.303T`], [*`0.183T`*], [`5.42x`], [`1.65x`], [`3.023T`], [`0.0606` (`16.5x`)],
  [`383`], [`3/3`], [`7.89e14`], [`2.277T`], [`0.655T`], [*`0.412T`*], [`5.53x`], [`1.59x`], [`5.337T`], [`0.0771` (`13.0x`)],
  [`503`], [`3/3`], [`4.05e15`], [`1.653T`], [`0.776T`], [*`0.464T`*], [`3.57x`], [`1.67x`], [`8.912T`], [`0.0520` (`19.2x`)],
  [`661`], [`3/3`], [`2.09e16`], [`2.185T`], [`1.067T`], [*`0.617T`*], [`3.54x`], [`1.73x`], [`43.385T`], [`0.0142` (`70.4x`)],
  [`1009`], [`3/3`], [`2.64e17`], [`3.218T`], [`1.879T`], [*`1.014T`*], [`3.17x`], [`1.85x`], [`130.782T`], [*`0.00775` (`129.0x`)*],
)

*Scaling, Overflow & Standalone Qualification (`[pcc:unverified]`):* Across $p in {307..1009}$ (`14/15` solved; `(307, 307)` hits the finite $B_"max" = 46,971$ plane cap across all arms), `R10` improves its speedup over `R1` monotonically with $p$ (`1.59x` at $p=383$ to *`1.85x`* at $p=1009$), lowering the 5-prime log-slope `d(log cost)/d(log p)` from `1.391` (`R1`) to *`1.280`* (`R10`: relations `1.389`, LA `2.053`, descent `0.964`) vs `3.299` for Karatsuba Pollard rho (`129.0x` bitop advantage at $p=1009$, `198.1x` vs schoolbook rho) (`[pcc:unverified]`). Runtime-trapped overflow-check controls (`overflow_release.json` `false` vs `overflow_checked.json` `true` on `(307,101), (383,211), (503,307)`) matched bit-for-bit on `sources_sha256`, `projections_sha256`, `recovered_scalar`, and `observed_bitops`. Pre-sealed `/tmp/immutable_jv_graph_ic_hillclimb_qualify` (`sha256 = fd26c4e4...`) independently verified all `17` trajectory and multi-prime bundles (`qualification_report.json`: `17/17` `all_passed = true`, `failed_bundles = 0`). Claims apply solely to the degree-2 genus-3 Weil-cover family over $FF_(p^6)$ (`[pcc:unverified]`).
