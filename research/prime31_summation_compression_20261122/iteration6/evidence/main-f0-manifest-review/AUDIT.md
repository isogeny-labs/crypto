# Root Cargo manifest: inherited failure already repaired on main

## Decision

**No Cargo.toml edit is required at pinned main `f0ca664eca0081a30adfcc79dcf62a32de40d2af`.** Its exact Git blob is `8d6da017040e8edbb2cd8b016a94cb2e62c7d1ca`; Python `tomllib` parses it successfully, with 37 uniquely named explicit binary targets and 56 uniquely named explicit example targets. This is a manifest-text check, not a Cargo build result.

The reported duplicate-key failure was from the old #4928 CI run, not current #4951. The later #4951 head, `f043ca05a338637f830e09fa19ad14d9b18a34ce`, already has the same repaired manifest as main.

## Exact failed run and checkout

- Run: [45100829948](https://github.com/isogeny-labs/crypto/actions/runs/45100829948), created 2026-11-23 02:32:48 UTC; failed job `137451053331`, `original-lock-correctness`. [pcc:unverified]
- The run API records branch head `fc184b03d47e13ec922c9b489f3b3f8a976a8bd9`, branch `lane/prime31-dickson-quotient-gate`.
- The actual checkout log records synthetic PR #4928 merge **`4a40be161b9b3e6bc4db8a8dc16864e82d754d7b`**, merging that branch head into `25ae0df28cb69efc5ec077c4860e2038a451012f`. This checkout identity must not be confused with the run's head SHA.
- Both that synthetic checkout and the branch head have manifest blob `2c5622514938938b7c91c34629d7e2dd877c1d33`, verified by exact API content and local Git-blob hashing.
- The job fails before the native tests run, while Cargo searches ancestor manifests: `Cargo.toml:132:1`, `name = "curve-transfer-sweep"`, `duplicate key`. Python independently reports `Cannot overwrite a value (at line 132, column 30)` on the same bytes.

`source_metadata.json` retains selected read-only API metadata. `checkout-log-excerpt.log` and `failed-job-excerpt.log` retain the relevant exact decoded log excerpts, not the full CI log.

## Cause and already-present repair

The manifest lost three array-table headers. Therefore separate targets were incorrectly interpreted as repeated `name` and `path` assignments in an existing table:

| Missing header | Following target | Incorrectly shares the preceding target's table |
|---|---|---|
| `[[bin]]` | `curve-transfer-sweep` | `hecx` |
| `[[bin]]` | `odd-transfer-catalog` | `hecx` |
| `[[example]]` | `floor_crater_bridge_cost` | `hw_counter_ab_bench` |

These are **different targets**, so deleting duplicate assignments would discard target declarations and is not a semantics-preserving repair.

Commit [69fad1271054225a95d5682f5fdcd11fd2a692d7](https://github.com/isogeny-labs/crypto/commit/69fad1271054225a95d5682f5fdcd11fd2a692d7), dated 2026-11-23 03:23:46 UTC, already adds exactly those three headers, with zero deletions in Cargo.toml. Its parent is `e95645aede904b21421683d917b29b622140da1e`, whose manifest is byte-identical to the failed manifest. Its resulting manifest is byte-identical to the pinned current main and to the observed #4951 head.

The independent audit reconstructs the repaired file by inserting those three headers into the failed input and asserts **whole-file byte equality** with current main. All package settings, dependency specifications, target names, target paths, and other manifest content are preserved. This audit does not assign an earlier introducing merge: it identifies the exact failed state, repair parent, repair commit, and current repaired state without extrapolating from truncated history.

## Reproduction and source identities

Run `python /workspace/scratch/2f841b0b9a19/ci_manifest_audit/audit_manifest.py`. The script reads the two frozen manifests, checks their Git-blob identities, verifies the historical negative parser control and current positive parser result, verifies unique target names, and proves the exact three-line reconstruction. It writes only the local audit receipt. [pcc:unverified]

| Frozen file | Bytes | SHA-256 |
|---|---:|---|
| `Cargo.failed-fc184.toml` | 22049 | `3f559087a5ef7c81357fc3e6a079c365df0ead7cbdc5e6f6710f4a921ec4bfb0` |
| `Cargo.main-f0.toml` | 22077 | `097c73742907c4d710cb0daff8cd0eaca763e260420675d2cc29a6c8f0cbf692` |

No source, lockfile, workflow, test, or remote resource was changed by this audit. No build or cryptographic experiment was run. The next correctness gate must bind its own actual checkout and source hashes; this repaired manifest alone does not establish that the remaining inherited source compiles or that any tests pass.
