# Raw evidence and reproduction

The completed `panel-v2` contains all 42 polynomial collectors and separate
auditors. Native summary: `results/panel-v2/native-summary.json`; its full
SHA-256 is in `native-summary.sha256`. The actual run starts at
2026-10-24T20:03:47Z and terminates successfully at 20:39:43Z. Counts are
correctness/counter diagnostics; elapsed values are not admitted timing.

## Complete raw archive: packaging pending

`pack-verify.sh` selects these ten terminal raw directories explicitly:

- `native-completion-20261024T1934Z`
- `native-gate-v2`, `native-gate-v3`, `native-gate-v4`, `native-gate-v5`
- `native-gate-v6`, `native-gate-v7`
- `native-gate-journal-20261024T1940Z`
- `panel-v1` (stopped before any native collector; nested-reservation receipts retained)
- `panel-v2` (all 42 collectors and audits complete)

The archive also includes the exact 38-file compiled-source snapshot rebuilt
from committed producer `57cde8bee873b43d50cf2dc4e1e74a6962604f1a`, the two
mixed oracle reference files and original summary for each of 21 cases, and
the unchanged independent point-replay summary. Source checks must pass.
Inside manifests bind all payload bytes. Fresh extraction, every native
file comparison and byte-identical deterministic repacking are mandatory.
The archive digest, byte count and canonical store publication remain pending;
a local archive is not described as remotely published.

## Existing dependencies remain unchanged

- `store:evidence/twist-index-calculus/20261023-image-base-runs.tar.xz`:
  SHA-256 `1e0a2139c9a32571a9476e4b9579f73da91e93e4a8cc7c2143d2bf4536c36038`;
  28,241,048 bytes; complete 189+21 controls and admitted old cold rounds.
- `store:evidence/twist-index-calculus/20261024-joint-image-runs.tar.xz`:
  SHA-256 `54b31b3d73602d7758dad263939bdfdcbd8c305df2299fe7beb498029090b729`;
  1,153,392,456 bytes; all 43 joint-study raw directories and independent
  scalar/point censuses. No BSGS or those censuses were repeated.

Reports and benchmark copies are rendered only from the recorded native
summary. Their PDFs must be inspected page by page before committing.
Benchmark copies remain unlisted: target descent, a complete price and
VALIDATION/rho/GAE obligations are still open.
