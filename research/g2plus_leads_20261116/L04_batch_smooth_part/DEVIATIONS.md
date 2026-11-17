# Deviations from PROTOCOL.md / SPEC.md

No change to a threshold, arm, seed rule, grid, T or the decision rule. No T halving. No hook refusal.

1. `splits_or_prefilter_pass` (SPEC section 7) is ambiguous for `A_pf`. Implemented as: for A arms, the number
   of inputs that reached `roots_of`; for `B_pf` and `Cpf_*`, the number that survived the prefilter; null for
   `B` and `C_*`.
2. SPEC test 15 is stated only as "C_m1 and B equal dec totals". Implemented in the `_cell` helper shared by
   tests 13, 14, 15 (test 15 re-runs the g = 3 helper). The helper additionally asserts
   `tprod.mul == 0` and `trem.mul == B.dec.mul - post.mul` for C_m1.
3. Extra files beyond the SPEC layout, both outputs only: `code/report.py` (writes the tables in RESULTS.md and
   `plot.svg` from `raw/analysis.json`) and `raw/stderr_*.log` (progress). `raw/` runs used `ulimit -t 600 -v 2000000`
   per invocation as the resource guard; no invocation hit it.
4. Performance-only implementation choices that do not change counts: slice/zip list comprehensions inside
   `vrem`, `poly_mul`, `build_FS` (counters are incremented exactly as SPEC states). The unit tests fix the counts.
5. G2 was evaluated by `analyze.py` from `raw/unittest.log`, which was produced by the first command of
   `raw/COMMANDS.log` before any cell was run.
