# Evidence

- `panel-02/` (reported): 10 interleaved repetitions of pair table with 1 thread
  (`pt1`), #5184 reference (`ref`), pair table with 4 threads (`pt4`), library default
  (`default`) and equal-engineering rho (`rho`). Driver: `tools/panel.sh` with
  `PANEL=panel-02`. Example binary `277d00d2…`.
- `panel-01/`: the same panel before the probe loop deferred ordinates (binary
  `2180f389…`). Only the summary and raw runs are kept.
- Each run has `<name>.json` (stdout), `.stderr` and `.meta.json` (argv, exit status,
  process wall time, binary SHA-256, SHA-256 of the original stdout). In the `*-ref.json`
  runs, `relation_rows` was replaced by its SHA-256 and row count afterwards.
- `summary.json` comes from `tools/summarize.py`.
- `libtest.txt` is the receipt for the module's unit tests.
- `sources.sha256` lists the source hashes.
