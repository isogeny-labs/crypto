#set page(paper: "a4", margin: 2cm)
#set text(font: "New Computer Modern", size: 10pt)
= Run report: E8 and E9a, per-system F4 profile
_2026-10-18, 17:42–18:15 UTC. research/isogeny_conductor_gap_round6_20261018._

6,000 S#sub[3] descent systems: 5 Koblitz curves (a = 1 over F#sub[2]#super[17], F#sub[2]#super[19], F#sub[2]#super[23]; a = 0 over F#sub[2]#super[19], F#sub[2]#super[23]), default and trace-coset bases at d = 8, 9, 10, with 200 shared targets. Each system was solved by full f4-f2 and by f4-f2-early (free = 8). Work counts are deterministic.

#figure(image("e8_e9a.svg", width: 100%), caption: [Left: early-stop saving against the solvable share of work (P-E8d). Right: coset yield gain against 1/f#sub[2] (P-E9a).])

- P-E8b, exactness: holds. Zero mismatches.
- P-E8a, solvable at least 3 times unsolvable: fails. Mostly 1.0 to 2.3.
- P-E8c, early stop at most 0.5 of full on solvable systems: fails pooled, at 0.94 [0.93, 0.95].
- P-E8d, saving at most the solvable share: holds. Savings are up to 43 % and down to −37 %.
- P-E9a, yield ratio near 1/f#sub[2]: holds in 7 of 9 rows. The F#sub[2]#super[19] explanation is refuted: f#sub[2] there is 0.45.
- P-E9b: holds in 7 of 9 rows.
- P-E9c: holds in 8 of 9 rows. The coset is 34 to 45 times costlier where the solving degree rises to 4, including the linear a = 0 case.
