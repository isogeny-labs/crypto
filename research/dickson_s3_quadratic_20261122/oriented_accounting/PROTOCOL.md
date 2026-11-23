# Oriented fourth arm and matched extraction protocol

Status: frozen development protocol, written before the new native controls.
Parent task: PR #4950. Earlier study evidence is immutable. All new observations
are unsigned producer evidence, `[pcc:unverified]`, until separately verified.

## Mathematical and implementation scope

Reuse the parent-oriented formulation from PR #4928: at a node with child
abscissae x,z and signed parent (r,s), impose lambda^2=x+z+r and
2s lambda=(x-r)(z-r)-(3r^2+A). Share the nonroot internal ordinates, and impose
y_child=lambda(x_child-r)-s for each internal child. The root is the actual
signed public target. No division by s is part of the equations. With m leaves
and depth l, O uses ml+3m-5 variables and ml+3m-4 equations.

All four arms use the same selector, public target, finite prefix chart,
variable ordering policy, scalar field and limits. Candidate extraction uses
the production **traced** visitor, no supplied variable domains, and refuses
its `full_field` branch before recursion. This is a deliberate backend choice:
the traced path records native F4 and retains incoming generators; it is not
neutral instrumentation of the previous plain visitor. Capped/trace-stopped
attempts are incomplete, including when no witness has yet been found.

## Fixed controls and stopping rules

1. F7: y^2=x^3+3, prime order r=13, G=(4,2), Q=(1,2), selector depth 1.
   For m=2,3,4 use the same public schedule (a,b)=(0,1). Independently enumerate
   the explicitly signed base after candidate extraction. Before execution the
   hand-derived finite ordered counts are 2/4/16, multiset counts 1/2/2, and
   per-cell coefficient-row counts 1/2/1. L's predicted raw multiplicities are
   4/16/128; other arms 2/4/16. Eight of 24 m4 group matches have an infinite
   nonroot prefix. The predicted values are assertions, not observations.
2. F7 zero-parent ordinate decoder control: A=3,B=3,target (1,0), points
   (3,2),(4,3) and their negatives. Keep both slope signs. This is a chart
   correctness control, with no prime-order rank claim.
3. One M31 known-answer development extraction: p=2147483647,A=847514846,
   B=2087444724,r=2147446831,G=(625037715,2100627251), depth 3, m=2,
   public Q=2(69204140,78579263), formed by a separately charged native fixture
   operation. The point is not passed as a solver domain, assignment or hint.
   Run each arm once; max pair degree 8, cooperative deadline 10 seconds,
   no full-field fallback, at most 20000 assignments and 100000 trace events.
   No retry or expanded search budget after seeing an outcome.

These are bounded native development/correctness controls, not a target cohort,
speed ranking or local scaling experiment. An outer 120-second deadline and a
3-GiB process address-space limit apply to each executable call. Cooperative
checks do not guarantee a hard native matrix memory bound. An outer kill is
censored process evidence, never a complete empty extraction.

## Cold boundary, signs, duplicates and useful yield

Each arm starts cold: primality/nonsingularity/subgroup admission, production
selector root construction, square roots, explicit signed base and indexes,
public target schedule construction, equations, visitor traversal, sign
reconciliation, membership, literal point replay, all deduplication, row
construction, native incremental rank and final recovered-scalar replay.
No known base logs or secret scalar enter candidate extraction. A public base
point equal to G may supply its charged log(G)=1 anchor row. Its rank is shown
separately. Hasse plus nonzero prime-order G certifies the required h=1 scope.

Report separately: raw assignments; full leaf-and-intermediate x chains;
ordered signed witnesses; signed multisets with multiplicity; distinct full
augmented rows (coefficients **and RHS**); independent/dependent/inconsistent
rows. For R=aG+bQ, form (c_1,...,c_B,-b) with RHS a over F_r, never F_p.
All cells for one arm share the same Q and cumulative row set/rank. Dedup must
not discard equal leaf-x tuples before recovering signed chains. A/Q derive
slopes from the signed parent, branching over both roots when its ordinate is
zero; L reconciles local line signs; O consumes slopes and shared ordinates.
Every retained witness must be in the actual base and replay to the target.

Opt-in production visitor timing partitions elapsed traversal into native F4,
root recovery, callback, and residual traversal. Callback subphases and an
outer cold residual account for all elapsed work through in-memory record
assembly; final JSON serialization and process startup are outside this
boundary and covered by the external call receipt. Root finder and row-reducer
operation counts remain their actual subsets. Group field counters are emitted
explicitly because GroupOps' default serializer omits them. Selector arithmetic,
polynomial construction, validation arithmetic, key comparisons, allocation,
and general runtime work are **not** given invented complete primitive costs.
Complete wall coverage does not establish a complete bit-operation price.

## Qualification and publication

Inventory exact sources and protocol before building; preserve each build/run
attempt. Freeze successful executables read-only before use. Run independent
small-field oracle/negative-boundary controls, then the one fixed M31 attempt.
Resume the root-library release build cache once under its existing finite
budget; scoped qualification is not a substitute for that gate. Register a
source-bound unsigned PCC statement before citing observed new results. Publish
source, raw receipts, report, editable figure and visually checked PDF on an
open PR, preserving published main's append-only ledger prefix.
