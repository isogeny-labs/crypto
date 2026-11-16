# Matched factored-source accounting

Baseline/A-A use the preserved homogeneous seed-pair consumer. Candidate uses
R(c)=c^E R(1), with E precomputed in setup and two final seed powers. Six generated
modules outside source.rs are byte-identical.14 exact reverse transformations
recover the originals. Every field mul helper, including the residual and final
powers, is instrumented under the shared field-event definitions.

## Separate public exponent counters

This component extends the shared meter to21 operations and uses new
native-seed-factor-run/v1 and native-seed-factor-row/v1 schemas. The first17
operation indices keep their meanings. Four Source setup counters are appended:

| Counter | Definition | Checked r479 setup count |
| --- | --- | ---: |
| exponent_mod_products | Calls to ((a mod m)(b mod m)) mod m, m=p-1 | 21 |
| exponent_mod_adds | Calls to (a+b) mod m | 9 |
| exponent_divisibility_tests | Tested factor x mod3 ==0 in integer cancellation | 6 |
| exponent_exact_divisions | Integer factor division by3 before modular products | 3 |

These are primitive-call counts, not field operations or machine instructions.
The product event groups its operand reductions, multiplication and final
reduction. The remainder of exponent setup (partition inputs, integer indexing,
branches, additions and constants outside these closures) is not fully priced.
The categories are not summed into a scalar cost. Baseline/A-A have zero events
in these categories; preparation/query/verification have zero in all arms.

Native tests assert this complete setup vector, location restrictions and all8
complete recovery receipts against baseline. Both source arms have zero query
Source inversions; geometry, phase and value inversions remain. Non-source
arithmetic remains identical. Two selected methods pass,84 inherited methods
filtered. Both optimized binaries compile;72 source/data files locked beforehand.

## Strict analysis and single-target accounting

analyze.py validates all31 records, exact headers/schedule,21-slot dimensions,
u64 types, stage/component placement, independent recovery results and full A/A
equality. It requires the fixed r479 setup vector and rejects exponent events
anywhere else. Seven synthetic methods pass; no panel data was collected.

cold_analyze.py first invokes that validator, then derives per-target field counts
as setup+preparation+query+verification. The prepared decoder is immutable and
each recover call constructs fresh query state. Adding the fixed stages to each
query gives a single-target arithmetic accounting view, not a measurement of
cold process time or cold allocator/cache behavior. No amortization is used for
the primary verdict. Query-only results, fixed costs and all raw counters remain.

Two additional synthetic methods check that increased setup can reverse a query
win into a primary regression, that setup inversions affect break-even weights,
and that A/A failures remain rejected. The weighted model is symbolic and covers
only field products/inversions. Exponent integer events, other arithmetic,
allocations and machine costs remain distinct; no total-cost or time claim.

The separately bound comparison draft names cold_analyze.py and its imported
analyze.py hash through analysis-lock.json. It remains unfrozen pending pinned
preflight, quota/charged-reserved allowance and authoritative admission. The old
preflight binary imports older arms and cannot substitute for this binary.
Original4563-case cohort and earlier panel identities remain unchanged.

Sources reviewed: prior native accounting meter/generator and strict parser;
feature-native-seed-factor/DERIVATION.md; the canonical experiment-protocol
instructions. This extends event accounting, not the mathematical identity.
