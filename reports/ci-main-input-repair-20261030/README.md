# Repair current main CI inputs and integration regressions

The requested outcome is all PRs merged and current main/PR checks passing. This change repairs observed failures; it makes no new scientific result or performance claim.

Sources reviewed: `.github/workflows/{rust-lint,curve-covers,compact-orbit-n61-isolated}.yml`, existing `scripts/catalog_snapshot.mjs` and generator/verifier controls, `docs/CONDUCTOR.md`, and the original qualified parser/runtime source and tests. Existing generator and exact Git materialization methods are reused. Conductor is unavailable (127); the documented warn-and-continue fallback applies.

| Requirement | Actual result | Remaining validation |
| --- | --- | --- |
| Rust syntax/API integration | Dangling/interleaved blocks repaired with original complete test modules; created-versus-existing backend API and open-index initialization restored | Final current-main full library and touched CLI/lint |
| Compact isolated checkout | Add actual result_switches helper and existing ci-scope materializer in both checkout profiles | Hosted current-head check |
| Autolab regressions | Original 52 controls passed; actual plan/invalid-target command controls passed; no paid measurement launched | None within this local scope |
| Dashboard freshness | Unmodified generator write/check pass; only one dependency hash differs; HTML and evidence pins unchanged | Hosted current-head check |
| Cover assessment freshness | Unmodified generator write/check pass; exactly Cargo.toml/Cargo.lock/src/hash/mod.rs source SHA fields differ | Hosted current-head check |
| Assessment integrity | Existing 6 catalog tests, schema validation, and semantic verifier pass; zero semantic problems across all 15,791 frozen rows | None within this local scope |

The original catalog commit 728304718cb0ab6df70da620a908a5f76f175293 and formula commit 0c82a7dd1d1e6e3c847b49f8446d29d405beb0c5 remain frozen. All scientific fields and original figures remain unchanged. Original figures were recovered against canonical manifest SHA/length, without replacement or regeneration. No artifact policy exception is introduced.

Failed attempts are retained in workspace logs: sparse input omissions, invalid initial invocation, and ML rebuild disk exhaustion were failures, not successful checks. The related ML integration has 339 focused tests passing (one existing ignore) and all 30 prior full-run failures passing against a newly rebuilt binary; its full library is still running and those results do not qualify this different final main source.

The exact original PR2972 head3966c95fea3c87f3fdc6d13c07c882cd2a33d55c and PR2973 head53acdcd5fd369496b3b116dd7b40cda9b29a66f9 are both preserved as actual merge ancestors. The boundary controls for invalid field degrees are retained. The existing p4/tower planted-only policy from current main is byte-exact, including its known2G arithmetic fixture. The native transport preserves actual creation-vs-existing results and independently verified readback rather than the older unconditional-created adapter. Final validation is through the existing ML2958 integration, so these committed fixes will land together with their complete original histories.

Post-merge sparse-checkout omissions are recorded as failures in the complete actual-generator attempt ledger. Exact HEAD inputs were rehydrated, with SHA/length custody. Final unmodified assessment check passes at attempt8 and dashboard check at attempt34; all preceding omitted-input failures remain recorded. Existing actionlint and workflow wiring checks pass. Final full library/CLI/strict lint and hosted current-head gates remain pending; earlier results do not qualify later main code automatically.
