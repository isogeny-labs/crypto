#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[The $k = 4$ index calculus end to end: \ the $D_4$ baseline and the $V_4$ line]
  #v(4pt)
  Goal G2, `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.5–§9.14 · 2026-10-17/18 · experiments E19–E24
]

= Result

On curves over $FF_(p^4)$ with full rational 2-torsion, a base line stable under all three
translations and $S_5$ written in the invariants of $V_4^4 _0 times.r S_4$ (four unknowns of
weights $(2,2,1,1)$, quotient $64$, Macaulay degree $11$) cut one decomposition from
$1.41 dot 10^9$ to $1.66 dot 10^6$ $FF_p$ multiplications. Measured end to end (E24, 12 curves,
$p = 101$–$521$), *the $k = 4$ index calculus is $35$–$240 times$ cheaper than the $D_4$ baseline
(E19, 10 curves) at every matched size, and the modelled handover with rho moves from
$2^(116.7)$ to $2^(90.1)$.* Every logarithm was recovered and verified; the exponent is unchanged
($S slash "rho" prop n^(-1 slash 4)$, fitted $-0.25$): *engineering, a constant.*

*Distance remaining.* Every row is still $1.3 dot 10^4$–$1.7 dot 10^5 times$ slower than the
matched negation rho. The handover is a model extrapolation of measured constants with the theory
exponents, not a measured crossing. *Scope:* full rational 2-torsion with rational halves of two
$T$'s, $h = 4$, $n <= 2^(34.1)$.

#figure(image("figures/e24_vs_e19.svg", width: 82%), caption: [$S slash "rho"$ per curve on both pipelines (E19: 10 curves; E24: 12 curves); 16 negation walks per curve; runs `experiments/27_quartic_pipeline_d4`, `experiments/29_quartic_pipeline_v4`.])

= What ran

Each experiment was pre-registered in the note before its code. E19: the $tau_T$-folded line and
the weighted $D_4$ solve (degree 26). E24: E19's protocol on the $V_4$ line and solve (degree 11).
Both: uniform targets $R = a G + b Q$; one verified decomposition per target as a folded row;
Wiedemann until $[d]G = Q$ verifies; every phase charged (precompute, base, generation, Weil
restriction, solves, lift-back, group checks, linear algebra at 16 per mod-$n$ multiplication);
16 negation and 16 unfolded walks on the same curve; unpriced native work at 0, 0.1 and 1 GAE.
Host: one shared 4-vCPU container, L0; operation counts only.

#table(
  columns: 7,
  align: (right, right, right, right, right, right, right),
  [*p*], [*E19 $S slash "rho"$*], [*E24 $S slash "rho"$*], [*ratio (by seed)*], [*E19 $C$*], [*E24 $C$*], [*E24 targets*],
  [101], [$6.33, 5.85 dot 10^6$], [$1.69 dot 10^5, 3.43 dot 10^4$], [$37, 171$], [$1.39 dot 10^9$], [$1.62 dot 10^6$], [77,960 / 15,387],
  [149], [$4.09, 3.40 dot 10^6$], [$3.80, 5.54 dot 10^4$], [$108, 61$], [$1.40 dot 10^9$], [$1.63 dot 10^6$], [31,225 / 45,343],
  [197], [$4.91, 2.83 dot 10^6$], [$1.40 dot 10^5, 5.44 dot 10^4$], [$35, 52$], [$1.41 dot 10^9$], [$1.65 dot 10^6$], [142,361 / 74,345],
  [269], [$5.51, 2.57 dot 10^6$], [$2.30, 2.34 dot 10^4$], [$240, 110$], [$1.41 dot 10^9$], [$1.65 dot 10^6$], [44,281 / 50,040],
  [397], [—], [$1.31, 1.58 dot 10^4$], [—], [—], [$1.66 dot 10^6$], [66,981 / 89,424],
  [521], [—], [$1.88, 2.15 dot 10^4$], [—], [—], [$1.66 dot 10^6$], [131,952 / 142,487],
)

#figure(image("figures/solve_cost_both.svg", width: 72%), caption: [Median $C$ per call inside the pipelines: weighted $D_4$ (quotient 512) against the $V_4$-invariant solve (quotient 64).])

= Predictions and verdicts

#table(
  columns: 3,
  align: (left, left, left),
  [*id*], [*registered*], [*measured*],
  [G2.9–G2.11], [E19 correct; $y in [70, 160]$; $C$ within ±30 % of d4-s5], [hold: all logs; $y = 115.1$; $C = 1.37$–$1.41 dot 10^9$],
  [G2.12], [E19 $S slash "rho"$ within ×2 of $0.584 C slash p$; slope $[-0.35, -0.15]$], [per size holds (one curve at 0.495); slope $-0.13$: falsified],
  [G2.13], [E19 handover within ±3 bits of $2^(118.9)$], [holds: $2^(116.7)$],
  [G2.14–G2.17], [E20: DLP dearer at measured sizes; crossovers $> 2^(160)$], [hold: $1.7$–$3.6 times$ dearer; $2^(164)$–$2^(222)$],
  [G2.18–G2.21], [E21: replay $1.03$–$1.20 times$, $<= 5 %$ fallbacks], [agreement holds; gain and fallbacks falsified (net loss)],
  [G2.22–G2.24], [E22: line exists; $C$ down $4$–$30 times$; $y$ up $3$–$10 times$], [line holds; $C$ down $approx 820 times$ (falsified high); $y$ up $approx 13 times$],
  [G2.26], [E23: no useful shared base on $j = 0$], [holds (confined or empty)],
  [G2.27–G2.31], [E24: correct; $C in [1.3, 2.2] dot 10^6$; $20$–$300 times$ below E19; slope; $n^* in [2^(86), 2^(98)]$], [all hold: $35$–$240 times$; slope $-0.25$; $n^* = 2^(90.1)$],
)

#figure(image("figures/distance_to_rho.svg", width: 72%), caption: [E19 against G2.12's registered prediction and its ×2 band (10 curves).])

= Records

Raw records and tables: `experiments/27_quartic_pipeline_d4/` (E19), `experiments/28_e20_dlp/`,
`experiments/28_e21_replay/`, `experiments/28_e22_stage_a/`, `experiments/28_e22_stage_b/`,
`experiments/29_quartic_pipeline_v4/` (E24). Code: `src/cryptanalysis/quartic_folds.rs`,
`src/cryptanalysis/klein_v4.rs`, `src/cryptanalysis/d4_weighted_s5.rs` (trace, additive);
runners `examples/quartic_pipeline_{d4,v4}.rs`, `examples/klein_stage_{a,b}.rs`,
`examples/d4_replay.rs`, the ladder's `--arms d4dlp`. Tables:
`examples/glv_invariant_experiment_tables.rs`.

= Errata (2026-10-21)

An independent validator replayed every E19 and E24 run (field-identical apart from wall time,
every logarithm confirmed) and recomputed the numbers above from the raw JSON
(`benchmarks/20261018-k4-v4-line/validator-report.json`). The text above is left as written; these
values supersede it. No verdict changes. Full list: the note's §9.19.

#table(
  columns: 3,
  align: (left, left, left),
  [*where*], [*written*], [*raw data*],
  [E19 / E24, $p = 101$ seed 2], [$171 times$], [$170 times$ ($170.4$; 171 came from dividing rounded values)],
  [G2.12, $p = 53$ seed 1], [0.495], [0.494 ($0.49448$)],
)

At 0 GAE per unpriced native operation the E19-to-E24 factor is $35$–$240 times$. Pricing one GAE
(97 $F_p$ multiplications) per Macaulay-matrix entry and per walk step, it is $19$–$132 times$ at
0.1 GAE and $6.5$–$45 times$ at 1 GAE; E24 stays below E19 on every curve at every price.
