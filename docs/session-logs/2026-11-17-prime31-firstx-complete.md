# 2026-11-17 — actual-prime31 matched Dickson31 versus first-x B31 complete Rust cost

## What was done (commands, files, features)
- Read repo AGENTS.md, prior #4629 full-cost and #4626 matched-coverage reports, original Rust example, last parent session log, and open #4596, #4604, #4594 owner lanes. Reviewed PKM (2016), APS (2018), and Kudo et al (2018) publication summaries to bound novelty.
- Created branch `automation/prime31-b31-firstx-complete-frontier-20261117` on #4629 and draft tracking PR #4635; recorded ACTIVE.md line first. The branch is stacked on the unmerged parent; no merges or paid compute.
- Committed prospective `B31_DICKSON_FIRSTX_FULL_COST_PROTOCOL_20261117.md` **before** editing candidate Rust, then appended a native release test in the existing `prime31_s9_b31_half_sum_oracle.rs` source and a sparse-checkout source-specific Rust 1.100 GitHub Actions workflow.
- Pinned curve p=2147483647, n=2147446831 and original native arithmetic commit 1df714645aa683cf975bc76fcacb9842cb4780fd. Candidate source commit fbbe0fe22c5d07c8ea1ac8b3e8a1f0d644a45d2c, workflow commit 7ecdf941daad3368ea1898070788a5e42c20e5ea.
- Initial exact-source CI: https://github.com/isogeny-labs/crypto/actions/runs/44215701063 (pending when this entry was written). No success or timing claim before the Rust runner completes.

## Decisions made (and why)
- The 244/256 first-x versus 237/256 Dickson prior result is a *coverage-only* control. Measure full original Rust index-calculus-style rank-31 learning, 16 control relations, 256 unknown logs and all misses/shift/fallback charges on the same public Q to see if the coverage signal carries into complete elapsed cost.
- The 4A+4A pair-sum oracle is **generic-equivalent**; do not call a full-cost improvement non-generic, novel or faster than matched shared BSGS. Treat first-x point selection as a conventional base change, not a trapdoor or new attack.
- Do not duplicate the dedicated same-runner 1024-Q BSGS owner #4596 or D9 Wiedemann owner #4604.

## Open threads (unfinished, pick up next time)
- Read CI, fix build/source-specific issues if needed, capture raw JSON, compare 256-Q complete costs, replay scalar/witness correctness and document limitations. Mark PR ready only if source-specific correctness gate passes.
- Primary unresolved requirement: sign-independent, public-R-only B31 RR9/S9/K961 extraction, with original Rust F4/quotient/root machinery and no FourSet generic enumeration. Then rank and solve unknown logs, compare complete 1024-Q against optimized BSGS, rho, published prime-field IC, with PCC validation.

## Errors (what went wrong, how it was handled)
- Rust/cargo and shared research-KB are unavailable in the connector runtime; no local Rust benchmark executed. GitHub Actions is the measurement source, not a replacement for independent PCC.

## Exact-source outcome and handoff — 2026-11-18 01:31 UTC

### What was done (commands, files, features)
- Initial prospective AB exact-source original Rust CI [#44215701063](https://github.com/isogeny-labs/crypto/actions/runs/44215701063) passed 40 native library and one focused full-solver test. Completed rank 31/31 and **256/256** blinded unknown logarithms per base. Charged total Dickson31 **3642.075 ms**, first-x31 **2884.425 ms**; 20.803% less total time for first-x on fixed corpus, still generic-equivalent signed 4A membership.
- Per protocol, after the first run froze supplemental order-bias protocol, then added only `PRIME31_B31_BASECHOICE_REVERSE=1` panel reversal to original Rust test and second source-specific CI run. AB/BA [#44216218925](https://github.com/isogeny-labs/crypto/actions/runs/44216218925) passed original 40 native library tests and 2/2 focused original-source complete-solver executions. AB charged Dickson31 **3401.400 ms** vs first-x31 **2734.304 ms**; BA charged first-x31 **2729.344 ms** vs Dickson31 **3413.456 ms**.
- Across 3 runs first-x total-cost reduction **19.61%, 20.04%, 20.80%**. The known 237/256 vs 244/256 first-shift hit difference remains exact; Dickson31 276 target attempts including 20 exhaustive negatives, first-x31 268 including 12 negatives. 0 generic fallbacks in either arm. All 31 factor-base logs and authentic rank rows, 256 same public Q/k/witness for each base, and first-hit outcomes are byte-for-byte identical in both AB and BA repeat transcripts.
- Preserved raw [initial 512-target JSON](../../research/prime31_ic_20261115/native/B31_DICKSON_FIRSTX_FULL_COST_RAW_20261117.json), raw [AB/BA 1024-target JSON](../../research/prime31_ic_20261115/native/B31_DICKSON_FIRSTX_ABBA_RAW_20261117.json), and full [scientific report](../../research/prime31_ic_20261115/native/B31_DICKSON_FIRSTX_FULL_COST_RESULT_20261117.md). Native archive ZIP hashes recorded as GitHub-reported; not downloaded/rehashed.
- Independent producer-side BigInt group replay checked 62 base logs, 62 rank-row equations, 512 unknown-target scalar products, 512 signed witness group identities, generator order, and identical 256 Q between panels. The AB/BA repeat records are bit-for-bit identical to the replayed initial source records on all non-timing fields. No independent signed PCC.
- This experiment addresses the missing *complete cost* for the first-x factor-base lead already identified by #4626; no new algebraic relation extractor was built.

### Decisions made (and why)
- For frozen 256 targets, retain first-x31 as faster generic-equivalent original-Rust correctness control; completed improvement is scoped, not a novel non-generic cryptanalytic speedup, a universal factor-base ranking or a matched victory over external index calculus/shared BSGS. The reported 1.244–1.263× ratio is on the **same 256 Q**, from same-runner/sequential hosted measurements.
- Do not duplicate owner #4604 D9 algebraic extractor or #4596 matched 1024-Q BSGS; the research-next gate is replacing FourSet with truly sign-independent public-residual RR9/S9 or K961/S5 extraction. Only after that, compare total 1024-Q end-to-end costs and seek independent signed PCC.
- Parent #4629/#4626/#4623 chain remains unmerged; leave #4635 stacked and unmerged.

### Open threads (unfinished, pick up next time)
- Implement public-R-only B31 sign-independent signed-eight relation recovery from existing F4/quotient/rootfinding, without any 4A group-sum table or support enumeration; exact B8 positive/two negative and B31 exceptional gates are required.
- If further factor-base work is justified, first perform a fresh 1024-Q blinded holdout and record internal 4A probe count and RSS; do not tune generic table endlessly.

### Errors (what went wrong, how it was handled)
- None in either exact-source GitHub Actions Rust run; separate producer BigInt checks succeeded. Host CI is not independently signed PCC or calibrated isolated multi-host timing.
