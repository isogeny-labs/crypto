# Next lead: coordinate-box vacuity and correlated finite-domain images

UNEXECUTED bounded next experiment. This fresh note does not alter the frozen
extension-coordinate worker, source pins, proof, inputs, or evidence. There is
no novelty, hard solve, elliptic-chart coverage, or comparative speed claim.

## A falsifiable obstruction to the naive extension route

Let p be prime. Suppose an integer linear certificate is
J=c+sum_j v_j z_j, with every coordinate independently relaxed to the FULL
canonical interval 0<=z_j<=p-1. The exact interval enclosure has integer endpoints
L=c+sum_j min(0,v_j(p-1)), R=c+sum_j max(0,v_j(p-1)). Consequently
R-L=(p-1)sum_j |v_j|. If any v_j is nonzero, this interval contains at least p
consecutive integers. Every set of p consecutive integers contains a multiple
of p. Therefore the interval exclusion criterion ceil(L/p)>floor(R/p) NEVER
holds. If all certificate coefficients vanish, a valid homogeneous/affine
dual identity with zero functionals also has c=0 modulo p, so it cannot exclude.

This applies to the unconditioned seven-block coordinate relaxation in which
all feature AND output coordinates are independent full canonical boxes. It
does not apply when target/output coordinates are fixed, coordinates have
smaller ranges, an affine domain is restricted, or correlations are retained.
It also does not say the exact sparse image of a linear form meets pZ: its
INTERVAL HULL does. Confusing these objects would discard useful structure.
Replacing LLL with a faster reducer cannot repair this representation failure.

The universal calculation is a mathematical counterexample to assuming useful
pruning follows merely from the correct p^(5r) dual-lattice representation.
It is not an impossibility result for extension-field root solving.

## Changed bounded experiment: measure information lost by each relaxation

Use the same explicit GF9 and GF25 polynomial bases and D={0,1,theta}. This is
a toy quadratic feature system, not an elliptic curve chart. For (x,y) in D²
define the FIVE correlated field features F=(x²,xy,y²,x,y). Freeze coefficient
lists a_i=encode(2i+1 mod q), b_i=encode(3i+2 mod q), i=0..4, with encoding
meaning polynomial-coordinate labels as in the preceding worker. Define
t=sum a_i F_i and u=sum b_i F_i. Exhaustively preregister all q² target pairs
(t0,u0) rather than selecting targets after seeing rejection counts.

The direct oracle enumerates the nine original ordered pairs (x,y), computes
both field equations independently, and emits EVERY matching pair for every
target, including empty and repeated-coordinate cases. Distinguish the actual
feature tuple image (nine source pairs, with collisions retained), its separate
coordinate marginal boxes, and its full canonical-coordinate box relaxation.

For eight fixed functional pairs in Fp^4 (the four coordinate unit vectors and
four sums e0+e1,e1+e2,e2+e3,e3+e0), construct the correct transposed dual feature
coefficients lambda_i=M(a_i)^T gamma+M(b_i)^T delta. For each fixed target form
J=sum lambda_i dot coord(F_i)-gamma dot coord(t0)-delta dot coord(u0), with
integer coefficient lifts fixed in advance. Compare these exact rejection tests:

1. Every coordinate including outputs independently spans [0,p-1]. Verify the
   lemma above gives zero interval exclusions, without enumerating that box.
2. Outputs are fixed to (t0,u0), features use their actual marginal coordinate
   min/max ranges over D². Reject only if the resulting integer interval has no
   multiple of p. This is a sound necessary condition for the original pairs.
3. Outputs fixed, take the min/max of J over the nine CORRELATED feature tuples.
   Again reject only if that interval omits pZ. This hull is contained in the
   marginal-box hull; it can lose less information but may still be vacuous.
4. Outputs fixed, retain the exact set of nine values of J, or residues modulo
   p. Reject iff that discrete set contains no zero residue. This retains
   correlations completely for one functional, though several individually
   feasible functionals need not share the same source pair. Intersecting these
   necessary filters is sound but is not an automatic completeness certificate.

Every rejection must be checked against the complete direct ordered root set:
an actual target witness must never be excluded. Every survivor remains UNKNOWN
until the direct oracle or a separate complete algorithm supplies all roots.
Record, for all targets and every fixed functional, rejection counts, residual
target sets, all false-negative checks, and cost of materialization and replay.
Compare whole charged filter-plus-survivor evaluation to direct all-target
evaluation. Nine points are calibration, not hardness or a predicted speedup.

Budget before registration: at most 200000 target/functional/source predicate
checks, 90 seconds, 4GiB, one core, no network. The principal loops use at most
(81+625)*8*9=50832 target-functional-source evaluations, plus independent
arithmetic and output checks. No LLL is needed for this information-loss
diagnostic, so a refutation cannot be concealed by changing reduction budgets.
Freeze source/input hashes and eight functionals before execution. Store all
per-target roots and masks, not only counts or a favorable subset.

## Decision rule

If correlated bounds or sparse images yield additional sound rejections at
acceptable TOTAL cost, derive a compact representation of those correlations
without enumerating a hard domain. If their whole cost exceeds direct solving
or they add no rejection, retire this toy feature/filter choice and change the
domain map or representation. Do not repeat it with a larger timeout. A useful
next algebraic branch would be a proved product-span/Frobenius representation
for the actual input domain, followed by its own independent completeness and
counterexample checks. Odd-extension chart guards and binary/Koblitz charts
remain separate required derivations.
