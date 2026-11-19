# Relation-finding benchmark trigger

Requested UTC: 2026-11-19 10:16:04.

On branch `hourly-runs`, run `.github/workflows/hourly-relation-finding.yml` using `python3 scripts/hourly_relation_finding.py --build-timeout 1200 --run-timeout 900`.

Scope: frozen synthetic Koblitz curve over GF(2^23), a=0, two summands, 256 natural targets and correctness-control arms. This does not measure ECC2K-130.

Append elapsed time, natural verified relations/sec, success rate, host CPU, result and run URL to `results/hourly_log.csv`; commit and push the measurements to `hourly-runs`. On any failure or >20% change in throughput or success rate relative to `results/relation_baseline.json`, create an issue with measured details and run URL. The baseline is on EPYC 7763; account for different runner CPU model when interpreting deviations.
