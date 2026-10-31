# Complete factor-base policy integration draft

Status: external source proposal only. No destination source writes, builds, test execution, dataset generation, cloud action, or Conductor scope mutation was performed. Rustfmt parsed every proposed module and `git apply --check` succeeded against the frozen external base. This is not a compiled or test-passing result.

Patch: `factor-base-integration.patch`
SHA256: `05518e629e387276685724471addd8b6edc0321660a0af9704946bcc7009879b`
Frozen source trees: `factor-base-integration-draft/{base,proposed}`.
Per-file original hashes: `factor-base-integration-draft/BASE-HASHES.json`.
Changed-file original/proposed hashes: `factor-base-integration-draft/CHANGED-FILES.json`.
The frozen destination inventory originated at d6f3865a9301cc852e8d32f2693f62e18b1a2b80. These hashes, rather than any later branch HEAD, are the patch base. Donor final 8736ed3eeaf66b86a9833d318eb824cb10273210; policy implementation edc27aa350f4c2a45e18778e09bc39f07d914c29 plus strict correction 2a53a76b5ee9a74e3fd894bfdf6d6e74750129fc. Donor budget, native root, and obsolete exporter changes were not adopted.

## Coordinated scopes

Seven primary files: types.rs, factor_bases.rs (new), curves.rs, mod.rs, solvers.rs, export.rs, report.rs, all under src/ml_dataset.
Three additional trusted-reader files: prime_f4_wide.rs, prime_f4_wide_m3.rs, koblitz_m3.rs. Each changes the original mathematical replay call to bind the factor-base recipe/version/sampling seed to its authenticated CaseConfig; each also has mechanical fixture fields. Koblitz positive conditioned fixture uses its actual config for replay.
Nine mechanical fixture-only files: binary_roots.rs, capture.rs, prime_chain_graph.rs, prime_f4_wide_dynamic.rs, relations.rs, relations_generic.rs, restore.rs, startup_gate.rs, telemetry.rs. Their only changes are explicit None or empty policy axes in existing struct literals.
No storage, execution, budgets, native engines, algebraic_targets, prime_chain_prefix, prefix_algebra, or koblitz_m3_causal production code changes.

## Integration behavior

- Strict tagged recipes: explicit legacy, sampled x projected control, binary subspace intersection, binary Frobenius-union intersection. Coordinates and subgroup membership are separated; the two binary policies do literal intersection, not projection. Unsupported family/recipe combinations retain explicit provenance and produce no model features.
- An absent recipe retains legacy CaseConfig serialization; an absent axis retains plan/v1 serialization, typed compact plan hash and existing cell ordering. Explicit legacy is a plan/v2 recipe with distinct identity. New plan axis is solver-fastest, then recipe, then twist and all existing axes. Empty/present, duplicate and malformed axes are refused.
- CLI accepts a JSON recipe array via --factor-base-policies. One plan integrates generation, original admission, solving, mathematical replay, report and export. This is not a detached construction API.
- Producer and verifier retain exact u64 prime cofactors from current destination. Policy replay uses independent arithmetic but shares policy enumeration logic; it is not an independent implementation of the whole construction algorithm.
- verify_and_store_worker_originals binds base to config inside its existing factor-base replay telemetry before minting a trusted token or writing original public/base files. Central verified-cell replay and native original reconstructors use the same binding. Mathematical APIs that have no config remain mathematical-only and are not misrepresented as config authenticators.
- All current budget/accounting, original-token, native dispatch, root receipt and closure gates remain. Unsupported recipe handling occurs before Group::read/domain construction.
- Legacy exports keep the current v6 three-channel manifest. Policy plan/v2 exports use v7. Only allowlisted public recipe fields enter features. Derived sampling seeds, construction candidates/counters/status, membership logs and private answers stay restricted.
- source_factor_base_policy preserves recipe identity when PKM or large-prime projections replace factor_base. Prime prefix and algebraic feature initial inputs inherit this allowlisted field through existing paths; their applicability bounds are unchanged.

## Controls proposed, not run

Donor controls retained: strict recipe serde/bounds/linear independence; every family and both twists; explicit legacy domain equality; unchanged public problem/scalar; candidate/recipe/counter/status replay mutations; subgroup membership and orbit behavior; valid-but-wrong policy/seed/version rejection; 720-cell legacy plan byte/hash/mapping; version/axis ambiguity; source export/report restricted construction labels and unsupported no-feature records. The historical pilot plan is embedded with its exact bytes and hashes in the test, because its tracked research file is absent from the sparse destination checkout.

Additional controls:
- A fresh original-bound worker fixture freezes the selected plan/v2 recipe before input generation. A self-consistent replacement recipe or derived sampling seed passes mathematical replay but must fail trusted-token admission before original public/base writes. Explicit legacy policy supplies the positive token case.
- Real PKM and double-large-prime exports must retain source recipe identity after domain replacement and omit construction/private fields.
- A real bounded prime F4 export checks recipe identity in every retained static, prefix and algebraic row and excludes restricted construction fields. This test explicitly does not require the bounded fixture to retain an algebraic consequence, so it cannot establish non-vacuous algebraic coverage by itself. The existing channel-specific replay tests remain unchanged.
- Existing sentinel projection control now covers versioned public recipes; unknown recipe fields are rejected rather than exported.

## Semantic merge and validation gates

Parent owns incoming causal/signed changes. Apply by semantic merge against the recorded base hashes, not by replacing destination files. Preserve new signed-domain exporter work, causal source, capabilities metadata, exact native single-serialization route, producer receipt accounting and root acknowledgements. Mechanical fixture None fields may need adding to additional current struct literals absent from the frozen snapshot.

Current parent HEAD 4df2357 adds project_koblitz_m3_prefixes and its dedicated causal module. This draft does not wire that exporter. Its static_inputs currently reconstructs mathematical data without a policy field. When wired, the Koblitz causal feature identity must explicitly include the authenticated source recipe; do not silently use a recipe-blind identity. This is an identified future integration gate, not a claim that policy-aware Koblitz causal export is already present.

Before acceptance: run current-head compile and focused policy/config/worker/export/report tests, including all family/twist semantics and genuine nonempty channel fixtures where needed. Then run existing legacy/native regression suites. No spend, workload, cloud schedule, stopping rule or admission limits are changed by this patch.

Integration: applied cleanly as a contextual patch after native4df validation was terminal and ordinary-binary component011818bb was committed. Existing source was not replaced wholesale. T1153 owns remaining policy surfaces; existing root capture/native/dispatcher reservations own their edits. All four policy paths are integrated in source; compilation, tests, real CLI, export and restore validation remain pending. Native Koblitz causal exporter is not wired and policy identity remains its explicit next gate.
