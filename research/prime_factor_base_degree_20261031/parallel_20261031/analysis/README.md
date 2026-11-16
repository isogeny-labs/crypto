# Frozen native analysis

`secant-panel-analysis AUDIT_DIR|- F4_DIR|- LADDER_DIR|- FRESH_OUTPUT.json`
reads emitted records, retains file hashes and failed/censored flow, and
computes exact SAT-stratified per-target comparisons. All four signed cases
must be present and verified before a secant target receives degree credit.
Missing trace metrics stay null. Identical candidate/control point sets are
labelled as an encoding comparison rather than a factor-base difference.

Four synthetic development tests check branch incompleteness, exact branch
coverage, null/duplicate handling, and the identical-point-set guard. They
contain no elliptic-curve panel data. No timing speedup or complete operation
cost is computed. Source is frozen before experiment outputs are read.
