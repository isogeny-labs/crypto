# Live CLI output and restart state

`icx run CURVE --verbose --json` keeps its final JSON document on stdout and
streams stage lines to stderr. `--progress-every SECONDS` sets the live
heartbeat interval and implies verbose output. The current `icx run` pipeline
does not checkpoint a partially completed relation collection or a completed
repeat; an interrupted run still needs to restart that computation. The
catalog, estimate, and validation commands are reports rather than long-lived
workflows.

`isogeny_walk walk` has an opt-in, checksummed state directory in its `--out`
path. For example:

```sh
isogeny_walk walk --curve p224 --max-ell 5 --max-curves 12 \
  --out ./walks/p224-12 --verbose --checkpoint-every 0 \
  --state-s3-uri s3://my-bucket/walk-state/p224-12 --sync-every 60
isogeny_walk walk --curve p224 --max-ell 5 --max-curves 12 \
  --out ./walks/p224-12 --resume --verbose \
  --state-s3-uri s3://my-bucket/walk-state/p224-12
```

The first invocation may be interrupted after a completed breadth-first
expansion batch. A resumed invocation restores `walk.ckpt` and continues at
the first unexpanded node. The checkpoint includes nodes, certified edges,
failures, and walk counters; work inside the interrupted batch repeats. The
modular polynomials and final class audits are rebuilt on restart. `--threads`
and checkpoint cadence may change; the curve, prime set, caps, seed, and audit
settings must match. A resumed graph is independently checked through its
route records before final output is written.

The local directory contains `manifest.json`, `walk.ckpt`, `progress.json`,
`curves.yaml`, `isogeny_routes.json`, and `walk.json`. It is locked against
another writer. A fresh local directory with `--resume --state-s3-uri` restores
the checkpoint from S3. The state prefix is separate from the existing
`--store` prefix, which publishes completed artifacts under a write-once run
identifier. `--state-s3-uri` uploads each checkpoint object explicitly and
publishes `walk.json` last; success requires the final upload to finish. Use a
dedicated prefix per run.
Only one host should write a given state prefix at a time; the directory lock
does not coordinate separate hosts.
