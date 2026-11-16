# iso-14: reading P-ISO-3 (`t = 14`), Amendment 19

Derived by `iso/score_iso.py` (bundle at `9ea3d533026a…`) from the rows in `fetch-20261115T0515Z/runs/` (manifest-checked).
`[pcc:unverified]`.

- **Gates.**
  - G1 pass. G-AUX on the pod: "pass 16 draws, 0 mismatched".
  - Recount of the `t = 14` auxiliary curve: 1 curve, 0 mismatched (`recount.json`).
  - Scorer: no conflicts, no STOP, exit 0.
- **Draws.** Both draws are certified inconsistent: 0 `V × V` solutions, so they are not decomposable, although λ = 0.133.
  Per draw:
  - closure (cmax 5, no 1): `d_F ≥ 6`;
  - uncapped F4: stopped at its 1,800 s budget, so no bound;
  - capped F4 at cap 5: completed, does not refute, so `d_F ≥ 6`.
- **Median** (lower median): `≥ 6`. The cell is complete. Draws with `d_F ≥ 6`: 2 of 2.
- **P-ISO-3 at `t = 14`: "grows"** (a censored `≥ 6` with the remaining draws certified; Amendment 19).
- **What it is.** A description of solving degree for this cell. It does not change P-ISO-2 (`t = 9..13`, pending in `iso-a`),
  and it is not an improvement claim. The exact value (6 or more) would need the Amendment 7 B bracket at cap 6. P-ISO-3 does not
  require it.
- **Cost.** Pod `91y9xgsnne97pj`, 04:36:56–04:43:21Z plus time until deletion at about 05:16Z: about US$0.7 (derived).
