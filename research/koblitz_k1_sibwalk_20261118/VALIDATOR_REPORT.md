# Validator report VAL-20261118-99f8e0: sibling giant-step walk + reuse of formed points on the K1 IC arms (n23, n29, n37)

This report independently validates `research/koblitz_k1_sibwalk_20261118/` on branch
`claude/f4-f5-algorithm-speed-bvp7qd`. The committed snapshot is `e7953e1a51` (results). It rests on
`f164bf67fc` (registration), `d321041bde` (pilot specs and selection rule, before pilot data) and
`217638524d` (code). The validator did not design, write or run any part of the study, and did not edit
any producer file. While it worked, another session committed `624775314b` and `e97ce22df9`
(`koblitz_n37_sibwalk_k8_20261118`). Those commits touch neither this study's directory nor `src/`,
`crates/`, `Cargo.toml` or `Cargo.lock`. The validator read only their commit messages and diff
file names.

Validator nonce: `dbccd01fcef3e99c138508570d579943`, drawn 2026-11-18T19:27:50Z. That is after the
results commit (19:25:42Z) and after its push (19:26:19Z).

## Verdicts

"Passed" means the receipt is admissible evidence for the claim as stated, on that curve only. The
claim is a T0-scope engineering gain of the validated incumbent `ls` (VAL-20261118-9a42f2). It is not
an ECDLP claim and not a claim below rho. It changes no exponent. It does not authorize a leaderboard
or `benchmarks/README.md` row by itself (section 9). "Failed" means the receipt is admissible and the
candidate does not meet the frozen rule on that curve, as the producer reports. In both cases the
numbers are confirmed.

| curve | claim: sw / ls pooled, w = 0 / 0.1 / 1 | recomputed (95%, wider of linearised-t and cluster bootstrap) | frozen rule: upper end < 1 at every weight, pooled and in each replicate | verdict |
|:--|:--|:--|:--|:--|
| **K1 a=1 n23** (k=4, m=8, ladder, key reuse) | 0.703 [0.686, 0.721] / 0.787 / 0.966 | 0.70295 [0.68586, 0.72140] / 0.78681 [0.77426, 0.80015] / 0.96580 [0.96326, 0.96824] | 12 of 12 upper ends < 1. The largest is 0.9707 (r2, w = 1). Replicate upper ends at w = 0 / 0.1 / 1: r1 0.753 / 0.822 / 0.969; r2 0.730 / 0.806 / 0.971; r3 0.731 / 0.807 / 0.971. | **passed** (the w = 1 reading is in section 4) |
| **K1 a=1 n29** (k=1, m=1) | 0.873 / 0.915 / 1.013; fails at w = 1 | 0.87261 / 0.91519 / 1.01272 [1.01126, 1.01421]. At w = 1 every replicate interval lies above 1: r1 [1.0109, 1.0168], r2 [1.0105, 1.0159], r3 [1.0092, 1.0135]. | 9 of 12 cells pass; all three w = 1 cells fail | **failed**, as claimed and as the frozen expectation (~1.015) predicted |
| **K0 a=0 n37** (k=9, m=8, from the pilot) | 0.785 / 0.874 / 1.050; fails at w = 1 | 0.78517 / 0.87392 / 1.05020 [1.00599, 1.09516]. At w = 1, r1 [1.0025, 1.1123] and r3 [1.0164, 1.1494] lie above 1, and r2 [0.9201, 1.1180] straddles it. **At w = 0.1, r3's upper end is 1.0009.** | 8 of 12 cells pass | **failed**, as claimed. It also fails at w = 0.1 in r3; the README table shows this (r3 hi 1.001), but its verdict column names only w = 1. |

| gap statement (pooled, w = 0 / 0.1 / 1) | claim | recomputed | verdict |
|:--|:--|:--|:--|
| n23 sw / rho-v5 | 0.927 [0.847, 1.020] / 1.237 [1.131, 1.361] / 2.577 [2.355, 2.842]; unresolved | identical. Band rank (ecbench): `unresolved inside the band`. | passed: **not below rho** |
| n23 ls / rho-v5 (before) | 1.318 [1.179, 1.478] / 1.572 / 2.668 | identical; `rho-v5 < ls` | passed |
| n23 sw / rho-strong | 0.236 / 0.325 / 0.788; below at every weight | identical; `sw < rho-strong` | passed. rho-strong is not the strongest measured reference. |
| n29 and n37 sw and ls / rho-v5 and rho-strong | README table | every point and interval identical | passed, with one discrepancy. In this study's own sessions, n29 `ls` / rho-v5 is 1.011 / 1.204 / 2.136, which is above 1 at every weight, and ecbench's band rank is `rho-v5 < ls`. The README's band-order column says "unresolved". The w = 0 interval [0.930, 1.100] includes 1, which may explain the wording, but §5a orders arms by the band, and the band says v5 is ahead. |

**Overall: the receipt is admissible on all three curves. n23 passed, n29 failed and n37 failed.**
Three readings matter most:

- **At n23 the w = 1 pass rests entirely on pricing 88 field squarings at 1 GAE each** (section 4).
  - At w = 1 the walk costs +14.7 GAE per solve and the ladder saves 15.0; key reuse saves 88.0.
  - With the field-arithmetic class unpriced, sw / ls at w = 1 is 0.9999 [0.9997, 1.0001].
  - AGENTS.md §5a requires a calibrated reading for field-arithmetic counters. The study does not
    report one, and no binary-field `w_cal` exists in the repository.
  - The band verdict governs, and it passes. But the robust part of the n23 gain is the walk at
    w ≤ 0.1 and the ladder at every weight.
- **The sibling walk and `ladder_verify` are sound, fully counted and real reductions** (sections 1
  and 2).
  - Same targets, in the same order. A hit on an abscissa only triggers forming the full point,
    which is then decomposed and counted as usual. Every lift is charged.
  - The ladder reads points that the same record's base phase formed and charged.
  - Both phases are faster in wall time on nearly every pair.
  - `ladder_verify` is not a relabel in the sense VAL-20261118-43658a found for `tau_verify`.
- **The count model reproduces the measured n23 and n29 ratios to 1e-16**, as claimed. For
  deterministic counts on an unchanged walk this is expected. The n37 k = 8 figure
  (0.736 / 0.817 / 0.977) is arithmetically right, but it is derived after the data and is not
  evidence (section 8).

## 1. Sibling giant-step walk (`walk_sib=m`, `TargetSource::StrideGeneratorSib`)

The code read is the diff of `217638524d` (`ic_boundary.rs`, `stages.rs`, `plugins.rs`, `mod.rs`) and
the relation loop around it.

- **Same targets, same order.**
  - `sw_next` steps k = 0, 1, 2, … and the target tried at step k is `Q + [k]G`:
    - k = 0 is Q itself;
    - for k < kc, it is `C − [i]G` with i = kc − k;
    - k = kc is C;
    - for k > kc, it is the stored `C + [k − kc]G`.
  - Blocks are centred at kc = m + 1 + b(2m + 1). The next centre is formed when
    k = kc + m + 1.
  - The source is neither guarded nor stride-renewing, the same as `StrideGenerator`, so no guard
    probe is skipped.
  - It stops at k = r, as `StrideGenerator` does. An identity target takes the same
    `direct_skipped` path.
  - On data, for n23 and n29 (k unchanged), sw and ls agree on 576 of 576 (replicate, round,
    target) keys, warm-ups included. They have the same `targets_tried`, `trials`, relation count,
    recovered `d`, `algorithm_seed`, linear-algebra phase (wall excluded), oracle set-up phase and
    `points_sha256`.
- **Soundness of the x-only probe.**
  - `probe_x` is the first half of the layout table's full probe: one canonicalisation and one
    lookup, both counted. Its key depends only on the abscissa. Under negation and the Frobenius
    map the abscissa orbit names the same key, so a miss on x is a miss of the full probe. The unit
    test asserts `hit || !full` on 3,000 walk points per curve.
  - A hit produces no relation by itself. The point `C − [i]G` is formed by one counted addition
    and goes through the oracle's ordinary `decompose_relations`, with its own counted
    canonicalisation and lookup and coefficients (a, b) = (k, 1).
  - A false x-hit can therefore cost work but cannot produce a wrong relation. The recovery check
    and the validator's independent `[d]G = Q` (section 6) close the loop.
  - The validator checked the formula itself with its own GF(2^n) arithmetic:
    λ₋ = λ₊ + x_J/(x_P + x_J), x = λ² + λ + x_P + x_J + a. On 24,000 pairs per curve (walk points
    from the first target, J = [1..8]G), the full point `P + J` and the abscissa of `P − J` equal
    two full additions, with 0 mismatches.
- **Every operation is charged.** The validator derived the walk's counts from the code
  independently and checked them on all 1,296 sw records and all 5,184 ls, ls-aa and ls-off records,
  with 0 mismatches:
  - ls relations group operations = (K − 1) + hits;
  - sw relations operations = (m + 1 set-up) + centres + pairs + lifts + full pairs + hits, with
    exactly one doubling;
  - `walk_sibling_pairs` = `sibling_second_points` = pairs (minus full pairs);
    `walk_x_only_probes` = pairs;
  - canonicalisations = lookups = K + lifts. This shows the lifted hit's second probe is always
    counted.
  - `walk_sibling_full_pairs` = 0 everywhere. Every solve has exactly one relation, and a lift
    occurs exactly when the final hit falls on a minus-side target (1,296 of 1,296).
- **Pricing.**
  - `sibling_second_points` sits in the relations phase's `work_native` behind
    `counted:sibling_second_points`. `band::table_work_counts` sums it over all phases and adds it
    to U, priced at w.
  - The validator's independent band recompute includes it and reproduces ecbench to 1e-14
    (section 5).
  - At w = 1 the band charges a full group addition for a cost of about one multiplication and
    one squaring. At w = 0 it is free. Both readings are the band's documented convention.
- **What the walk costs and saves** (per solve, against a plain walk over the same K).

  | curve | pairs | centres | set-up | lifts | unused plus points (block tail) | w = 0 | w = 1 |
  |:--|--:|--:|--:|--:|--:|--:|--:|
  | n23 | 180.8 | 22.8 | 8.9 | 0.38 | 4.3 | −166.8 group operations | **+14.0** |
  | n29 | 12.2 | 12.2 | 1.9 | 0.27 | 0.6 | −9.2 | **+3.1** |
  | n37 | 568.3 | 71.2 | 9.0 | 0.43 | 4.5 | −554.0 | **+14.3** |

  The walk is a net cost at w = 1 on every curve. Its saving is field inversions, and it shows in
  wall time: the paired relations-phase difference is −21.5 µs median at n23 (82% of pairs faster)
  and −118 µs at n37 (94%).
- **New uncounted work.**
  - Block bookkeeping (`sw_kc`, `sw_next`, the index i) and reads and stores of at most m stored
    points. This is the same class as the plain walk's uncounted `addmod` per step.
  - One point negation per lift (`g.neg(j)`, a single field XOR), 0.27–0.43 per solve. It should be
    counted for strictness, but it is immaterial (under 2e-4 of any w = 1 total).

## 2. `ladder_verify=1`: a complete, fully counted check that reuses points already formed

- **Complete and sound.**
  - `ladder_mul` forms `[d]G` from the rungs `[2^i]G`, i < L:
    - one counted addition per set bit, the first free;
    - above the chain, one counted doubling of the top rung, a counted double-and-add of
      `d >> L`, and one counted addition.
  - The result is compared with the target by `check == target`, a full point comparison as
    established in VAL-20261118-43658a. No eigenvalue, log or group order is used.
  - A wrong rung would fail the check, and every record is `verified`.
- **The rungs are genuinely formed and charged in the same record.**
  - The base's seed loop now pushes each point it doubles: `G`, then 5 doublings per extra column.
    `prepare` refuses a ladder that does not start at G.
  - On every record, the factor-base phase charges exactly 5(k − 1) doublings: 15 at n23 and
    40 for sw at n37. At n23 the base's group operations and native counts are identical between
    sw and ls on 576 of 576 keys, apart from the key squarings in section 3.
  - The base is built per record (cold, section 6), so nothing is amortized across targets.
- **Counts.** On every sw record, verify = popcount(d) − 1 additions and
  max(0, bitlen(d) − (5(k − 1) + 1)) doublings. On every ls record it is popcount(d) − 1 and
  bitlen(d) − 1. There were 0 mismatches.
  - Verify falls from 30.38 to 15.38 operations per solve at n23 (15 doublings).
  - It falls from 38.97 to 13.11 at n37: 41 rungs cover every bit, so there are no doublings at all.
- **Not a relabel.**
  - `tau_verify` replaced doublings with recode steps that were priced as native operations, and
    its verify wall time rose.
  - `ladder_verify` adds no counter and moves no work; the doublings simply are not repeated.
  - The verify phase's paired wall time falls on 100% of pairs at n23 (−1.77 µs median) and 99% at
    n37 (−4.26 µs).
  - The only new unpriced work is two vector clones of at most 41 points (memory copies).

## 3. `orbit_key_reuse=1`: real work removed; repeated columns still refused

- The key is the least abscissa over `points[first..].step_by(2)`, which is σ^t(seed) for
  t = 0..n − 1. These are exactly the values x^(2^t) that the incumbent recomputed with n − 1
  squarings.
- `seen.insert(key)` still refuses a repeated column, with the same error. The unit test asserts
  identical points, columns, coefficients and `neg_index`.
- `min_key_field_squares_uncharged` falls from k(n − 1) to 0: 88 at n23, and 288 for ls against 0
  for sw at n37. All other base counts are unchanged. n29 (k = 1) is unaffected.
- The minimum over n abscissae is the same class of comparisons the incumbent also left uncounted.
- **The saving is not visible in wall time.**
  - The paired factor-base phase difference at n23 is a median of +2.25 µs, with 61% of pairs
    slower. The A/A median is −0.12 µs.
  - Arm position confounds this. In the `alternate` order sw always runs at position 0 or 5, and ls
    at 2 or 3. But sw is not faster in either position: +2.56 µs at position 0 and +0.91 µs at
    position 5.
  - A GF(2^23) squaring costs far less than a group addition, and 88 of them are below what L0
    timing resolves.

## 4. Where the n23 gain comes from, and the w = 1 reading

The table gives per-solve priced GAE, sw − ls, paired over 432 pairs, from the validator's own
pricing:

| w | factor base (key squarings) | relations (walk) | verify (ladder) | total | ratio |
|:--|--:|--:|--:|--:|--:|
| 0 | 0.0 | −166.8 | −15.0 | −181.8 of 612.0 | 0.7030 |
| 0.1 | −8.8 | −148.7 | −15.0 | −172.4 of 808.9 | 0.7868 |
| 1 | **−88.0** | **+14.7** | −15.0 | **−88.3 of 2580.8** | 0.9658 |

- At w = 1 the walk and the ladder cancel (+14.7 − 15.0). The whole w = 1 margin is the 88 removed
  key squarings, each priced at 1 GAE.
- `min_key_field_squares_uncharged` is in `FIELD_ARITHMETIC_COUNTERS`. AGENTS.md §5a (owner,
  2026-11-12) requires such counters to be read at a per-host `w_cal` as well as on the band. The
  protocol and README report no calibrated reading, and the repository has no binary-field `w_cal`.
- The validator did not invent a `w_cal`. It ran a **sensitivity analysis instead (not a
  measurement)**: the field class priced at wf, every other native counter at w = 1.

  | wf | sw / ls (pooled, 95%) |
  |:--|:--|
  | 0 | 0.9999 [0.9997, 1.0001] |
  | 0.01 | 0.9995 [0.9993, 0.9998] |
  | 0.05 | 0.9981 [0.9979, 0.9984] |
  | 0.1 | 0.9964 [0.9960, 0.9967] |
  | 1 | 0.9658 [0.9634, 0.9682] |

  - The pooled upper end is below 1 only if wf > 0.0035. Per replicate the condition is about
    wf > 0.01; r1 and r2 at wf = 0.01 have an upper end of 1.0000.
  - At w = 0 and 0.1 the order is the same at every wf.
- **Reading.**
  - The frozen rule is the band, and the band verdict governs (§5a). The n23 pass stands.
  - The w = 1 magnitude (3.4%) is the band's price for 88 field squarings. At a plausible
    calibrated price (0.01–0.05 GAE) the gain at w = 1 would be about 0.05–0.2%, borderline per
    replicate.
  - This is not relabelling. Total cost falls at every weight in the band, and total solve wall
    time falls: median 281.9 to 262.0 µs, mean 324.0 to 283.4 µs (L0, descriptive).
  - But it should be booked as an accounting-magnitude effect, not a measured real-time saving at
    w = 1.
  - The README is right that the w = 1 ratio "comes from the ladder check and the key reuse". It
    omits that the walk's +14.7 cancels the ladder, which leaves the 88 squarings alone carrying the
    pass.
- At n37 the k = 9 table adds +315 / +1887 GAE per solve to the oracle set-up at w = 0 / 1. At
  w = 1 that outweighs the walk (−1016), the key squarings (−208 net) and the ladder (−26). The
  failure does not depend on the field price: with wf = 0 it is 1.075.
- Wall time at n37 (descriptive, L0) is lower for sw: median 884.8 to 830.0 µs, mean 1090.7 to
  904.5 µs. On this host, the band's w = 1 price for each extra canonicalisation and table insert
  exceeds its wall-time cost. That is a reading of the convention; it does not change the frozen
  verdict.

## 5. Recomputation

- **ecbench rerun.**
  - The validator reran all 99 `ecbench compare --weight {0,0.1,1} --resamples 4000 --seed 20261002`
    calls of the frozen `analyze.py` (8 pairs × 3 weights pooled per curve, plus the primary per
    replicate). It used the pinned binary on its own session copies, under the CPU lock.
  - Every output is identical to the committed `results.json.gz`, ignoring path fields.
  - The pooled n23 `band --ranks` file is identical to `tables/band-n23-pooled.json`.
- **Independent Python** (shares no code with the producer or ecbench).
  - Formula: priced = total_gae − C + w(U + n_C).
  - U: `cost.unpriced` without the byte counters, plus marker-gated `table_insert_attempts`,
    `sibling_second_points` and `sibling_lift_products`.
  - C and n_C: `PRICED_NATIVE` at the pinned K_1/GF(2^23), K_1/GF(2^29) and K_0/GF(2^37) rows of
    `docs/ic/calibration.json`.
  - Ratio of sums over pairs keyed by (session, workload, round), with workloads as clusters.
  - Results:
    - 90 ratios within 1e-14 (relative);
    - linearised-t intervals within 5.4e-15 (its own Student-t quantile);
    - its own cluster bootstrap within 0.016 of ecbench's;
    - band S identical to `summary.json`.
  - The rule applied to the independent numbers gives the same three verdicts.
  - No unpriced counter lacks a count on any arm.
- **Pooled band S** (w = 0 / 0.1 / 1):

  | curve | ls | sw | rho-v5 | rho-strong |
  |:--|:--|:--|:--|:--|
  | n23 | 0.299 / 0.395 / 1.260 | 0.210 / 0.311 / 1.217 | 0.227 / 0.251 / 0.472 | 0.890 / 0.955 / 1.545 |
  | n29 | 0.349 / 0.451 / 1.370 | 0.304 / 0.413 / 1.388 | 0.345 / 0.375 / 0.641 | |
  | n37 | 0.185 / 0.250 / 0.838 | 0.145 / 0.219 / 0.880 | 0.150 / 0.166 / 0.303 | |

- **Multiplicity.** There is one primary per curve, and the rule needs all 12 cells. With
  Bonferroni over the three curves (98.33% linearised-t), the largest n23 upper end is 0.9717
  (r2 and r3, w = 1). All 12 cells stay below 1.

## 6. Replay, digests, answers, controls

- **Digests.**
  - The nine gunzipped `records.jsonl` hash to `summary.json` → `sessions`:
    - n23: `97f5c09a…`, `3fc66675…`, `83221fbc…`;
    - n29: `59943872…`, `20304720…`, `91936711…`;
    - n37: `08792d13…`, `e78704c3…`, `55c60af9…`.
  - Each also matches its `session.json`: status complete, 1,152 of 1,152 verified, binary
    `7b182360…fd315`.
  - The pinned file `bins/ecbench-sw-217638524d` hashes to `7b182360…fd315`, which matches
    `logs/binary.sha256` and PROTOCOL. It contains the lever strings.
  - Each session's `spec.json` is byte-identical to the registered `specs/*.json`, and each pilot
    spec to `pilot/pilot-n*.json`.
  - The three pilot `records.jsonl` match PROTOCOL's prefixes (`4efc35a7…`, `f60b7381…`,
    `22b5d567…`).
  - `tables/records-n*.csv.gz` equal the raw records on 7,776 of 7,776 measured rows.
  - The two files the benchmark record hashes match.
- **Replay.**
  - `verify --replay-all` on all 12 validator copies, under the CPU lock: 10,368 + 4,464 records
    verified, 7,776 + 2,976 replays reproduced, 0 problems.
  - The audit JSONs equal the committed `audits/` and `pilot/logs/` files except
    `audited_unix_ms`.
- **Cross-binary replay.**
  - The pinned binary replays the validated r1 sessions of `koblitz_lean_sib_confirm_20261118`
    identically: n23 `ECBS1h5c854bfbdf99`, n29 `ECBS1h61314027c2df` and n37 `ECBS1h628553e4500f`.
  - Those sessions were recorded by binary `43f44fec…`, and their records hash to the confirm
    study's committed digests.
  - Result: 864 of 864 each, 0 problems. So `ls`, the former incumbent and both rho references are
    unchanged by `217638524d`.
- **Answers** (the validator's own arithmetic).
  - Field polynomials: `x^23+x^5+1`, `x^29+x^2+1` and `x^37+x^6+x^4+x+1`, each the only trinomial
    or pentanomial that puts G and a target on the curve.
  - `[r]G = O`, `h·r = #E`, and #E equals the Lucas-sequence count.
  - For all 432 targets: Q is on the curve, `[r]Q = O`, one recovered `d` per target across all
    10,368 records, 0 ≤ d < r, and `[d]G = Q`. There were 0 failures.
  - On 16 targets per curve drawn by the nonce (sha256 order), `[d]G = Q` was re-checked with a
    Montgomery ladder. There were 0 mismatches.
- **Controls.**
  - `ls-off` equals `ls` on 576 of 576 keys per curve, warm-ups included. Every field was compared
    except volatile telemetry. The only differing fields are parameter echoes: `method.params.*`,
    `method_id`, `method_sha256`, `fbm_id`, `factor_base.{fb_id, fb_sha256, params.orbit_key_reuse}`
    and `detail.label`.
  - `ls-aa` is fully identical.
  - Both compare at 1.000 [1.000, 1.000] at every weight. They are the null objects; the metric is
    a deterministic count, not a statistical signal.
- **Cold, single target.**
  - Every record is one public target (`hash_to_subgroup_v1`, `planted = null`).
  - Every IC record charges its own factor base, pair table, walk set-up, relations, linear
    algebra and verify.
  - rho-v5 charges its set-up per target (42.9 / 22.5 / 76.0 GAE), and so does rho-strong
    (1056 / 738 / 1333).
- **Memory.** Median max RSS is about 19.9–20.1 MiB for both arms. The walk stores at most m + 1
  points and the ladder at most 41.

## 7. Pilot, selection, registration order

- **Timeline** (commit times equal the reflog):
  - code `217638524d`, 18:04:11Z;
  - binary built 19:16Z (26 min 28 s; the interrupted first build log is kept);
  - pilot rule, pilot specs and `pilot_select.py` committed `d321041bde`, 19:17:20Z;
  - pilot sessions 19:17:26–19:18:09Z;
  - registration `f164bf67fc`, 19:19:48Z (`registered-at.txt` 19:19:49Z), with `selection.json`
    and `make_specs.py` RULE `{23: (4, 8), 29: (1, 1), 37: (9, 8)}`;
  - confirmatory sessions 19:19:54–19:21:45Z, in `run.sh`'s interleaved order;
  - results 19:25:42Z.
- The 15 registered files hash to `REGISTRATION.sha256` at the registration commit, at HEAD and in
  the working tree.
- Only the 3 pilot and 9 confirmatory session specs in the scratchpad contain `walk_sib`,
  `ladder_verify` or `orbit_key_reuse`. No undisclosed session was found.
- **Seeds.**
  - Pilot target seeds 20261118471–473 and confirmatory 20261118481–489 are distinct. Neither was
    used anywhere in git before this study.
  - One n29 confirmatory target (r3, index 40) coincides with a pilot target (index 27). With
    r = 42457, 48 × 144 draws give about 0.16 such coincidences by chance. It is a birthday
    collision of independent seeds, it affects nothing, and the n29 pick had k fixed at 1.
- **Selection applied as frozen.**
  - The validator reran the pilot compares with its own script: every pilot point ratio equals
    `selection.json` exactly.
  - Applying the rule independently gives the same picks:
    - n23: c4-m8, mean 0.8175 (all < 1);
    - n29: c1-m1, no arm below 1 at all weights, smallest maximum 1.0152;
    - n37: c9-m8, mean 0.7886.
  - The pilot's n37 c9-m8 interval at w = 1 was [0.820, 1.053]. The PROTOCOL disclosed before the
    data that the k = 9 call was uncertain.
- **Caveat.** The registration was not pushed before the data: the remote ref moved from
  `3efdc10e7f` (17:19:57Z) straight to `e7953e1a51` (19:26:19Z). There is no pcc preregistration
  claim, so the order rests on local commit and file times. That is a limitation for any tier claim.

## 8. The count model (not evidence)

- `derived_counterfactual.json` gives n23-k4-m8 0.702952 / 0.786811 / 0.965797 and n29-k1-m1
  0.872608 / 0.915190 / 1.012716. These equal the measured ratios to 1e-16.
- This is expected. The walks are unchanged, the counts deterministic, and the model's lift rule
  holds on all 1,296 records.
- The validator's own derivation from this study's n37 `ls` records gives 0.7360 / 0.8166 / 0.9765
  for n37 k = 8, m = 8. It uses the same count rules, checked exactly on the measured sw arms.
- That matches the file. It is a post-hoc derivation, not a measurement, and it supports no claim.
  It is correctly labelled an open thread. The concurrent `koblitz_n37_sibwalk_k8_20261118` study
  was not examined.

## 9. Classification, gate, limitations

- **Classification.** n23 sw is a T0 engineering gain of the validated incumbent `ls` at K1 a=1 n23
  under the frozen band rule.
  - Same boundary, unit, targets, oracle, table and pricing.
  - The removed work is executed work: field inversions in the walk, doublings that recomputed
    `[2^i]G`, and squarings that recomputed Frobenius abscissae.
  - It is not a relabelling. One magnitude is accounting, though: the w = 1 margin is the band's
    1-GAE price for 88 field squarings (section 4).
  - It is still a Frobenius- and negation-folded BSGS under the IC harness, and its asymptotic
    ceiling is unchanged.
  - n29 and n37 are recorded failures at the tested parameters. At n37 that means k = 9; k = 8 is
    unmeasured here.
- **Gap to rho.**
  - n23 sw against rho-v5, the strongest measured reference in the same session and binary, is
    unresolved inside the band. It is below at w = 0 by the point estimate only (0.927, interval to
    1.020), and 2.58× at w = 1. **Not below rho.**
  - rho-v5 leaves its jump-index selection untallied, which slightly favours rho at w > 0.
- **Gate (docs/VALIDATION_GATE.md).** Not satisfiable yet.
  - The nine raw sessions are only in the producer scratchpad.
  - The benchmark record's `raw[]` lists derived files (`results.json.gz`, `summary.json`), not the
    session records, all `uploaded: false`.
  - The committed CSVs carry only `total_gae` and `S`, so no band headline recomputes from
    in-repository data.
  - A `VALIDATION.json` needs four things: the sessions archived, a replay certificate, this report,
    and a rho descriptor (`negation_frobenius`, rho-v5, `order_holds_across_band: false` for n23 sw).
- **Limitations and minor findings.**
  - **No §5a calibrated reading.** The field-arithmetic counter is decisive at w = 1 on n23
    (section 4). A binary-field `w_cal` (best-of-N timing, labelled) should accompany any row.
  - **Binary binding.** The ecbench binary was not rebuilt by the validator, and the unit test
    `k1_sibling_walk_ladder_verify_and_key_reuse_keep_the_solve` was not run.
    - Disk: 3.8 GB free against about 4 GB needed, and another session shares the checkout.
    - The build worktree is deleted, the build log does not name the commit, and the session
      records `git_commit: null`.
    - The binding therefore rests on the hash pin, the lever strings, unchanged source from
      `217638524d` to HEAD, the cross-binary replay, and exact agreement of every record with
      counts derived from the code at `217638524d`.
  - **Registration.** No pcc preregistration, and the freeze was not pushed before the data.
  - **Timing.** Local L0 shared host, `--cpus none --allow-busy`, no Iso Lab, EKS or S3, as
    declared. Wall times are descriptive and position-confounded by the `alternate` order.
  - **Uncounted negations.** One point negation per lift is uncounted (section 1, immaterial).
  - **Secondary unit.** In `bitop/v1` (not the claim's unit), `sibling_second_points` is priced as
    1 squaring + 2 additions, with no multiplication. The descriptive `walk_sibling_*` counters are
    listed there as unpriced.
  - **README imprecisions.**
    - n29 `ls` / rho-v5 band order is "unresolved" in the README and `rho-v5 < ls` by the band.
    - The n37 verdict column omits r3 at w = 0.1.
    - "oracle set-up 1350 → 1708 at w = 0" are phase GAE including calibrated natives; the w = 0
      band figure is 1188 → 1503.
    - The benchmark record's metric notes call n37 "K1"; the curve is K0 (a = 0).
  - **Large file.** `results.json.gz` (101,263 B) is data-suffixed and over 64 KiB, so it belongs in
    the artifact store.

## validation_report

```yaml
validation_report:
  id: VAL-20261118-99f8e0
  task_id: null  # dispatched by the producing session; no TASK card was named
  snapshot: e7953e1a5115224d911d839252ab75159b27b42c
  code_commit: 217638524db18d2c59c61f019a529ae10fb63dce
  pilot_rule_commit: d321041bdeb973ca53565496a91f2358c764f79d
  registration_commit: f164bf67fcade81da719281829ee583b75e1e6ef
  nonce: dbccd01fcef3e99c138508570d579943  # 2026-11-18T19:27:50Z, after results commit and push
  run_ids:
    - ECBS1h2ec43bcc2551  # n23-r1 97f5c09a...
    - ECBS1hf5fd8d78e1eb  # n23-r2 3fc66675...
    - ECBS1hc715681e6b51  # n23-r3 83221fbc...
    - ECBS1h45c52306814f  # n29-r1 59943872...
    - ECBS1h40d710b3940f  # n29-r2 20304720...
    - ECBS1hec202af9ca5c  # n29-r3 91936711...
    - ECBS1heee27829bf60  # n37-r1 08792d13...
    - ECBS1h08dced6df30f  # n37-r2 e78704c3...
    - ECBS1ha1aac934b706  # n37-r3 55c60af9...
    - ECBS1hbbe3c979c823  # pilot n23 4efc35a7...
    - ECBS1h92c19e058f57  # pilot n29 f60b7381...
    - ECBS1hb820e40707b1  # pilot n37 22b5d567...
    - ECBS1h5c854bfbdf99, ECBS1h61314027c2df, ECBS1h628553e4500f  # lean+sib confirm r1 sessions (cross-binary replay)
  artifact_checks:
    - 9 confirmatory + 3 pilot records.jsonl sha256 = summary.json / PROTOCOL = session.json; binary 7b182360...fd315 = logs/binary.sha256 = PROTOCOL = session.json
    - session spec.json byte-identical to registered specs; REGISTRATION.sha256 holds at f164bf67fc, e7953e1a51 and working tree
    - tables/records-n*.csv.gz = raw (0/7776 mismatches); benchmark record hashes match; committed audits = validator replay audits except audited_unix_ms
    - order: code 18:04:11Z < pilot rule 19:17:20Z < pilot data 19:17:26Z < registration 19:19:48Z < confirmatory data 19:19:54Z < results 19:25:42Z (local; first push 19:26:19Z)
    - src/crates/Cargo unchanged 217638524d..HEAD; concurrent commits 624775314b, e97ce22df9 touch neither this study nor source
  metric_recomputations:
    - n23 sw/ls 0.70295 [0.68586,0.72140] / 0.78681 [0.77426,0.80015] / 0.96580 [0.96326,0.96824]; 12/12 upper < 1 (max 0.9707)
    - n29 sw/ls 0.87261 / 0.91519 / 1.01272 [1.01126,1.01421]; w=1 fails pooled and in r1, r2, r3
    - n37 sw/ls 0.78517 / 0.87392 / 1.05020 [1.00599,1.09516]; w=1 fails pooled, r1, r2, r3; w=0.1 fails in r3 (upper 1.0009)
    - gaps: n23 sw/rho-v5 0.927 [0.847,1.020] / 1.237 [1.131,1.361] / 2.577 [2.355,2.842], unresolved; sw/rho-strong 0.236/0.325/0.788, below; all README gap cells identical (n29 ls/rho-v5 band rank is rho-v5 < ls, README says unresolved)
    - method: 99 ecbench compare calls rerun (identical); independent Python band and ratios (1e-14), own t quantile (5.4e-15), own bootstrap (within 0.016); band S identical
    - n23 decomposition (GAE/solve, sw - ls): w=0 walk -166.8, ladder -15.0; w=1 key squarings -88.0, walk +14.7, ladder -15.0
    - sensitivity (not a measurement): n23 w=1 with field class at wf: 0.9999 [0.9997,1.0001] at wf=0; pooled upper < 1 iff wf > 0.0035
    - Bonferroni over 3 curves (98.33% linearised-t): n23 largest upper 0.9717; all 12 cells < 1
  control_checks:
    - ls-off = ls on 576/576 keys per curve except parameter echoes; ls-aa = ls fully; both 1.000 [1,1] at all weights
    - n23, n29 sw vs ls on 576/576 keys: same targets_tried, trials, relations, recovered d, algorithm_seed, linear algebra, oracle set-up, points_sha256
    - verify --replay-all on 12 sessions: 0 problems (7776 + 2976 replays reproduced); cross-binary replay of 3 validated incumbent sessions 864/864 each
    - independent arithmetic: [d]G = Q, Q on curve, [r]Q = O for all 432 targets / 10368 records; nonce sample of 48 re-checked by Montgomery ladder; sibling formula = two full additions on 72000 pairs
    - walk, verify, key-square counts equal the validator's derivation on all 6912 IC records (0 mismatches); lifts charged (add + canonicalisation + lookup) on every lifted hit
    - pilot ratios equal selection.json; rule re-applied independently gives c4-m8, c1-m1, c9-m8
  heuristic_validation_checks:
    - frozen expectations (n23 ~0.70/0.79/0.97; n29 ~1.015 at w=1, expected to fail; n37 w=1 uncertain) recorded before data; observed match
    - count model reproduces n23/n29 measured ratios to 1e-16 (expected for deterministic counts on unchanged walks); n37 k=8 derived 0.736/0.817/0.977 reproduced arithmetically; post hoc, not evidence
    - null objects: ls-aa and ls-off, exactly 1.000
  cost_model_checks:
    - unit ecbench.gae; S = GAE/sqrt(r); band w = 0, 0.1, 1 with table and sibling work priced behind markers; no missing unpriced count on any arm
    - sibling walk: one addition + one sibling_second_points (priced at w) per pair; set-up m+1; lift = one addition + counted canonicalisation and lookup; net +14.0 / +3.1 / +14.3 GAE per solve at w=1
    - ladder_verify: popcount(d)-1 additions, max(0, bitlen(d)-L) doublings; reads base-formed, base-charged rungs from the same record; verify wall falls on 99-100% of pairs; not a relabel
    - orbit_key_reuse: k(n-1) squarings removed (executed work); priced at 1 GAE each at w=1, which carries the whole n23 w=1 margin; wall saving not detectable; AGENTS.md section 5a calibrated reading for field-arithmetic counters missing
    - cold single-target: every phase charged per record on IC arms; rho set-ups per target; memory unchanged (~20 MiB)
  proof_architecture_checks: []
  verdict:
    n23: passed (frozen rule met at all 12 cells; w=1 margin is the band's price for 88 field squarings)
    n29: failed (as claimed; w=1 above 1 pooled and in every replicate)
    n37: failed (as claimed; w=1 pooled and replicates; also w=0.1 in r3)
    gap_statements: passed (n23 sw unresolved against rho-v5; not below rho)
    receipt: passed (admissible on all three curves)
  classification: n23 T0 engineering gain over validated incumbent ls (VAL-20261118-9a42f2); not below rho; not a relabelling; the w=1 magnitude is accounting under the band convention
  limitations:
    - raw sessions only in the producer scratchpad; benchmark record raw[] lists derived files; gate needs sessions archived before VALIDATION.json
    - no AGENTS.md section 5a calibrated (w_cal) reading for min_key_field_squares_uncharged; decisive at w=1 on n23
    - no pcc preregistration; registration not pushed before the data
    - ecbench binary not rebuilt and unit test not run by the validator (disk, shared checkout); binding by hash, strings, unchanged source, cross-binary replay and exact count derivation on every record
    - L0 shared local host; wall times descriptive and position-confounded by the alternate order
    - one uncounted point negation per lift; bitop/v1 secondary unit omits the sibling point's multiplication
    - README minor imprecisions (n29 ls/rho-v5 order, n37 r3 w=0.1, n37 set-up figure units, K0 labelled K1 in the benchmark record)
  artifact_paths:
    - research/koblitz_k1_sibwalk_20261118/PROTOCOL.md
    - research/koblitz_k1_sibwalk_20261118/README.md
    - research/koblitz_k1_sibwalk_20261118/REGISTRATION.sha256
    - research/koblitz_k1_sibwalk_20261118/summary.json
    - research/koblitz_k1_sibwalk_20261118/results.json.gz
    - research/koblitz_k1_sibwalk_20261118/pilot/selection.json
    - research/koblitz_k1_sibwalk_20261118/derived_counterfactual.json
    - research/koblitz_k1_sibwalk_20261118/audits/audit-n*-r*.json.gz
    - research/koblitz_k1_sibwalk_20261118/tables/
    - benchmarks/records/20261118T1917Z-koblitz-k1-sibwalk.json
    - (scratchpad, not durable) sw-sessions/n*-r*/records.jsonl.gz; sw-pilot/pilot-n*/records.jsonl.gz; bins/ecbench-sw-217638524d; validator workspace val-sw/
  review_attestation:
    joints_owned: [whole receipt, as tasked]
    read_sibling_reports: false  # research/koblitz_lean_sib_confirm_20261118/VALIDATOR_REPORT.md (VAL-20261118-9a42f2) and research/koblitz_n29_orbit_levers_20261118/VALIDATOR_REPORT.md (VAL-20261118-43658a) were read as prior context, as the task directed; they are earlier rounds, not siblings. The concurrent koblitz_n37_sibwalk_k8_20261118 study was not read beyond commit messages and file names.
    sources_read:
      - research/koblitz_k1_sibwalk_20261118/{PROTOCOL.md,README.md,REGISTRATION.sha256,registered-at.txt,run.sh,run_pilot.sh,analyze.py,summarize.py,pilot_select.py,pilot/SELECTION_RULE.md,pilot/selection.json,pilot/pilot-n*.json,specs/*.json,summary.json,results.json.gz,derived_counterfactual.json,prof/model.py,audits/*,pilot/logs/audit-pilot-*,tables/records-n*.csv.gz,tables/band-n23-pooled.json,logs/binary.sha256}
      - git show 217638524d (ic_boundary.rs, stages.rs, plugins.rs, mod.rs, cm_orbit.rs); ic_boundary.rs at 217638524d (ladder_mul, RecoveryCheck, relation loop, TargetSource guards); ecbench band.rs, compare.rs, methods.rs PRICED_NATIVE, solver_units.rs PINNABLE_NATIVE, work.rs sibling mapping at 217638524d
      - docs/ic/calibration.json; docs/VALIDATION_GATE.md; AGENTS.md (What counts, section 3 classes, section 5a, cold interval rule)
      - benchmarks/records/20261118T1917Z-koblitz-k1-sibwalk.json
      - research/koblitz_lean_sib_confirm_20261118/{VALIDATOR_REPORT.md,summary.json (sessions digests)}; research/koblitz_n29_orbit_levers_20261118/VALIDATOR_REPORT.md
      - scratchpad sw-sessions/*, sw-pilot/*, lsc-sessions/n*-r1 (validator copies), wt-sw-build*.log
    verdict: n23 passed; n29 failed (as claimed); n37 failed (as claimed); receipt admissible
```
