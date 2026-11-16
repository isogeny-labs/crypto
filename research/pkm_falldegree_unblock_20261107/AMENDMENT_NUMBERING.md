# Amendment numbering: branch → preregistration

Two sessions froze amendments to `research/prime_fall_degree_20261027/PREREGISTRATION.md` in parallel on 2026-11-14/15. The
other session's Amendments 15–21 merged into `main` first (PRs #4168, #4173, #4306). This branch's amendments are renumbered
+7 in the preregistration. Files written on this branch before the merge cite the **branch** numbers.

| branch number | preregistration number | subject | draft / evidence |
|--:|--:|:--|:--|
| 15 | 22 | reserved RunPod route (R3) | `AMENDMENT_R3_RUNPOD.md` |
| 16 | 23 | 2-isogeny arm at `P1` (R4) | `AMENDMENT_R4_ISOGENY_P1.md`, `gaux/`, `iso/` |
| 17 | 24 | git storage in place of S3 | decision 12 |
| 18 | 25 | 16 vCPU, 120,000 s limit | `results/ISO/iso-a/LAUNCH.md` |
| 19 | 26 | isogeny `t = 14`, Kummer retries, stage B as two jobs | `results/LAUNCHES_20261115_A19.md` |
| 20 | 27 | stage B G3 rerun with `--jobs 1` | `results/KUMMER/pfd-b-g3b/` |
| 21 | 28 | Kummer `t = 14`, `m = 4`, isogeny null | `results/LAUNCHES_20261115_A21.md` (all 11 lost) |
| 22 | 29 | per-batch balance, 15-min fetches | `results/LAUNCHES_20261115_A22.md` |

Overlap with `main`'s series (independent runs, compared, not merged):
- stage B: this branch `results/KUMMER/`, main `results/RPX/` and others;
- `t = 13` draw 0: both;
- the stage B G3 race: this branch's 27, main's 21;
- `t = 14` cap 6: main only (this branch's attempts were lost, then stopped as duplicates);
- `m = 4`: main has rows, this branch has none.
