# Proposed cyclic-subgroup ECDLP export amendment

Design only, pending root review. No runtime implementation, build, native
execution or cloud activity. The completed factor-base export proposal02 remains
unchanged. This follow-on is a full requested corpus requirement, not a condition
for integrating the already scoped post-construction PDP view.

## Observed gap and smallest useful extension

The existing exporter initializes a single static feature channel inside its
public.json + factor-base.json + witness.json branch. Its feature_projection
requires a factor base and labels inputs with pdp_domain. The proposed policy
export correctly excludes empty populations from that channel. Nevertheless,
restricted targets still retain scalar-recovery certificates and their scalar
when feature_id is null. known_answer_scalar is separately suppressed when that
PDP feature is unavailable; its witness artifact remains referenced.

Therefore a valid BSGS scalar recovery with an empty, capped, unused factor base
has retained evidence but no model-input row for the actual ECDLP problem. Reusing
the existing feature as if it were PDP would be incorrect. Reuse the existing
bounded sorter, split helper, source/certificate custody and checked scalar
arithmetic instead; add a separate named ECDLP channel and restricted joins.

The current scalar branch of verify_solution_artifact checks exact generator and
target equality, canonical scalar range and Q=[k]G without calling domain().
It can support this task without manufacturing a factor-base admission. Its
Group::read API currently receives CaseData including a base, but the scalar
check does not depend on that base's population. A later pure public+scalar
adapter may factor this branch out without changing its semantics. Do not bypass
central source/occurrence verification to make the new export succeed.

## Task and public mathematical input

Task schema: cyclic-subgroup-ecdlp/v1.
Question: given the explicit field, curve, prime cyclic subgroup generator G and
Q in <G>, propose the canonical k with 0 <= k < r and Q=[k]G. Generated current
instances have nonidentity G/Q, hence a checked answer is nonzero. Do not silently
extend to infinity or composite-order subgroups in this version.

The public input is exactly the following recursively typed allowlist:

- task: the fixed schema string above;
- field: canonical prime residue field (characteristic/modulus decimal strings,
  degree=1, canonical basis), or binary polynomial field (characteristic="2",
  degree, ordered irreducible low terms, canonical basis);
- curve: canonical model name, a and b in canonical decimal coordinate encoding;
- subgroup_order: certified prime r as a canonical decimal string;
- generator and target: canonical finite x/y decimal coordinate pairs.

No factor-base population/digest/actions, policy or policy budget, requested
size, m, solver, twist/family label, seed, instance index, constructor cost/status,
certificate, group-order factorization, trace or source path is a feature. The
actual twist is already represented by its mathematical coefficients. n for
existing split routing is derived from the admitted field representation and
checked against the verified source declaration; it need not be duplicated in
inputs. Canonical model/basis/field keys are derived or enforced, never copied
from arbitrary descriptive metadata. Ignore irrelevant optional family fields.

Domain-separated identity:
SHA256(ASCII("crypto.ml-dataset.cyclic-subgroup-ecdlp-features/v1") || NUL ||
       canonical compact JSON(inputs)).
Use the repository's deterministic Value serialization and the existing exact
integer/string conventions. Decimal field values and scalars must never pass
through floating-point representation. The versioned domain separates this task
from PDP even if an incidental JSON substructure matches.

Equivalent public G/Q/field/curve/r across different m, policies, budgets,
constructor outcomes or methods share an ECDLP feature_id. Changing G or Q changes
the full problem identity. Mathematically equivalent coordinates under an
unimplemented isomorphism are not automatically identified; no such equivalence
claim is necessary. A lower-level curve/subgroup-only identity can be designed
later if needed; do not fold it into this minimal increment.

## Admission, labels and honest failure semantics

Every source is first admitted by the complete existing sealed-cell verifier,
including original-byte plan/occurrence/protocol binding, source artifact
hashes, roles and replay obligations. Censored or failed cells can still have
valid public ECDLP inputs; their constructor or solver outcome does not determine
whether the public mathematical problem exists.

Require curves::verify_public plus exact membership Q in <G>. The current public
verifier alone checks prime r, nonzero points and [r]G=[r]Q=O; that is insufficient
to identify <G> when rational r-torsion has multiple cyclic subgroups. Establish
membership by one of:

1. Existing checked private construction witness replay. This is restricted
   source admission, not independent scalar recovery supervision.
2. A checked canonical scalar-recovery certificate for this exact G/Q/r.

Do not launch a new DLP/membership search from the exporter. With neither proof,
retain an explicit restricted unavailable reason and no ECDLP feature. A bounded
constructor failure with a valid checked synthetic witness remains admissible.
A setup failure without public inputs remains unavailable. An unsealed attempt
is recorded as such and is not promoted to a verified public training example.

Emit a restricted ECDLP occurrence for every observed source occurrence,
including unavailable/unsealed cases. For admitted instances, retain a scalar
label only after checking the certificate's canonical range, exact G/Q match
and Q=[k]G. Enforce a single unambiguous certificate or explicitly identical
canonical values; reject conflicting or duplicate role/path aliasing rather
than silently selecting the last scalar. Prefer rejecting duplicate scalar-role
artifacts in this initial profile.

Represent these concepts separately:

- instance_admission: public facts + exact target membership;
- scalar_label_verification: exact equation/range/custody, or not_present;
- recovery_execution_provenance: separately retained native source/capture and
  claimed method; an equation certificate alone does not prove how k was found;
- recorded_attempt_outcome: actual native outcome/status and resource censoring.

Do not promote the producer's read_private_witness=false field into independent
execution proof. Preserve existing checked no-private-input admission/provenance
where available, name its precise scope, and leave unknown scopes unknown. A
known synthetic scalar can admit the instance and serve as a restricted reference
for replay, but is not automatically emitted as an independently recovered label.

For no checked recovery: recovered_scalar=null, typed attempt outcome preserved.
A cap/timeout/unsupported/PDP-no-decomposition is not evidence of no ECDLP solution.
Every admitted ECDLP input has a membership witness; censored/failed rows must
not be trained as an UNSAT/no-solution class. BSGS success with empty unused FB
keeps source_population_admitted=false and source_domain_digest=null in restricted
provenance while its independent ECDLP row and checked scalar label are available.
PDP feature_id remains null. Other methods may link to the ECDLP problem even
when they produced only a PDP result; only an actual scalar certificate supplies
a recovery label.

## Files, joins, counts and compatibility

Add ecdlp-features.jsonl (model input) and restricted/ecdlp-records.jsonl (labels,
occurrences and provenance). Keep existing PDP files and their schemas/identities
unchanged. Each restricted record joins source_target_id to the existing sealed
restricted target, and independently contains ecdlp_feature_id or null. Do not
reuse pdp_domain or overwrite its feature_id. Unsealed records join existing
source export-index record_id, with target_id=null explicitly.

Public row schema: ml-dataset.ecdlp-features/v1 with feature_id, split_component,
split and inputs. Restricted row schema: ml-dataset.restricted-ecdlp-record/v1
with record_id, source_target_id/source_index_record_id, ecdlp_feature_id,
instance admission, recorded outcome, checked recovery label/reference,
execution-provenance scope, policy occurrence reference and unavailable reason.
Source artifact references remain restricted. Shared model IDs never erase
separate policy/solver attempts or multiply them into independent examples.

Introduce an explicitly versioned/export-option channel so old V1 default output
bytes and existing policy proposal02 artifacts are unchanged. Suggested API:
ExportOptions { cyclic_subgroup_ecdlp: bool }, default false. Expose an explicit
CLI option, and enable it in the next preregistered corpus/export definition.
An enabled manifest must use a new declared version (v3, subject to coordination)
and updated allowlist/model-input-files list; never silently append files to an
already frozen v1/v2 artifact identity. Keep immutable retries option/version
bound and reject incompatible reuse of an output directory.

Manifest exact counts: observed ECDLP occurrences, admitted feature occurrences,
unique ECDLP feature rows, checked recovery-label occurrences, membership-only
occurrences, unavailable/unsealed occurrences and typed reasons. These are not
counts of novel solutions, unique keys, independent discoveries, or completed
ECDLP pipelines. Manifest file hashes/counts/joins include both new files. Report
summary may add explicitly scoped channel coverage; constructor status remains
separate from scalar recovery status.

Reuse bounded external sorting/row limits, deterministic feature hashing, source
lease and immutable installation. No corpus-sized map or extra solver execution.
The view has constant-sized public math per instance (binary modulus terms bounded
by admitted degree); restricted records reference large proofs instead of copying
them. Source artifacts are not modified. Preserve raw native costs with scope;
unknown whole-worker instruction counts remain null, with no group-op conversion.

## Splits and learning interpretation

Use existing whole-field-size split assignment for all channels. The same problem
must receive the same split as its PDP/SAT/algebraic occurrences; methods, policies,
generators, keys and twists of one size remain together. Do not introduce random
row splits or success-only inclusion. Preserve the existing limitation that
cross-size algebraic links have not been merged. Future new key/generator
sampling experiments require their own frozen population/evaluation contract.

The model can learn a public-input-to-checked-scalar hypothesis, or predict typed
recovery/censoring behavior when a separately explicit algorithm-and-budget
conditioning task is designed. This minimal mathematical task deliberately has
no algorithm/budget features; mixed-method outcomes are restricted observations,
not counterfactual cost or optimal-method labels. It does not establish a faster
algorithm or generalization outside tested curves from a training fit.

Policy-selection remains a distinct pre-construction task. It may receive public
curve/subgroup/target, permitted candidate policies and allowed resources fixed
before construction. It must not receive the future chosen population/domain
hash/orbits or construction outcomes. Current post-construction PDP views remain
valid for their declared task and are never relabeled as policy-selection data.

## Required authored controls before implementation acceptance

1. Checked BSGS scalar with empty capped unused FB: exactly one admitted ECDLP
   occurrence/checked label, no PDP feature, no implied FB admission.
2. Same G/Q under different policies/m/budgets/constructors -> same ECDLP ID,
   separate occurrence joins; changing G/Q/field/curve/r changes or rejects it.
3. Recursive unknown sentinel fields and known-but-irrelevant family fields,
   unverified symmetry flags and policy/history/outcome/cost changes do not
   enter inputs or identity.
4. Valid membership-only synthetic witness with censored/no-recovery outcome ->
   admitted ECDLP input, null recovery label; setup failure/unsealed -> explicit
   unavailable row. No negative-existence class.
5. Fully rehashed wrong scalar/G/Q/r, noncanonical numeric strings, duplicate
   recovery roles and wrong occurrence links reject. Public [r]-torsion without
   exact <G> membership evidence is not sufficient for admission.
6. Full feature/restricted manifest joins/counts/hashes, existing size splits and
   exact unchanged V1/default/PDP output bytes; complete immutable enabled retry,
   source hash readback and incompatible output option/version rejection.
7. Actual native CLI fixture after integration with recovered/censored/unavailable
   cases. Software tests are not dataset cells, speedups or whole-cost evidence.

No tests are authored or executed in this design-only amendment. Implementation
needs root approval of schema/option/version and coordination with held CLI and
solver files. Review this separately; do not modify immutable proposal02.

## Sources and novelty boundary

Source inventory binds exact inspected exporter, scalar verifier/BSGS, public
curve verifier, central sealed-cell verifier and existing corpus design/contract.
Key implementation references: export feature/target loop; solvers.rs
verify_solution_artifact scalar branch and bsgs; curves.rs verify_public/verify;
mod.rs verify_cell replay gates. This reuses existing group replay and dataset
custody; no new cryptanalytic method or empirical comparison is proposed, so
no literature novelty claim is made.
