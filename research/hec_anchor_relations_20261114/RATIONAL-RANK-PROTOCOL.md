# Frozen rational-family rank/yield screen

Freeze after algebra gate 092de1672f, before rank-screen implementation/run.
Question: does increased interpolation arity permit a smaller anchor set with
enough independent relations, after common-factor and duplicate rejection?
This is a deterministic count diagnostic, not a performance comparison.
Sources and interpolation proof: RATIONAL-PROTOCOL.md. Algebra-gate result:
9408 signed curve cases, 1108 split primitive principal checks, all passing.

Attempt the existing deterministic pick_instance rule for (g,p)=(2,1009),
(3,503),(4,127), without replacing failures. Require ell^2 greater than the
conservative Hasse upper bound (ceil(sqrt(p))+1)^(2g). This ensures the
ell-primary component is cyclic of order ell, because the selected generator
has order ell. Take the first H canonical factor-base places as anchors,
H in {g+6,2g+8,4g+12}. They contain the source place used to build D1 (first8).
Hence the anchor relation matrix modulo ell has rank at most H-1.

Arms: d=0 is the original M=g+1 polynomial family, computed by fresh Newton
interpolation for each sign class; d=1,2,3 use M=g+2d and signed Walsh output.
No wall comparison is permitted between these implementations in this screen.
At each (g,p,H,d), seed SplitMix64 with 2026111503 xor (g<<40) xor (H<<16).
Draw distinct unsigned anchor tuples uniformly by rejection, sort each tuple,
and retain unique tuples. Visit signs in integer order, fixing the first sign+.
Do not choose tuples/signs based on previous factorization or rank outcome.

Caps: 1024 unique tuples or 32768 visited sign classes per arm, whichever
comes first; if all C(H,M) tuples are exhausted, retain exhaustion. Halt an arm
once anchor rank reaches H-1. Count fully constructed Walsh outputs separately
from consumed outputs, including unused tail of a batch. Record zero,
nonprimitive, degree-drop, nonsplit, >2 off-anchor, empty and duplicate rows.
Require exact division, degree<=g and correct orientations. Verify each unique
admitted raw principal row in the full Jacobian before modular forest insertion.
Merge repeated place coefficients before counting off-anchor nodes. Normalize
modular rows for deduplication. Feed <=2 off-anchor rows into the existing
DenseForest and pure rows into Echelon with zero rhs; target column unused.
Assert rank never exceeds H-1, and retain nonattainment rather than replacing it.

Observables: exact curve c/ell, FB size, H, d/M, tuple count, constructed and
visited outputs, rejection counts, unique admissions, projected rows, rank,
and terminal status rank_reached/tuple_cap/class_cap/exhausted/unavailable.
Output JSONL with one record per attempted arm. No setup, descent or end-to-end
speed claim follows; reaching rank is necessary but not sufficient for 10x.

Local bounded correctness/count screen, no cloud spend, one process and <=60s
per invocation, <=256MiB target memory. If cap prevents completion, retain the
partial/censored result. Any failed principal identity, interpolation/division
mismatch or excessive rank fails the screen. A useful successor should reach
rank with smaller H without an explosion in visited/constructed functions;
otherwise reject its small-anchor cost premise before timing integration.

## Independent rank verification amendment (before verifier implementation)

Keep the original36raw records unchanged. Re-execute the deterministic count
screen, retaining every unique admitted modular row. Independently compute
ordinary dense Gaussian ranks of the full relation matrix R and its off-anchor
restriction R_off. The dimension of row(R) intersect the anchor coordinate
space is rank(R)-rank(R_off), by the rank-nullity theorem applied to coordinate
projection restricted to row(R). Require this difference to equal the forest
and anchor-echelons' rank in every arm. Verify every original JSON field is
unchanged, excluding the two new independent-rank fields. Preserve original
stdout and publish verifier stdout separately. This is a second correctness
gate; it does not select arms or revise cap/stop criteria.
