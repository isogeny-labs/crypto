# PKM prime tower, m = 5: closure degree and cost

Registered 2026-11-04 as a draft (`protocol.json`, schema `experiment.protocol/v1`, id `ARITY-COMPLETION-PRIME-TOWER-M5-20261104`, version 1). Freeze status: draft; the main session freezes it before any data. No experiment has been run.

## Question

What is the F4 closure degree D, and what is the cost, of the PKM prime-field tower factor base at arity m = 5 at N = 10, 15 and 20, for the Kummer and isogeny towers over p1 = 2013265921? This arity was never measured; the arity review has m = 2, 3, 4 only.

Neighbours (*recorded*): m = 2 Kummer D = 4 (N 2..10), 5 (N 12..18) (corrected by AM1; v1 read "4 (N 2..8), 5 (N 10..18)", which is the isogeny tower's boundary), 6 (N 20, 22); m = 3 D = 6, 6, 7, 7 at N = 9, 12, 15, 18 and >= 8 at N = 21 (died at 99.6 GiB); m = 4 D = 6, 7, 7, >= 8 at N = 8, 12, 16, 20; D independent of prime and tower kind over 82 cells. m = 4 costs: N = 8 1.5-2.7 s; N = 12 595-1242 s, 477 MB; N = 16 `f4_fp_tower` 55.5 min, 8.06 GB; N = 20 msolve died at the 5 GB cap after 3 degree-8 rounds. Sources: `research/ic_next10_20261014/03-pkm-degree/README.md` sections 2.1-2.2 and `results.csv`; `research/pkm_m3_n21_msolve_20261018/RESULTS.md`; arity review `research/factor_base_arity_review_20261024/README.md` section 1.2 (m = 2, 3, 4 only).

## Instrument

`examples/pkm_tower_pilot.rs`, engine `tower` (in-repo `f4_fp_tower`), prime p1 = 2013265921 (= 15*2^27+1, v2(p-1) = 27), Kummer and isogeny towers, controls `tower`, `--planted 0 --random 2 --ladder-t none --trace --max-nnz 2000000000`, one t per process via `--t-min T --max-t-m4 T`, default seed 0x504B4D54 (1347112276), one `--out` file per cell. Flag style from `research/pkm_tower_round3_20260925/README.md`. Cells: m = 5 at t = 2, 3, 4 (N = 10, 15, 20), both kinds, two random targets each = 12 target rows. A memory or wall stop is inconclusive, not evidence.

## Per-cell command lines (run from the repository root; `cells/` is this study's directory)

```
prlimit --as=8589934592 timeout 7500 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds kummer --controls tower --planted 0 --random 2 --ladder-t none --m 5 --t-min 2 --max-t-m4 2 --budget 7200 --max-nnz 2000000000 --trace --out cells/kummer-m5-t2.jsonl 2> cells/kummer-m5-t2.time
prlimit --as=8589934592 timeout 7500 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds kummer --controls tower --planted 0 --random 2 --ladder-t none --m 5 --t-min 3 --max-t-m4 3 --budget 7200 --max-nnz 2000000000 --trace --out cells/kummer-m5-t3.jsonl 2> cells/kummer-m5-t3.time
prlimit --as=128849018880 timeout 86700 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds kummer --controls tower --planted 0 --random 2 --ladder-t none --m 5 --t-min 4 --max-t-m4 4 --budget 86400 --max-nnz 2000000000 --trace --out cells/kummer-m5-t4.jsonl 2> cells/kummer-m5-t4.time
prlimit --as=8589934592 timeout 7500 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds isogeny --controls tower --planted 0 --random 2 --ladder-t none --m 5 --t-min 2 --max-t-m4 2 --budget 7200 --max-nnz 2000000000 --trace --out cells/isogeny-m5-t2.jsonl 2> cells/isogeny-m5-t2.time
prlimit --as=8589934592 timeout 7500 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds isogeny --controls tower --planted 0 --random 2 --ladder-t none --m 5 --t-min 3 --max-t-m4 3 --budget 7200 --max-nnz 2000000000 --trace --out cells/isogeny-m5-t3.jsonl 2> cells/isogeny-m5-t3.time
prlimit --as=128849018880 timeout 86700 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds isogeny --controls tower --planted 0 --random 2 --ladder-t none --m 5 --t-min 4 --max-t-m4 4 --budget 86400 --max-nnz 2000000000 --trace --out cells/isogeny-m5-t4.jsonl 2> cells/isogeny-m5-t4.time
```

Control cells:

```
prlimit --as=8589934592 timeout 900 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds kummer --controls tower --planted 0 --random 2 --ladder-t none --m 4 --t-min 2 --max-t-m4 2 --budget 600 --max-nnz 2000000000 --trace --out cells/ctl-m4-n8-kummer.jsonl 2> cells/ctl-m4-n8-kummer.time
prlimit --as=8589934592 timeout 7500 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds kummer --controls tower --planted 0 --random 2 --ladder-t none --m 3 --t-min 5 --max-t-m3 5 --budget 7200 --max-nnz 2000000000 --trace --out cells/ctl-m3-n15-kummer.jsonl 2> cells/ctl-m3-n15-kummer.time
prlimit --as=8589934592 timeout 900 /usr/bin/time -v target/release/examples/pkm_tower_pilot --engine tower --p 2013265921 --kinds kummer --controls tower --planted 0 --random 2 --ladder-t none --m 2 --t-min 5 --max-t 5 --budget 600 --max-nnz 2000000000 --trace --out cells/ctl-m2-n10-kummer.jsonl 2> cells/ctl-m2-n10-kummer.time
```

Caps: wall 7200 s and `prlimit --as` 8 GiB for t = 2, 3; wall 86400 s and 120 GiB for t = 4 (only on a host with that memory). The `timeout` value is the budget plus 300 s as a backstop.

## Predictions (preregistered, with falsifiers)

| id | claim | falsified by |
|---|---|---|
| P1 | D in {6, 7} | falsified by any D outside {6, 7} on either tower or target |
| P2 | D in {7, 8} | falsified by any D outside {7, 8} |
| P3 | D >= 8, or the cell stops at its cap | falsified only by a completed cell with D <= 7 |
| P4 | zero difference | falsified by any pair of completed cells at equal N with different D |
| P5 | wall_s < 7200 and peak RSS < 8 GiB on both towers | falsified by a cap stop. Diagnostic only unless run on isolab-eks L2 |

The five claims in `protocol.json` map as: C1 = P1, P2; C2 = P3; C3 = P4; C4 = P5; C5 is the descriptive distance to the recorded m = 4 cells (out of scope as a decision).

**Kill criterion.** If P1 or P2 show D >= 8 at N <= 15, the m = 5 line is closed on prime towers as no cheaper than m = 4 at equal N.

## Controls

- **Positive**: re-run the recorded m = 4, N = 8 Kummer cell (D = 6, 1.5-2.7 s) and the m = 3, N = 15 Kummer cell (D = 7) with the same binary; require identical D and rows identical to the recorded rows in every field except `ms` and `wall_s` (the pilot prints no system dump, so the exported system is compared through its curve, tower, `x_r`, `input_degrees` and `basis_len` fields). Any difference voids the binary and all m = 5 rows.
- **A/A**: the two random targets of a cell are independent draws and must give the same D (the within-cell A/A); registered as arms `aa-target0` and `aa-target1` on the m = 4, N = 8 cell, and checked on every m = 5 cell by `analyze.py`.
- **Negative**: m = 2, N = 10 (Kummer D = 4, known; v1 read 5, corrected by amendment AM1) as the null-chain comparison; any other D stops the study.

## Variable ledger and comparisons

Independent: m (fixed at 5), t (2, 3, 4), tower kind (kummer, isogeny). Fixed: prime, engine, seed, binary sha256, host, caps. Randomized: target index (0, 1).

- `C-m5-t2` (one_factor): D(Kummer) = D(isogeny) at N = 10, and D in {6,7}.
- `C-m5-t3` (one_factor): D(Kummer) = D(isogeny) at N = 15, and D in {7,8}.
- `C-m5-t4` (one_factor): D(Kummer) = D(isogeny) at N = 20; D >= 8 or cap.
- `C-m5-vs-m4` (confounded): Descriptive: D at m = 5 against the recorded m = 4 cells at equal N (recorded m = 4: N = 8, 12, 16, 20; shared with m = 5 only at N = 20).
- `C-aa-targets` (aa): The two random targets of each cell are independent draws and must give the same D.

## Metrics

Primary: closure degree D (exact, `solving_degree_max` of a completed trace). Secondary: wall seconds, peak RSS (`/usr/bin/time -v`), widest matrix columns, steps. Operation counts and D need no isolation; wall seconds are diagnostic only unless the run is on isolab-eks L2 (state which in the report).

## Execution

Backend: isolab-eks on EKS Spot per AGENTS.md section 9a (registered as `isolab-k8s`); a local development smoke of the t = 2 cell is permitted and is labelled not evidence. Resume-checkpoint and Conductor requirements of the repository apply at launch.

## Stopping rules

- (budget) Each cell has a wall budget of 7200 s (t = 2, 3) or 86400 s (t = 4) and a memory cap by prlimit --as of 8 GiB (t = 2, 3) or 120 GiB (t = 4, only on a host with that memory). A wall or memory stop is inconclusive_cap, not evidence.
- (preregistered_sequential) Stop the t = 4 cells if a t = 3 cell exceeds its cap.
- (invalidation) Run the three controls first. If the m = 4 N = 8 or m = 3 N = 15 control differs from its recorded D or rows, or the m = 2 N = 10 negative control does not give D = 4 (AM1; v1 read 5), stop and run no m = 5 cell.
- (fixed_n) Run each cell once per process (--t-min T --max-t-m4 T); 12 target rows plus controls, no extra cells.

## Decision rule

Per cell, D is the exact solving_degree_max of a completed trace; no sampling interval. P1 to P5 are evaluated as stated in hypotheses H-m5. Kill criterion: if P1 or P2 show D >= 8 at N <= 15, the m = 5 line is closed on prime towers as no cheaper than m = 4 at equal N. A cell stopped at its cap is inconclusive_cap and is never reported as a degree. No multiple-comparison correction applies because every cell is an exact classification, not a significance test. Gap to the matched rho is not a gate here; this study reports degree and cost only.

Verdict vocabulary: supported, refuted, inconclusive_cap.

## Analysis plan

`analyze.py` (stdlib only) reads `cells/*.jsonl` and `cells/*.time`, prints D, wall and RSS per row, the control and A/A checks, and the P1-P5 verdicts; it exits 0 and tolerates missing files. Blinded: false. Frozen before data: true.

## Inadmissible

- Changing the prime, seed, engine or caps after the freeze
- Reporting a capped (wall or memory stopped) cell as a degree
- Using wall seconds for a speed or comparative claim unless the run is on isolab-eks L2
- Treating a local development smoke as evidence
- Generalizing m = 5 toy cells to cryptographic scale or another presentation

## Amendments

None.

## Literature and records

- `research/factor_base_arity_review_20261024/README.md`: Section 1.2 records m = 2, 3, 4 only; m = 5 was never measured.
- `research/ic_next10_20261014/03-pkm-degree/README.md`: Sections 2.1-2.2 and results.csv give the m = 2, 3, 4 degrees and costs used as predictions and controls; D independent of prime and tower kind over 82 cells.
- `research/pkm_m3_n21_msolve_20261018/RESULTS.md`: m = 3, N = 21 died at 99.6 GiB with D >= 8; msolve at m = 4, N = 20 died at a 5 GB cap after 3 degree-8 rounds.
- `research/pkm_tower_round3_20260925/README.md`: Source of the command-line flag style for the tower engine.

## Amendment AM1 (2026-11-04, version 2, filed before any m = 5 data)

v1 set the m = 2, N = 10 Kummer negative control to a recorded D = 5. The
records it cites give 4: `research/pkm_tower_round2_20260925/runs/XV1-m2-tower-null.jsonl`
(p = 786433) and `XV5-kummer-m2-p32.jsonl` (p = 3221225473) both record
`solving_degree_max` 4 on the two random targets of the Kummer m = 2, t = 5
cell; D = 5 at N = 10 is the isogeny tower's value, and the Kummer chain
reaches 5 at N = 12. Attempt 1 on Iso Lab EKS measured 4, 4 at p1 and
stopped before any m = 5 cell, as v1 required; the two positive controls
reproduced 6, 6 and 7, 7. Only the expected value of that control changes;
claims, predictions, cells, commands, caps, seeds and `analyze.py` do not.
