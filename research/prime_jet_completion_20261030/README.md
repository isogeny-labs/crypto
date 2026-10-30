# Nonscalar origin channels and rank-directed jet completion

Research proposals, 2026-10-30. No new ECDLP algorithm or asymptotic
improvement is established. Priority is unverified. The design combines
standard Hermite interpolation and low-rank completion, applied to the
specific obstruction in the preceding common-kernel study.

## Sources checked first

The pinned Oct25 manuscript, D.4.3–D.4.6, supplies the actual Cauchy matrix,
local charts, mode expansion and precision costs; exact provenance is in
[the previous bibliography](../prime_common_kernel_20261030/sources.md).
The [coded-selector analysis](../prime_common_kernel_20261030/reviews/coded-kernel.md)
and [packing result](../prime_common_kernel_20261030/reviews/exact-filter-packing.md)
show why neither few scalar operator coefficients nor exact pair filtering
automatically gives cheap Boolean modes. This is a narrow follow-up, not
an exhaustive new literature or six-repository novelty search.
After the initial finite control, an additional metadata-only atlas query
was recorded in `atlas-match.log`. Its prime-to-p/canonical-lift jet lead
concerns lifts of curve points and scalar digits. The present jets concern
an auxiliary tensor interpolation variable, so that lead's obstruction is
not treated as a rejection of this proposal. No historical timing run is
reclassified here.

## Idea 1: replace the scalar origin channel

The earlier family contains f_star(T)^2 I. Near an odd node its first
surviving crossing block is a unit times the identity after normalization.
Making the other matched operators singular does not address that term.

Proposed change: use a matrix R(T) with R(0)=I and R(t)=0 at every active
nonzero node, but require low *weighted crossing rank* of its jets. It must
be nonscalar off the nodes. Combine it with the local node maps through one
fixed polynomial W(T), rather than choosing a different polynomial by level.
This leaves the exact paired-operator template; its packing theorem is no
longer a complete analysis. That is an opportunity to investigate, not a
proof of improvement.

The target is an explicit formula and integral factorization certificate for
R's crossing jets. Ordinary rational rank is insufficient: every denominator,
2-adic Smith order and surviving coefficient at the actual precision matters.

## Idea 2: optimize free jets instead of zeroing every derivative

Follow-up: an [analytic degree-zero completion H](../prime_pole_anchor_20261030/README.md)
now preserves the same nodes and gains one odd-level crossing bit with only
one extra matrix degree in exact development checks. This advances the
proposal without running the broader bounded search; its global integration
and asymptotic improvement remain unproved.

Let C(T) be the source's full nodal polynomial. Search in

    W_new(T) = W_base(T) + C(T) H(T).

Every node value, including the origin identity, is preserved by construction.
For an initial bounded problem restrict deg H <= 2L-1, so the degree remains
O(L), but with a larger charged constant. The first derivative at a node t
changes by C'(t) H(t). Local derivative blocks are free variables; crossing
blocks should be chosen jointly to have few low-valuation factors.

Unlike scalar smoothing, this allows cancellation between the origin
channel and the remaining matrix entries. Unlike repacking, it specifies one
common polynomial. Neither fact guarantees integral charts or cheap modes.

The finite search specification is: choose rational coefficient matrices H,
bounded degree and denominator budget; at every retained level require an
explicit integral U,V factorization of the crossing jet modulo its stipulated
precision. Price degree and precision before comparing mode counts. Return
the polynomial and factorization witnesses, not just vanishing minors.
Uniform generation of these objects must eventually be charged as well;
an expensive small-instance search is not a DLP precomputation algorithm.

## Exact control: ordinary jet flattening

For the cardinal polynomial ell_delta and d_delta=ell_delta'(delta), test

    H_delta(T)=ell_delta(T)^2 (1-2d_delta(T-delta)),
    W_delta(T)=rho_delta H_delta(T).

This is a control for the proposed search, not an asserted novel method.
The [prospective protocol](native/PROTOCOL.txt) fixes four sizes and exact
rational computations. All5440 nodal value/first-derivative checks pass.
At odd node1, equal-low-bit crossing matrices give:

| L | Original first coefficient valuation | Flattened first coefficient valuation | Both normalized residue ranks |
| --- | --- | --- | --- |
| 4 | 3 | 5 | 2 |
| 8 | 4 | 6 | 2 |
| 16 | 5 | 7 | 2 |
| 32 | 6 | 8 | 2 |

The original coefficient is linear; the flattened one is quadratic in the
scaled chart. These are different coefficients, explicitly identified.
Flattening roughly doubles polynomial degree but gains only two valuation
bits in these cases. No uniform theorem, speed comparison or runtime
improvement follows. Higher jet coefficients and full chart integrality
have not been checked by this unit. CPU contention cannot change its exact
rational equalities.

## Formal and execution scope

The pinned Lean project attempts the general scalar own-node value and
first-derivative identities using actual polynomial differentiation.
Those two identities alone do not formalize all nodes, the rank table,
integrality, precision or either proposed matrix construction. Build/audit
receipts record the successful clean build, two-declaration transitive axiom
audit/shared gate, and completed official fresh replay. Only standard
foundations are used; no admitted proof or native trust is present.
The exact proof map is in [lean/PROOF_MAP.md](lean/PROOF_MAP.md). The original
current Rust validation is unchanged: this branch changes no `src`, `crates`
or Cargo inputs relative to the source of the preceding research package.
Its429 affected passing Rust tests remain the relevant recorded check; no
new full-suite result is claimed.

Run `bash native/check.sh /tmp/jet-completion-check` for the finite control.
No performance experiment or ECDLP instance is run. The two design changes
remain proposals: the next discriminating result is a certificate that beats
the original kernel's *degree-adjusted and precision-adjusted* crossing-mode
cost at growing sizes. A small successful rank completion would still need
a uniform construction and full recursive proof.

For a first unrun completion search, use L=4,8 and explicit precisions8,12,16.
Require entry-by-entry reconstruction of every proposed integral factor at
every retained node. Controls must include H=0 (the original kernel), the
flattening control above, and rejection of an invalid even-denominator
inverse. Freeze the coefficient/denominator budget before searching. These
are proposed next inputs, not reported experiments or scheduled jobs.
