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
