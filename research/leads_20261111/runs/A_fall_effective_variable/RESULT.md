# LEAD-A-fall-effective-variable-law (lane A, Lead A3): result, 2026-11-12

Lead: `research/leads_20261111/lane_A_falldegree.md`, Lead A3. All results below are
`[pcc:unverified]`. They come from one agent on a shared 4-vCPU container. There is no replay
certificate and no independent stall certificate.

## Verdict

**The preregistered v-law is FALSIFIED** `[pcc:unverified]`. Two preregistered falsifiers fired:

- Cell (i), T-type n=35, k=17 (v=34), was predicted to **stall**. Both unsatisfiable instances
  **fall**: 1 is in W_4.
- Cell (iii), R-type n=36, k=18 (v=35), was predicted to stall. Both instances **fall**.

The positive controls rule out a tool or generator mismatch:

- T33 (also N=34, v=34) stalls with exactly the treatise's dimensions.
- R35 stalls with exactly the treatise's dimensions.

So at the same v = 34 and the same N = 34, T33 stalls and T35 falls. The equation count matters.
This is the lead's own reading for "(i) falls": the v-law is at most a one-parameter slice, and
neither v nor N alone fixes d_F.

The other preregistered parts held `[pcc:unverified]`:

- Cell (ii) falls, as predicted.
- Cell (iv) falls, as predicted.
- Lower degrees: d_F is monotone in v within each family at every n tested (11, 13, 15, 17, 19).
- T(15,7) has d_F = 3, inside the predicted {3,4}.
- T(15,5) has d_F = 3, inside the predicted {2,3}.

## Infrastructure: a usable native closure now exists

**Search result.** I found no usable degree-4 closure in this repository:

- `lfd_small.py` is a pure-Python closure that enumerates 2^N points for the descent, so it is
  limited to N ≲ 20.
- `research/guarded_fall_degree_20261028` and `research/fall_degree_bounds_20261008` hold F_p and
  guarded-chain tools, not this binary system.
- `examples/hky_last_fall_degree.rs` was not used. It computes the same polynomial S_3. Its
  limits, read from its header and argument handling (it was not test-run):
  - dim V is fixed at ⌊n/2⌋, and V comes from its own families. It does not read a given (V, z),
    so it cannot produce T-type V ⊂ ker Tr or k ≠ ⌊n/2⌋.
  - It is a work-queue engine aimed at n ≈ 24.
  - It was not tried at N = 34.

The sibling `lfdclose2.c` (`isogeny-labs/crypto-autoresearcher/analysis/semaev-m2-last-fall-degree/code/`)
is VLT1 vault-encrypted on disk. It was not decrypted or recovered.

**New tool.** I wrote `fallclose.c`, a new native port of about 290 lines of C over the system
libm4ri:

- **Descent.** The Weil descent is built symbolically: S is bilinear, plus linear terms, plus a6.
  There is no 2^N enumeration.
- **Closure.** W_D is computed as the least subspace containing F and closed under
  f -> x_i f for f in W_D ∩ B_{<D}. This is the same definition as `lfd_small.closure`.
- **Method.** Columns are ordered by degree, highest first. W is kept in reduced row-echelon form,
  so the rows with low-degree pivots span W ∩ B_{<D}. Products are added in batches of 8192 rows by
  M4RI matrix products. Only new low-degree rows are queued (semi-naive), and the fixed point does
  not depend on order.
- **Cost.** N=34 takes 107–270 s and about 0.8 GB. N=36 takes 210–280 s and about 1.2 GB. Each run
  is one thread.

**Validation** (all `[pcc:unverified]`):

1. **Against `lfd_small.closure`.** I compared the two on 24 small R- and T-type instances
   (n = 9..17, N <= 16) at D = 2, 3, 4: 72 runs, 0 disagreements. The 42 stall runs agreed on every
   cumulative dimension. Output: `out/crosscheck.jsonl`.
2. **Against the treatise records** (`papers/ecdlp-index-calculus-treatise/figures/data-bin-fd-instances.json`).
   My instances are fresh, because the treatise's seeds are not recorded.
   - R35 (seed 6): stall, W_dims (0,1,105,3885,61425), codim 5287. This equals R35a/b exactly.
   - T33 (seeds 2 and 18): stall, (0,0,64,2968,48068), codim 4888. This equals T33a/b exactly.
   - T31 (seed 16): fall, as T31a/b.
3. **Negative controls** (satisfiable instances, which must stall):
   - R n=34, k=17 with 2 solutions: stall, codim 2.
   - T n=35, k=17 with 4 solutions: stall, codim 4.
   - In the small check, every satisfiable instance stalled.
4. **Solution counts.** `emin count` agrees with `lfd_small`'s brute-force count on 6 small
   instances.

## What was run (exact commands, from this directory)

```
gcc -O3 -march=native -o fallclose fallclose.c $(pkg-config --cflags --libs m4ri)
python3 crosscheck.py > out/crosscheck.jsonl                 # implementation control
for c in "T 35 17" "R 34 17" "R 36 18" "T 32 16" "T 31 16" "R 35 18"; do ./select_instances.sh $c 12; done > out/instances.jsonl
./scan_T31.sh                                                # T31 unsat seeds (found 16, 17)
./select_instances.sh T 33 17 12 >> out/instances.jsonl      # + seed scan to 18 (T33 control)
./run_cell.sh T 32 16 3 4 0 cell_iv                          # first timing run
./run_all.sh        # cells (i)-(iv) x2, pos R35, neg sat R34 (each: timeout 1200 ./run_cell.sh ...)
./run_controls.sh   # pos T31, added T33 x2, neg sat T35
python3 lowdeg_scan.py                                       # D=2..6, n=13,15,17 -> out/lowdeg.jsonl
# post-hoc follow-up, prediction frozen first in POSTHOC_PREDICTION.md:
./select_instances.sh {T 34 17 | T 37 18 | R 37 18} 12 >> out/instances.jsonl
./run_followup.sh; python3 lowdeg_followup.py                # -> out/cells.jsonl, out/lowdeg_followup.jsonl
python3 summarize.py > out/tables.md; python3 summarize.py lowdeg_followup.jsonl > out/tables_followup_lowdeg.md
```

**Instances.**

- R-type: `lfd_lower_bound_20261111/emit_instance.py N SEED K`.
- T-type: `emit_instance_T.py N SEED K`, with V ⊂ ker Tr and Tr(a6/z^2) = 0.
- These are the same generators as lead A2.
- Satisfiability was decided by `emin count`.
- Each cell takes the first two unsatisfiable seeds, counting up from 1.
- No large-cell instance is degenerate: 1 is not in span(F) (`out/bigcell_span.jsonl`).
- T-type span(F) contains no linear form. R-type span(F) contains exactly one linear form (ell_0).

**Raw data.**

- Results: `out/cells.jsonl`, `out/lowdeg*.jsonl`.
- Instance lines with their solution counts: `out/instances.jsonl`.
- Per-batch progress: `out/log/`.

## Results: D = 4 (preregistered cells, controls, post-hoc follow-up)

In the table:

- v is the number of variables after eliminating the trace form: N-1 for R-type, N for T-type.
- m = n - 1 is the number of nontrivial equations left after that elimination.
- e = 2v - m is the post-hoc "excess" described in the next section.

| cell | type | n | k | N | v | m | e | predicted (v-law) | observed (2 inst.) | W_4 dims at stop |
|:--|:--|--:|--:|--:|--:|--:|--:|:--|:--|:--|
| (i) | T | 35 | 17 | 34 | 34 | 34 | 34 | stall | **fall ×2** (falsifier) | (1,35,596,6580,52956) |
| (ii) | R | 34 | 17 | 34 | 33 | 33 | 33 | fall | fall ×2 | (1,35,596,6580,52956) |
| (iii) | R | 36 | 18 | 36 | 35 | 35 | 35 | stall | **fall ×2** (falsifier) | (1,37,667,7807,66712) |
| (iv) | T | 32 | 16 | 32 | 32 | 31 | 33 | fall | fall ×2 | (1,33,529,5489,41449) |
| pos T31 | T | 31 | 16 | 32 | 32 | 30 | 34 | fall | fall ×1 | (1,33,529,5489,41449) |
| pos R35 | R | 35 | 18 | 36 | 35 | 34 | 36 | stall | stall ×1, = treatise dims | (0,1,105,3885,61425) |
| added T33 | T | 33 | 17 | 34 | 34 | 32 | 36 | stall | stall ×2, = treatise dims | (0,0,64,2968,48068) |
| neg (sat, 2 sols) | R | 34 | 17 | 34 | | | | stall | stall, codim 2 | (0,33,594,6578,52954) |
| neg (sat, 4 sols) | T | 35 | 17 | 34 | | | | stall | stall, codim 4 | (0,31,592,6576,52952) |
| F1 (post-hoc) | T | 34 | 17 | 34 | 34 | 33 | 35 | v-law: stall; excess: fall | fall ×2 | (1,35,596,6580,52956) |
| F2 (post-hoc) | T | 37 | 18 | 36 | 36 | 36 | 36 | both: stall | stall ×2 | (0,0,74,3551,60902) |
| F3 (post-hoc) | R | 37 | 18 | 36 | 35 | 36 | 34 | v-law: stall; excess: fall | fall ×2 | (1,37,667,7807,66712) |

- **Agreement between instances.** Both instances of every cell agree, and the W dimensions are
  identical within each cell.
- **Fall runs.** These stop as soon as 1 enters W, so their dimensions are the full space.
- **Complete per-run table.** `out/tables.md` adds seeds, seconds and RSS.
- **Note on (ii).** The lane lists cell (ii) as having n_eq = 33. Here span(F) has rank 34: 33
  nonlinear equations plus ell_0.

## Results: lower degrees (D = 2, 3)

These are the non-degenerate unsatisfiable instances, two per (family, n, k). The threshold v_c(D)
is the largest v with d_F <= D. It is pooled over R and T, because each pooled sequence was
monotone.

| n | v_c(2) | v_c(3) | monotone within family | monotone pooled R+T |
|--:|--:|--:|:--|:--|
| 11 | 7 | 12 | yes | yes |
| 13 | 8 | 13 | yes | yes |
| 15 | 9 | 14 | yes | yes |
| 17 | 10 | 15 | yes | yes |
| 19 | 10 | 16 | yes | yes |

- **Scope.** n = 11 and n = 19 are post-hoc follow-ups.
- **Per-instance data.** `out/tables.md`, `out/tables_followup_lowdeg.md`.
- **Degenerate instances excluded.** An instance is degenerate when 1 is already in span(F),
  mostly at k = 3, 4. I fixed that rule before the scan, and these instances are counted separately.
- **Match with earlier data.** The R-type values at n=15 match `out/n15_kscan.txt`: k=5 gives 2,
  k=6 gives 3, k=8 gives 4.

**Reading.**

- At fixed n, d_F is a monotone function of v across both families. The lead's T/R
  "fill-the-gaps" picture holds at each n.
- The thresholds move with n: v_c(3) = (n-1)/2 + 7 at all five n.
- A single family-free constant v_c(D) is therefore not supported at D = 3 either.

## Post-hoc hypothesis (frozen in `POSTHOC_PREDICTION.md` before F1–F3 and n=11,19 ran)

The hypothesis was an "excess law" in e = 2v - m:

- d_F <= 2 iff e <= 4
- d_F <= 3 iff e <= 14
- d_F <= 4 iff e <= 35

Outcome `[pcc:unverified]`:

- **D = 4: consistent.** F1, F2 and F3 all came out as predicted. The two cases that discriminate
  against the v-law are F1 (falls at v=34) and F3 (falls at v=35). With these, all 13 D=4 cells
  (this run plus the treatise's R33, R37 and R39) fit "fall iff e <= 35".
- **D = 3: consistent** at all five n.
- **D = 2: FALSIFIED at n = 19.** R-type k=6 (v=11, e=4) has d_F = 3 on both instances.
  v_c(2) is 10 at both n=17 and n=19, so it stops tracking n/2.

So the excess law is falsified as stated. Its D=3 and D=4 parts are a fit that has passed one
round of fresh predictions, and nothing more.

## What is blocked

- **Independent stall certificate.** The checker (treatise Prop. `cert`) and `lfdclose2.c` are in
  the vault-encrypted sibling repository, so the stalls here were not certified independently.
  - What exists instead: the stalls reproduce the treatise's certified dimension vectors exactly
    (R35, T33).
  - What a satisfiable stall shows: its codimension equals its number of solutions (2 and 4).
- **AWS, S3 and Iso Lab EKS.** No bench records, S3 hashes or EKS replay. The AWS identity is
  rejected.
- **research-kb MCP.** Not available to this subagent, so no knowledge-base receipt.
- **Other hosts.** Nothing was replayed on a second host.

## Default unchanged

- No existing source file was edited. Everything new is in this directory.
- `git status` shows no change under `research/lfd_lower_bound_20261111/`. Its scripts and
  `emin` were only executed.
- The build artifact `fallclose` is not meant to be committed; rebuild it with the gcc line above.

## Next steps

1. **Separate a two-parameter law (v, m) from the excess form 2v - m at D = 4.**
   - Run T n=36, k=18 (v=36, m=35, e=37): predicted stall.
   - Run R n=38, k=19 (v=37, m=37, e=37): predicted stall.
   - Run R n=39, k=19 (v=37, m=38, e=36): predicted stall.
   - These are N = 36–38 cells, about 4–6 min each.
   - Also find a pair with equal e but different (v, m) on both sides of 35.5.
2. **Explain the D = 2 plateau** (v_c(2) = 10 at n = 17 and n = 19). Extend the scan to n = 21, 23.
   It is cheap.
3. **Certify the new stalls** (T37 at k=18, and R35) with an independent checker before citing them.
   Port the treatise's certificate check, or compute an explicit annihilating functional on W_4 from
   the closure's RREF.
4. **Lane README.** Record that A3's dependency is resolved by `fallclose.c`, and that the v-law is
   falsified by (i) and (iii).
