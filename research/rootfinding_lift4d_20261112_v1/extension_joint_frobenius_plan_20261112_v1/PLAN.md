# Joint functional consistency and a compact Frobenius branch

UNEXECUTED changed hypothesis. No measured source, input, proof, closure or raw record is edited. These elementary coordinate/Frobenius identities are not a novelty claim and do not establish an actual curve chart or speed ratio.

## Exact counterexample to independent functional consistency

Take GF9=F3[theta]/(theta²+1), but now explicitly change the source domain to the prime subfield D={0,1,2}. Map each ordered pair (x,y) to t=x+theta*y and u=x²+theta*y². Its four output coordinates are (x,y,x²,y²). The target (t,u)=(1+theta,0) has coordinate vector (1,1,0,0) and has no source: the first two coordinates force x=y=1, making the last two coordinates (1,1).

Nevertheless every one of the eight previously frozen scalar functionals accepts this target independently. Explicit witnesses for (e0,e1,e2,e3,e0+e1,e1+e2,e2+e3,e3+e0), in order, are source pairs (1,0),(0,1),(0,0),(0,0),(1,1),(0,1),(0,0),(1,0). Each witness matches that functional's target value. The witness may change between functionals; there is no single common source. Thus intersecting eight separately feasible scalar images is not a completeness certificate. The joint projection (x,x²) alone rejects target (1,0).

This does not assert that the currently measured D={0,1,theta} feature map has this particular false positive. It supplies a separate explicit counterexample and a bounded new control. Retain both domains and all outputs separately.

## A complete compact image criterion for this special map

For any odd p and quadratic K=Fp(theta) with theta²=d nonsquare, Frobenius sends theta to -theta. Every t in K has a unique representation t=x+theta*y, x,y in Fp, recovered by

    x=(t+t^p)/2,   y=(t-t^p)/(2theta).

Frobenius fixes x and y: the first expression is half a trace, and both numerator and denominator of the second change sign. Direct substitution proves existence and uniqueness. Hence the above map has an ordered source for target (t,u) iff

    u = ((t+t^p)/2)^2 + theta*((t-t^p)/(2theta))^2.

If the identity holds, emit that unique ordered pair; otherwise emit the empty set. This remains exact when x=y or either is zero. For a proper declared subset D of Fp, additionally test both recovered elements for membership; do not substitute a field encoding for membership. The first-coordinate map is invertible over Fp, a strong premise not established for the actual curve problem. The inverse fails in characteristic two because division by2 is invalid; use an independently proved binary linear map instead.

The criterion uses a constant number of field operations and one Frobenius evaluation, rather than a list of all p² image points. For a generic polynomial-basis backend, exponentiation supplies Frobenius in O(log p) field multiplications; precomputed linear Frobenius may be cheaper but its setup must be charged. This is a known structural inverse technique, not a universal root-solving advance or an 8x prediction.

## Bounded next experiment

New source/experiment IDs only, 90 seconds, 4GiB per process, one core. Correctness controls cover p=3,5,7,11,13 with (p,d)=(3,2),(5,3),(7,3),(11,2),(13,2); independently check d is nonsquare before constructing theta²-d. All q² targets total46,309 across these five quadratic fields. Independently enumerate every ordered source pair from the explicitly changed domain Fp, index its outputs using separate arithmetic, retain complete target rows and compare every derived inverse result. Check the eight-functional counterexample plus joint projection separately. The field construction and nonsquare proof are part of each declared input.

Then compare a directly derived Frobenius inverse arm with a strong indexed-image baseline on GF9/GF25/GF49 under three fixed alternating repetitions, setup, exact root verification, serialization and memory charged. These three fields have3,107 target pairs total, so full root verification for six arms adds18,642 target checks. A separately frozen200,000-predicate budget leaves room for arithmetic, domain and output controls; count the complete implementation before registration. Larger prime domains require a fresh bounded field definition and registration; their cardinality alone is no hardness evidence. A negative timing is evidence to change representation, not to repeat with more time.

If correct and useful, the next non-toy derivation is to locate an actual guarded chart whose source-linear output map is invertible on the prescribed finite-domain product. Prove that map's rank, domain size, poles, exceptional fibers and original auxiliary-root coverage before using it. A singular or noninjective map requires complete kernel-fiber enumeration or an explicit UNKNOWN result. A tiny bijection must not be presented as coverage of binary/Koblitz curves, the GF(2^83) hard target or arbitrary index-calculus systems.
