# Projected adaptive monomial transport

The paired source integrates the two-primary projection into initialization,
transport, adaptive advance and skipped-sign transitions. It retains the full-value
source as an obligatory validator for every evaluated sign. See DERIVATION.md.
The public wrapper in lib.rs accepts only order-two/order-four projections.

Two selected native test invocations pass: 499 complete-subgroup rows,100 split
windows,1375 advances,450 skips, plus limits/rejection checks. Each invocation
filters51 other methods. Claim PCC-20261107-8ba257 is unsigned producer evidence.
[pcc:unverified]

Fixtures: F5 a2 b1 P(0,1), subgroup7; F7 a0 b3 P(1,2), subgroup13;
F509 a2 b10 P(3,68), subgroup479. The original4563-case cohort is unchanged.
No scalar recovery or comparative panel was run. Both paths share inherited
geometry and residual code; this is not an independent verifier.

Source/compiler/binary/dependency hashes and exact commands are in receipt.json,
source-lock.json and limits-receipt.json. generate.py applies exact guarded
transformations to the preserved adaptive monomial source; generation.json binds
inputs and outputs. Raw logs retain compiler warnings and test-runner formatting.
The report PDF was rendered and inspected; source and vector diagram are included.

Replay: compile lib.rs with rustc --edition=2021 -O -C overflow-checks=yes --test,
then invoke tests::projected_split_windows --exact --test-threads=1 and
 tests::limits_and_rejection --exact --test-threads=1. run-record.py records the
first invocation; limits-receipt.json records the second. Use a fresh output
location when reproducing historical evidence.

Next: determine which zero/domain checks remain necessary when projected factors
vanish; develop the projected residual; then evaluate complete work. The paired
wrapper is deliberately a development path and retains the full validator cost.
