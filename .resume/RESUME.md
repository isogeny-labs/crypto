# Resume: crypto @ aburan28/kind-volta-4zo2ul

## Goal
Owner (2026-11-11): "run the whole thing" — the frozen 30-seed relation-oversampling run.

## Lead / hypothesis
research/logs_oversampling_20261111/protocol.json (frozen, sha256 aa928f27…3076). H1: some f>0 lowers
total counted cost C = trials + lambda*row_ops vs f=0. Pilot (run-20261111T0451Z) favoured H0.

## Spec
research/logs_oversampling_20261111/full/run_full.sh: ic oversample, cells C1-C4, 30 seeds (base +100),
fractions 0,0.1,0.25,0.5, --aa-control --logs-arm, release build. Local, unisolated (owner-requested;
no AWS identity). S3 checkpoint waived by owner; this file in Git is the checkpoint.

## State
Protocol frozen and committed; run next under a bench record.

## Runs in flight
Local `ic oversample` full run, cells C1-C4 sequential, session_01TCbLH7MBa3sEKfh3Trpi2R.

## Stop and success conditions
Fixed sample: stop when all cells finish 30 seeds; any unverified arm is a failure (reported).
Decision rule per protocol.json; no claim beyond it.

## Next steps
1. full/analyze.py; run report + graph + PDF under reports/run-<UTC>/; bench record close.
2. Commit on PR branch; report to owner.
