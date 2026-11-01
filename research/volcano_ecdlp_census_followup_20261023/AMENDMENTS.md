# Amendments

## A1 (2026-10-23, after gate A failed, before any Phase B session; no data exists)

Gate A was reviewed independently in workflow run `wf_190adf49-05b`. It failed
on storage and driver defects, and on interpretations that `ANALYSIS_PLAN.md`
uses without declaring. No measurement had been run. None of the changes below
alters an estimand's meaning, a margin, a family, a test or a threshold.

1. **Disk guard: 1.0 GiB, not 1.5 GiB.** `PLAN.md` §3 assumed about 1.4 GB
   free under a 1.5 GiB guard, which cannot work. Removing the scratch
   `volcano-rho` worktree (its binary is kept, and its patch is committed as the
   census's `volcano-rho-min-n.patch`) left 1.8 GB free. Raw sessions can reach
   about 140 MB each.
2. **Push and prune after every session**, not every 4. Each finished session
   is archived, checked with `xz -t`, committed and pushed, and only then is its
   raw directory removed under census A1.6. This keeps at most one raw session
   on disk.
3. **Driver duties.** `scripts/run_phase_b.sh` must do all of the following
   before any session runs:
   - write a header line to `runs/progress.jsonl`, giving the start time,
     binary sha256, branch and commit;
   - create `sessions/`, `runs/extract/` and `runs/tables/`;
   - check the binary sha256 before each session;
   - record a failed or missing session as a progress line with
     `status: "failed"` (never skip it silently), and count it toward the
     3-failures guard;
   - commit and push per item 2.

   The 20 h wall guard counts session time after the header. Pauses are
   recorded as `pause` lines and excluded, as in the census.
4. **ANALYSIS_PLAN interpretations, declared before data.** These are binding
   as written in `ANALYSIS_PLAN.md`:
   - (a) **H-T is computed in priced `ops_gae`** (C_p = ops_gae + p ×
     uncharged + p × Vélu maps), not in `S`. The formula in `PLAN.md` §4 adds
     GAE-denominated Vélu maps to `S`, which mixes units. Using `ops_gae`
     follows census A2.3's transport-then-solve. The ratio is
     dimensionless, so its meaning (cost of carrying up and folding, over cost
     of solving at the source) is unchanged.
   - (b) The H-T bootstrap resamples (target_index, round) pairs jointly
     across Tcr_v and N_v (shared k_i), inside a cluster bootstrap over source
     vertices.
   - (c) **H-M details:**
     - a (curve, arm) with zero within-model variance reports no F and counts
       as not significant;
     - Holm α = 0.05;
     - permutation p = (1 + #{F* ≥ F}) / (1 + 10,000).
   - (d) "Pooled" means a ratio of means. Only the overall pooled H-T value
     carries the verdict; per-level values are descriptive.

## A2 (2026-10-23, during Phase B; operational, no data inspected)

1. **Busy-host refusals are retried, not recorded as failures.** Between
   05:41Z and 05:43Z, ecbench refused to start 9 short `Tcr` sessions with
   "host is not quiet (PSI cpu some avg10 > 5.00)". It exited 2 and no session
   was started. The previous session's archive, verify and prune work was
   still loading the CPU. Three refusals in a row tripped the
   3-failures guard (`GUARD_AUDIT`) at 87/147 sessions. No measurement was
   affected.
   - `scripts/run_phase_b.sh` now waits 15 s and retries a refusal up to 40
     times, as ecbench's message asks.
   - The failure streak skips the refusal signature (`rc_run` 2 with
     `session_status` "missing").
   - The 9 refused specs are not done, so they run again. Their `failed`
     lines stay in `runs/progress.jsonl` as records.
2. **Measurement orchestration.** In workflow run `wf_ff44b5a4-872`, the
   mechanical-tier restart cards were refused as underspecified or ran the
   wrong command, so the driver was never restarted. The main session now
   starts the driver directly with `scripts/start_phase_b.sh`. The workflow
   resumes at Gate B once every session is done.
3. **Base merge.** `origin/main` was merged into the branch (`1b746e53b0`).
   The `validation-gate` check had failed because a `benchmarks/README.md`
   row added on `main` was missing from the branch. The frozen ecbench binary
   was not rebuilt, and the driver checks its sha256 before every session.
