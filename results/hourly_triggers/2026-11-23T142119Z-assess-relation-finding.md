# Relation-finding benchmark trigger

UTC: 2026-11-23T14:21:19Z
Repository: isogeny-labs/crypto (aburan28/crypto redirect)
Branch: hourly-runs
Command: python3 scripts/hourly_relation_finding.py --build-timeout 1200 --run-timeout 900
Curve: public synthetic Koblitz GF(2^23), a=0, m=2 (frozen control; not ECC2K-130)
Metrics: timing, verified natural relations/sec, success rate
Output: results/hourly_log.csv
Baseline: results/relation_baseline.json
Alert threshold: absolute change >20%, or execution failure
