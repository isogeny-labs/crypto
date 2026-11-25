# Picard p31 full-graph index calculus

This directory publishes the source used for a full-graph relation arm on the
plane quartic transfer of the public (p=2^{31}-1) instance. The graph arm
recovered a factor-base kernel over the selected prime subgroup and checked
the derived scalar on the original genus-four Jacobian. The factor base and
the six input split points were inherited from the earlier triangle-bootstrap
campaign. Thus the source-curve check is end to end for the graph relation and
linear-algebra arm, while the timing is **not** a cold independent graph setup
or a demonstrated speedup.

## Method and prior art

The relation construction follows the double-large-prime graph method for
plane curves in [Diem, *On the discrete logarithm problem in elliptic curves*](https://jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf)
and the [time-memory variant](https://eprint.iacr.org/2014/346.pdf).
The implementation peels the graph to its two-core, suppresses degree-two
paths into a weighted kernel, chooses a minimum-chain-length spanning forest,
and extracts fundamental cycles. It connects residual large-prime pivots by
multi-source Dijkstra and a Kruskal forest on Voronoi boundaries. These are
standard graph algorithms applied to exact source-line coefficients; no
minimum-cycle-basis or new asymptotic claim is made.

## Checked result

The original graph had 299,945,714 retained two-large-prime edges and
37,445 formal cycle edges. The 26,000 shortest fundamental cycles replayed
from original line ordinals yield 4,299 directly balanced rows and 21,701
residual-pivot cycles. The connector forest selects 21,698 joins, producing
**25,997 full factor-base rows**. The first 24,999 rows gave an invertible
24,999-square system after fixing factor-base column zero to one. Native FLINT
solved it in 427.28 seconds external wall, peaking at 8,604,336,128 bytes
maximum RSS. The resulting vector has 25,000 entries.

`verify_all_rows.py` independently checked all 25,997 full rows and their
79,642,974 nonzero coefficients, including 998 rows outside the solve input.
`verify_source_scalar.py` checked the factor-base file and transfer parameters,
then tested the derived scalar on the original source Jacobian. The source
group equation holds for scalar **81985529216486895**. The row and Sage receipts are
[`all_rows_26000_receipt.json`](all_rows_26000_receipt.json) and
[`source_scalar_receipt.json`](source_scalar_receipt.json). The kernel hash is
`1c2df63b8659d98d5244c2c41ede4c9544ba593f1e236e22b86f08c67a9c6a02`.
An independent pure-Python Cantor composition and reduction check reached
the same scalar without Sage or a fixture scalar; its receipt is
[`source_integer_receipt.json`](source_integer_receipt.json).

## Reproduction and data custody

The three subdirectories preserve the exact source layout used in the run.
The large binary edge stream, compressed core, JSONL rows, and 4.7 GB solve
matrix are retained outside Git under `/Volumes/SSD990-2/research_notes/`.
[`receipts/setup.json`](receipts/setup.json), the row and matrix manifests,
and the full-row receipt bind their checksums and dimensions. The 490 KB
[`graph_kernel_24999.json`](receipts/graph_kernel_24999.json) is included.

On a machine with the retained data and Sage, the independent checks are:

```sh
python3 tools/ic/p31_graph/verify_all_rows.py /Volumes/SSD990-2/research_notes \
  --kernel tools/ic/p31_graph/receipts/graph_kernel_24999.json
sage -python tools/ic/p31_graph/verify_source_scalar.py /Volumes/SSD990-2/research_notes \
  --kernel tools/ic/p31_graph/receipts/graph_kernel_24999.json
python3 tools/ic/p31_graph/verify_source_integer.py /Volumes/SSD990-2/research_notes \
  --kernel tools/ic/p31_graph/receipts/graph_kernel_24999.json
```

The producer stages are `emit_edges.cpp`, `xor_core.cpp`,
`compress_kernel.cpp`, `kernel_cycles.cpp`, `stream_cycle_batch.py`,
`connect_pivot_forest.cpp`, `stream_connectors.py`,
`build_dense_solve_input.py`, and `solve_factor_base_flint.cpp`. They retain
their original directory-relative input layout. C++ stages need a C++20
compiler; the final solve needs FLINT. The replay uses the original audit
arithmetic. The full raw dataset is too large for Git; generating it starts
from the hash-bound `input25k.txt` and `fb25k.bin` in the retained workspace.

The graph collector and filtering stage alone took 232.79 seconds in the
earlier receipt. Two-core extraction, cycle replay, pair replay, matrix
materialization, and dense solve add substantial setup cost. A cold
same-target comparison with optimized rho has not been measured for this arm.
The remaining performance work is a sparse linear-algebra implementation
and a fully charged cold setup and target-descent comparison.
