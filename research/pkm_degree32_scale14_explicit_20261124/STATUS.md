# Explicit 14-bit panel status — 2026-11-24

The size-only phase is complete. All sixteen explicit EFD curves reconstructed successfully, every preflight reproduced its frozen interval/candidate size, and the append-only selection record fixes curves 3, 13 and 16. Each selected curve has 16 lifting abscissae in both arms. No timed record or performance output has been written.

Current store SHA-256: `3826d4f143b7645775f10d128f9941a35087968f4f5010e22a32a0d4bc995133`.

Timing is intentionally deferred because concurrent source compilation and full-rank `ic bench` processes were consuming multiple cores after selection. Starting the mirrored panel under that load would violate the frozen stability qualification. Resume only after those processes finish:

```bash
python3 research/pkm_degree32_scale14_explicit_20261124/run.py \
  --bin /Volumes/SSD990-2/crypto/target/release/ic \
  --run-timed
```

The runner will append a host-load receipt before the first timed call and will refuse to proceed if its process snapshot still contains cargo, rustc, another `ic bench`, or the full-cost lane. It then resumes interval/candidate/candidate/interval for each selected curve without rewriting the completed preflights.

## Binary custody update

The frozen executable SHA-256 is
`06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59`.
Its recorded path, `/Volumes/SSD990-2/crypto/target/release/ic`, now contains a
different executable (SHA-256
`3c22886c1cd1b73fe7a5679890fd0c85acae9a1f7b20a2930f2f538079aefd6c`).
The frozen executable was not found in the retained source-replay target or
the lane worktree. Therefore the command above is historical and cannot be
resumed from the current workspace without violating the binary-hash check.

Do not rebuild and continue this store: the evidence binary did not embed a
source commit, and the source-pinned replay produced a different candidate
factor-base size. Keep the completed selection record immutable. A successor
panel must freeze a source commit and build recipe first, regenerate explicit
curve preflights under that executable, then select three equal-size curves in
a new append-only store.

## Source-pinned successor, 2026-11-25

The retained `ic` from commit `a5503f4bb66ac3aa4eac3125fba01534d2574096`
was screened against the frozen 16-curve roster in a new store. Only indices
6 and 13 matched factor-base sizes. A pre-timing amendment extended the same
deterministic enumeration; index 36 became the third match. Selected indices
are 6 (17 abscissae), 13 (16), and 36 (19). The first index-6 control timing
was terminated after unrelated Rust compilation began during the call; the
attempt and process snapshot remain in the append-only store. There is no
qualifying timed result yet. The store's current SHA-256 is
`ddab40eb3660da1fd75f282ebf05e17d74d28ae181459ed972dcebd2239eea47`.
The successor store has no measured `s_over_rho` yet.
In-call load polling now censors any future competing process. The stored
`amended_selection.last_screened_index=37` is an off-by-one metadata error;
`selection-correction:1` records the actual last screened index, 36, without
rewriting the raw store.
