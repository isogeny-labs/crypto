#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E34: a cheaper order-24 class representative, and the matched rho on E18's $j = 0$ curves across the §5a band

*Run report and results, 2026-11-01.* Note §9.38 (pre-registration, `45378560d`) and §9.39 (results). Code and binary `bd0118f39` (new module `src/cryptanalysis/twist24_rep.rs`; E29's functions untouched), tables `examples/e34_tables.rs`. Records `experiments/38_twist24_cheap_rep/`. Host: the shared 4-vCPU container, one core, serial; isolab-eks unreachable (no `aws`, no `kubectl`); no isolation tier; *operation counts only*, no wall-time claim. Under one CPU-minute of runs.

== Construction
On the canonical model $y^2 = x^3 + b_0 t^3$, candidate $(j, k)$ of a class is $iota^k compose Phi^j (P)$. On abscissae both maps are coordinate-wise: $x_i |-> c_(i j k) x_i$ with $c_(i j k) = h^(2j) z^(i j) omega^k in FF_p$ ($h^2 = z^(-1)$). The cheap representative therefore (1) computes coordinate 0 of the twelve abscissae ($10$ products, $1$ negation) and keeps the minimum with $11$ comparisons, going on to the next coordinate only on a tie ($x_0 = 0$); (2) completes only the winner's abscissa and computes only the winner's ordinate ($Phi^j$ on $y$ is a monomial map); (3) fixes the sign by a half-$p$ test on the first non-zero ordinate coordinate. The multiplier comes from a 24-entry table. Arms: order 6 (E29), order 6 cheap, order 24 (E29), order 24 cheap, order 24 cheap mul-lean (sign classes: $5$ products and $6$ negations on coordinate 0).

== Checks
- *E29 replay*: counted copies reproduce all 512 E29 Stage R records in every field.
- *Representative identity*: 0 mismatches over 39,936 order-24 class members (both cheap variants) and 9,984 order-6 members (200 random points of $chevron.l G chevron.r$ plus 8 forced $x_0 = 0$ points per curve), $"rep" = [m]P$ on every scalar check. No curve has a point with $x = 0$.
- *Walk identity*: 768 of 768 cheap walks equal their built walk (steps, group operations, escapes, log, correctness, visit digest).
- *Negative controls*: omitted candidate 3,328 of 39,936 mismatches and digest differs on 16 of 16 walks; wrong ordinate 39,936 of 39,936 mismatches.
- *Unit tests*: 3 of 3 pass (`cargo test --release --lib twist24_rep`). Clippy and rustfmt pass on E34's files.

== Measured (per step, schoolbook, $p >= 541$)
Representative $FF_p$ products per call: E29 order 24 $86$–$92$, cheap $15.17$, mul-lean $10.17$, E29 order 6 $8.00$, cheap order 6 $4.00$. Multiplications per step: cheap order 24 $117.2$–$120.2$ against $109.1$–$111.2$ (E29 order 6). Native operations per step: $25.8$ (cheap order 24; $13.95$ comparisons), $16.8$ (E29 order 6), $65.3$ (E29 order 24, of which $40.6$ mod-$n$).

#figure(image("figures/per_step.svg", width: 100%), caption: [Per-step $FF_p$ multiplications (left) and emitted native operations by kind (right), $p = 1069$, schoolbook, 16 walks per arm. Source: `summary.json`.])

== The band (pooled order 6 / order 24; every emitted operation priced at $w dot 97$)
#table(columns: 6, align: (left, left, right, right, right, left),
  [pair], [arith.], [0], [0.1], [1 GAE], [holds],
  [E29 / E29], [school], [0.962], [0.562], [0.453], [yes, order 6],
  [E29 / cheap], [school], [1.563], [1.254], [1.126], [yes, order 24 (7/8 curves)],
  [E29 / cheap], [Karatsuba], [1.544], [1.232], [1.122], [yes, order 24 (7/8)],
  [cheap / cheap], [school], [1.505], [1.167], [1.026], [yes, order 24 (3/8; 5 unresolved)],
  [cheap / cheap], [Karatsuba], [1.476], [1.140], [1.021], [yes, order 24 (3/8; 5 unresolved)],
)
#figure(image("figures/band.svg", width: 100%), caption: [Walk cost ratios across the §5a band; large dots pooled, small dots the eight curves (16 walks each). Source: `summary.json`.])

== Verdicts (§9.38)
G2.134–G2.142 hold (G2.135's registered count 256 was per arithmetic; 512 were compared). G2.143 (derived): E18's $psi$-arm distances lengthen by $1.632 times$, $1.254 times$, $1.126 times$ at 0, 0.1, 1 GAE (schoolbook; Karatsuba $1.627$, $1.232$, $1.122$), and the modelled handover moves later by $2.83$, $1.31$, $0.68$ bits.

== What it says
With its representative built from the coordinate-wise structure, the order-24 walk is the stronger rho on E18's $j = 0$ curves at every weight of the band, pooled, in both arithmetics, against both order-6 walks. The margin over the cheap order-6 walk at 1 GAE is $2$–$3 %$ and is not resolved per curve on five of eight curves (16 walks each). With every operation emitted, E29's own order-24 walk loses at every weight; §9.33's crossing came from pricing one operation per step. *Class: a reference correction*: E18's rows move further from rho; no index-calculus cost changed, and this is not an improvement. No leaderboard or `benchmarks/` row.
