# VERIFY: FOLLOWUP-selector-extra-syzygies (skeptic pass, 2026-11-12)

All statements are `[pcc:unverified]`. I edited no file except this one.

## What I re-ran

All commands were run from `research/lfd_lower_bound_20261111/`, single-threaded.

1. **`syz` rebuilt from source** into the scratchpad
   (`gcc -O2 -march=native -o $S/syz out/lead_A1_syzygies/syz.c -lm4ri -lm`). This checks that the
   committed binary is not the only source of the numbers.
2. **An independent code path:** `./emin nullity E` (emin.c, unmodified). It builds the full
   homogeneous matrix on all `2^k` points and reports columns minus rank. Its kernel
   construction is separate from syz.c's.
   - `./emin count` = 0 (unsatisfiable) for every instance below.
3. **e_min for two seeds missing from the committed files:** `./emin solve 4 100000000 SEED` for
   n = 23 seed 11 and n = 25 seed 7.

| n | seed | e | emin nullity (mine) | syz kernel_dim (my rebuild) | rank T | excess | RESULT.md |
|--:|--:|--:|--:|--:|--:|--:|:-:|
| 15 | 1 | 2 | 8 | 8 | 8 | 0 | match |
| 15 | 1 | 3 | 146 | 146 | 29 | 117 | match |
| 15 | 6 | 2 | 8 | 8 | 8 | 0 | match |
| 15 | 6 | 3 | 147 | 147 | 29 | 118 | match |
| 17 | 1 | 3 | 87 | 87 | 37 | 50 | match |
| 17 | 8 | 2 | 9 | 9 | 9 | 0 | match |
| 17 | 8 | 3 | 88 | 88 | 37 | 51 | match |
| 19 | 4 | 3 | 46 | 46 | 46 | 0 | match |
| 19 | 4 | 4 | 1164 | 1164 | 130 | 1034 | match |
| 19 | 7 | 4 | 1165 | 1165 | 130 | 1035 | match |
| 21 | 3 | 4 | 1211 | 1211 | 176 | 1035 | match |
| 21 | 11 | 3 | 56 | 56 | 56 | 0 | match |
| 21 | 11 | 4 | 1210 | 1210 | 176 | 1034 | match |
| 23 | 11 | 4 | 1143 | - | - | - | match |
| 25 | 5 | 4 | 741 | - | - | - | match |
| 25 | 6 | 3 | 79 | - | - | - | match |

- `kernel_check_ok` and `trivial_in_kernel` are true in every syz run I made.
- `emin solve` gives `e_min = 4` for n = 23 seed 11 (infeasible at e = 3, feasible at e = 4 with
  0 violated points on all points) and for n = 25 seed 7 (same pattern, 3 rounds).
- I spot-checked the duality identity of point 6 by hand:
  - n = 15, e = 3: 1395 − 2304 + (255 + h) + 800 = 146 + h;
  - n = 19, e = 4: 7334 − 11264 + (1023 + h) + 4070 = 1163 + h.

  Both agree with the table.

## Checks against the preregistration

- **Where P comes from.** P ("the excess is nonzero only at e_min") is not written in
  `lane_A_falldegree.md`. It comes from the task card and from the conjecture in
  `out/lead_A1/RESULT.md` / `leads_20261111/README.md`.
  - The reading P1/P2 is frozen in RESULT.md, together with its falsifiers and bug conditions.
  - The verdict applies those falsifiers exactly as written. I found no moved threshold.
  - The restriction to `e <= e_min` is mathematically forced (`K_e ⊆ K_{e+1}`). It is not a
    post-hoc narrowing.
- **Logic of the verdict.**
  - P2 follows from nullity > M(k−1, e−1) alone. emin's independent nullity confirms this in
    every cell I re-ran.
  - P1 also needs `T_e ⊆ K_e`, which syz checks and which the proved F3 of Lead A1 asserts.
  - Both are consistent.
- **Exactness of the sampled n = 27, e = 5 kernel.** By code reading (syz.c, lines 464–485), the
  sample kernel contains the full kernel. A sample kernel with violation rank 0 on all `2^k (k+1)`
  rows therefore is the full kernel. The logic is sound.

## Issues found (none overturns the verdict)

1. **The preregistration cannot be verified as prior to the data.** It is a section of RESULT.md,
   which was last modified at 07:03, after `grid.jsonl` (06:58). No separate frozen file or
   timestamp exists. The same holds for the 06:46 addendum and the n = 27 seed 6 out-of-sample
   prediction (`n27_s6.out` at 06:58 is consistent with it, but this is not proof).
2. **Some cells were not new.** 12 of the 37 cells (n = 23, 25 at e = 3, 4 and n = 27 at e = 4)
   were already measured in `out/lead_A1/nullity_grid.jsonl` before the prediction was made, so
   P was already known to hold there. The new evidence is n = 15–21 and n = 27, e = 5.
3. **e_min provenance.** The preregistration says e_min comes from `out/c/emin_n*.jsonl`, but
   n = 23 seed 11 and n = 25 seed 7 are not in those files. I confirmed `e_min = 4` for both by
   running `emin solve`, so the cells stand. The provenance statement is inaccurate.
4. **Not reproduced by me:** n = 27, e = 5 (10400 / 10401). Each run takes about 750 s and
   1.5 GB, and the container is shared. That number rests on syz alone.
5. **Points 5–8 are post-hoc,** as RESULT.md itself says. Only point 7's n = 27 seed 6 value was
   predicted in advance. Nothing in points 5–8 is machine-checked.
6. **Default unchanged: confirmed.**
   - `git status` on the directory shows only the untracked `out/lead_A1_syzygies/`.
   - `git diff HEAD` on `research/lfd_lower_bound_20261111/` is empty.
   - emin.c is untouched, and its nullity output reproduces the committed `nullity_grid.jsonl`
     values for n = 23 seed 11 and n = 25 seeds 5 and 6.

## Verdict

**I agree** that P held (P1 and P2) in the cells I could re-run: 16 cells, through two code
paths [pcc:unverified]. I found no data contradicting the other cells. The caveats are issues 1–4.
