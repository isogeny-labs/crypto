# Saved signed-stream group labels

Hydrate the previous fixture packet from repository root if needed:

```sh
python3 tools/artifacts/artifacts.py pull research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-replay
```

Run `bash research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-labels/check.sh` with exclusive ownership of its private build cache. It uses one build job, 3 GiB virtual memory and a 180-second command deadline, binds sources and inputs, and preserves all logs. It verifies saved streams and does not invoke a collector.

Library: `verify(input, independently_expected_header, Limits)`. The input first passes strict signed replay; the checker then reconstructs the source/transfer identities and every candidate/restart group-label equation. Use separately pinned input/custody for the expected header. `summary.json` and `equations.csv` retain the concrete checks; REPORT.md states the arithmetic, unverified properties and failed attempt.

`check-first.sh` is retained historical source and does not fail its process on a binding mismatch. Use `check.sh`, which checks both outcomes.

Raw check logs use the repository artifact store. Hydrate this packet with `python3 tools/artifacts/artifacts.py pull research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-labels` from repository root. Hashes and byte lengths are committed in each check directory.
