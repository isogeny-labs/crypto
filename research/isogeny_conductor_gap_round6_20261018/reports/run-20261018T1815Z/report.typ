#set page(paper: "a4", margin: 2cm)
#set text(font: "New Computer Modern", size: 10pt)
= Run report: E9b, a = 1 Koblitz curves, whole method
_2026-10-18, 18:15–18:20 UTC. Session ECBS1hb4ef3b5d1409: 168 of 168 verified; audit OK with 12 replays reproduced._

Arms on the a = 1 Koblitz curves over F#sub[2]#super[17], F#sub[2]#super[19] and F#sub[2]#super[23] (h = 2): rho, plus MITM and the S#sub[3] descent (f4-f2 and f4-f2-early), each over the default and the affine trace-coset base (d = 8). There are 4 targets and 2 rounds.

#figure(image("e9b_s_over_rho.svg", width: 100%), caption: [Mean S over mean rho S per arm and curve, with the solver priced at its pinned word-XOR ratio.])

- MITM, coset below default across the band: holds at n = 19 and 23 (S 28.4 vs 31.2; 14.1 vs 17.3). Fails at n = 17 (71.6 vs 61.6).
- f4-f2-early below f4-f2 at weights 0.1 and 1: holds on 3 of 6 pairs, with the rest unresolved. At n = 23 early costs more.
- All IC arms are above rho. Best cell: coset MITM at F#sub[2]#super[23], 12.7 times rho.
