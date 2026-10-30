# Proposed versioned factor-base policy contract

Design only, 2026-10-30. No implementation, build, experiment, generation, Git
commit, external-task message or cloud action was performed for this audit.
Source was inspected in `crypto-ml-sat-prefix-20261030` at observed HEAD
`8bddd785ff0d44f50b6710649ed403401075f5cb`; this worktree is active, so the
source hashes in FACTOR-BASE-POLICY-SOURCE-MAP.md identify the inspected inputs more precisely.

## Scope and intended learning tasks

Add an explicit, versioned factor-base policy axis to the existing dataset CLI.
Keep Koblitz, ordinary binary and prime curves, both twists, n23..31 and47..61
(including p=2^61-1), m2..15, all existing backend names, multiple generators
and keys in the requested plan. The new axis does not expand a backend's proven
domain. Unsupported, empty, capped, interrupted and failed cells are retained.

The smallest useful increment compares the current signed projected small-x
population with a union of complete signed Frobenius point orbits, using the
same public target. It trains three distinct tasks:

1. Given a public curve/subgroup and permitted construction budget, propose a
   base policy or orbit-selection action. The target is subsequent independently
   checked yield/rank/cost under a fixed solver protocol, with censored outcomes.
2. Given a concrete public point population and action graph, predict structural
   properties or propose relations/identities. Check proposed identities and
   point relations exactly; orbit count is not independent relation count.
3. Given a causal solver prefix for that exact domain, propose the next algebraic
   or SAT action. Future actions/outcomes never enter the feature context.

Policy imitation is not policy optimality. A low observed solving degree is not
an exact first/last fall degree. No empirical finite sample proves a universal
fall-degree statement. A putative counterexample needs the precise hypothesis,
ring, field equations, ordering and independently checked certificate.

## Reused literature and existing implementation

Read `docs/ml-dataset/DESIGN.md`, `tools/factor-base/FACTORY_DESIGN.md`, current
coverage reconciliation, and the actual recipe/curve/export source before this
proposal. Existing factory recipes already include binary linear/affine spaces,
trace-zero/geometric controls, prime progressions, random x and Frobenius orbits;
do not reinvent them or claim that dataset integration already exists.

Primary source checked: Avanzi, Ciet and Sica, *Faster Scalar Multiplication on
Koblitz Curves*, §§1–2, https://www.iacr.org/archive/pkc2004/29470028/29470028.pdf.
Its nontrivial squaring action uses a model defined over F2 and its subgroup
scalar interpretation uses an invariant cyclic subgroup. This motivates separate
checks for model preservation, subgroup preservation and coefficient transport.

Primary source checked: Kosters and Yeo, *Notes on summation polynomials*, §2 and
§4, https://arxiv.org/html/1503.08001. Summation-polynomial zeros require rational
lifts/group verification for this corpus; small first fall degree does not by
itself justify a regularity or runtime label. Their trace morphism in §4.2 is a
useful later exact structural feature/control with explicit ordinary-curve
assumptions. This proposal does not assert a new theorem or extend their bounds.

The factory's cited SAT article at
https://pmc.ncbi.nlm.nih.gov/articles/PMC7334981/ returned a browser challenge.
No unread contents of that source are used here. This increment changes domain
construction, not the SAT algorithm.

## Compatibility and minimal API

Keep the existing `DatasetPlan`/`CaseConfig` v1 serialization and old generation
path byte-for-byte. Their compact typed serialization defines current plan hashes;
adding a default serialized field silently changes historical identity.

Introduce explicit dispatch types (`PlanDocument::{V1,V2}` and a versioned case
envelope), with normalization into an internal `CaseSpec`. Proposed V2 adds
`factor_base_policies: Vec<FactorBasePolicy>`; each element has a versioned kind
and explicit immutable parameters. Reject duplicate canonical policy objects,
unknown kinds/fields and arithmetic overflow. V1 normalization yields a legacy
policy without rewriting the historical source bytes or identifiers.

Minimal new public interfaces:

```text
FactorBasePolicy = LegacySmallXProjectedV1
                | SignedFrobeniusUnionV1 { max_candidates, max_points,
                                           max_action_steps, max_artifact_bytes }
construct_factor_base(verified_context, policy, requested_points)
    -> { public_definition, restricted_construction, costs, terminal_status }
verify_factor_base_policy(verified_context, expected_policy, artifacts)
    -> VerifiedFactorBase
VerifiedFactorBase exposes ordered_points, domain_digest, verified_action_maps
```

Retain existing `--factor-base-size` as a requested population size. Add optional
`dataset plan --factor-base-policies legacy-small-x-v1,signed-frobenius-union-v1`
and `--factor-base-policy-file PATH` for explicit parameters (mutually exclusive).
Omission emits the unchanged v1 plan. Explicit policies emit plan/v2, including
explicit legacy-only requests. Print policy count, sizes/caps, total cells and
per-backend applicability in plan/preflight/capabilities output. Policy is a new
most-significant cell-index digit, so each policy occupies one complete existing
axis block; twist remains the current inner digit and paired coverage is intact.
All existing generate/status/verify/export/start/budget paths must dispatch v2.

Use `ml-ecdlp-factor-base/v2` only for new construction. V2 must bind the exact
policy, requested and actual size, ordered concrete points, action descriptor,
action maps and orbit records. A distinct domain digest binds field/model,
certified subgroup, actual ordered points and action/domain semantics. Keep
policy/provenance identity separate from mathematical population identity: two
policies producing the same domain are duplicate mathematical inputs, not two
independent examples. Do not expose random seeds or construction histories via
a feature hash. Occurrence IDs bind plan/cell/source artifact separately.

## Signed Frobenius union algorithm and mathematical gates

Work on the exact twisted model, not the family's untwisted shorthand. For a
binary model independently compute the least d dividing n fixing both actual
coefficients. Define phi(x,y)=(x^(2^d),y^(2^d)), with ambient order t=n/d.
For prime-field rational points use phi=identity, t=1; no artificial nontrivial
prime-field Frobenius examples. In an even-degree Koblitz twist d may exceed1.

First certify that phi preserves H=<G>. If r divides #E exactly once, the
unique order-r subgroup is invariant. If r² divides #E, [r]phi(G)=O alone is
insufficient: verify phi(G) is in <G> with the existing bounded cyclic-membership
checker. Charge that work; restrict computed logarithms. Failure to establish
stability yields `unsupported_subgroup_action` or `membership_budget_exhausted`,
not a silent fallback to identity or a subset of an invalid orbit.

Enumerate x=1,2,... under a frozen finite candidate cap independent of target,
key, m, solver and outcomes. Lift all rational points in canonical order. Apply
the existing exact prime-primary projection and cyclic membership check. For
each eligible P, enumerate A.P={(-1)^s phi^j(P):s=0,1;0<=j<t}; deduplicate by
canonical coordinates. Canonicalize each complete orbit by its smallest point,
sort its unique points and append it only when not already selected. Record all
lift/projection/rejection/duplicate decisions in restricted construction data.

Use whole-orbit atomic admission. Stop successfully after actual_size >=
requested_size; a final orbit may overshoot by at most2t-1. Never exceed frozen
`max_points` (at most4096 initially). If the next new orbit would exceed it,
terminate `point_cap` before adding any of that orbit; do not scan onward for a
convenient smaller orbit. Exhausting candidates or action/byte budgets retains
the already complete orbits and a typed terminal reason. Success also permits
exhausting all r-1 nonzero subgroup points. An empty result cannot enter a solver
as a nonempty base. This rule is intentionally deterministic; an exact-size
subset-sum/knapsack orbit selector would be a different future policy.

For every point retain exact index maps N(i) and F(i), both total permutations.
Check N²=I, F^t=I, NF=FN and coordinate equations independently. Partition the
population into disjoint complete signed orbits; verify their union is exactly
the population, with no duplicate representatives or omitted points. Record
each point's canonical (representative, sign, exponent) witness and all
stabilizer pairs (s,j) fixing its representative. Use the specified abstract
action C2×Ct: |orbit|×|stabilizer|=2t, even when the action is not faithful.
Keep this distinct from the existing Frobenius-only stabilizer t/orbit_length.

Materialization does not authorize compressed relation columns. A compressed
action requires a separate verified lambda satisfying phi(G)=[lambda]G and
coefficient transport (-1)^s lambda^j mod r, bound to the subgroup/generator.
Finding lambda is setup work, never free witness access. Without this certificate
use expanded points and ordinary relations. A diagonal action transforms target
Q to phi(Q); it is generally not a symmetry of the fixed-target equation.
Permutation of summands is valid only with identical domains/compatible encoding;
independent sign flips or independent Frobenius actions on summands are not a
relation symmetry. Any augmentation must transform the complete relation.

## Solver binding and learning export

All solvers consume exactly the independently verified concrete domain, not the
source x-candidate list. Cofactor projection changes x coordinates; never label a
projected population as satisfying the original linear/interval restriction.
Retain ordered signed-point-to-distinct-x maps for SAT/Weil/F4/F5 and the exact
lift/sign membership maps. Replay recomputes them from population coordinates.
F6/root/PKM/large-prime adapters retain their actual supported arity/family/domain
and explicit alternative-base rules; an orbit policy does not imply support.
Alternative Kummer bases must not inherit a standard-domain digest accidentally.

For V2 public features allow only exact curve/field/subgroup, G,Q,m, concrete
point/domain coordinates, action descriptors/maps, orbit partition/stabilizers
and a canonical mathematical policy definition if meaningful at decision time.
Exclude candidate seeds, generator multipliers, membership logs, rejected
candidate history, scalar answers, outcomes, costs, timings, future solver events
and raw provenance. Reuse strict recursive allowlists, not copying arbitrary
new JSON objects. A policy-selection example must not expose its chosen future
population; a post-construction solver example may expose that population.

Version the export projection while preserving v1 feature bytes/IDs unchanged.
Feature IDs hash only the permitted mathematical context. Restricted records
carry policy occurrence/provenance, construction decisions, costs and labels.
All policies, generators, keys, twists and equivalent encodings at one field
size remain in the existing conservative whole-size split. Cross-size algebraic
links remain an explicit unresolved split issue, never claimed merged.

## Artifacts and accounting

Retain definition, complete ordered points, candidate/lift/projection stream,
dedup/admission events, action/stabilizer witnesses, domain adapter maps, raw
solver inputs/events, relation verification, rank/recovery certificates and
terminal summaries as immutable bounded artifacts. Bind them through source and
protocol hashes; use the existing native store publication/readback path. A
trace cap marks incomplete raw execution coverage and cannot silently produce a
complete replay claim. Never archive credentials or task lease files.

Separate constructor, membership precomputation, orbit generation, verification,
encoding, solver, lift/relation checking, filtering/rank, scalar recovery,
serialization and S3 I/O cost. Count candidates, lifts, group/field operations
where actually instrumented, action steps, unique points, duplicate hits,
stored bytes, PUT/GETs and failures. Label instrumented subscopes precisely;
native instruction counts require admitted hardware counters with thread and
privilege coverage. Missing counts stay null, not estimated from elapsed time.
Report cold total and amortized cost with explicit reuse count; no double charge
for a shared artifact and no omission of its original construction.

Matched performance comparisons require a separately frozen official protocol:
same public targets, actual retained-point sizes or explicit size strata, same
solver settings and resource envelope, randomization, A/A, control policies,
discovery/confirmation separation and charged failures. Request size alone is
not a matched size. Orbit expansion already increases B; do not multiply the
same materialized population by orbit size again in a yield estimate.

## Required acceptance matrix (not executed)

Positive checks: legacy plan/config bytes, hashes, indices and exported feature
bytes unchanged; explicit V2 singleton/multiple-policy indexing and checked
counts; both twists across all families; model-preserving actions with t=1 and
t>1; odd/even binary degree; stable repeated-r subgroup fixture; short orbits,
sign/Frobenius intersection and nonfaithful action stabilizers; target/key/m/
backend independence of construction; order independence of candidate duplicate
encounter under the fixed canonical rule; exact full-orbit boundary and subgroup
exhaustion; one retained partial result per cap reason; exact signed/x-domain
maps and one positive relation replay per supported adapter.

Negative checks, including rehashed semantic corruption: changed policy with
same points; changed ordered population; forged/missing/duplicate action edge;
wrong sign, power, representative or stabilizer; omitted orbit member; squaring
on a coefficient-nonfixed model; orbit leaving <G> despite [r]P=O; unsupported
compressed action without lambda; wrong lambda/transport; transform summands
without target; wrong twist; projected x falsely declared in source x-space;
duplicate/unknown policy, integer overflow, extra JSON fields, missing artifact,
wrong role/hash, premature claimed success, and truncated orbit labelled closed.

Leakage tests inject unique sentinels into every restricted nested field and
prove exported bytes/feature IDs unchanged; mutate one permitted mathematical
input and prove identity changes. Policy occurrences producing equal public
domains deduplicate mathematically while retaining distinct charged attempts.
All related examples remain in one split; all source artifact hashes are stable
across export/retry. Corpus coverage smoke tests return typed unsupported/capped
outcomes rather than demanding expensive solves at every requested n/m.

## Open conditions and staged scope

Before implementation settle the public serialization names and finite defaults
for action/byte/candidate caps; these affect plan identity. Do not run a campaign
with unspecified budgets. Coordinate reserved mod.rs/solvers.rs/export.rs paths
before integration. The minimum increment is V2 axis plus complete orbit-union
construction, replay, export and CLI checks; native recipe factory integration
is a separately versioned next increment, preserving its subgroup-filter versus
projection semantics. Existing field/solver support limitations remain visible.

Subgroup stability may be expensive or unprovable within caps in repeated-r
cases. General conjugate-model transport, nontrivial prime automorphisms,
isogeny-linked augmentation, optimal base search and relation-column compression
remain explicit future capabilities. No generation, cloud corpus, model training,
novel SAT algorithm, speedup or fall-degree resolution follows from this draft.


## Implementation freeze: 2026-10-30

Task T-936; isolated branch codex/ml-dataset-factor-base-policy-20261030,
base3d2632ce31bd. This addendum fixes the implementation choices left open above.
No generation or measurement campaign is authorized by a software contract.
The original minimum deliverable remains the full v2 policy axis, construction,
replay, export and every dataset CLI path; standalone APIs are an intermediate
implementation step, not completion.

### Types, compatibility and registration

Use public types::plan_v2::{FactorBasePolicy, DatasetPlanV2, PlanDocument,
CaseSpec}. Register plan_v2 as a public child of the existing public types module.
Register factor_base_policy as a public child of the public curves module.
These registrations avoid modifying currently reserved mod.rs/solvers.rs.

FactorBasePolicy is internally tagged by kind, denies unknown fields, and uses
Rust variants LegacySmallXProjectedV1 and SignedFrobeniusUnionV1. Serialized
kind values are legacy-small-x-projected/v1 and signed-frobenius-union/v1.
The signed variant requires max_candidates:u64, max_points:usize,
max_action_steps:u64 and max_artifact_bytes:u64. Defaults when explicitly
requested by a future CLI selector are 131200,4096,10000000,67108864.
JSON must carry all four parameters: no invisible defaults. Admitted ranges are
candidates1..1000000,points1..4096,actionsteps1..10000000,bytes4096..67108864.
Requested population must be nonzero and no greater than max_points.

DatasetPlanV2 has exactly schema="crypto.ml-dataset.plan/v2", axes:DatasetPlan
(an unchanged valid v1 plan), and nonempty factor_base_policies. Validate both
layers, reject canonical duplicate policies, and use checked total-cell
multiplication. Policy is the most significant cell-index digit. A custom
PlanDocument schema dispatcher delegates v1 compact serialization and binding
hash to the unchanged legacy representation; never add default v2 fields to v1.
The normalized CaseSpec binds configuration, policy and index provenance without
changing v1 CaseConfig bytes. Explicit legacy-only v2 remains a v2 occurrence.
Malformed or duplicate object keys must not be silently accepted in new parsing.

### Construction and verification

Expose curves::factor_base_policy::{construct,verify} and a policy-aware
curve-generation entry point. Factor shared curve/generator/target construction
so the v2 path does not construct and discard a legacy factor base. Preserve
legacy output bytes, order and identities exactly, with regression fixtures.
New restricted construction data is appended only on the v2 path; the existing
CaseData shape remains stable. Public instance identity keeps target semantics.

Freeze numeric (u64 x,u64 y) order for lifted points, orbit representatives and
orbit members. max_candidates counts attempted x values starting at1, no more
than min(max_candidates,q-1); exhausting the field is a distinct terminal state.
Normally process all lifts for the final x. A resource stop may occur between
complete orbit admissions inside a candidate; retain its exact lift cursor and
partial status. Never truncate an admitted orbit or silently skip a too-large
orbit in search of a smaller one.

Use the actual twisted model's least coefficient-fixing Frobenius power.
Certify model and subgroup preservation before closed-action claims. For repeated
r in N, pre-admit bounded cyclic-membership setup/query costs and verify computed
logs by exact scalar replay; [r]P=O alone is insufficient. Membership logs and
any scalar action remain restricted. No relation-column compression is implied.

The mathematical domain digest excludes chosen generator, target, seeds, policy
provenance and construction history. For a nonempty exact-order-r population,
the canonical smallest population point identifies H=<P>; bind field/model,
r, this canonical subgroup anchor, ordered population and action semantics.
Thus identical domains under different generators of H deduplicate. An empty
population has null domain digest and cannot be admitted as a nonempty solver
or mathematical training domain. Occurrence identity separately binds full
public instance and policy.

### Bounded accounting

Charge before allocations or arithmetic where a budget is required. Never call
uncapped CyclicMembership::new before admitting its sqrt(r)+1 table/work bound.
Pre-admit worst-case giant steps before each membership query; preserve actual
reported work separately. Scalar-call admission may reserve 2*bit_length(k)
binary-method group-operation bound, clearly labeled conservative admission
units rather than observed native operations. Field/action, membership, scalar
admission and serialization scopes remain explicit; no instruction conversion.

max_artifact_bytes covers exact serialized policy public plus restricted
artifacts, with a reserved bounded terminal record. Logical membership storage
must also be admitted conservatively before table allocation and separately
identified; neither scope is RSS. Stop before an event/orbit would exceed its
admitted budget, retaining the last complete population and typed censoring.
Replayer must validate arithmetic/maps/orbit partitions, construction ordering,
policy binding, counters and terminal causes; native regeneration alone is not
an independent arithmetic checker. Record which checks use reference arithmetic.

### Validation and outstanding shared integration

Authors may format and author software tests; no concurrent Cargo invocation
while consequence pipeline05 owns the shared target. Required controls include
legacy exact bytes, v2 axis overflow/duplicates/unknownfields, both twists,
nontrivial and identity actions, coefficient-nonfixed twists, repeated-r subgroup
rejection, short/nonfaithful orbits, every cap prefix, semantic corruption,
restricted-label leakage and identical-domain/different-generator identity.
Freeze implementation and inventory before scoped execution; preserve failures.
All plan/generate/start/budget/status/verify/export/capabilities dispatch changes
must be coordinated before v2 corpus admission. Current held files stay untouched.

Literature refreshed this turn: Kosters-Yeo primary arXiv text1503.08001,
sections2 and4, confirms rational-lift and first-fall/regularity boundaries.
The PKC2004 primary PDF request returned403 in this executor; retain the prior
read-only design's source citation without asserting a fresh successful fetch.
No novelty or performance claim is made.

### Final pre-implementation clarifications

Use atomic complete-x transactions: process canonical lifts, projections and
new signed orbits in a bounded pending candidate, then commit its population and
full artifact contribution together. A cap retains the prior committed x prefix
and a separately bounded pending-candidate cursor/reason; it never claims the
pending candidate was completed. Requested-size success is tested after the
whole x transaction. This supersedes the optional between-orbit partial-x choice
above. Pending work actually performed remains charged even when uncommitted.

Every admission includes final derived action maps, partitions, stabilizers,
counters and terminal serialization. Keep conservative reservations and refused
reservations separate from observed executed operations. Unsupported/denied
subgroup membership may retain an empty typed result without a domain digest.

PlanDocument::parse_json(bytes) must probe schema and then deserialize the
original bytes into the selected strict typed V1/V2 struct, rejecting duplicate
fields at every typed level. Do not round-trip through Value before typed parse.
It need not implement generic Deserialize until a duplicate-preserving visitor
is available. Serialize delegates directly to the active inner representation.
CaseSpec has an explicit origin discriminator V1 versus V2, including V2 with
legacy-only policy; normalized policy equality never collapses occurrence IDs.
Validate axes before calling legacy cell() to avoid zero-radix panics.

Core construct handles the signed policy; legacy goes through unchanged generate.
generate_with_policy dispatches accordingly after shared setup refactoring.
Public verify_factor_base may dispatch v2 mathematical checks, but full worker
admission MUST additionally call the full verifier with the expected plan policy,
requested size and restricted construction transcript. Never trust an embedded
policy alone as authorization to change the plan's domain.
