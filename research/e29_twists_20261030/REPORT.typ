#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E29: the order-24 twist action on E18's $j = 0$ line (kill test, Stage R, Stage A)

*Run report and results, 2026-10-30.* Note §9.27 (pre-registration, `e772a783c`) and §9.31 (results). Code and binary `0e4fccce1`. Records `experiments/34_twist24_stage_r/`. Host: the 4-vCPU container; no isolation tier; operation counts only. Under 10 core-minutes in all. *No Stage B* (no $S_5$ solve).

== Measured
- *Kill test* (E18's 8 curves, $p = 241 dots 1069$, E18's own models): $psi_p = (u^2 x^p, u^3 y^p)$, $u = b^(-(p-1)\/6)$, is an endomorphism on all 8; its eigenvalue has order 24 ($= 4 dot "ord" chi_6(b)$, $chi_6(b)$ of order 6); $|chevron.l lambda_p, lambda_omega, -1 chevron.r| = 24$; $u^2 in.not FF_p$ (E18's base is not kept); a canonical model $y^2 = x^3 + b_0 t^3$ of the same order exists for every curve. Not killed.
- *Stage R* (256 walks, all correct): steps order 6 / order 24 pooled 1.701 (per curve 1.29–2.10); $FF_p$ cost ratio 0.743–1.202 per curve, 0.962 pooled (schoolbook), 0.884 pooled (Karatsuba): the order-24 class representative costs 191–210 per step against 109–122.
- *Stage A* (lane's `run_k4_twist24`, E18's exact oracle; 9 arms, all logs recovered, 144 walks correct): columns cubic / twist24 = 4.00; LA ratio 13.3–15.2; $r_24$ 0.0058–0.0103; modelled handover $n^*$ $2^(135.0)$–$2^(142.2)$ (twist24) against $2^(145.7)$–$2^(151.9)$ (cubic, E18's fold, same curves).

#figure(image("figures/stage_r.svg", width: 95%), caption: [Stage R ratios per curve (16 walks per order). Source: `stage_r_*.json`.])
#figure(image("figures/handover_model.svg", width: 70%), caption: [Stage A modelled handover (extrapolation from measured constants, $C_4 = 1.213 dot 10^12$). Source: `summary.json`.])

== Verdicts (§9.27)
G2.104 holds. G2.105 holds pooled (at the band's edge), falsified per curve. G2.106 falsified (order-24 walk not cheaper as built). G2.107: no correction of E18's $psi$ rows at the pooled level. G2.108 holds except $r_24$ (falsified low at $p = 241, 541$). G2.109 holds at $p = 241$, falsified low at 337 and 541 (and the $>= 2^45$ clause misses by 0.1 bit at 337).

== What it says
$psi_p$ has order 24 on every E18 curve. As a *walk*, the order-24 action is not a stronger reference with the lane's representative (a cheaper, abscissa-first representative is the next lever, not built). As a *base fold* it cuts E18's $j = 0$ linear algebra ≈ 14× and moves the modelled $S_5$ handover ≈ 10 bits earlier: engineering on the $j = 0$ line, a model, *not an improvement of the baseline*, which sits ≥ $2^45$ earlier (E24/E25/E31). No leaderboard or `benchmarks/` row.
