#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
= Run report: e7a_h72
_2026-10-18T16:04:56Z to 2026-10-18T16:05:21Z UTC. research/isogeny_conductor_gap_round5_20261012, lead E7._

*Spec* specs/e7a_h72.json. *Session* ECBS1hd38ca558d7e4 at commit 05ef9aff3 (git_dirty false), run with `ecbench run --wait` and audited with `ecbench verify --replay 12`: ok, 432 of 432 verified, 12 replays reproduced, 0 problems. *Host:* cloud container; isolation earned L2 on 380, L1 on 52 (spec asked for L2). *Measured:* mean S per arm and curve, 6 runs each; per-curve values in TABLES_E7.md.

#figure(image("s_by_arm.svg", width: 95%), caption: [Session ECBS1hd38ca558d7e4: mean S per arm against log#sub[2] \#E. Records sessions/e7a_h72/records.jsonl.])
