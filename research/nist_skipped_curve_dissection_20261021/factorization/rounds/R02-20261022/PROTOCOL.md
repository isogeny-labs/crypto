# R02: continue the three extension-twist factorizations

Frozen before R02 search, 2026-10-22. User-authorized continuation on the
three composite residuals from R01. Baseline: crypto commit
`99132d1fd1a9d9c125eec4c1a9761edce068076a`; the nine exact target integers,
six complete results, original hexadecimal fixture and historical raw
evidence are unchanged. Copy the baseline results and bind their SHA-256.

## Question and acceptance

Can complementary native smoothness methods, a higher ECM bound, or a
fresh PARI search on the now smaller cofactors produce certified proper
splits? This is a completion search on already seen inputs, not a sampled
comparison or a test of a historical skip explanation.

Reconstruct the targets from the hex fixture and replay R01 before search.
Screen each residual with native `ispower` and exact GCDs against all six
curves' fields, orders, traces, extension-twist orders and trace sums and
differences. A hint or reported factor is untrusted until exact division,
native prime certification and a separate verification process pass.
Unsplit composites remain explicit. Product equality alone is insufficient
for a complete prime factorization.

## Methods and fixed allocations

For each of the same three target lineages, selecting the largest current
composite for each next attempt:

1. Read-only FactorDB lookup; retain its response, using any factors only
   as untrusted hints. Do not submit values or results to the service.
2. GMP-ECM P−1: one base-2 attempt at B1=10000000, B2=10000000000,
   with a 300-second process limit.
3. GMP-ECM P+1: eight attempts at B1=1000000, B2=1000000000, using
   starting values 6, 7, 11, 13, 17, 19, 23 and 29, one run each and a
   90-second process limit each. Continue on the updated partition after
   a verified split; do not reset the eight-start allocation.
4. ECM: up to 100 further curves at B1=11000000 and the tool's default B2,
   with a 900-second process limit per cofactor attempt. Count `Using B1=`
   records, including factors found before stage 2. After a split use only
   the remaining allocation. Preserve unattempted composites in the partition.
5. PARI/GP: one up-to-600-second `factorint(C,6)` search on the largest
   residual, avoiding the initial ECM and rho/SQUFOF stages so MPQS can be
   tried directly when the native implementation supports the input size.
   Final ECM fallback is allowed by this flag. Do not call this a pure MPQS
   run or assume that an unsupported-size input was sieved.

Stop a target once every part is prime-certified. Run at most two factor
workers concurrently. GMP-ECM stage-2 memory is capped at 1024 MB per
worker, PARI's maximum stack at 2 GB. No paid or remote machines are launched.
Do not infer factor-size lower bounds from exhausted budgets or timeouts.

## Controls and evidence

Before production, check P−1 on `1009*1000003` at B1=100, B2=1000, base 2;
check P+1 on `1019*1000003` at B1=100, B2=1000, start 6. Both must produce
a proper divisor, checked natively. Keep the existing deterministic ECM
positive control and wrong-target rejection. Control failure is an
infrastructure/method-setup result, never a production negative result.

Retain exact inputs, versions, code/input hashes, commands, parameters,
residues, stdout/stderr, exit statuses, alarms, intermediate certificates
and partial partitions. Native `certify_parts.gp` and separate
`verify_factors.gp` replay must validate all final results. Use decimal
strings for large JSON integers. Preserve R01 without repacking or editing
its historical records. R02 gets its own deterministic archive, report,
graph, inspected PDF and truthful proof scope. No performance claim follows
from these unisolated searches. Any further search allocation is a new
protocol entry.
