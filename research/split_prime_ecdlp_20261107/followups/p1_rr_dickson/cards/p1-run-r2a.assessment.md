# Assessment of CARD-p1-r2-a (execution only; interpretation: not_assessed)

card_id: CARD-p1-r2-a
relay_verbatim: false (check-relay on the launch log as report: the only missing line is the provenance.txt output line, present at launch.log line 40 but out of the order the tool recomputes from the receipt (receipt lists outputs run.stdout, jobs.tsv, parsed.jsonl, provenance.txt; the log prints provenance.txt first, card order). The report file was the tool's own launch log, not an executor-returned text; every digest in it equals the receipt. Not treated as an executor deviation; the executor's own relay text was not supplied and is unchecked.)
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed

## Checked against disk
- Card sha256 9c7dcb6a...2cb6 and receipt sha256 d2ccb6e6...feed match the dispatch. Receipt: status completed, commands_total 3, exit 0 for rustc-version, build-p1tool, run-cells, 1 attempt each (no retry).
- All 11 binds (PROTOCOL-R2, PROTOCOL, followups/README, p1tool README/Cargo.toml/Cargo.lock/main.rs, msolve_cell.sh, round 1 cells.tsv, run_cells_r2a.sh, /usr/bin/msolve) recompute to the card's sha256 today. provenance.txt records main.rs sha 7e50f8c8...4ed7 (= pinned), msolve c2722288...9fa, p1tool aa8a4b23...e6c8, git_head d6619e21 (= receipt commit; current HEAD fabca47 is later, binds pin inputs).
- Output digests: run.stdout 6fad0ce1..., provenance.txt fd17970d..., jobs.tsv 4816baab..., parsed.jsonl 5f66612d..., rustc.stdout ac31499b..., build.stdout e3b0c442... (empty) all equal the receipt values and the on-disk files.
- Jobs: jobs.tsv has 26 rows; parsed.jsonl 26 lines; 1..18 part R2c system f over D24/D31/K0 x (m,t) (3,3),(3,4),(3,5) in the driver order; 19..26 part R2a system t: D24 m3t5, D31 m3t5, K0 m3t5 (seeds 1,2) then D24 m5t3 (seeds 2,5). Seeds on every row equal round 1's admissible seeds in runs/run-20261107T214621Z/cells.tsv (D24_m3_t3 2,5; D31_m3_t3 2,4; K0_m3_t3 2,3; all (3,4) and (3,5) cells 1,2; D24_m5_t3 2,5). 26 .ms sha256 in jobs.tsv equal the 26 .ms files on disk.
- Limits: VLIMIT=6291456 and TLIMIT=3600 are driver constants applied to every job (provenance line VLIMIT=6291456 TLIMIT=3600). No HARD_ERROR in run.stdout/stderr; no ERROR.txt.
- Caps: 24 jobs parse status ok, exit 0; jobs 25 and 26 (D24_m5_t3 t, seeds 2 and 5) parse status cap, exit 139 (segmentation fault under ulimit -v, wall 2212.9 s and 2276.6 s, below TLIMIT; the two Segmentation fault lines in run.stderr). They are recorded as status cap with quotient_dim null and D_is_lower_bound true as emitted by p1tool parse; nothing imputed by the driver (it only runs emit, msolve_cell.sh, parse and appends). Stop rule (HARD_ERROR / nonzero exit) not triggered; cap is a recorded result per the card.
- Order: one job at a time, F jobs first, then T jobs (job numbering and the run.stdout sequence).

## Write scope
- Run directory runs/run-r2a-20261108T053528Z/ and the card's cards/ outputs: in scope. The launch log is a declared harness file.
- Outside the card's scope, observed: benchmarks/records/20261108T0538Z-split-prime-p1-r2a-armf-caps.json (mtime 05:38:01, before the run started 05:38:21, created by the harness launcher via BENCHREC; present in receipt dirty_before). Also untracked and unrelated to this card's commands: calibration/run_cells_r2b.sh, cards/p1-run-r2b.yaml (separate not-yet-run card; no r2b run directory exists), analysis/p1r2analyze/ (files mtime 07:34, after the run ended 07:33:05, not written by card commands). No msolve job other than the 26 listed appears in the run directory.

## Reasons
- Receipt, binds, digests, job count, seeds, limits, ordering all agree with the card and driver.
- Two cap rows are results, not failures.
- Caveat: check-relay was run with the launch log; result false for ordering of one output line only (see relay_verbatim). Re-run it on the executor's returned text if the main session wants a verbatim verdict.
