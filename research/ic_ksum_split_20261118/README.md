# k-sum `split=1` (pair-sum build) and `stride-renew` targets against the pruned incumbent, Table B's eight curves

**Standing: counted cost at L0, `[pcc:unverified]`.** No pcc claim filed; no independent validator report or `VALIDATION.json`
yet, so the validation gate is not met and no leaderboard row is added. Wall time is not claimed. Track: generic decomposition
(bounded by sqrt(r); cannot beat rho or BSGS).

## Outcome

- **Step 1, where the incumbent's cost goes** (`analysis/phases.json`, from the validated prune study's sessions): table build
  29-55 % of S (ksum5-p, w = 0), walk steps 13-55 %, walk set-up and restarts (16 jumps, start, pooled restarts: about 48 scalar
  multiplications a run) 5-47 %, everything else at most 4 %.
- **Lever A, `split=1` (new, commit fc0babc73e): holds on all 8 rows for k = 5 and k = 4** (T1: pooled and every replicate upper
  end below 1 at w = 0, 0.1, 1). `ksum5-ps / ksum5-p` = **0.910-0.943** at w = 0, 0.920-0.950 at w = 0.1, 0.954-0.976 at w = 1;
  `ksum4-ps / ksum4-p` = 0.939-0.965 / 0.945-0.967 / 0.972-0.982. Correctness: 9,216 target pairs (24 sessions x 2 k x 192 runs)
  identical in outcome, log, counters, factor base, every native and work counter and every phase cost except the build adds, which
  fall by exactly `2^{k-2} C(a-1, k-1) - 2 C(a-k+2, 2)` on every run (60-2,568 adds by row and k). The director's prediction
  (R_0 within +/-0.03 of 0.91-0.94) held on all 8 rows.
- **Lever B, `targets=stride-renew` (existing option), with split: `ksum5-psr / ksum5-p` = 0.536-0.866 at w = 0, 0.564-0.881 at
  w = 0.1, 0.716-0.937 at w = 1;** every pooled upper end below 1. The registered rule (every replicate too) holds on 7 of 8 rows
  for k = 5 (fails on B24: one replicate's upper end at w = 1 is 1.011) and 6 of 8 for k = 4 (fails on P24 and B27 at w = 1).
  Stride-renew alone (`-pr`) gives 0.611-0.956 at w = 0: it removes the walk's jump table and pooled restarts (about 42 of 48
  scalar multiplications), the largest gain on the small rows B15/B18 where set-up is half of S.
- **Best against best** (cheaper of k = 4/5 before and after, by mean S): **0.525-0.866 at w = 0**, 0.554-0.881 at w = 0.1,
  0.710-0.937 at w = 1, every 95 % interval below 1.
- **Distance remaining to matched rho-neg (best k-sum / rho-neg):** w = 0 **4.95-5.73 → 2.78-4.97**; w = 0.1 5.08-5.90 → 2.92-5.20;
  w = 1 5.15-6.70 → 3.66-6.28. Split alone: 4.55-5.22 at w = 0. Rho is still ahead on every row at every weight.
- **Controls.** `rho-neg-aa / rho-neg` = 1.0000 on every row and weight. All 46,080 runs verified; 24/24 sessions pass
  `ecbench verify --replay-all --exit-code` (46,080 records, 34,560 replays reproduced, 0 problems; same binary, same host).
- **Not an accounting change.** Split removes group additions the build performed (the (k-1)-summand partial sums) and replaces them
  with a smaller pair-sum table, all counted; stride-renew removes scalar multiplications the walk performed. Both arms priced by the
  same harness. The gains are constant factors that shrink as r grows (fixed set-up against sqrt(r); 1/a of the build); no exponent
  changes.

## Deviations (all recorded in PROTOCOL.md)

- Route: local L0 `--cpus none` instead of isolab-eks (no AWS/S3 in this container: token rejected, checkpoint token expired); no
  S3 checkpoint or raw-data upload; conductor and research-kb unavailable.
- A1/A2: two launch-script fixes before any session ran. A3: the first full launch errored on every binary row because merge
  833a35d21e had dropped `ksum`/`ksum-frobenius` from the binary oracle match (restored in e93c3cb2da); binary rebuilt, full re-run,
  only status counts of the failed launch read. A4: a container restart killed the run with B27-r3 at 928/1,920 records; that session
  was deleted unread and re-run, the 23 complete sessions kept.
- Raw session trees (256 MB) are not committed (disk/size limits); `data/sessions.sha256` pins them, the verify reports are in
  `data/*.verify.json.gz`. They are kept outside Git on the producing host; an independent replay needs them (open thread).
- The Holm 97.5 % reading of the protocol was not computed; the split margins (largest upper 0.977) and the combined failures
  (per-replicate at w = 1) would not change direction.

## Files

`PROTOCOL.md` (registered 55310069ea, amendments A1-A4), `specs/`, `run_all.sh`, `analysis/` (phases, predictions, evaluate,
correctness, summary, plot, render), `results.md`/`results.json` (all ratios), `summary.json`, `data/` (run log, verify reports,
correctness), `ratios.svg/png`, `gap.svg/png`, `report.pdf`.

| row | w | best before (S) | best after (S) | after / before [95%] | gap to rho before → after | ksum5-ps / ksum5-p [95%] |
|:--|--:|:--|:--|:--|:--|:--|
| P20 | 0 | ksum5-p 6.359 | ksum5-psr 4.926 | **0.7746** [0.7569, 0.7926] | 5.02 → 3.89 | 0.9185 [0.9173, 0.9198] |
|  | 0.1 | ksum5-p 7.091 | ksum5-psr 5.660 | **0.7981** [0.7791, 0.8176] | 5.15 → 4.11 | 0.9270 [0.9258, 0.9281] |
|  | 1 | ksum5-p 13.684 | ksum5-psr 12.267 | **0.8965** [0.8714, 0.9223] | 5.77 → 5.17 | 0.9621 [0.9614, 0.9629] |
| P22 | 0 | ksum5-p 6.096 | ksum5-psr 4.965 | **0.8145** [0.7986, 0.8306] | 5.42 → 4.42 | 0.9127 [0.9116, 0.9137] |
|  | 0.1 | ksum5-p 6.835 | ksum5-psr 5.695 | **0.8332** [0.8162, 0.8505] | 5.57 → 4.64 | 0.9221 [0.9211, 0.9231] |
|  | 1 | ksum5-p 13.488 | ksum5-psr 12.265 | **0.9093** [0.8873, 0.9319] | 6.27 → 5.70 | 0.9605 [0.9599, 0.9611] |
| P24 | 0 | ksum5-p 6.015 | ksum5-psr 5.105 | **0.8488** [0.8323, 0.8656] | 5.42 → 4.60 | 0.9183 [0.9172, 0.9194] |
|  | 0.1 | ksum5-p 6.797 | ksum5-psr 5.873 | **0.8640** [0.8464, 0.8819] | 5.62 → 4.85 | 0.9277 [0.9267, 0.9288] |
|  | 1 | ksum5-p 13.837 | ksum5-psr 12.779 | **0.9235** [0.9016, 0.9460] | 6.56 → 6.06 | 0.9645 [0.9639, 0.9651] |
| B15 | 0 | ksum4-p 10.551 | ksum4-psr 5.536 | **0.5247** [0.5165, 0.5331] | 5.30 → 2.78 | 0.9245 [0.9239, 0.9251] |
|  | 0.1 | ksum4-p 11.252 | ksum4-psr 6.230 | **0.5537** [0.5445, 0.5630] | 5.27 → 2.92 | 0.9291 [0.9285, 0.9296] |
|  | 1 | ksum4-p 17.565 | ksum4-psr 12.475 | **0.7102** [0.6953, 0.7254] | 5.15 → 3.66 | 0.9541 [0.9536, 0.9545] |
| B18 | 0 | ksum4-p 9.456 | ksum4-psr 5.338 | **0.5645** [0.5458, 0.5838] | 5.11 → 2.88 | 0.9383 [0.9376, 0.9391] |
|  | 0.1 | ksum4-p 10.201 | ksum4-psr 6.086 | **0.5966** [0.5760, 0.6180] | 5.12 → 3.05 | 0.9426 [0.9419, 0.9434] |
|  | 1 | ksum4-p 16.907 | ksum5-psr 12.644 | **0.7479** [0.7272, 0.7691] | 5.15 → 3.85 | 0.9647 [0.9642, 0.9653] |
| B21 | 0 | ksum5-p 6.436 | ksum5-psr 5.014 | **0.7791** [0.7635, 0.7951] | 4.95 → 3.86 | 0.9195 [0.9185, 0.9205] |
|  | 0.1 | ksum5-p 7.179 | ksum5-psr 5.762 | **0.8027** [0.7858, 0.8199] | 5.08 → 4.08 | 0.9279 [0.9269, 0.9288] |
|  | 1 | ksum5-p 13.865 | ksum5-psr 12.492 | **0.9010** [0.8789, 0.9236] | 5.69 → 5.13 | 0.9626 [0.9621, 0.9632] |
| B24 | 0 | ksum4-p 6.415 | ksum5-psr 5.291 | **0.8247** [0.8040, 0.8458] | 5.26 → 4.34 | 0.9432 [0.9421, 0.9444] |
|  | 0.1 | ksum4-p 7.176 | ksum4-psr 6.151 | **0.8571** [0.8421, 0.8725] | 5.39 → 4.62 | 0.9500 [0.9489, 0.9510] |
|  | 1 | ksum4-p 14.019 | ksum4-psr 13.042 | **0.9303** [0.9106, 0.9504] | 6.04 → 5.61 | 0.9758 [0.9753, 0.9764] |
| B27 | 0 | ksum5-p 6.055 | ksum5-psr 5.246 | **0.8665** [0.8522, 0.8809] | 5.73 → 4.97 | 0.9103 [0.9094, 0.9112] |
|  | 0.1 | ksum5-p 6.812 | ksum5-psr 5.998 | **0.8805** [0.8653, 0.8960] | 5.90 → 5.20 | 0.9203 [0.9194, 0.9211] |
|  | 1 | ksum5-p 13.622 | ksum5-psr 12.761 | **0.9368** [0.9174, 0.9566] | 6.70 → 6.28 | 0.9601 [0.9596, 0.9607] |

Full per-arm tables: `results.md`.
