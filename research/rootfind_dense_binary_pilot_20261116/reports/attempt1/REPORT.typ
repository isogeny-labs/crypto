#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)

= First local K13 epoch: source-stamp mismatch

This report describes observations only. It makes no performance verdict or
scientific state transition.

*Session:* `ECBS1h3cf526729f36` · *Spec:* `ECS1hb376719703c9` ·
registered Koblitz E0 over $F_(2^13)$, subgroup order 2003. Eight fresh public
targets; five arms; four measured rounds plus one warmup; 200 records, all
answers verified. `ecbench verify --replay-all` reproduced 160 measured runs
with zero audit problems. There were no failed or timed-out executions.

All executions earned L0 on an Apple M4 Pro host. The binary hash is
`c4b63be34e52621d5d43e9cfcb2107f596b1148c7d06715fb7560dcae32e5d80`.
The run recorded commit `557f28ef...` from the binary's other worktree path,
while the protocol was committed at `984cf46d...`. The Rust source trees
match, but this commit-field mismatch invalidates performance admission.
`AMENDMENT-1.md` defines the custody-only rerun.

== Complete cold process wall, local diagnostic

#table(
  columns: (1.7fr, 1fr, 1fr),
  [Arm], [Median ms], [Complete work rows],
  [batch m=3], [7.714], [32/32],
  [batch m=3 A/A], [7.674], [32/32],
  [scalar rootfinding m=3], [7.589], [32/32],
  [F6-IC], [38.935], [32/32],
  [strong rho], [7.370], [32/32],
)

The derived ratio of summed batch/F6 process walls over 32 exact target-round
pairs is 0.1942; all 32 individual ratios are below 1. Batch/A/A is 0.9988
and batch/scalar is 1.0065. These are L0 observations from a custody-invalid
epoch. They do not establish the frozen F4/F5/F6 decision.

#image("paired-ratios.svg", width: 100%)

*Figure.* Session `ECBS1h3cf526729f36`; target-cluster ratios from four
matched rounds per target. The dashed line is parity. Source:
`reports/analysis.json`, computed by `analyze_local.py` from the session's
`records.jsonl`. High host load, no CPU reservation and the source-stamp
defect limit these observations.
