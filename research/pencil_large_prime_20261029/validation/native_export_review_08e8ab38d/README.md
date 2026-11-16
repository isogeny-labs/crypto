# Independent native export code review

The review binds delivered source `08e8ab38d3de525779da57f27e14e7e9a81a2d21`. Its original paths describe the scratch reproduction environment; the original review, patch and logs are copied here unchanged. The patch applies to that source's native canonical export tests. Passing reproducer tests demonstrate the defects, not corrected behavior or empirical success.

Six findings cover verification, native analysis input completeness and provenance, warmups, backend metadata, exact counters and checkpoint histories. The prior 28-test suite and supplemental test remain valid records of those commands but did not cover these defects. Auto-merge was disabled before correction. No cohort or cloud job used this exporter.

Correction evidence must identify its own source and regression tests; this review is preserved without retroactive edits.
