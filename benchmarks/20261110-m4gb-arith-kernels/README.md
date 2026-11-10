# M4GB kernel patches: upstream vs patched, local A/B (2026-11-10)

Copies for browsing of
[`research/m4gb_integration_20261110/`](../../research/m4gb_integration_20261110/README.md)
(source of truth: protocol, raw rows, logs, scripts). Benchmark record
`benchmarks/records/20261110T0551Z-m4gb-arith-kernels-ab.json`.

- `REPORT.pdf`: README plus results table.
- `results.svg` / `results.png`: median wall seconds per cell, upstream
  `01860a52` (grey) against patches 0001 + 0002 (blue), one thread, log scale.

Headline `[pcc:unverified]`: random dense `GF(31)` systems 3.4–4.1× (bases
identical), `GF(65521)` 1.6–2.0×, `GF(2)` under exact-width binaries
unchanged, `wd_n16` under a 64-variable binary 1.26×; wide-binary cells
censored by the container's memory limit. Local container shared with a Rust
build; not an isolab run and not a table row in `benchmarks/README.md` (unvalidated:
no `VALIDATION.json`, no replay certificate, no rho reference, because this is
a kernel speed-up of an external Gröbner solver, not an ECDLP cost). The
benchmark record and `benchmarks/progress/` list it.
