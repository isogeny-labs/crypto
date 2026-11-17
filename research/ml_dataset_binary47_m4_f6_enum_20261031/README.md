# Ordinary-binary n47 m4 four-cell dataset pilot

The frozen [protocol](protocol.json) and bound [plan](plan.json) specify one
ordinary-binary curve seed at n=47, m=4, both quadratic twists, one generator,
one key, and native `f6`/`enumerate` methods. The four native indices are
`solver + 2*twist`; methods share one public instance within each twist.
The protocol and plan were committed at `d1eebc768e3e` before any attempt.

The bounded [controller](campaign.py) admitted the pinned clean native binary,
exact committed protocol and plan bytes, XFS output, and checkpoint credentials.
Each index had one 30-second native attempt, a 90-second outer cap, 2 GiB
memory and 64 MiB trace limits, and an 8 GiB free-space gate. It retained
native verify/report/strict-export results, complete restricted archives,
content-addressed S3 PUTs, fresh GET/hash checks, and Git
[checkpoints](checkpoints/). Do not rerun the terminal campaign.

The [run report](report/run.md), [chart](report/outcomes.svg), and
[PDF](report/run.pdf) describe the observations. The
[official export](experiment-export.json) has four attempted rows, three
completed and one native timeout, with no exclusions. The
[independent audit](AUDIT.json) replays four archives and 128 native artifact
hashes. [Cost observations](cost-observations.jsonl) preserve phase telemetry
without promoting raw scoped PMU counts to complete instruction-to-solve labels.
The separate [S3 sample](S3-READBACK-SAMPLE.json) freshly re-read archives
0 and 3 after the producer's own readbacks.

`f6` is the geometric four-sum pair index, not a Gröbner F6 algorithm.
Enumeration is bounded; its timed-out twisted cell is censored. The ordinary
binary coefficients have degree 47, so public metadata correctly records no
nontrivial curve-preserving Frobenius action.
