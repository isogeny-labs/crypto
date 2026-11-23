#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)

= K13 local diagnostic: source-bound repeat

This report describes observations only. It makes no performance verdict or
scientific state transition. The original custody-invalid epoch is retained;
the repeat uses the same targets and is no independent holdout.

*Session:* `ECBS1h08f6d5475be8` · *Spec:* `ECS1hb376719703c9` · registered
Koblitz E0 over $F_(2^13)$, subgroup order 2003. Eight public targets, five
arms, four measured rounds plus one warmup. All 200 records verified, no
failures or timeouts; `ecbench verify --replay-all` reproduced all 160
measured executions. Every arm had 32/32 complete `bitop/v1` work rows.

Binary SHA-256: `c4b63be34e52621d5d43e9cfcb2107f596b1148c7d06715fb7560dcae32e5d80`.
The worktree-local path recorded commit `02113398fc2c4544db35f879b6a55add1bff9767`;
`git_dirty=true` reflects the checkpoint and run artifacts, with no compiled
Rust source change. All executions earned L0 on an Apple M4 Pro with no CPU
reservation.

== Complete cold process wall, local diagnostic

#table(
  columns: (1.8fr, 1fr, 1fr),
  [Arm], [Median process ms], [Median method ms],
  [batch m=3], [8.277], [0.419],
  [batch m=3 A/A], [8.187], [0.420],
  [scalar rootfinding m=3], [8.274], [0.449],
  [F6-IC], [39.482], [31.655],
  [strong rho], [7.794], [0.106],
)

The derived ratio of summed batch/F6 process wall on 32 exact matched pairs
is 0.2078 (all 32 pair ratios below 1; pair range 0.123–0.477).
Batch/A/A is 1.0245, batch/scalar 1.0163, and batch/rho 1.0772. The
A/A pair range is 0.809–1.374. The first epoch's 0.1942 ratio is
custody-invalid and is not pooled.

#image("paired-ratios.svg", width: 100%)

*Figure.* Session `ECBS1h08f6d5475be8`; four measured rounds per target,
shown as ratios of target-summed complete process walls. Source:
`reports/custody-rerun/analysis.json`, derived by `analyze_local.py` from
the sealed `records.jsonl`. L0 timing and the lack of an independent host
replay prevent a qualified fixed-panel claim. The frozen EKS panel still
includes matrix F4, F5 and inherited F4 on its own targets.
