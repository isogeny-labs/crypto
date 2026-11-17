# Refusals and gate outcomes, local diagnostic run (2026-11-17)

Local diagnostic, non-admitted, never a leaderboard row. Owner decision
2026-11-17: AWS does not work; Magma is not available. All results [pcc:unverified].

## scripts/checkpoint.sh init

```
$ scripts/checkpoint.sh init
checkpoint: .resume/RESUME.md already exists
EXIT=1
```

The existing `.resume/RESUME.md` is a tracked file describing a different effort
(codex/retained-chain-ic-20261109). It was not overwritten by this subagent.

## scripts/checkpoint.sh gate / push

```
$ scripts/checkpoint.sh gate
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /root/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
EXIT=1

$ scripts/checkpoint.sh push
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /root/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
EXIT=1
```

No S3 checkpoint exists for this run. Continued per owner decision (AWS does not work).

## tools/benchrec.py init

Not refused:

```
$ python3 tools/benchrec.py init --slug gb-solver-shootout-local-diag --problem groebner:semaev-K17-m3-plain-l5-a1
benchmarks/records/20261117T0722Z-gb-solver-shootout-local-diag.json
EXIT=0
```

`benchrec.py upload` (S3) is expected to fail; see below if attempted.

## False start (launcher error, not a repo-hook refusal)

First launch at 2026-11-17T07:22Z exited 1 after 0.11 s wall:
`FileExistsError: [Errno 17] File exists: '.../run-local'` from
`out.mkdir(parents=True, exist_ok=False)`, the first statement of
`run_local.main_flow`, because this REFUSALS.md had been written into
`run-local/` beforehand. No oracle, seed, rho or solver call ran. The
benchrec record logs this command with exit 1. The pre-run files were moved
to `run-local.launch/` and the driver was relaunched at 07:23Z unchanged.

## Main run

Relaunched 2026-11-17T07:23Z via
`benchrec.py run <record> -- python3 run_local.py run --out run-local`;
driver finished 08:19:51Z, `/usr/bin/time` exit status 0, elapsed 56:17.76.

## tools/benchrec.py upload (S3)

```
$ python3 tools/benchrec.py upload benchmarks/records/20261117T0722Z-gb-solver-shootout-local-diag.json --all
benchrec: no AWS credentials here; CI uploads files that are in Git, push larger ones with tools/artifacts/artifacts.py push where credentials exist
EXIT=1
```

## tools/benchrec.py close (refused; record left OPEN)

```
$ python3 tools/benchrec.py close benchmarks/records/20261117T0722Z-gb-solver-shootout-local-diag.json --status measured --summary "LOCAL DIAGNOSTIC, non-admitted, [pcc:unverified]: amendment-1 frozen run, 996 rows, all calls ok"
benchrec: benchmarks/records/20261117T0722Z-gb-solver-shootout-local-diag.json: a measured record needs at least one metric
benchrec: benchmarks/records/20261117T0722Z-gb-solver-shootout-local-diag.json: a measured record names its report PDF
benchrec: benchmarks/records/20261117T0722Z-gb-solver-shootout-local-diag.json: command 1 has no successful integer exit code; use invalid/censored or an explicit pinned historical exclusion
benchrec: benchmarks/records/20261117T0722Z-gb-solver-shootout-local-diag.json: a measured record needs at least one metric
EXIT=1
```

The record was not hand-edited. Command 1 is the false start above (exit 1).
Closing needs a decision by the main session: metrics and a report PDF
(analysis script PA-5 not written), and how to treat the false-start command.
