# Capped-outcome derivation check

Before the degree-83 run, `derive-capped-outcome.jq` was exercised on a
synthetic 7-phase, 100-sample capped receipt. The positive fixture had
sequences 0 through 99, increasing completed decisions and conflicts,
`Fixpoint` at root, a last `XorPropagation` sample after 1,700,000 ms,
exit 124, no OOM and the exact 4 GiB / zero-swap envelope. The derivation
returned `UNKNOWN_wall_cap` with 99 decisions and 99 conflicts at the last
sample. A copy with only sample 99's completed-decision count changed from
99 to 0 exited 5 with the intended fail-closed error.

The exact synthetic files are retained under
[`deriver-fixtures/`](deriver-fixtures/). The discriminating mutation used
to make its negative input was:

```sh
jq -c 'if .sequence == 99 then .completed_decisions = 0 else . end' \
  progress.jsonl > progress-nonmonotone.jsonl
```

From this study directory, the positive invocation is:

```sh
jq -n \
  --slurpfile state deriver-fixtures/state-object.json \
  --slurpfile host deriver-fixtures/host-object.json \
  --slurpfile phases deriver-fixtures/phases.jsonl \
  --slurpfile progress deriver-fixtures/progress.jsonl \
  --arg start '2026-10-18T00:00:00Z' \
  --arg end '2026-10-18T00:30:00Z' --arg peak '2661863424' \
  -f derive-capped-outcome.jq
```

Substitute `progress-nonmonotone.jsonl` for `progress.jsonl` to reproduce
exit 5. This checks acceptance and a monotonicity rejection; the actual run
still needs its own container-state, phase, sample and hash checks.
