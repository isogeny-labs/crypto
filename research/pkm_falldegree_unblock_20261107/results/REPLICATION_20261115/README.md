# Cross-session replication: stage B and Kummer `t = 13` draw 0 (owner decision 19)

Two sessions ran the same registered PKM cells independently on 2026-11-15, on different reserved RunPod pods:
- this branch: preregistration Amendments 26–27, pods `agyp0kbpguz1ss`, `ug0ohhn9h6pdv4`, `q7scp0zg6ic3oj`;
- session `01TCbLH6osebpnEbutNogBzD`: Amendments 16–18, 21; `results/RPB/6m9ewd49m2ivbn/`, `results/RPX/`.

Both used the frozen binaries and 12 rayon threads. Compared here field by field. Measured; `[pcc:unverified]`.

## Stage B (`stage_b_compare.json`)

- **144 systems in common** (`B-*` closures and `DB-*` F4, keyed by file kind, `p`, kind, `m`, control, `t`, draw and `x_R`):
  **144 identical, 0 different.**
  - Closure fields: `last_fall_degree`, `first_fall_degree`, `falls`, `muladds`, `timed_out`, `censored_at`.
  - F4 fields: `solving_degree_max`, `inconsistent`, `muladds`, `timed_out`, `oversize`.
- 0 systems in their run only. 1 in this branch's only: its E3.7 positive control (`t = 12`, draw 0, cap 5), which is not a
  stage B row.

## Kummer `t = 13`, draw 0, cap 6

Equal on `x_R` (1118191946), `degree_reached` 6, `inconsistent` true, `basis_len` 1, `max_cols` 177,063 and `muladds`
288,586,444,503,310; neither run timed out or hit oversize. Wall time differs, 7,908 s against 12,372 s (different hosts),
as expected for a wall-clock measure.

## Reading

Every exact count of both stage B runs and of the `t = 13` cell is reproduced by an independent run on another pod. The
admitted values do not depend on which session or pod produced them. This is a reproducibility check; it changes no reading.
