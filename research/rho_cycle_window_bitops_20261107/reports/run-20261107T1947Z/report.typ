#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= Run report: rho cycle-window re-pricing and audit-ir ladder
Started 2026-11-07T19:47Z (UTC). Host: Claude Code cloud container vm, 4 vCPU, shared with two other agents; isolation L0 (no pinning). Counts and Callgrind only; no wall time is used. All values unverified assertions [pcc:unverified].

== Runs
+ *audit-ir ladder* (19:47Z): `ecbench audit-ir --spec ir_spread/spec-koblitz-symmetrised-n17.json --seq 0 2 4 6 9 11 13 15 16 18 20 22`, existing binary sha256 4ff2214e… (ir\_spread/binary.sha256), 12 of 12 records replayed under Callgrind; each line in ir\_spread/audit-ir-ladder.jsonl. Fixed parts by `callgrind_annotate --inclusive=yes` (ir\_spread/fixed-parts.jsonl). Fits by examples/ir\_gate\_fit.rs (ir\_spread/fit-ladder.json, fit-session-binary.json).
+ *rho re-derivation* (20:28Z–20:34Z): `examples/hec_rho_window_reprice --raw research/hec_bitop_20261020/raw --variant V`, V in full-width, collide-first, cycle-escape, negation-full-width; binary sha256 30395fc2c611d985… built from base a2be660914 plus this branch's charging change; 18 rows × 40 seeds (0xC0FFEE + t) per variant, 2,880 rho runs, all verified. Output raw/V.jsonl and raw/V.txt.

== Controls
- full-width: all 18 rho means equal the committed hec\_bitop\_20261020 values exactly (raw/full-width.txt).
- negation-full-width (passive window): window charges 0 on every row, as designed.
- audit-ir: each ladder record's bitops equals the committed record's (e.g. seq 9: 1,785,258; seq 11: 6,581,823).

== Values
#figure(image("../../figures/ratio_and_window_share.svg", width: 95%), caption: [Default walk: IC/rho per row (left) and window share of rho bitops (right); raw/collide-first.jsonl.])
#figure(image("../../figures/ir_vs_gates.svg", width: 95%), caption: [Ir against gates, 12 ladder records and 2 committed records; ir\_spread/fit-\*.json.])

Default walk: window share of rho bitops 0.037–0.277%; IC/rho change −0.037% to −0.277% per row; window word ops per walk operation 18.4–31.2. Ladder fit: Ir = 1.441e6 + 0.08704·gates (R² 0.9993, n = 12); without SignedFrobeniusClasses::new (≈605,300 Ir) and full\_price::reprice (≈778,000–786,000 Ir): Ir = 5.7e4 + 0.0859·gates (R² 0.9997).

== Missing or not run
No isolab run (no AWS identity on this host). The resume checkpoint push failed (no upload token). The pre-change binary was not built (shared build lock); the pre-change totals are the charged totals minus each run's reported window charges, priced at the row's widths.
