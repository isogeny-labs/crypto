# First-independent rootfinding relation screen

The frozen [run](reports/run-20261116T1342Z/REPORT.md) passed one source-bound
test on the three fixed Koblitz streams. The candidate reached full rank
with more modeled group additions than first-hit in every cell: K13
991 vs 963, K19 15,337 vs 14,203, and K23 144,219 vs 128,211. K19 used
156 instead of 182 target queries, but its extra scanning outweighed that
query saving in this operation proxy. This is a fixed-stream collector result,
not a complete-call timing comparison. [pcc:unverified]

The [machine result](RESULTS.json), [run record](RUN_RECORD.json), raw
[`run.stdout`](run.stdout), [`run.stderr`](run.stderr), diagram source/SVG,
chart source/SVG and [inspected PDF](reports/run-20261116T1342Z/report.pdf)
are in this study. The protocol was committed before source and run.
Every curve/arm reached full rank and the harness asserted scalar 641;
independent replay is pending. [pcc:unverified]

The first-independent policy is not proposed for production integration
on this evidence. The existing [complete-cost problem](../../problems/rootfinding-complete-cost-calibration.md)
and [frozen F4/F5/F6 panel](../rootfind_f456_batch_combined_20261115/PROTOCOL.md)
carry the next performance gate. The panel remains unrun because the
correct-account monthly and daily spend readbacks exceeded their limits.
[pcc:unverified]
