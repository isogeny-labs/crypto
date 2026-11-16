# r29 fixed-base attempt-02 freeze

This amendment is scoped to coordinate bit 29. The original v9 attempt-01 failed and remains preserved, including 75 recovered S3 snapshots. Protocol v10 binds a fresh r29 attempt-02 output URI and records local execution on the current managed research worker because r34's v9 EKS job remains unschedulable under the available node affinity/anti-affinity constraints.

The mathematical generation scope is unchanged: all 2^27 selectors, three curve families, 128 shards per family, all affine lifts and both signs, same base reused for m=2..9. It uses the same native binary (SHA-256 `949b020e880f1b638d43fdd79bbebe842b2d91d8ccbabcb94a849f22629eb9f2`), six threads, an 8 GiB virtual-memory limit, 5,400-second producer cap, 5,700-second outer cap, and one attempt with no automatic retry.

Protocol SHA-256: `a6e2341f2c226b1a9c85ec72b6595f1d0926a54d2b842118694bf84cdab7fe81`.
Source index SHA-256: `b784eaf6729d548cbe0e021838a12c980a67b8a913581aa0c9baeebb588fb7c9`.
Launch command SHA-256: `cf050dc9f3c61399d27ee7ccc79ff2de4ffbb647a9285389adbfcff51f5c0c88`.

The full source and executable bundle is in `../source-v10/`. The run command verifies all indexed source hashes, the protocol sidecar, the instrument binary, and the runner before creating a fresh local output directory. It writes immutable snapshots to `s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r29-20261031-r2/attempt-02/`.
