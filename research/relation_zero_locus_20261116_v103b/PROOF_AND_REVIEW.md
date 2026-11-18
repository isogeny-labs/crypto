# Exhaustive local equivalence test using truth vectors

Bit x of the truth vector for coordinate j is (x>>j)&1. Small widths use a
direct sum; larger widths repeat the exact little-endian byte patterns AA,
CC,F0, then blocks of zero/one bytes. A monomial truth vector is the bitwise
AND of its coordinate vectors. A relation truth vector is XOR of its monomial
vectors. Intersect their complements inside the finite universe to obtain
exactly the zero set of every relation, exhaustively, not probabilistically.

The allowed table mask has a one precisely at each enumerated coefficient
tuple. Equality of the two masks certifies local equivalence; subset alone
certifies necessity. If masks differ, retain literal factor-division-checked
counterexamples. Never claim a weaker cubic predicate is exact based only on
positive checks. Every frozen relation and feature order comes from the
previous successful profile, registered as an input hash before this run.

Truth storage is preflighted at<=512MiB and the worker stays under4GiB/one
core/90seconds. Each field releases local vectors before the next call; at
width20,1351features occupy about169MiB plus bounded temporary vectors and
Python objects. Setup, oracles, table regeneration and reporting count. Tiny
widths1..8 compare every truth bit with scalar monomial evaluation, followed
by frozen actual coefficient spot checks. These checks supplement the proof;
the whole local cube is nevertheless represented exactly in the bitset.

Scope remains the fourteen necessary residue predicates. Even exact local
cubic replacement does not establish full prefix support, original source/U
recovery, ghost/chart coverage, the actual339-variable solve, novelty or a
qualified speed ratio. Original completion/count fields stay false/null.
