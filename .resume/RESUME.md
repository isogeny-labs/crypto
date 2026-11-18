# Resume: crypto @ aburan28/inspiring-dijkstra-ay05qi

## Goal
Owner (2026-11-18): run P2 (KSUM-SIB: x-only siblings at the k-sum table's last level) as a paired recorded benchmark, plus the P1 recurrence probe. P3 dropped (field work only, ~2% of ksum5).

## Spec
research/ic_ksum_sibling_20261118/PROTOCOL.md (frozen in b01bb12ed0 with code); specs/P20,P22,P24.json (Table B curves); runner run.sh (ecbench run/verify/compare/band). Code: KSumTable::build_negation_folded_full(.., sib), oracle param ksum:m=5,sib=1. Unit test sibling_build_is_the_same_table_with_one_inversion_per_last_pair passes. Same commit fixes main's f4_fp.rs duplicate-definition build break.
P1 result: killed (kernel 0 on 5 curves, controls OK) — scratchpad p1run/p1_out.txt; to be copied into the study dir.

## State
Run complete: 0.825-0.863x at w=0, 0.857-0.887x at w=0.1, 1.000 at w=1; P1 killed. Record cannot close (main's malformed hec-multi-production-cold record breaks the progress build). PR #4649.

## Next
Run run.sh under benchrec run; summarise vs PROTOCOL predictions; README + graph + PDF; close record; PR; report.
