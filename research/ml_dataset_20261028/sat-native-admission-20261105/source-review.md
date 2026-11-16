# Independent SAT native admission composite source review

Verdict: isolated compilation and bounded native controls may proceed after composing the separately reviewed XOR test correction. This is NOT permission to remove the current whole-path fail-closed gates or enable dataset production. No build, native test, cloud call or main edit occurred in this review.

Reviewed composite manifest 6ec306d4be98491ca5aa6cca79715b80dfd18e007e4d361bdfe14e83df12dea0; patch 32e7ba0650ebc6dd6958c179f9610c85116ce0bac583b4f83b08ed4b8f569df1; base 1ca1241f1daaba98c61524d7df9cec1b8506dec081c429f923532bd998f6511f; proposed dd4c3675e94647f9f1fa1a1f3256340421cefd902697e8a18ee66f97b720c257. Exact source hashes and external apply-check verified. Source syntax is not Rust borrow/type checking; that remains a native compile gate.

## Concrete correction before tests

Original actual_xor_rebuild_word_refusal test rejects the first XorWord call. Rebuild now charges before first row copy, leaving matrix/rhs empty, while the stale test expects two rhs rows and nonempty matrix. The immutable correction sat-native-xor-retention-test-correction-20261105 (manifest 9d1f5e6a3266f6ad90cf06f34d09567e3f14706444fb318765a3fd1ae11aff3c) is accepted: refuse the second actual word chunk and require matrix [0b011], rhs [true], empty pivots, sticky Work and public solve Unknown. Its base exactly matches composite proposed bytes; external apply-check passed. This preserves meaningful partial-state coverage rather than weakening the assertion.

## Admission findings

native_pay checks sticky refusal and deadline before callback, charges before the operation, then checks deadline again after callback. native_reserve computes replacement capacity/bytes with checked arithmetic, prepays before try_reserve_exact and checks deadline after allocation. Requested replacement capacity is a charging metric, not a physical allocator/RSS bound. Existing no-hook behavior skips admission callbacks. Trail, watcher, clause, analysis, reduction and matrix growth paths are admitted before their corresponding growth, with reused buffers retained across many refusal paths.

XOR reason and matrix operations have bounded word/chunk checkpoints; partial matrix/rhs/source state remains owned. Analysis retains learnt and seen/redundancy scratch; sort uses stable descending clause-length merge with retained scratch. Event payload copies are prepaid before publication without replacing the original observer. Public solve/root propagation distinguish sticky refusal from SAT/UNSAT and perform final deadline checks before returning an authoritative result. The comparison control checks actual legacy answer/model/word_ops for a real nonroot conflict-free CNF case.

## Restrictions that must remain

1. Keep native_entry rejection of installed event observer, XOR formulas, lazy theory and preexisting nonroot trail; keep the admitted nonroot-conflict rejection before analysis. Private-operation tests exercise instrumentation beneath these gates but do not establish complete observed binary search. A separate reviewed opening is necessary.

2. Preserve partial-clause ownership before opening conflict search. analyze can set native_pending_clause to a partial actual clause and return an empty sentinel on refusal. The caller's unconditional native_pending_clause=Some(learnt) then overwrites it with empty. This is currently behind the admitted-conflict gate; it must be corrected and fault-tested before that gate is removed.

3. Pin the original deadline in the owning admission closure or immutable owner state. Solver.deadline remains publicly mutable and this patch reads its current value; it does not freeze the installation-time deadline against later relaxation. Tests may tighten it, but production ownership must prohibit extending/removing the original deadline. Every accepted model/publication must check that same original deadline.

4. Admitted owner must not expose legacy model(), phase/priority mutation helpers or other unmetered public paths after installing admission. model() remains an immutable uncharged copy; admitted_model() is a prepaid copy of actual assignments, not a proof that solve returned Sat. Require a successful admitted solve, original formula/group checks and the original observer/custody before using it. Arbitrary lazy callbacks remain unsupported.

5. Do not claim hard real-time preemption. Allocator calls and several prepaid slice/event copies remain atomic; original-deadline checks prevent later authoritative success but cannot interrupt their interior. Budget accounting is conservative abstract work/requested replacement bytes, not exact instruction counts or peak memory.

6. Native retained-failure semantics are sticky stop states, not resumable valid solver checkpoints. Mid-reduction/watch/backjump state can be partially mutated; no retry or reset may clear that refusal and reinterpret the state as completed work. Original paid ledgers and partial state must remain attached to the owner.

## Required native gates

Use an isolated reviewed checkout/target with exact source/toolchain/input fingerprints and finite build/test limits. Run all twelve new native controls including the corrected XOR test, plus existing SAT CNF/XOR randomized/reference checks, observer fidelity/failure controls, UNSAT/model verification, checkpoint and legacy operation-counter regressions. Add a regression specifically retaining a partial pending clause through the caller before any conflict gate opening; add original-deadline relaxation prevention in the owning integration. Keep compile failures and native test failures as immutable evidence. A source review is not a passing native result.

Production integration still requires genuine both-twist nonroot SAT models from the actual original owner/installed observer, original signed-group relation validation, cumulative allowance, deadline and failure-publication gates. Positive finite/PDP evidence must retain its exact label; this composite does not enable full-span or scalar-recovery labels.
