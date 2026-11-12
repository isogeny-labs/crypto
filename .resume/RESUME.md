# Resume: crypto @ aburan28/youthful-hopper-hfrwkd

## Goal
User (2026-11-12): run steps 1–3 of the cheapest test of "idea 2": a cover φ: C → E used only to
generate the factor base x∘φ(V) on E(F_p); measure the F4 solving degree D against its null,
the PKM tower and a random base of equal size.

## Lead / hypothesis
`research/cover_image_fb_20261112/PROTOCOL.md` (committed 07c6471fe5 before any cell). P0 positive
control (square map collapses Kummer depth t → t−1), H1 survive / H0 falsifier. Prior: H0 ≈ 0.8.

## Spec
Local container (4 cores), `target/release/examples/pkm_tower_pilot --engine f4 --p 2013265921
--kinds kummer,dickson --m 2 --controls tower,null,randbase [--cover-map square|quad|generic]
--t-min 2 --max-t 8 --budget 600`, plus m = 3 t ≤ 4. Run script `research/cover_image_fb_20261112/run.sh`.
Counts only (D), no timing comparison, not a benchmark.

## State
Code: `examples/pkm_tower_pilot.rs` (`--cover-map`, `CoverMap`, control `randbase`). Built.
Runs done (m3-square stopped at time limit). README written.

## Next step
Run run.sh, tabulate D per (map, kind, control, t), plot, write README + report, open PR.

## Stop and success conditions
Stop when every arm has run t = 2…8 at m = 2 (or stopped at the first all-timeout t) and t ≤ 4 at m = 3,
and the verdict (P0, H1/H0/inconclusive) is written. P0 failing stops the reading. Success = a verified
verdict either way, not H1.
