# Assessment of CARD-p1-r2-b (execution only; interpretation: not_assessed)

card_id: CARD-p1-r2-b (sha256 b12bd576138ee5cbc83a488fad80d474c778a886201244d165e6b45b9efd52e2; lint: CARD OK)
receipt: p1-run-r2b.receipt.json sha256 26b2446cbe658f92c8e9cd0d98323a739fff592266153ab55a422617484719ef (matches the value given)
relay_verbatim: not_applicable (detached harness run, `tools/task_card.py run`; no executor text to compare). launch.log "status: completed" agrees with the receipt.
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis

## Checks against disk
- Receipt: schema task-card-receipt.v1, status completed, 3 commands in card order (rustc-version, build-p1tool, run-cells), each exit 0, 1 attempt, no timeout; run-cells 13419.338 s (limit 30000). commit 39635898... equals current HEAD and git_head in provenance.txt.
- Binds: all 10 bound files (PROTOCOL-R2, PROTOCOL, followups/README, p1tool README/Cargo.toml/Cargo.lock/src/main.rs, msolve_cell.sh, run_cells_r2b.sh, /usr/bin/msolve) re-hashed now and equal the card's sha256. provenance.txt records main.rs 7e50f8c8... and msolve c2722288... (equal to the pins) and the built p1tool aa8a4b23... (not bound by the card; recorded only).
- Outputs: the 8 output digests in the receipt (rustc.stdout, build.stdout, run.stdout, provenance, cells, skips, jobs, parsed.jsonl) re-hashed now and equal. must_not_exist paths were absent at start (dirty_before lists only the BENCHREC record and launch.log).
- Seed scan: skips.tsv has one row, seed 1, emit_exit 101, "base set too small: 5 points for m=5" (rr_s1.emit.err agrees; a permitted skip reason); no T emit for seed 1 (RR is emitted first). cells.tsv: admissible_seed 2, skipped_seeds 1, one_admissible_seed. Seed 2 is the first admissible seed per the frozen rule.
- Jobs: jobs.tsv has job 1 rr then job 2 t, both D24_m5_t4, dickson p 14680063 a 3607823 b 6826282 m 5 t 4 seed 2, with ms sha256 recorded. provenance.txt: VLIMIT=13631488 TLIMIT=10800 (equal to the frozen values and the script constants).
- Recorded outcomes (as parsed, nothing imputed): RR status timeout, exit 124, wall 10800.431 s, D 6 (D_is_lower_bound true), parse_json fields as in parsed.jsonl. T status cap, exit 139 (segfault under ulimit -v 13631488 KiB; run.stderr shows the shell's "Segmentation fault" line), wall 2618.720 s, D 9 (D_is_lower_bound true). Both marked lower bounds by the parser; the logs' last lines match the stdout job lines.
- Idle host: provenance.txt at start shows an empty msolve process list, load average 2.42 / 0.52 / 0.17 at uptime 1 min (the build ended just before; the load is recorded, not explained). Memory at start: 15.6 GB available of 16.5 GB, swap 0. Provenance records the start only; no concurrent-load record exists for the 3.7 h of the run.

## Notes / deviations to be aware of (none changes the outcome)
1. `git status --porcelain` shows paths outside the card's write_scope: `benchmarks/records/20261108T0748Z-split-prime-p1-r2b-m5t4.json` (created by the BENCHREC launcher variable before the receipt, listed in dirty_before; not a card argv write), and `RESULTS.md`, `report/REPORT.typ`, `report/REPORT.pdf` (modified) plus `analysis/derived_r2b/` (untracked). The latter four are absent from the receipt's dirty_after, so were written after the receipt (analysis dir 11:34 vs receipt 11:32) and are not by the card's commands. The benchmark record is a harness side effect that the card declared "no benchmark record steps"; report it, do not treat as executor deviation.
2. The run directory name carries 053603Z while run_start_utc is 07:49:05Z; the name is a frozen constant in the script. Not an error.
3. The card's `commit` field is absent (card notes say no commit pin; binds pin inputs); receipt commit is the HEAD at launch.

## Classification
Every card command ran once, in order, exit 0, within time; frozen inputs unchanged; seed rule and limits as frozen; the timeout and the cap are recorded results (parse status), not stops. interpretation: not_assessed. No RR vs T comparison or decision-rule 3 label is made here.
