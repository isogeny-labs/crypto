# Point-driven pencil degree-peeling: copied visual record

The [study](../../research/pencil_sparse_20261022/REPORT.md) is the source of
truth. Exact degree peeling solved and source-verified 60 targets on each of
nine original pilot curves. At `M + 0.5S + 10I` and 0.1 GAE per native logical
operation, peel/online is 0.269806–0.898244 and peel/rho is
11.615357–1,204.654657 across those nine 60-target slices. Every slice has
24 of 36 price cells below the earlier online eliminator and 12 above; none
is below matched rho. A separately replayed p=48,383 first target costs
0.233555× online and 22,644.843832× rho; it is one target, not a curve
average. The original 1,260-target requirement remains open: 543 distinct
source targets solved and 717 still open. Three degree-peeling 32-bit
coefficient-cap attempts have no candidate/rho price. A separately frozen
[Markowitz-priority study](../../research/pencil_markowitz_20261022/REPORT.md)
reached a fourth 32-bit coefficient cap; it also has a null candidate/rho
ratio and does not add a distinct solved target. These are complete
cold logical-cost-model prices for completed targets; isolated wall time and
a fastest-external-tool runtime comparison were not measured.

The copied [cross-lead results PDF](results-report.pdf) contains the paired
graphs. Six [run PDFs](run-nine-curves.pdf) are copied as
`run-*.pdf`, including the upper-size cap failures. Every graph in those PDFs
has an SVG and a PNG here; the principal views are
[nine-curve cost](peel-nine-curves.svg),
[native-price cost](peel-nine-native.svg),
[p=28 price band](p28-price-band.svg), and
[cross-lead paired prices](review-pencil-ratios.svg).
Each PNG was rendered from its copied SVG with
`rsvg-convert -d 150 -p 150`; [COPY_MANIFEST.json](COPY_MANIFEST.json) pins
the exact source and copy hashes.

The [local validation gate](VALIDATION.json) pins the original independent
replay certificate and matched-rho price band. The copied
[validator report](validator-report.json) changes only six raw-data paths to
refer to the parent pencil study from this directory; its source hash and
unchanged headline calculations are recorded in the copy. The `results`
symlink resolves the other raw paths back to the study. Check with
`python3 scripts/validation_gate.py evidence benchmarks/20261022-pencil-degree-peeling`.
