# Exact uniform useful-row yield, solver degree and collection-state experiment

2026-10-25. Preregistered before execution, existing PR2381 only. This tests a
stronger sampling statement than a favorable individual target. The explicit
F13 dual remains queued; it is not executed or silently replaced as evidence.

## Fixed populations and algorithms

Use actual registered F43 E:y²=x³+x,P=(31,18),r11 and its T:y²=x³+4x,
U=(24,26), actual map(41x,11y), with inverse(21x,4y). Use the executed F13
E:y²=x³+x+4,P=(0,2),r7 and T:y²=x³+4x+6,U=(8,11), actual degree-two
Phi from frozen cm8-transfer.cpp, including its kernel. F13 public-base inverse
still enumerates seven public subgroup points. Do not reuse unknown EC1 aliases
or introduce models/registry edits/uploads. Existing Lean source/scalar bridges
remain the actual verified reductions; no generic twist-rational map assumed.

For b2 and b3 enumerate ALL increasing b-subsets of the nonzero subgroup x
coordinates on each curve; use least-y subgroup points. Three roles accompany
each canonical pair: source bases transported to T and canonical T bases pulled
back to E, keeping column order/signs. Thus F43 has10+10canonical bases per
curve and80 total arms; F13 has3+1 per curve and16 total arms.96 arms total.
Each arm queries every nonidentity generator multiple once: F43 ten/F13 six,
896 actual query records. Every query executes original and reduced S3 with
full three-input witnesses and the unchanged named adaptive solver/order/caps.
For b2 reuse checked size-two remainder. For b3 original S3 already has degree
at most2 in each factor variable; division by cubic membership changes nothing,
so reduced equals original explicitly. Preserve complete roots, all b²*4ordered
sign tests, actual point sums, accepted coefficient rows modulo r and terminal
Macaulay/evaluation certificates. Verify every constrained root lifts; retain
any exception. Distinguish homogeneous cutoff, operational matrix degree and
expanded witness peak. No intrinsic regularity/F4/F5 runtime claim.

Build base logs from actual public mG queries until coefficient rank b, solve
augmented equations modulo r, verify each base point. Give recovery Q alone,
map it if needed, try shifts0..r-1, actually solve/query each required system,
retain every sign including after success, recover/verify the source scalar.
All896 separate source targets must remain, including failures. Known labels
construct/check public fixtures only, never provide a solver or map answer.

## Exact sampler, useful yield and state-conditioned collection

Uniformly sample a nonidentity scalar m independently with replacement from
1..r-1 and query mG. This is a finite hypothetical sampling protocol applied
to exhaustively executed responses, not a timed random-search run.
From empty coefficient space report exact fractions for probability of positive
rank increase, expected rank gain per call, and mean original/reduced named
matrix degree. Raw zero count and expected rank gain are different quantities.

Also enumerate every coefficient subspace reachable by adjoining any query's
row space, from zero until full rank b. Use canonical RREF over F_r. A transition
that does not increase rank stays in the same subspace. For each state L, all
N=r-1 exact query outcomes determine stays and higher-rank successors. Solve
for expected calls by the exact recursion

    E(L) = (N + sum_{queries increasing rank} E(L+rows(query))) / (N-stays).

Full rank has E=0. Reject nonabsorbing reachable states. Retain all states,
transitions/counts and rational numerator/denominator values. Degree-weighted
request expectation replaces numerator N by sum of query degrees, separately
for original/reduced. These are algebraic Markov recurrences under the explicit
iid sampler, not calibrated costs or observed wall time. Provide a faithful
Lean proof attempt for the recurrence/uniqueness/conditional interpretation;
do not label that alone a formal probability expectation theorem.

Compare every canonical main/twist base pair at fixed field,b (100+100+9+1
=210policy pairs). Primary joint criterion: strictly larger EMPTY-STATE expected
rank gain AND strictly lower mean named operational degree. Also retain useful
probability, expected calls, degree-weighted requests and all ties/losses.
Report source/twist nondominated policy sets over useful rank gain/mean degree
and expected collection calls; do not omit strongest source controls. An
improvement over one declared base need not be intrinsic to twisting. Matched
row/state transport must conserve all useful-yield and call profiles; degree
profiles are observed, not assumed equal. Do not choose a target subset or
policy after seeing outcomes and present it as a preregistered uniform result.

## Independence, proofs, controls and resource envelope

C++ may reuse frozen native field/polynomial/adaptive helpers, exact rational
arithmetic uses GMP exact rationals. Independent Rust uses existing separate
point/field/row/witness arithmetic plus independently implemented exact
rational/state recursion; no C++ arithmetic shared. Recompute all responses,
matrices/ranks/states/rationals and actual scalars, not merely producer hashes.
Pin all sources/inputs before native execution, commit compiled source first.

Rehash manifest after corrupting: false row, degree, witness, omitted model/base
or target, false state transition/rational, lost actual query, false log/scalar
and stale source. Require rejection/no receipt; explicit null if a proposed
failed-query class does not occur. Use valid curve points; no invalid-curve
scope. Faithful Lean attempts/gate/axiom audit; source-bound IDC1 for any new
polynomial identity. All previous proofs/results/certificates immutable.

Native cap600s, focused iteration3600s, existing local host/two build workers,
no paid compute or extra upload/credential reuse. Fresh root release library,
affected backend/workflow tests, exact replay, all-source/raw failure retention,
report/graph/inspected PDF, artifact/size/naming/card checks and existing PR
update. No timing/GAE/S/rho/external-fastest/leaderboard promotion or general
hardness claim. Actual Conductor precheck127/no reservation/replacement.
Broader mathematical, coordination, review, CI/access and integration holds
remain; open theorem/null result does not cancel independent feasible research.

Pre-execution tooling correction: the first compile found Boost headers absent.
Use existing installed GMP/gmpxx headers/libraries for exact C++ rationals
(-lgmpxx -lgmp), preserving the same rational protocol and independent Rust
implementation. Failed source/log retained; no native execution preceded this
correction and no new resource/provisioning is requested.
