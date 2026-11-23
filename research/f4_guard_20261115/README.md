# f4-f2: refuse an oversized step before building its rows `[pcc:unverified]`

Follow-up to `research/descent_m3_memory_20261115/README.md`. Every number is an unverified assertion: one target, one
shared host, `RAYON_NUM_THREADS=1`, `ulimit -v 9000000`, no `bench.record/v1` (a robustness fix, not a benchmark), no
leaderboard or benchmark row. research-kb was not available in this session; no novelty claim.

## Change (`src/cryptanalysis/pq_f4_f2.rs`)

`groebner_basis_f4_until` (hence `groebner_basis_f4`) now delegates to `groebner_basis_f4_inner(.., max_words, guard)`.
For a step whose S-rows start from more than 2^23 source terms (64 MiB of sparse terms), a *dimension pre-pass* runs
before any row is stored:

- it forms the S-row products and the reducers (symbolic preprocessing) one slab of 256 at a time, keeping only the set
  of monomials examined and a row count (memory proportional to the column count, not to the terms);
- between slabs it checks the budget deadline and the bound `ceil(cols/64) * rows > max_words` (`MAX_MATRIX_WORDS`,
  `F4_F2_MAX_MATRIX_WORDS` as before); rows and columns counted so far only grow, so both are lower bounds on the
  step's final rows and columns, and the refusal never fires on a step that fits;
- at its end it holds the step's exact rows and columns, the very ones the late check computes.

Refusal sets the existing `F4Stats::oversize` (guard) or `timed_out` (deadline) and puts the step's pairs back, as the
late check did; the callers (`F4F2::solve` -> `SolverVerdict::BudgetExceeded`, `pdp_split::solve_branch`) read those
flags unchanged. The pre-pass is uncounted (no `bitop` charge) and a fitting step is then built exactly as before, so
output and counted operations are unchanged. Cost: a step above the threshold forms its products twice.

## Checks

- `cargo test --release --locked --lib pq_f4_f2`: 18 passed (15 existing, 3 new).
  - `the_early_guard_keeps_fitting_steps_identical`: for three random systems, bisects the smallest cap that does not
    refuse under the late guard alone and under the pre-pass on every step; both agree; at that cap basis, `word_xors`,
    `steps`, `reducer_rows`, `divisor_tests` and the `bitop` tally are identical; one word lower both refuse.
  - `an_oversized_step_is_refused_while_building`: caps 1, 8, 64 words refuse with `oversize`, pairs kept.
  - `the_budget_is_honoured_while_building_rows`: a pre-pass deadline already past gives `timed_out`, no matrix built.
- Replay: `ecbench verify --replay-all` with the rebuilt binary on two committed sessions that run `f4-f2`:
  `research/tableb_binary_descent_pilot_20261115/sessions/desc24` (audit OK: 3 records, 1 replay reproduced, 0 problems)
  and `research/ecbench_factor_bases_20261013/sessions/koblitz17-algebraic` (audit OK: 5 records, 5 replays reproduced,
  0 problems). Those are small systems below the pre-pass threshold; the equality of counted operations above it rests on
  the unit test (forced pre-pass on every step).

## Before / after, n21 k=7 direct S_4 (`research/descent_m3_memory_20261115/profile_example.rs`)

| budget | before (committed `raw/`) | after |
|--:|:--|:--|
| 5 s | stops after step 4, `timed_out`, peak RSS 1.6 GB | (not rerun) |
| 12 s | **killed** (allocation failed, RSS 8.25 GB at 24 s) | `timed_out`, F4 wall 12.2 s, peak RSS 1.56 GB |
| 22 s | **killed** | (not rerun) |
| 120 s | not run | `oversize` refusal, F4 wall 41.7 s, peak RSS 1.56 GB, 44,443 pairs left, 3,546 elements |

Peak RSS is the harness's `VmHWM` (sampled every 2 s). The step-5 matrix would be about 4.9 GB dense (4.6 x the guard),
so the unbudgeted run now ends as `BudgetExceeded` after about 42 s instead of an out-of-memory kill. The time to
refusal is dominated by forming the reducers of the degree-7 step, which is inherent to knowing its column count.
