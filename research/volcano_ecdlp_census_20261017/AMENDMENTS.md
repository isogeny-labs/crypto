# Amendments

Each amendment was made before any Phase B data existed, and none depends on a
measured outcome.

## A1 (2026-10-17, after G-A, before Phase B)

1. **V4 is censused, not measured.** The twist of the V1 root has largest
   prime subgroup r = 1583, so it cannot support rho / BSGS / IC comparisons.
   Choosing a different V1-shaped curve because of its twist would be a new,
   unregistered selection rule. The plan's "stretch" volcano is therefore
   dropped from measurement.
2. **V3 (j = 0) has 16 vertices.** The U families reach 15 of them, because
   `ca` does not step onto j = 0. The positive control (PLAN §5 test 5) is the
   crater curve under D, N and ctl with `rho.cubic`. The level analysis on V3 is
   descriptive only: one curve per upper level.
3. **Target seeds.** PLAN §3 said both "a different target_seed per family" and
   "D and U share k_i". The second wins because it is what pairing needs: D, Ua,
   Ub, S and ctl use 20261111, and N uses 20261115 (`census/seeds.json`).
4. **Rho CV.** The measured per-solve CV is 0.40 at b = 26 (24 solves), not
   the 0.52 placeholder. The ≥ 2000 rho solves per level rule stays, so
   precision is a little better than planned.
5. **Drive.** The only upload path is the Drive connector, which carries file
   bytes through the model context. That fits small text files, but not
   megabyte archives. **git (GitHub) is the durable copy of record.** Drive
   gets:
   - the index;
   - `FROZEN.json`, the calibration summary, the run manifest and the progress
     log (as text);
   - the Phase C report, PDF and summary CSVs.

   Each upload is size-checked on read-back. The census and specs archives
   staged in Phase A are in git in full. Their SHA-256 values are in `DRIVE.md`.
6. **Raw session storage.** After each session's audit passes, its directory
   is packed to `sessions/<spec>.tar.xz`, with SHA-256 recorded in
   `runs/progress.jsonl`. Each session also gets:
   - a compact per-run CSV extract (`runs/extract/<spec>.csv.xz`, one row per
     run: workload, arm, round, ops / S, wall, verified, isolation level);
   - the audit receipt.

   After the first 3 sessions, the executor projects the total archive bytes:
   - **If the projection is ≤ 200 MB,** the archives are committed.
   - **Otherwise,** the CSV extracts and audits are committed, and the archives
     stay local, with SHA-256 values in git.

   An uncompressed session directory is removed locally only if free disk drops
   below 2 GiB. Even then, its archive must first be committed and pushed, or
   have its hash recorded, and must extract and re-verify (`ecbench verify
   --replay 3`) from the extracted copy.
7. **Clean binary.** Phase A's ecbench reports a dirty tree (`9bc00fa`, dirty).
   Phase B rebuilds ecbench from a clean, committed HEAD of this branch and
   records the commit and binary SHA-256 in `runs/progress.jsonl`'s header.
   Calibration sessions keep their own binary record. Calibration numbers are
   not pooled with Phase B.

## A2 (2026-10-22, Phase C, after Phase B data existed; post hoc)

These are written before `analysis/analyze.py` is run. None changes an
estimand, margin, family, test or threshold in `ANALYSIS_PLAN.md`.

1. **Source of the uncharged counters (plan §9).** The plan said to take them
   from `ecbench table` / `ecbench compare --save`. Both print only the names
   of the unpriced counters (`cost.unpriced`) and a `lower_bound` flag, not their
   counts, and the CSV extracts do not carry them. The counts are in each run
   record. `analysis/extract_uncharged.py` therefore reads the counts from each
   session's `records.jsonl` (unpacked from its committed archive, one at a
   time) and writes `runs/uncharged/<spec>.csv.xz`, one row per measured run:
   `total_gae`, `S`, the sum of the uncharged counts and the per-counter detail.
   A counter named in `cost.unpriced` takes its value from the record's
   top-level `counters` if present, otherwise from the sum of the phases'
   `native` counters of the same name without the `_uncharged` suffix (this is
   the only place ecbench puts the index-calculus counts). Priced S at price p
   is `S * (total_gae + p * uncharged) / total_gae`, which is the definition of
   S as total_gae over the session floor. No cost is re-derived. The totals
   (`total_gae`, `S`) agree with the committed CSV extracts row by row; the
   analysis checks this and reports mismatches.
2. **A1.6 cleanup.** The nine remaining uncompressed session directories were
   removed under A1.6, each with a `cleanup` line in `runs/progress.jsonl`,
   after the same checks as Phase B; each archive was unpacked once more for
   the uncharged extract.
3. **Interpretations the plan leaves open (post hoc, fixed in code before the
   final run).** A 100-replicate smoke run was inspected before the final run,
   to test the code. No choice below was changed after seeing its output.
   - *Curve = census vertex.* §1 keys curves by `curve_slug` and maps each slug
     to its vertex. The five isogenous families give five slugs (five
     generators) per vertex, so the bootstrap resamples **vertices**, then rows
     of each chosen vertex (all five families together), stratified by level.
     Resampling slugs would count one curve five times.
   - *Level analysis uses `S` and carries `ops_gae` and `solve_wall_ns` on the
     same resamples.* The wall column is descriptive unless §8 admits it.
   - *p-values.* A bootstrap two-sided p = 2·min(P(R* ≤ 1), P(R* ≥ 1)) with
     +1 smoothing, used only for Holm. A test is "different" when the 95% CI
     excludes 1 and the Holm-adjusted p is below 0.05. A result that meets both
     TOST and "different" is labelled "equivalent" (the plan's order).
   - *§4 per-curve p.* Within-vertex label shuffles have 3 outcomes per vertex,
     so a per-curve p cannot reach 0.01 from them. p_v is the share of the
     pooled |z*| (all vertices, all 10,000 shuffles) at least |z_v|, Holm over the
     curves of that (V, arm). A degenerate (V, arm) with zero within-vertex
     variance (BSGS: S depends only on r and k) reports no F and no flags.
   - *§5 paired source ratio.* The unit is the (vertex, target_index, round)
     triple, resampled jointly across the two families; pooled over all levels,
     strata = level. N does not share k_i with D, so its pairing is nominal.
     Holm family (c) = V × {rho-neg, rho-plain, ic} × {Ua, Ub, S, N}. bsgs and
     bsgs-neg get the identity check only.
   - *§5 transport then solve.* Per (V, source family, arm, destination level):
     [mean ops_gae at the destination + p × Vélu point maps of the path] ÷
     mean ops_gae of the same family's rows at the source vertex, for p = 0,
     0.1, 1. Cost of both point maps (G and Q) is charged to every instance, with
     no amortisation. The plan gives no definition of "exceeds the control
     spread"; the control is the same ratio computed with the native (N) rows at
     the same destinations and the same source vertex, no Vélu cost. A signal
     needs the CI to exclude 1 on the lower side at all three prices and to be
     disjoint from the control CI. The source mean comes from one vertex, so
     it is noisy and shared by all destination levels.
   - *§6.* The V3 crater is one curve, so the primary bootstrap resamples
     (workload, target, round) pairs of `rho-cubic` and `rho-neg`. A
     slug-cluster resample is reported as a sensitivity.
   - *§8.* "A/A" ratio = pooled mean of `solve_wall_ns` of `rho-neg-aa` over
     `rho-neg` on the same triples, level-stratified vertex bootstrap.
   - *§9.* Pairs are (ic, rho-neg) runs of the same (workload, target, round);
     priced S = S · (total_gae + p · uncharged) / total_gae. Only counters that
     ecbench names in `cost.unpriced` are priced; any other native work is not
     counted, so both arms stay lower bounds.
   - *Reps.* All statistics use the plan's 10,000 replicates; no reduction.
4. **Python.** AGENTS.md bars Python from computing results; the plan names
   `analysis/analyze.py`, so it is Python. It reads only committed records and
   is a candidate for a native re-implementation by the validator.
