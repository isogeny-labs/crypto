# Prime root records and certificate scope

The native `root` adapter for prime curves at m=2 emits intermediate records
while solving each univariate S3 problem. It uses the existing word-field
arithmetic range, including p=2^61-1. Binary root behavior and the legacy GLV
root APIs remain unchanged. Other arities retain their explicit unsupported
outcomes.

For each fixed factor-base x coordinate, the adapter records the original S3
coefficients, modular powers, polynomial products/divisions, Euclidean steps,
split proposals and results, linear-root extraction, and final distinct roots.
Completed modular powers and GCDs produce live payloads. Raw event streams and
`root-call-NNNN.json` remain solver/provenance artifacts, excluded from static
training inputs. Existing partial-artifact capture can publish acknowledged
prefixes; it does not restore the in-memory solver stack.

`prime-root-certificates.json` binds those calls to the public target/modulus,
ordered actual factor-base domain and frozen cell budget. The independent
checker uses separate arithmetic to establish the monic
`gcd(f, X^p-X)` and its equality to the product of distinct returned linear
factors. A zero polynomial has an explicit all-field result; the dataset then
checks only the actual finite factor-base coordinates for point lifts.
Multiplicities are outside the distinct-root certificate. A root set does not
by itself establish a PDP witness or a recovered scalar.

The effective aggregate work cap is `min(config.work_limit, 100000)`. It charges
the 15 field operations in coefficient setup, native field additions,
subtractions, multiplications and squares, original-polynomial evaluation,
and group-lift additions across calls. Inversion exponentiation operations
are included. Primality/curve validation, serialization and certificate replay
are excluded from that counter. Native calls also have bounded split attempts,
events, polynomial degree and event bytes. Cap exhaustion remains censored;
an I/O sink failure stops the computation and preserves only its acknowledged
artifacts. The event cap counts an attempted but unacknowledged event too.

The checker validates each supplied event identity and recomputes its recorded
counter tally. It does **not** establish that the event stream contains every
operation from a native execution. Artifacts explicitly set
`execution_trace_complete_verified=false` and
`total_operation_count_verified=false`. Producer counts are neither retired
CPU instructions nor an independently certified total machine cost. This
boundary is covered by a regression that omits field events while retaining
a valid mathematical root certificate.

`crypto dataset verify` requires a unique JSON root-certificate artifact on
a prime/root/m=2 cell with its public instance and factor base. It rejects
orphan certificates and mismatched targets, moduli, coefficients, budgets,
query order, roots and finite-domain lift records. A capped call cannot become
a completed distinct-root claim. Whole-solve status, per-call root completion
and a verified point decomposition remain separate labels.

This implementation supports future datasets. It supplies no new measurement
of first or last fall degree, no higher-m elimination, no complete binary-root
internal trace and no general performance comparison. Source design and
validation records are in
[the implementation record](../../research/ml_dataset_policies_20261028/DESIGN.md).
