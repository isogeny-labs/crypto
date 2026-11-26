# Four-leaf composition of the existing oriented S3 identity

2026-11-24. Local source and output qualification, not a cost benchmark.
The original oriented-pair example is a construction/F4 diagnostic; the
composition and signed extraction here are new adapter code. They are not an
existing complete four-leaf Isogeny Labs API or a new complexity result.

## Prior source and exact scope

The source is
[`prime31_s3_oriented_pair.rs`](../prime31_ic_20261115/native/examples/prime31_s3_oriented_pair.rs),
especially `oriented_raw` and `lift_residuals`, with its
[report and retained negative diagnostics](../prime31_ic_20261115/native/ORIENTED_PAIR_S3_F4_20261115.md).
It gives the classical chord/tangent lift of Semaev S3. The current adapter
uses the same identities parameterized by the two existing public toy curves,
and invokes the unchanged production `f4_fp::solve_visit_traced` engine and
resource admission. It does not reuse the original example's fixed prime31
constants or describe its historical incomplete F4 output as a complete solve.

The intended output is every signed four-leaf relation summing to the public
target, with four distinct abscissae and no leaf at the target abscissa.
This is the full cofactor comparison scope, not the Refined regular subset.
The solver receives only the public curve, factor base and target.

## Equations and completeness boundary

For each pair with abscissae x,z, first ordinate y, slope t and sum abscissa w,
use the existing equations

\[
y^2=x^3+ax+b,\qquad t^2=w+x+z,
\qquad 2yt+(z-x)w=2x^2+xz+a.
\]

Recover the second ordinate as y+t(z-x) and the pair-sum ordinate as
v=t(x-w)-y. These give the usual affine group sum, including doubling when
x=z and y is nonzero. Inverse pairs summing to infinity are outside this
affine model. They cannot occur within a required distinct-abscissa tuple.

For two pair sums (w0,v0),(w1,v1) and target (r,s), add an outer slope u with

\[
u^2=w_0+w_1+r,\qquad
v_0=u(w_0-r)-s,\qquad v_1=u(w_1-r)-s.
\]

The line contains both pair sums and -target; the cubic root-sum condition
supplies the group relation, including a tangent when the pair sums agree.
The existing fixtures have odd prime order, so affine 2-torsion does not occur.
Each valid distinct-leaf tuple admits these slopes for all 24 orderings.
Conversely, every returned assignment must pass separate curve membership,
both pair sums, total sum and signed-base replay. Repeated-x or target-x leaves
are explicitly filtered only after replay; other replay failures are errors.
These arguments are source-level reasoning, awaiting independent standing.
[pcc:unverified]

Dense membership uses the exact product over liftable base abscissae in each
slot. Tower membership uses the original quadratic Dickson chain to the
frozen terminal in each slot. Its oriented equations enforce rational curve
lifts for all four leaves; over the toy rational field this gives the same
signed output set. The algebraic-closure ideals and variable counts differ,
so raw F4 matrix counts alone are not an equal-ideal cost comparison.

A labelled four-leaf system has 24 assignments per distinct-leaf relation.
The existing F127 13-row fixture therefore has at least 312 rational
assignments. A single-valued parametrization by one F127 coordinate can encode
at most 127 distinct rational assignments. This rules out that particular
RUR representation of the full labelled output, not the relation algorithm.
Use native recursive algebraic-root extraction rather than the external RUR
backend. Do not erase multiplicity by dropping some leaf orders from the
complete-output contract. [pcc:unverified]

## Qualification before panels

1. Evaluate both membership systems on every row and every ordering of the
   existing F127 13-row target. Check the derived signs independently and
   retain the 312 distinct witness assignments for each model as a unit assertion.
2. Check an already-expired budget calls neither builder nor solver.
3. Use a separately planted fixed-slot unit control through the actual native
   visitor, comparing all sign choices for those slots to the test-only direct
   reference. This is not a full-base natural query.
4. Run the complete feature-enabled suite and unchanged default suite. Preserve
   all initial compile/test failures, then freeze the exact source and binary.

## Fixed panels and resource limits

Run dense and tower modes on all existing targets, in that order per target:

- F127 depth four: four targets, eight records.
- F8191 depth four: four targets, eight records.
- F8191 depth five: five targets, ten records.

Every query has one original three-second enclosing deadline, including
construction and signed extraction. The new common native budget is degree
16, 20,000 input terms, 4,096 visited assignments, 20,000 pairs, 8,192 symbolic
monomials and 2,000,000 matrix entries. Degree 16 admits the dense degree-14
membership input; it does not change the earlier cofactor protocol's degree-10
cap. No timing ratio compares these different internal budgets. Scope is
explicitly limited to depth 3..5 and base size at most 16.

Use one worker, 2 GiB whole-process address space, a 60-second outer panel
timeout and five-second kill grace. No panel retries or cap increases.
Every system and native traversal status is retained, including allocation
refusals, assignments visited, chart filtering and forbidden fallback counts.
Only algebraic-univariate root branches are permitted; full-field or supplied
finite-domain fallback is refused before specialization. This does not itself
establish subenumerative complexity for algebraic branching.

Compare complete signed sets with the existing direct reference. A mismatch
is logged before nonzero exit; any incomplete traversal remains undetermined
and cannot expose a partial set as exact. Output and construction limits,
deadlines and unsupported inputs are not no-relation certificates.

The raw records will be in `evidence/oriented_v1/`, with before-call source and
binary hashes. The full cold recovery comparison, independent signed replay
and admitted measurement remain separate gates. The existing failed cofactor
and external-solver protocols remain unchanged. No paid run or external stream
delivery is scheduled by this development protocol.
