#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E32: the handover ladder — distance to the matched rho over $2^(22.6)$–$2^(42.4)$, and the crossover order fitted with intervals

*Run report and results, 2026-10-31 to 2026-11-01.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.34 (pre-registration, commit `246460f094`, pushed before any run) and §9.36 (results). Binaries built from `e757710521`, run only from pinned scratchpad copies (sha256 in `experiments/36_handover_ladder/binary_commit.txt`, re-verified after a container restart). Records: `experiments/36_handover_ladder/` (`ic_p*s*.json`, `walk_p*s*.json`, `run.log`, `tables.md`, `summary.json`, `fit.json`); tables and fits by `examples/e32_tables.rs`. Lead: `LEAD-ICE-charged-toy-ic-exponent-vs-rho` (crypto-autoresearcher).

*Host and route:* the 4-vCPU Claude Code container, 3 cores (`taskset -c 1-3`). isolab-eks was checked and is unreachable from it (no `aws` or `kubectl`; the GitHub dispatch route is refused by the session proxy), so no isolation tier is earned: *operation counts only, wall times not admitted*.

== What ran
- *IC arm:* the $k = 4$ $V_4$/GLS index calculus on E25's GLS twists (`generate_gls_klein_instance(p, seed, 16)`), arm B ($chevron.l V_4, psi chevron.r$-folded base), E31's solver A (`chain`), E27's Karatsuba products. Every phase charged in $FF_p$ multiplications.
- *Matched rho:* E27's walk (iii): the $chevron.l plus.minus psi chevron.r$ walk with Karatsuba products and norm inversions batched over a pool of 16. There are 16 walks per curve; the $[d]G = Q$ check is excluded.
- *Ladder:* $p in {101, 149, 197, 269, 397, 557, 773, 1093, 1549, 2213, 3089}$, seeds 1–3: 33 paired curves and 528 walks, with no seed replaced. All logarithms were recovered, all rows re-added and all walks correct.
- *Deviations:* (1) the container restarted during the $p = 2213$ seed 2 walk, and only that walk was rerun. (2) The $p = 4337$ seed 1 index calculus completed, but its 16-walk pool was OOM-killed by the session's memory cgroup at 4.8 GB. The pool was not shrunk, because that would change the reference. Its record is kept unpaired and outside every fit. Compute used: 8,952 s of the 14 h budget.

== Checks
- *Identity (G2.114, kill test):* the ten E25-curve runs reproduce E27's arm-B records in 6,040 of 6,040 compared fields (wall times aside). The walks reproduce E27's matched-rho means and step counts exactly.

== Measured
Distance $D(g) = (S + 97 g E) \/ ("rho" + 97 g dot "steps")$, where $E$ is the Macaulay entries and one GAE is 97 $FF_p$ multiplications.

#table(columns: 5, align: right,
  [$p$], [$log_2 n$], [$D(0)$ median [min, max]], [$D(0.1)$], [$D(1)$],
  [101], [22.63], [$2.07 dot 10^4$ [1.71e4, 3.93e4]], [$7.21 dot 10^4$], [$3.83 dot 10^5$],
  [149], [24.88], [$4.03 dot 10^4$ [8.26e3, 1.52e5]], [$1.38 dot 10^5$], [$6.59 dot 10^5$],
  [197], [26.49], [$1.49 dot 10^4$ [1.07e4, 3.49e4]], [$5.09 dot 10^4$], [$2.38 dot 10^5$],
  [269], [28.29], [$1.75 dot 10^4$ [8.83e3, 2.75e4]], [$5.90 dot 10^4$], [$2.61 dot 10^5$],
  [397], [30.53], [$8.75 dot 10^3$ [6.39e3, 9.72e3]], [$2.91 dot 10^4$], [$1.24 dot 10^5$],
  [557], [32.49], [$6.46 dot 10^3$ [4.98e3, 7.43e3]], [$2.13 dot 10^4$], [$8.94 dot 10^4$],
  [773], [34.38], [$7.63 dot 10^3$ [6.52e3, 1.15e4]], [$2.50 dot 10^4$], [$1.04 dot 10^5$],
  [1093], [36.38], [$4.37 dot 10^3$ [3.67e3, 5.06e3]], [$1.42 dot 10^4$], [$5.84 dot 10^4$],
  [1549], [38.39], [$2.37 dot 10^3$ [2.07e3, 3.72e3]], [$7.66 dot 10^3$], [$3.14 dot 10^4$],
  [2213], [40.45], [$1.61 dot 10^3$ [1.34e3, 2.05e3]], [$5.15 dot 10^3$], [$2.10 dot 10^4$],
  [3089], [42.37], [$1.54 dot 10^3$ [1.14e3, 1.70e3]], [$4.93 dot 10^3$], [$2.01 dot 10^4$],
)

#figure(image("figures/distance.svg", width: 100%), caption: [$log_2 D$ per curve (33 curves, 3 per size) against $log_2 n$ at 0, 0.1 and 1 GAE, with the M1 (free slope, $p >= 269$) and M2 (slope $-1\/4$) fits run out to $D = 1$. The shaded band is the measured range. Source: `experiments/36_handover_ladder/summary.json`.])

== Crossover order (frozen fits; stratified cluster bootstrap over curves, 10,000 replicates)
#table(columns: 4, align: (center, left, right, right),
  [GAE], [model], [$beta$ [95 %]], [$log_2 n^*$ [95 %]],
  [0], [M1, $p >= 269$], [$-0.243$ [−0.279, −0.205]], [*85.5* [79.2, 94.3]],
  [0], [M1, all], [$-0.232$ [−0.272, −0.191]], [87.7 [80.2, 98.6]],
  [0], [M2, $beta = -1\/4$], [—], [84.0 [83.4, 84.6]],
  [0.1], [M1], [$-0.248$ [−0.284, −0.211]], [*91.2* [84.4, 100.7]],
  [0.1], [M2], [—], [90.8 [90.3, 91.4]],
  [1], [M1], [$-0.256$ [−0.292, −0.219]], [*97.6* [90.1, 107.9]],
  [1], [M2], [—], [99.1 [98.5, 99.6]],
)
No bootstrap replicate had $beta >= 0$. Yield exponent in $p$: $-0.066$ [−0.285, 0.154]. The linear-algebra share is 0.0044 % at $p = 3089$; extrapolated as $prop p^2$, it is about 5.6 % at M2's $n^*$.

#figure(image("figures/nstar.svg", width: 100%), caption: [Fitted crossover order $log_2 n^*$, with points and 95 % bootstrap intervals, against the largest measured size. Source: `fit.json`.])
#figure(image("figures/components.svg", width: 100%), caption: [Components per curve: $S\/p$, rho$\/sqrt(n)$ (walk (iii)) and targets per relation $y$. Source: `summary.json`.])

== Verdicts (pre-registered, §9.34)
G2.114–G2.118 hold.

- *G2.119:* the steps hold. The cost per step is falsified high (95.9–96.5 against the band [66, 80]): the band priced the batched addition and omitted the $psi$ map in the class representative.
- *G2.120:* the level ($1.54 dot 10^3$) and "rho ahead of every row at every weight" hold. "Falls with $n$" holds as a trend but not monotonically: the median rises at 3 of 10 steps, within the per-curve scatter.

G2.121–G2.123 hold.

== What it says, and what it does not
Over 20 bits of $n$, the line's distance to its strongest matched rho falls as $n^(-0.243 plus.minus 0.04)$. The slope is identified with eleven sizes and three curves each, and its interval contains $-1\/4$. *No crossover occurs in the reachable range: $n^* > 2^(42.4)$, measured.* The fitted crossover order is $2^(85.5)$ [$2^(79.2)$, $2^(94.3)$] at 0 GAE, $2^(91.2)$ at 0.1 GAE and $2^(97.6)$ at 1 GAE. It is an extrapolation of about 43 bits beyond the top size, not a measured crossing. *Class:* a measurement of distance, not an improvement. The interval contains §9.33's single-point model handover ($2^(88)$–$2^(90)$). *Distance remaining:* about $1.5 dot 10^3 times$ at $2^(42.4)$ (0 GAE; best row $1.14 dot 10^3 times$). No leaderboard or `benchmarks/` row is added; independent validation is arranged by the owner.
