# Resume: crypto @ claude/tooling-build-alq44y

## Goal
Measure whether odd-degree (3, 5) and mixed 2/3 PKM Kummer towers change F4's counted work
(`field_ops`) against the degree-2 tower at equal information log2 S (EXP-PKM-ODD-001). Part of
PR #4091 (pkm-smooth-tower tooling, report and tutorial).

## Lead / hypothesis
Barrier-note information reading: cost tracks log2|V|, not how membership is written. Prediction:
no_material_effect for A3, A5, A23, A32 at m = 2, 3, tower and null; no order effect. Falsified by
gain or loss for any arm on the tower at m = 2. Never run before: the tower engine cannot express
odd degrees (research/curve25519_leads_wave4_20261104/l11_odd_kummer_design, Q-B).

## Spec
research/pkm_odd_tower_f4_20261114/PROTOCOL.md and protocol.json (frozen in the commit that adds
them, before data). Instrument examples/pkm_odd_tower_f4.rs (generic f4_fp, grevlex, D = 64).
Runner run_cells.sh; analysis analyze.py.

## State
Instrument built and smoke-tested (all rows agree with exhaustive check; planted points check).
Protocol frozen; run launched locally with CHECKPOINT_SKIP=no-s3-credentials (no AWS here).

## Next steps
1. `cargo build --release --example pkm_odd_tower_f4`
2. `RAYON_NUM_THREADS=4 CHECKPOINT_SKIP=no-s3-credentials research/pkm_odd_tower_f4_20261114/run_cells.sh`
   (skips completed cells; reruns a partial cell from the start).
3. `python3 analyze.py` in the study dir; write REPORT.md/.typ with graphs and PDF; update the
   tutorial §9; push to PR #4091.

## Runs in flight
run_cells.sh on the session container (local, 4 vCPU). Progress in progress.txt; rows in runs/.

## Stop and success conditions
Stop when every cell in run_cells.sh has its rows (censored rows count) and analyze.py has run.
Success = a reported verdict per arm under the frozen rule, whatever it is. instrument_invalid if
any row disagrees with the exhaustive check, a planted row fails, or the A/A cell differs.

## Open questions
Isogeny (rational-map) towers of odd degree are not in this study.
