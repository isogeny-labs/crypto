# Planted-positive PDP control contract v1

Status: **design contract only**. No native `crypto dataset` mode, corpus cells,
or measured result is claimed by this file. This contract extends the
[dataset design](../../docs/ml-dataset/DESIGN.md) with a falsifiable positive
control that can be implemented without changing the existing natural-target
population. No trials may start until an `experiment.protocol/v1` binds a
specific plan, constructor binary, limits, population counts, and analysis.
The [native integration handoff](INTEGRATION.md) locates the reserved planner
touchpoints and lists the source-level merge gates.

## Purpose and denominator

The current `plan/v1` constructor samples a nonzero scalar uniformly and sets
`Q=[scalar]G` before building a small-x factor base. Those cells remain
`natural-v1`: their plan bytes, index order, generator and key RNG streams,
factor-base policy, public instance, and restricted witness are unchanged.
The new control is `planted-pdp-v1`: it creates exactly one known length-`m`
signed factor-base decomposition before any solver runs. It is a separate
population and factor-base policy, never silently mixed into natural-cell
success rates or treated as an independently recovered ECDLP scalar.

Introduce `crypto.ml-dataset.plan/v2` with required, nonempty, unique
`target_populations`, each member `natural-v1` or `planted-pdp-v1`. Keep
`plan/v1` decoding and `DatasetPlan::binding_sha256` byte semantics unchanged.
The v2 cell index includes an explicit population digit after the key digit;
the remaining axes retain their v1 order. `cell_count` multiplies by the
number of populations with checked overflow. `crypto dataset plan
--target-populations natural-v1,planted-pdp-v1` writes v2; omitting the option
writes the exact v1 plan format and behavior. Reject duplicate/unknown
populations before any cell starts. A worker config carries a required v2
population; old v1 config decoding remains unchanged. This avoids altering
the hash or replay of every frozen v1 campaign.

For each frozen plan, report the full Cartesian denominator by population,
family, n, m, twist, curve seed, generator and key. Keep construction errors,
zero-sum rejections, incomplete factor bases, resource caps, native timeouts,
and solver failures in their attempted cells. Do not resample an entire cell
after failure or export only successfully constructed positives. Report both
requested cells and achieved positive witnesses, with failure reasons.

## Construction and exact witness

The general-size planted control needs known factor-base logarithms. Use a
distinct `planted-known-log-subgroup/v1` factor-base policy: after the normal
curve, order, prime subgroup and generator are certified, draw independent
nonzero `b_i` in `1..r` from a domain-separated deterministic RNG and form
`B_i=[b_i]G`. Retain the public points in canonical encoding, with no `b_i`
or construction RNG state in any public artifact. Deduplicate points and
negatives, charge every rejected draw, and stop at the frozen point-attempt
cap. If fewer than the requested points remain, seal a construction failure.
This policy deliberately differs from the natural small-x factor base and
must be reported as a separate control. A small-x planted variant may be
added later only if every selected base logarithm is independently certified
within a frozen budget; do not infer a log from subgroup membership.

Draw exactly `m` `(index, sign)` terms with replacement from the completed
base, using a second domain-separated RNG keyed by the plan seed, curve seed,
generator index, key index, twist, family, n, and m. No solver outcome enters
the draw. Let `Q=Σ sign_j B[index_j]` and
`s=(Σ sign_j b[index_j]) mod r`. Recompute both through independent reference
arithmetic. If `s=0` or `Q` is infinity, record a rejected construction draw
with its attempt number and reason, then draw again up to a fixed cap. The
accepted `s` lies in `1..r`; `Q=[s]G`, and the ordered signed terms constitute
a planted PDP witness. Repeated indices are allowed because `m` may exceed
the base size. This proves existence of one length-`m` decomposition, not
minimality, uniqueness, solver success, or scalar recovery by a solver.
The raw base-log draw has probability `1/(r-1)` per value; each raw signed
term has probability `1/(2B)` for realized base size `B`. Retain the exact
raw-draw probabilities and every rejection, including duplicate/negative
base points and zero-sum targets. Publish observed rejection rates with their
denominators; only publish a conditional accepted-target probability if an
exact finite-group calculation certifies it. Do not substitute an observed
frequency for that exact probability or silently condition on success.

The public instance retains the curve, field, subgroup, generator, target,
factor-base point set and ordinary mathematical symmetry data. A restricted
`planted-construction/v1` record retains the population, all `b_i`, ordered
selected indices and signs, accepted attempt number, all rejected draws and
reasons, derived scalar, seed-domain identifiers, exact group-sum replay,
source plan/binary digests and construction operation counters. The separate
private witness still carries the scalar for known-answer replay. Construct
the target before handing public inputs to a solver; never pass the restricted
record or private witness to solver APIs. Store it under a
`restricted-*` artifact role and include it in the cell manifest hash chain.

## Independent verification and causal export

The verifier must reconstruct the field and curve from strict canonical
public data, verify order and generator membership, verify every base point
equals `[b_i]G`, check base uniqueness, replay each signed addition to `Q`,
recompute `s` modulo `r`, and independently check `Q=[s]G`. It must check
indices in range, signs in `{+1,-1}`, exactly `m` terms, nonzero `s`, the
recorded draw/rejection sequence against the frozen RNG, and the declared
factor-base policy. Tampering with any point, sign, scalar, seed, or digest
must fail. The ordinary known-answer verifier continues to check the scalar;
this extra check certifies that it is derived from the planted relation.

The model input allowlist is a strict projection of public mathematics at
decision time. Neither `population`, `planted`, selected indices/signs,
`b_i`, scalar, RNG domain/seed, acceptance/rejection count, construction
cost, target provenance, witness-existence flag, nor solver outcome may enter
`features.jsonl`, `prefix-features.jsonl`, `algebraic-features.jsonl`,
feature IDs, or any nested public graph. These fields belong only in
restricted labels/provenance. The public target and factor base necessarily
encode the mathematical problem; audit the exporter recursively for forbidden
keys and compare input bytes after mutating only restricted construction
records. Keep split components at least as coarse as the current whole-size
holdout, so related curves/twists do not cross train/test. Publish separate
population counts and calibrate routing only within a stated mixture; the
control's constructed positivity is not a natural-target prevalence estimate.

The restricted label distinguishes
`constructed_pdp_witness_verified=true` from
`pdp_witness_found_by_solver` and
`scalar_recovered_without_witness`. A solver receives no planted proof and
must return its own exact signed relation for promotion. Exact replay of its
returned relation is necessary even if it matches the planted relation.
The planted scalar is a known answer, never an independent ECDLP solve.

## Source-level acceptance gates

Implementation should touch `DatasetPlan` and `PlanArgs` in
`src/ml_dataset/mod.rs`, the worker config in `src/ml_dataset/types.rs`,
construction and replay in `src/ml_dataset/curves.rs`, and the strict exporter
in `src/ml_dataset/export.rs`. `mod.rs` is currently reserved by Conductor
T-887; integrate that change with its owner rather than editing the reserved
file independently. No source change in this contract PR is runtime support.

1. **Compatibility:** Serialize a frozen v1 plan and several v1 cell configs
   before/after the patch; require byte-identical plan bindings, cell indexes,
   public inputs and known-answer witnesses. Omitted CLI flag writes v1.
2. **Determinism and denominator:** v2 plan round-trips through CLI and worker;
   each population's full Cartesian count and fixed index mapping match the
   frozen formula. Duplicate/unknown values and arithmetic overflow fail.
3. **Positive replay:** For prime, ordinary binary and Koblitz curves and both
   twists, including `m` greater than base size, construct a positive cell,
   replay its ordered relation and scalar with independent point arithmetic,
   and verify every manifest artifact digest. A bounded zero-sum fixture must
   preserve rejected draws and a capped failure without replacement.
4. **Tamper and isolation:** Corrupt a selected index, sign, base log, scalar,
   point, seed, and manifest digest one at a time; each fails exact replay.
   Instrument a mock solver to prove it receives only public inputs and base.
5. **Leakage:** Recursively inspect all three model-input files, nested graphs
   and feature IDs for construction or outcome fields. Alter only restricted
   data while keeping public mathematics fixed and require byte-identical
   model inputs. Restrict population metadata to restricted labels and reports.
6. **Censoring:** Force factor-base cap, zero-sum cap, solver timeout and
   interrupted upload; each remains in the denominator with its exact status
   and one-attempt receipt. An exact checker must not promote a constructed
   witness to a solver-found relation or an independent scalar solve.

Before first data, freeze the protocol with natural/planted arms, same
mathematical axes where possible, exact constructor source/binary hash,
base and draw caps, positive and negative controls, per-population sample
sizes, randomization, costs, censoring and analysis. Run no construction
trials under this contract alone.
