#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Developmental projective run: retained, not ranked

*Started:* 2026-10-20T06:04Z UTC. *Sessions:* ECBS1h2f24466a4450, ECBS1h14baf6b3d5e7, ECBS1hb1ce8f6052b8, ECBS1hd2e8b326493e, ECBS1h58c6719ee3d5, ECBS1hb0d79ccffd4e, ECBS1h98c0e6ebf06f.

The first projective-chart run completed seven ecbench sessions with 540 source-verified arm records at each of the 16–40-bit order sizes. All 3,780 records were independently replayed; the receipts are in `replay_projective_chart/` and the raw archive is `sessions_projective_chart_unsealed_v1.tar.zst`.

The adapter inserted two zero-valued projective counters and an addition-backend detail label into the unchanged affine arm. Arithmetic and recovered scalars were unaffected, but the old method's record was changed. This archive is therefore excluded from all cost rankings. The corrected run is in `sessions_projective_chart_v2.tar.zst`; its independent audit enforces exact baseline-counter equality.

#figure(image("records.svg", width: 100%), caption: [Recorded verified arm counts in the unsealed developmental sessions; no operation-cost conclusion is drawn from this archive.])

*Isolation:* busy, unpinned L0 operation run; wall times are unqualified. *Reference:* the corrected projective chart [results report](../results-20261020/report.pdf).
