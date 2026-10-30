# Setup-charged signed base cache: fixed comparison

2026-10-25, PR2381. Commit this protocol before new execution. Native arithmetic,
producer, auditor and proofs are versioned additions; all previous rounds stay frozen.

## Hypothesis, reference and scope

Replace repeated binary ±1 scalar requests on the same two factor-base points
by a validated signed-point cache. Preserve every ordered signed lift (including
after the first successful lift in a query), root, coefficient row/rank, complete
solver certificate, degree/witness/promotion trace, failed shift, base log and
recovered source scalar. The reference is the checked-root candidate at
 e1e0b9cbf0821e5c89639245267c1c2986aeba12, not the older field scan.

Execute40 setups and400 separate nonidentity source recoveries per arm:
ten frozen F43 size2 policies, roles main/twist/matched/pulled_back_twist_base,
ten targets each. Thus80 paired setups/800 source checks. All inputs are the
thirty preceding reduced/adaptive/known-root case files. Known-answer labels
only generate/check public synthetic fixtures outside recovery. Recovery accepts
Q alone. Keep the actual F43 main icv1-fp6-t0-74779950 P=(31,18), twist
icv1-fp6-t0-c6a8a47c U=(24,26), order11 and explicit additive/injective
phi(x,y)=(41x,11y). This is exceptional base-field-isomorphic evidence only;
U is not EC1-registered and no older generator alias is reused.

## Exact cache and accounting

Both arms retain checked-root setup/validation and the entire named row solver.
For each base-column i=0,1, build two entries in order (-1,+1): negative is one
native curve negation, positive is the supplied base point. Emit index, sign and
point for all four entries. Charge existing point_negations for the two
construction negations. Four new counters extend the existing32-component vector:

- cache_entries_built: one stored entry, four/setup;
- cache_entry_validations: one canonical finite on-curve check, four/setup;
- cache_binding_checks: one sign/index/source binding, four/setup;
- cache_reads: one point read, two per tested signed lift.

For each entry validate its finite canonical F43 coordinates and actual curve
membership. Positive entries equal the supplied base; negative entries must sum
with that base to O. Charge both negative inverse-validation additions in the
existing point_add_requests counter. No extra uncharged negation is used for
validation. Distinct entries are addressed by original base-column index and sign,
never by sorting the base. The reference has zero values for all four new counters.

At each sign-lift candidate, read the two cached points instead of the old binary
scalar calls. Still perform and charge their actual point addition and comparison
with the query point, and still test ALL signs before ending that query. Setup,
query generation, relation-log solve, target shifts and final source verification
are otherwise unchanged. No table of relations, targets or scalar answers.

The old binary ±1 chain requests two additions per operand (including its final
unused double), hence four per lift before summing. The candidate requests one
sum per lift, two cache reads, and two inverse-check additions once at setup.
Check the exact ledger implications: point-add change -4L+2 in setup/cold and
-4L online; scalar-iteration change -2L; point-negation change -L+2 in setup/cold
and -L online, where L is that phase's complete signed_lifts count. All other
old components must be identical. Preserve every separate cold record; no
multi-target average replaces one-target work.

Counters are DIFFERENTLY WEIGHTED and may be nested. Field tables, allocations,
hashing, sorting/traversal, coordinate primitives within curve checks, cache
storage cost and auditing remain unpriced. No summing requests into time/GAE,
S or speedup. An emitted cache-validation request is not its calibrated cost.
This round tests a query/lifting algorithm request change, not a complete cost win.

## Proof and independent controls

Before native execution, prove in Lean that an entry equal to P for plus and
summing to O with P for minus equals ±P. Prove ordered filtered lift-list equality,
and shifted source recovery using the actual injective map and explicit source
annihilator, with a concrete F43 instantiation. Do not assume the conclusion as a
cache correctness hypothesis: derive negative equality from its validation.
The native arithmetic/demand algorithm is independently replayed, not claimed
universally Lean-formalized. A source-bound IDC1 for y+(-y)=0 certifies only the
coordinate polynomial; the group/list/recovery implications rest on Lean.

Independent Rust uses separate point/field/polynomial arithmetic and reproduces
both arms' full vectors/cache checks/order. Require the reference exactly equals
frozen known-root records after dropping new zero counters, every new certificate
matches the prior one, and the old batch-RREF/witness semantic audit passes.
Rehashed controls: false cached sign, false cached point, missing cache entry,
false cache demand/read count, stale source, missing failed query, false recovered
scalar. Each must reject101/no receipt. Retain exact payloads/logs and false
identity/source-hash controls. No skipped failures or favorable target selection.

Acceptance is complete mathematical equality, independent scalar verification
and the exact request deltas above, including setup costs. Candidate cold
point-add requests must be lower on all400 target records for that particular
component claim. Retain any loss/null or mismatch; fix a defect without changing
the population, or choose a different lead. A lower component count does not
establish runtime, calibrated total work or intrinsic regularity. Appropriate
rho/external-fastest/ecbench/reference and benchmark gates remain for actual
speed claims. No registry, cover, leaderboard or benchmark row is to be changed.

## Budgets and delivery

Commit this protocol and compiled source/proof hashes before first execution.
Native command cap600s; one iteration envelope3600s, existing local resources
only, two build workers. No paid compute, artifact upload or credential reuse.
Fresh directories for every attempt, exact raw failures, source-linked report,
quantitative graph/diagram and inspected PDF. Mandatory fresh root release/locked
library tests, isolated backend tests, workflow/naming/card/artifact checks and
unmodified scoped Lean gate. Actual Conductor precheck exits127: no shared lease
or replacement coordinator. Profile-tool availability is checked; missing tools
supply no cost evidence. Keep PR open with all review/CI/integration and broader
scientific holds, and continue the existing single automation with a next step.
