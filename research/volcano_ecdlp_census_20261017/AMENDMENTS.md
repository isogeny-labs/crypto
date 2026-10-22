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
