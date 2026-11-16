#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set table(stroke: 0.4pt)

= Run report: lazy orbit-member index, frozen claim sessions

Run 2026-10-18T05:40Z (UTC start of the first session). Observations only; interpretation is in `../../REPORT.pdf`.

*Command.* `bash research/ecbench_lazy_orbit_members_20261018/run.sh` (each session: `ecbench run --wait --spec docs/ecbench/specs/claim-koblitz-lazy-members-a<a>-n<n>.json --cpus none --allow-busy`, then `ecbench verify --replay-all`, `ecbench table`, `ecbench compare --save`). Specs from `make_specs.sh`; protocol `PROTOCOL.md`, committed in `0334afb11` before the run; sessions record `git_commit 0334afb11deefc91e54486c5c4fa4f89e4f5bb66`, `git_dirty false`, binary `2689809e3df73b85bfe17e098cd771c5a6100660e6273c2d844befc041beec02`.

*Host and isolation.* 4-vCPU Intel Xeon VM (`ECBENV2he1389885ccd0`), Linux x86-64, no CPU reservation (`--cpus none`): isolation L0 for every record. Counts are recorded at any level; wall times are not admitted.

*Arms* (all `*_full`): `rho-frob` (reference), `rho-strong`, `ic-orbit-c<c>-renew` (`mitm-frobenius-counted:m=2`), `ic-orbit-c<c>-renew-lazy` (`mitm-frobenius-lazy-counted:m=2`), `rho-frob-aa` (A/A). Columns c: 1, 1, 2, 3, 1, 2, 8, 90 for n = 13, 17, 19, 23, 29, 31, 37, 41. Targets per curve 32, 32, 32, 32, 32, 32, 16, 8; rounds 3 (n41: 2) plus one warm-up; public targets; target seeds 202610181001 to 202610181015.

#table(columns: 7, align: (left,left,left,right,left,right,left),
  [session dir], [session id], [spec id], [records], [status], [replays reproduced], [audit ok],
  [a0-n13], [ECBS1hbd6dafc7363a], [ECS1h1a6d5b8b28f4], [640], [verified:640], [480], [true],
  [a1-n17], [ECBS1h2a2479247f55], [ECS1h9e8c3ef15d11], [640], [verified:640], [480], [true],
  [a0-n19], [ECBS1h5149dde01d5c], [ECS1h78cec09ab00e], [640], [verified:640], [480], [true],
  [a0-n23], [ECBS1h7daf91450aa7], [ECS1hb916a4f4905b], [640], [verified:640], [480], [true],
  [a1-n29], [ECBS1h807488a20506], [ECS1h2618577802c0], [640], [verified:640], [480], [true],
  [a0-n31], [ECBS1h9649ebbe0eac], [ECS1h6d0169c9e7a0], [640], [verified:640], [480], [true],
  [a0-n37], [ECBS1h74c2d8ce9372], [ECS1h42662284fb49], [320], [verified:320], [240], [true],
  [a0-n41], [ECBS1h81fbae75c542], [ECS1hce58b43b3dee], [120], [verified:120], [80], [true],
)

*Completed vs requested.* 4 280 executions planned, 4 280 records written, all `verified`; no failed, timed-out or exhausted run. 3 200 replays reproduced, 0 problems (`../../audits/`).

*Measured values.* Paired cost ratios `eager / lazy` in `ecbench.gae` with the harness's cluster-bootstrap 95 % interval (`../../sessions/*/comparisons/`): n13 1.273 [1.267, 1.278]; n17 1.132 [1.116, 1.151]; n19 1.295 [1.283, 1.308]; n23 1.203 [1.189, 1.217]; n29 1.441 [1.427, 1.455]; n31 1.240 [1.223, 1.259]; n37 1.116 [1.106, 1.127]; n41 1.014 [1.013, 1.015]. `rho-frob-aa ÷ rho-frob` is 1 [1, 1] on every curve. Raw records: `../../sessions/<dir>/records.jsonl`; tables `../../tables/`.

#figure(image("fig1-improvement.svg", width: 100%), caption: [Eager ÷ lazy paired cost ratio per curve, sessions as in the table above.])
