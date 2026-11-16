# Frozen local evidence

`runs-local-20261023.tar.xz` contains the complete original 33-run `runs/`
tree, including failed setup and cap attempts. `SHA256SUMS` pins it.
`verify-recorded.sh` expands it in a fresh directory and independently
recomputes the entire `report-data.json` with native Rust.

`check-logs-raw-20261025.tar.xz` preserves byte-for-byte test/build logs and
the masked CI launcher-denial log before display formatting. Four browsing
copies remove only trailing spaces or blank lines at EOF so the repository
whitespace check remains useful; their original bytes are in this archive.
No native run record or sealed ecbench file is normalized.

`runners/` preserves frozen runner copies, including the failed first
sparse-input batch and its preflight-checked restart. Individual native
runs record actual source and binary hashes; later source changes are not
silently assigned to earlier binaries. `browser/` records the static preview
checks at 1280/390/320 pixels. Managed Chromium blocks local file URLs, so
the same canonical page was served on loopback HTTP without external
requests; no browser policy was changed.

All PDFs in `reports/INSPECTION.json` were opened page by page after
rendering. Native operation observations, incomplete cost bands, physical
timings and historical extrapolations remain separate.

## Correction and external-CAS checks, 2026-10-25

`check-logs-correction-20261025.tar.xz` retains the exact original native
fixture-audit tests, pricing-audit lint check, strengthened CAS tests/lint
and archive replay, site tests and dashboard generation check. Its adjacent
SHA-256 file verifies it. The browsable fixture test log drops only blank
lines at EOF; the raw archived log is unchanged. Native results and sealed
records are never normalized.

`runs-external-cas-20261025-01.tar.xz` is the complete separate CAS run tree
and launch provenance, with deterministic packing and an adjacent SHA-256.
It does not replace or modify the first 33-run archive. Run the study's
`verify-external-cas.sh` to re-enumerate every equation and returned basis.
