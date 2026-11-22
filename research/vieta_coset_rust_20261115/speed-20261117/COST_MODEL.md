# What the speed comparison measures

For each fixed field, curve, representation, size and physical target, every
arm starts in a new process and constructs the same certified descriptor and
complete curve-point factor base. Lookup also constructs a hash index. The
construction backend is fixed Karatsuba/Newton for every arm; only the query
backend differs. This isolates the requested GCD change without giving any
arm free target-independent work.

Let C_A be an arm's recorded construction cost and Q_A its query cost. The
reported cold cost is C_A+Q_A in primitive counters, and the recorded enclosing
cold clock in elapsed measurements. The elapsed enclosing clock also includes
small intervening object/context operations; it is not reconstructed by adding
rounded phase times. Query-only ratios use Q_A. Ratios compare matched targets
and are computed by the frozen native analyzer before presentation.

One multiplication counter is the implementation's primitive field operation,
including multiplication work inside inversions and square roots; additions,
polynomial divisions/products, rational-map steps, certified fibers, GCD nodes,
lookups and group additions remain separately visible in each raw row. It is
not a priced machine instruction or a time proxy. Hashing/allocation/sorting
and memory traffic have no field-operation price. The same counted binary is
used for every arm, so diagnostic clocks include instrumentation overhead.

The schoolbook query is the reference. Newton denotes fast multiplication and
long division with classical Euclid, retained to distinguish a multiplication
change from a GCD change. Half uses recursive polynomial transformation matrices
with schoolbook Euclid base cases below the frozen threshold. A successful
query with zero Newton divisions is not a broken Newton implementation: short
quotients trigger its classical fallback. Half-GCD batches those Euclidean
steps even when Newton division itself is unused.

The lookup comparator scans each base point and uses the hash index to test its
complement. Scalar enumeration outside all algorithm clocks supplies a complete
ordered relation-set oracle; this is stronger than checking only that emitted
relations are valid. Every successful arm must match this complete set. The
post-call replay repeats verification with the wide-field scalar implementation.

A construction reused for k queries would have an amortized accounting
C_A/k+Q_A. That formula is an unmeasured scenario, not a result of this one-query
cold protocol. No timing from this experiment includes logarithm setup, relation
collection, linear algebra, scalar recovery, auxiliary-isogeny search or a
complete DLP solve. None of those costs can be silently inferred from a local
polynomial GCD gain.

Rational fixtures use homogeneous numerator/denominator pairs evaluated in the
quadratic Vieta algebra. This avoids interpolation and divisions in that
algebra. Initial map coprimality, exact degrees, every backward sibling fiber,
distinct splitting and finite nonpoles are certified before query evaluation.
The direct finite-target path composes maps without enumerating certified leaves;
the identity target has its own explicit base-enumeration branch. Empty towers
mean singleton membership, rather than unrestricted coordinates.

Raw samples and the primary analysis stay separate by target population,
finite/identity branch, representation, field and size. Calibration has distinct
seeds and is never pooled with the primary panel. Observed elapsed ratios are
below-L2 diagnostics; the final timing-control audit determines whether they
are interpretable even at this local scope. Counts alone are not speedups.

The common full-point-base setup is an explicit protocol choice. A future
candidate that omits unused point lifts is a different construction policy and
needs a separate frozen comparison; it is not silently substituted after seeing
these measurements. Likewise, the schoolbook/half comparison changes the whole
polynomial backend, while Euclid/Newton versus half isolates GCD batching with
the same fast multiplication and division machinery.

Wrapper maximum RSS is the aggregate child-tree peak for a panel, not a matched
per-arm allocation or live-storage price. No memory-saving ratio is inferred.
The retained integer nanoseconds are native monotonic observations; seconds in
the unified export are a unit conversion only. Censored rows never receive an
imputed completion cost, nor are failed counterparts dropped from the flow.
