# Formal checks for the `ic` install/update quality work

This directory contains Lean 4 proofs for the pure arithmetic and selection
logic added or hardened by the `ic` macOS install/update work. It is a
specification verification, not a proof of the whole Rust or Python program.

Run:

```sh
./verify.sh
```

The script checks every file with Lean 4, rejects `sorry`/`admit`/errors, and
writes `proof_check.log`. Each file ends with `#print axioms` so the log shows
that the theorems do not depend on `sorryAx`.

## Files and implementation correspondence

| Lean file | Proved statement | Implementation |
| --- | --- | --- |
| `PairCursor.lean` | The resume cursor advances by one per candidate; the last candidate of row `i` and the first of row `i+1` have the same count; the prefix is monotone. | `ecc2k130/codegen/indexcalc_fixed.py`: `consideredCount`, `checkPairs`, `buildPairs` |
| `VersionSelection.lean` | Three-component lexicographic order is irreflexive, asymmetric, transitive and total; `isNewer` is exactly that order. | `src/bin/ic/update.rs`: `parse_version`, `is_newer` |
| `AssetSelection.lean` | A universal archive wins when present; otherwise the architecture match is chosen; no match returns `none`. | `src/bin/ic/update.rs`: `choose_asset` |
| `InstallLayout.lean` | For `<prefix>/bin`, the share tree is `<prefix>/share/cryptocli`; the binary directory is never renamed. | `src/bin/ic/update.rs`: `layout`; `src/bin/ic/install.rs` |
| `DiskBudget.lean` | The disk estimate includes the 32 MiB overhead, is at least the raw planned bytes, and is monotone in planned work and bytes per pair. | `ecc2k130/codegen/indexcalc_fixed.py`: `requiredBytes` |

## Bridges from the model to the code

- `PairCursor.lean` models the cursor recursively. `test_cursor_closed_form_matches_recursive_model` in `ecc2k130/codegen/test_indexcalc_fixed.py` exhaustively checks that the closed form used by Python equals the recursive model for every cursor with `size <= 30`.
- `DiskBudget.lean` uses the same floor-division expression as `requiredBytes`; `test_disk_estimate_matches_lean_model` checks the implementation against the expression over boundary and representative values.
- `VersionSelection.lean` abstracts the parsed tuple. The Rust test `versions_compare_exhaustively_as_tuples` checks `parse_version`/`is_newer` against the tuple order over a 5×4×4 cube, and the invalid-string tests cover rejection.
- `AssetSelection.lean` abstracts the archive/universal/architecture booleans. The Rust test `selects_the_native_or_universal_archive` checks the exact name filtering (including `SHA256SUMS`, `.zip`, Linux and non-`crypto-cli` assets).
- `InstallLayout.lean` models paths as segment lists. `layout_uses_the_prefix_share_tree_only_for_bin` checks `/usr/local/bin` and a non-`bin` target directory; `install::tests` checks the source/installed fallback.

## Non-claims

- This does not prove the Python field arithmetic or SQLite transactions.
  Those are covered by differential tests such as
  `test_field_multiplication_matches_reference_algorithm` (500 random pairs
  against the pre-change bit loop),
  `test_batch_inversion_and_addition_match_scalar_paths` (500 random pairs plus
  every doubling on a small curve) and
  `test_batch_addition_matches_on_ecc2k130_sample` (64 pairs at degree 131).
- It does not prove the whole `ic` CLI or the release workflow.
- Rust's `u64` parser is checked by exhaustive bounded tests, not by a
  refinement proof to the Lean string parser.
