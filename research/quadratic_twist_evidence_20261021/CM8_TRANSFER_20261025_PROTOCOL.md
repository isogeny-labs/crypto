# Registered F13 degree-two quadratic-twist transfer and useful S3 comparison

2026-10-25. Preregistered before native execution. Existing PR2381 only.
This round follows the atlas transported-relation and presentation-invariance
leads. It does not claim a new general isogeny construction, CM order, speed,
stationary useful yield, intrinsic regularity, or a practical DLP improvement.

## Exact models and construction

All are already registered and in the frozen942-model population:
E/F13:y²=x³+x+4 (`icv1-fp4-t0-6bee67e0`),
H/F13:y²=x³+4x+7 (`icv1-fp4-t0-bb514d71`),
T/F13:y²=x³+4x+6 (`icv1-fp4-t0-097da6b6`), the d2 monic twist of E.
Registry/model inputs are unchanged; unknown generator traits remain unknown.
The proposed translated standard degree-two map is
h(x,y)=(x+2/(x-10), y*(1-2/(x-10)^2)), followed by unit5 scaling
Phi(x,y)=(12*h_x,8*h_y). Set O and the rational kernel point(10,0) to O;
assert no other source denominator is zero. Verify all actual source points,
image equations, full rational addition table, exact kernel and all subgroup
scalar images independently. Check no unit c in F13 satisfies c⁴=4,c⁶=8.
This proves absence of a short-model scaling; a general model-classification
bridge must remain explicit unless formally supplied. Field definition,
geometric degree, finite rational group checks and algebraic group-map theorem
are distinct. New kernel-checked actual-point/subgroup proofs and a source-bound
cleared coordinate identity are attempted; retain any failures and gaps.

## Generator, bases and coverage

Choose P as the lexicographically first affine nonidentity source point killed
by7; U=actual Phi(P), checked nonzero and exact order7. Do not construct Phi by
scalar lookup. For each curve take its three distinct nonzero subgroup x
coordinates; generate all three increasing size-two subsets, and choose the
least y subgroup representative at each x. Six canonical arms plus three E
bases transported through actual Phi and three T bases pulled back by searching
the seven **public base setup** points give12 arms. Transport/pullback preserves
column order/signs; it need not sort mapped coordinates. The inverse base search
is setup only, not recovery or target scalar input.

For each arm, process all six nonzero generator multiples in order1..6.
Build original S3 plus two quadratic membership polynomials, and the existing
size-two reduced S3 remainder. Use unchanged frozen adaptive row solver helper:
graded order(total degree,x exponent), D0..8, at most64 promotion rounds,
expanded witness cap32. Preserve full three-input witnesses, promotions,
Macaulay/evaluation ranks, exact ordered roots and actual signed point lifts.
Check every S3 zero has an actual signed lift on these fixed bases; if false,
retain/report exception instead of silently discarding it. Compute coefficient
row rank modulo7 and incremental useful rank on the fixed1..6 query stream.
Original/reduced roots must agree. First-query and whole-stream ranks differ.

Compare every canonical E/T base pair (nine comparisons,54 target comparisons)
on both original/reduced systems; count joint strictly higher incremental
useful rank and lower **named operational matrix degree**, not intrinsic
regularity or F4/F5 time. Retain all ties/losses and expanded witness peaks.
For each matched/pullback pair check ordered lifts/rows and useful rank under
actual subgroup transport. Compare solver degree; equality is a control to
observe, not a theorem assumed. Existing leading-form theorem applies to these
same size-two systems: cutoff2 on both sides. Do not substitute that statement
for a different homogenization or solver runtime.

## Actual source recovery

Each of12 arms builds base logs from its actual signed relations for public
mG, m1..6 until coefficient rank2. Solve the augmented equations modulo7,
verify both base logs against points; never enumerate a base logarithm as the
solver. For each of six independently constructed public synthetic source
Q=nP, give recovery Q alone, map Q if using a target arm, try shifts0..6,
solve the actual reduced system, retain **every** tested ordered sign in each
query including after the first success, and derive
n=(si*l_i+sj*l_j-shift) mod7 from the first accepted lift. Record failed shifts,
identity shifts, actual query points/solver certificates and verify nP=Q.
All72 separate recoveries, setup/online query counts and map calls are retained.
Labels generate/check fixtures outside recovery. Full-cost timing/calibration,
rho/external-fastest and benchmark promotion remain outside this math round.

## Independence and fail-closed controls

C++ may reuse frozen full.cpp and adaptive helpers; Rust uses its existing
separate coefficient-pair/point/polynomial arithmetic and independent row
replay, plus exact witness checks. No shared C++ arithmetic. Canonical outputs
are source-bound; changing a payload and rehashing its manifest must still fail
without a receipt for: false map point/kernel, false subgroup source binding,
missing arm/target/failed query, false solver degree/witness/useful row, false
base log/scalar and stale source. Null controls must be explicit if a required
kind of failed query does not exist. Valid on-curve wrong points are used;
invalid-curve attacks remain out of scope.

## Resource and publication gates

Compile and commit native source before the first execution; preserve failed
attempts and compile logs. Native command cap600s, focused iteration3600s,
existing local host only/two build workers, no paid compute or upload/credential
reuse. Source-bound polynomial certificate32+16 checks and corruption controls,
faithful pinned Lean attempts/unmodified proof gate/axiom audit, fresh root
release library suite, affected backend/workflow tests, exact replay,
report/graph/two-page inspected PDF, archive/size/artifact/naming/card checks.
New versioned sources/projects; all old frozen data/proofs/certificates and
registry/catalog/benchmark/conflicted files unchanged. Keep PR open/noauto-merge.
Actual Conductor check exits127 before editing; no reservation/replacement.
