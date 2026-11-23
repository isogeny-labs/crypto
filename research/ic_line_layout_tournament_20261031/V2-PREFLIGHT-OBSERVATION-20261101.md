# Relation-solving tournament v2 preflight observation

Status: **stopped before the scientific tournament; no end-to-end ECDLP
speedup is established**.

Protocol v2 merged through PR #3151 at
`ea8e9ef71c3c82027dd9e01535302725519c3fca`. Its frozen staged rule required
one fixed, non-resumable L2 qualification before either scientific session.
Exactly that qualification was dispatched once in [workflow run
41271383665](https://github.com/isogeny-labs/crypto/actions/runs/41271383665).

The contracts and credential-free build jobs passed. The build produced native
binary SHA-256
`73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`
and immutable image digest
`sha256:3c4ad8bf8ee1bd1fdc327cb4c0e2597a1fc5ef1cdd889aa21e1b1294bdd22914`
from the exact protocol merge. The EKS experiment job then terminated failed:

| Field | Observed value |
|---|---|
| Tested source | `ea8e9ef71c3c82027dd9e01535302725519c3fca` |
| Workflow revision | `e443a60f00839aa84a7734053b8d6b5b0bad0f40` |
| EKS Job | `gh-41271383665-1` |
| Job UID | `06bc6798-a65b-4805-9e43-47f16be8d97f` |
| Launcher | `c4090885-9236-4b36-8dba-8cea6f746655` |
| Request SHA-256 | `9e4c461dd35f5301360efb1a67351d6097735857d79cc79ae055427684f70971` |
| Terminal result | state `failed`, remote exit 23, wrapper exit 26 |
| Successful Pod UID / results URI | absent / absent |

The public receipt does not identify a payload exit classification or exact
failed L2 predicate. In particular, the exit-42 `l2_isolation_gate`
classification established for the older v1 jobs must not be inferred for this
qualification. No manifest-verified accepted-L2 receipt exists.

The frozen rule therefore stopped v2. Qualification jobs planned/attempted were
1/1; conditionally planned scientific jobs attempted were 0/2; retries and
alternate seeds were 0. No authenticated session directory, performance row or
numeric comparison exists, so the frozen analyzer and `experiment.export`
could not be run without fabricating an input.

The machine-readable record is
[`preflight-observation-v2-20261101.json`](preflight-observation-v2-20261101.json).
The source-linked report, diagram and PDF are in
[`reports/run-20261101T0400Z-v2-preflight/`](reports/run-20261101T0400Z-v2-preflight/REPORT.md).
The continuing infrastructure obstruction is tracked in
[`problems/ic-line-layout-eks-failure-cause.md`](../../problems/ic-line-layout-eks-failure-cause.md).

Relation-yield pruning from PR #3091 remains a separate merged instrumentation
result. Neither that instrumentation nor this terminal preflight supplies the
missing complete, verified pipeline comparison.
