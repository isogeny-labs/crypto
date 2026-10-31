#set page(paper: "a4", margin: 2cm)
#set text(font: "New Computer Modern", size: 10pt)
= Round 6: early-stopping F4 and the trace coset on every Koblitz curve
_research/isogeny_conductor_gap_round6_20261018, 2026-10-18. E8/E9a: 6,000 descent systems on 5 Koblitz curves. E9b: one audited ecbench session (168 records, 12 replays reproduced)._

== Findings
+ *Lead 1 does not carry over to the binary descent.* A solvable system costs 1.0 to 2.3 times an unsolvable one. Stopping F4 once its degree-1 elements leave 8 unknowns saves up to 43 %, but only where most systems are solvable; it costs up to 37 % more where few are. Solutions are identical on all 6,000 systems.
+ *The trace coset's yield gain is 1/f#sub[2]* (7 of 9 rows). f#sub[2] is 0.45 at F#sub[2]#super[19], which refutes round 5's explanation.
+ *The affine coset extends the yield gain to a = 1* (1.6 to 3.1 times). F4 work per system rises 1.1 to 45 times; the large factors come where the solving degree goes from 3 to 4, on a = 0 too.
+ *Whole method on a = 1:* coset MITM beats default MITM across the band at n = 19 and 23. Best cell: 12.7 times rho. The study's best stays 7.29 times rho.

#figure(image("reports/run-20261018T1742Z/e8_e9a.svg", width: 100%), caption: [E8 saving against solvable share; E9a yield ratio against 1/f#sub[2].])
#figure(image("reports/run-20261018T1815Z/e9b_s_over_rho.svg", width: 90%), caption: [E9b: S over rho per arm per curve (a = 1 Koblitz).])

== Leads
+ Choose the early-stop threshold from the expected solvable share, or use free = 2 to 4.
+ A 2E base inside the default's span, without z#super[d], may keep the solving degree at 3.
+ Round 5's F#sub[2]#super[19] yield of 1.48 times is unexplained (per system: 1.81). Check whether the ecbench walk targets are evenly spread.
+ Binary MITM stays 13 to 45 times above rho; no lever changes the exponent.
