# Lossless post-run metadata adapter

This separate Rust program packages the terminal main-panel records. It never
executes a solver, certificate replay or primary timing analysis. The frozen
`panel-v2` analysis remains the source of every summary. CSV numeric tokens and
the original status JSON use `serde_json/arbitrary_precision`; output readback
must preserve those values. The report table only formats the existing summary
to six decimal places for seconds and three for ratios. Full precision stays in
the original CSV and unified export.

The adapter requires all 360 accepted cells, all point controls, 295 passing
post-call hashes and an explicit observed supervisor exit of zero. Canonical
IDs are `case-seed`, replicate is the collector round, and only round 0 is a
warmup. Raw native `u`, replicate 0 and warmup false are retained separately.
`counted.completed_calls=1` represents the frozen primary metric for each cell.

Partial/failed native records are preserved. This complete-panel adapter refuses
them: export-v1 requires `attempts>=1` and recomputes planned flow from its rows,
so it cannot truthfully encode an unattempted cell. It does not fabricate
attempts or omit the original analysis/schedule. A failure to produce the
canonical export is a packaging gap, not permission to rerun the main panel.

Arguments: `ROOT STUDY NEW_OUTPUT UTC EXPORTER_SOURCE`. The output's existing
parent must resolve outside `ROOT/main`, including `..` and symlink aliases.
A new `.table.tex` sibling accompanies the export. Every output uses exclusive
creation. Qualified source, build and unit receipts are retained separately;
this draft source was prepared during main execution and is built/tested only
after that supervisor becomes terminal. Local source/content checks do not
authenticate the execution history.
