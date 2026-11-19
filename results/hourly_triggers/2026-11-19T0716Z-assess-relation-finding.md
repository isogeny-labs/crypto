# Hourly relation-finding benchmark trigger

UTC requested time: 2026-11-19 07:16

Runs `.github/workflows/hourly-relation-finding.yml` on `hourly-runs` with the frozen Koblitz GF(2^23), a=0, m=2 experimental control. The harness (`python3 scripts/hourly_relation_finding.py --build-timeout 1200 --run-timeout 900`) records runtime, relations/sec, and natural-arm success rate, and compares against `results/relation_baseline.json` with a >20% threshold.

Do not interpret these synthetic control measurements as ECC2K-130 results.
