# Validation sweep D

Read the [2026-10-20 review qualification](review-20261020/README.md) together
with the [original report](REPORT.md). The qualification records native replay
limits, corrected runner failure handling, and the updated chronology of item 7.
The original report and its artifacts remain available with their original bytes.

Item 1 was qualified again on 2026-10-26: see the
[accounting addendum](ADDENDUM-20261026-item1-accounting.md), which restates the
cause of the cross-ISA result (eager against lazy accounting) and records that
the recommended charge fix was already on `main`.

The 48 root-finder failures of the same session were traced on 2026-10-26 to two
added native counters, not to any change in counted work: see
[`item1_rootfind_drift_20261026/RESULT.md`](item1_rootfind_drift_20261026/RESULT.md).
