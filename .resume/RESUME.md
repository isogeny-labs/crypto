# Resume: crypto @ claude/gifted-noether-3i38v4

## Goal
Owner (2026-11-15): work until a new SAT solver for the point decomposition problem is near rho parity and better than every existing implementation here. Plan: research/sat_pdp_exotic_20261115/PLAN.md. Success criterion: every verified, measured improvement over the re-measured baseline counts; distance to rho is reported, not gated.

## Spec
Protocol: research/sat_pdp_exotic_20261115/b0/{PROTOCOL.md,protocol.json}, frozen by FREEZE.sha256 (commit recorded there). Instrument: examples/b0_pdp_sat_baseline.rs (arms oracle, native, native-aa, cms, cadical, chain-p12, lin, lin-nolex); new solver examples/pdp_lin_cdcl.rs; external arms tools/b0-ext-solvers (needs third_party/cryptominisat-rs-patches/prepare.sh then cargo build --release there). Bench record: benchmarks/records/20261115T0359Z-b0-pdp-sat-baseline.json. Rows: l = 7..10, 16 uniform + 4 planted, seed 20261115, budget 4e6 (1.6e7 at l = 10), one thread, local container (counted metrics; L0).

## State
2026-11-15 ~04:00Z: instrument checks done at l = 5, 6 (all arms; 0 disagreements, 0 unverified) and l = 5 for the new solver (R = 1.20 vs incumbent 10.9). l = 6, 7 lin checks running. Next commit freezes the protocol; rows follow.

## Next
1. Freeze (FREEZE.sha256 commit). 2. Run rows l = 7, 8, 9 (all arms; chain-p12 at l = 7 only), l = 10 fast arms, into research/sat_pdp_exotic_20261115/b0/rows/l*.jsonl. 3. analyze.py -> ANALYSIS.json, table.md, figures. 4. benchrec add-raw/metric/close; progress report. 5. Report + PDF (research-visuals); session log; PR #4225 (already open, auto-merge). 6. Validator agent replay of l = 8, 9. 7. Then candidates 3.3 (REUSE across targets) and the generalisation study on lin's learnt clauses.
