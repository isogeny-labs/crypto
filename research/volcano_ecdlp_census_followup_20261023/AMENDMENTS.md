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

## A3 (2026-11-06, POST HOC: written after all Phase B data existed, before the final analysis run)

Phase C was written by an executor who had read `ANALYSIS_PLAN.md`, the census
`analyze.py`, and the committed extracts' structure. The code was debugged with a
100-replicate smoke run, whose numbers were looked at (including H-T and H-M)
only to check that the code ran. None of the declarations below was changed
after seeing a number. They fill gaps the frozen plan leaves open, or record
what could not be done as written. No estimand, margin, family, threshold or
seed changes. Everything here is labelled post hoc in the report.

1. **Uncharged-count extract, as the plan prescribes (s1).** `analysis/extract_uncharged.py`
   unpacked each of the 147 archives once, one at a time, into scratch and deleted
   it, writing `runs/uncharged/<session>.csv.xz` and `runs/slugmap/<session>.json`.
   `<session>` is the basename of the progress line's `out`: the three redone
   sessions are `V6-D-L1-c0.r2`, `V6-D-L2-c0.r2` and `V6-N-L1-c0.r2` (their progress
   lines carry spec `V6-D-L1-c0` etc., with `out` = `sessions/<spec>.r2`). Each
   spec therefore counts once, from its `.r2` session. The three partial first
   attempts (`runs/orphans/*.orphan.tar.xz`, cut short by container restarts) hold
   no progress line and are not analysed. The 9 `failed` lines (busy-host
   refusals, A2.1) are ignored by s1's rule (no `done` status).
2. **H-T pooled values (s5b, "ratio of means ... averaged over the vertices").**
   A per-vertex mean is the mean over that vertex's (target_index, round) pairs.
   A stratum or overall pooled numerator (denominator) is the plain average of
   the per-vertex numerators (denominators) over the vertices in the stratum (all
   63), so the overall value is vertex-weighted. This follows A1 4d and the
   literal "averaged over the vertices"; it is not the row-pooled ratio.
   The bootstrap draws vertices with replacement within each source level, then
   draws (target_index, round) pairs with replacement within each chosen vertex,
   jointly for the three columns rho-cubic on Tcr_v, rho-neg on Tcr_v and rho-neg on
   N_v. The per-level pooled CIs are taken from the same replicates. Seed 20261023.
   `M_v = velu_point_maps (tcr-v<id> row) + 2` as s5b states; as a check it must
   equal `2 * path_len_to_crater` from `census/V5/tcr/<id>.json` (no failures).
3. **H-T per-vertex T_v and T0_v CIs (descriptive).** Pair bootstrap inside the
   vertex only (no cluster level), 10,000 replicates, generator
   `SeedSequence([20261023, v])`. Each vertex has 8 targets x its level's rounds
   pairs; every Tcr_v row pairs with an N_v row of the same (target_index, round)
   (zero unpaired rows is a check).
4. **H-T verdict arithmetic.** "T0 CI includes 1" is the closed interval test
   `lo <= 1 <= hi`. "Refuted" is tested before "inconclusive"; "supported" first.
5. **Holm family (a) (s7a).** 35 tests: V6 levels 1-4 x 5 arms (20) and V5 levels
   4-6 x 5 arms (15). V5 levels 1-3 get the verdict "descriptive" and no Holm p.
6. **V5 per-family R(L) (s3 robustness).** Ua and Ub have no level-0 rows on V5.
   Per-family R is reported against the family's own level 0 where it exists (D, N)
   and, for all families, also against the pooled level-0 mean (D and N rows).
   Both are descriptive.
7. **H-M permutation (s5c).** Rows are shuffled across the 8 models as the plan
   says, ignoring the (target_index, round) pairing that the k_i identical across
   models would allow; that makes the test conservative. Each (curve, arm) gets
   `numpy.random.default_rng(20261024)`; the effect-size bootstrap is a second,
   separate `default_rng(20261024)`. The Holm family is the curves of an arm with a
   defined F; a degenerate curve (zero within-model variance, none expected) would
   count as not significant. BSGS has between-model variance exactly 0 if the
   models carry the same instances (F = 0, p = 1); this is also the instance check.
8. **IC against matched rho (s9).** On V5, levels 1-6 use `rho-neg`; the level-0
   `rho-neg` ratio is printed but marked "not the matched rho"; the crater's matched
   comparison is the `rho-cubic` block (D and N pairs, 0 / 0.1 / 1 GAE). The V5
   pooled value is given with and without level 0. The M specs carry only `rho-neg`,
   so their IC ratio is against `rho-neg` (a row bootstrap over the curve's rows,
   one cluster, descriptive); on the V5 crater M curve it is not the matched rho.
9. **Not run, per the plan.** s4 (curve effect against ctl) and the paired source
   comparison run on V6 only; V5 has no S family. The Q2a positive control is the
   detector for every "equivalent" verdict, V6 included (s6).
10. **Report tooling.** `REPORT.md` is laid out from `results.json` by
    `analysis/build_report.py`; figures by `analysis/figures.py`; the PDF by
    pandoc (Markdown to Typst) and a Typst wheel in a scratch virtualenv.
