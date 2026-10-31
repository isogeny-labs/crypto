#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set table(stroke: 0.4pt)

= Run report: two orbit columns under the lazy member index

Run 2026-10-18T18:00Z (UTC start of the first session). Observations only; interpretation is in `../../REPORT.pdf`.

*Command.* `bash research/ecbench_lazy_columns2_20261018/run.sh` (each session: `ecbench run --wait --spec docs/ecbench/specs/claim-koblitz-lazy-c2-a<a>-n<n>.json --cpus none --allow-busy`, then `ecbench verify --replay-all`, `ecbench table`, `ecbench compare --save`). Specs from `make_specs.sh`; protocol `PROTOCOL.md`, committed in `760c672a5` before the run; sessions record that commit (`git_dirty false`) and binary `ae108d54…`.

*Host and isolation.* 4-vCPU Intel Xeon VM, Linux x86-64, `--cpus none`: isolation L0 for every record. Counts are recorded at any level; wall times are not admitted.

*Arms* (all `*_full`): `rho-frob` (reference), `rho-strong`, `ic-lazy-c<c0>` (`compact-orbit-scan:columns=<c0>`, `mitm-frobenius-lazy-counted:m=2`, `stride-renew`; c0 = 1, 1, 3, 1, 8 for n = 13, 17, 23, 29, 37), `ic-lazy-c2` (the same with `columns=2`), `rho-frob-aa` (A/A). Targets per curve 32 (n37: 16), 3 rounds plus one warm-up, public targets, target seeds 202610182001 to 202610182009.

#table(columns: 6, align: (left,left,left,right,left,right),
  [session dir], [session id], [spec id], [records], [status], [replays reproduced],
  [a0-n13], [ECBS1h5f7f47653898], [ECS1hbe4a2df36fc5], [640], [verified:640], [480],
  [a1-n17], [ECBS1had6c4dfa75fc], [ECS1h322728aff7a6], [640], [verified:640], [480],
  [a0-n23], [ECBS1h88a016cfa7c0], [ECS1h980f02d2fbe5], [640], [verified:640], [480],
  [a1-n29], [ECBS1h486ae2ee8d06], [ECS1he1726b2155de], [640], [verified:640], [480],
  [a0-n37], [ECBS1hbef0622fd5f3], [ECS1hce97051a6418], [320], [verified:320], [240],
)

*Completed vs requested.* 2 880 executions planned and written, all `verified`; no failed, timed-out or exhausted run. 2 160 replays reproduced, 0 problems (`../../audits/`).

*Measured values.* Paired ratios `frozen c ÷ c = 2` in `ecbench.gae` with the harness's cluster-bootstrap 95 % interval (`../../sessions/*/comparisons/`): n13 0.637 [0.630, 0.645]; n17 1.176 [1.063, 1.297]; n23 0.778 [0.695, 0.874]; n29 0.644 [0.625, 0.665]; n37 0.246 [0.200, 0.309]. `rho-frob-aa ÷ rho-frob` is 1 [1, 1] on every curve. Raw records `../../sessions/<dir>/records.jsonl`; tables `../../tables/`.

#figure(image("fig1-ratio.svg", width: 100%), caption: [Frozen-c ÷ two-column paired cost ratio per curve, sessions as above.])
