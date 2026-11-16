#set page(margin: 1.6cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
= N4 iteration 2: double large primes (B2) and structured elimination (B1) vs the recorded B0
2026-10-21. Branch `claude/quartic-ic-n4-la-20261021`, protocol commit eab1bdfacd, records `results/`. Descriptive run report; interpretation in `README.md`. Cost in F\_q-multiplication equivalents (M-eq); every IC unit verified kG = Q.
== Result in brief
B2 verified on 57 of 57 units (first attempts retried and charged) and costs IC/rho = 0.024 at q = 1048507. Exponent on q >= 4051: B2 1.045 [1.037, 1.052], rho 1.476 [1.438, 1.508]; the recorded B0 baseline had 1.083. B1 (structured elimination) is a small stage gain and a regression at q = 262111. B2 is the new baseline. GHS-weak family only; plain rho; shared VM.
== B2 against the matched rho
#table(columns: 7, align: right, [q],[units ok],[rho runs],[B2 M-eq],[rho M-eq],[B2/rho w=0 (95%)],[resolved],
[223],[5/5],[150],[8.01e+05],[2.08e+05],[3.848 (2.665-5.251)],[rho lower],
[499],[5/5],[150],[8.23e+05],[5.88e+05],[1.400 (1.080-1.792)],[unresolved],
[991],[5/5],[150],[1.21e+06],[1.53e+06],[0.791 (0.710-0.881)],[IC lower],
[2011],[5/5],[150],[1.97e+06],[4.54e+06],[0.434 (0.388-0.485)],[IC lower],
[4051],[5/5],[150],[3.39e+06],[1.32e+07],[0.257 (0.231-0.288)],[IC lower],
[8191],[5/5],[120],[6.2e+06],[4.07e+07],[0.153 (0.139-0.168)],[IC lower],
[16363],[5/5],[120],[1.16e+07],[1.13e+08],[0.103 (0.094-0.114)],[IC lower],
[32719],[5/5],[80],[2.29e+07],[2.8e+08],[0.082 (0.074-0.091)],[IC lower],
[65479],[5/5],[80],[4.5e+07],[9.2e+08],[0.049 (0.043-0.055)],[IC lower],
[131071],[3/3],[24],[8.98e+07],[2.55e+09],[0.035 (0.030-0.042)],[IC lower],
[262111],[3/3],[18],[2.28e+08],[6.4e+09],[0.036 (0.029-0.046)],[IC lower],
[524287],[3/3],[18],[4.91e+08],[2.01e+10],[0.024 (0.019-0.033)],[IC lower],
[1048507],[3/3],[12],[1.09e+09],[4.49e+10],[0.024 (0.019-0.031)],[IC lower],
)
#figure(image("reports/cost_vs_q.png", width: 82%), caption: [Cold cost vs q (recorded means). B0 and rho: records of PR 2283; B1, B2: this study.])
#figure(image("reports/ratio_vs_b0.png", width: 78%), caption: [Cost ratios against the recorded B0 baseline, paired by size, 95% bootstrap over units.])
#figure(image("reports/ratio_vs_rho.png", width: 78%), caption: [IC cost over matched rho cost with 95% bootstrap intervals.])
#figure(image("reports/phases_b2.png", width: 78%), caption: [Phase shares of B2: linear algebra and relations; descent under 1% from q = 262111.])
== Paired comparison with B0
#table(columns: 5, align: right, [X/Y],[q],[X units],[Y units],[cost ratio (95%)],
[B0n/B0],[4051],[3],[5],[0.983 (0.919-1.049)],
[B0n/B0],[8191],[3],[5],[1.039 (1.016-1.062)],
[B0n/B0],[16363],[3],[5],[1.012 (0.965-1.059)],
[B0n/B0],[32719],[3],[5],[1.027 (1.007-1.046)],
[B0n/B0],[65479],[3],[5],[1.012 (1.005-1.019)],
[B1/B0],[4051],[5],[5],[0.876 (0.820-0.941)],
[B1/B0],[8191],[5],[5],[0.912 (0.871-0.949)],
[B1/B0],[16363],[5],[5],[0.813 (0.777-0.845)],
[B1/B0],[32719],[5],[5],[0.853 (0.837-0.870)],
[B1/B0],[65479],[5],[5],[0.842 (0.832-0.851)],
[B1/B0],[131071],[3],[3],[0.875 (0.550-1.254)],
[B1/B0],[262111],[3],[3],[1.534 (1.523-1.547)],
[B2/B0],[223],[5],[5],[0.932 (0.630-1.304)],
[B2/B0],[499],[5],[5],[0.517 (0.385-0.693)],
[B2/B0],[991],[5],[5],[0.475 (0.422-0.536)],
[B2/B0],[2011],[5],[5],[0.407 (0.359-0.459)],
[B2/B0],[4051],[5],[5],[0.355 (0.324-0.392)],
[B2/B0],[8191],[5],[5],[0.324 (0.313-0.336)],
[B2/B0],[16363],[5],[5],[0.273 (0.261-0.285)],
[B2/B0],[32719],[5],[5],[0.248 (0.245-0.252)],
[B2/B0],[65479],[5],[5],[0.224 (0.222-0.227)],
[B2/B0],[131071],[3],[3],[0.197 (0.124-0.282)],
[B2/B0],[262111],[3],[3],[0.297 (0.295-0.299)],
[B2/B1],[4051],[5],[5],[0.405 (0.374-0.441)],
[B2/B1],[8191],[5],[5],[0.356 (0.338-0.376)],
[B2/B1],[16363],[5],[5],[0.336 (0.324-0.349)],
[B2/B1],[32719],[5],[5],[0.291 (0.286-0.296)],
[B2/B1],[65479],[5],[5],[0.267 (0.263-0.270)],
[B2/B1],[131071],[3],[3],[0.225 (0.222-0.227)],
[B2/B1],[262111],[3],[3],[0.194 (0.192-0.196)],
)
== Exponents
#table(columns: 4, align: left, [range / arm],[IC (w=0)],[rho (w=0)],[gap (w=0)],
[B2, q >= 4051],[1.045 [1.037, 1.052]],[1.476 [1.438, 1.508]],[0.430 [0.391, 0.463]],
[B2, all 13 sizes],[0.895 [0.878, 0.915]],[1.482 [1.462, 1.499]],[0.587 [0.558, 0.612]],
[B0 baseline, q >= 4051],[1.083 [1.040, 1.129]],[1.485 [1.437, 1.525]],[0.402 [0.335, 0.468]],
)
== Limits
GHS-weak family only; plain rho without a negation map (up to sqrt 2); L0 shared VM; B2 first attempts at q = 223 included two wrong answers caught by the kG = Q check (vacuous hold-out at 30 columns); B1 regresses at q = 262111; not compared with an external implementation.
