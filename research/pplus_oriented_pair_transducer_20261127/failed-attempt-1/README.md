# Failed attempt 1

The Git checkpoint, v3 source hashes, and locked release build passed at
2026-11-27T06:02:54Z. The wrapper then exited 127 before candidate execution
because `/usr/bin/time` is absent on the executor.

`candidate.txt` is intentionally empty. No depth row, query result, verifier
output, analysis, or promotion classification was produced. Amendment 4 binds
the runner-only fix and permits one explicit post-push retry.

Verify the retained bytes with:

```sh
(cd research/pplus_oriented_pair_transducer_20261127/failed-attempt-1 && sha256sum --check --strict SHA256SUMS)
```
