# Resume: crypto @ aburan28/inspiring-dijkstra-ay05qi

## Goal
Owner (2026-11-14): "tune the rho dp bits and rerun" — after PR #4107 (Kuhn-Struik restarts), the multi-target rho was still 1.4-4.8x above sqrt(2Tn/6) steps, partly because dp bits grow with T.

## Spec
examples/prime_fast_j0_rho_multi.rs: DP=<k> override, DPRULE=ks (2^dp ~ W/(T*2^DPDIV)), INST0 offset.
Tune: DP sweep 2..12 on held-out instances INST0=10 (2 curves x 2 inst), cells 26/{20,200,1000}, 28/{200,1000,3000}, 30/{200,1000,3000}; pick a rule.
Confirm: chosen rule on INST0=0 (the IC's planted targets) vs research/ic_e222_m221_20261106/results/pf_j0/symmulti_*.jsonl.
Outputs: results/pf_j0/dpsweep/*.jsonl, results/pf_j0/rhotuned_*.jsonl.

## State
2026-11-14 ~19:00Z: complete. DPDIV=1.5 fixed on held-out sweep; confirmed on INST0=0: rho/IC 0.13-0.40. Record 20261114T1854Z-j0-multirho-dp-tune closed; PR #4144 open with auto-merge.

## Next
Track PR #4144 to merge. Open threads: 26-bit T=20 lane overhead; interleaved rho vs folded BSGS; isolated host and §5a band.
