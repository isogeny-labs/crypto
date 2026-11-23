# CT219 — anchor-free signed-factor toy IC and fresh paired cold-cost diagnostic

2026-11-18 · Existing [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) · **[pcc:unverified]**

## Distinct new result, after reviewing concurrent CT218

CT218 already obtains rank 54/54 with 61 smooth relations and an *explicit known-generator factor-base anchor*. This CT219 experiment independently reaches rank **54/54 with no known-generator anchor row and no source-E discrete-log oracle in its relation solver**. It builds the 54 signed factor symbols from the complete 108 affine rational points on `C/F101: y²=x⁹+x³+46`, folds `y↦-y`, and collects real squarefree degree-4 Cantor relations from sequential source-subgroup walks. Each equation has the exact known RHS `3k mod 109` and subtracts the offset's actual point support. A unique rank-54 solution is recovered by modular elimination and each of the 54 resulting source norm logs is checked by independent E arithmetic *after* solving.

Local developer-run evidence on an unisolated Python host: **14 offset pairs**, **1,421** degree-4 candidate checks, **62** smooth equations, **54** independent columns, **1,435** charged Jacobian relation/offset additions. The independent verifier also recovered **all 109** public group targets using only the solved factor-base logs and a new Jacobian target walk, and rejected a deliberately inconsistent RHS in the adversarial control. The target `T=(64,19)` returned scalar **37** after 3 target candidates and 5 charged Jacobian additions. The matching fresh direct elliptic BSGS used **25 E additions**.

In contrast to CT218, CT219 does not preregister `log_G(π(P_G))=1` as an anchor; 62 geometric relations supply the complete rank on their own. This eliminates a special-case generator-seed requirement at the cost of 19 more trial candidates and one additional offset. This is a finite toy solver, not a growing-field breakthrough. Here `x↦x³` permutes F101 and the 54 signed factor symbols effectively enumerate *all* source E rational points modulo sign; such a full group-sized factor base does not demonstrate sub-generic scaling.

## Host diagnostic: 7 paired fresh cold single-target executions

The candidate rebuilds its entire factor-base, all signed relations, Gaussian solver and target recovery each repetition, plus verifies all 54 factor logs; the reference builds a new BSGS baby table for the same public target and verifies its answer. Both use the same Python finite-field implementation, one process/host, alternating candidate-first and BSGS-first order. All timings include Python `tracemalloc` instrumentation; this is **not isolated hardware, the strongest Rust rho/BSGS/IC baseline, native operation pricing, independent PCC or a calibrated performance result**.

| Paired round | First | Candidate ns | Direct BSGS ns | Direct/candidate |
|---|---|---:|---:|---:|
| 1 | candidate | 1,696,795,103 | 179,761 | 0.00010594 |
| 2 | direct | 1,661,022,881 | 150,357 | 0.00009052 |
| 3 | candidate | 1,625,180,794 | 154,163 | 0.00009486 |
| 4 | direct | 1,694,504,844 | 148,695 | 0.00008775 |
| 5 | candidate | 1,613,514,106 | 154,283 | 0.00009562 |
| 6 | direct | 1,580,218,355 | 145,821 | 0.00009228 |
| 7 | candidate | 1,582,534,469 | 153,431 | 0.00009695 |

The median **direct/candidate = 0.00009486** on this diagnostic, a roughly 10,542× *Python-host slowdown*. This must **not** be extrapolated to optimized C/Rust or to prime-field transfer security. No A/A isolation or cross-host CI interval was measured. Instrumented `tracemalloc` peak was approximately 99 KB of tracked Python allocations in one local run, not an RSS peak or a bound on Rust memory. No performance improvement is claimed. The small-field toy relation path is deprioritized for performance.

## Reproduction and scientific gates

```bash
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct219_anchor_free_cold_cost.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct219_anchor_free_cold_cost.py --paired
```

The existing [bounded CT205 workflow](https://github.com/isogeny-labs/crypto/blob/research/ct204-odd-cover-toy-dlp-20261118/.github/workflows/ct205-toy-transfer.yml) now checks the self-test and seven raw pairs against the exact GitHub branch head. **CI outcome was not known when this report was committed**. The measurements above were local producer observations and are not signed; repeat on exact-head CI before advancing their provenance. Avoid duplicate execution of CT218's preexisting solver and reuse [crypto #4215](https://github.com/isogeny-labs/crypto/pull/4215), [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and [genus-4 #4693](https://github.com/isogeny-labs/crypto/pull/4693).

For the owner's decisive benchmark, the authenticated frozen 695/701 model still requires S3 authorization; then measure actual norm-point reuse and C169, C13a, C13b and D on the same host, with setup/transfer/relation/LA/verification/memory and matched optimized rho/BSGS/IC. **This CT219 fixture is not a substitute**. No EKS/Spot launch, protected merge, PCC speedup, or unapproved spending occurred.
