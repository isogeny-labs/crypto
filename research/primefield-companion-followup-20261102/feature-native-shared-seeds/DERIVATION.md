# Shared seeds and reusable source preparation

PreparedSource borrows immutable Source state and retains prepared projective and
affine strides. Calibration supplies c(P) once. Setup stores c(P)^2 and anchors
(O,0), (P,c(P)), (-P,-c(P)), (2P,c(P)^4*2y(P)). The 2P point uses the usual affine
doubling formula in setup. Source::window retains the earlier direct-phase path;
Source::prepare()?.window(...) selects the shared path. The seeded raw entry is
private, so external callers cannot inject unvalidated seeds or a foreign grid.

The source was read directly from mixed-projective-strides-v2/run.py, whose hash
is recorded. At offset i, an anchor is handled first. Otherwise offsets 0 and 1
use direct phase. Later offsets use
c(T+P) = (x(P)-x(T))*c(P)^2*c(T)^2/c(T-P).
Each step uses four field products, one subtraction and one field inversion,
with c(P)^2 already stored. These are derived arithmetic counts, excluding the
inverse algorithm's internal arithmetic. After O, the P and 2P anchors prevent a
zero predecessor from reaching the reciprocal. Nonzero recurrence output is
checked. At most two direct initial phases are needed per nonempty window.

## Checked evidence

One native build and one filtered invocation passed three methods. Five fixed
length-16 windows on F509/r479 start at O,P,2P,-P,3P. All 80 signed outputs match
checker-only Euler symbols. All 78 finite seeds match freshly computed direct
phases; identity seeds are zero. The first four windows also reproduce the 64
saved public shifts. counts.json projects the five method-counter records from
test.log; its generation does not execute an algorithm.

Recorded counts (direct, recurrence, anchor) are respectively:
O (0,13,3), P (0,14,2), 2P (1,14,1), -P (0,12,4), 3P (2,14,0).
The reference direct-phase path would invoke phase once for each of the 78 finite
points. The shared path records 3 direct phases and 67 recurrence updates; ten
anchor hits include two identities. The 67 updates entail 268 field products
and 67 inversions by the formula. This is a component tradeoff, not complete-cost
or elapsed-time evidence. Test-only reference phase calls are outside the records.

On each original curve, a length-five O-start window is evaluated twice from the
same prepared state and returns identical values, seeds, methods and counts.
Saved base phases and raw final ratios remain exact. Empty/invalid inputs are
checked. F7: y²=x³+3, P=(1,2), r=13 exercises epsilon=0; F5: y²=x³+2x+1,
P=(0,1), r=7 exercises s=0. Both length-five windows agree with expected signs
and direct finite phases. The branch with both s=0 and epsilon=0 is not separately
exercised. Nineteen earlier/dependency methods were filtered out.

## Domain, costs and next dependency

Inherited prime-field, prime-order, word-size and phase-domain assumptions apply.
Prepared state is borrowed from one immutable source; target grids remain private
and are generated from the public Q. Known point doubling and stride preparation
are setup costs. Per-target geometry conversion, scalar-value division engines,
polynomial allocation, recurrence inversions, raw quotients and final calibration
products remain. The full4563-case source/recovery plan is unchanged and did not run.

Next bind this shared source to the checked native sign-mask decoder, retaining
the repaired coefficient program and checker-only scalar labels. Then instrument
complete setup/query costs. Any homogeneous-seed optimization must propagate a
compatible ratio through scaled division values and the inner recurrence; simply
setting the seed to one does not preserve the normalized half-index boundary.
