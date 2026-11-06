# Live CLI output and restart state

`icx run CURVE --verbose --json` keeps its final JSON document on stdout and
streams stage lines to stderr. `--progress-every SECONDS` sets the live
heartbeat interval and implies verbose output. `icx run` can save completed
repeats in a dedicated local directory and mirror each checkpoint to S3:

```sh
icx run p192 --bits 7 --repeats 3 --rho-runs 0 --max-trials 2000 \
  --state-dir ./icx-state/p192-7 --s3-uri s3://my-bucket/icx/p192-7 \
  --verbose --json
icx run p192 --bits 7 --repeats 3 --rho-runs 0 --max-trials 2000 \
  --state-dir ./icx-state/p192-7 --s3-uri s3://my-bucket/icx/p192-7 \
  --resume --verbose --json
```

The directory contains `manifest.json`, `icx.ckpt`, `progress.json`, and,
after completion, `result.json`. The checkpoint is checksummed and binds all
run inputs and the executable digest; restart skips only fully saved repeats
and reports their original measurements. An interrupted repeat restarts from
its beginning, including relation collection. The original rho reference is
reused on the same host;
cross-host resume is accepted only with `--rho-runs 0`. The resume annotation
names both hosts and states that per-repeat costs exclude restart setup and
storage. A fresh local directory with `--resume --s3-uri` restores S3 state.
One writer may use a state prefix at a time: the local lock cannot coordinate
separate hosts. The catalog, estimate, and validation commands are reports
rather than long-lived workflows.

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

`isogeny_walk traits` can also report each completed curve with `--verbose`
and a heartbeat with `--progress-every SECONDS`. Add `--checkpoint` to save a
checksummed `traits.ckpt` in `--out` after every completed curve, or add
`--state-s3-uri` to save locally and mirror it to S3. Resume with the same
walk, root curve, shard, and detector settings; a stopped detector repeats
its curve. A fresh `--out` can restore directly from S3:

```sh
isogeny_walk traits --curve p224 --dir ./walks/p224-12 --shard 0 --of 4 \
  --out ./traits/p224-shard-0 --checkpoint --verbose \
  --state-s3-uri s3://my-bucket/trait-state/p224-shard-0 --sync-every 60
isogeny_walk traits --curve p224 --dir ./walks/p224-12 --shard 0 --of 4 \
  --out ./traits/restored-shard-0 --resume --verbose \
  --state-s3-uri s3://my-bucket/trait-state/p224-shard-0
```

The manifest binds the routes file, root curve, shard parameters, and
executable digest. `progress.json` records the committed curve count.
`traits.jsonl`, `class_audits.json`, `metrics.json`, and `result.json` are
uploaded after the checkpoint, with the result last. Class audits repeat on
restart. Ordinary runs without these progress or state options keep the
parallel detector path. Use one writer per state prefix; the local lock does
not coordinate hosts. The existing `--store` prefix separately publishes
completed shard artifacts.

`crypto isogeny class build` and `resume` keep the class graph in SQLite and
now stream committed vertex counts with `--verbose` (or
`--progress-every SECONDS`). They can publish transaction-consistent SQLite
snapshots to a dedicated S3 prefix:

```sh
crypto isogeny class build --curve ./curve.json --out ./class/graph.sqlite \
  --steps 100 --verbose --state-s3-uri s3://my-bucket/classes/run-1 \
  --sync-every 60
crypto isogeny class resume --input ./class/graph.sqlite --steps all \
  --verbose --state-s3-uri s3://my-bucket/classes/run-1
```

The second command also works with a fresh local path: it downloads the
snapshot named by S3 `LATEST.json`, checks its SHA-256 and size, and validates
the stored plan before expanding. Each S3 upload writes an immutable SQLite
snapshot first and moves `LATEST.json` last. `graph.sqlite.progress.json` and
`graph.sqlite.result.json` remain local; the final result is uploaded after
the final snapshot. A stopped vertex expansion repeats from its last SQLite
transaction. Snapshot cadence may lag local transactions by `--sync-every`
seconds. The local lock does not coordinate separate hosts, so use one writer
per prefix. The original graph replay and completeness checks still decide
whether a class is complete.
