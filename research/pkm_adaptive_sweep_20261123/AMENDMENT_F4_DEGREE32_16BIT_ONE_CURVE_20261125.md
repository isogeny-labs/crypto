# Amendment — 16-bit rung has one roster curve (2026-11-25, before any timed call)

The frozen selection scan (`scan16_matches.jsonl`) returned the same instance,
`icv1-fp16-t351-d433f24e`, for CLI seeds 20261400, 20261401 and 20261402. A
census of all 61 seeds in the window (`--family generic`, interval preflight
only) gives **61/61 the same curve**: the generic roster has one 16-bit
curve, and `--seed` changes only the targets, as at 14 bits (#5067 §3).

Amended design: the 16-bit rung is **one curve × three target seeds**
(20261400, 20261401, 20261402), equal bases of 17 abscissae
(`prime-abscissa:below=32` against `prime-isogeny:t=5,seed=15,aux=ordinary`).
This replicates targets, not curves. The slope fit treats the three runs as
one curve-level point with target spread, and the 16-bit point carries less
weight than the 13- and 15-bit three-curve panels. Nothing else changes; CM
families (`j0`, `j1728`, …) were not substituted because they are a different
population.
