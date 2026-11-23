# Shared norm algebraic oracle: bounded software qualification

Frozen before implementation and execution of the new component, 2026-11-15.
The owner requests a complete end-to-end alternative to the tensor route.
Completion of this bounded qualification does not discharge the growing-family
subexponential cost/yield requirement.

Question: can the existing shared norm/remainder equations replace the explicit
p(p-1) coefficient census in the cold mixed-target IC path, with independently
checked rational point extraction? The existing F17 m3,t4 system has quotient
size 28 > 17, so one F17-valued separating linear form cannot cover all points.
The current one-linear-form extractor in PR #4091 reports unknown on collided
kernels. Use successive coordinate multiplication operators on the dual
quotient algebra, exact characteristic polynomials and full finite-field roots,
then simultaneous eigenspaces. No projected Krylov sequence is a completeness
certificate. This is standard finite-algebra machinery, not a novelty claim.

Sources checked: PKM 2016 sections 3.1–3.4 (primary PDF); the one-shot norm
continuation and its initial-elimination/selective-oracle notes; the retained
unbound CAS receipt; PR #4091's native extraction code. KB unavailable (no
MCP tools or CLI backend). Debian msolve 0.7.5-1 and FLINT 3.1.3-2 are the
existing pinned backend; downloaded packages must match packages.json hashes.
No backend source, existing gate, frozen evidence or historical output changes.

Before code: the changed component is rational extraction and the adapter from
public curve/target/base to an unknown polynomial solve. The existing native
point arithmetic, canonical polynomial parser and rank-free recovery are reused.
Missing prerequisites: native backend installation, exact staircase/reduction,
coordinate eigenspace splitting and independent model replay.

Qualification fixtures and acceptance:

1. A rectangular F5 grid of 25 points (> characteristic), a nonreduced one-point
   quotient, an extension-only quotient, and a unit ideal. Extract the exact
   rational point sets. Reject caps and malformed/noncommuting presentations;
   never call an incomplete or failed solve UNSAT.
2. The retained unbound F17,m3,t4 system/GB: extract all 28 complete rational
   assignments, evaluate every original equation, recover its curve points,
   and compare complete models with the independent native point oracle.
3. Generate fresh m3 remainder systems for the existing 90 affine-target/domain
   controls (p=17, a=b=2; (t,c)=(1,1),(2,1),(3,1),(3,16),(4,1)). Compare exact
   complete rational model sets, not only their counts. No witness assignments,
   factor-base logs, target scalars or point-sum tables enter the candidate.
4. Cold mixed-target IC on the fixed order-19 fixture: base x^8=1, P=(5,1),
   Q supplied as coordinates only. Draw a,b, ask the algebraic oracle about R
   alone, independently re-add every returned decomposition, collect B+1 rows,
   compute a left dependency from C only, reject zero denominators and verify
   recovered scalars. Harness verifies all 18 nonzero targets outside the solver.
5. Preserve all unsuccessful attempts, unit-ideal/unknown statuses, backend and
   extraction invocations, failures and caps. A default F17 linear-form
   parametrization failure is not nonexistence. No elapsed-time comparison or
   claimed exponent is collected in these software tests.

Budgets: one thread per backend call, 20s wall/2GiB address-space cap per fixed
fixture; at most 90 complete-model controls and 18 cold IC controls; staircase
cap 128, polynomial-reduction cap and per-point verification. Stop qualification
on the first disagreement and preserve the system/output. Larger arity or a
scientific growth panel requires a separate frozen protocol and the existing
EKS/checkpoint/admission gates. No scientific benchmark/cloud job is launched
by this software-qualification protocol.

The oracle returns independently verified relations or unknown. A returned GB
is a backend computation, not an independent source-ideal provenance proof.
Completeness relative to the input ideal requires that proof or the independent
complete-set reference on these bounded fixtures. No general good-target yield,
solving-degree bound or subexponential theorem is assumed from these controls.

## Amendment A1 — growing-arity interface qualification (03:38 UTC)

After the frozen m3 controls and cold path have passed, test the generic norm
builder at exactly three additional public fixtures: (m,t,c)=(5,3,16),
(5,4,1),(7,4,1), all on the same F17 curve and R=(0,6). No scalar, norm
coefficients or decomposition witness is supplied to the backend. This is a
bounded arity/interface control, not a scaling campaign or distribution claim.
Compare any completed extracted point sets to the exhaustive signed-point
reference. Keep the 20s/2GiB backend limits, staircase cap128 and reduction
cap unchanged. A backend error, resource cap, nonzero-dimensional output or
staircase cap is unknown and preserved, never certified nonexistence. Limit
this amendment to those three calls. Do not expand caps adaptively.

## Amendment A2 — remove the cold setup's field-point census

The first cold fixture still used exhaustive public curve-point enumeration
for order admission and factor-base construction. That is acceptable as a
reference but cannot be part of a prospective efficient candidate. Replace
candidate setup by factoring X^8-1 and the quadratic lift equations with FLINT;
admit the fixed whole order19 from a nonidentity order19 point and Hasse's
interval [10,26] for p=17 (19 is its unique multiple of19). Do not compute base
logs. The independent OpenSSL reference remains exhaustive at this fixed size.

After this meaningful setup change, repeat exactly the 18 frozen point-only
cold controls once in a new write-once cold-002 directory, replay every query,
relation, kernel and final scalar, and run negative replay controls. Preserve
cold-001 as the earlier software version. No repeated arity calls, cap change,
scientific timing comparison or inferred speedup is authorized by A2.

## Amendment A3 — source/binary custody confirmation

The first 90-case model run preceded cold trace instrumentation; its exact
binary was not retained before that subsequent rebuild. Preserve that run as
historical development evidence, without fabricating an execution digest.
Repeat exactly those 90 model controls once as models-002 with the final
source/binary hashes captured before launch. The final cold-002 run used that
same unchanged final binary and source; retain both its exact input/output
records and the observed binary digest. Do not repeat the three arity calls.
This is a custody correction, not a new performance experiment.

## Amendment A4 — witness-pruned encoding controls (04:10 UTC)

Before new backend calls, test the separately selected witness-pruned encoding
from chart-pruning.md. Keep the original generator/default explicit-chart
interface and its historical records. With odd characteristic, c!=0 and target
separation retained, omit only the (U,C) and (U,U') Bezout blocks. Reject c=0
in this mode. Direct relation checks remain unchanged. The written projection
argument is not a formal proof or a bound on elimination cost.

Exactly90 m3 model controls, the same3 arity controls from A1, and exactly18
cold point-only controls may run once with that mode after strict compilation
and hashes captured before calls. Preserve full inputs, outputs and statuses
in fresh pruned-001 directories, use the same20s/2GiB caps and quotient128.
The independent OpenSSL replayer must match complete m3 models and cold traces;
completed arity models must match the bounded reference. Stop on disagreement,
keep caps as unknown, and do not adaptively expand the panel. Also require an
input rejection check for c=0. These are encoding/correctness controls; no
scientific timing, speedup, fitted exponent or general degree claim is admitted.

A4 timestamp correction (04:07 UTC, before its backend calls): the preceding
04:10 header was entered ahead of the clock in error. The protocol freeze is
commit ee82e32f1525472579489d19b14a435ce03f0e1d at 04:05:13 UTC. This exact
commit, rather than that mistaken header, determines prospective ordering.
No calls have run yet. The implementation variable-count expression is
(7m-3)/2; the earlier prose numerator typo does not affect its9/16/23 counts.
