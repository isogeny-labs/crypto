# Independent point arithmetic for qualified query fixtures

2026-10-25, after the additional query generator was frozen and while it is
running. Do not alter or replace its source-3cabc768f frozen executable or
its outputs. Build a separately named auditor from the extended source.

For every generated 128-row cell/family file, reconstruct the exact curve
and prime subgroup. Recompute all factor-base lifts and subgroup membership
with polynomial-basis F2mElement/BinaryCurve arithmetic and its independent
Artin--Schreier lift implementation. Recount all ordered signed m-point
relations using this reference group's addition/negation instead of the
producer's Gf2/FastCurve implementation. Compare exact base inventories,
counts, point witnesses and substitutions. This checks independent arithmetic;
the group MITM combinatorial routine is shared and separately controlled by
exhaustive cyclic-group tests with repetitions and infinity intermediates.

Also regenerate the declared basis/hash-to-subgroup target, check family
pairing, duplicates and the frozen first-four-negative/first-two-positive
selection. Keep every process status and any mismatch or incomplete file.
Use 900 seconds and 2 GiB address space per file, one thread. A cap is unknown,
never a zero count. The existing b-aware verifier separately recomputes all
finite-chain counts and substitutes SAT witnesses in independent equations.

After generation and replays, the native summary recomputes all classifications
from raw rows, checks input hashes in both replay streams and counts successful
reference rows explicitly. Summaries with missing reference files remain partial.
Group costs are diagnostic; no native physical price or closure-degree result
is inferred. Freeze this auditor/protocol before its first outputs.
