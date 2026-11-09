# Retained factor-base policy integration plan

Read-only assessment, 2026-10-29. No repository files changed, tests/builds run,
new draws taken or cloud actions performed. Destination reviewed:
`/home/ec2-user/crypto-ml-root-cofactor-integration-20261029`, source
`a0619090bd99b18e828bc16b4b508ca7dafd58e0`. Source validation remains parent-owned.

## Donor and original scope

Donor worktree `/home/ec2-user/crypto-ml-policies-20261028`, head
`8736ed3eeaf66b86a9833d318eb824cb10273210`, tested runtime
`d71ec2ef5ab9b7f2a038f61c941af10297a775ad`. Main implementation commit
`edc27aa350f4c2a45e18778e09bc39f07d914c29`; strict-recipe correction
`2a53a76b5ee9a74e3fd894bfdf6d6e74750129fc` must also be preserved. Read donor
docs/ml-dataset/FACTOR-BASE-POLICIES.md, research/ml_dataset_policies_20261028/
VALIDATION.md, source changes and current destination admission/export code.

Implemented donor policies are Legacy {}, SampledXProjectedControl,
BinarySubspaceIntersection and BinaryFrobeniusUnionIntersection. All four must
survive integration; no policy is silently substituted for another. The sampled
control is a projected sampled-x image, not uniform subgroup sampling. Literal
subspace domains intersect the selected cyclic subgroup without projection;
Frobenius unions use the model-preserving recorded power and whole sign/orbit
units. Recipe dimensions <=12 and candidate_limit <=65536 are explicit caps.
Binary policies on prime fields yield unsupported records. Exact subfields can
be represented only when their supplied basis actually defines that subfield;
the donor does not establish a general automatic subfield/trace-zero/novel-base
catalogue, lower regularity, or any first/last-fall theorem. Those broader user
requirements remain open alongside m2..15, n23..31/47..61, twists and models.

Historical software evidence: 5169 full-library passes, zero failures,123 ignored;
153 dataset passes;72 software cells audited separately with22 positive PDP
certificates. This is historical donor evidence, not combined-source validation
or cloud corpus coverage. The independent consumer policy view and producer
view were not proved identical.

## Exact integration surfaces

- `src/ml_dataset/types.rs`: strict tagged FactorBasePolicy enum plus validation;
  optional CaseConfig.factor_base_policy with serde default/skip-if-None.
- New `src/ml_dataset/factor_bases.rs`: retained constructor, independent
  reference reconstruction, sampled control seed, cyclic membership, actions,
  whole-unit caps and tests. Copy final donor version, not initial enum match.
- `src/ml_dataset/curves.rs`: expose only required existing arithmetic helpers;
  add reference-mode CyclicMembership construction/logarithm; dispatch explicit
  policy construction after unchanged curve/generator/target selection; add
  config-aware replay. Preserve the exact-cofactor context and original no-policy
  RNG/public output path. Do not wholesale replace the older donor curves.rs.
- `src/ml_dataset/mod.rs`: module registration; CLI --factor-base-policies JSON
  recipe array; plan-v2 policy axis validation/count/order; config-aware worker
  and sealed-cell replay. Keep current budget/startup and both corrected root
  auxiliary/event-cap verifiers. The old donor root code is already superseded.
- `src/ml_dataset/solvers.rs`: fail closed for explicit unsupported construction
  before native dispatch; only policy-specific guards from donor. Preserve
  ExactPrimeCurve, prime-chain and binary/prime root implementations.
- `src/ml_dataset/report.rs`: preserve donor policy/construction coverage axes,
  separating actual points, complete/capped/unsupported populations and costs.
- `src/ml_dataset/export.rs`: owner-coordinated public recipe allowlist, v2
  source schema admission, restricted construction labels, unsupported/null
  feature path, policy-aware feature identity and tests. Merge with current
  graph/prefix routing; do not overwrite it with older donor export.rs.
- Mechanical CaseConfig/Plan literal additions are needed in current tests in
  curves,solvers,binary_roots,mod,export,prime_chain_graph and any additional
  enumerated consumers. The donor also adjusts live,restore,telemetry,relations,
  relations_generic and integration tests; inspect which paths exist in current
  main before transplanting. No blanket donor-tree replacement.

## Schema and identity obligations

No option means byte-compatible legacy plan/v1 and config serialization. Explicit
nonempty recipes mean plan/v2; explicit Legacy {} is distinct from omission.
The policy axis is after solver and before twist. Existing v1 cell ordering must
remain unchanged. Present empty axes, duplicate/unknown recipes, excess counts,
dependent/out-of-field bases fail admission. Read strict2a53 correction.

Preserve current DatasetPlan::binding_sha256 EXACTLY: compact serialization of
the typed struct in field order, not generic sorted/canonical JSON and not saved
file bytes. Add skipped empty policies at the donor-compatible position and test
old plan hashes/config bytes. Do not replace current grant/journal bindings with
donor-era direct digest plumbing. Existing720-cell pilot and frozen outputs must
not be rewritten, reindexed or amended by enabling a separate v2 plan.

Binary certificates bind the complete serialized config and base/public objects.
Omitted policy must serialize identically and preserve old binding; new explicit
policies intentionally bind to new identities. Prime-v1 root certificates bind
mathematics/config fields only; central config-aware factor-base replay must
enforce recipe/domain identity rather than claiming binary-style full-object
binding. Preserve auxiliary byte/call/state checks and test policy substitution
on independently rehashed records. Exact cofactors must remain exact through
all projected-control/reference-membership operations; do not reintroduce old
CurveParams.h narrowing through donor imports or restore fake cofactors.

## Ownership and safe staged delivery

Fresh Conductor read shows T828 owns export.rs,prime_chain_prefix.rs,
prefix_algebra.rs,docs/ml-dataset and research/ml_dataset_20261028. T840 owns
curves.rs; root has T803/T805/T836 integration scopes. This inventory grants no
new ownership and sends no message to the other chat.

Stage1 while exporter remains owned: finish this design and retain exact donor
patch inventory; prepare source-bound compatibility cases/design off the active
repository. Do not activate a half-integrated plan-v2 as completed functionality.
Adding public struct fields already requires mechanical updates to export.rs
test literals, so a full-library-compilable schema patch cannot simply avoid
that owner. A standalone extracted API could be staged, but it would be partial
preparation and is not the requested CLI capability; do not build a duplicate
side implementation merely to bypass ownership.

Stage2 after coordinated scope availability: integrate donor types/constructor/
CLI/replay/report together, preserving all four policies. Obtain the export
owner's changes for mechanical fixtures and an explicit v2 publication boundary.
Current feature_projection rejects v2 source schemas, which is fail-closed;
do not weaken that to generic feature fallback. An interim publication rejection
must remain explicitly incomplete until full policy export joins the branch.

Stage3: merge full recipe allowlist and unsupported-provenance behavior into
current static graph/causal-prefix exporter; retain exact alternative PKM and
large-prime domains. Review interactions with prime_f4_chain_input replay, graph
point/term caps and signed columns. Root certificate reference lists are another
owner-coordinated change, not permission to edit export.rs now.

Stage4: combined-source library/lint checks; legacy byte/digest/regression replay;
all-policy matched public-instance tests across families/twists, repeated-prime
membership controls and unsupported prime binary-policy controls; exact-cofactor
control; root recipe tamper replay; policy/graph/PKM/large-prime export leakage
and unsupported-path controls. Freeze finite actual CLI software fixtures before
draws and retain failures plus S3 readback/replay. Preserve source/protocol/binary
identities. No old test count substitutes for these new integration checks.

No scientific generation is authorized by this design. The full requested corpus,
native-solver coverage, instruction labels, formal fall-degree evidence and model
training remain the objective; worker IAM, cumulative admission and original
$200 total ceiling/deadline remain unchanged.
