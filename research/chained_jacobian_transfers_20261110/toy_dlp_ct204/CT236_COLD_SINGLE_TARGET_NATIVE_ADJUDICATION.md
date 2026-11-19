# CT236 — CT235 Q73-only cold native cost adjudication (2026-11-19)

**Standing: [pcc:unverified].** This is an independent *record-only* numeric audit of producer-hosted Rust output, **not** an independent native relation replay, growing-size attack, or signed PCC.

## Immutable provenance and acceptance
- Source branch: `research/ct204-odd-cover-toy-dlp-20261118`, exact native source commit `58179c8befdaa5137281fcdd8caf3cbb4fd050c4`. [Original native CI run #44547727541](https://github.com/isogeny-labs/crypto/actions/runs/44547727541), native job `135642735410` and independent-field-oracle job both **success**.
- Original native log has **16 distinct `CT235_SINGLE_JSON` records** (four factor bases times four in-process ABBA paired rounds); a final shell `grep` echoes all 16 once. The echoed rows were explicitly matched and excluded. The raw, original 16 rows and source/run/job metadata are committed as [ct235_native_panel.jsonl](ct235_native_panel.jsonl). Never count 32 observations or claim four independent hosts.
- Separately triggered [exact-head hosted record-audit run #44559834670](https://github.com/isogeny-labs/crypto/actions/runs/44559834670), job `135685768154`, **passed** on frozen-panel commit `50fa94ea6d9b72482b1897c62e0d0b0fdf62e98d`. Both Python and Python `-O` printed `CT235_RECORD_ONLY_AUDIT_PASS 16 unsigned observations`; 14 synthetic missing/duplicate/corruption controls rejected in both. Only record algebra, numeric fields and independently calculated elliptic Q73 point multiplication were audited; raw Jacobian relations were not independently regenerated.

## Four cold one-target results
Curve `E/F101: y²=x³+x+46`, order-`109` source generator `G=(2,37)`, public `Q=(19,37)=[73]G`; genus-4 composite Jacobian with weighted two-large-prime elimination. Every recorded arm has full rank, produces scalar 73, and verifies Q. All costs are from one GitHub Actions runner, with four fresh in-process constructions and alternating comparator order; BSGS is a **tiny provisional control**, not the matched strongest rho/frontier IC.

| Signed factor-base size | Matrix rank | Relation candidates | Median complete IC cost | Median paired BSGS cost | Median paired BSGS/IC ratio | Median paired slowdown |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 24 | 24/24 | 2882 | 52.456 ms | 1.377 µs | 0.000026267 | ~38,222× |
| 36 | 36/36 | 1596 | 29.328 ms | 1.477 µs | 0.000050361 | ~19,857× |
| 48 | 48/48 | 1421 | 26.277 ms | 1.492 µs | 0.000056778 | ~17,662× |
| 54 | 54/54 | 1421 | 26.474 ms | 1.623 µs | 0.000061392 | ~16,404× |

These are **median paired ratios**, not the ratio of the two separately reported medians; the values are allowed to differ. Per-arm original timing vectors, all setup/relation/LA/verification/target/recovery phases, LP counts, actual Cantor and field counters, logical memory and process-wide VmHWM are in the JSONL panel.

- The B48 median solver time is 0.197 ms (0.74%) below B54 in **this one runner**. However B48 median paired ratio is *worse* (0.0000568 vs 0.0000614), and neither a statistically supported relative advantage nor a meaningful ECDLP speedup follows. No cherry-picking.
- Relation collection accounts for approximately 99.10%, 98.42%, 98.26%, 98.28% of full candidate wall time for B24/B36/B48/B54 respectively. Those percentages are diagnostics, not a reduction in complete solving cost.
- B24 has 2,909 Cantor additions and 1,945,962 field multiplications; B36 1,611/1,083,268; B48 and B54 each 1,435/966,327. Approximate logical memory peaks: 49,208 / 46,760 / 51,928 / 54,568 bytes respectively. VmHWM is **process-wide**, and does not isolate candidate memory from BSGS or the test runner.
- B48 vs B54 candidate cost ordering is too close to distinguish on a single, unisolated host; neither beats even the provisional BSGS baseline. No confidence interval across independent hosts and no PCC signature.

## Commands and scope
```sh
cargo test --locked --release --test ct226_native_genus4_transferred_ic ct235_ -- --nocapture --test-threads=1
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct235_single_target_cost_audit.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct235_single_target_cost_audit.py research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct235_native_panel.jsonl
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct235_single_target_cost_audit.py research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct235_native_panel.jsonl
```

## Decision / next unblocked lead
Stop tuning this unchanged F101 full-Jacobian collector solely to improve relation yield; its cold full cost is decisively dominated by the collector and strongly worse than tiny direct BSGS. Next prioritize a **different cover/collector or source-preserving transfer with a feasible target factor-base growth law**, plus independent raw relation replay and strongest exact-curve same-host optimized rho/BSGS/frontier IC controls. Avoid duplicate concurrent genus-4 and transfer catalog branches. Before any speedup claim require independently reproduced same-source scalar, cold full setup + transfer + LP + LA + target + verification cost, memory, calibrated uncertainty and signed PCC.

The **original frozen 695/701 Jacobian norm-point fixture** has already produced unsigned finite parity evidence in cryptanalysis; its independent authenticated source replay, actual norm reuse, physical `C169`, `C13a`, `C13b`, `N2_actual` and `D` pricing, and approved EKS budget/data access remain unfulfilled. Do not invent those inputs. No paid jobs, protected merge, or certificate promotion occurred.

Sources checked: merged [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), ongoing [draft #4715](https://github.com/isogeny-labs/crypto/pull/4715), CT233/CT234/CT235 frozen local protocols, cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), and shared AGENTS.md/ACTIVE/session log. The research-KB MCP and checkout-local wrapper were not accessible in this tool environment; no novelty claims are made.
