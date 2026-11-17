# Prime31 batched pair-table index calculus versus shared-BSGS on identical 1024-Q — 2026-11-17

## 18:25–18:32 UTC — frozen protocol, real Rust implementation, GitHub source-specific CI

### What was done (commands, files, features)
- Read `AGENTS.md`, `docs/ACTIVE.md`, the dated prime31 seven-quotient session log, prime IC frontier session log, exact source of `src/cryptanalysis/prime_fast.rs`, merged PRs #4572 (new batched lock-step complete IC), #4581 (seven-quotient split), #4532 (actual prime31 1024Q full-cost generic control), and pre-existing native 1024Q IC and BSGS source. Checked cross-repo #3301/#3305 and cryptanalysis #1536 and the recorded primary PKM 2016 and APS 2018 paper identifiers. KB MCP/CLI **unavailable** in this connector-only runtime, so bounded original-file/PR audit substituted; no novelty absence claim.
- Created task branch `automation/prime31-batched-ic-shared-frontier-20261117` and its first tracking commit (cc9a2f4), opened [draft PR #4596](https://github.com/isogeny-labs/crypto/pull/4596), committed PR link in `docs/ACTIVE.md`.
- Froze `research/prime31_ic_20261115/batched_frontier/PROTOCOL_20261117.md`: actual p=2147483647, n=2147446831 ordinary cofactor-one curve and exactly the earlier 1024 public Q; Rust batched IC with FB=1920, extra=32, max_rounds=500000, seed=20261117; same-host shared-BSGS controls with three preregistered m. All setup and failures charged. 
- Implemented `examples/prime31_batched_ic_same_1024q.rs` reusing original `prime_fast::ic_multi_solve_ex`, verifying true curve, subgroup, original inputs, and all returned k with original group law, reporting 1024 per-target statuses, costs, walks and process high-water memory. No known caller k enters the candidate solver.
- Committed `.github/workflows/prime31-batched-ic-shared-frontier.yml`, pinned Rust 1.100, original native curve/BSGS source reconstruction as #4532, same runner/corpus, native and candidate tests, raw exact JSON artifact and same-Q comparison. Source latest commit `b2b74238967cdb1beb80c49a88aee4a7cfcb6063`. [GitHub Actions run #44148122865](https://github.com/isogeny-labs/crypto/actions/runs/44148122865) started and was **in progress** at initial readback (no pass or timing claim).

### Decisions made (and why)
- The merged #4572 batched table is the new fastest measured in-repo prime-field IC branch at nearby size, but only on *generated* a=-3 curves against rho. The actual ordinary prime31 1024-Q fully-shared BSGS benchmark (#4532) remains a more meaningful control. Rather than duplicate earlier RR9 signed fibre/quotient negative branches, evaluate the batched optimization on this exact published corpus and then return to truly sign-free B31 algebraic extraction.
- Pair-table detection `R=±F_i±F_j` is **group-arithmetic/generic MITM equivalent**, even though complete log recovery is an IC pipeline. A speed claim here would concern implementation and same-cohort cost only, not a new non-generic asymptotic algorithm.
- No claim of successful 1024-Q batched recovery until exact-source CI and JSON audit. Historical BSGS best 170.113ms and mixed IC 2221.474ms are *prior producer* data, not this new run.

### Open threads
- Read exact-source CI #44148122865; repair compilation faults without changing frozen curve, cohort or parameter choices, preserve errors. Publish raw benchmark/negative result, compare 1024 complete answers with BSGS, update tracking and PR.
- If batched candidate remains slower/incomplete, prioritize real public-residual-only sign-independent full-eight RR9 sparse-D9 or B31 multiplicity/F4 extraction and matched optimized-rho/BGS external IC cost gates.
- Independently signed PCC review and external mature IC comparator remain required for leaderboard/security claims.

### Errors
- No local Cargo toolchain or research-KB MCP in this connector-only execution; Rust tests delegated to exact-source GitHub Actions. No results inferred from queued CI.
