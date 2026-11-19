# Hourly relation-finding benchmark trigger

UTC requested time: 2026-11-19 09:11

Run `.github/workflows/hourly-relation-finding.yml` on `hourly-runs`, using the established frozen synthetic Koblitz GF(2^23), a=0, m=2 two-summand control. Command: `python3 scripts/hourly_relation_finding.py --build-timeout 1200 --run-timeout 900`.

Append timing, natural-arm verified relations per second, success rate, and host provenance to `results/hourly_log.csv`. Commit and push measurements to `hourly-runs`. Create a GitHub issue on failure or metric deviations greater than 20% against `results/relation_baseline.json`.

Do not interpret this control as ECC2K-130 performance. Keep the baseline immutable for longitudinal comparison and note differing runner CPU models.
