# Run 2026-10-18: replicating the n = 31, m = 3 curve effect

**Result:** the H1 on work_per_call from `run-20261011T2357Z` does not replicate, and
neither does the |F| effect that explained it.
- Full arm with seeds 5–8: work_per_call ICC 0.000 [0, 0.043]; |F| ICC 0.000 [0, 0.033].
- 32 fresh V: ICC(|F|) is 0.000 [0, 0.007] at n = 31 and 0.000 [0, 0.009] at n = 19.

The n = 31 m = 3 verdict becomes **inconclusive**, not H1.

Details: `research/notes/koblitz-isogeny/m3-presentation-vs-curve-n31-design-20261012.md`,
section "Follow-up (2026-10-18)".

Data:
- `experiments/koblitz_presentation/pres_31_0_l10_geom_m3pin_rep_s5-8.jsonl`;
- `icc_n31_m3_geom_rep_s5-8.json`;
- `pres_{31_0_l10,19_1_l6}_geom_F_rep_s101-132.jsonl`;
- `icc_{31_0_l10,19_1_l6}_F_rep.json`.

Graph: `icc_replication.svg`. PDF: `report.pdf`.
