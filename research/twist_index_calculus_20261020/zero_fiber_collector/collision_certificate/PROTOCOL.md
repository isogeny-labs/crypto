# Repeated-factor certificates for zero-fiber relation search

Prospective protocol, 2026-10-25. This is an algorithmic follow-up inside the
existing synthetic twist/index-calculus study. The live Phase 2 producers,
their 85 configurations, deadlines and source seals are immutable. No old
F4 solve, point census, target-recovery run or timing round is repeated.

## Question and prior work

For the peeled ordinary abscissa `a`, write
`q_a = a^3 - a^2 e1 + a e2 - e3`. A decoded triple with elementary symmetric
coordinates `(e1,e2,e3)` has `q_a = (a-x1)(a-x2)(a-x3)`. If an exact
certificate `q_a^k = sum(h_i f_i)` holds for the four restricted S4
polynomials, every solution repeats `a`; none can pass the existing
distinct-factor decoder. This is a sufficient rejection condition. Failure
to find a certificate within a degree cap establishes no obstruction.

Hypothesis: certificates are cheap enough and frequent enough to reduce
relation-search work, with a possible difference between the existing phi
and doubling arms. This is untested. Doubling receives exactly the same
certificate degrees, powers and accounting. Signed orbit copies are not
new independent relations.

The atlas match connects this to `LEAD-ICP-xline-symmetry-special-j` and
polynomial-solving work, including `KR-IC-955fd6`. Pinned atlas main
`ab0609ebee3d17527a51667591bda56f7cdece31` has the same lead/frontier files
as the earlier `1ba8e8ca6f94da82dd8563797dd43429127eb143` match checkout.
The cross-repository delta remains partially reviewed; no novelty or atlas
saturation claim is made. This does not reopen the prime-field bounded-degree
factor-base theorem: the current systems are over the existing quartic
extension and the factor base is unchanged.

Symmetrizing the relation equations and exploiting torsion are established
approaches: Faugère, Huot, Joux, Renault and Vitse, EUROCRYPT 2014,
[publisher abstract](https://link.springer.com/chapter/10.1007/978-3-642-55220-5_3).
The abstract was read; the IACR full-text URL refused access. No claim about
the full paper's treatment of saturation follows from the abstract.

## Fixed workload and separation

Reuse all seven primes, all three presentations and both existing zero
arms: **42 case-arms**, every registered canonical pair, in their stored
order. The existing full Phase 1 records carry the actual four polynomial
inputs and complete root/physical references. Freeze their hashes and
independent-audit hashes before execution. Also bind the unchanged public
input and its ordinary-factor indexing; the zero record alone does not
provide the peeled abscissa values.

Native preparation extracts a separate input containing only `p`, public
abscissa/index information, pair context and the four exact polynomials.
It excludes root lists, accepted relations, original F4 costs, factor logs
and target scalars. The producer receives only this pack and its hash.
The post-run auditor reads the sealed complete references. All packing,
search, verification, reduction, result generation and replay are Rust.

Start with compiled synthetic positive/negative/mutation controls. The
first two production case-arms are p37 seed 0, phi and doubling; require
their independent audit and deterministic A/A checks before proceeding.
Reuse those exact registered executions in the full comparison, rather
than run another pilot. Each of all 42 case-arms has first/A/A (84 producer
executions), an independent first-run audit and exact deterministic parity.
Freeze source, binary and 42-case workload before the first production
certificate search. Preserve failed gates and all partial records.

## Search and stop rule

Use ordinary total-degree Macaulay spaces at D=4,5,6,7,8, without field
equations, saturation variables, extra root-derived equations or hidden
oracle information. At each D include every monomial multiple of each
original nonzero polynomial with total degree at most D. Build a deterministic
echelon basis with an explicit representation in those rows. Try powers
`k=1,2,3` in that order. Stop at the first exactly checked certificate;
otherwise retain `fallback-needed` after D=8. There is no fallback F4
execution in this diagnostic: the original records are only post-run
comparisons. A valid relation can never be discarded by an accepted
certificate; any contrary reference is an audit failure and stops promotion.

Retain every attempted D, rank, row/column size, arithmetic counters and
certificate. A separate sparse polynomial expansion must verify each
certificate identity, independently of the dense elimination. Charge both
construction and this check. Reject a changed input binding, noncanonical
coefficients, excess degree/power, changed certificate and missing/duplicate
system. Test a case requiring a power greater than one and a case with a
genuine non-repeated root where rejection is forbidden.

## Accounting and interpretation

One worker, shared busy reservation, 8 GiB per case, 1,200-second outer
watchdog and 1,180-second cooperative deadline. Compile separately with one
worker and no address-space cap. Reserve separately per case so the existing
target-recovery panel can continue. Never wrap a per-case driver in an
outer reservation. Journals precede and follow each attempted system; a
timeout preserves the completed prefix and pending system, never a new
deadline for that attempt.

Count modular multiplications, additions and inversions for failed and
successful searches and sparse certificate checks, plus memory and raw
shared-host elapsed diagnostics. Compare the multiplication count with
the sealed F4 row-reduction count only as a partial stage diagnostic.
Any projected replacement count is labelled a calculation on retained
records, not a new fallback execution or complete solver price. No admitted
wall-time, full-pipeline, scaling, rho or global-best claim follows.
Inversion calls are reported separately, but their exponentiation products
are already included in multiplication counts. Process RSS is recorded by
the outer per-case `time -v` receipt; it is not a native live-allocation counter.

The independent post-run audit checks the exact polynomial identity and
all recorded field roots for a certified system, and requires that its
sealed accepted physical set is empty. Uncertified systems remain unchanged.
This establishes a safe static removal relative to the retained physical
reference; actual integrated target recovery/rank/cold price remains a
separate experiment after the live producer is reconciled.

Include a readable proof and a faithful Lean attempt for the rejection
lemma, with the actual hypotheses and remaining native/field bridges
explicit. Preserve every unsuccessful search, report all 42 case-arms,
and deliver native analysis, a diagram/graph and inspected PDF. Publish a
new versioned raw archive under existing store rules; never overwrite the
completed archives. All original stronger baseline and complete-price
requirements remain open.
