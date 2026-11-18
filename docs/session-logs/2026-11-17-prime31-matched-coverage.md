# 2026-11-17 — Prime31 B31 matched signed-eight-leaf coverage null control

## What was done (commands, files, features)

- Checked `AGENTS.md`, `docs/ACTIVE.md`, newest completed original-prime31 log `docs/session-logs/2026-11-17-prime31-complete-rank1024.md`, source and raw 1024-Q transcript from #4623. Inspected currently active #4596 (matched IC/BSGS), #4604 (D9 negative certificates), #4624 (48-curve synthetic Dickson comparison), #4617 (synthetic coverage), #4594 portfolio, #4615/#4618 generic signed oracle, and PKM/APS/Kudo literature. KB MCP/CLI unavailable in this connector runtime; existing repo and primary-source fallback only.
- Confirmed that first 256 frozen public targets in complete Rust transcript have 237 a=0 hits; all 1024 had 951. The formal signed-net support cardinality is N=5,654,193,417, N/n=2.6329841; the naive independent-bin occupancy heuristic predicts 237.60/256 and 950.41/1024. This is an analytic **heuristic**, not a causal explanation or proof for a fixed base.
- Registered task in `docs/ACTIVE.md`, created branch `automation/prime31-b31-coverage-null-control-20261117` from #4623 exact head `e4d5a8ba`, opened draft PR #4626 stacked on unmerged #4623, committed frozen prospective `B31_MATCHED_COVERAGE_NULL_PROTOCOL_20261117.md` before touching Rust experiment code.

## Decisions made (and why)

- Use original native Rust group, factor-base generator, 4A index and 512-way complement arithmetic; match target Q at first public shift with one Dickson, one first-x and two fixed-seed random B31 bases. This falsifies a proposed *unique special-base coverage* advantage without duplicating #4624's synthetic 48-curve study.
- Explicitly distinguish the exact pairwise-collision expectation for 31 **independent uniform abstract scalars** from the independent-bin occupancy heuristic (correlated signatures) and from actual Dickson B31.
- Use fixed 256-Q subset and seeds, original-curve witness checks, full negative exhaustion, counts, and source-specific pinned CI. No new solver or cryptanalytic novelty claim.

## Open threads (unfinished, pick up next time)

- Implement native Rust matched coverage test and exact-schema output on branch, pin workflow to original immutable native source, execute exact-source CI, preserve raw output, audit deterministic counts and matched base differences.
- Record observations and update PR #4626; no merges or PCC/leaderboard promotion. Continue true public-only sign-independent B31 RR9/S9 or K961/S5 extraction in separate owner lane after the coverage audit.

## Errors (what went wrong, how it was handled)

- Local container has no rustc/cargo; native measurement must run through GitHub Actions on exact committed Rust source. No result is claimed before source-specific CI.

## Exact-source outcome and handoff — 2026-11-17 23:30 UTC

### What was done (commands, files, features)

- Implemented the prospective original-prime31 Rust experiment in existing `research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs`: four fixed B31 bases, 256 identical public residuals, unshifted exhaustive 4A+4A, batch inversion, original-curve witness replay and exact boolean mask/counter JSON. Added pinned workflow `.github/workflows/prime31-b31-matched-coverage.yml`.
- Exact-source [GitHub Actions #44199751750](https://github.com/isogeny-labs/crypto/actions/runs/44199751750) succeeded on source `d797fad388b78ef79082379679ce8c8812502971`: **40/40** original native library tests and **1/1** new Rust test. Published complete 4×256 raw JSON `B31_MATCHED_COVERAGE_NULL_RAW_20261117.json` and detailed `B31_MATCHED_COVERAGE_NULL_RESULT_20261117.md`; artifact [#14381131375](https://github.com/isogeny-labs/crypto/actions/runs/44199751750/artifacts/14381131375) has GitHub-reported SHA256 `44a524ffe3f89b5bb9488827050b51a038efe419b734e1e0e2df49c6aecc7d7d`, not locally ZIP rehashed.
- Same 256 public Q first-shift hit counts: actual Dickson 237, first31 rational-x 244, two fixed-seed random B31 bases 237 and 237. Exact distinct 4A cardinalities 618885 / 618885 / 618765 / 618765. Pairwise contingency tables vs Dickson (both/Dickson-only/comparator-only/neither): 225/12/19/0; 222/15/15/4; 219/18/18/1. All misses exhaustively checked. No new ECDLP unknown logs or sign-independent relation extractor claimed.
- Separately checked each of 256 Dickson boolean hit bits and each of 31 base points against previous original 1024-Q #4623 raw transcript. Independently recomputed all 4 mask counts, all 3 paired tables, unique x, exact N4=618885/N8=5654193417 signed net counts using binomial coefficients, and original finite-field curve equation for all 124 canonical public base coordinates. Four bases' unshifted membership records preserved in durable JSON, not only short-lived CI logs. All these producer-side checks passed; signed independent PCC still absent.

### Decisions made (and why)

- Dickson's observed 8A hit rate was **not better** than the two seeded generic bases and was lower than first-x in this fixed sample; do not treat the high Dickson hit fraction as evidence of a non-generic mechanism. Do not mistake 120 extra 4A signature collisions in random bases as evidence of slower 8A coverage.
- Independent-bin Poisson 0.928136 heuristic matches the original ~0.929 observed well, but formal signatures are highly correlated, so that agreement proves neither uniformity nor ECDLP complexity. Preserve exact pairwise-collision null as a distinct rigorous statement only for IID abstract group scalar inputs.
- Retire this bounded generic-equivalent *coverage diagnostic*, not the algebraic sign-free RR9/S9/K961 lead. No merged branch, paid compute, PCC promotion, or performance leaderboard change.

### Open threads (unfinished, pick up next time)

- The *global* goal still requires public-R-only, truly nonenumerative B31 relation extraction using original Rust F4/quotient/rootfinding, then full rank→1,024 unknown-log recovery against equal-advice BSGS and optimized rho, including all failed attempts and peak RSS.
- Optional actual-curve B27/B29/B31 arity control uses same frozen corpus, rank and complete cost; this task's completed B31 four-base null control does not duplicate synthetic #4624 nor competing #4596/#4604.
- Signed PCC validation and isolated independent timing remain pending, as documented in the source-linked result. PR #4626 stacked on parent #4623; no auto-merge until eligible.

### Errors (what went wrong, how it was handled)

- None in source-specific Rust CI. One local tool-connector JS variable declaration typo during progress polling was immediately corrected and did not change source or results. Original Rust test passed on first source-specific run.
