# Living leaderboard and browser provenance repair

The official generators refreshed four living outputs from the existing frozen inputs at HEAD `55da3adf5dc96be86687583c5ae967a686bc1a09`. No research run, measurement, ranking promotion, source edit, frozen evidence rewrite, sparse-pattern edit, staging, or commit was performed. Root owns Git and integration.

The leaderboard JSON has **one changed leaf**, `/sources/curves_yaml/sha256`: `ab4891397fe07fca9478d96e1533c167a885b7557be9e6801025d14fec0976c5` → `658a422ce97ef3ea505ed6337fe4680e146f1b951b208688a13f9199bbdd5712`. Every numeric/boolean value, array length, board row, leader, oracle, roster record, exponent, and diagnostic is identical. Markdown changes only the corresponding 16-character digest; HTML changes only its 12-character digest. The official generator was used; generated files were not hand-edited.

The browser's initial freshness check failed. Before applying a repair, official generation to `/tmp` established **one raw changed leaf**, `/sources/docs~1ic~1leaderboard.json`: `49c52128766a7641ce353bae63b4422ee1b8b5caf0dcf19e8011cc77795debc1` → `96387b63f1d93e3cf7cc01b10a73937e0951b643b6d59b52f4b4a0dde238844e`. There are zero numeric, array-length, candidate-mention, model, or measurement changes. All counts remain: 2,572 curves; 211 methods; 672 factor bases; 271 candidates; 30 rounds; 226 sessions; 14,648 yields. Parent then authorized `docs/browser/data.json`; official living generation matches the reviewed candidate byte-for-byte.

This is a provenance chain from the campaign's existing crosswalk alias append: base `b7cafe04485eb97d873dffef698e7337cfb38cfe` has crosswalk digest `ab4891397fe07fca9478d96e1533c167a885b7557be9e6801025d14fec0976c5`, while captured HEAD has `658a422ce97ef3ea505ed6337fe4680e146f1b951b208688a13f9199bbdd5712`. Regeneration updates only that source digest in the leaderboard and the resulting leaderboard-file digest in the browser. It introduces no scientific admission or new data record.

Input completeness was checked before generation: 29 declared sources, 181 exact `r*-candidate.price.json` files across all 36 M1–M4 directories, six v3 parameter files, and the ledger total **217 evidence inputs**, plus two builder source files. Every byte matched its captured HEAD Git blob. Twenty-one missing tracked files were restored by exact `git show HEAD:path`, exclusively creating absent files. Existing files were not overwritten. The full committed candidate glob equals the materialized filesystem glob, including a retained retry; no sparse set was silently omitted. All 219 hashes were rechecked after validation. Exact paths, Git blobs, SHA256s, and sizes are in `input-materialization.json` and `.tsv`.

| Command receipt | Exit |
| --- | --- |
| `build` | 0 |
| `builder-check` | 0 |
| `path-selftests` | 0 |
| `path-check` | 0 |
| `browser-check` | 1 |
| `browser-candidate` | 0 |
| `browser-build` | 0 |
| `browser-check-final` | 0 |

Path self-tests passed **9/9**. Workflow path coverage passed for both pull requests and pushes and reported all **217** builder input files covered. Final browser freshness passed. Checked source hashes are retained separately; parent owns concurrent checker changes and final integration checks.

Preserved setup failures: Conductor is unavailable (exit127). The first metadata preflight incorrectly assumed five candidate reports per directory and stopped before writing; the exact official glob includes a committed retry. That assumption was removed and complete tracked/glob equality verified. The initial browser failure is retained alongside the successful downstream repair. These are integration observations, not experiment results.

`semantic-delta.json`, `browser-semantic-delta.json`, `rendered-delta-check.json`, `provenance-chain.json`, output hashes in `validation.json`, and before/candidate snapshots make the repair reviewable. Commands, stdout, stderr, and exit codes are retained in this directory. Only the four living generated outputs differ from the provided backups; materialized inputs remain byte-identical to HEAD. Frozen campaign integrity is independently checked by root.
