# The ladder panel: wwalk C = 2 and k = 6, 7 sum tables on the 36-48-bit prime ladder (prepared 2026-11-15)

Status: **prepared, not run.** No evidence run has been made; the only runs are the toy-size smoke test and the Step 0
replays described below. `[pcc:unverified]` throughout.

Two leads of the Idea Generator (2026-11-15), measured in one panel, one binary, one ecbench session per size:

- **L1 (IDG-20261115-C2-LADDER):** does the frozen autolab wwalk C = 2 recipe (`g=1,grow=2,seed=run`), measured at [pcc:unverified]
  1.8-1.9x matched rho-neg at k = 24-32, hold on the validated 36, 40, 44, 48-bit prime ladder, where the k = 5 sum table
  stands at 9.7-11x?
- **L2 (IDG-20261115-KSUM-K67):** do k = 6 and k = 7 sum tables beat k = 5 there? Code: `src/cryptanalysis/ic_boundary.rs`
  (packed index widened for k > 5, `KSUM_MAX_K = 9`), exposed as `ksum:m=6..9` wherever `m = 3..5` were.

Track: generic decomposition, bounded by sqrt(r); cannot beat rho or BSGS. Gap to the matched rho is the distance remaining. [pcc:unverified]

## Files

| file | purpose |
|:--|:--|
| [`PROTOCOL.md`](PROTOCOL.md) | claims, predictions, arms, seeds, controls, kill criteria, cost and pod estimate (draft; freeze before data) |
| `curves.json` | the four ladder curves and the k = 32 positive-control curve (copied from committed specs) |
| `make_specs.mjs` | `train`, `test`, `control` specs (`TOY=1` for the local smoke test) |
| `select.mjs` | selection rule for ksum6/ksum7 base sizes from the train sessions |
| `pod/build.sh`, `pod/run.sh` | build ecbench on a plain Linux CPU box; run train, select, test, control, replays, analysis, SHA256SUMS |
| `analysis/evaluate.mjs`, `analysis/memory.mjs`, `analysis/model.mjs` | registered tests; peak RSS and wall per arm; the derived cost model |
| `step0/` | identity replay of committed k <= 5 sessions before and after the code change |
| `smoke/` | toy-size end-to-end output (non-frozen seeds; never evidence) |

## Code change (L2), minimal

- `src/cryptanalysis/ic_boundary.rs` (k-sum code only): `KSUM_MAX_K` 5 -> 9; new `ksum_index_bits(k)` = 12 for k <= 5 (the layout of
  every recorded table, so the change is a no-op there; `step0/RESULT.md`) and `64/k` above (10, 9, 8, 7 for k = 6..9), stored per
  table; the base-size check names the width ("fewer than 2^bits points"); `KSUM_MAX_ENTRIES = 2^31` refuses a table that could not
  fit in memory; `saturating_sub` in the entry-count estimate (no underflow for a base smaller than k); unit tests in
  `ksum_wide_index_tests` (layout, probe of sums for k = 3..9, refusals).
- `ecbench` wiring: `ksum:m=6..9` is accepted by `ic.pipeline` wherever `m = 3..5` was (no parser change: the oracle takes `m` from
  the spec); `ic.known_log_ksum_v1` (`known_log.rs`) takes `k = 3..9` with the same width rule; help text and one test updated
  (`methods.rs`). `docs/ecbench/README.md` still says `k = 3, 4, 5` (outside the files this preparation may edit).

## Reproduce (pod)

```sh
pod/build.sh /path/to/source.tgz /work          # source.tgz: git archive of the commit under test
pod/run.sh   /work/study /work                  # study dir = a copy of this directory; outputs in /work/out
```
