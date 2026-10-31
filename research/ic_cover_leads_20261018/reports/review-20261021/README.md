# Corrected derived aggregates

These files reaggregate the frozen run-20261018T1900Z raw records. No new engine run was made. F4 means divide total rows and pivots by the 100 declared systems (one seed, 100 residuals, zero constructed inputs). The `step_records` column records 103 degree-6 steps and 98 degree-8 steps; it is not the denominator. The historical run directory remains unchanged. All four non-F4 aggregates remain byte-identical to that directory. Input SHA-256 values are in provenance.json.

Reproduce using a temporary directory whose `raw` symlink points to the historical raw directory, then run `bash aggregate.sh <temporary-directory> 100`. Plot with `python plot.py reports/review-20261021 run-20261018T1900Z`.
