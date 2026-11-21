# Before-data amendment 1: lazy binary scan sums

Status: specified before any study execution or outcome row, 2026-11-15.
This amendment applies to the `rootfind-negfold-batch` candidate in
[`SPEC-K13.json`](SPEC-K13.json). It preserves the original
[`PROTOCOL.md`](PROTOCOL.md) and [`publication.json`](publication.json) as
immutable records of the earlier eager-sum source. Execution under this
amendment requires a separate `publication-amendment-1.json` receipt that
checks the source revision, protocol, spec, this amendment and all listed
source inputs against the pushed commit.

## Source change and reason

The candidate's `m=3` scan still subtracts eight consecutive factor-base
points in one batched inversion and probes their table keys in scalar order.
At source revision `c30a899bce6e74fe4574f042016ed7b6ddd1e1ac`, it uses
`FastCurve::add_many_lazy` to compute each final abscissa and slope, and
`PairTable::probe_binary_lazy` completes an ordinate only when its table key
is present. This replaces the eager `add_many` scan in the source pinned by
`publication.json`. The prediction is fewer field multiplications on table
misses; this is a mechanism prediction, with no measured advantage asserted.

The original protocol's phrase "probes completed sums in scalar order" now
means "probes final abscissae in scalar order and completes an ordinate on a
table-key hit." The table build, method identity, fixed targets and seeds,
six arms, 336 planned executions, controls, primary complete cold bitop
metric, secondary wall admission rule, stopping limits, budget admission,
and inference policy are unchanged. A completed eight-point block still
charges all eight abstract additions, including candidates after the first
hit. Every hit still re-adds its stored pair and resolves sign as the scalar
probe does. The existing 263-point release test is a correctness check, not
study data.

## Discriminating outcomes

If the exact relation controls differ, retain the first differing execution
and withhold a correctness-preserving interpretation. If they match but
complete bitop or admitted wall cost fails the original all-target criterion,
retain that measured outcome and do not claim an advantage. If the criterion
holds, require the original audit, second-host replay and progress-fidelity
claim record before promoting any quantitative conclusion. No result from the
earlier eager-sum revision is pooled with this revision.
