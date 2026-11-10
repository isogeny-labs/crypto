# Infrastructure loss ledger

Source: harness review I9 (`docs/autoresearcher-harness-review-20261110.md`,
section 4 and 1.3). [pcc:unverified]

## Purpose

Every run that infrastructure killed, preempted, never launched or made
unreachable leaves one record, so lost time and money are counted by cause
instead of being scattered across session logs.

## Schema `crypto.infra-loss/v1`

One file per loss: `benchmarks/losses/<YYYYMMDD>-<slug>.json`.

- `id`, `observed_at`, `what` (one line)
- `route`: isolab-eks, runpod, local, ci, s3, other
- `cause`: spot_preemption, balance_exhausted, shell_time_limit,
  pool_blind_spot, access_denied, container_restart, quota, other
- `lost_cpu_hours`, `lost_usd`: a number or null, each with a derivation
  string. Only numbers present in the source log are used; otherwise null
  and "not recorded".
- `relaunched`, `relaunch_ref`
- `source`: session-log path and heading
- `pcc`: `[pcc:unverified]` by default

```
python3 tools/loss_ledger.py add --slug S --route R --cause C --what "..." --source "path#heading"
python3 tools/loss_ledger.py report [--since YYYY-MM-DD] [--json]
python3 tools/loss_ledger.py top-cause
```

## Weekly rule

Run `top-cause`. That cause gets exactly one fix that week; no other
infrastructure change is made for it.

## Metric

Lost CPU-hours per verified evidence record (a record that passed the
validation gate), falling week over week. Quantified losses are a lower
bound: records with null hours count as occurrences only. [pcc:unverified]

## Falsifier

If the ledger shows loss is under 10% of spend, the cost is elsewhere
(likely reasoning-tier tokens, see `docs/token-efficiency.md`), and the
ledger's weekly fix is dropped.
