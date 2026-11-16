# Cover-image factor bases, round 6: seed replication and the censored m = 4, t = 4 arms (2026-11-13)

**Question.** Rounds 1–5 used one curve/tower/target draw per arm and left `quart` and `generic3`
censored at m = 4, t = 4. Do the ratios hold across fresh draws, and do the censored arms change
the verdict?

**Answer** [pcc:unverified]:

- **W is fixed by the system's shape.** In every cell of set S, the widest-matrix width W is
  identical across all five seeds, for PKM's tower and for every cover map: zero variance across
  different curves, towers and targets. The single-draw ratios of rounds 1–5 are therefore exact,
  not sampling accidents. W cannot see a curve- or target-specific effect.
- **m = 3, t = 4: closed for all four maps** (W 1.55–1.99× PKM's tower in every seed; 0.96–1.00×
  their own random-coefficient null).
- **m = 4, t = 3: mixed for all four maps.** W is 0.986–0.990× PKM's tower, exactly, in every seed.
  Round 7 then timed this cell and found every cover map 1.26–1.42× slower
  ([`../cover_image_fb_r7_mixed_20261113/`](../cover_image_fb_r7_mixed_20261113/README.md)).
- **m = 4, t = 4: `quart` closed** (W 245,911 = 1.466× PKM). PKM's tower at seed 202 has
  W = 167,726, the same as at the default seed. `generic3` (default seed) and `cubic` (seed 202)
  are **censored**: both were still inside F4 when the 7.5 h hard stop deleted their pods.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), pushed in `9ae08bef32` before any cell.
- Frozen binary of rounds 2–5 (SHA-256 `923a1c95…`, matched on every pod). RunPod `cpu3m` pods,
  32 vCPU, 256 GB, $1.76/h: one for set S (15:23–16:49Z) and four for set L (15:23Z until each arm
  ended or the 22:53Z hard stop). All five pods deleted and confirmed gone.
- Tables: [`cells_S.tsv`](cells_S.tsv) ([`tabulate_S.py`](tabulate_S.py)). Rows and logs:
  `runs/S/` and `runs/<pod name>/`. 0 planted violations in any finished system.

## Set S (5 seeds × 5 maps)

| cell | map | W / PKM (each of 5 seeds) | W / own null | gate |
|---|---|---|---|---|
| m = 3, t = 4 | quad | 1.988 | 0.997 | closed |
| m = 3, t = 4 | cubic | 1.617 | 1.000 | closed |
| m = 3, t = 4 | quart | 1.549 | 0.956 | closed |
| m = 3, t = 4 | generic3 | 1.613 | 0.997 | closed |
| m = 4, t = 3 | quad | 0.990 | – | mixed |
| m = 4, t = 3 | cubic, quart, generic3 | 0.986 | – | mixed |

PKM W across seeds: 12,314 (m = 3, t = 4) and 21,619 (m = 4, t = 3), every seed.

## Set L (m = 4, t = 4, one arm per 256 GB pod)

| arm | W | ratio | F4 time | status |
|---|---|---|---|---|
| PKM, seed 202 | 167,726 | reference | 4,834 s | complete |
| `quart`, default seed | 245,911 | 1.466× PKM (round 5 reference) | 18,714 s | **closed** |
| `generic3`, default seed | – | – | > 7 h 29 m | censored |
| `cubic`, seed 202 | – | – | > 7 h 29 m | censored |

Round 5's `cubic` at the default seed had W = 245,930 (1.466×). `quart` matches it to 0.01 %.

## Launch errors (no data affected)

- The set-S binary upload failed (HTTP 404), and the first launch ran with no binary: every arm
  exited 127 and produced nothing. After a SHA-256-checked re-upload the set was relaunched; the
  first launch's empty files are kept on the pod's `failed_launch/` only.
- `pod_run_S.sh` used `xargs -L 1`. For PKM arms the empty `--cover-map` left a trailing space,
  which `xargs` reads as a line continuation. Each S3-none line swallowed its S4-none line and the
  following S3-quad line, so 15 of the 50 cells never ran (exit 101). They were rerun one command
  each (`pod_run_S_none.sh`, `pod_run_S_quad3.sh`). All 50 cells are now complete and valid.
- An accidental local run of `pod_run_S.sh` created an untracked `runs/` directory of failed starts
  in the repository root. It was never committed and was deleted.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. A stage
diagnostic, counts only.
