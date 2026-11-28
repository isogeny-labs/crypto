# Provisional local trace-incidence runtime comparison

[Study and full tables](../../research/trace_incidence_20261116/measure_20261117/README.md) ·
[Run PDF](RUN_REPORT.pdf) · [Results PDF](RESULTS_REPORT.pdf).

These are byte-identical report/graph copies from the native study. At B128,
the new Bloom first-witness path is 5.27× cheaper than the prior pair control in
the median paired wall comparison; composition is 4.99× cheaper than
same-generator Horner. Complete setup, actual labels/rejections, prior misses
and recovery are included. The new pair path remains 3.06× matched rho cost.
PCC-20261117-c1a1e9 is unsigned. [pcc:unverified]

The user explicitly authorized shared-host local execution and temporary local
checkpoint/raw custody. No successful S3 admission/upload is asserted. Raw
archive location, size and hashes are in the study's results/LOCAL_CUSTODY.json.
LOCAL_RECORD.json records the execution without pretending to satisfy the
standard S3-backed bench.record/v1 schema. Canonical benchmark-index and
leaderboard promotion remain blocked on their existing validation requirements.
