#set page(margin: 2cm)
#set text(size: 10pt)
= R6.8 Transport-oracle census: corrected status

*2026-10-20 review: the proposed general simulation is not established.*
The previous proof and closure claim are withdrawn. The original PDF and
source are preserved in `historical-04c3e817/` with SHA-256 hashes.
The recorded census values and plots are unchanged.

== Counterexample to Lemma 5
In an order-5 cyclic subgroup with generator $P$, take the identity map,
$S = {O, P}$ and exactly two summands. The original sums are $O$, $P$ and
$2P$. Removing $O$ from the image base leaves ${P}$ and only the sum $2P$.
The decomposition $P = O + P$ disappears. Thus the claimed sumset inclusion,
count monotonicity and size lower bound do not hold as written.

The executable `review_counterexample.py` checks this enumeration and a
separate kernel-fiber example. Keeping the identity repairs this example,
but does not establish the theorem. A corrected proof must also account for
all lifts, challenge-adaptive transport paths and their complete costs.

== What the census establishes
Eight raw census files cover four curves, two base sizes and two arities:
16 cells. The programmed falsifier fires in 0 of 16 cells. Union/best
class-factor-per-column ratios are 0.02-0.27. These are finite measurements;
they do not establish the general simulation, its overhead, or a constant-only
asymptotic effect. The derived cost uses the recorded model, not end-to-end
wall time. No ledger row is promoted.

#figure(image("class_factor_per_unknown.svg", width: 100%), caption: [Unchanged recorded class factors. Finite census only; no general proof follows.])
#pagebreak()
= Recorded cost comparison
#figure(image("cost_ratio.svg", width: 100%), caption: [Unchanged derived model cost relative to the plain base.])

== Limits and remaining obligations
Four curves, one seed and two base sizes; some three-term sumsets approach
subgroup saturation. The class-factor model is heuristic. The unrestricted
6r image counter is not a subgroup-restricted identity certificate.

A corrected theorem must define identity and kernel handling, fixed versus
at-most-m decompositions, lift enumeration, adaptive path dependence, and
complete cost/scaling bounds. Full details and unchanged tables are in
`RESULTS.md`. The historical report is retained as superseded evidence.
