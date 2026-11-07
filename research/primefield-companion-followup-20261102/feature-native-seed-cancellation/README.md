# Seed-free projected monomial initialization

The monomial seed exponent cancels under the supported order-two/order-four
projection. Direct initialization now uses raw division values; the twice-base
seed power is also removed. A constructor checks the modular exponent closure.
DERIVATION.md gives the parity argument and exact domain. The phase routine and
mandatory full-value validator remain, so this is a local work elimination.

Four selected native methods pass: paired-source agreement over499 subgroup rows,
100 splits,1375 advances,450 skips; limits; retained factor requests; and closure
on3479 algebraic partitions with3479 changed-exponent rejections. Each invocation
filters53 other tests. The algebraic partition panel is not an elliptic-curve
cohort and does not replace the original4563-case cohort. Claims
PCC-20261107-c45d3c (execution) and PCC-20261107-c64866 (derivation) are unsigned.
[pcc:unverified]

Exact source/compiler/binary hashes, commands and logs are in receipt.json,
source-lock.json and the four named logs. run-record.py contains replay commands;
use fresh outputs to preserve historical evidence. generate.py records guarded
transformations against the preceding component.

The first compile failed on an ambiguous integer type in the new test. Its exact
source, generator, source lock, environment, receipt and error log are retained in
initial-build/. That lock describes the failed source state, with transport.rs
supplied by the snapshot; it is not a lock for the corrected current source.
The fix types the test modulus array as u64. No native test failure occurred.

The report PDF was rendered and inspected. No cloud job, scalar recovery or
complete-cost comparison ran. Next: replace unused projected-path phase-value
computation with sufficient subgroup validation, then address residual work.
