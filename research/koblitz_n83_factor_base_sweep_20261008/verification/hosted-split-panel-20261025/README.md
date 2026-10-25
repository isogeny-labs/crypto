# Primary N83 K600 six-summand 5+1 access pilot

This protocol tests whether the retained primary-curve K600 base can supply a
natural relation when five summands come from the sequential base and one
comes from a residual pool. It uses the exact public corpus trial, three
S3-backed archives, pinned mathematical source, image digest, build limits,
search cgroup, 20,000-conflict cap, and 1,800-second worker wall cap from the
[4+2 pilot](../hosted-sixsum-access-20261025/README.md). The Gray and hash
arms again have the same base; their residual pools contain 97 and 600 signed
Frobenius orbits respectively. Source formulas admit both 5+1 models below
the 150,000-variable and 12-million-domain-clause caps. The support census
counts 1,315,342,556,830,474,064,023,551,840 Gray and
8,136,139,526,786,437,509,424,032,000 hash point multisets for this split.
Those counts are necessary-support diagnostics, not observed relations.
The mathematical source is pinned to
`cb98e85d1c1c1afdd5000bc70eea00969ee86c6c`; the dispatch checkout
supplies only the new protocol, auditor, and workflow.

The frozen [v4 grid](../../PAIRED_FRONTIER_V4.md) addresses these exact
source configurations at ordinals `11817231005` (Gray) and `11986592285`
(hash). The protocol pins the v4 design hash, and a focused test decodes
both ordinals back to their curve, streams, 5+1 split, SAT backend, symmetry,
thread, and full-width linear-algebra settings. The v4 design's historical
`available_unexecuted` disposition is immutable; any hosted execution is
recorded separately and cannot by itself mark the full pipeline complete.

`protocol.py` regenerates the exact protocol JSON from the retained parent
plan and support census (SHA-256
`c1b039657af2821d5493ea840cc45642425faa366dc44d0c0c5162c6884a1526`).
`run.py` invokes the hash-pinned hosted stage runner
with this directory's protocol and 5+1 auditor; it retains independently cold
builds/imports for the two arms. `audit.py` checks all source, resource,
capacity, disposition, and candidate-domain fields, including five base
indices, one residual point, and the original group equation. Failed and
censored outcomes remain explicit. The conservative full charge is 8,000
seconds including 1,800 seconds of public base preparation and 3,100 seconds
per arm. The fully charged index-calculus runtime and selected best base
remain unset pending natural relations, rank, linear algebra, unseen-target
online work, and matched rho.

The reused stage runner retains its `n83.hosted-sixsum-access-...` outer
receipt schema. The `n83.hosted-sixsum-split-protocol/v1` protocol, worker
arguments, and search config identify the actual 5+1 split; an audit must
check all three rather than infer the split from the outer schema name.

The 5+1 split is a separate source-pinned screening run. Comparing its wall
time with the prior 4+2 dispatch is descriptive because the hosted runs use
different runner allocations; a causal split comparison would need matched
rotated cold trials. The prior 4+2 run remains frozen and independently
replayable at its original dispatch commit.

Source and no-launch controls:

```sh
python3.14 research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-split-panel-20261025/protocol.py
python3.14 research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-split-panel-20261025/test_controls.py
actionlint .github/workflows/n83-hosted-paired-gate.yml
```

Once this protocol is committed, dispatch it with:

```sh
gh workflow run n83-hosted-paired-gate.yml \
  --ref codex/n83-aws-resume-20261024 -f experiment=six_sum_split
```

The workflow retains preparation, archive bytes, Docker stage receipts,
search config/results, worker binaries, and outer outcomes even on failure.
Any apparent positive result requires an independent replay of the uploaded
artifact before it can inform a factor-base ranking.
