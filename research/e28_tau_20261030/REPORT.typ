#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E28: the GLS fold on E19's single-$tau_T$ line (stages 0, A, A2)

*Run report and results, 2026-10-30.* Note `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.26 (pre-registration, `4845b32c7`) and §9.30 (results). Binary `789dc2ff6` (Stage 0, Stage A); supplementary control `0e4fccce1`. Records `experiments/33_tau_stage_a/`. Host: the 4-vCPU container, no isolation tier (isolab-eks unreachable); operation counts only. *No Stage B was run* (by instruction).

== What ran
- Stage 0: 40 GLS twists over $FF_(p^4)$ on E19's line ($sigma(x_0) = -x_0$, $c in FF_p$), $p in {53, 101, 149, 197, 269}$, seeds 1–8. 0 failures.
- Stage A: 10 curves (seeds 1, 2): one exact-oracle stream (`mitm_signed`, every decomposition re-added), two arms on the same points: τ (E19's fold) and τψ ($chevron.l tau_T, psi chevron.r$); Wiedemann to the verified log; 16 $chevron.l plus.minus psi chevron.r$ and 16 negation walks per curve in E25's and E27's arithmetic (640 walks, all correct).
- Solve-cost control: E19's weighted $D_4$ solve on 3 uniform targets per curve (registered; vacuous agreement, no target had a decomposition) and, as a recorded deviation, on 3 targets per curve the oracle decomposes (30 calls).

== Measured
- Stage 0: every $psi$ and line check holds on all 40 curves; *$h = 4$ on all 40*; density 0.50–0.52; 4 / 8 points a column (with ±).
- Stage A: columns ratio 2.00; decompositions per target median 8; residuals to the log pooled 40,749 / 23,345 (×1.75; per curve 1.36–2.56); $y$ pooled 98.4 / 110.1; every log recovered.
- Control: $C$ median $1.36$–$1.44 dot 10^9$ (uniform), $1.37$–$1.48 dot 10^9$ (decomposed); all 30 decomposed calls equal the oracle's set.
- Stage A2 (*model*, derived from measured constants): τψ arm $6.99 dot 10^5$–$1.74 dot 10^7$ × the $chevron.l plus.minus psi chevron.r$ walk at 0 GAE; ×1.30–2.82 below the τ arm on the same curve; against E19 at the same $p$: by size ×1.77–4.29.

#figure(image("figures/stage_a.svg", width: 100%), caption: [Residuals to the verified logarithm per arm (Stage A). Source: `stage_a_p*s*.json`, binary `789dc2ff6`.])
#figure(image("figures/distance_model.svg", width: 100%), caption: [Stage A2 model distances against matched walks, E19's measured rows and E31 A's measured range. Source: `summary.json`.])

== Verdicts (§9.26)
G2.100 holds (gate passed). G2.101 holds (pooled). G2.102 holds with the recorded deviation. G2.103 falsified per curve and at $p = 101$ by size; the $>= 10 times$ gap to E31 A fails at $p = 149$ under the strict reading (best E28 row vs worst E31 row, 3.3×), holds at the other matched sizes.

== What it says
The GLS fold carries over to the single-$tau_T$ line with $h = 4$ (unlike the $V_4$ line, §9.17) and halves its columns; with E19's unchanged solve the line's modelled cost moves ≈ 2–4× against its own matched walks. *Engineering on the $D_4$ line, a model, not an improvement of the baseline*; distance remaining ≥ $7 dot 10^5$× (E25 arithmetic). No leaderboard or `benchmarks/` row: no measured end-to-end $S$ and no validation-gate evidence.
