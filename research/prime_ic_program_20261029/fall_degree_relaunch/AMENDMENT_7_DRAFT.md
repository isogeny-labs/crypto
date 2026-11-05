# DRAFT Amendment 7 for `research/prime_fall_degree_20261027/PREREGISTRATION.md`

**Status: draft (plan), not registered.** Written 2026-10-29 by a preparation track. It
becomes Amendment 7 only if the owner adopts it and it is committed to the study's
PREREGISTRATION.md **before any `t >= 12` row of the study exists**. Before committing,
check the Mac worktree (`MAC_RECOVERY.md`). If it holds `t >= 11` rows, the amendment must
list them and must not change any rule those rows bear on.

Text proposed for the preregistration follows the horizontal rule. The notes after it
explain why.

---

## Amendment 7 (DATE, before any Kummer `m = 2` row at `t >= 12`; the `t <= 10` rows are being recomputed)

Changes to resources and instruments for stage A at `t >= 11`, a rule for decomposable
targets, and the scorer that implements Amendments 3, 5 and 7. No hypothesis changes, no
§6 rule changes, and the Amendment 6 prediction is unchanged.

**A. Resources.**
- The relaunch runs on Iso Lab EKS (`isolab_k8s_run`), not on the shared 32 GB host of
  Amendment 2. Each job pins `RAYON_NUM_THREADS` and records the instance type.
- For `t >= 12`, each F4 system runs as its own job, with `--targets k`. It takes
  `--budget 86400`, and `--max-dense` in place of `--max-nnz` (§16.2 of the PKM note: `--max-dense`
  bounds the block held in memory). The memory request is 56Gi at `t = 12` and 112Gi at
  `t = 13, 14`. `--max-dense` is the largest multiple of 10⁸ with `4·C <= request − 8 GiB`.
- Closure limits (`max_cols 2·10⁶`, `max_entries 3·10⁸`, `--budget 1800` per closure) are unchanged.

**B. A two-cap bracket for `t >= 12`.** Uncapped F4 at `t = 12, 13, 14` is expected to need
1.7 h, 17 h and 7 days if the recorded 10x-per-level growth holds. That growth is derived in
`research/prime_ic_program_20261029/fall_degree_relaunch/README.md` §2.6. For `t >= 12`, `d_F`
therefore comes from capped F4 at the two degrees that bracket the Amendment 6 prediction
`c* = 4 + ⌊(t − 2)/4⌋`, which is 6, 6, 7 at `t = 12, 13, 14`. This is the procedure §3 and §5 D
allow once G3 has passed.
- Run cap `c* − 1` first. If it refutes, `d_F <= c* − 1`; step the cap down until it does not refute.
  The last non-refuting cap `c` gives `d_F = c + 1`.
- If cap `c* − 1` does not refute, run cap `c*`. If that refutes, `d_F = c*`. If it does not, step the cap
  up until a cap refutes, or until a cap is stopped by a limit (`d_F >= that cap`, censored).
- A capped run stopped by its budget or `--max-dense` is censored: it bounds `d_F` from neither side.
- Uncapped F4 still runs at `t = 11`, and at `t = 12` where it fits the budget, for H-FFD and H-gap.
  Rows without `D` do not enter H-FFD (§6 as written). G2 is checked wherever `D` exists.

**C. Decomposable targets.** For Kummer `m = 2` (every cell of stage A), a draw is certified
inconsistent when the reduced `S₃` has no zero on the grid (D1). The certificate is
`prime_fall_degree_tools ns-degree DIR --t-max 14`, field `grid_zeros`, computed on the
closure row of the same system. A draw with `grid_zeros > 0` is **decomposable**. It is
reported with its relation and excluded from every median and rule. It is not counted as
censored, and it is not redrawn. A cell whose two draws are both decomposable has no median.
For other kinds and `m`, a closure row that never contains `1` while `λ >= 0.01` is reported
as "possibly decomposable", not as `d_F > cmax`.

**D. The scorer.** Before scoring, `prime_fall_degree_tools score` is patched, committed and
reviewed. The patch implements only the following.
1. It reads `CAP-*.jsonl` and the bracket rows `X-*.jsonl` and keys them as the closure rows are keyed.
2. For a system with a closure row censored at `c_last` with no `1`, so `d_F > c_last`:
   - `d_F = D (F4)` if an uncapped `D*` row refuted at `D` and a capped row at `D − 1` did not refute
     (Amendment 3);
   - otherwise the bracket of B;
   - otherwise the closure's one-sided value.
   The source is printed per row (`closure`, `=D (F4)`, `bracket`).
3. The C exclusion.
4. The Amendment 2 repeat check, extended to rows imported from the first run if they are recovered.
5. A line per `t >= 11` giving the C-step reading: **holds at t** (median equals `c*`), **refuted at t**
   (median exact and different, or a bound that excludes `c*`) or **undecided at t** (a bound that
   includes `c*`, or no median).

---

## Notes (not part of the amendment text)

- **Why the bracket is admissible.** §3 states that refuting at cap `c` proves `d_F <= c`, and that the
  converse holds once G3 has shown the capped run is a `c`-truncated basis. Amendment 3 already uses
  this for `D − 1`. The bracket only chooses which caps to run without first paying for the uncapped `D`.
  It extends G3 beyond its validated range (`N <= 20`, Amendment 3) to `N = 24...28`. That extension must
  be stated in RESULTS. Rerunning G3 in the relaunch (README §2.2) covers `N <= 20` again, not `N >= 24`.
- **Expected cost (derived, low confidence).** Non-refuting capped runs are far cheaper than refuting ones
  at the recorded sizes. Cap 5 at `t = 10, 11` took 17.8 s and about 22 s, against 57 s and 600 s uncapped
  (`research/pkm_tower_round2_20260925/runs/C-K1-kummer-m2-p1-t10.jsonl`, `-t11.jsonl`, `K1-kummer-m2-p1.jsonl`).
  The decisive `t = 14` cell is cap 6 (`c* − 1`). Under C-step it should not refute, and it may stay cheap
  if the degree-6 window saturates as the degree-5 window did (RESEARCH_PKM_TOWER_ORACLE.md §11.7: "What F4
  learns at degree 5 no longer depends on `t`"). Cap 7, which is needed to read `d_F = 7` exactly rather than
  `>= 7`, costs about as much as an uncapped `D = 7` run.
- **Why consistent targets matter here.** At `P1`, `λ = 0.133` for `m = 2`, `t = 14`: about a 12% chance per
  draw of a decomposable target, and 23% that at least one of two is. The committed scorer reads such a draw
  as `"> cmax"` and folds it into the lower median, which would bias the decisive cell upward.
- **What Amendment 7 does not do.** It does not change C-step, the medians rule, the `t <= 10`
  reproduction requirement or any §6 reading. It does not add draws. A median taken over one inconsistent
  draw is still the registered lower median.
