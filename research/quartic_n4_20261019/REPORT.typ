#set page(margin: 1.6cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
= N4: relation-collection index calculus on the GHS plane quartic vs a matched rho on E0(K)
2026-10-19. Branch `claude/quartic-ic-n4-20261019`, protocol `PROTOCOL.md` (commit 9a63c9161), records `results/`. Descriptive run report; interpretation in `README.md`. Measured values are in F\_q-multiplication equivalents (M-eq); every IC run verified kG = Q on E0.
== Result in brief
The old quartic "pure IC" enumerated the subgroup (T1). A real pipeline (line relations with one large prime, Lanczos mod ell, descent through canonical Pic0 arithmetic) verified on 51 of 51 instances and costs IC/rho = 0.122 [0.10, 0.16] at q = 262111. Exponent on q >= 4051: IC 1.083 [1.040, 1.129], rho 1.485 [1.437, 1.525], gap 0.402 [0.335, 0.468] (w = 0). Lead N4 stays open by the frozen rule. Scope: GHS-weak family only, plain rho, shared VM.
== Cold single-target cost
#table(columns: 8, align: right, [q],[units ok],[rho runs],[IC-B0 M-eq],[rho M-eq],[IC/rho w=0 (95%)],[IC/rho w=1 (95%)],[resolved],
[223],[5/5],[150],[8.59e+05],[2.04e+05],[4.22 (3.55-4.92)],[2.94 (2.55-3.36)],[rho lower],
[499],[5/5],[150],[1.59e+06],[5.78e+05],[2.75 (2.24-3.19)],[1.97 (1.69-2.22)],[rho lower],
[991],[5/5],[150],[2.54e+06],[1.51e+06],[1.69 (1.49-1.91)],[1.32 (1.19-1.45)],[rho lower],
[2011],[5/5],[150],[4.84e+06],[4.48e+06],[1.08 (0.97-1.21)],[0.90 (0.82-0.99)],[unresolved],
[4051],[5/5],[150],[9.54e+06],[1.3e+07],[0.73 (0.66-0.82)],[0.66 (0.60-0.72)],[IC lower],
[8191],[5/5],[120],[1.91e+07],[4e+07],[0.48 (0.44-0.52)],[0.46 (0.42-0.50)],[IC lower],
[16363],[5/5],[120],[4.26e+07],[1.11e+08],[0.38 (0.35-0.43)],[0.37 (0.34-0.41)],[IC lower],
[32719],[5/5],[80],[9.22e+07],[2.76e+08],[0.33 (0.30-0.37)],[0.33 (0.30-0.37)],[IC lower],
[65479],[5/5],[80],[2e+08],[9.04e+08],[0.22 (0.20-0.25)],[0.23 (0.20-0.26)],[IC lower],
[131071],[3/3],[24],[4.57e+08],[2.45e+09],[0.19 (0.12-0.29)],[0.22 (0.14-0.35)],[IC lower],
[262111],[3/3],[18],[7.65e+08],[6.28e+09],[0.12 (0.10-0.16)],[0.14 (0.12-0.19)],[IC lower],
)
Baseline arm IC-A0r (all rational points, retries charged): q=223: 2.04e+06 M-eq; q=499: 8.49e+06 M-eq; q=991: 3.12e+07 M-eq; q=2011: 2.24e+08 M-eq; q=4051: 5.03e+08 M-eq.
#figure(image("reports/cost_vs_q.png", width: 82%), caption: [Cold cost vs q (recorded means; w = 1 GAE dotted). Source: results/summary\_all.json, grid commit in README.])
#figure(image("reports/ratio_vs_q.png", width: 78%), caption: [IC-B0 / rho with 95% bootstrap intervals (2000 resamples of units and runs).])
#figure(image("reports/phases.png", width: 78%), caption: [Phase shares of IC-B0 cost: linear algebra and relations dominate for q >= 4051; descent (the only phase that adds Pic classes) falls to 1%.])
== Exponents
#table(columns: 5, align: left, [range],[IC-B0 (w=0)],[rho (w=0)],[gap (w=0)],[gap (w=1)],
[all 11 sizes],[0.998 [0.971, 1.023]],[1.486 [1.464, 1.505]],[0.487 [0.455, 0.521]],[0.410 [0.378, 0.443]],
[q >= 4051 (primary)],[1.083 [1.040, 1.129]],[1.485 [1.437, 1.525]],[0.402 [0.335, 0.468]],[0.332 [0.266, 0.398]],
[IC-A0r, q <= 4051],[1.986 [1.894, 2.072]],[1.451 [1.407, 1.492]],[-0.534 [-0.626, -0.422]],[-0.554 [-0.643, -0.445]],
)
== T1: old quartic\_ic.rs solves at q = 5, 7, 11
#table(columns: 6, align: right, [q],[old S before targets],[loop over n = 1..ell share],[line relations (rank/cols)],[deg-3 pic\_canon merges],[old / canonical targets correct],
[5],[650 443],[99.82%],[0 (0/6)],[1],[10/10, 5 (5 n/a)],
[7],[2 751 342],[99.95%],[1 (1/8)],[7],[10/10, 7 (3 n/a)],
[11],[37 920 004],[99.99%],[3 (3/12)],[20],[10/10, 10]
)
The recovered scalars and the deg-6 table survive; the "recovery:ic" label, the deg-3 reduction, `2lp` (= `full`) and the rho S = 0 do not.
== T3 and T4
Addition cost (3523M + 44I, #2227) vs published 148M + 15SQ + 2I (Flon, Oyono, Ritzenthaler, flex, char > 5; not implemented): descent share of IC-B0 is 55% at q = 223, 13% at 4051, 1% at 262111; with descent free the ratio at 4051 moves 0.735 to 0.637, at 262111 0.122 to 0.120. Not the bottleneck where IC wins; linear algebra is (57 to 74%). Stop rule: gap interval above 0 at w = 0 and w = 1, so N4 is open.
== Limits
GHS-weak family (4 divides the order of E0, E0/F\_{q^3}), plain rho without negation (sqrt 2 sensitivity), one shared 4-core VM at L0, wall time is a practicality note only (IC 0.57 of rho at q = 262111, count ratio 0.12), runs not bit-reproducible (HashMap order), arm A0 as frozen failed 5 of 15 units (A0r added post hoc), not compared with an external implementation.
