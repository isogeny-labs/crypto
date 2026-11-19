# Making unsolvable decomposition systems cheaper to prove: static occurrence-ranked relabelling (refuted)

2026-11-19. Everything here is [pcc:unverified]. Single host, 4-vCPU development VM, one thread, not an
admitted Iso Lab measurement: no leaderboard, no `benchmarks/` row, no replay certificate.

## Result

**Negative.** Renaming the variables of a decomposition system by occurrence count before the
unchanged `InheritedF4{max_degree:3}` / `HighestFree` solver is exact (0 verdict or root mismatches in
5,760 attempts) but makes the solver stage slower in all 8 cells, in both directions. The
shipped variable layout (including the shipped chain/interleaved order) already beats both static
occurrence orders. Effect on total relation-phase (solver-stage) cost, counted word ops, arm / base,
geometric mean over 60 targets (95% bootstrap CI):

| cell | UNSAT targets | A/A word ops | freq_hi (primary) | freq_lo (exploratory) |
|:--|--:|--:|--:|--:|
| K(0,13) dim 5 | 33/60 | 1.0001 | 35.6 (35.0-36.1), 180/180 censored | 3.27 (3.16-3.38) |
| K(0,15) dim 5 | 59/60 | 1.0000 | 13.0 (9.3-17.6), 57/180 cens. | 4.64 (4.51-4.77) |
| K(1,15) dim 5 | 41/60 | 1.0000 | 12.3 (8.7-16.7), 81/180 cens. | 4.18 (4.05-4.34) |
| K(1,17) dim 6 | 41/60 | 1.0000 | 11.6 (8.8-14.7), 174/180 cens. | 4.61 (4.32-4.82), 33 cens. |
| K(1,19) dim 7 | 36/60 | 1.0000 | 11.7 (11.5-11.9), 180/180 cens. | 3.02 (2.88-3.17), 180 cens. |
| K(0,19) dim 7 | 24/60 | 1.0000 | 12.3 (12.1-12.6), 180/180 cens. | 2.81 (2.67-2.95), 180 cens. |
| K(0,23) dim 8 | 37/60 | 1.0000 | 1.38 (0.95-1.97), 180/180 cens. | 1.24 (1.21-1.27), 180 cens. |
| K(1,23) dim 8 | 44/60 | 1.0000 | 2.06 (1.45-2.84), 180/180 cens. | 1.27 (1.24-1.30), 180 cens. |

Wall-time ratios track the word-op ratios (freq_hi 1.4 to 40.8x, freq_lo 1.08 to 4.6x; per-cell values in
`results.json`). "Censored" = the arm hit the 2000-reduction node budget; its cost is then a lower bound,
so every freq_* ratio above is a lower bound on the slowdown (base never exhausted). Counts at n = 23
are the least informative because both arms are censored there. Per-attempt means of the shipped solver
stage: 5 to 236 ms median per target depending on n (`base_ms_median_per_target` in `results.json`).

Decision rule (PROTOCOL): improving only if the word-op and wall upper CIs are below 1 and A/A contains 1.
Met in 0 of 8 cells.

### Distance to rho

No row improves, so the distance to the matched rho does not shrink; it grows in the solver stage by
at least the ratios above. For scale only (different factor bases, cold full-DLP rows, not matched to
these cells): `research/f4_f5_ecbench_20261019/README.md` records f4-baseline vs rho-strong solve
medians of 91.3 vs 2.68 ms at n17 (about 34x) and 2592.7 vs 2.78 ms at n23 (about 930x). The
candidate would move those gaps up, not down. `ecbench band` weights w = 0, 0.1, 1 were not
computed: the candidate is not an ecbench oracle (no `src/` edits allowed in this lane), so there is no
session to band. The counted unit here is `bitop/v1` word ops of the solver stage only.

## Where UNSAT work goes (profile on the shipped solver)

Refuted systems (0 roots) are 40 to 99% of base counted word ops per cell (share of word ops in
UNSAT attempts: 0.54, 0.99, 0.67, 0.68, 0.60, 0.40, 0.62, 0.73 in the table order). Systems with roots
cost more per attempt because the splitting tree must enumerate every root (all-roots mode), so the
share is a property of this all-roots harness, not the first-witness pipeline. Refutation nodes in the
shipped tree are already cheap (research/lit2026_w4_early_abort_20261011); this lane did not
re-profile degrees (no new data beyond word ops, reductions, splits, propagations per attempt).

## Predictions and outcomes

| id | prediction | outcome |
|:--|:--|:--|
| P1 | 0 verdict/root mismatches (hard gate) | **held**: 5,760 attempts, harness never aborted, no stderr |
| P2 | A/A word ops exactly 1.000, wall within 0.97-1.03 | **partly**: word ops 1.00000 to 1.00006 (the `bitop` tally of system cloning differs by ~3e3 ops between identical arms; native counts identical and repeatable); wall A/A 0.982-1.005, all inside 0.97-1.03 |
| P3 | freq_hi improves in >= 4 of 8 cells | **falsified**: 0 of 8 (not blind for K(1,17), see below) |
| P4 | freq_lo worse than base by word ops in every cell | **held**: 8 of 8 (1.24 to 4.64) |
| P5 | UNSAT >= 70% of base word ops in every cell with n >= 17 | **falsified**: 0.68, 0.60, 0.40, 0.62, 0.73 (only K(1,23)) |
| P6 | effect on relation phase = solver-stage word-op ratio; scale rho reference | **done as stated**; scale only, see above |

## Deviations and caveats

- A 2-target smoke run (K(1,17), seed 1) preceded the protocol; it showed the same direction. The registered
  run used seed 20261119 and fresh targets; the smoke output was discarded. P3 is therefore not blind there.
- The verdict is the full root set of the raw Semaev/descent system (trace row included), not the lifted
  factor-base decomposition of `groebner_decompose`; this is a stronger gate but excludes the F6-IC node oracle
  and the first-witness stop. The all-roots mode inflates the share of SAT-system work.
- Because the budget censors freq_* arms, their CIs are on lower bounds. The gate for censored arms is
  soundness only (returned roots must be in the base set); non-censored attempts matched exactly.
- The protocol said wall time per attempt as an outcome; wall is single-thread on a shared 4-CPU host
  (other lanes ran concurrently), medians of 3 repetitions per target.
- research-kb MCP was not available in this subagent; prior art was checked by grepping `research/`
  and the option docs (list in PROTOCOL.md). A candidate not covered by that grep could exist.
- One new file outside the lane dir: `examples/ic_unsat_relabel.rs` (public APIs only, as permitted).
- What would still be untested: dynamic (per-node) branching such as look-ahead or conflict-driven
  scoring, which needs `choose_split` in `src/`; not attempted (no `src/` edits).

## Files

PROTOCOL.md (frozen before the registered run), run.sh, analyze.py, results.json (derived, with raw
SHA-256), figure.svg, `.gitignore` (raw/ ignored, 1.4 MB, 8 JSONL files of 720 lines).

Raw SHA-256:
- a0_n13.jsonl 2e76050629dce8d58fbca1a512ba09a208bfdd6528eef5414cea3ea507aef173
- a0_n15.jsonl 48d41c16b3a5c0aa83aed808c4f508b895ae9b293d247074125d63ca89e05251
- a0_n19.jsonl 5c9ac1cf31688d35c9c61e46199b8fd7176d660772b68b8215811562c6a94d6b
- a0_n23.jsonl 108c3c9f4d8a7e5732053b044c8f6b2eb76815b85ee1528fc5482002401454d4
- a1_n15.jsonl f4aa1dc7fa904956a509cbfac20fa981a461ec357f7bbd776f640b1d22e4ba1e
- a1_n17.jsonl fe4fa6a609b8334883839517034bdc4316fcce8a49834767ccccf117d45d4c3a
- a1_n19.jsonl 359a5d3b6980c32554874fa5dcb7ad9d62f7c6d3b054a62b675299845808e22b
- a1_n23.jsonl a19397f75c4da0156641b544c34c2c9603f7b16ff08ec482d1718b5574c9a87f

## Reproduce

```
CARGO_INCREMENTAL=0 cargo build --release --example ic_unsat_relabel   # from /home/user/crypto
research/ic_binary_unsat_cost_20261119/run.sh     # about 35 minutes single thread
python3 research/ic_binary_unsat_cost_20261119/analyze.py
```
