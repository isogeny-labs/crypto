# Evidence

- `panel-01/`: library as merged (example binary SHA-256 `c7215853…`). Driver `tools/panel.sh`.
- `panel-02/`: after the block-wise early-exit patch to `decompose_four_fast`
  (example binary `94b7f64e…`). Driver `tools/panel2.sh`.
- Each run has `<name>.json` (stdout), `.stderr` and `.meta.json` (argv, exit status, process
  wall time, binary SHA-256, SHA-256 of the original stdout).
- The reference runs (`*-ref.json`) are crypto #5184's `cost-v2 sample:13:<rep>`
  (binary `a836892e…`). Their `relation_rows` were replaced afterwards by
  `relation_rows_sha256` and `relation_rows_count`, so their `stdout_sha256` refers to the
  unreduced output.
- `summary.json` in each panel comes from `tools/summarize.py`.
- `libtest.txt`: `cargo test --release --lib semaev_projection` on the patched library.
