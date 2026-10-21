#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Projective chart run: 32-bit source order

*Session:* `ECBS1h6c29b9eb9279`. *Spec:* `ECS1h11990c07fc92`.
*Binary SHA-256:* `024f5094fb5d949ef93e6da3dfb404ffcbe79efbe5e1aa466886d393542a7b80`.
*Started:* 20261020T0645Z UTC. *Isolation:* L0 operation counts, `--cpus none --allow-busy`; wall time is unqualified.

The frozen session requested and completed 540 arm records on 180 exact-order targets: projective chart, unchanged affine two-choice collector, and source negation rho. All 540 records were source-verified; no failure or exhaustion was recorded. The independent `verify --replay-all` audit is in `replay_projective_chart_v2/order-genus-32.json`.

At M+0.5S+10I and 0.1M per declared native logical operation, the candidate/reference ratios are 0.626158 against affine and 2.734092 against rho. Each ratio divides sums of complete cold one-target costs over the same 180 targets.

#figure(image("cost-ratios.svg", width: 100%), caption: [Recorded pooled operation-cost ratios; 180 paired targets; session ECBS1h6c29b9eb9279. Dashed line is equal cost.])

*Raw evidence:* `sessions_projective_chart_v2/order-genus-32` (packed in `sessions_projective_chart_v2.tar.zst`); native analysis source `native/genus2_projective_analyze.rs`. Price-grid scope is M/S/I plus declared native logical operations; it does not establish wall speed or compare an external implementation.
