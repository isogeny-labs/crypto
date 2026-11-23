# Actual-root dependency recovery: partial, not a 15x result

The maintained certificate producer and independent checker remain qualified, but actual crypto_lib integration is not complete. After parse recovery and materializing required include files, its library check still fails with109 diagnostics across unrelated APIs/types. No alternate baseline, disabled module, stub arithmetic or gate bypass substitutes for that build. Full output is retained in development-01/root-recovered-check-104.stderr; ROOT_BUILD_BLOCKERS_104.json indexes it. [pcc:unverified]

Changes proposed in the isolated recovery branch:

- Export the existing hashed-ecdl-tensor metrics module so existing callers resolve.
- Remove a duplicated ecbench match-arm prefix and restore a missing Koblitz test closure, matching primary-source structure.
- Reconstruct SAT from observer sourcef0863aefd6 and arena/indexed-XOR/telemetry source7fd907a84e using commonancestor7e271266f8. All six textual conflicts were reviewed. Both feature sets and the exact union of46 source tests remain. Initial propagation executes once; both indexed and dense row updates emit observer events; phase entry/return surrounds fail-fast observer outcomes.
- Use stable clause insertion IDs for trace keys across arena compaction. Every snapshot,reason,conflict,new clause,learned/deletion event maps current physical references consistently. The v1 event fields remain unchanged; consumers treat keys as opaque. The existing packed initial-event codec's optional contiguous-ID shortcut has a general explicit-ID path.
- Recover complete custody source93a5ab621c and the intended boolean negation from25b1ccdacf, preserving original run-identity checks and budget enforcement. This is parser/source recovery, not an independently qualified cloud-custody change. No cloud call was made.

Validation: the actual recovered SAT module passes all46 existing tests, including model/brute-force comparisons, counter equality, indexed/dense equivalence, observer replay, resume correctness and pinned search digests. The actual packed initial-event codec's existing canonical/roundtrip test also passes:47 tests total. Earlier45/46 and43/46 failures are retained; no expectations were relaxed. The focused harness imports the real source files and is explicitly not a full crypto_lib build. [pcc:unverified]

The full packed basis adapter remains in pending-actual-root-integration.patch and its36case test matrix is still unrun. After the small build repairs already applied here, apply only that patch's src/cryptanalysis/f4_fp/certified.rs portion; do not apply duplicated build repairs. Then run root-integration/tests only after the remaining library API/type errors are resolved.

Examples of remaining blockers: duplicate touch declarations, missing BigInstance/pair_claw_big/wiedemann_coord interfaces, absent module exports, exact-scalar conversion mismatches, and incompatible encoding APIs. They span the repository's ecbench/dataset/transfer code. Reconstructing their intended combined behavior requires repository-wide source recovery beyond these reviewed prerequisites; guessing replacements would weaken the requested validation.

The15x objective is active and unachieved. Numerical96 results remain unchanged at0.9960–1.0489 against strongest compact controls,inside A/A spread. These dependency repairs are not speedup evidence. Reports and archived data remain in their frozen source contexts. The separate benchmark/PCC historical metadata blockers were not changed. [pcc:unverified]

The metrics module's existing scoped-observer test also passes. Numeric observer clause IDs now differ from old physical-offset values; immutable old trace artifacts are unchanged. Existing logical replay and packed-codec tests pass, but this is not byte-equivalence or throughput evidence for trace consumers.
