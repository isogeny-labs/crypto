# R92 independent reproduction of the rank-32 multiplication table

R92 recomputed the unit and all 528 unordered products of the accepted R50
rank-32 module in the source field. Its native run passed the strict gate.
From a fresh raw readback, an ideal-free F2(u) replay checked the full
table, 16,896 polynomial coordinates and 528 corrupted-coordinate controls;
a separate source-field replay recomputed all 528 actual products. A
post-run comparison found exact equality with all 528 pair labels and
16,896 coefficients of the previously accepted R50 table.

R50 had already established the multiplication table. R92 is a second
construction and exact cross-check, as the scope erratum records. The
diagram shows the two production paths and their equality check; no
timings or performance comparison were taken. Source revision
`a58703742c8cdd4a91beb0eb1bc78c7eb8cb0c51`, seed 92, one
Singular/FLINT thread, 4 GiB virtual memory and 1200-second cap. The
18-member raw archive passed checksum-protected upload, fresh readback,
and all member hashes.

The next construction is an explicit cocycle between the two R91 affine
charts using this algebra table, followed by actual boundary normalization
and nonzero modular-factor action. Evidence: `../CM.md`,
`../SCOPE-ERRATUM.md`, `../results.json`, three replay/comparison logs,
and the raw archive digest recorded in the evidence note.
