# Corrections to frozen run inputs (2026-10-20)

Sidecar only. No file under this directory was changed for these corrections;
the run-input copies of `docs/ic/boundary_targets.json` stay byte-identical so
that each run's inputs remain what the run actually read.

## Superseded claim

The n=61 compact-orbit one-target online ratios, **12.9x (arm64 local) and 13.9x
(x86_64) medians**, compare IC against a strong rho with an **empty
distinguished-point table**. They are superseded (validation sweep B, verdict
108.1) and are not improvements: Bernstein-Lange rho at equal advice is 53-317x
faster online (`benchmarks/20261014-ic-next10-09-equal-advice`). The cold
conclusion (cold ratio 83.4; signed-Frobenius rho remains best; constant factor
only, no asymptotic claim) is unaffected.

## Affected copies (text `12.9x and 13.9x` in `single_target_crossover` / n=61 summary)

The `inputs/boundary_targets.json` copies under the runs listed by
`grep -l "12.9x and 13.9x"` over this directory, including:

- `20261016-n61-bl-reference-prepare/20261016T050621Z-363bc50e99/inputs/boundary_targets.json` (sha256 `800c7080adc1de4bee14e953efd708fa0e67a6a14407a622841595716f632437`)
- `20261016-n61-bl-reference-prepare/test-bl-ref-prepare-2026-10-16T050724.4100070000/inputs/boundary_targets.json` (same sha256)
- `20261014-koblitz-factor-base-n53/autolab_runs/20261014T014101Z-9264d0fee7/inputs/boundary_targets.json` (sha256 `ed5f7d2fd174d8b10805de3a44581c14758fa3a4d79e1aa08c2189fe13c06a3f`)
- `20261014-koblitz-decomposition-n31-m2-dim16/autolab_runs/20261014T043448Z-f2b0ba0fd2/inputs/boundary_targets.json` (same sha256)

Their per-run `ledger_pin.json` records the ledger hash at launch and is not
touched.

## Corrected live file

`docs/ic/boundary_targets.json` (hand-maintained; no generator writes it) and
`docs/ic/BOUNDARY_TARGETS.md`, with the old figures kept as "was".
