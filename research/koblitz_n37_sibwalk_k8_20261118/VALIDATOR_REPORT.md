# Validator report VAL-20261118-24d204: sibling walk (k = 8, m = 8) + ladder check + orbit-key reuse vs the lean+sib incumbent, K1 a = 0 n = 37

This report independently validates `research/koblitz_n37_sibwalk_k8_20261118/` on branch
`claude/f4-f5-algorithm-speed-bvp7qd`. The committed snapshot is `e97ce22df9` (results). It rests on
`624775314b` (registration) and `217638524d` (code). The validator did not design, write or run any part
of the study and edited no producer file. It did not read the other validator's report on the parent study
(`research/koblitz_k1_sibwalk_20261118/VALIDATOR_REPORT.md`, untracked in the working tree during this
validation).

Sources read:

- this study's `PROTOCOL.md`, `README.md`, `run.sh`, `make_specs.py`, `analyze.py`, `summarize.py`,
  `model.py`, `tables/model_check.json`, the specs, the audits and `summary.json` / `results.json.gz`;
- the parent study `koblitz_k1_sibwalk_20261118` (README, `pilot/SELECTION_RULE.md`, `pilot/selection.json`,
  `analyze.py`, `summarize.py`, `prof/model.py`, and its n37 `ls` raw records for the m-curve check);
- VAL-20261118-9a42f2 (`koblitz_lean_sib_confirm_20261118`) and VAL-20261118-43658a
  (`koblitz_n29_orbit_levers_20261118`, read through `git show`; sparse checkout);
- the lever diff `217638524d` (`plugins.rs`, `ic_boundary.rs`, `ic_framework/mod.rs`, `stages.rs`), the
  incumbent paths around it (`StrideGenerator`, `ladder_mul`, `RecoveryCheck`, `FrobeniusPairTable::
  build_layout_with` and `probe_x`, the compact-orbit-scan key loop), `ecbench/band.rs` (pricing,
  `FIELD_ARITHMETIC_COUNTERS`) and the `compare.rs` interval definition;
- AGENTS.md "Success criterion", §5a, and `docs/ic/calibration.json` (row `koblitz/K_0 / GF(2^37)`);
- the three raw sessions named by the README, through byte-identical validator copies, and the
  `koblitz_lean_sib_confirm_20261118` raw session n37-r1 (cross-binary replay).

Validator nonce: `b98c243d2a5e27b4db6c8395fe19dec7`, drawn 2026-11-18T19:38:58Z. That is after the results
commit (19:33:26Z) and its push (19:33:56Z).

## Verdict

"Passed" means the receipt is admissible evidence for the claim as stated, on K1 a = 0 n = 37 only. The claim
is a T0-scope engineering gain over the validated incumbent `ls` (VAL-20261118-9a42f2) in the generic-
decomposition track (pair-table oracle). It is not an ECDLP claim, not a claim below rho, and changes no
exponent. It does not authorize a leaderboard or `benchmarks/README.md` row by itself (section 8).

| claim | recomputed | frozen rule / reading | verdict |
|:--|:--|:--|:--|
| **sw / ls pooled** 0.724 [0.705, 0.745] / 0.808 [0.794, 0.823] / 0.977 [0.976, 0.979] | 0.72422 [0.70523, 0.74455] / 0.80792 [0.79386, 0.82275] / 0.97742 [0.97589, 0.97893] (ecbench rerun bit-identical; independent Python: points within 1e-14 rel., linearised-t ends within 5e-15) | upper 95% < 1 at w = 0, 0.1, 1 pooled and in r1, r2, r3: **12 of 12**; largest 0.98142 (r3, w = 1) | **passed** |
| r1 / r2 / r3 | 0.727 / 0.743 / 0.706 (w = 0); 0.810 / 0.822 / 0.795 (0.1); 0.977 / 0.976 / 0.979 (1) | identical | passed |
| gap sw / rho-v5 0.900 [0.825, 0.983] / 1.225 / 2.673 | 0.90042 [0.82523, 0.98325] / 1.22541 [1.12481, 1.33730] / 2.67316 [2.45775, 2.91496]; band rank `unresolved inside the band` | never a gate | passed: **not below rho** |
| gap sw / rho-strong 0.516 / 0.683 / 1.327 (unresolved) | identical; `unresolved inside the band` | never a gate | passed |
| before: ls / rho-v5 1.243 / 1.517 / 2.735 (`rho-v5 < ls`), ls / rho-strong 0.713 / 0.846 / 1.357 | identical | | passed |
| controls: ls-aa, ls-off identical to ls; all solves verified; same targets and log | 576 / 576 keys each (warm-ups included); 3,456 / 3,456 verified; same `targets_tried` and log 576 / 576; independent first-hit index equals `targets_tried` on all 144 targets | void-if-fail controls hold | passed |

**Overall: passed.** The three readings that matter most:

- **At w = 1 the 0.977 is almost entirely 288 field squarings priced at 1 GAE each.** Per solve at w = 1 the
  change is −298.9 GAE: orbit-key reuse −288.0, ladder check −25.9, sibling walk **+15.0**. The squarings are
  in `FIELD_ARITHMETIC_COUNTERS` (the §5a mulmod class). The study did not report the §5a calibrated
  reading. Priced at 0 (the worst case for `sw`, since only `ls` carries the counter), the validator's
  derived reading is 0.99916 (upper 0.99923) at w = 1 pooled, and every replicate upper end is ≤ 0.99936.
  So the order holds at every band weight for **any** w_cal ≥ 0. The real w = 1 gain is the ladder check net
  of the walk, about 0.08%.
- **ladder_verify is a real saving, not a relabel** (section 2). It is fully counted, it lowers the priced
  verify phase at every weight with no new native work, and the verify wall time falls from 7.7 to 3.05 µs
  (lower in 429 of 432 pairs). That is the opposite of the `tau_verify` pattern that VAL-20261118-43658a
  rejected.
- **The sibling walk is sound and complete, and slightly worse at w = 1.** Its gain is at w = 0 and 0.1
  (relations −795.7 / −714.6 GAE per solve; wall 571 to 355 µs). The README's "neutral at w = 1" is +15.0
  GAE per solve (+0.11% of the total) in fact.

## 1. Sibling walk: same targets, same order, complete probe, every operation charged

Code (`217638524d`, `TargetSource::StrideGeneratorSib`, `x_sibling_pair`, `FrobeniusPairTable::probe_x`):

- **Same progression.** Both walks try `R_k = Q + [k]G`, k = 0, 1, …, with `R_0 = Q` at no cost, and stop
  at k = r. The sibling walk forms the block around `C = Q + [k_c]G`, with k_c = m + 1, 3m + 2, … For
  k < k_c it forms the pair i = k_c − k from ONE inversion: `C + [i]G` in full (kept for k = k_c + i) and the
  abscissa of `C − [i]G = R_k`. k = k_c is C itself, and k > k_c reads the kept plus point. `trials` rises
  once per target on every path, so the order and the count are those of `StrideGenerator`.
- **The x-only probe is complete.** `probe_x` canonicalises the abscissa and looks it up (one counted
  canonicalisation and one lookup, as the full probe's miss path). The table key is a function of the
  abscissa's ⟨σ, −1⟩ class only. So "x in table" is "some sign of the point decomposes", and the probe
  misses exactly when the full probe misses. On a hit, the point is formed by one counted addition and
  decomposed as usual (a second canonicalisation and lookup, also counted).
- **Equal-abscissa fallback.** `x_sibling_pair` returns `None` when `C = ±[i]G` or C is infinite. Both signs
  are then formed in full (`walk_sibling_full_pairs`). This was 0 on every record.

Records (all 576 `sw`/`ls` key pairs, warm-ups included; validator code):

- Relations phase of `ls`: adds = `targets_tried`, 0 doublings. That is K − 1 walk additions plus one
  common addition on the hit, which `sw` has too.
- Relations phase of `sw`: the validator derived the walk counts from `targets_tried` alone. Set-up
  1 doubling + (m + 1) additions, one centre per later block, one addition per pair, one lift on a
  minus-side hit, plus the common hit addition. Adds, doublings, `walk_sibling_pairs` and
  `walk_sibling_lifts` match on 576 / 576 records.
- `sibling_second_points` = `walk_sibling_pairs` = `walk_x_only_probes`, behind
  `counted:sibling_second_points`, on 576 / 576.
- `canonicalisations` and `lookups` of `sw` = those of `ls` + lifts on 576 / 576. The extra probe on a hit
  is charged.
- `targets_tried`, `recovered`, `algorithm_seed`, target and `points_sha256` are equal to `ls` on
  576 / 576. The oracle set-up and linear-algebra phases are identical (wall time excluded).

Pricing:

- The band adds `sibling_second_points` to `U` behind its marker (`band::table_work_counts`). The
  validator's own pricing includes it and reproduces ecbench to 1e-14. Leaving it out would read about
  0.916 at w = 1 instead of 0.977, so **it is priced at w in every reported number**.
- A pair executes I + 3M + 2S and is charged one addition (I + 2M + S) plus one second point at w. The extra
  M + S is free at w = 0 (slightly optimistic) and about fair near w = 0.1. This is the convention of the
  layout table's `sib=1` (VAL-20261118-9a42f2).

Independent arithmetic. This is the validator's own GF(2^37) with `x^37+x^6+x^4+x+1` and its own curve code,
sharing nothing with the producer:

- `G` is on the curve, `[r]G = O` and `h·r = #E` (596 · 230,603,167 = 137,439,487,532).
- The pair-table classes, built from all `S_o + ε σ^t(S_o')` with `o ≤ o'`, number **2,326** =
  `pair_table_entries`.
- For **all 144 targets**, the first k with `canon(x(Q + [k]G))` in the table, plus one, equals the
  recorded `targets_tried` of `ls`, `ls-aa`, `ls-off` and `sw` in every round.
- Along each walk up to the hit (116,590 pairs over the 144 targets), the producer's sibling formula
  (`λ₋ = λ₊ + x_J/(x_C + x_J)`) gives `C + [i]G` and `x(C − [i]G)` equal to two full additions:
  **0 mismatches**. The x-only probe verdict equals the full-point probe verdict on every one.

## 2. `ladder_verify`: a complete [d]G = Q check on charged, already-formed points (not a relabel)

- **What it reads.** The compact-orbit-scan builder pushes `G` and then every doubling on its way to the
  seeds `[2^(5(j−1))]G`. With 8 columns that is `[2^i]G` for i = 0..35, formed by 35 counted doublings in
  the factor-base phase. Those 35 doublings (42.9 GAE with the Frobenius maps) are charged in **every**
  `ls` and `sw` record, so cold single-target accounting is kept. The ladder points are those values, kept
  instead of dropped (36 points, memory only; median max RSS 20,036 / 20,072 KiB for `ls` / `sw`).
- **Guards.** The oracle refuses the lever without a stored chain or when `ladder[0] != G`. It also refuses
  it with `tau_verify`.
- **What it computes.** `ladder_mul` sums `ladder[i]` over the set bits of d (popcount − 1 counted
  additions; the first rung is free, as double-and-add's first bit). Bits above the chain would fall back to
  doubling, but none occur (d < r < 2^28). The result is compared with the target by full point equality
  (`check == target`). It is the same complete check as double-and-add. The only change is that the
  doublings are read instead of recomputed.
- **Soundness does not depend on trusting the chain.** The chain is built before and independently of the
  target, so a wrong rung could only make the check fail. 0 `verification_failures`, and every one of the
  3,456 solves is `verified` with `matches_target`.
- **Counts.**
  - On 576 / 576 `sw` records, verify = (popcount(d) − 1) additions and 0 doublings.
  - On 576 / 576 `ls` records it is (popcount(d) − 1, bitlen(d) − 1), so 38.74 falls to 12.84 group
    operations per solve.
  - The verify phase has no native counter in either arm. Nothing is moved into an unpriced or w-priced
    counter. Double-and-add and the ladder both loop over the bits of d, so the uncounted loop work is
    comparable.
- **Independent check.**
  - With its own doublings, the validator built `[2^i]G` and summed the set bits of each recorded d. The
    sum equals Q for all 144 targets, with the recorded addition count.
  - A Montgomery ladder (a second route) also gives `[d]G = Q` on all 144.
  - Every Q is on the curve with `[r]Q = O`, and all 3,456 records agree on d per target.
- **Relabel test** (AGENTS.md "What counts"; the standard VAL-20261118-43658a applied to `tau_verify`):
  - The headline count falls, and so does the priced total, by 25.9 GAE at **every** weight.
  - The wall time falls too: verify 7.7 to 3.05 µs mean (7.16 to 2.81 µs median), lower in 429 of 432
    pairs.
  - `tau_verify` failed this test: its recoding steps were native work priced at w, it read 1.030 at
    w = 1, and its verify wall time rose. `ladder_verify` has no recoding.
  - **Not a relabel.** It is a small but real saving: reuse, within one cold solve, of points the solve has
    already formed and paid for.

## 3. `orbit_key_reuse`: real work removed, refusal kept; wall effect below resolution

- **Code.**
  - The incumbent computes each seed's key as min over `x, x², …, x^(2^36)` (36 counted
    `min_key_field_squares`). It refuses the column if `seen` already holds the key.
  - With the lever, the key is the min over the abscissae of the orbit members the builder forms anyway:
    `points[first..].step_by(2)`, the `σ^t(S)` half of the (σ^t(S), −σ^t(S)) pairs, already counted as 37
    Frobenius maps. The same `seen.insert(key)` refusal follows ("the orbit of [j]G repeats an earlier
    column").
  - The key set is identical by construction (`x(σ^t S) = x^(2^t)`).
  - `orbit_key_reuse` refuses values other than 0 or 1 and needs `generator_orbit=1`.
- **Records.**
  - On 576 / 576 pairs, `ls` has 288 `min_key_field_squares_uncharged` and `sw` has 0.
  - Every other factor-base native count, the 35 doublings and `points_sha256` are equal.
  - `sw`'s `cost.unpriced` is `ls`'s minus exactly that counter.
  - In both arms, every non-zero `*_uncharged` native counter is listed in `cost.unpriced`.
- **Independent check.** For the 8 seeds `[2^(5(j−1))]G`, min over 36 squarings equals min over the formed
  Frobenius images, the orbits close after 37 maps, and the 8 keys are distinct. So the refusal is not
  triggered here, and it would be triggered identically in both arms. No negative-control run exercising
  the refusal with the lever on was made (none exists in this study). The refusal rests on code reading.
- **Wall time.**
  - The removal does not show. The factor-base phase median is 54.4 µs for `ls` and 56.4 µs for `sw`
    (paired median +3.0 µs; `sw` higher in 260 of 432).
  - The `oracle_setup` phase does identical work in both arms and also reads +5.8 µs for `sw`. A position
    or cache effect of a few µs is therefore present (`sw` ran before `ls` in 288 of 432 pairs).
  - 288 squarings in GF(2^37) are expected to cost well under 1 µs. The saving is real executed work but
    below the resolution of this L0 measurement.
- **Consequence for the headline (§5a calibrated reading).**
  - `min_key_field_squares_uncharged` is in `FIELD_ARITHMETIC_COUNTERS`. AGENTS.md §5a (2026-11-12) asks
    for those counters to be priced at 0, w_cal and 10·w_cal beside the band. The study does not report
    this; the validator derived it. No w_cal was measured, so these are not measurements.

    | field-class weight | w = 0 | w = 0.1 | w = 1 | largest linearised-t upper end over pooled, r1–r3 (w = 0 / 0.1 / 1) |
    |:--|:--|:--|:--|:--|
    | 0 (worst for sw) | 0.72422 | 0.81377 | 0.99916 | 0.78384 / 0.85863 / 0.99936 |
    | 0.013 (illustrative) | 0.72331 | 0.81301 | 0.99887 | 0.78256 / 0.85760 / 0.99912 |
    | 0.13 (illustrative) | 0.71523 | 0.80618 | 0.99628 | 0.77123 / 0.84851 / 0.99697 |

  - The field-class weight only adds to `ls`, so the ratio is monotone in it. The order `sw < ls` holds at
    every band weight for every w_cal ≥ 0: **ranked at calibrated price**.
  - But the magnitude at w = 1 is about 0.999, not 0.977. The band verdict governs (§5a); the 0.977 should
    be read as "288 squarings at 1 GAE each".

## 4. Registration, the no-pilot choice of m, and what was seen before the freeze

- **Order** (local commit and reflog times):

  | event | time (UTC) |
  |:--|:--|
  | code `217638524d` | 18:04:11 |
  | binary file | 19:16:56 |
  | parent study pilot, registration and results | 19:17:20–19:25:42 |
  | **this registration `624775314b`** | **19:29:26** |
  | benchmark record created | 19:29:35 |
  | `run.sh` started | 19:29:40 |
  | sessions | 19:29:41.5–19:30:20.1 |
  | results `e97ce22df9` | 19:33:26 |
  | first push (both commits) | 19:33:56 |

  The registration was not pushed before the data, as declared (PROTOCOL §5).
- **Integrity.**
  - The 10 registered files hash to `REGISTRATION.sha256` at both commits, and none changed between them.
  - Each session's `spec.json` is byte-identical to the registered spec.
  - The seeds 20261118491–493 do not occur in the tree before the registration.
  - No session with these specs or seeds exists in the scratchpad before 19:29:41Z. The only earlier
    sibling-walk sessions there are the parent study's: its pilot (seeds 471–473, which include k = 8
    arms at n37; see the disclosure gap below), its confirmatory sessions (seeds 481–489) and a validator
    copy of them.
  - `src/`, `Cargo.toml` and `Cargo.lock` are unchanged from `217638524d` to `e97ce22df9`.
  - `analyze.py` and `summarize.py` differ from the parent's frozen scripts only in the session directory,
    the curve list and the docstring. `model.py` is byte-identical to the parent's `prof/model.py`.
- **m = 8 without a pilot, declared before data** (PROTOCOL §1, in the registration commit). The argument
  is checkable and holds:
  - At k = 8 the walk's targets are those of `ls` (shown in section 1), so m changes only the grouping.
  - The validator's own count model (not the producer's) on the parent's n37 `ls` records (seeds 487–489)
    reproduces the PROTOCOL §0 m-table to within 0.001 in every cell. At m = 8 it gives
    0.736 / 0.817 / 0.977.
  - On this study's own `ls` records, the same model gives exactly the measured 0.724 / 0.808 / 0.977.
  - It also shows m = 16 would have read 0.719 / 0.805 / 0.978. So m = 8 is not a post-hoc best: it is
    marginally worse at w = 0 and 0.1 and marginally better at w = 1.
- **Disclosure gap (minor).**
  - k = 8 was chosen after the parent's k = 9 failed at n37, and that is disclosed. Before this freeze the
    parent's pilot (`pilot/selection.json`, seed 20261118473, 96 pairs) had also measured the c8-m8 arm at
    n37: 0.721 / 0.806 / 0.978 (upper 0.762 / 0.835 / 0.981).
  - PROTOCOL §0 cites the derived counterfactual but not this pilot reading.
  - It does not affect validity: the confirmatory seeds are fresh, and the effect is a deterministic
    function of the unchanged walk.
- **Multiplicity.**
  - This is the thread's second attempt at n37, with no Holm correction and no pcc preregistration.
  - The margins make that immaterial for the band verdict. At w = 1, r3 has R = 0.97891 and
    se ≈ 0.0012, so even a 99.9% bound is about 0.983.
  - In the field-class-at-0 reading the bound is about 0.9995.

## 5. Recomputation, replay, digests

- **ecbench rerun.** With the pinned binary, under the CPU lock, on validator copies, the validator reran
  all 33 `compare` calls of `analyze.py` (8 pairs × 3 weights pooled, plus the primary per replicate,
  `--resamples 4000 --seed 20261002`) and the 4 `band --ranks` calls.
  - Every field equals the committed `results.json.gz`, except session paths.
  - The band files equal `tables/band-*.json`.
  - `summarize.py`, run on the committed `results.json.gz` in a scratch copy, reproduces `summary.json`
    byte for byte.
- **Independent Python** (no producer code).
  - Pricing: `priced = G − C + w(U + n_C)`, with C and n_C from `PRICED_NATIVE` and the pinned
    `K_0 / GF(2^37)` calibration row. U is `cost.unpriced` without `NON_OPERATION_COUNTERS`, plus table and
    sibling work behind their markers.
  - Pairing is by (session, round, target), with workloads as clusters.
  - Point ratios agree within 9.7e-15 (relative), and the log-scale linearised-t ends within 4.4e-15, using
    the validator's own Student-t quantile.
  - The validator's own cluster bootstrap moves interval ends by at most 0.032 and changes no conclusion.
  - Band S agrees to 4 decimals:

    | arm | S at w = 0 / 0.1 / 1 |
    |:--|:--|
    | ls | 0.1962 / 0.2637 / 0.8718 |
    | sw | 0.1421 / 0.2131 / 0.8521 |
    | rho-v5 | 0.1578 / 0.1739 / 0.3188 |
    | rho-strong | 0.2752 / 0.3119 / 0.6423 |

- **Rule.** 12 of 12 upper ends < 1 (max 0.98142, r3, w = 1).
- **Replay.**
  - `verify --replay-all` on the three validator copies: 3 × 1,152 records, all verified, 3 × 864 replays
    `identical`, 0 problems, all checks true. The replay lists equal the committed `audits/`.
  - A nonce-seeded sample (`--replay 24 --replay-seed 13370101238878578612`, the first 64 bits of the
    nonce) on each session: 72 / 72 identical.
- **Cross-binary replay.** The pinned binary (`7b182360…fd315`) replays the validated
  `koblitz_lean_sib_confirm_20261118` session n37-r1 identically: 864 / 864, 0 problems. That session was
  recorded by binary `43f44fec…` (code `33dccd579d`) with records `7f8fe3a8…dc4`. So `217638524d` leaves
  the incumbent `ls`, its off-state and both rho references unchanged.
- **Digests.**
  - The three `records.jsonl` files (gunzipped from the scratchpad) hash to the values in `summary.json`,
    `session.json` and the audit `files` maps:

    | session | id | records sha256 |
    |:--|:--|:--|
    | r1 | `ECBS1h2823ec84f7fb` | `b775c10e…3169` |
    | r2 | `ECBS1hea22025e233f` | `139abac6…df6f` |
    | r3 | `ECBS1h317535f8bbca` | `e593e46d…6461` |

  - Every session records binary `7b182360…fd315` = the pinned file = `logs/binary.sha256`.
  - `tables/records-n37.csv.gz` equals the raw records on 2,592 / 2,592 measured rows.
  - The benchmark record's 3 `raw[]` files match their sha256.
- **README figures.** The per-solve means recompute exactly:

  | quantity | ls | sw |
  |:--|:--|:--|
  | relations | 1717.26 | 921.60 |
  | verify | 38.74 | 12.84 |
  | sibling pairs | — | 809.65 |
  | lifts | — | 0.50 |
  | `total_gae` | 3276.1 | 2454.6 |
  | solve wall (µs) | 1144.9 | 935.9 |
  | relations wall (µs) | 571.1 | 355.1 |
  | verify wall (µs) | 7.7 | 3.05 |

## 6. Controls

- **ls-aa:** identical to `ls` on 576 / 576 keys in every non-telemetry field.
- **ls-off (flags written as 0):** differs only in parameter echoes on 576 / 576:
  - `method.{params,method_id,method_sha256}`;
  - `fbm_id`;
  - `factor_base.{params.orbit_key_reuse,fb_id,fb_sha256}`;
  - `detail.label`.

  Cost, phases (wall time excluded), counters, outcome, seed and `points_sha256` are identical. Both
  controls read 1.000 [1.000, 1.000] at every weight.
- **Answers:** 3,456 / 3,456 solves `verified` with `matches_target`. Independent `[d]G = Q` holds on all
  144 targets (section 2).
- **Cold, single target:**
  - Each record is one public target (`hash_to_subgroup_v1`, `planted = null`).
  - Every IC record charges its own factor base, pair table, relations, linear algebra and verify.
  - rho-v5 charges its set-up (76 GAE) per target, and rho-strong its 32-scalar-multiplication set-up
    (1,333 GAE) per target.
  - No unpriced operation counter lacks a count on any arm.

## 7. Where the gain comes from (per solve, priced as the band prices it)

| phase | w = 0 | w = 0.1 | w = 1 | lever |
|:--|--:|--:|--:|:--|
| factor base | 0 | −28.8 | −288.0 | orbit_key_reuse (288 field squarings at w) |
| oracle set-up | 0 | 0 | 0 | unchanged |
| relations | −795.65 | −714.59 | **+15.00** | walk_sib (saves adds; pays 1 second point + lift at w) |
| linear algebra | 0 | 0 | 0 | unchanged |
| verify | −25.90 | −25.90 | −25.90 | ladder_verify |
| total (ls → sw) | 2979.0 → 2157.4 | 4005.0 → 3235.7 | 13239.3 → 12940.4 | |

The README's "Total GAE at w = 0: 3276 -> 2455" is the record's calibrated `total_gae`. The band's w = 0 price
is 2979.0 → 2157.4. The ratio is the same up to calibration.

## 8. Classification, gate, limitations

- **Classification.**
  - `sw` is a T0 engineering gain of the validated incumbent `ls` on K1 a = 0 n = 37. It belongs to the
    generic-decomposition track (pair-table oracle; AGENTS.md "Tracks of index calculus"). It remains a
    Frobenius- and negation-folded BSGS under the IC harness, so its asymptotic ceiling is unchanged.
  - The boundary, unit, targets, their order, the table and the pricing are the same. Every removed
    operation is executed work.
  - It is not a relabelling: the priced total falls at every weight, and the total solve wall time falls
    (median 952 to 809 µs, L0, descriptive).
  - Its size is weight-dependent: about 28% / 19% at w = 0 / 0.1. At w = 1 it is 2.3% under the band, but
    about 0.08% with field squarings at any calibrated weight (section 3).
  - Under the ratchet, `sw` is the new n37 incumbent when the Coordinator records it.
- **Gap to rho.**
  - rho-v5 is the strongest measured reference (same session and binary). `sw` is 0.900 [0.825, 0.983] at
    w = 0 but 1.225 and 2.673 at w = 0.1 and 1. The band does not order them ("unresolved"; it was
    `rho-v5 < ls` before).
  - Against rho-strong, also unresolved.
  - rho-v5 leaves its jump-index selection untallied, which makes it slightly optimistic for rho at w > 0.
    That does not change "unresolved".
  - Nothing here is below rho.
- **Gate (docs/VALIDATION_GATE.md).** Not satisfiable yet.
  - The raw sessions exist only in the producer scratchpad. The benchmark record's `raw[]` lists derived
    files only (`uploaded: false`). The committed CSV carries only `total_gae` and `S`, so no band headline
    can be recomputed from in-repository data.
  - A `VALIDATION.json` needs four things: the raw sessions archived, a replay certificate, this report,
    and a rho-reference descriptor (`negation_frobenius`, rho-v5, `order_holds_across_band: false`).
- **Limitations.**
  - Single curve; local L0 shared host, `--cpus none --allow-busy`; no Iso Lab, EKS or S3, as declared.
    Wall times are descriptive.
  - No pcc preregistration. The registration was not pushed before the data. This is the second n37
    attempt in the thread, with no Holm correction (immaterial at these margins).
  - The §5a calibrated reading for `min_key_field_squares_uncharged` was not reported by the producer. The
    validator's derived reading holds for any w_cal ≥ 0, but no w_cal was measured.
  - The ecbench binary was not rebuilt by this validator: about 1.3 GB of disk was free during
    validation, and the producer's build took 26.5 min. Its binding to `217638524d` rests on:
    - the hash pin and the 102 lever strings in the binary;
    - unchanged source;
    - the cross-binary replay;
    - the validator's independent checks of every walk's first hit, every sibling abscissa along the
      walks, every verify count and every answer.

    The producer's build log does not name the commit, and the build worktree is deleted.
  - The `orbit_key_reuse` refusal path was not exercised by a negative control. It rests on code reading.
  - Minor README inaccuracies:
    - "neutral at w = 1": the walk is +15 GAE per solve there;
    - "Total GAE at w = 0" is the calibrated total;
    - "Not pushed": it was pushed 30 s after the results commit.
  - PROTOCOL §0 does not quote the parent pilot's c8-m8 n37 reading that was visible before the freeze.

## validation_report

```yaml
validation_report:
  id: VAL-20261118-24d204
  task_id: null  # dispatched by the producing session; no TASK card was named
  snapshot: e97ce22df9cd5f4c24e63aa6e22dd53365ebaa81
  registration_commit: 624775314b5ece8dcf4927024c96a11f2a0adbb7
  code_commit: 217638524db18d2c59c61f019a529ae10fb63dce
  run_ids:
    - ECBS1h2823ec84f7fb  # n37-r1 records b775c10e509811aeddfb739662abfeb55b5658f0d6fc39c59b87c63be8803169
    - ECBS1hea22025e233f  # n37-r2 records 139abac63df88f9d363c436a0ac1bf5695ba2a7ed297aea7950c1b282a63df6f
    - ECBS1h317535f8bbca  # n37-r3 records e593e46dae5d459bdcc4e197f44f48ccb20026903acc96efd2a62023c3546461
    - koblitz_lean_sib_confirm_20261118 n37-r1 (cross-binary replay), records 7f8fe3a8e30557bb41d15f01c75e8c3a42cfe31b71e9e2544418315e84860dc4
  artifact_checks:
    - three records.jsonl sha256 = summary.json = session.json = audit files maps
    - binary 7b182360f531492214010854acb92ff1218a1f7fe29f5ac03cc8c3ac221fd315 = run.sh pin = logs/binary.sha256 = every session; 102 lever strings
    - REGISTRATION.sha256 10/10 at 624775314b and e97ce22df9; session spec.json = registered specs 3/3
    - src, Cargo.toml, Cargo.lock unchanged 217638524d..e97ce22df9; analyze/summarize = parent's frozen scripts up to session dir and curve list; model.py = parent prof/model.py
    - order: registration 19:29:26Z < first session 19:29:41.5Z < last 19:30:20.1Z < results 19:33:26Z < push 19:33:56Z (registration not pushed before data, declared)
    - seeds 20261118491-493 absent from the tree before registration; no earlier session with these specs or seeds in the scratchpad (the parent pilot's c8-m* n37 arms, seed 473, predate the freeze)
    - records-n37.csv.gz = raw on 2592/2592 measured rows; benchmark record raw[] 3/3 sha256 match (derived files only, uploaded false)
  metric_recomputations:
    - ecbench compare (33 calls) and band (4 calls) rerun bit-identical to results.json.gz and tables/ except session paths; summarize.py reproduces summary.json byte for byte
    - independent Python band/ratio: points within 9.7e-15 rel, log-scale linearised-t within 4.4e-15, own bootstrap within 0.032, band S to 4 decimals
    - primary sw/ls pooled 0.72422 [0.70523,0.74455] / 0.80792 [0.79386,0.82275] / 0.97742 [0.97589,0.97893]; 12/12 upper ends < 1, max 0.98142 (r3, w=1)
    - gaps sw/rho-v5 0.90042 [0.82523,0.98325] / 1.22541 / 2.67316 (unresolved); sw/rho-strong 0.51625 / 0.68314 / 1.32664 (unresolved); ls/rho-v5 1.24329 / 1.51675 / 2.73490 (rho-v5 < ls)
    - derived (not measured) section 5a calibrated reading, min_key_field_squares at 0 / 0.013 / 0.13: w=1 0.99916 / 0.99887 / 0.99628, all upper ends < 1 (max 0.99936)
    - m-table of PROTOCOL section 0 reproduced within 0.001 with the validator's own count model on the parent's n37 ls records
  control_checks:
    - ls-aa = ls on 576/576 keys (telemetry only); ls-off = ls on 576/576 except parameter echoes; both 1.000 [1,1]
    - 3456/3456 solves verified with matches_target
    - sw vs ls on 576/576: same targets_tried, log, seed, target, points_sha256; oracle set-up and LA identical; walk counts = validator's model; verify = popcount-1 adds, 0 doublings (ls popcount-1, bitlen-1); 288 key squarings removed, nothing else in the base changed
    - verify --replay-all 3/3 sessions, 2592/2592 identical, 0 problems; nonce-seeded --replay 24 x 3 identical; cross-binary replay of lean_sib n37-r1 864/864 identical
    - independent GF(2^37) (x^37+x^6+x^4+x+1) arithmetic, all 144 targets - Q on curve, [r]Q=O, [d]G=Q by double-and-add, Montgomery ladder and own [2^i]G ladder sum; 2326 table classes = pair_table_entries; first-hit k+1 = targets_tried for every arm and round; sibling formula = full additions on 116590 walk pairs; x-only probe = full probe
  heuristic_validation_checks:
    - frozen model expectation ~0.74/0.82/0.98 recorded before data (PROTOCOL section 4); measured 0.724/0.808/0.977; for deterministic counts on an unchanged walk this agreement is expected, not evidence of peeking
    - null objects A/A and default-off exactly 1; no statistical-signal claim
    - no extrapolation beyond n37; asymptotic ceiling unchanged
  cost_model_checks:
    - unit ecbench.gae, S = GAE/sqrt(r); band w = 0, 0.1, 1 with table and sibling work priced behind markers; no missing unpriced count on any arm
    - sibling_second_points priced at w in every number (without it w=1 would read about 0.916)
    - per solve, w = 0 / 0.1 / 1 - walk -795.65 / -714.59 / +15.00; ladder -25.90 at every w; key reuse 0 / -28.8 / -288.0
    - w=1 gain is 96% field squarings priced at 1 GAE; section 5a calibrated reading not reported by the producer; derived reading keeps the order for any w_cal >= 0 (w=1 about 0.999)
    - ladder_verify lowers priced cost at every weight and verify wall time (7.7 to 3.05 us, 429/432 pairs); not a relabel, unlike tau_verify (VAL-20261118-43658a)
    - orbit_key_reuse removes executed squarings but its wall effect is below L0 resolution (base phase +3.0 us paired median, same order as the +5.8 us position effect on the identical oracle set-up)
    - cold single target, public targets, every phase charged per record on IC and both rho arms; memory unchanged (median max RSS 20,036 / 20,072 KiB)
  proof_architecture_checks: []
  verdict: passed
  verdict_detail:
    primary sw/ls K1 n37: passed (all weights, pooled and each replicate)
    gap statements: passed (unresolved against rho-v5 and rho-strong; not below rho)
    ladder_verify: real saving, fully counted, not a relabel
    orbit_key_reuse: real but tiny; carries the w=1 magnitude under band pricing
    sibling walk: sound and complete; gain at w=0 and 0.1, +15 GAE per solve at w=1
  classification: T0 engineering gain of the validated incumbent ls on K1 a=0 n=37, generic-decomposition track; not below rho; not a relabelling
  limitations:
    - raw sessions only in the producer scratchpad; benchmark record raw[] lists derived files only; gate not satisfiable until archived
    - no pcc preregistration; registration not pushed before data; second n37 attempt in the thread without Holm correction (immaterial at these margins)
    - section 5a calibrated reading for the field-arithmetic counter not reported; validator's derived reading uses no measured w_cal
    - ecbench binary not rebuilt by this validator (disk); binding rests on hash pin, strings, unchanged source, cross-binary replay and independent checks; build log does not name the commit
    - orbit_key_reuse refusal path not exercised by a negative control (code reading only)
    - L0 shared local host; wall times descriptive
    - README - walk called neutral at w=1 (it is +15 GAE per solve), "Total GAE at w=0" is the calibrated total, "Not pushed" is stale; PROTOCOL section 0 omits the parent pilot's c8-m8 n37 reading seen before the freeze
    - rho-v5 jump-index selection untallied (optimistic for rho at w > 0)
  artifact_paths:
    - research/koblitz_n37_sibwalk_k8_20261118/PROTOCOL.md
    - research/koblitz_n37_sibwalk_k8_20261118/README.md
    - research/koblitz_n37_sibwalk_k8_20261118/summary.json
    - research/koblitz_n37_sibwalk_k8_20261118/results.json.gz
    - research/koblitz_n37_sibwalk_k8_20261118/tables/
    - research/koblitz_n37_sibwalk_k8_20261118/audits/
    - benchmarks/records/20261118T1929Z-koblitz-n37-sibwalk-k8.json
    - scratchpad sw8-sessions/n37-r{1,2,3}/records.jsonl.gz (raw; not in git)
  review_attestation:
    joints_owned: [all checks named by the dispatching task, (1) to (5)]
    sources_read:
      - research/koblitz_n37_sibwalk_k8_20261118/ (all committed files except REPORT.pdf and figures, whose presence only was checked)
      - research/koblitz_k1_sibwalk_20261118/{README.md,analyze.py,summarize.py,prof/model.py,pilot/SELECTION_RULE.md,pilot/selection.json}
      - research/koblitz_lean_sib_confirm_20261118/VALIDATOR_REPORT.md
      - research/koblitz_n29_orbit_levers_20261118/VALIDATOR_REPORT.md (git show)
      - src/cryptanalysis/{ic_boundary.rs,ic_framework/plugins.rs,ic_framework/mod.rs,ic_framework/stages.rs,koblitz_fast.rs,ecbench/band.rs,ecbench/compare.rs,ecbench/methods.rs,ic_framework/solver_units.rs} (relevant ranges)
      - docs/ic/calibration.json; AGENTS.md (success criterion, tracks, section 5a)
      - benchmarks/records/20261118T1929Z-koblitz-n37-sibwalk-k8.json
      - scratchpad raw sessions sw8-sessions/n37-r*, sw-sessions/n37-r* (ls records only), lsc-sessions/n37-r1
    read_sibling_reports: false  # research/koblitz_k1_sibwalk_20261118/VALIDATOR_REPORT.md (concurrent validator) was not opened
    verdict: passed
```
