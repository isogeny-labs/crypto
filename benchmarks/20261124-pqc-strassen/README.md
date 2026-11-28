# 20261124-pqc-strassen

Copies for browsing; the source of truth is
[`research/pqc_strassen_20261124/`](../../research/pqc_strassen_20261124/) (raw timings, harness,
`analyze.py`), regenerated from there. Record:
[`benchmarks/records/20261125T0508Z-pqc-strassen.json`](../records/20261125T0508Z-pqc-strassen.json).

Shared `src/pqc/strassen.rs` product (PR #5172) against each toy scheme's old loop, and Strassen
against the shared schoolbook loop for n = 36 to 512, over GF(2), GF(16), GF(256) and
F_31[x]/(x^3 - x - 1). One thread, Rust 1.99.1 release, shared macOS host under load (20 to 43),
not isolated.

**Unvalidated: no `VALIDATION.json`, so there is no row in `benchmarks/README.md`** (validation gate).

- `REPORT.pdf`: summary page with the full tables, then both graphs.
- `regression.svg` / `.png`: old loop time / new time per field and shape.
- `crossover.svg` / `.png`: Strassen (cutoff 32) time / schoolbook time against n.
