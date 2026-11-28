# Branch-local affine consequence bridge, v1

Frozen before implementation, 16 November 2026. Native fixed development
controls, not a benchmark or scaling campaign. Original task: continue toward
an index-calculus scaling advantage while retaining the original route criteria.

## Sources checked before choosing the implementation

KB MCP is unavailable; `scripts/research-kb status` returned127 (no executable).
Repository fallback: static `affine-closure-v1/{PROTOCOL.md,NEXT.md}` and native
compiler/checker, unchanged worklist v1 engine and controls. Its static pass
retained all28 curve outcome classes/node counts. Shoup NTBv2 section14.4
pp388–391 explains the classical row operations already used.

Primary source read: Soos, Nohl, Castelluccia, *Extending SAT Solvers to
Cryptographic Problems*, section3.2 pp248–249,
<https://www.msoos.org/wordpress/wp-content/uploads/2011/03/Extending_SAT_2009.pdf>.
It invokes Gaussian reasoning at propagation quiescence and restores saved
matrices on backtracking; its reported search-space reductions need not reduce
time. That work is over Boolean XOR/GF2. It supplies prior art for integration
and rollback, not a proof or speed prediction for these prime-field quadratics.
The first guessed paper URL returned404; it was corrected using the author's
publications index. No novelty is claimed for this standard hybrid approach.

## Question and competing predictions

Can specialization after solver assignments expose linear combinations that
the original single-equation extractor misses? H1: certified combinations can
propagate or reject a branch, changing root discovery/node counts. H0: existing
propagation already makes these combinations redundant; compiler/checker and
dependency rebuild overhead adds work or reaches a cap. Preserve both outcomes.

## Public interface and invariants

An isolated crate forks the unchanged worklist engine, recording its original
source identity. Modes are Off and BranchAffine. Inputs are only public System,
semantic branch indices, mode and finite limits. No labels, signs, point list,
scalar or witness may enter a solver query. A disabled copy must exactly match
old v1 answers, full decisions and old counters on every completed declared
control before interpreting the enabled arm.

At quiescence before a nonunique branch or Unsupported exit, specialize ONLY
original public equations under the current assignment trail. Invoke the
immutable static affine compiler/checker on that specialization, keeping
semantic inputs protected. The active System retains every original equation
and variable as a literal prefix and appends only coefficient-checked
conditional consequences. Recompute rather than claiming an incremental basis.

Record every assignment, activation, drop and undo. An independent arithmetic
path recomputes specialization directly with BigUint coefficients, checks the
exact premise snapshot against a replayed LIFO trail, and invokes the old
coefficient checker. A generation MUST be dropped before any Undo; no row may
survive removal of one of its premises. At a replacement/rebuild, reset caches,
dependencies and queued flags. Returned roots require independent original
assignment checking and native indexed-row/group replay for curve inputs.
This is conditional coefficient/lifetime checking, not a search-completeness,
formal compiler, signed execution or prime-field proof.

## Fixed domains and controls

Copy the exact28 sealed public request byte strings from the immutable static
qualification:13toy targets times original/indexed, plus two distinct actual-q1
inputs. Keep root order, semantic roles and v1 limits. Compare old v1, exact
disabled fork and enabled bridge; retain every Root, Unsupported, cap and
NoRootObservation. The actual domain remains the distinct TWO-column q1 case,
not the huge original q213156431 base. Huge actual inputs receive no solver call.

Add Fp29 quadratic controls: a coupled linear system; a conditional coupled
system with z0 exposing a root before z1; a conditional z0 contradiction then
z1 root requiring drop-before-undo; a protected-input control; a genuinely
underdetermined nonlinear Unsupported control. Inspect their complete original
truth sets where bounded (at most three variables,24389 assignments). Mutation
controls must include wrong premise, row coefficient, activation binding,
LIFO/duplicate assignment and omitted Drop before Undo. Retain malformed input
and forced pass/trace/solver caps. No refusal is an absence certificate.

## Finite resource policy and acceptance

Old solver limits remain4096variables/8192equations/32768terms,degree2,
1024nodes/depth128/10000000 charged diagnostic events/64MiB cumulative modeled
reservations. Dynamic pass cap1024 and trace-event cap65536 are additional
finite refusals, never cap increases. Charge specialization/rebuild/trace
reservations and static compiler's recorded diagnostic/model work to those
old event/reservation budgets; compiler per-call limits are bounded by the
remaining budgets. The unchanged coefficient checker has its own finite work
guard. Its internal arithmetic, serialization, allocation/container overhead
and original/group replay remain unpriced: full cold cost acceptance is OPEN.
These caps bound the declared implementation counters, not all physical work.

Acceptance is a checked conditional-certificate/lifetime bridge and faithful
disabled baseline, with all declared outcomes retained. A node reduction is
reported only per fixed input and is not a speed or scaling claim. No elapsed
timing, external fastest-tool comparison, rho comparison or leaderboard
promotion is attempted. No paid job, sampling, full-base enumeration or broad
root-library rerun. Native development checks and immutable qualification only.

## Evidence and publication

Preserve all old source/criteria/raw/claim/report bindings; new code lives in
`tools/torus-branch-affine`, evidence here, reports beside it under
`reports/branch-affine-v1`. Save failed source/build/control attempts. Freeze
source snapshot, copied executable and thin-shell qualifier before qualified
calls; retain every request/certificate/trail/root/refusal and complete native
log. Use the actual artifact policy and genuine receipts. Extend the existing
appropriate regular PR, lint its body first and restore the same maintenance
checker after publication. Signed standing remains unverified.
