# Validator report VAL-20261118-1e81cb: `ic_ksum_split_20261118` (k-sum `split=1` and `targets=stride-renew`, Table B)

Independent validation of the producer's evidence at commit `8aff81b841` (branch `claude/f4-f5-algorithm-speed-bvp7qd`).
The study directory is unchanged at the branch head when this report was written (`git diff 8aff81b841 HEAD -- research/ic_ksum_split_20261118` is empty).
The validator did not edit, repair or regenerate any producer file. Incumbent reference: `research/ic_ksum_prune_20261115`,
validator VAL-20261115-276156.

## Verdicts

| claim | verdict | basis |
|:--|:--|:--|
| **A. `split=1` alone** (`ksum<k>-ps` against `ksum<k>-p`) | **passed** | Registered T1 holds on 8 of 8 rows for k = 5 (primary) and k = 4. The pooled and every per-replicate upper end is below 1 at w = 0, 0.1 and 1, and the Holm 97.5 % reading is the same. `ksum5-ps / ksum5-p` is 0.9103–0.9432 at w = 0. The build additions are removed, not relabelled: 9,216 of 9,216 paired runs are identical apart from the oracle-setup adds, which fall by exactly `2^{k-2} C(a-1, k-1) - 2 C(a-k+2, 2)` on every run. |
| **A + B combined** (`ksum<k>-psr` against `ksum<k>-p`) | **failed** as an 8-row claim. **Passed** on the 7 rows P20, P22, P24, B15, B18, B21 and B27. | The registered primary T2 (k = 5) fails on B24. Replicate r2 at w = 1 has bootstrap upper 1.0114 (ratio 0.9387). The secondary T2 (k = 4) fails on P24 and B27, both at w = 1 per replicate. Every headline number recomputes. The producer's verdict booleans and row lists match the frozen rule exactly. The best-against-best range 0.525–0.866 (w = 0) is a reported reading, not a registered test, so it cannot carry B24. |
| Validation gate (leaderboard / `benchmarks/README.md`) | **not met** | The 24 raw `records.jsonl` (256 MB) are not in the repository or in the artifact store. They exist only in a scratch directory, and the commit pins them by digest alone. There is no `VALIDATION.json`, replay certificate file or JSON validator report. See "Gate status". |

Lever B (`targets=stride-renew`) removes real, counted work. It is fully charged and single-target cold, and it is not an accounting change (question 1). It is an existing option that earlier studies had already validated on other oracles. The gain is engineering: a configuration adopted by the k-sum pipeline. It is a constant set-up saving that shrinks as r grows.

## Answers to the four specific questions

**1. Is `stride-renew` fully charged and single-target cold? Is it an accounting gain?** It is fully charged and cold, and it is not an accounting gain.

- **Implementation** (`src/cryptanalysis/ic_boundary.rs`): `TargetSource::StrideRenew` is at about line 13645. The target loop is at about lines 14284–14330 and 14526–14625.
  - S and T are drawn by `draw_jumps`, and R0 by `g.mul`/`g.add`. Each draw is two scalar multiplications plus one addition on `rel.group_ops`.
  - Each target step `R <- R + S` is one counted `g.add`. Each renewal `S <- S + T` after a relation is one counted `g.add`.
  - The repeat guard is one counted probe (`target_guard_probes`, priced in the band through the pinned `ns_per_lookup`). A repeat is stepped past rather than restarted.
  - All state is local to `collect_relations` for one target. Every run is a separate child process: each record carries `exit 0`, `process_wall_ns` and `max_rss_kib`. No in-memory state carries across targets or rounds.
- **Records** (validator census over all 18,432 stride-renew runs):
  - The relation phase has exactly 6 scalar multiplications in 18,358 runs. The other 74 runs (0.4 %) have 8: one fresh R0 after the progression hit the identity, charged. So "no restarts" is true of self-collision restarts only; 74 runs had one identity restart.
  - The walk arms average 41.6 (k = 4) and 40.6 (k = 5) relation-phase scalar multiplications per run. They also average 4.9 and 4.3 guard-forced walk restarts per run (pooled offsets, charged).
  - `stride_renewals = relations` on every run.
  - The factor base and the whole `oracle_setup` phase (adds, inserts, native and work counters) of `-pr` equal those of `-p` on 9,216 of 9,216 runs. Stride-renew changes only the relation phase.
- **The problem is unchanged.** On every (target, round), every arm saw the same workload (`workload_sha256`) and the same `algorithm_seed`. The factor base is the incumbent's, with no known-log columns.
  - Targets are `[a]G + [b]Q` with random, tracked `a, b`. Every answer is verified inside the run.
  - The validator checked `[d]G = Q` with its own elliptic-curve arithmetic on 960 records drawn by nonce (0 failures), and that G has order r on all 24 session curves.
- **No hidden work moved off the books.** The work the walk does and the band leaves unpriced (per-step jump hashes, coefficient updates mod r, random draws) is removed or reduced by stride-renew, not added. Its own extra native work is 2 mod-r adds per renewal.
  - Cross-check in the `bitop/v1` unit, which counts that work: `ksum5-psr / ksum5-p` = 0.573 (B15-r1), 0.814 (B24-r2), 0.863 (B27-r1), 0.839 (P24-r1). These are the same direction and size as in GAE.
- **Why the incumbent did not use it.** No documented exclusion exists. `rg -i stride.?renew` finds nothing in `research/ic_ksum_prune_20261115`, `research/ic_tableb_ksum_20261115` or `research/ic_tableb_binary_ksum_20261115`: those studies inherited the `ic.pipeline` default `targets = walk`.
  - Stride-renew is a validated lever elsewhere:
    - `research/ic_stride_renew_prime_20261029`: prime IC with the `mitm` oracle, an improvement at 20 and 28 bits, null at 24 and 32 bits (VAL-20261029-4aed57).
    - The Koblitz planner rule `koblitz.targets.stride-renew` (`src/cryptanalysis/ic_framework/planner.rs`): 0.40–0.75 of the walk for log2 r ≤ 21, and about 1 above.
  - One documented caution concerns a different oracle: `research/curve25519_autolab_20261104` round 030 reports progression clustering costing 2.6 times the probes for `wsum2r` at k = 32.
  - This study measures stride-renew directly on these rows, so the caveats are tested, not assumed. They show in the data: `ksum5-pr / ksum5-p` alone is 0.611 (B15) but 0.956 (B27) at w = 0, and it fails the per-replicate rule at w = 1 on 6 of 8 rows. That rule is not registered for `-pr`; this is an attribution reading.
  - The gain from B is a fixed set-up saving (about 35 scalar multiplications and the walk restarts) set against sqrt(r). It vanishes with size and does not change any exponent. The producer states this.

**2. Does `split=1` remove additions that are really performed, with identical table contents?** Yes.

- **Code** (commit `fc0babc73e`; `9cb70209f3` changes only a test bound):
  - `build_negation_folded_split` forms each k-sum as prefix plus a precomputed pair sum. The pair-sum table costs two counted adds per abscissa pair; the negatives come from `g.neg`.
  - The (k-1)-summand partial sums are never computed.
  - The identity skip `prefix + P_j = O` becomes the equality test `prefix == P_{neg(j)}`. This is exact, because the factor base stores both signs and a 2-torsion point is its own negative.
  - Visiting order and inserts are unchanged.
  - The pair table is rebuilt in every run's `setup` (cold), and `ops.merge(table.build_ops)` charges it to `oracle_setup`.
- **Records.** Validator code compared `ksum<k>-ps` with `ksum<k>-p` on every (target, round), warm-up included, over 24 sessions × 2 k × 192 runs: 9,216 pairs.
  - These are identical: outcome, recovered log, counters, factor base, every phase's `native` and `work_native` (including `table_insert_attempts`), and every phase's adds, doubles, scalar multiplications and GAE outside `oracle_setup`.
  - The `oracle_setup` doubles and scalar multiplications are also identical.
  - The setup adds fall by exactly the formula on all 9,216 pairs: 60–2,568 adds, 0 below the formula. `total_gae` falls by exactly the same amount.
  - The same holds for `ksum<k>-psr` against `ksum<k>-pr`: 9,216 pairs, 0 mismatches.
- **Work the split adds and nobody counts:** up to `2 C(a-k+2, 2)` point negations per build (one field negation or XOR each), and a pair table of `(2a)^2` points (under 100 KB at these sizes).
  - Even pricing each negation at a full GAE leaves a net saving on every row. The smallest case is B15 k = 4 (a = 7): 60 adds saved against 20 negations.
- **Unit tests.** `cargo test --release --locked --lib ksum` was run on a clean sparse worktree of `8aff81b841`, whose `src/` is identical to `e93c3cb2da`, the measured binary's source. It ran under the CPU lock. 13 passed and 0 failed, including both split tests ("Unit tests" below).

**3. Is `rho-neg` the correct matched reference?** Yes, and it is the same as in the prune study.

- The `rho-neg` arm in every spec is JSON-equal to the prune study's (`rho.negation`, role `reference`). So are `rho-neg-aa`, the curves, factor bases, timeouts, rounds and order. Only the seeds are fresh, as registered.
- It runs in the same session and binary as every k-sum arm, on the same targets and seeds per round.
- Negation is the strongest available symmetry:
  - The three prime curves have a ≠ 0 and b ≠ 0, so j is not 0 or 1728 and Aut = {±1}.
  - For the five binary curves, the validator checked in GF(2^n) that a and b lie in no proper subfield. So no Frobenius fold exists, and records give `automorphisms_available = 2`.
- **§5a band.** Rho prices `canonicalisations_uncharged` at w. The k-sum arms price `table_insert_attempts` and the pinned native counters (lookups, guard probes, row ops) at w. Rho stays ahead of every k-sum arm on every row at w = 0, 0.1 and 1, so the order holds across the band.
- **Distance remaining (validator recompute, ratio of totals).** w = 0: 4.95–5.73 to 2.78–4.97. w = 0.1: 5.08–5.90 to 2.92–5.20. w = 1: 5.15–6.70 to 3.66–6.28. Split alone at w = 0: 4.55–5.22.
- **Positive control of the incumbent.** With fresh seeds, the incumbent and rho-neg reproduce the prune study's values per row within 5 %.
  - For example, `ksum5-p / rho-neg` at w = 0 was 5.07 / 5.22 / 5.50 / 5.70 / 5.14 / 5.03 / 5.38 / 5.65 there.
  - It is 5.02 / 5.42 / 5.42 / 5.62 / 5.23 / 4.95 / 5.28 / 5.73 here.

**4. Do the reported per-replicate failures at w = 1 on B24, P24 and B27 match the frozen rule?** Yes.

- **The frozen rule** (PROTOCOL.md §3, registered `55310069ea`, implemented by the registered `analysis/evaluate.mjs`): a row holds if both of these hold:
  - The upper end (the larger of the linearised-t and cluster-bootstrap 95 % ends) is below 1 at all three weights, pooled and in each replicate.
  - `1 - max_w upper > max_w |R_aa - 1|`.
- **The validator's result.** Reading all 912 `ecbench compare` outputs with its own code, the validator gets exactly the producer's row lists:

  | test | rows that hold |
  |:--|:--|
  | split5 | all 8 |
  | split4 | all 8 |
  | combined5 | P20, P22, P24, B15, B18, B21, B27 (fails B24) |
  | combined4 | P20, P22, B15, B18, B21, B24 (fails P24, B27) |

  The producer's per-row maximum uppers agree to 1e-12.
- **The failing cells.** All are at w = 1, per replicate:

  | cell | ratio | lin-t upper | bootstrap upper |
  |:--|--:|--:|--:|
  | B24 `ksum5-psr` r2 | 0.9387 | 0.9947 | **1.0114** |
  | P24 `ksum4-psr` r1 | 0.9677 | 1.0129 | 1.0266 |
  | P24 `ksum4-psr` r3 | 0.9607 | 0.9921 | 1.0036 |
  | B27 `ksum4-psr` r1 | 0.9827 | 1.0160 | 1.0228 |
  | B27 `ksum4-psr` r2 | 0.9699 | 1.0015 | 1.0124 |
  | B27 `ksum4-psr` r3 | 0.9667 | 1.0021 | 1.0087 |

  Every pooled upper is below 1.
- **The producer did not overclaim.** `summary.json` gives `combined5_holds_all_rows = false` and `combined4_holds_all_rows = false`. The README states "7 of 8 rows" and "6 of 8 rows". The commit message says the registered rule holds on 7/8 (k = 5).
- **The Holm reading** ("each primary test at 97.5 %") was required by the protocol but not computed by the producer. The validator computed it with its own linearised t and its own two-stage cluster bootstrap. That bootstrap reproduces ecbench's 95 % uppers to within 0.004, which is Monte Carlo noise.
  - It gives the same rows: T1 (k = 5) holds on 8 rows, T2 (k = 5) on 7 (B24 fails at 1.022).
  - The closest holding cells are P22 r3 at 0.9966 and B27 r2 at 0.9967, both at w = 1. These are within Monte Carlo noise of 1, so they are fragile.

## Artifact and protocol checks

| check | result |
|:--|:--|
| Protocol frozen before data (git order) | Registration `55310069ea` was at 05:09:08Z. Amendments A1 `23f646f70c` and A2 `e9327fdea5` came before any session. A3 `26dac15b74` (new binary) was at 05:47:25Z. All 24 measured sessions started between 05:47:31Z and 05:56:09Z, after A3 (`session.json` `started_unix_ms`). Predictions, specs, rows and `evaluate.mjs` match `REGISTRATION.sha256`. |
| Post-data changes | A4 (container restart; B27-r3 re-run from its spec) was committed with the results. Counted cost is deterministic in the seeds, so a re-run cannot select. `analysis/correctness.mjs` changed after data in `8aff81b841` without a registration hash. The change is I/O only: it reads uncompressed `records.jsonl` when present, and the logic is unchanged. The validator's own check reproduces its result. `analysis/summary.mjs` is new and tabulation only. |
| Session digests | All 96 entries of `data/sessions.sha256` (records, plan, session, spec × 24) verify against the raw sessions. |
| Specs | Each session's `spec.json` is JSON-equal to the committed `specs/<row>-r<n>.json`. |
| Binary | `ecbench-split2` sha256 `80851139…ffe161` = `BINARY.sha256` = every record's and session's `binary_sha256`. Sessions record `git_commit = null` and `git_dirty = null`, so the source binding rests on `BINARY.sha256`. `target/release/deps/ecbench-cbccabe447eb2ab0` (mtime 05:47) has the same sha256. The validator did not rebuild the binary (see Limitations). `src/` is identical between `e93c3cb2da` and `8aff81b841`. |
| Coverage | 24 sessions `complete`, 1,920 records each (46,080). 34,560 measured. 46,080 `verified` with `matches_target`. 0 censored, 0 unpaired. Every pooled comparison has 432 pairs on 144 workloads. |
| Seeds | Fresh `target_seed = 20261118000 + idx` and `seed = 20261118500 + idx`, as registered. Every (target, round) has the same workload and algorithm seed across the 10 arms. |
| Route | Local L0 with `--cpus none`, counted cost only (a declared deviation from isolab-eks). No wall time is claimed. The prune study was also counted L0. |
| Replay | `ecbench verify --replay-all --exit-code` on 8 sessions drawn by nonce (`os.urandom`, drawn after `8aff81b841`): nonce `fb2db1e6be9cc59e7bc247a0581cb151`, one replicate per row, P20-r3, P22-r2, P24-r2, B15-r1, B18-r1, B21-r2, B24-r1, B27-r3 (the re-run session). All `audit OK`: 15,360 records, 11,520 replays reproduced, 0 problems. The producer's 24 committed receipts also say `audit OK` (34,560 replays). The replay used the producer's binary, so it shows determinism and record integrity, not code correctness. Answer correctness was checked separately with the validator's own EC arithmetic (question 1). |

## Metric recomputation

Every registered ratio was recomputed two ways.

- **(i) `ecbench compare --weight 0/0.1/1 --resamples 4000 --seed 20261118`**, pooled and per replicate: 912 comparisons, all admissible.
- **(ii) The validator's own pricing from the raw records**, sharing no code with `analysis/*.mjs`:
  - `S_w = (G0 + w N) / sqrt(r)`.
    - G0 is the sum of adds and doubles for records with pinned native work, and `total_gae` otherwise.
    - N is the sum of the `cost.unpriced` counts, `table_insert_attempts` (behind `counted:`), and the pinned `PRICED_NATIVE` counts.
  - It agrees with (i) to 2.1e-15 relative on all 576 ratios checked.

Ratios are `cost(X) / cost(ksum<k>-p)`, ratios of totals over 432 pairs. Bold marks a registered failure.

| row | `ksum5-ps / ksum5-p`, w = 0 / 0.1 / 1 | T1 k = 5 max upper, pooled / replicate | `ksum4-ps / ksum4-p`, w = 0 / 1 | `ksum5-psr / ksum5-p`, w = 0 / 0.1 / 1 | T2 k = 5 max upper, pooled / replicate | T2 k = 5 | T2 k = 4 | `ksum5-pr / ksum5-p`, w = 0 / 1 |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| P20 | 0.9185 / 0.9270 / 0.9621 | 0.9631 / 0.9639 | 0.9582 / 0.9818 | 0.7746 / 0.7981 / 0.8965 | 0.9275 / 0.9649 | holds | holds | 0.8560 / 0.9343 |
| P22 | 0.9127 / 0.9221 / 0.9605 | 0.9613 / 0.9628 | 0.9495 / 0.9782 | 0.8145 / 0.8332 / 0.9093 | 0.9373 / 0.9875 | holds | holds | 0.9018 / 0.9488 |
| P24 | 0.9183 / 0.9277 / 0.9645 | 0.9653 / 0.9662 | 0.9516 / 0.9796 | 0.8488 / 0.8640 / 0.9235 | 0.9522 / 0.9734 | holds | **fails** | 0.9304 / 0.9590 |
| B15 | 0.9245 / 0.9291 / 0.9541 | 0.9546 / 0.9558 | 0.9555 / 0.9733 | 0.5358 / 0.5637 / 0.7160 | 0.7309 / 0.7529 | holds | holds | 0.6113 / 0.7620 |
| B18 | 0.9383 / 0.9426 / 0.9647 | 0.9655 / 0.9663 | 0.9649 / 0.9804 | 0.5621 / 0.5921 / 0.7468 | 0.7708 / 0.8008 | holds | holds | 0.6238 / 0.7820 |
| B21 | 0.9195 / 0.9279 / 0.9626 | 0.9635 / 0.9642 | 0.9584 / 0.9818 | 0.7791 / 0.8027 / 0.9010 | 0.9316 / 0.9739 | holds | holds | 0.8596 / 0.9384 |
| B24 | 0.9432 / 0.9500 / 0.9758 | 0.9765 / 0.9773 | 0.9387 / 0.9719 | 0.8208 / 0.8426 / 0.9263 | 0.9642 / **1.0114** | **fails** | holds | 0.8776 / 0.9505 |
| B27 | 0.9103 / 0.9203 / 0.9601 | 0.9608 / 0.9615 | 0.9466 / 0.9769 | 0.8665 / 0.8805 / 0.9368 | 0.9626 / 0.9921 | holds | **fails** | 0.9561 / 0.9767 |

**Best against best** is a reported reading, not a gate. On each side it takes the cheaper of k = 4 and k = 5 by point estimate. The interval is pooled only.

| row | w | best before | best after | after / before [95 % lin-t; bootstrap] | gap to rho-neg, before → after |
|:--|--:|:--|:--|:--|:--|
| P20 | 0 | ksum5-p | ksum5-psr | 0.7746 [0.7569, 0.7926; 0.7525, 0.7961] | 5.02 → 3.89 |
| P20 | 1 | ksum5-p | ksum5-psr | 0.8965 [0.8714, 0.9223; 0.8649, 0.9275] | 5.77 → 5.17 |
| P22 | 0 | ksum5-p | ksum5-psr | 0.8145 [0.7986, 0.8306; 0.7942, 0.8345] | 5.42 → 4.42 |
| P22 | 1 | ksum5-p | ksum5-psr | 0.9093 [0.8873, 0.9319; 0.8811, 0.9373] | 6.27 → 5.70 |
| P24 | 0 | ksum5-p | ksum5-psr | 0.8488 [0.8323, 0.8656; 0.8282, 0.8702] | 5.42 → 4.60 |
| P24 | 1 | ksum5-p | ksum5-psr | 0.9235 [0.9016, 0.9460; 0.8960, 0.9522] | 6.56 → 6.06 |
| B15 | 0 | ksum4-p | ksum4-psr | 0.5247 [0.5165, 0.5331; 0.5143, 0.5354] | 5.30 → 2.78 |
| B15 | 1 | ksum4-p | ksum4-psr | 0.7102 [0.6953, 0.7254; 0.6916, 0.7294] | 5.15 → 3.66 |
| B18 | 0 | ksum4-p | ksum4-psr | 0.5645 [0.5458, 0.5838; 0.5425, 0.5928] | 5.11 → 2.88 |
| B18 | 1 | ksum4-p | ksum5-psr | 0.7479 [0.7272, 0.7691; 0.7220, 0.7741] | 5.15 → 3.85 |
| B21 | 0 | ksum5-p | ksum5-psr | 0.7791 [0.7635, 0.7951; 0.7584, 0.8005] | 4.95 → 3.86 |
| B21 | 1 | ksum5-p | ksum5-psr | 0.9010 [0.8789, 0.9236; 0.8715, 0.9316] | 5.69 → 5.13 |
| B24 | 0 | ksum4-p | ksum5-psr | 0.8247 [0.8040, 0.8458; 0.7995, 0.8521] | 5.26 → 4.34 |
| B24 | 1 | ksum4-p | ksum4-psr | 0.9303 [0.9106, 0.9504; 0.9059, 0.9567] | 6.04 → 5.61 |
| B27 | 0 | ksum5-p | ksum5-psr | 0.8665 [0.8522, 0.8809; 0.8481, 0.8853] | 5.73 → 4.97 |
| B27 | 1 | ksum5-p | ksum5-psr | 0.9368 [0.9174, 0.9566; 0.9116, 0.9626] | 6.70 → 6.28 |

The w = 0.1 rows lie between these two weights on every row. They agree with the README table to the printed digits.

The producer's headline numbers recompute exactly:

| headline | value |
|:--|:--|
| `ksum5-ps / ksum5-p`, w = 0 | 0.910–0.943 |
| `ksum5-ps / ksum5-p`, w = 1 | 0.954–0.976 |
| `ksum4-ps / ksum4-p`, w = 0 | 0.939–0.965 |
| `ksum5-psr / ksum5-p`, w = 0 | 0.536–0.866 |
| best against best, w = 0 | 0.525–0.866 |
| best against best, w = 1 | 0.710–0.937 |
| gap to rho-neg, w = 0 | 4.95–5.73 → 2.78–4.97 |
| gap to rho-neg, w = 0.1 | 5.08–5.90 → 2.92–5.20 |
| gap to rho-neg, w = 1 | 5.15–6.70 → 3.66–6.28 |

**Predictions** (`analysis/predictions.json`, hash-registered before data) hold on all 8 rows for both k:

- split R_0 within ±0.03 of its prediction; the largest miss is 0.002;
- combined R_0 within ±0.06 of its prediction; the largest miss is 0.014 (B15 k = 5, B27 k = 4).

## Controls

- **A/A.** `rho-neg-aa / rho-neg` is exactly 1.000000 on every row and weight, pooled and per replicate.
  - Counted cost is deterministic in the seeds, so this control shows determinism and correct pairing, not between-arm noise. The A/A term of the gate is vacuous.
  - The noise control that matters is the registered per-replicate rule. It is what fails on B24 (k = 5) and on P24 and B27 (k = 4).
- **Correctness controls.** 9,216 split pairs and 9,216 stride-split pairs are identical, as described in question 2. All 46,080 runs are verified.
- **Incumbent positive control.** The incumbent reproduces the prune study's values (question 3).

## Unit tests

`cargo test --release --locked --lib ksum` passed: 13 passed, 0 failed, 7,605 filtered out.

How it was run:

- Worktree: a clean sparse worktree of `8aff81b841`, detached. Its `src/` is identical to `e93c3cb2da`, the measured binary's source.
- Locking and target: under `flock` on the session CPU lock, with the shared `target/`.
- Time: 07:07–07:23Z.

The tests that bear on the claims:

- `ic_boundary::ksum_wide_index_tests::split_build_is_the_same_table_with_fewer_adds` checks, on prime and binary curves, with and without prune, for k = 3–7:
  - the split table's `map` (keys and packed indices), `collisions`, `insert_attempts` and `entries` equal the unsplit build's;
  - the pair table costs exactly `2 C(a-k+2, 2)` adds;
  - the build adds fall, matching the closed form under prune.
- `ic_framework::plugins::ksum_split_tests::split_runs_are_identical_but_for_fewer_setup_adds` runs the pipeline with k = 4 and k = 5 under walk, and k = 5 under `Targets::StrideRenew`. It checks that the recovered log, trials, relations, relation-phase group operations, native and work counters, setup native and work counters and rank are identical. Only the setup adds fall; setup doubles and scalar multiplications are unchanged.
- `pruned_build_is_the_same_table_with_fewer_adds`, `tables_up_to_k9_answer_every_sum_of_distinct_abscissae`, `ksum_oracle_recovers_logs_with_three_to_seven_summands`, and the `ksum_frobenius` and `known_log` k-sum tests.

Two matched tests are unrelated: `wots_digits_have_correct_checksum` and `native_sdk_checksum_failure_never_verifies_object`.

## Gate status (what is still missing before any leaderboard or benchmarks row)

1. **Raw data must be durable.** The 24 `records.jsonl` live only in a scratch directory under `/tmp`, which a reboot or cleanup can clear. `sessions/` is an uncommitted symlink.
   - The prune study stored them as `records.jsonl.gz` in the artifact store, pinned by per-session `.artifacts.json` (about 1.1 MB gzipped per session here).
   - Until that is done, the gate's "raw data that is in the repository" clause fails, and nobody else can repeat this validation.
   - What is committed (digests, gzipped verify receipts, run logs, `correctness.json`, `results.json`) proves nothing without the raw records.
2. **Gate files are missing.** No `VALIDATION.json` exists, and there is no replay certificate or JSON validator report with `raw_data` paths.
   - This Markdown report is not the gate's `validator-report.json`.
   - The rho descriptor would be `kind: negation`, the same binary, unit `ecbench.gae` per sqrt(r), with the band as above.
3. **A combined (A + B) row can be stated only for the 7 rows where T2 (k = 5) holds.** B24's combined row must say that the registered rule did not hold.

## Limitations

- The validator did not rebuild `ecbench` from source to reproduce sha256 `80851139…`. CPU and memory were contended by a concurrent agent's release builds (about 12 GB of RSS each).
  - The binary-to-source binding rests on `BINARY.sha256`, the identical hash of the 05:47 build artifact in `target/`, and the empty `src/` diff between `e93c3cb2da` and `8aff81b841`.
- Replays re-execute the producer's binary. They test record integrity and determinism, not the binary's arithmetic. The answers were checked independently only on a 960-record sample.
- Counted cost at L0 only. No wall time.
  - Gains are constant factors at 2^14–2^24.4 group orders, and they shrink with r:
    - B's set-up saving is about 35 scalar multiplications per run against sqrt(r).
    - A's saving is the share of the table build in S.
  - No transfer to larger r is supported.
  - This is the generic-decomposition track: bounded by sqrt(r), and it cannot beat rho or BSGS.
- The A/A control is uninformative about noise (determinism), as noted.
- The validator's analysis scripts are not committed. Only this report is committed, at the caller's instruction. Their sha256 are listed below; their pricing formula and rules are stated in full above.

```yaml
validation_report:
  id: VAL-20261118-1e81cb
  task_id: caller-dispatched (no TASK id given)
  study: research/ic_ksum_split_20261118
  validated_commit: 8aff81b841
  run_ids: [24 ecbench sessions P20/P22/P24/B15/B18/B21/B24/B27 x r1..r3; session ids in data/*.verify.json.gz]
  artifact_checks:
    - registration_before_data: pass (55310069ea 05:09Z; A3 26dac15b74 05:47:25Z; first session 05:47:31Z)
    - registered_hashes: pass for specs, rows, predictions, evaluate.mjs; correctness.mjs changed post-data (I/O only, logic unchanged)
    - session_digests: pass (96/96 data/sessions.sha256)
    - binary_sha256: pass (808511395723f84505d4fe8edaf32e66e8d8bff34cdad57777360c5999ffe161 in every record)
    - specs_equal_committed: pass (24/24)
    - coverage: pass (46,080 records, all verified, 0 censored, 0 unpaired)
    - replay: pass (8 nonce-drawn sessions, 11,520 replays reproduced, 0 problems; nonce fb2db1e6be9cc59e7bc247a0581cb151)
    - raw_data_durable: FAIL (outside Git and outside the artifact store)
  metric_recomputations:
    - ecbench compare, 912 comparisons (w 0/0.1/1, 4000 resamples, seed 20261118): all producer numbers reproduce
    - own pricing from raw records: agrees with ecbench to 2.1e-15 relative (576 ratios)
    - T1 split5/split4: 8/8 rows hold (matches producer); Holm 97.5 %: 8/8
    - T2 combined5: 7/8 (fails B24); combined4: 6/8 (fails P24, B27); matches producer; Holm 97.5 % combined5: 7/8
  control_checks:
    - aa: exactly 1 (deterministic; uninformative about noise)
    - split_identity: 9,216/9,216 pairs identical except oracle_setup adds; saving equals the closed form on every pair
    - stride_split_identity: 9,216/9,216
    - stride_build_identical_to_walk: 9,216/9,216
    - unit_tests: cargo test --release --locked --lib ksum at 8aff81b841, 13 passed, 0 failed (both split tests included)
    - answers_independent_ec: 960/960 [d]G = Q; G of order r on 24/24 curves
    - incumbent_positive_control: reproduces prune-study S within 5 %
  heuristic_validation_checks:
    - predictions registered before data: pass; all 32 (row, k, lever) predictions within tolerance
  cost_model_checks:
    - unit: ecbench.gae per sqrt(r), cold, every phase; band w = 0, 0.1, 1 priced identically on every arm, rho included
    - stride-renew fully charged, single-target cold, problem unchanged; bitop/v1 cross-check agrees in direction
    - split removes real group additions; uncounted additions (negations, pair-table memory) bounded and smaller than the saving
  proof_architecture_checks: []
  verdict:
    claim_A_split_alone: passed
    claim_A_plus_B_combined: failed as an all-8-row claim; passed on P20, P22, P24, B15, B18, B21, B27 (T2 k = 5)
    gate: not met (raw data not durable; no VALIDATION.json / replay certificate / JSON validator report)
  limitations: [binary not rebuilt, same-binary replay, L0 counted cost only, constant-factor gains shrinking with r, A/A vacuous, validator scripts not committed]
  artifact_paths:
    - research/ic_ksum_split_20261118/VALIDATOR_REPORT.md
review_attestation:
  joints_owned: [whole-study validation as dispatched; specific questions 1-4]
  sources_read:
    - research/ic_ksum_split_20261118/{PROTOCOL.md, README.md, REGISTRATION*.sha256, BINARY.sha256, registered-at.txt, run_all.sh, specs/*, analysis/evaluate.mjs, analysis/predictions.json, results.json, summary.json, data/sessions.sha256, data/*.verify.err, data/verify.txt, launch-a3-interrupted/*}
    - raw sessions (scratch) records.jsonl, session.json, spec.json x 24
    - research/ic_ksum_prune_20261115/{PROTOCOL.md, README.md, summary.json, specs/*, validator/VALIDATION.json, validator/validator-report.json (header)}
    - research/ic_stride_renew_prime_20261029/README.md (git show); research/curve25519_autolab_20261104/README.md (one line)
    - src/cryptanalysis/ic_boundary.rs (TargetSource, collect_relations target loop, KSumTable split build), src/cryptanalysis/ic_framework/plugins.rs (KSumOracle setup, split tests), src/cryptanalysis/ecbench/{band.rs, compare.rs, stats.rs, methods.rs PRICED_NATIVE}
    - docs/VALIDATION_GATE.md; AGENTS.md (success criterion, section 5a, validation gate)
  read_sibling_reports: false
  verdict: see verdicts above
```

Validator scripts (scratch, not committed), sha256:

| script | sha256 |
|:--|:--|
| `independent.py` (pricing, correctness, census) | `3359f0b42f01ce10deb6aaf6a595e84de49f21087b37c8ef39e35836e0d527f8` |
| `rules.py` (registered rules, agreement) | `575e5fe6f15dec70ea94d49e842771586b5360f0749dfc786187142939045ce3` |
| `holm.py` (97.5 % reading, two-stage bootstrap) | `b967b7ef8af287001df9f5d25f98806546acfbf8e68bcadfad73aacfa534abef` |
| `ec_check.py` (independent `[d]G = Q`) | `bbadafe9663189732e45183b034e635df47cc3e01c797fbf9718176dc2aa7a90` |
| `run_compares.py` (ecbench compare driver) | `dd99f240b1d68289b5ea55370fb95c4d8f8880bddae401ff0b769b5a50dc6b4d` |
