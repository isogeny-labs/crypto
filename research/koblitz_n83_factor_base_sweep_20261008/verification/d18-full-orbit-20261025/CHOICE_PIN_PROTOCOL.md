# Full-D18 planted-control choice isolation

The shared-root full-D18 control with free selectors and phases reached its
300-second whole-process cap. The same model with both known selectors and
phase-zero wires pinned passed SAT and independent group replay. This
preregistered pair isolates those two dimensions, while keeping the exact
32,643-column S3 base, public fixture, four planted representative points,
summand/intermediate x pins, chained-S3 circuit, 100,000-conflict bound and
original-field row/rank/tamper checks unchanged.

Commit and validate the opt-in source before measuring. On the same L0 local
host, run these arms serially using the same checked object and binary:

1. `primary-full-d18-m4-l0-shared-phase-pinned BASE_DIR FIXTURE_DIR NEW_JSON`
   fixes all four seven-bit Frobenius phases to zero; all 32,643 selectors
   per summand remain free. The expected total CNF count is 2,807,780.
2. `primary-full-d18-m4-l0-shared-selector-pinned BASE_DIR FIXTURE_DIR NEW_JSON`
   fixes one checked representative selector per summand; all four phases
   remain free. The expected total CNF count is 2,807,756.

Each arm has a 300-second internal whole-process cap and independent
310-second outer cap. Retain fresh source/binary and input hashes, host and
resource facts, UTC interval, stdout/stderr, phase/shape/result or cap
records, and a raw-file SHA-256 inventory. On SAT, require the worker's
original-field model and row checks plus independent replay against the
archived four points. A cap or missing result is censored, not zero yield.

These are constructed controls with at least one choice dimension supplied.
Their different endpoints can localize search cost but cannot estimate free-
choice natural yield or select a factor base. Keep
`selected_best_total_runtime=null` until complete cold IC and matched-rho
gates pass.
