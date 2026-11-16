# Complete factor-base policy integration — reconciled patch plan

2026-10-31, external design only. Destination snapshot d6f3865a9301cc852e8d32f2693f62e18b1a2b80, worktree /home/ec2-user/crypto-ml-koblitz-m3-adapter-20261030. Donor remains 8736ed3eeaf66b86a9833d318eb824cb10273210 at /home/ec2-user/crypto-ml-policies-20261028. No worktree writes, builds, cloud actions or new datasets. Exact file SHA256/length inventory is factor-base-current-source-inventory.json. This replaces stale integration assumptions in factor-base-integration-plan.md, preserving its scope.

## Reconciled evidence and ownership

Donor contains the final strict FactorBasePolicy enum including Legacy{} zero-field struct, not a unit variant; strict 2a53a76b5ee9a74e3fd894bfdf6d6e74750129fc changes are present. Donor HEAD has not advanced since retained plan. Its historical runtime/test/cloud obstruction evidence does not validate destination integration.

Current local Conductor default project is experiments (projectcrypto lookup404); references T828/T840 resolve there. Fresh JSON receipts factor-base-owner-T828.json and factor-base-owner-T840.json show both ready with EMPTY scopes, unlike the old note. Current full read saved factor-base-conductor-status.json shows relevant active territory:

* T887: solvers.rs,mod.rs,prime_f4_wide_dynamic.rs and dynamic research.
* T888: destination capture.rs.
* T907: binary_semaev_chain_sat.rs,sat_prefix.rs and SAT-prefix research.
* T960: case_binding.rs,execution.rs,startup_gate.rs.
* T1024: koblitz_m3.rs and its research directory.

No active export.rs/curves.rs scope was listed in this snapshot. That is a current read, not a new ownership claim or permission to override another session. Coordinate edits with the above active tasks; old T828/T840 ownership must not be cited as a present blocker. No messages or ownership mutations were sent through Conductor.

## Scope: one end-to-end feature, four policies

Preserve Legacy{}, SampledXProjectedControl, BinarySubspaceIntersection and BinaryFrobeniusUnionIntersection together across CLI, plan/config identity, generation, worker admission, central replay, report, static/causal/algebraic export and restored replay. A new enum or copied factor_bases.rs alone is not completed functionality.

Omitted policy keeps existing legacy bytes. ExplicitLegacy{} produces a distinct versioned policy identity. The sampled-x policy remains a projected sampled image, not uniform subgroup sampling. Subspace policies preserve literal x coordinates and intersect the selected cyclic subgroup; Frobenius policy admits whole recorded model-preserving sign/orbit units. Preserve1..12 independent nonzero in-field basis masks, candidate_limit1..65536, policy-axis1..32, existing point/domain caps and unsupported prime/binary-policy records. No automatic subfield/trace-zero catalogue or fall-degree conclusion is supplied.

## Exact source patch units

### A: strict schema, plan/CLI, unchanged legacy identity

1. types.rs: transplant strict donor enum/validate/name and its negative controls. Add optional CaseConfig.factor_base_policy immediately after factor_base_size, serde default+skipNone. No policy field onNone serialization.
2. mod.rs: register factor_bases module/import enum; add CLI --factor-base-policies recipe-array path; preserve all current CLI flags/current solver identifiers, includingdynamic. Append DatasetPlan.factor_base_policies after split_rule exactly as donor, default+strict nonempty deserializer+skipempty. Absent meansplan/v1; explicit nonempty meansplan/v2; presentempty/mixedschema/duplicates/unknownkeys reject. Validate every recipe against every n before generation.
3. Preserve DatasetPlan::binding_sha256 implementation verbatim: compact serde serialization of typed struct in field order. Do not replace with canonicalValue or prettyfilehash; do not touch grant/journal/budget hash plumbing. Factor axis enters cell_count withmax(1), and cell decoding after solver before twist. None path preserves every old index/config/hash. CLI reading recipes must occur before source-bound plan write and receipt hash.
4. MechanicalNone/empty additions to all current struct literals listed in inventory, plus repository-wide scan outside ml_dataset. No broad donor file replacements. Current inventory includes startup_gate.rs,capture.rs,koblitz_m3.rs,prime_f4_wide_dynamic.rs and both wide-prime adapters absent or different in older donor assumptions. Coordinate T960/T888/T1024/T887 because even mechanical test fixtures touch owned files.

### B: constructors and exact arithmetic

5. Add final donor factor_bases.rs with hashes pinned to inventory, preserving all four policies, sampling_seed, full ordered metadata and generic-reference arithmetic replay. Its build(public,policy,requested,seed,reference) returns versioned base and restricted membership logs; verify reconstructs exactly withreference=true. Shared enumeration logic means this is independently checked arithmetic, not an independently written algorithm/proof of its entire implementation.
6. curves.rs: expose only required Arithmetic/WPoint/helpers; extend CyclicMembership with new_with_reference and logarithm_with_reference, preserving oldnew/logarithm behavior. Keep destination ExactPrimeCurve and u64 cofactor throughout; never restore donor-era CurveParams.h narrowing. Reuse destination prime_projection and exact order logic.
7. In curves::generate, validate optional recipe, leave curve/generator/key draws and original RNG schedules unchanged; form same public instance, then explicit-policy branch callsfactor_bases::build using independent sampling_seed. ExistingNone small-x loop/orbit/candidates/private logs must retain byte-identical output and order. Avoid building unused legacy membership table forSome policy, but preserveNone exactly. Explicit versionedLegacy uses donor policy semantics and distinctv2 metadata.
8. curves::verify_factor_base dispatches only explicitv2 to policy verifier; retains strictv1 rule path. Add donor verify_factor_base_config binding requestedsize,n,m,family,twist, exactrecipe and derived controlseed to frozenCaseConfig before mathematical replay. Extra/changedrecipe keys reject through strict enum.

### C: trusted originals, solver/refusal, central replay

9. Critical new callsite: solvers::verify_and_store_worker_originals factor-base-replay closure must call verify_factor_base_config(config,...). Otherwise a mathematically valid different recipe/base can receive a trusted original token despite frozen-policy mismatch. Keep its existing one-time telemetry/write/hash ordering; no duplicate replay or refreshed budget.
10. mod.rs sealed-cell verifier currently callsverify_factor_base at its public/base read path; replace withconfig-aware version usingmanifest.config. curves::generate finalselfcheck likewiseconfig-aware. Direct original reconstruction in solvers::dynamic_original, prime_f4_wide original replay, prime_f4_wide_m3 original replay and koblitz_m3 original admission must bindconfig too; these paths can be reached through standalone validators and cannot rely on metadata flags.
11. Pre-dispatch explicit unsupported policy guard before Group::read/domain/native selection in solve_with_context, plus verified-worker dispatch behavior. Preserveunsupported construction/outcome/emptybase rather than treating it as no-decomposition or a malformed successful domain. No Koblitz profile widening: only exact existingFB4/two-x/degree/n/m/solver shape reaches that engine even when policy is explicit. Native work/clock budgets and all actual stored-root custody remain unchanged.
12. Generic relations APIs withoutCaseConfig continue verifying actual public/base math; do not synthesize a fakeconfig to claim recipe binding. Their CLI callers are protected by config-aware original/central gate. Preserve exact PKM/large-prime transformed domains and ordinaryroot certificate checks. Binaryroot full-object config hash naturally includesSome policy and keepsNone bytes; prime root mathematics-only binding still relies on centralconfig-aware replay. Do not remove strict auxiliary/state/call checks.
13. CoordinateT960 identity work before introducingplan/v2: plan schema version is not permission to replace case/execution/startup schemas. Confirm any unmerged versioned case-binding reader handles new typedconfig bytes and oldNone identity. Current destination hasstartup_gate/execution but not case_binding.rs; active task scope is material future integration.

### D: current report and all three export channels

14. report.rs: transplant only policy axes/hash, per-recipe coverage map and factor_base_construction measurement rows. Preserve current source/protocol/actual-root/othercoverage behavior. Separate requestedsize,actualpointcount,complete/capped/unsupported construction, sampling/context/counters and actual solvercompletion. No actualcofactor narrowing.
15. export.rsfeature_projection currently REJECTSfactor-base/v2 even though exporter advertisesstrict-structural-allowlist-v6. Add strict donor policy_projection allowlist and allowv2 only after centralverifiedsource/config-aware replay. Legacy input projection remainsbyte-identical. Add only publickind,basis,candidate_limit; never copy fullpolicy/base objects generically. Restrictedtarget records receiveconstructionstatus/completeness/counts/counters; seed/context/candidatehistory/logs remain excluded.
16. Unsupported v2 policies must branch to None inputs BEFORE PKM, large-prime or graph routing. Retain source provenance and restricted labels, null feature_id and explicit unsupported_factor_base_policy. Do not substitute empty features or generic fallback. Keep graph oversize and missing-input states distinct.
17. Preserve the v6 static, causal and algebraic sorter/replay code. Actual code passes the already allowlisted inputs to prime_chain_prefix::replay as initial; algebraic_targets uses node.initial in its feature object. Adding policy to standard public inputs therefore binds policy into causal/algebraic feature identities. No wholesale replacement or ad-hoc field is needed in those engines. Test this propagation and exclusion explicitly; immutable original graph/domain checks stay.
18. PKM and large-prime helpers replace factor_base with transformed definitions. Preserve the source recipe separately as public source_factor_base_policy, only for v2, so replacement cannot lose it. The transformed PDP domain remains authoritative. Include that public recipe in the feature hash; do not claim source points are transformed columns. Existing None/legacy features omit the field and remain identical. This is an additional destination obligation beyond copying donor static projection.
19. Make the policy-aware export contract version explicit. Proposed compatibility rule: legacy-only exports remain v6; exports containing policy-v2 sources advertise strict-structural-allowlist-v7 while retaining all three channels and exclusions. Mixed exports preserve legacy feature bytes/IDs but identify the new allowlist in their manifest. Do not rewrite old exports. Freeze the chosen version and consumer controls before data.
20. Current prefix replay accepts only small-prime rings p<2^32 and m3..15. Wide-dynamic and Koblitz routes do NOT automatically gain causal/algebraic exporters. Static policy export support must not broaden those engine gates. Preserve existing missing/unsupported trace feature statuses.

## Minimal coordinated files

Functional core: types.rs, new factor_bases.rs, curves.rs, mod.rs, solvers.rs, report.rs and export.rs. Original-reader propagation: koblitz_m3.rs, prime_f4_wide.rs and prime_f4_wide_m3.rs (dynamic original reconstruction is in solvers.rs).

Mechanical fixtures: binary_roots.rs, relations.rs, relations_generic.rs, telemetry.rs, prime_chain_graph.rs, prime_f4_wide_dynamic.rs, restore.rs, capture.rs, startup_gate.rs and core fixtures. The inventory records literal counts; scan integration tests outside this directory before applying.

Expected production edits to prime_chain_prefix.rs, prefix_algebra.rs and algebraic_targets.rs are zero if initial-input propagation proves adequate. Add owner-coordinated controls instead of rewriting their algorithms. Documentation includes the policy contract, coverage/validation notes and CLI help. Historical test counts do not substitute for combined-source checks.

## Validation and completion gates

* Preserve stored byte fixtures for None plan/config/public/base/witness, typed plan hash and old cell ordering. Explicit Legacy gets a distinct identity while preserving matched curve/generator/target.
* Reject extra, empty, duplicate, dependent, out-of-field and wrong-version recipes. Exercise all four policies across families and both twists in bounded software fixtures, retaining unsupported prime/binary-policy cases.
* Compare native/reference construction; verify recorded Frobenius power, whole sign/orbit units, sampling-seed binding, repeated-prime subgroup membership, cofactor greater than u32 and literal-x nonprojection.
* Reject rehashed recipe/seed/base/config substitutions at trusted-token and sealed replay boundaries. Preserve old binary-root bindings byte-for-byte. Prime math-only root checks must not bypass config-aware central replay.
* Static, causal and algebraic leakage sentinels cover constructor seeds, candidates, logs, status and counters. Distinct public recipes produce distinct feature identities even if point sets coincide. Legacy row bytes remain unchanged. PKM/large-prime transformed domains remain authoritative. Unsupported cases retain provenance and null features in every channel.
* Exercise actual CLI plan/preflight/generate/verify/status/export/report and restored replay for all four recipes under a frozen bounded software protocol after combined-source library/build/lint gates. Retain failures without resampling or rewriting existing campaigns. Scientific batches and S3 jobs remain separately root-authorized under the original total USD200 ceiling and deadline; this design launches nothing.

No partial source patch is presented as an end-to-end implementation. The current integration requires at least seven functional files, three additional trusted-reader sites, mechanical fixtures in owned scopes and an explicit extension to the v6 export contract. Assemble a coordinated whole-source patch after the active immutable-source gate using the inventory. Copying a standalone policy API would not fulfill the CLI/replay/export scope.
