# Rejected experiment: coarser rayon chunks in the tower echelon — 2026-11-25

`PER_OP_PRICE_20261125.md` proposed that the 4.5× per-operation price gap is
task granularity: `eliminate` issues one `par_iter` per chunk per step, and
`per_task` is sized from column count alone, so sparse systems run many small
tasks. The obvious fix was to group more tasks per call.

Change tested (one line, `src/cryptanalysis/f4_fp_tower.rs`):

```rust
let tasks_per_call = 8usize;
let chunk = (per_task * tasks_per_call * rayon::current_num_threads().max(1)).min(n_s.max(1));
// was: per_task * 2 * current_num_threads()
```

Built at `118bb90929d` with the patch applied, binary `e6aa54d2…` (the
unpatched build is `1ae5170f…`). Same parameters as the single-thread baseline:
13 bits, seed 20261185, 4 repeats, one rayon thread.

| Arm | Ops (before) | Ops (after) | Wall (before) | Wall (after) |
|:--|---:|---:|---:|---:|
| interval | 43,189,337,424 | 43,189,337,424 | 16.16 s | 15.85 s |
| candidate | 1,303,185,697 | **1,339,907,806** | 2.20 s | 2.31 s |

## Verdict: rejected, on two independent grounds

1. **It changes the recorded work.** The interval's multiply-add total is
   identical, but the candidate's rises 2.8 %. Chunking therefore perturbs the
   candidate's accounting — most plausibly through the `full_rank_exit` /
   skipped-row path, where a bigger chunk can skip a different set of rows. The
   correctness invariant stated in the proposal ("operation totals must not
   change; anything that changes `ops` is a bug") is violated, so the change
   cannot be landed as a pure scheduling tweak.
2. **It does not help anyway.** The interval gains 2 % and the candidate loses
   5 %; per-op price is flat (0.374 → 0.367 and 1.689 → 1.722). The
   handoff-dominance hypothesis is not supported at this knob.

## What the failure does establish

The candidate's multiply-add total is **not** invariant under chunking, only
under repetition of the same binary. That is worth knowing: cross-binary or
cross-chunk comparisons of `ops` are exact only when the chunking is identical,
and the sealed panels all used one binary, so their internal comparisons remain
valid. Future work that changes `eliminate`'s scheduling must re-baseline the
op counts rather than assume them fixed.

The price gap is therefore still unexplained by this knob. The remaining
candidates are inside `echelon` proper — `Acc::take`/`drain_from` reductions,
`Pivot::new` copies, the per-hit bitset maintenance — and the honest way to
apportion them is to enable `record_trace` for a single measured run (accepting
its overhead on that run only) and read the `ms_rows / ms_reduce / ms_echelon /
ms_update` split, rather than to keep guessing at scheduling.

Artifacts: patched-binary runs `st1_int1t_new.json`, `st1_cand1t_new.json`
(kept outside the repository, since the binary is rejected); baseline runs are
`single_thread_interval_20261185.json` and `single_thread_candidate_20261185.json`
in this directory.
