# Resume: crypto @ aburan28/inspiring-dijkstra-ay05qi

## Goal
Owner (2026-11-14) asked to implement the PKM auxiliary-curve IC and show the prime-field speedup, then: "the results we saw before were pretty good so something must be off" — audit all favourable earlier results.

## Spec
Audit finding: the batched multi-target folded rho in examples/prime_fast_j0_rho_multi.rs restarts walks from a target drawn uniformly over all T targets (solved or not), so its step counts are 8-35x above Kuhn-Struik sqrt(2 n T / 6). Fix: restart only from unsolved targets (default); RESTART=all reproduces the old policy.
Rerun: target/release/examples/prime_fast_j0_rho_multi <out.jsonl> <bits> <T> 2 for bits 26,28,30 and T 200,1000,3000 (plus 20 at 26), both policies; compare with research/ic_e222_m221_20261106/results/pf_j0/symmulti_*.jsonl (IC rows unchanged, same planted targets).
Outputs: research/ic_e222_m221_20261106/results/pf_j0/rhoks_*.jsonl and rhoall_*.jsonl.

## State
2026-11-14: patch written, release build running. No rows yet.

## Next
Run both policies, tabulate rho/IC, append correction section to RESULTS_prime_fast_j0_sym.md, session log, bench record, PR.
