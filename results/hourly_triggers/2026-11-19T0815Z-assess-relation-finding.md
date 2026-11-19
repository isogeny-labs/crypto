# Hourly relation-finding benchmark trigger

UTC requested time: 2026-11-19 08:15

Run `.github/workflows/hourly-relation-finding.yml` on `hourly-runs`, using the existing frozen synthetic Koblitz GF(2^23), a=0, m=2, two-summand control. Command: `python3 scripts/hourly_relation_finding.py --build-timeout 1200 --run-timeout 900`.

Record timing, verified natural-arm relations per second, and success rate in `results/hourly_log.csv`; commit and push the measurement to `hourly-runs`; open a GitHub issue for execution failure or metric deviations exceeding 20% against `results/relation_baseline.json`.

This control does not measure ECC2K-130; do not extrapolate its rate to ECC2K-130.
