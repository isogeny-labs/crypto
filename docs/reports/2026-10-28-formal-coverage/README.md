# Frozen crypto Lean path inventory

This is a **syntactic navigation aid** at crypto main commit
`a5db1413a772ff2250aad38c5c05f6715647a6d5`. It does not certify any
experiment, theorem statement, Lean build, code path, or performance result.
The [machine inventory](inventory.json) binds every row to the frozen Git tree;
the [report source](report.typ), [graph](coverage.svg), and [inspected PDF](report.pdf)
present its scoped observations.

The metadata script [formal_coverage_inventory.py](../../../tools/formal_coverage_inventory.py)
uses complete `git ls-tree -r -z` and batched Git blob reads, so a sparse
checkout or truncated GitHub recursive API cannot silently omit tracked files.
It records 165,588 blobs and one unexpanded Git submodule entry at
`verification/pqc-assay/upstream`. It enumerates 1,355 tracked study directories:
997 under `research/`, 30 under `experiments/`, and 328 under `benchmarks/`.
There are 242 tracked `.lean` files, 229 under those three roots; 57 study
directories contain a co-located Lean file. The 4,192 `theorem`/`lemma` pointers
are regex hits with path, line and blob ID, not parsed Lean declarations.

The remaining 1,298 study directories are **not** 1,298 missing proofs. Many
benchmark and operational directories have no mathematical claim to formalize,
and a proof may reside in a shared module or another repository. Claim fidelity
requires reading each written claim and theorem statement with its hypotheses,
building the pinned target, auditing axioms, and reviewing the experimental
evidence. No leaderboard or performance dashboard changed because this
inventory adds no new measurement or verified result; the new graph describes
only repository paths.

## Reproduce

```sh
python3 tools/formal_coverage_inventory.py \
  a5db1413a772ff2250aad38c5c05f6715647a6d5 \
  docs/reports/2026-10-28-formal-coverage/inventory.json
python3 docs/reports/2026-10-28-formal-coverage/render.py
```

Compile `report.typ` with Typst for the PDF. `render.py` is the editable graph
source and reads the frozen JSON. These scripts inspect Git metadata and draw
the report; they do not run cryptographic research or performance measurements.
