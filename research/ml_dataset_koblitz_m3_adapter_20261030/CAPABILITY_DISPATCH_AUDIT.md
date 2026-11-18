# Capability metadata versus dispatcher — external audit

Read-only audit against current solvers.rs and research/ml_dataset_koblitz_m3_adapter_20261030/POST_M3_ROADMAP.md. No source edits, builds, datasets or jobs. External capability-audit.patch SHA256dc23af7b6a2b487a74a267eb87dffa7a6003abc1714268ddc0ae722e068672bd passed rustfmt generation and git apply --check. Source base/proposed snapshots saved as capability-audit.{base,proposed}.rs. Controls are drafted, not executed.

## Koblitz

Runtime solve_with_verified_worker_originals first authenticates original token/config/case/root/binding. koblitz_worker_profile then selects only familykoblitz,m3,n23..31 or47..61,solverf4/f5,factor_base_size4,degree1..6,positivework. It requires four distinct signed affine-coordinate entries in range, negation closure and exactly two distinct x coordinates. Original execution context is mandatory; caller constructs bound capture before native entry. Neither generic solve nor ordinary binarym3 receives this route. Both twists remain eligible, subject to original mathematical verification.

Existing capabilities omit that extension. Patch reuses/refines the saved capability draft's accurate profile as one shared JSON object nested under f4 andf5. Existing base m/m_by_family ranges remain unchanged; m_scope explicitly distinguishes base routes from separately gated supervised extensions. No broadening to binarym3, other arities/solverids, larger factor bases, arbitrary n or absent context. The profile describes eligibility, not guaranteed completion or completed wide validation.

The metadata retains missing independent matrix derivation, hardware instructions and exact fall-degree certificates. No generic incremental F5 claim is added.

## Prime wide dynamic

Existing separate f4-wide-dynamic-v1 metadata already matches main runtime gates: prime,n33..61,m4..15,degree12,original context/bound capture,maximum64signedpoints/32xcodes. Dynamic_original additionally requires positivework/memory/trace, original public/config/domain consistency, affine valid prime-group points and nonempty domains. solve_dynamic_bound rejects a trace allowance not divisible by1MiB and expired deadline.

Patch makes positivework/memory/trace, whole-MiB trace and nonempty-domain lower bounds explicit. It clarifies this is an explicitly selected solver id, not automatic genericf4 dispatch. The f4 row's wide-prime gap now explicitly scopes m2/m3 to that id and points m4..15 to the separate profile. Generic primef4's existing small-prime chain eligibility is unchanged; this audit does not claim general negative PDP completeness for exceptional affine-chain branches.

## Controls

Metadata assertions preserve base ranges and exact supervised restrictions. Shape-only config tests compare stated Koblitz n/m/degree/family ranges with actual profile selector using explicitly synthetic signed coordinates; they do not claim a valid curve fixture, original-input authentication or cryptographic execution. Existing original-worker fixtures test those separate runtime gates.

Dynamic tests compare its family/n/m/degree/work metadata with dynamic_supported, and assert the deeper mandatory context/domain/trace flags. No control changes workloads, dispatch logic or original budgets.

# Capability metadata independent review

Reviewed external `capability-audit.patch`, SHA256 `dc23af7b6a2b487a74a267eb87dffa7a6003abc1714268ddc0ae722e068672bd`, and `capability-dispatch-audit.md` against the actual solvers.rs dispatcher and dynamic admission functions. No source changes, compilation, tests, CLI commands or cloud operations performed. Current immutable build remains the parent's separate validation lane.

**No concrete blocker found.** This patch changes capability metadata and adds tests; it does not alter dispatch or admission.

Koblitz profile naming matches actual adapter/strategy `koblitz-m3-native-v1`. Placement under f4/f5 matches `koblitz_worker_profile` and `solve_with_verified_worker_originals`: Koblitz only, m3, n23–31 or47–61 inclusive, degree1–6, requested factor-base size4, four distinct signed points, sign closure, two distinct x codes and positive work. Original-token matching precedes profile selection; context is mandatory and a bound capture emitter is installed before native entry. Both twists are eligible under the same shape predicate and remain subject to mathematical verification. No native m3 extension is advertised for binary, prime, other solvers or larger factor bases.

Existing base m/m_by_family ranges remain unchanged and the added m_scope explicitly identifies them as base routes, with supervised profiles separate. This preserves compatibility without silently broadening legacy eligibility. The Koblitz profile explicitly describes dispatch eligibility subject to original resource/evidence gates, not a promise of completion or evidence that every listed field degree was executed. Missing basis derivation, retired-instruction and fall-degree assurances remain explicit.

The dynamic profile keeps the actual separate id `f4-wide-dynamic-v1`, familyprime,n33–61,m4–15 and exact degree12. `dynamic_supported` requires positive work; `dynamic_original` independently requires positive memory/trace and nonempty signed/code domains within64/32; `solve_dynamic_bound` requires whole-MiB trace, bound capture and a live original deadline. The added metadata flags and domain minima match these gates. The f4 gap clarification correctly distinguishes generic f4 wide m2/m3 routes from the explicit dynamic m4–15 id. No automatic routing or completion claim is introduced.

The Koblitz test coordinates are deliberately synthetic and sign-closed, exercising only selector shape. Comments and audit notes identify this correctly; the test does not claim a valid curve, verified originals or cryptographic execution. Dynamic controls test the actual eligibility predicate and metadata assertions, while deeper runtime requirements remain separately tested. The audit clearly labels all added controls as drafted/unexecuted. Existing native tests and the eventual exact-source CLI capability output remain necessary validation; this review does not advance those gates.

Integration: metadata and controls applied after d6f3865 validation became terminal. Runtime dispatch unchanged. New controls pending execution.
