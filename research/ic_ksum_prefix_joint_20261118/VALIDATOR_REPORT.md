# Validator report VAL-20261118-facd3e: `ic_ksum_prefix_joint_20261118` (k-sum `prefix=1` and `targets=stride-renew-joint`, Table B)

This is an independent validation of the producer's evidence on branch `claude/f4-f5-algorithm-speed-bvp7qd`. It covers three commits:

- `1732d93121`: code;
- `5f726868dd`: protocol;
- `35634bd3e9`: results, which is also HEAD when this report was written.

The validator read the study directory from the commit (`git archive 35634bd3e9`), not from the working tree, where the directory is not materialised. It did not edit, repair or regenerate any producer file. The incumbent is the `-psr` arm of `research/ic_ksum_split_20261118` (VAL-20261118-1e81cb).

Every number below is `[pcc:unverified]` until the Coordinator archives this report. The study has no pcc claim.

## Verdicts

| claim | verdict | basis |
|:--|:--|:--|
| **L2, `targets=stride-renew-joint`** (`joint / inc`), 8 Table B rows | **passed on 8/8 rows** (P20, P22, P24, B15, B18, B21, B24, B27) | The registered T2 holds on every row: the pooled and every per-replicate upper end is below 1 at w = 0, 0.1 and 1, and the largest upper end is 0.99869 (B27). The registered T2 correctness check finds 0 mismatches. `joint / inc` at w = 0 is 0.9382 (B15), 0.9505 (B18) and 0.9868–0.9967 on the other rows. Shamir's trick is charged in full (question 1). |
| **L1, `prefix=1`** (`prefix / inc`), per row | **passed** on P20, B15, B18, B21, B24. **Failed** on P22, P24, B27 (registered per-replicate rule). | P20 0.9536, B15 0.9591, B18 0.9368, B21 0.9505, B24 0.9308 at w = 0. On each of these rows the pooled and per-replicate upper ends are below 1 at every weight. P22 (0.9667), P24 (0.9700) and B27 (0.9800) have pooled intervals below 1 at every weight. Each, however, has one replicate whose ecbench bootstrap upper end passes 1: P22 r1 1.00016 (w = 1), P24 r2 1.00019 (w = 1), B27 r1 1.00769 (w = 1) and 1.00002 (w = 0.1). No row is significantly worse. The lever verdict "improvement on the rows that hold" follows the registered rule. The producer's "5/8" is exact. |
| **Gap to `rho-neg`, w = 0: 2.76–5.01 → 2.48–4.99** | **passed** as a recomputation, with one labelling correction (finding F1) | Every row recomputes. The producer's "registered best" is prefix-joint on the five L1 rows and joint on P22, P24 and B27. The protocol instead registers "the cheapest arm by pooled S". That arm is prefix-joint on all 8 rows at all three weights, which gives 2.48–**4.89** at w = 0 (P22 4.18, P24 4.58, B27 4.89). The producer's reading is the more conservative one. |
| Secondary `prefix-joint / inc` (not gated) | recomputes | 0.8873 (B18) to 0.9768 (B27) at w = 0. |
| Validation gate (leaderboard / `benchmarks/README.md`) | **not met** | The raw sessions exist only as a scratch tarball pinned by digest. The repository has no `VALIDATION.json`, replay-certificate file or JSON validator report. The producer claims no row, and none may be added. |

Both levers are constant factors, so no exponent changes:

- **L2** removes a fixed 22–102 GAE per run, against a cost that grows as sqrt(r).
- **L1** enlarges the table by 4.7–13.6 % (`n_prefix / T`) at the same group-operation cost.

`rho-neg` stays 2.48–6.33× cheaper on every row and at every weight. Neither lever is a relabelling or an accounting change: in both, the total counted cost falls because fewer operations are actually executed, as shown below.

## Answers to the specific questions

### 1. L2: is Shamir's trick charged honestly, and are the points identical?

**The trick is fully charged, and the points are identical.**

**Code** (`src/cryptanalysis/ic_boundary.rs` at `1732d93121`):

- `joint_mul` (about line 1840) forms `[a]P + [b]Q` by one MSB-first double-and-add.
  - Every doubling is `g.double` (`ops.doubles += 1`) and every addition is `g.add` (`ops.adds += 1`).
  - The per-bit selection among `{P, Q, P+Q}` is a 2-bit match on the scalars. It involves no table, no lookup, and no stored state beyond the one precomputed point. `g.mul` selects `P` the same way, and the harness charges no lookup for that either.
- `G + Q` is formed once per run as `g.add(&mut rel.group_ops, generator, target)`. It is charged to the relation phase, and only when `targets == StrideRenewJoint`.
- `scalar_mults` is a label. GAE is `adds + doubles` (`GroupOps::gae`), so halving the recorded scalar-multiplication count saves nothing by itself.
- **Draw order.** `draw_one` draws `a = scale_coeff(rng.gen_range(1..r))` and then `b = rng.gen_range(1..r)`. These are the same calls, in the same order, as `draw_jumps(..., 1, ...)` for the stride and the renewal step, and as the inline `walk_fresh` draw for `R_0`. The `rng_draws` bookkeeping is unchanged.

**Records** (validator code, all 24 sessions, warm-up included):

- **Identity.** The validator checked 24,576 pairs: 12,288 `joint/inc` and 12,288 `prefix-joint/prefix`.
  - Identical on every pair, 0 mismatches:
    - outcome, recovered log and algorithm seed;
    - factor base and top-level counters (trials, relations, rank, matrix work);
    - every non-relation phase, in adds, doubles, scalar multiplications, GAE, `native` and `work_native`;
    - the relation phase's whole `native` map (trials, lookups, stride steps, renewals, restarts, guard probes) and its `work_native` map.
  - On every pair, the relation-phase scalar-multiplication count is exactly halved (6 → 3, or 8 → 4 after a restart).
  - The `total_gae` difference equals the add-plus-double difference to within 1e-6.
  - No pair has a negative saving.
- **Size of the saving.** For `a, b` uniform in `[1, r)`, the exact saving of Shamir's trick per pair is:
  - `min(len a, len b) − 1` doublings, plus
  - one addition per bit column where both scalars have a 1;
  - minus one addition per run for `G + Q`.

  The validator compared that expectation (200,000-draw Monte Carlo per curve) with the observed saving on the 1,152 measured `inc/joint` pairs of each row:

  | row | doublings saved / run, observed vs expected | additions saved / run, observed vs expected | GAE saved / run (range) |
  |:--|:--|:--|:--|
  | P20 | 52.08 vs 52.03 | 13.89 vs 14.00 | 66.0 (51–92) |
  | P22 | 56.88 vs 57.01 | 14.47 vs 14.59 | 71.4 (55–90) |
  | P24 | 62.03 vs 62.16 | 15.87 vs 15.75 | 77.9 (54–102) |
  | B15 | 34.12 vs 34.08 | 9.52 vs 9.48 | 43.6 (22–67) |
  | B18 | 37.20 vs 37.16 | 10.28 vs 10.28 | 47.5 (29–74) |
  | B21 | 52.16 vs 52.07 | 13.88 vs 14.00 | 66.0 (47–96) |
  | B24 | 55.10 vs 55.05 | 14.68 vs 14.77 | 69.8 (51–95) |
  | B27 | 65.16 vs 65.25 | 16.61 vs 16.50 | 81.8 (62–101) |

  - Every residual is within 1.6 SE. Per row, the SE is about 0.083 for doublings and 0.08–0.11 for additions, from the per-pair variance and 1,152 runs; the residual z is −1.59 to +1.08 for doublings and −1.22 to +1.14 for additions.
  - An uncharged `G + Q` would shift every addition row by +1.00, a deviation of 8.4–11.7 SE, so it is visibly charged.
  - An uncounted doubling or addition inside `joint_mul` would show the same way.

### 2. L1: are the prefix sums already formed, consistently charged, verifying, and safe? Is the gain real?

**Already formed, with no added group operation.**

- `note_prefix` is called only on values the build computes anyway:
  - the depth-0 base point (no operation);
  - each non-last partial sum `s` in `extend`;
  - each pair sum computed in `pair_sums`.
- It is never called in `extend_split` or at the last level. It forms nothing itself: it only packs indices and computes `g.key(s) >> 1`, as `insert_last` does for every k-sum.
- **Closed-form check.** With prune and split, the noted count should be:

  `(a − k + 1) + Σ_{d=1}^{k−3} 2^d C(a − k + d + 1, d + 1) + 2 C(a − k + 2, 2)`

  - That is 205 for k = 5, a = 9; 314 for a = 10; 455 for a = 11; 632 for a = 12; and 44 for k = 4, a = 7.
  - The records show exactly this per-row constant as the `table_insert_attempts` excess of `prefix` over `inc`, on every one of 24,576 pairs. The counts are P20/B21/B24 205, P22 314, P24 455, B27 632, B15/B18 44.
- **Duplicates.** The kept prefix entries (`ksum_prefix_entries`) are the count above minus exact duplicates. The depth-1 partials and the pair sums share abscissa pairs `k−2 ≤ i < j ≤ a−k+1`, which accounts for 6, 12, 20 and 30 duplicates. The kept counts are 199, 302, 435, 602 and 38. B15 has 37: one further key was already held by an equal point, either a 4-sum or another prefix sum, and its insert attempt was still charged.
- **Set-up work.** `factor_base` and `oracle_setup` adds, doubles, scalar multiplications, GAE and `native` are identical on all 24,576 `prefix/inc` and `prefix-joint/joint` pairs. All set-up `work_native` other than `table_insert_attempts` is also identical.

**Consistent charging.**

- Each pending prefix sum costs one `insert_attempts += 1` whether it is kept or dropped. This is the same rule as `insert_last` for k-sums.
- The band prices `table_insert_attempts` at w on every arm where the `counted:table_insert_attempts` marker is present (`band.rs`, `TABLE_WORK_COUNTERS`), and the marker is present on all IC arms here. The prefix inserts are therefore charged at w = 0.1 and 1, which is why `prefix / inc` rises with w (P20 0.9536 → 0.9598).
- Key computation is uncharged for k-sums and prefix sums alike.
- **Probe.** The probe is unchanged: one counted lookup per trial, then `len − 1` counted additions to confirm. A prefix hit costs `j − 1 < k − 1`.

**Fewer-summand relations verify, and the solver needs no extra work.**

- `unpack` stops at the first sentinel slot, and `probe` returns the summands only if their sum equals `±target`.
- `Relation::Summands` of any length becomes a dense row (`collect_and_solve_relations_scaled_touching`, about line 14837). Its mod-r adds are counted per summand, so a short row costs less, and nothing new is uncounted.
- Every one of the 73,728 records is `verified` with `matches_target: true` and `exit 0`. The validator independently checked `[d]G = Q` (below).
- **Summand deficit.** The mean summands per relation on the prefix arm is 4.79–4.90 for k = 5 and 3.75 for k = 4. This matches the share of prefix entries in the table: for P20 the predicted mean deficit is 0.209 and the observed is 0.206.

**The padding sentinel is safe.**

- The sentinel `2^bits − 1` lives in the packed value, never in the key. The build refuses `fb.points.len() >= 2^bits`, and `bits = 12` for k ≤ 5. The bases here have 14–24 points.
- Keys are the exact, negation-invariant abscissa `x + 1`, on prime curves (`PrimeCurve::key`) and on binary curves (`FastPoint::pack`). Two keys match only if the points are equal up to sign, so there are no false key collisions.
- A prefix sum whose key a k-sum already holds is the same point up to sign and adds no coverage. Such sums are dropped and their insert attempt is still charged.

**The gain is a real hit-rate gain, not accounting.**

Pooled over the measured runs, the observed hit rate (relations per trial) rises by exactly the factor by which the table grows, `(T + n_prefix) / T`:

| row | T (k-sum inserts) | n_prefix | predicted hit-rate ratio | observed | trials `prefix / inc` |
|:--|--:|--:|--:|--:|--:|
| P20 | 2016 | 199 | 1.0987 | 1.0953 | 0.9106 |
| P22 | 4032 | 302 | 1.0749 | 1.0695 | 0.9332 |
| P24 | 7392 | 435 | 1.0588 | 1.0595 | 0.9424 |
| B15 | 280 | 37 | 1.1321 | 1.1286 | 0.8795 |
| B18 | 280 | 38 | 1.1357 | 1.1395 | 0.8700 |
| B21 | 2016 | 199 | 1.0987 | 1.1038 | 0.9035 |
| B24 | 2016 | 199 | 1.0987 | 1.1130 | 0.8979 |
| B27 | 12672 | 602 | 1.0475 | 1.0441 | 0.9560 |

- Relations per run are nearly unchanged (P20 9.884 → 9.859), so the trial count falls by the coverage factor.
- `S` then falls by about the relation phase's share of `S` (46–70 %) times that fall. For example, on B24 0.704 × 0.102 predicts 0.928 against 0.931 measured, and on P20 0.557 × 0.089 predicts 0.950 against 0.954.
- **Null and decay control.** The trial reduction follows `n_prefix / T` on every row, so the quantity behaves as a coverage effect should as its source shrinks:
  - the largest reduction is on B15 and B18 (`n_prefix / T` 13.2 % and 13.6 %, trials × 0.88 and 0.87);
  - the smallest is on B27 (4.7 %, trials × 0.956), which also has the weakest `S` effect (0.980).
  - The `S` effect is not a monotone function of `n_prefix / T` alone, because the relation phase's share of `S` also differs by row (B24 70 %, B27 46 %).
- The protocol's "off" control is the incumbent itself. The unit test `prefix_off_is_identical_and_on_adds_no_setup_operation` checks that `prefix=0` is bit-identical to the build without it.

### 3. Was the protocol frozen before the confirmatory data?

**Yes, with a 7 s margin and two caveats.**

The timeline, from committer timestamps, the session files' `started_unix_ms` and the producer's bench records:

| time (UTC) | event |
|:--|:--|
| 10:31:56 | code commit `ea46319d35` |
| 10:59:11 | binary built |
| 11:04:39 | pilot started (`benchmarks/records/20261118T1059Z-…-pilot.json`, commit `ea46319d35`, clean) |
| **11:06:42** | registration commit `bc3122e8dc` |
| 11:06:48 | confirmatory bench record created (commit `bc3122e8dc`, clean) |
| **11:06:49.6** | first confirmatory session (P20-r1) |
| 11:16:33 | last confirmatory session started (B27-r3), finished 11:17:06 |
| 11:20:49 | verify runs end |
| 11:42:14–16 | cherry-picks onto the lane branch |

Registration and code identity:

- `REGISTRATION.sha256` is byte-identical in `bc3122e8dc` and `5f726868dd`. 30/30 registered files (PROTOCOL, `make_specs.py`, `run_all.sh`, `evaluate.py`, `correctness.py`, BINARY.sha256 and the 24 specs) hash correctly at `35634bd3e9`.
- The study directory of `5f726868dd` equals that of `bc3122e8dc`.
- The code patch of `1732d93121` equals that of `ea46319d35` (same `git patch-id`, `ca05cd93…`). The four touched source files have identical blobs.
- The two trees differ only in a test-only `mod.rs` borrow fix and `tests/ml_dataset_live_capture.rs`, both from the lane.

The 96 → 128 targets change is part of the registered text and specs. All 24 confirmatory sessions use 128 targets, 3 rounds and 1 warm-up. No other confirmatory session exists in the tarball, and `kp_confirm.log` was created at 11:06:48. The validator found no evidence of a 96-target draft, nor of an earlier confirmatory start; the "drafted 96" rests on the producer's statement alone.

The pilot used separate seeds:

- pilot: `target_seed` 20261118400–407 and `seed` 20261118450–457;
- confirmatory: `20261118800 + idx` and `20261118900 + idx`, with idx = 10 × row + rep, verified on all 24 specs.

Caveats (finding F4):

- Commit timestamps are self-reported.
- `bc3122e8dc` and `ea46319d35` are reachable from no ref, because the side branch `ksum-prefix-wip` was removed. They survive only as loose objects and as citations in the bench records, so a `git gc` could delete the pre-registration evidence.

### 4. Recomputation

| check | result |
|:--|:--|
| Raw tarball | sha256 `5cb867b808e0df1734f61f095f8d3747d977de523272e6279c8cd09e33ff10dd` matches |
| Per-file digests | 48/48 of `data/sessions.sha256` match |
| Specs | 24/24 session `spec.json` byte-identical to the committed specs |
| Binary | `32507dc9c5777b8b50196575000562f1478d0eb77de9738abe65d77feb55e3b6` in every record and every `session.json`; `strings` shows `stride-renew-joint`, `stride_renew_joint`, `ksum_prefix_entries` and the `prefix` parameter text |
| Records | 73,728 (24 × 3,072), all `verified`, `matches_target`, `exit 0`; 0 unpaired, 0 censored |
| `ecbench compare` | 384 comparisons, `--weight 0/0.1/1 --resamples 4000 --seed 20261118`, with the producer's binary and the validator's own driver: lever pooled and per replicate, A/A, gap for all four IC arms, and the three secondary pairs. **All 672 numbers in the producer's `results.json` (ratios, linearised and bootstrap bounds) reproduce exactly (maximum difference 0).** All comparisons are admissible. |
| Own pricing | Priced cost recomputed from raw records with validator code: Σ(adds + doubles) + w·(calibrated native ops + `cost.unpriced` + counted `table_insert_attempts`), and for rho `total_gae + w·canonicalisations`. It agrees with every ecbench point ratio to 1.1e-14. |
| Own intervals | Linearised intervals clustered by workload agree with ecbench's. Pooled bootstraps agree. The ecbench single-session cluster bootstrap is wider than the validator's own: P22 r1 w = 1 upper is 1.00016 against 0.9949. The registered "max of linearised and bootstrap" rule is therefore the conservative one, and the three L1 failures fall inside this method-dependent margin. The registered tool governs. |
| Replay | `ecbench verify --replay-all --exit-code` on one session per row, drawn by nonce `515457dff58fcf5b` (P20-r1, P22-r1, P24-r2, B15-r3, B18-r3, B21-r2, B24-r2, B27-r3): 8/8 rc 0, 18,432 replays reproduced, 0 problems. The producer's 24 verify receipts in the tarball all say "audit OK", 0 problems. |
| Independent EC check | Validator-written prime and GF(2^n) arithmetic on 2,000 records drawn by nonce `b0fed01bb212d4ff-20261118T115446Z`, all six arms: 2,000/2,000 have `[d]G = Q`, with `Q` on the curve and of order r. G has order r on all 24 session curves. |
| A/A | `rho-neg-aa / rho-neg` = 1 exactly at every weight on every row. This is a deterministic pairing control (same seed, counted cost) and carries no information about noise. |
| Incumbent positive control | The `inc` arm reproduces the split study's `ksum<k>-psr` S on fresh targets within 2.2 % at w = 0 (P20 4.948 vs 4.926; B21 4.902 vs 5.014, the largest difference). Gap to rho-neg: 2.76–5.01 here against 2.78–4.97 there. |

Headline ratios (`ecbench compare`, pooled, 1,152 pairs a row):

| row (k, N) | L1 `prefix/inc` w = 0 / 0.1 / 1 | L1 max upper (where) | L2 `joint/inc` w = 0 / 1 | L2 max upper | both w = 0 | gap rho-neg w = 0: inc → joint / prefix-joint |
|:--|:--|:--|:--|:--|:--|:--|
| P20 (5, 9) | 0.9536 / 0.9549 / 0.9598 | 0.98237 (r3, w1) **holds** | 0.9870 / 0.9948 | 0.99495 **holds** | 0.9405 | 3.882 → 3.831 / 3.651 |
| P22 (5, 10) | 0.9667 / 0.9682 / 0.9733 | 1.00016 (r1, w1) **fails** | 0.9922 / 0.9968 | 0.99695 **holds** | 0.9589 | 4.362 → 4.327 / 4.182 |
| P24 (5, 11) | 0.9700 / 0.9710 / 0.9746 | 1.00019 (r2, w1) **fails** | 0.9954 / 0.9982 | 0.99822 **holds** | 0.9654 | 4.744 → 4.722 / 4.580 |
| B15 (4, 7) | 0.9591 / 0.9610 / 0.9689 | 0.99914 (r3, w1) **holds** | 0.9382 / 0.9725 | 0.97376 **holds** | 0.8974 | 2.763 → 2.592 / 2.480 |
| B18 (4, 7) | 0.9368 / 0.9369 / 0.9370 | 0.96934 (r3, w1) **holds** | 0.9505 / 0.9793 | 0.98008 **holds** | 0.8873 | 2.861 → 2.719 / 2.538 |
| B21 (5, 9) | 0.9505 / 0.9516 / 0.9557 | 0.97730 (r2, w1) **holds** | 0.9868 / 0.9947 | 0.99490 **holds** | 0.9373 | 3.872 → 3.821 / 3.629 |
| B24 (5, 9) | 0.9308 / 0.9309 / 0.9314 | 0.96480 (r2, w1) **holds** | 0.9909 / 0.9966 | 0.99670 **holds** | 0.9217 | 4.462 → 4.421 / 4.113 |
| B27 (5, 12) | 0.9800 / 0.9812 / 0.9856 | 1.00769 (r1, w1) **fails** | 0.9967 / 0.9987 | 0.99869 **holds** | 0.9768 | 5.009 → 4.993 / 4.893 |

At w = 1 the gaps are inc 3.62–6.33, joint 3.52–6.33 and prefix-joint 3.41–6.24. Holm across the 8 rows on the pooled linearised intervals (two-sided 0.05) passes for every row of both levers at every weight. The smallest z is 4.64 (L1, B27, w = 1), against the producer's "4.6".

### 5. Was the registered decision rule applied correctly?

**Yes.** The validator applied the rule in its own script, `rule.py`, to its own ecbench outputs:

- **T1:** for each row, the upper end is `max(linearised, bootstrap)`, taken over pooled plus three replicates at w = 0, 0.1 and 1.
- A row holds when the upper end is `< 1` and `1 − max upper > max_w |R_aa − 1|`. Here the right-hand side is 0, because `R_aa` = 1 exactly.
- A row is "significantly worse" when the pooled lower end is `> 1` at any weight.

Results:

- **L1** holds on {P20, B15, B18, B21, B24}, the same rows as the producer's `results.json`.
- **L2** holds on all eight rows.
- No row of either lever is significantly worse.
- All three L1 failures are per replicate. The pooled upper ends at w = 1 are below 1: 0.98355 (P22), 0.98448 (P24) and 0.99352 (B27).
  - P22 fails only on r1 at w = 1 (1.00016) and P24 only on r2 at w = 1 (1.00019).
  - **B27 r1 fails at w = 0.1 as well (1.00002) as at w = 1 (1.00769).** The README and the dispatch note mention only w = 1 for B27. The verdict is unchanged.
  - At w = 0 every per-replicate upper end on these rows is below 1; the largest are 0.98870, 0.99126 and 0.99791.
- B15 holds with the smallest margin: r3 at w = 1 has upper end 0.99914.

The registered T2 correctness check (outcome, log, trials and relations of `joint` against `inc` for every session, target and round) passes with 0 mismatches. The validator's check above is a superset of it.

### 6. Unit tests

**17 passed, 0 failed.** The validator ran `cargo test --release --locked --lib ksum` on a validator-made export of `1732d93121`, with `CARGO_BUILD_JOBS=1`, its own `CARGO_TARGET_DIR` in the scratchpad, and the scratchpad CPU lock (`flock`). The build took 63 min 27 s; the target directory was deleted afterwards.

- **The export** (`git archive`) contains `Cargo.toml`, `Cargo.lock`, `src`, `crates`, `benches`, `examples` and `tests`, plus three further sets of files:
  - the four path-dependency crates (`tools/factor-base`, `tools/volcano-cli`, `isogeny_algos`, `research/pencil_large_prime_20261029`) and `research/five_leads_e2e_20261019/native`;
  - the 90 out-of-tree `include_str!`, `include_bytes!` and `#[path]` inputs and manifest target paths. The validator resolved these itself from the source at that commit.
- **The sparse-checkout caveat did not apply**, because the export carries every input the lib tests need. The `src/` of this export is the tree the measured binary was built from: the patch-id is equal, and only test code differs.
- **The three new tests passed:**
  - `prefix_build_keeps_the_ksum_table_and_adds_shorter_sums` (ic_boundary.rs): every k-sum entry is kept, with the same build operations and collisions, and every prefix entry is a correct decomposition found by a probe of `±` its sum. It covers k = 3–6 prime and 3–5 binary, split on and off.
  - `prefix_off_is_identical_and_on_adds_no_setup_operation` (plugins.rs).
  - `stride_renew_joint_is_stride_renew_with_a_cheaper_setup` (plugins.rs).
- **Earlier k-sum tests still pass:** split, prune, sib, wide index, known-log and Frobenius.

## Further findings (not verdict-changing)

- **F1, gap label.** The protocol registers the gap for "the cheapest arm by pooled S". That arm is prefix-joint on all 8 rows at every weight, giving 2.48–4.89 at w = 0, 2.63–5.13 at w = 0.1 and 3.41–6.24 at w = 1. The README's "registered best" (joint on P22, P24, B27) gives 2.48–4.99, 2.63–5.23 and 3.41–6.33. That choice is defensible, since it does not credit L1 where L1 failed its test, but it is not what was registered. It is a reporting inconsistency in a secondary, not a gated claim.
- **F2, additivity.** The README says L1 given L2 and L2 given L1 equal the marginal ratios "to within 0.003". On B18 at w = 0 the differences are 0.0032 (L1|L2 0.9336 vs 0.9368) and 0.0033 (L2|L1 0.9472 vs 0.9505). This is rounding-level. The direction is what additivity in GAE predicts: a fixed saving is a larger fraction of a smaller S.
- **F3, bitop/v1 reading of L1.** The new gauge counter `ksum_prefix_entries` sits in the relation phase's `native` map. The `bitop/v1` work pricer lists it as unpriced and marks all 24,576 `prefix` and `prefix-joint` work blocks `complete: false`; `inc`, `joint` and rho are `complete: true`. The GAE band is unaffected, because the counter is neither in `cost.unpriced` nor in `PRICED_NATIVE`, and the inserts it describes are already charged through `table_insert_attempts`. But L1 cannot be read in `bitop/v1` until the counter is registered as a non-operation gauge. The study does not claim bitop.
- **F4, registration evidence.** The registration commit `bc3122e8dc` and the built commit `ea46319d35` are unreachable loose objects. Push a ref or tag so the pre-registration survives `git gc`.
- **F5, raw data.** The raw data is not durable. `kp-confirm-sessions.tar.gz` (40.7 MB; 625 MB unpacked) exists only in the producer's scratchpad. The bench record lists an S3 URI that could not be checked from this container. The pilot raw data was deleted after summarising, so the pilot is not re-verifiable. It is not evidence, and the registration does not depend on it.
- **F6, memory.** Memory is not reported. The prefix tables hold 37–602 extra entries (4.7–13.6 % of T), which is small at these sizes. The band prices operations, not bytes (§5a).

## Gate status

Not met, and the producer claims no row. The gate needs three things:

- the raw records in the repository or the artifact store;
- a `VALIDATION.json` pinning a replay certificate and a JSON validator report with `headline` entries;
- the matched rho reference block.

That block would carry `rho-neg`, negation, which is the strongest symmetry available on these generic prime and binary curves (`automorphisms_available: 2` in every record). It would come from the same binary, in ecbench GAE, with the band at 0, 0.1 and 1. On every row the IC arm stays above rho at all three weights, so `order_holds_across_band` would be true with rho ahead.

## Limitations

- **Binary provenance.** The binary was not rebuilt, so the binary-to-source link rests on `BINARY.sha256`, the `strings` check and behaviour that matches the source on every check here. Replay uses the producer's binary, so it shows determinism, not independent implementation. The independent checks are the EC verification, the Monte Carlo of the Shamir saving, the closed-form prefix counts and the validator's own pricing.
- **Scope of the cost claim.** Only L0 counted cost is measured, on 8 curves from 2^14 to 2^24.4. Wall time and scaling are not claimed. Both levers are constant factors that shrink against sqrt(r).
- **A/A.** The A/A control is deterministic and therefore says nothing about noise.
- **Validator scripts.** The scripts are in the validator's scratchpad, not committed (task scope: report only): `val_kp/scripts/{extract,analyse,correct,mech,ecverify,cluster_ci,rule,incl}.py`, `run_compares.sh` and `build_tests.sh`.
- **Raw data deleted.** The extracted raw data was deleted after the analysis (disk limit). The tarball itself was not touched.

```yaml
validation_report:
  id: VAL-20261118-facd3e
  task_id: caller-dispatched (no TASK id given)
  study: research/ic_ksum_prefix_joint_20261118
  validated_commits: {code: 1732d93121, protocol: 5f726868dd, results: 35634bd3e9 (HEAD)}
  registration_commit: bc3122e8dc (11:06:42Z; unreachable from refs)
  run_ids: ["24 ecbench sessions {P20,P22,P24,B15,B18,B21,B24,B27} x r1..r3; raw kp-confirm-sessions.tar.gz sha256 5cb867b8...10dd"]
  artifact_checks:
    - registration_before_data: pass (bc3122e8dc 11:06:42Z; first session 11:06:49.6Z; self-reported timestamps)
    - registered_hashes: pass (30/30; REGISTRATION.sha256 identical in bc3122e8dc and 5f726868dd)
    - code_identity: pass (patch-id equal for ea46319d35 and 1732d93121; four source blobs identical)
    - session_digests: pass (tarball + 48/48 per-file)
    - specs_equal_committed: pass (24/24); seeds fresh and as registered; pilot seeds disjoint
    - binary_sha256: pass (32507dc9...e3b6 in all 73,728 records)
    - coverage: pass (73,728 records verified, exit 0, 0 unpaired, 0 censored)
    - replay: pass (8 nonce-drawn sessions, 18,432 replays reproduced, 0 problems; nonce 515457dff58fcf5b)
    - raw_data_durable: FAIL (scratch tarball only)
  metric_recomputations:
    - "ecbench compare 384 comparisons (w 0/0.1/1, 4000 resamples, seed 20261118): 672/672 producer numbers identical"
    - "own pricing from raw records: agrees with ecbench point ratios to 1.1e-14"
    - "T1 (L1): holds P20, B15, B18, B21, B24; fails P22, P24, B27 (per-replicate: P22 r1 and P24 r2 at w = 1; B27 r1 at w = 0.1 and 1); none significantly worse"
    - "T2 (L2): holds 8/8; max upper 0.99869"
    - "Holm (8 rows, pooled): passes every row, both levers, every weight; smallest z 4.64"
    - "gap to rho-neg: producer's reading recomputes (2.76-5.01 -> 2.48-4.99 at w = 0); registered cheapest-arm reading 2.48-4.89 (F1)"
  control_checks:
    - aa: exactly 1 (deterministic; uninformative about noise)
    - L2_identity: 24,576/24,576 pairs identical except relation-phase adds/doubles; scalar mults exactly halved; GAE diff = group-op diff
    - L2_charging: Monte Carlo of exact Shamir saving matches observed per row within 1.6 SE; an uncharged G+Q would be 8.4-11.7 SE off
    - L1_no_added_op: 24,576/24,576 pairs with identical set-up group ops; extra insert attempts equal the closed form on every row
    - L1_mechanism: observed hit-rate ratio equals table growth (T+n)/T on all 8 rows; effect decays with n/T
    - answers_independent_ec: 2,000/2,000 [d]G = Q; G of order r on 24/24 curves
    - incumbent_positive_control: inc reproduces split-study -psr S within 2.2 %
    - unit_tests: cargo test --release --locked --lib ksum on a validator export of 1732d93121, 17 passed, 0 failed (all three new tests included)
  heuristic_validation_checks:
    - "predictions registered before data: pass. T2 within +/-0.01 of pilot on 8/8 (met). T1 R_0 in 0.91-0.97 on the six rows and 0.96-1.00 on P24/B27 (met); \"holds on most\" 5/6 (met)."
  cost_model_checks:
    - unit: ecbench.gae per sqrt(r), cold, every phase; band 0/0.1/1 priced identically on every arm, rho canonicalisations and IC table inserts included
    - L2: real reduction of executed doublings and additions, every remaining operation counted; scalar_mults is a label only
    - L1: no added group operation; prefix inserts charged as table_insert_attempts at w; probe confirmations counted (j-1 adds)
    - memory not reported (+37..602 table entries); bitop/v1 incomplete for prefix arms (F3)
  proof_architecture_checks: []
  verdict: passed
  claim_verdicts:
    L2_stride_renew_joint: passed on 8/8 rows (P20, P22, P24, B15, B18, B21, B24, B27)
    L1_prefix: "passed on P20, B15, B18, B21, B24; failed on P22, P24, B27 (registered per-replicate rule: P22 r1 and P24 r2 at w = 1, B27 r1 at w = 0.1 and 1)"
    gap_to_rho_neg: passed as recomputed; label "registered best" differs from the registered cheapest-arm definition (F1)
    gate: not met (raw data not durable; no VALIDATION.json / replay certificate / JSON validator report)
  limitations: [binary not rebuilt, same-binary replay, L0 counted cost only, constant-factor gains shrinking with r, A/A deterministic, registration commit unreachable from refs, validator scripts not committed, raw data scratch-only]
  artifact_paths:
    - research/ic_ksum_prefix_joint_20261118/VALIDATOR_REPORT.md
review_attestation:
  joints_owned: [whole-study validation as dispatched; specific questions 1-6]
  sources_read:
    - research/ic_ksum_prefix_joint_20261118/{PROTOCOL.md, README.md, results.md, REGISTRATION.sha256, BINARY.sha256, registered-at.txt, run_all.sh, make_specs.py (grep), specs/*, pilot/specs/*, pilot/{runlog.tsv, verify.txt, pilot_table.md, sessions.sha256 (head)}, analysis/evaluate.py, results.json.gz, data/{sessions.sha256, verify.txt, verify_summary.txt, correctness.txt, evaluate.txt (tail), runlog.tsv (head)}} via git archive of 35634bd3e9
    - "raw sessions (validator scratch extract of kp-confirm-sessions.tar.gz): records.jsonl, session.json, spec.json, host.json (head), *.verify.err x 24"
    - git objects bc3122e8dc, ea46319d35, 1732d93121, 5f726868dd (logs, diffs, patch-ids, cat-file)
    - src/cryptanalysis/ic_boundary.rs at 1732d93121 (GroupOps, CountedGroup::mul, PrimeCurve/BinaryGroup add/double/key, joint_mul, draw_jumps, scale_coeff, KSumTable build/pair_sums/extend/extend_split/insert_last/note_prefix/unpack/probe, Relation, row construction, StrideRenewJoint target loop)
    - src/cryptanalysis/ic_framework/plugins.rs (KSumOracle prepare/decompose/relation_counters, new tests), stages.rs and mod.rs diffs; src/cryptanalysis/ecbench/band.rs (pricing), methods.rs PRICED_NATIVE; koblitz_fast.rs FastPoint::pack
    - benchmarks/records/20261118T1059Z-ksum-prefix-joint-pilot.json and 20261118T1106Z-ksum-prefix-joint-tableb.json (top-level keys)
    - research/ic_ksum_split_20261118/{VALIDATOR_REPORT.md (prior study, for context and format), specs/*-r1.json, summary.json}
    - docs/VALIDATION_GATE.md (head); AGENTS.md (success criterion, section 5a, validation gate); crypto-autoresearcher docs/agent-runtime-core.md, agents/validator.md
    - producer scratch logs kp_bin.log, kp_test.log (listing only; not relied on)
  read_sibling_reports: false
  verdict: passed (receipt admissible; per-claim verdicts above)
```
