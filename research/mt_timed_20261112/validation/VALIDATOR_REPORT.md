# Validator report: multi-target IC vs rho in wall time (mt_timed, 2026-11-12)

Validator: independent `validator` subagent (Opus 5.5), 2026-11-12. All validator code is in
`scripts/`; none of it is the producer's. Every figure below is the validator's observation or
recomputation; none is a signed claim yet [pcc:unverified].

**Disposition: headline as stated not validated (`validator-report.json` status `fail`,
disposition `insufficient_evidence`). The measurement is validated, and so is an amended headline.**

- `replay-certificate.json`: **verified**. Counters replay exactly, every sampled recovered log
  verifies with the validator's EC code, and the binary rebuilds bit for bit.
- `validator-report.json`: **fail** for the headline as stated. Every headline number
  recomputes exactly, but two parts of the headline are not supported:
  - The selection-bias control does not show "the same pattern": its decisions are unranked in
    3 of 9 cells.
  - "A native operation 0.005-0.009 GAE" is the price of two counted native operations.

  The 28 b T = 1024 "0.93x faster" is fragile.
- `claim-report.json`: **fail**. The statement is not endorsed byte for byte, because of
  "fresh" curves, the native-operation unit and the omitted robustness information. The
  validator endorses the proposed wording in it.

What holds:

- The IC is faster than rho at 2^24: 12 of 12 configurations, 0.78-0.83x.
- The IC is slower at 2^32: 12 of 12 configurations, 1.11-1.29x.
- 2^28 is near parity.
- These directions reproduce in the validator's own pinned rerun on the same host. Pooled over
  8 repeats, that rerun gives 0.77-0.84x at 2^24 and 1.11-1.22x at 2^32 and reproduces all 9
  cell decisions.

## 0. What was validated, and from where

- **Frozen set.** Commit `544f8320631c9d71b7636d0c51ae031472457cf7`, branch
  `aburan28/eloquent-bell-2jerrf`, extracted with `git archive`.
  - The study directory and `examples/mt_fieldops.rs` are clean.
  - The only dirty file is `benchmarks/records/20261112T0422Z-mt-timed-ic-vs-rho.json`. It is
    not part of the study and is not validated.
  - During validation HEAD moved to `88ee570743`, which changes only that bench record. It is
    not validated, and the study and harness are unchanged in it.
- **Protocol commit.** `a992ed6c80` at 04:22:59Z. It contains PROTOCOL.md, run.py, analyze.py
  and the harness. The run started at 04:23:14Z (bench record).
  - The rows record `298863e526`, a bench-record-only commit 8 s later.
  - PROTOCOL.md, run.py and analyze.py are byte-identical at 544f832063.
- **Binary.** The rows record `bin_sha256` `77bb5092…e52e4`. It is the same in all 1836 rows
  and all 6 opcost records, and it equals `target/release/examples/mt_fieldops`.
  - A rebuild of the committed source into a separate target directory
    (`cargo build --release --example mt_fieldops --offline`, rustc 1.97.0) reproduces it bit
    for bit.
  - The validator's timings therefore use the producer's binary, which is the build of the
    frozen source.
- **Nonce.** `3f09704503c71d99b8cdd885e6f42941`, drawn with `os.urandom(16)` at 04:56:27Z,
  before any sampling.
- **Command sequence.** `scripts/run_all.sh`.

## 1. Harness (check 1)

- **Diff from v5** (`git diff 5aab996e55 a992ed6c80`). It adds the `opcost` arm, the `ms_arm`
  variable and the `ms_arm` output field. Nothing else changed.
- **Counters unchanged.** 35 nonce-drawn v5 rows (5 per arm; seeds 41 and 42, v5 curves)
  replay with the timed harness. Every printed counted field is identical: I, M, S, H, L, N,
  phases and the arm extras.
- **`ms_arm` placement** (T-F9).
  - `t0` is taken after the instance is parsed, after the targets are planted (with an
    uncounted field copy) and after the RNG is seeded.
  - `ms_arm` is read right after the arm returns and before the shared counted verification.
  - The same two lines bracket every arm. All method work is inside: factor base, tables,
    relations, walks, restarts, trials and log bookkeeping. Planting is excluded for every arm.
- **opcost** (T-F8).
  - **No dead code.**
    - mul, sqr, inv and padd are loop-carried chains through `black_box`.
    - The native and lookup loops feed values that are `black_box`ed.
    - At param 1.0 every loop runs 4x longer and the counters scale exactly: I 4,000,006,
      M 26,000,012, S 24,000,009. Per-operation times stay nonzero and comparable.
  - **The padd chain is exactly one affine addition (I + 2M + S) per step.** The counters
    1,000,006 / 6,500,012 / 6,000,009 equal the expected values exactly, so there were no
    doublings and no resets.
  - **Caveats to disclose:**
    - Lookups are timed as independent probes (memory-level parallelism), not interleaved
      with dependent field arithmetic as in the harness.
    - The probe mix is 50% inserted keys and 50% uniform in [0, p). At 24 bits with 2^22-2^24
      keys, most "misses" hit (2^24 random keys in [0, 2^24) are about 63% distinct), so the
      hit mix varies with size.
    - Table capacity is 2x the key count.
    - Single segments can be far off. In the validator's first opcost run one mul segment read
      28.8 ns, against 7.3-7.8 ns normally (I/M 6.0 in that run). The producer's 28 b "after"
      padd is 1.30x its own I + 2M + S.
  - **Reproduction.** The validator's 7 opcost runs reproduce the producer's ranges, apart from
    that segment:

    | price | validator |
    |---|---|
    | GAE | 209-267 ns |
    | I/M | 25-34 |
    | canonicalisation + hash | 0.0055-0.0081 GAE |
    | std lookup, 2^10-2^16 | 0.047-0.144 GAE |
    | std lookup, 2^22-2^24 | 0.36-0.73 GAE |

## 2. Correctness (check 2)

| check | scope | result |
|---|---|---|
| harness counted verification | 1836 rows, 13,160,448 targets | ok = targets in every row, 0 errors |
| counter replay, validator build of the frozen source | all 612 run keys (1836 rows) | 612/612 identical (all printed fields except ms_arm, ms) |
| counter replay, producer binary | 17 nonce-drawn keys (51 rows) | 17/17 identical |
| counters across the 3 repeats of a key | 612 keys | identical in every key |
| validator EC check (`ec_check.py`, own affine code) | 54 nonce-drawn keys (162 rows, 30 configurations, 7 arms, 3 sizes) | 319,488 logs; Q_t on curve, log in [0, n), log = planted, [log]G = Q_t; 0 failures |
| instances W{24,28,32}-{0..3} | 12 | p, n prime; [n]G = O; cofactor 1 (Hasse); n of 24-25, 28-29 and 32-33 bits |

`runs.json` is an exact projection of `runs.jsonl`: 0 field mismatches, and `source_sha256`
matches.

## 3. Recompute (check 3; `recompute.json`, `results_check.json`)

- **Method.** `scripts/recompute.py`, the validator's own code. Three bootstraps give
  identical decisions in all 9 cells:
  - the protocol's bootstrap: `random.Random(1)`, B = 4000;
  - the exact bootstrap over all 4^4 resamples;
  - a numpy bootstrap with B = 20000, seeded from the nonce.
- **summary.json.** 272 of 272 fields match, with a maximum relative difference of 0.

| bits | T | ic/rho solve [95%] | decision | fixed-key control | control decision | LOO (validator) |
|---|---|---|---|---|---|---|
| 24 | 1024 | 0.778 [0.765, 0.922] | faster | 0.825 [0.637, 0.961] | faster | 10/12 below 1 |
| 24 | 4096 | 0.792 [0.753, 0.821] | faster | 0.818 [0.731, 0.847] | faster | 11/12 |
| 24 | 16384 | 0.829 [0.811, 0.872] | faster | 0.752 [0.655, 1.211] | **unranked** | 11/12 |
| 28 | 1024 | 0.932 [0.869, 0.967] | faster | 0.975 [0.884, 1.137] | **unranked** | 7/12 |
| 28 | 4096 | 1.053 [0.987, 1.101] | unranked | 1.031 [0.967, 1.143] | unranked | 3/12 |
| 28 | 16384 | 1.007 [0.954, 1.193] | unranked | 1.017 [0.940, 1.077] | unranked | 6/12 |
| 32 | 1024 | 1.114 [1.049, 1.211] | slower | 1.096 [0.972, 1.351] | **unranked** | 2/12 |
| 32 | 4096 | 1.203 [1.116, 1.303] | slower | 1.180 [1.052, 1.305] | slower | 0/12 |
| 32 | 16384 | 1.289 [1.269, 1.319] | slower | 1.284 [1.218, 1.374] | slower | 0/12 |

- **Recomputed exactly:**
  - incl. verify 0.82-1.27, with the same decisions;
  - IC/BSGS 1.33-1.59 (cell medians; 1.23-1.87 per configuration);
  - A/A rep0/rep1 contains 1 in 9/9 cells;
  - A/A median |log| 0.0277, 95th percentile 0.2767;
  - winners: ic5 20, ic5w 16; rhodpu κ = 0 29, rhodpu κ = 0.25 1, rho2xks 6.
- **The n = 4 decision rule (T-F4).** In all 9 cells the protocol's "95%" interval equals the
  [min, max] of the 4 configuration ratios. A resample has its median at the maximum with
  probability 13/256, which is more than 2.5%.
  - So a decision means all 4 configurations are on one side of 1. That is a sign test with
    two-sided p = 0.125 per cell.
  - The strong evidence is at the size level: 12/12 configurations below 1 at 2^24 and 12/12
    above 1 at 2^32 (p = 0.00049 each); 7/12 at 2^28.
- **LOO** (validator diagnostic, not pre-registered): leave one repeat out, choose each side's
  key on the other two repeats, and score it on the held-out repeat.
- **Quoted numbers** (`results_check.json`). Four RESULTS.md numbers do not recompute; see T-F1
  and T-F6:
  - "same pattern";
  - rho2xks "7" (recomputed 6);
  - lookup 2^18-2^20 "0.12" (recomputed 0.11);
  - fast hasher "2-5x" (recomputed 1.2-6.1x).

## 4. Timing reproduction (check 4; `timing_reproduction.json`)

- **Setup.** Same host, the bit-identical binary, `taskset -c 2`, sequential, nonce-seeded
  interleaving. The producer's per-configuration fastest keys were selected on the producer's
  medians and scored on the validator's new timings, so the validator selects nothing on its
  own data.
- **Round 1:** 3 repeats, 702 executions.
  - Part A: all 36 configurations × the producer's IC, rho and bsgs2 keys.
  - Part B: one nonce-drawn configuration per cell × all 17 keys.
  - opcost runs before and after.
- **Round 2:** 5 repeats of the IC and rho keys, 360 executions. It was added after round 1
  showed three unranked cells; it is not pre-registered and is reported separately.

| bits | T | producer | validator round 1 (3 rep) | round 2 (5 rep) | pooled (8 rep) |
|---|---|---|---|---|---|
| 24 | 1024 | 0.778 faster | 0.812 [0.733, 0.854] faster | 0.799 faster | 0.819 [0.659, 0.831] faster |
| 24 | 4096 | 0.792 faster | 0.779 [0.768, 0.786] faster | 0.766 faster | 0.768 [0.755, 0.814] faster |
| 24 | 16384 | 0.829 faster | 0.911 [0.811, 1.009] **unranked** | 0.827 faster | 0.845 [0.776, 0.914] faster |
| 28 | 1024 | 0.932 faster | 0.991 [0.970, 1.043] **unranked** | 0.943 **unranked** | 0.980 [0.959, 0.999] faster |
| 28 | 4096 | 1.053 unranked | 0.979 unranked | 1.023 unranked | 1.016 unranked |
| 28 | 16384 | 1.007 unranked | 1.015 unranked | 0.974 unranked | 0.976 unranked |
| 32 | 1024 | 1.114 slower | 1.072 [0.954, 1.109] **unranked** | 1.099 slower | 1.107 [1.045, 1.134] slower |
| 32 | 4096 | 1.203 slower | 1.122 [1.046, 1.146] slower | 1.122 slower | 1.116 [1.089, 1.162] slower |
| 32 | 16384 | 1.289 slower | 1.238 [1.147, 1.257] slower | 1.251 slower | 1.216 [1.191, 1.301] slower |

- **Decisions reproduced:** 6/9 in round 1, 8/9 in round 2 and 9/9 pooled. Every producer
  interval overlaps the pooled validator interval.
- **The three edge cells.** Round 1's unranked cells are exactly the three in which the
  producer's own fixed-key control is unranked.
- **Size level:**

  | size | round 1 | round 2 | pooled |
  |---|---|---|---|
  | 2^24, configurations below 1 | 11/12 | 12/12 | 12/12 |
  | 2^32, configurations above 1 | 11/12 | 12/12 | 12/12 |
  | 2^28, configurations below 1 | 6/12 | 8/12 | 9/12 |

- **Repeat-level ratios in round 2 at 2^32:** 2, 3 and 0 of 20 are below 1.
- **Agreement:**
  - The 2^24 "faster" and 2^32 "slower" reproduce.
  - The 2^32 magnitudes are about 5% lower in the rerun (1.11-1.22 against 1.11-1.29).
  - 28 b T = 1024 reproduces only pooled and only by a hair: 0.980, against the producer's
    0.932.
- **Between-session spread.** Per-configuration ratios differ between sessions by a median of
  5.1% (p90 14%, max 22%). That is more than the within-session A/A median deviation (2.8%) and
  inside the per-cell A/A rep0/rep1 intervals.
- **Independent best-of** (part B, 9 configurations, the validator's own selection over 17
  keys): ratios within ±0.14 of the producer's (largest 28 b T = 1024, +0.13), all on the same side of 1 except 28 b T = 1024
  (1.043 against 0.912).
- **Absolute per-key times:** median ratio 1.004 against the producer's (range 0.70-1.39).
- **Host state** (T-F12). This is the same VM; the producer's rows and the validator's run share
  its boot.
  - **Producer run:**
    - load average 0.55-1.08 (median 1.00);
    - CPU 2 steal of at most 13 s (≤ 0.8%) over a window containing the whole run.
  - **Validator rounds:**
    - steal on CPU 2 was 0.74% and 0.75% of wall time;
    - the other CPUs were 0.66% and 0.62% busy.

## 5. Protocol (check 5)

- **Frozen and followed.**
  - The protocol was frozen before the data.
  - The seeds are as declared: planting 43, order 20261112.
  - The execution order, re-derived from run.py's rule, matches all 1836 rows. Repeats of a
    key are 652-1816 executions apart.
  - There was no post-hoc change to run.py, analyze.py or PROTOCOL.md.
  - The failed size prediction is reported as failed.
- **Wording.** "Faster/slower" is the protocol's decision applied to wall time. Its strength is
  limited as described in T-F4, and its edge cells are not robust (T-F1, T-F3).
- **Limits.** These are stated: one shared VM, diagnostic seconds, no PMU, one implementation,
  std SipHash, 4 curves and one seed, nothing on single-target ECDLP or on ECC2K-130 / m = 83.
  Two more should be added:
  - the between-session spread (median 5%);
  - `std` HashMap's per-process SipHash seed.
- **Interpretation** (T-F7). It is labelled as interpretation. The validator's price model
  (`price_model.json`, not pre-registered) tests it:
  - Field operations alone favour the IC at 2^28-2^32 (0.72-0.78x).
  - The measured lookup price, at each run's own table size, predicts 1.14-1.22x at 2^32
    ("before" prices; 1.09-1.15 "after"), against 1.11-1.29 measured. Lookups are about 40% of
    the IC's predicted time and 1-2% of rhodpu's.
  - So "large-table lookups cost the IC the 2^32 cells" is quantitatively consistent.
  - "Cache misses" is not measured (no PMU) and must stay interpretation.
  - At 2^24 the price list predicts 0.85-1.02 against 0.78-0.83, so it does not explain the
    2^24 advantage.
- **Reporting contract.**
  - The comparison is in one table and one unit, with ratios. Solve and verify are reported
    separately.
  - The rho reference is in the same binary.
  - The null (A/A) and the cheaper baseline (BSGS) are reported.
  - No external implementation was compared; the claim is scoped to this harness.

## 6. Findings

| id | severity | finding |
|---|---|---|
| T-F1 | blocking (headline wording) | fixed-key control unranked at 24/16384, 28/1024, 32/1024; "same pattern" holds for the medians (0.75-1.28) only |
| T-F2 | blocking (claim unit) | 0.005-0.009 GAE is canonicalisation + hash = 2 counted native ops; one is 0.0025-0.0047, a step index 0.0024-0.0057 |
| T-F3 | disclosure | rerun: 6/9 decisions (round 1), 8/9 (round 2), 9/9 pooled; 28/1024 0.98 not 0.93 |
| T-F4 | disclosure | n = 4 bootstrap interval = range; each decision is 4-of-4; size level 12/12 is the strong evidence |
| T-F5 | blocking (claim wording) | the curves are v5's, not fresh |
| T-F6 | non-blocking prose | rho2xks wins 6 (not 7) + rhodpu κ 0.25 wins 1; lookup 2^18-2^20 is 0.11-0.42; fast hasher 1.2-6.1x; "never won in v5's counted model" is false (4/198) |
| T-F7 | interpretation | lookup-price account consistent at 2^32, not at 2^24; cache misses unmeasured |
| T-F8 | pass, caveats | opcost sound (no DCE, padd = 1 GAE exactly); throughput-mode lookups, size-dependent hit mix, noisy segments |
| T-F9 | pass | ms_arm brackets all method work of every arm, excludes planting and verification |
| T-F10 | pass | protocol before data; binary rebuilt bit-identical; 612/612 keys replay; v5 counters unchanged |
| T-F11 | pass | 319,488 logs EC-verified, 0 failures; instances valid |
| T-F12 | pass, disclosure | load and steal small in both sessions |
| T-F13 | non-blocking | runs.jsonl (1.36 MB) in Git, not the artifact store |
| T-F14 | non-blocking | smoke run (W24-0, seed 43, T = 1024, also a holdout configuration) not preserved |
| T-F15 | pass | protocol followed; limits stated; add between-session spread and the per-process SipHash seed |

## 7. Required amendments

1. **T-F1.** Replace "Selection-bias control ... 0.75-1.28x, the same pattern" with the
   control's decisions:
   - faster at 2^24 T = 1024 and 4096, unranked at 2^24 T = 16384;
   - unranked at every 2^28 cell;
   - unranked at 2^32 T = 1024, slower at 2^32 T = 4096 and 16384.
2. **T-F2.** Write "canonicalisation plus one hash of x (two counted native operations)
   0.005-0.009 GAE; a step index 0.002-0.006 GAE".
3. **T-F3 and T-F4.** Disclose three things:
   - each cell decision is a 4-of-4 sign rule;
   - 28 b T = 1024 is unranked under the control and 0.98x in the rerun;
   - the pooled rerun reproduces all 9 decisions at 0.77-0.84x and 1.11-1.22x.
4. **T-F5.** Write "the 12 prime-order Weierstrass curves of the v5 study", not "fresh".
5. **T-F6 and T-F7** (prose):
   - correct the four numbers;
   - keep "cache misses" labelled as an untested mechanism.
6. **T-F13 and T-F14:**
   - move runs.jsonl to the artifact store;
   - preserve the smoke data, or record that they are absent.

The accepted wording is `claim-report.json` → `proposed_wording` (sha256 in that file); it is
also `validator-report.json` → `accepted_results_headline`.
