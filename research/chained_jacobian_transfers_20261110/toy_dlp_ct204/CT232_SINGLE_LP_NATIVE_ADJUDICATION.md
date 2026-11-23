# CT232 — Native single-large-prime IC: recovered 36-variable target, still slower end-to-end

Date: 2026-11-19 UTC. Branch: `research/ct204-odd-cover-toy-dlp-20261118`; [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715). **[pcc:unverified]**.

## Actual hosted measurements (not predictions)

Original preregistration: [CT231 protocol](CT231_SINGLE_LARGE_PRIME_PROTOCOL.md). Exact source checkout: `4c5ef4e4d65d62b93501b0b3a1417001aab0f14f`. Original [GitHub Actions #44509580066](https://github.com/isogeny-labs/crypto/actions/runs/44509580066), job `135509986267`, completed with native Rust test and the independent field-oracle job **successful**. Native test `ct231_single_lp_join_signs_and_rhs` and full `ct231_single_lp_complete_solve_panel` both passed. At 11:11 UTC, the native log emitted four `CT231_PANEL_JSON` records, then the workflow's final `grep` echoed precisely the same four lines. **This was one native execution on one hosted runner, not two or eight independent repetitions.**

| Signed-base variables | Rank | Full-smooth | One-LP events | Pairs added | Candidates | Verified target | Complete native wall time |
|---:|---:|---:|---:|---:|---:|---|---:|
| 24 | 17/24 | 3 | 46 | 24 | 6,540 | No | 118.968 ms (failed arm) |
| 36 | 36/36 | 16 | 50 | 34 | 2,615 | Yes, log 37 | 47.432 ms |
| 48 | 48/48 | 40 | 21 | 16 | 1,569 | Yes, log 37 | 28.543 ms |
| 54 | 54/54 | 62 | 0 | 0 | 1,421 | Yes, log 37 | 25.867 ms |

This is a **real correctness advance** relative to CT230: base 36, which previously stalled at rank 34/36 with no target, now reaches rank 36/36, verifies each recovered factor-base norm and the large-prime logs, and recovers the public source logarithm 37. Base 24 still fails. All four arms were retained, without assigning a completed-solve ratio to the failed one.

Within this same host invocation, the completed 36-variable arm is **1.834× slower** than the unchanged 54-variable control; 48 is **1.103× slower**. Relative to the same-job tiny direct elliptic BSGS: basis 36 is **37,885× slower**, basis 48 **26,138× slower**, basis 54 **22,089× slower**. These are single unisolated observations, not confidence intervals or an optimized-rho result. Do not compare the absolute times to CT230 across different GitHub runs as though the runner were calibrated.

## Independent record adjudication

Frozen unmodified four-line native payload: [ct232_single_lp_native_panel.jsonl](ct232_single_lp_native_panel.jsonl). A separately authored Python standard-library auditor, [ct232_single_lp_cost_audit.py](ct232_single_lp_cost_audit.py), tests the exact schema, four-arm completeness, source provenance, support classification bounds, `pair_rows = single_lp - distinct_lp`, integer counter consistency, full source scalar verification field, rank/failure distinction, candidate/offset caps, full-base control and phase sums (observed residual 140–340 ns). It independently derives the single-large-prime elimination/recovery equations for **all four sign combinations** over F109 and rejects **11 mutation controls**, including false recovered scalar, missing arm, altered source, and unearned PCC status. This is **independent analysis of producer records**, not independent execution of the native Jacobian solver.

Reproduce after checkout:

```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct232_single_lp_cost_audit.py --self-test
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct232_single_lp_cost_audit.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct232_single_lp_cost_audit.py
cargo test --locked --release --test ct226_native_genus4_transferred_ic ct231_ -- --nocapture
```

New bounded hosted workflow: [ct232-single-lp-cost-audit.yml](../../../.github/workflows/ct232-single-lp-cost-audit.yml). Its results must be inspected on the exact new source head; a created workflow or local parser is not equivalent to passed hosted CI or signed PCC.

## Decision and next best experiment

CT231 supports the single-large-prime technique as a way to recover *otherwise unsuccessful* small-basis source-target logs, but **does not show a complete-cost advantage** even against the full-base control in this F101 toy. The next bounded branch should test a true two-large-prime collision graph/cycle, with exact signed incidence elimination, full source-log recovery, memory accounting and same-run optimized rho/BSGS/frontier IC, or move to a cover that avoids this particular `Theta(p)` full-base scaling. Do not reuse successful rank alone as a performance claim.

The original 695/701-record native norm-reuse result is already published unsigned in cryptanalysis. Independently authenticated fixture replay, physical `C169`, `C13a`, `C13b`, `D` and actual norm-operation prices, cold setup+transfer+LA+target+RSS, matched reference and independent PCC remain open; S3 authorization and EKS budget STOP are unchanged. No paid compute, original-source substitution or protected merge was authorized or performed. Research-KB MCP / repo-local wrapper unavailable here; prior art and source reviewed at CT231 preregistration (Gaudry–Thomé–Thériault–Diem and Diem).
