# Native read-only accounting reconciliation

The Rust checker reconciles saved retained-coordinate input and output without launching the producer or evaluating curve operations. Existing `retained-duplicate-executed/run.py` already has a once-per-physical-row accounting pass, but its entrypoint first launches the experimental executable. That entrypoint was not run. The private Rust tool preserves its setup-copy equality and total checks, adds archive SHA256 to the identity namespace, checks expected versus observed disposition identities, joins each disposition to its archived case and target identity, and retains null/missing fields.

## Observed saved-file reconciliation

- The archive contains 180 physical rows. The saved index selects 60 rows; the other 120 remain explicitly represented as `archive_row_not_selected_by_index`.
- All 420 indexed case dispositions are present exactly once, with no unexpected identifiers or reported binding inconsistencies.
- Observed dispositions: 218 `collector_ineligible`, 202 `compared`. Adapter dispositions: 174 `first`, 28 `redundant`.
- Setup/constructor metadata copies agree per physical row. Aggregating each selected physical row once exactly reproduces the saved setup totals.
- Full-cohort obligation remains 1,260 targets. Complete cost ratio and natural yield remain null. Archive row counts are not newly completed target runs.

`reconciliation.json` includes every physical archive row, every observed disposition, full source/raw hashes, unselected rows, setup totals and explicit missing cost-category flags. Per-case nulls are not converted to zeros. Ring before/after snapshots remain unmodified; no new aggregate combines overlapping counter families.

## Cost categories still outside this check

The saved counters do not supply a complete denominator for natural relation generation, allocation/hash/comparison/memory work, all primality or square-root search work, input parsing/curve construction/process setup, matched rho controls, or diagnostic counter isolation. Existing field intervals and source scope remain visible; a numeric performance ratio is not generated.

## Reproduction and source basis

Repository instructions read: `AGENTS.md`, especially native implementation and evidence-scope rules; latest materialized journal `docs/session-logs/2026-10-31.md`. Source basis: the historical `run.py` accounting block, `src/pruned.rs::archive_target`, and the preceding source-only C1/C2 contract audit. This is deterministic file accounting, not a new mathematical or performance claim; it reuses the recorded ledger definition rather than proposing a new research method.

```
cargo run --offline --locked --manifest-path Cargo.toml -- /absolute/path/to/retained-duplicate-executed reconciliation.json
```

Inputs are read-only. Only the explicitly supplied output JSON is written. The tool does not invoke producer binaries, shell commands, cloud clients or experiments. Missing/malformed numeric input fails rather than silently supplying zero. No repo files were changed.

The initial build failed because a JSON map iterator yielded `&str` keys instead of owned strings; the compiler output is retained as `attempt1.stderr`. Correcting the key conversion yielded native exit 0. `src/main.rs` was subsequently rustfmt-formatted; the final build/readback is recorded separately. No experimental replay or new measurements were performed.

## Bounded negative accounting controls

Four copies of the saved inputs were altered independently: duplicate a disposition, omit a disposition, alter one repeated setup counter, or change a workload identity. All four executions returned native exit 1 with `reconciliation_pass=false` and explicit errors. The originals were not edited. Controls manipulate JSON only; no group operations or trajectory simulation occur. Receipts and exact altered raw hashes are in `controls/summary.json` and individual control directories.
