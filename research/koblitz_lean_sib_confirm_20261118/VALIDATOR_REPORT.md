# Validator report VAL-20261118-9a42f2: lean base + sibling pair table (tau off) on the K1 incumbents n23, n29, n37

This report independently validates `research/koblitz_lean_sib_confirm_20261118/` on branch
`claude/f4-f5-algorithm-speed-bvp7qd`. The committed snapshot is `1e0964673b` (results). It rests on
`2d67198576` (registration) and `33dccd579d` (code). The validator did not design, write or run any
part of the study and did not edit any producer file. The study directory left the sparse working
tree during validation (the producer session committed `58f5ddddc3`, which touches only
`research/KOBLITZ_K1_SPEEDUP_LOG.md`), so the snapshot was read from `git archive 1e0964673b`.

Sources read:

- the frozen protocol, README, `run.sh`, `analyze.py`, `summarize.py` (both versions), `make_specs.py`;
- the code paths of the two levers at `33dccd579d` (`ic_framework/plugins.rs`, `ic_boundary.rs`,
  `ic_framework/mod.rs`), and `ecbench/band.rs`, `ecbench/compare.rs`, `ecbench/methods.rs`;
- the nine raw sessions named by the README, through byte-identical validator copies;
- the A1 n23 and n37 sessions of `koblitz_known_log_columns_20261118`, for a cross-binary replay;
- the prior report VAL-20261118-43658a, for format and its n29 findings.

Validator nonce: `b7d9d00510fb40ab5ce6e819384cf8d1`, drawn 2026-11-18T17:10:31Z. That is after the
results commit (16:58:58Z) and its push (16:59:32Z).

## Verdicts

"Passed" means the receipt is admissible evidence for the claim as stated, on that curve only. The
claim is a T0-scope engineering gain of the validated incumbent arm. It is not an ECDLP claim and not
a claim below rho. It changes no exponent. It does not authorize a leaderboard or
`benchmarks/README.md` row by itself (section 8).

| curve | claim: ls / incumbent, pooled, w = 0 / 0.1 / 1 | recomputed | frozen rule (upper 95% < 1, pooled and each replicate, every weight) | verdict |
|:--|:--|:--|:--|:--|
| **K1 a=1 n23** (ls / sp-c4-d5) | 0.770 / 0.753 / 0.720 | 0.76958 [0.74986, 0.78980] / 0.75299 [0.73419, 0.77226] / 0.71972 [0.70368, 0.73613] | 12 of 12 upper ends < 1; largest 0.8196 (r1, w = 0) | **passed** |
| **K1 a=1 n29** (ls / gen-c1) | 0.814 / 0.639 / 0.432 | 0.81384 [0.80143, 0.82583] / 0.63892 [0.62182, 0.65582] / 0.43182 [0.41748, 0.44635] | 12 of 12; largest 0.8421 (r3, w = 0) | **passed** |
| **K1 a=0 n37** (ls / sp-c8-d5) | 0.703 / 0.732 / 0.796 | 0.70324 [0.68560, 0.72051] / 0.73183 [0.71719, 0.74621] / 0.79616 [0.78721, 0.80500] | 12 of 12; largest 0.8165 (r3, w = 1) | **passed** |

The intervals are the frozen rule's, the wider of ecbench's linearised-t and cluster-bootstrap
95% ends.

| gap statement (pooled) | claim | recomputed | verdict |
|:--|:--|:--|:--|
| n23 ls / rho-v5 | 1.370 / 1.631 / 2.762; v5 ahead at every weight | identical; band rank `rho-v5 < ls` | passed |
| n29 ls / rho-v5 | 0.875 [0.775, 0.969] / 1.048 [0.931, 1.157] / 1.878; unresolved | identical; band rank `unresolved inside the band` | passed: **not below rho** |
| n37 ls / rho-v5 | 1.236 / 1.517 / 2.771; v5 ahead | identical; `rho-v5 < ls` | passed |
| ls / rho-strong | n23 0.341 / 0.419 / 0.823 (below); n29 0.070 / 0.090 / 0.229 (below); n37 0.667 / 0.795 / 1.289 (unresolved) | identical | passed |
| before: incumbent / rho-v5 and / rho-strong | README table | identical (every point and interval) | passed |

**Overall: passed on n23, n29 and n37**, with the readings and limitations below. The three
readings that matter most:

- At w = 0 the whole gain is `sib` (field inversions saved in the pair table). At w = 1 the whole
  gain is the lean base: `sib` alone is 0.999 there, as at n29. At w = 0.1 both contribute (section 2).
- The removal of `P + (−P)` is accounting (one charged, nearly free addition per orbit: 4 / 1 / 8
  GAE per solve). It moves the ratio by at most 0.0125 (n29, w = 0: 0.814 to 0.826) and changes no
  verdict.
- The w = 1 magnitudes price each removed hash probe, insert, negation or curve check at one group
  addition. By wall time a removed operation costs about 0.05–0.2 GAE-equivalent on this host. The
  measured wall-time ratio, ls / incumbent total wall 0.79–0.93 per replicate (L0, descriptive), is
  the better guide to the real-time size.

## 1. Code paths: both levers apply soundly to the multi-column spread-seed arms

### `lean_layout=1` with `generator_multiples = columns` (`plugins.rs`, compact-orbit-scan `build`)

- **Parameter check.** `lean_layout()` refuses the lever unless `generator_orbit=1` and
  `generator_multiples == columns`, and refuses values other than 0 and 1. Both specs satisfy this:
  n23 has `columns=4,generator_multiples=4`; n37 has `columns=8,generator_multiples=8`. Both use
  `multiple_doublings=5`.
- **Seeds.** The seed loop builds `[2^(5(j−1))]G` with counted doublings before the lean branch is
  reached, so `multiple_doublings` and the lever do not interact. The seed's own on-curve check is
  kept.
- **Repeated columns are still refused.** `if lean && multiples == 1` is the only branch that skips
  the orbit key. With more than one column, the min-over-conjugates key is computed, and a repeated
  orbit fails with `the orbit of [j]G repeats an earlier column`.
  - The records agree: `min_key_field_squares_uncharged` stays 88 (n23) and 288 (n37) in ls.
  - Independently, the validator recomputed the 4 and 8 orbit keys with its own arithmetic. They
    are distinct.
- **What lean skips.** It skips the per-image on-curve check and the hash index
  (`from_signed_layout`, `neg_index[i] = i ^ 1`).
  - The builder pushes each orbit point as the pair `(σ^t(S), −σ^t(S))` for every column, so `i ^ 1`
    is the negation map for multi-column bases too.
  - `from_signed_layout` checks the lengths, the even count and the column range.
- **Nothing that needs the index reads it.**
  - The known-log column values come from `known_column_logs`, from the parameters, not from
    `point_index`.
  - `run_pipeline_inner` (`mod.rs:579`) and the Frobenius MITM `prepare` refuse an index-elided base
    for any oracle except `mitm-frobenius-layout-counted`.
  - That oracle's layout table names members by `layout_member` index arithmetic.
- **Measured removal per solve.** Every record shows the same removal, incumbent → ls:

  | curve | curve-equation checks | hash probes | factor-point inserts | point negations | removed native ops |
  |:--|:--|:--|:--|:--|--:|
  | n23 | 96 → 4 | 552 → 0 | 184 → 0 | 276 → 92 | 1,012 |
  | n37 | 304 → 8 | 1,776 → 0 | 592 → 0 | 888 → 296 | 3,256 |
  | n29 | 30 → 1 | 174 → 0 | 58 → 0 | 87 → 29 | 347 (incl. 28 → 0 min-key squarings) |

  The bitop-only `index_sort_word_ops` (736 / 174 / 2,960) also goes. The Frobenius maps, base size
  (184 / 58 / 592 signed points), `points_sha256`, columns and coefficients are unchanged.
- **Soundness.** The skipped image check is the one VAL-20261118-43658a found unneeded: the Frobenius
  maps a K1 curve to itself. A defect would also fail the final `[d]G = Q`, which the validator
  re-checked independently (section 5).

### `sib=1` across orbits (`ic_boundary.rs`, `FrobeniusPairTable::build_layout_with`)

- **Which pairs the build forms.** For each orbit `o`, the build iterates the representative's own
  rotations `rep..rep+n`, then every later orbit `rep+2n..f`. The sibling condition (`j` even, next
  index `j+1`, finite points, `p.x != x.x`) has no single-column assumption, so cross-orbit
  `(X, −X)` pairs are formed as siblings.
  - Sibling pairs = orbits·(n−1)/2 + n·orbits·(orbits−1)/2 = 182 (n23), 14 (n29), 1,180 (n37).
    These equal the recorded `sibling_second_points` on every ls record.
  - Pair-table additions fall by pairs + orbits: 372 → 186, 30 → 15, 2,376 → 1,188.
- **Formula check.** The validator rebuilt the three bases with its own GF(2^n) arithmetic.
  - Field polynomials, each the unique trinomial or pentanomial that puts `G` on the curve with
    `[r]G = O`: n23 `x^23+x^5+1`, n29 `x^29+x^2+1`, n37 `x^37+x^6+x^4+x+1`.
  - Each of the 182 / 14 / 1,180 sibling pairs was computed twice: once by the producer's formula
    (`λ₋ = λ₊ + x_X/(x_P+x_X)`, `x = λ²+λ+x_P+x_X+a`) and once by two full additions. There were
    **0 mismatches**, including n37 with a = 0, which the n29 check did not cover.
  - No non-representative pair took the equal-x fallback.
- **The same table.** On 576 of 576 keys per curve (warm-ups included), these are identical between
  ls and the incumbent:
  - the pair-table native map (canonicalisations, entries, representatives);
  - `table_insert_attempts`;
  - `targets_tried`, all counters, and the relations, linear-algebra and verify phases.
- **Pricing in the band.**
  - Each ls record carries `work_native.sibling_second_points` behind
    `counted:sibling_second_points`.
  - `band::table_work_counts` adds the `SIBLING_WORK_COUNTERS` to `U` behind their marker, under the
    default pricing.
  - The validator's own band includes them and reproduces ecbench to 1e-14. Leaving them out would
    give, for example, n37 w = 1 0.723 instead of 0.796. **The sibling points are priced at w in
    every reported number.**
- **Nothing newly uncounted.** Every native counter that differs between ls and the incumbent
  decreases, except `sibling_second_points` (added and priced). Code reading finds only index
  arithmetic newly executed (`i ^ 1`; at n29 one `seen.insert(seed.x)` replacing the incumbent's
  `seen.insert(key)`).
  - A sibling pair executes I + 2M + 2S, charged as one addition (I + 2M + S) plus one sibling point
    at w. The extra multiplication is unpriced at w = 0 (slightly optimistic) and over-priced at
    w = 1 (conservative).

## 2. Where the gain comes from (validator decomposition from the records)

The ratios are pooled ls / incumbent. "sib only" removes the pair-table additions and adds the
sibling points at w. "lean only" removes the factor-base unpriced operations at w.

| curve | w | ls / inc | sib only | lean only | with `P + (−P)` charge restored |
|:--|:--|--:|--:|--:|--:|
| n23 | 0 / 0.1 / 1 | 0.770 / 0.753 / 0.720 | 0.770 / 0.846 / 0.999 | 1 / 0.907 / 0.721 | 0.775 / 0.757 / 0.721 |
| n29 | 0 / 0.1 / 1 | 0.814 / 0.639 / 0.432 | 0.814 / 0.898 / 0.998 | 1 / 0.741 / 0.434 | 0.826 / 0.646 / 0.434 |
| n37 | 0 / 0.1 / 1 | 0.703 / 0.732 / 0.796 | 0.703 / 0.794 / 1.000 | 1 / 0.937 / 0.797 | 0.705 / 0.733 / 0.797 |

Wall times are descriptive (L0, shared host, medians in µs, incumbent → ls).

- **n23:** factor base 50.5 → 31.5, pair table 118.5 → 93.6, solve 291.8 → 247.8.
- **n29:** factor base 35.0 → 23.1, pair table 64.0 → 61.2, solve 154.9 → 141.5.
- **n37:** factor base 91.2 → 47.1, pair table 634.2 → 406.8, solve 1,167 → 905.

The README's means (342 → 292, 172 → 149, 1,337 → 1,061) reproduce. The A/A wall median ratios are
0.98–1.01. Both levers therefore remove executed work.

The lean base removes about 14–34 ns per removed native operation. The README's sentence on n37 is
correct: the 1,180 sibling points priced at w = 1 take back the pair-table saving, so the ratio rises
from 0.70 to 0.80.

## 3. Protocol before data, the smoke check, and the summarize.py fix

- **Order.**
  - Registration `2d67198576` was committed at 16:52:53Z. The benchmark record was created at
    16:52:57Z.
  - The first session (n23-r1) started at 16:53:01.9Z, and the last (n29-r3) finished at
    16:54:48.7Z. The order matches the frozen interleave.
  - The results were committed at 16:58:58Z.
  - All nine session `spec.json` files are byte-identical to the registered `specs/*.json`.
  - All 14 registered hashes check at `2d67198576`. At `1e0964673b` all check except
    `summarize.py`, which is disclosed.
  - `src/`, `crates/`, `Cargo.toml`, `Cargo.lock` and `docs/ic/calibration.json` are identical
    between `33dccd579d`, `1e0964673b` and `58f5ddddc3`.
  - The nine target seeds 20261118431–439 occur in no file at `5f512cbf8f` (`git grep`).
- **Caveat.** The registration was not pushed before the data: the remote-tracking ref moved from
  `5f512cbf8f` straight to `1e0964673b` at 16:59:32Z. There is also no pcc preregistration. The order
  rests on local commit and reflog times.
- **Smoke check.** It is disclosed in PROTOCOL §0 (inside the registration commit) and in the session
  log: the candidate arm, seeds 999023/999037, 2 targets, status only, deleted.
  - It cannot be verified, because it was deleted. Searching the scratchpad and `/tmp` found no
    remnant and no session with `lean_layout`/`sib=` other than the n29 study's and these nine.
  - It could not have informed a frozen choice. The arms come from validated incumbents and levers.
    The seeds differ. The metric is a deterministic count with an unchanged walk.
- **The `summarize.py` fix is logic-neutral.**
  - The diff is the single statement `n = n[1:]`.
  - The registered version, run on the committed `results.json.gz`, stops with `KeyError: '23'`
    before writing anything.
  - The fixed version reproduces the committed `summary.json` byte for byte.
  - Apart from that statement the two files are identical.
- **Prediction.** The frozen model expectation at w = 0 was ~0.79 / ~0.83 / ~0.71; the measured
  values are 0.770 / 0.814 / 0.703. For deterministic counts with an unchanged walk this agreement is
  expected.

## 4. Recomputation (point estimates, intervals, per-replicate rule)

- **ecbench rerun.** The validator reran all 99 `ecbench compare --weight {0,0.1,1} --resamples 4000
  --seed 20261002` calls of `analyze.py` with the pinned binary on its session copies:
  - 8 pairs × 3 weights pooled per curve;
  - the primary per replicate;
  - pooling as `--dir d1 d2 d3`, and identically with `--dir` repeated.

  Every field is identical to the committed `results.json.gz` except the session paths. The pooled
  band files are identical too. The per-replicate `tables/band-*.json` equal `results.json`.
- **Independent Python.** It shares no code with the producer and computes:
  - `priced = G − C + w(U + n_C)`, with `cost.unpriced` looked up as the band does;
  - `NON_OPERATION_COUNTERS` excluded;
  - table and sibling work behind markers;
  - C and n_C from `PRICED_NATIVE` and the pinned K_1/GF(2^23), K_1/GF(2^29) and K_0/GF(2^37) rows
    of `docs/ic/calibration.json`.

  It pairs by (session, workload, round). Workload ids are distinct across sessions, so pooled pairs
  stay inside a session. Results:
  - every point estimate within 9e-15 (relative);
  - every linearised-t interval within 6e-15, using the validator's own Student-t quantile;
  - every band S within 1e-14.

  The validator's own one-stage cluster bootstrap (different resampler) moves interval ends by at
  most 0.088 (gen-c1 / rho-v5, w = 1). It changes no conclusion and no rank.
- **Rule.** Pooled and in each replicate, at each weight, the upper end is below 1 on all three
  curves: 36 of 36 intervals. The smallest margin is n29 r3 w = 0 (0.842). The rule is far from any
  multiplicity concern.

## 5. Replay, digests, answers

- **Digests.** The nine `records.jsonl` (gunzipped from the scratchpad) hash to the values in
  `summary.json`, `session.json` and the audit `files` maps. Session ids:
  - n23: `ECBS1h5c854bfbdf99`, `ECBS1hea4c323b6cd6`, `ECBS1heaa62f2662d1`;
  - n29: `ECBS1h61314027c2df`, `ECBS1h5a3ff67ba07f`, `ECBS1hcaa51d68ee61`;
  - n37: `ECBS1h628553e4500f`, `ECBS1h750abe4da2c8`, `ECBS1h20bd9ed7f141`.

  Each session records binary `43f44fec…3e65`, which matches the pinned file and
  `logs/binary.sha256`. The 20 files that the benchmark record hashes match their sha256.
  `tables/records-n{23,29,37}.csv.gz` equal the raw records on all 2,592 measured rows per curve
  (0 mismatches).
- **Replay.** `verify --replay-all` was run on all nine validator copies, under the CPU lock: 10,368
  records, 10,368 verified, 7,776 of 7,776 replays `identical`, 0 problems, all checks true. The
  replay lists equal the committed audits.
- **Cross-binary replay.** The pinned binary replays the A1 n23 and n37 sessions of the known-log
  study identically: 1,152 of 1,152 each, 0 problems. Those sessions were recorded by binary
  `58cfff90…` with records `5f869ba3…` and `488f758b…`, as validated in VAL-20261118-4ae594. So the
  incumbents `sp-c4-d5` and `sp-c8-d5`, and both rho references, are unchanged by `33dccd579d`.
- **Answers.**
  - With its own arithmetic (polynomials in section 1), the validator checked all 432 targets
    (144 per curve; all 10,368 records map to them consistently): Q is on the curve, `[r]Q = O`,
    `0 ≤ d < r` and `[d]G = Q`. There were 0 failures.
  - It also checked `[r]G = O` and `h·r = #E`, and that the Frobenius image of G has order dividing
    #E.
  - On 16 targets per curve drawn by the nonce (sha256 order), `[d]G = Q` was re-checked by a
    Montgomery ladder, a second route. There were 0 mismatches.
  - Every record has `status = verified` and `matches_target = true`.
- **Cold, single target.**
  - Each record is one public target (`hash_to_subgroup_v1`, `planted = null`).
  - Every IC record charges its own factor base, pair table, relations, linear algebra and verify.
  - rho-v5 charges its set-up (42.9 / 22.6 / 76.0 GAE) per target, and rho-strong its
    32-scalar-multiplication set-up (1,056 / 738 / 1,333 GAE) per target.
  - No unpriced operation counter lacks a count on any arm.
- **Memory.** Median max RSS is about 27.2 MiB for both arms. Pair-table entries are unchanged
  (2,326 at n37); lean drops only the base's hash maps.

## 6. Controls

- **Default-off.**
  - `INC-off` (`lean_layout=0`, `sib=0,tau_verify=0`) equals the incumbent on 576 of 576 keys per
    curve, warm-ups included. Every field was compared except volatile telemetry and parameter
    echoes.
  - The differing fields are exactly the echoes: `method.*`, `fbm_id`,
    `factor_base.{params.lean_layout, fb_id, fb_sha256}` and `detail.label`.
  - The ratio is 1.000 [1.000, 1.000] at every weight.
- **A/A.** `INC-aa` equals the incumbent on 576 of 576 keys with no echo stripping: only wall-time
  telemetry differs. The ratio is 1.000 [1.000, 1.000].
- **Same walk.** Over 576 of 576 keys per curve, ls has the same:
  - `algorithm_seed`, `targets_tried` and all counters;
  - relations, linear-algebra and verify phases (so `tau_verify` is off);
  - pair-table native map and insert attempts;
  - `points_sha256` and recovered `d`.
- **Null object.** The A/A and default-off arms are the null objects. Both are exactly 1. The metric
  is a deterministic count, not a statistical signal.

## 7. Committed files over 64 KiB and the benchmark record

The benchmark record `benchmarks/records/20261118T1652Z-koblitz-lean-sib-confirm.json` hashes 20
files. All 20 are in git, all match, and all are `uploaded: false`. Five exceed 64 KiB:

| file | bytes | policy (AGENTS.md §13, `docs/artifacts.md`) |
|:--|--:|:--|
| `results.json.gz` | 101,117 | **data-suffixed (`.gz`) and over 64 KiB: belongs in the artifact store with an `.artifacts.json` pointer, not in git** (`artifacts.py check` flags it; the PR's artifact-autofix would move it) |
| `REPORT.pdf` | 231,269 | allowed: `policy.keep` holds `*.pdf` |
| `figures/fig2-ratios.svg` | 237,749 | allowed: not data-suffixed, under 1 MiB |
| `figures/fig1-phases-band.svg` | 150,176 | allowed |
| `summary.json` | 91,925 | allowed: `.json` is not data-suffixed, under 1 MiB |

None reaches the 10 MiB `large-files` gate. The prior two studies committed their `results.json.gz`
the same way (97,893, 211,054 and 226,250 B).

The substantive problem is what `raw[]` lists: derived products only. The actual raw data, nine
`records.jsonl` (about 6.4 MB gzipped), is named only by sha256 in `summary.json`. It lives only in
the producer scratchpad, outside git and the store. The record therefore does not bind the raw data,
and `uploaded: false` cannot be fixed by uploading the listed files.

- `benchmarks/progress/` was not regenerated, because `benchrec.py report` refuses the malformed,
  unrelated record `20261117T0334Z-hec-multi-production-cold.json`.
- For the `close` call the producer moved that record aside and restored it. The working-tree file
  equals the HEAD blob (sha256 `95485dfd…`). This is disclosed in the session log.

## 8. Classification, gate, limitations

- **Classification.** ls is a T0 engineering gain of the validated incumbent on each of K1 n23, n29
  and n37.
  - The boundary, unit, targets, oracle, walk, table and pricing are the same.
  - The removed work is executed work: field inversions (sib), and hash-map operations, negations and
    curve checks (lean).
  - It is not a relabelling. Total priced cost falls at every weight, and wall time falls.
  - One component is accounting: the `P + (−P)` charge, ≤ 0.0125 of the ratio.
  - The asymptotic ceiling (Frobenius- and negation-folded BSGS-type) is unchanged.
  - Under the ratchet, ls is the new incumbent on each curve when the Coordinator records it.
- **Gap to rho.**
  - rho-v5, the strongest measured reference in the same session and binary, stays ahead on n23 and
    n37 at every weight. The gap shrinks by about 20–30%.
  - n29 is unresolved inside the band. Nothing here is below rho.
  - rho-v5 leaves its jump-index selection untallied, which makes it slightly optimistic for rho at
    w > 0. That does not change any reading.
- **Gate (docs/VALIDATION_GATE.md).** Not satisfiable yet, for three reasons:
  - The raw sessions are not in the repository or the store. The committed CSVs carry only
    `total_gae` and `S`, so no band headline can be recomputed from in-repository data.
  - A `VALIDATION.json` needs the raw sessions archived, a replay certificate, this report, and a
    rho-reference descriptor (`negation_frobenius`, rho-v5; `order_holds_across_band` true for n23
    and n37 with rho ahead, false for n29).
  - `results.json.gz` should move to the store.
- **Limitations.**
  - Local L0 shared host, `--cpus none --allow-busy`; no Iso Lab, EKS or S3, as declared.
  - No pcc preregistration, and the freeze was not pushed before the data.
  - The ecbench binary was not rebuilt by this validator (disk; the producer's build took about
    46 min). Its binding to `33dccd579d` rests on:
    - the hash pin and the lever strings in the binary;
    - unchanged source;
    - VAL-20261118-43658a's lib-test rebuild;
    - the cross-binary replay;
    - the validator's independent checks of answers and sibling abscissae.
  - The unit test's end-to-end part uses one column only. Multi-column equivalence rests on its
    two-column table assertions and on this study's 576 identical walks per curve.
  - The bitop/v1 secondary unit marks ls records `known_uncounted: [base_index_builds]`, although
    the lean base builds no index (the marker is simply absent). This is a conservative label in a
    unit the claim does not use.
  - The smoke check is unverifiable (deleted).

## validation_report

```yaml
validation_report:
  id: VAL-20261118-9a42f2
  task_id: null  # dispatched by the producing session; no TASK card was named
  snapshot: 1e0964673b96170728ac0c20d73acfb7310be817
  registration_commit: 2d671985762e722be172c0d21ee63ab4c328502c
  code_commit: 33dccd579d0e7e2e93835ceb519b86885be24c19
  run_ids:
    - ECBS1h5c854bfbdf99  # n23-r1 records 40059e42006e6d317195d5354ef0ce0aaf366292fa9885790ea35f0be309edf2
    - ECBS1hea4c323b6cd6  # n23-r2 records 37c14dbfbb6001743ed0428a44533217371d8472a35f3434b092884edec86e0e
    - ECBS1heaa62f2662d1  # n23-r3 records d51e03ed77e2da3b0aeccd1f6ede4689a4699dda9b377196b47cad8bf11ab03a
    - ECBS1h61314027c2df  # n29-r1 records f5abf0d7f2ce59226e833434bbe11ec77079d071d8b2d061f3a7fdbf30191cbe
    - ECBS1h5a3ff67ba07f  # n29-r2 records 50df904b486e922bdd303e49c1a0431969dfe1c7d99a2d930e94b9cf3b862e30
    - ECBS1hcaa51d68ee61  # n29-r3 records cf2113f51a13ef65e95a3358e65c84b98664203c0c28cf33c91bf890feda2cf2
    - ECBS1h628553e4500f  # n37-r1 records 7f8fe3a8e30557bb41d15f01c75e8c3a42cfe31b71e9e2544418315e84860dc4
    - ECBS1h750abe4da2c8  # n37-r2 records 16b0221c4eb7cf1235ee254842332dfe659ae60ccea0764f52ec6ea6fb9df064
    - ECBS1h20bd9ed7f141  # n37-r3 records 8b1b369954e1eee6f3502bc8ba583735adea24298035a97dc43d607fc7d04dc1
    - A1 n23 / n37 known-log sessions (cross-binary replay), records 5f869ba3ae71f1724d160e30ce4701b6ad80613cb2cbd8b5657a7c19770eef65 / 488f758b116846957737e85b92865ea536ae28ecc872269dca76801e4a016d1e
  artifact_checks:
    - nine records.jsonl sha256 = summary.json = session.json = audit files maps
    - binary 43f44fec44b902f22753c2b44fd10d0567ad327cf73c9298c3558de3ff5c3e65 = run.sh pin = logs/binary.sha256 = every session and record
    - REGISTRATION.sha256 14/14 at 2d67198576; 13/14 at 1e0964673b (summarize.py, disclosed); session spec.json = registered specs (9/9)
    - src, crates, Cargo.*, docs/ic/calibration.json identical 33dccd579d..58f5ddddc3
    - order: registration 16:52:53Z < first session 16:53:01.9Z < last 16:54:48.7Z < results 16:58:58Z (local; first push 16:59:32Z)
    - benchmark record raw[] 20/20 sha256 match; records CSVs = raw on 3 x 2592 measured rows
    - seeds 20261118431-439 absent from the tree before registration
  metric_recomputations:
    - ecbench compare rerun (99 calls) bit-identical to results.json.gz except session paths; band files identical
    - independent Python band/ratio: points within 9e-15 rel, linearised CI within 6e-15, band S within 1e-14; own bootstrap ends within 0.088, no conclusion changed
    - primaries pooled n23 0.76958/0.75299/0.71972, n29 0.81384/0.63892/0.43182, n37 0.70324/0.73183/0.79616; 36/36 upper ends < 1 (max 0.8421)
    - gaps ls/rho-v5 n23 1.370/1.631/2.762, n29 0.875/1.048/1.878 (unresolved), n37 1.236/1.517/2.771; ls/rho-strong n23 below, n29 below, n37 unresolved
    - summarize.py fix logic-neutral (registered version raises KeyError; fixed version reproduces summary.json byte for byte)
  control_checks:
    - default-off = incumbent on 576/576 keys per curve except parameter echoes; ratio 1.000 [1,1]
    - A/A = incumbent on 576/576 keys (telemetry only); ratio 1.000 [1,1]
    - ls same walk (seed, targets_tried, counters, relations/LA/verify phases, pair-table natives, insert attempts, points_sha256, d) on 576/576 per curve
    - verify --replay-all 9/9 sessions, 7776/7776 identical, 0 problems; cross-binary replay of A1 n23/n37 2304/2304 identical
    - "[d]G = Q, Q on curve, [r]Q = O on all 432 targets (own arithmetic); nonce b7d9d00510fb40ab5ce6e819384cf8d1 sample 48 re-checked by Montgomery ladder"
    - sibling formula = two full additions on 182 / 14 / 1180 pairs (n37 a=0 included), 0 mismatches
  heuristic_validation_checks:
    - frozen w=0 model ~0.79/~0.83/~0.71 recorded before data; measured 0.770/0.814/0.703
    - null objects A/A and default-off exactly 1; deterministic counts, no statistical-signal claim
    - no extrapolation beyond the three measured curves; asymptotic ceiling unchanged
  cost_model_checks:
    - unit ecbench.gae, S = GAE/sqrt(r); band w = 0, 0.1, 1 with table and sibling work priced behind markers; no missing unpriced count on any arm
    - sibling_second_points priced at w in every reported number (without it n37 w=1 would read 0.723)
    - gain is sib at w=0 and lean at w=1; P+(-P) removal is accounting (4/1/8 GAE per solve, <= 0.0125 of the ratio)
    - lean removes 1012/347/3256 executed native ops per solve; wall time corroborates; about 0.05-0.2 GAE-equivalent per op by wall, so w=1 magnitudes overstate real time
    - cold single target, public targets, every phase charged per record on IC and both rho arms; memory unchanged (~27.2 MiB median RSS)
  proof_architecture_checks: []
  verdict:
    n23 ls/sp-c4-d5: passed
    n29 ls/gen-c1: passed
    n37 ls/sp-c8-d5: passed
    gap statements: passed (v5 ahead on n23/n37; n29 unresolved in the band; not below rho)
    overall: passed
  classification: T0 engineering gain of the validated incumbent on each curve; not below rho; not a relabelling (P+(-P) is a small accounting component)
  limitations:
    - raw sessions only in the producer scratchpad; benchmark record raw[] lists derived files only; gate not satisfiable until archived
    - results.json.gz (101,117 B, .gz) committed against the 64 KiB data-file policy (AGENTS.md section 13)
    - benchmarks/progress not regenerated (unrelated malformed record blocks benchrec report)
    - no pcc preregistration; registration not pushed before the data
    - ecbench binary not rebuilt by this validator; binding rests on hash pin, strings, unchanged source, prior lib-test rebuild, cross-binary replay and independent checks
    - L0 shared local host; wall times descriptive; smoke check unverifiable (deleted)
    - unit-test end-to-end covers one column; multi-column equivalence rests on the session data
    - rho-v5 jump-index selection untallied (optimistic for rho at w > 0)
  artifact_paths:
    - research/koblitz_lean_sib_confirm_20261118/PROTOCOL.md
    - research/koblitz_lean_sib_confirm_20261118/README.md
    - research/koblitz_lean_sib_confirm_20261118/summary.json
    - research/koblitz_lean_sib_confirm_20261118/results.json.gz
    - research/koblitz_lean_sib_confirm_20261118/tables/
    - research/koblitz_lean_sib_confirm_20261118/audits/
    - benchmarks/records/20261118T1652Z-koblitz-lean-sib-confirm.json
    - (scratchpad, not durable) lsc-sessions/n{23,29,37}-r{1,2,3}/records.jsonl.gz; bins/ecbench-n29lv-33dccd579d; validator work in val-lsc/
  review_attestation:
    joints_owned: [whole receipt, as tasked]
    read_sibling_reports: false  # VAL-20261118-43658a and VAL-20261118-4ae594 are prior studies' reports, read for format, the n29 findings and the A1 digests
    sources_read:
      - research/koblitz_lean_sib_confirm_20261118/{PROTOCOL.md,README.md,REGISTRATION.sha256,registered-at.txt,run.sh,analyze.py,summarize.py (2d67198576 and 1e0964673b),make_specs.py,summary.json,results.json.gz,tables/*,audits/*,logs/*}
      - src/cryptanalysis/ic_framework/plugins.rs (compact-orbit-scan build and parameters; FrobeniusMitmOracle prepare)
      - src/cryptanalysis/ic_boundary.rs (from_signed_layout, index_elided, FrobeniusPairTable::build_layout_with, layout_member, index_of_key)
      - src/cryptanalysis/ic_framework/mod.rs (known-log handling, index-elided refusal, unit test generator_orbit_lean_sib_tau_levers_keep_the_solve_and_remove_work)
      - src/cryptanalysis/ecbench/band.rs, compare.rs, methods.rs (PRICED_NATIVE); ic_framework/solver_units.rs (PINNABLE_NATIVE); docs/ic/calibration.json
      - AGENTS.md (Success criterion, section 3 classes, section 5a, Validation gate, section 13); docs/VALIDATION_GATE.md; docs/benchmark-records.md; docs/artifacts.md
      - crypto-autoresearcher docs/agent-runtime-core.md, agents/validator.md
      - benchmarks/records/20261118T1652Z-koblitz-lean-sib-confirm.json; docs/session-logs/2026-11-18.md (1e0964673b diff)
      - research/koblitz_n29_orbit_levers_20261118/VALIDATOR_REPORT.md; research/koblitz_known_log_columns_20261118/{a1/summary.json,summary.json}
      - scratchpad copies of lsc-sessions/n{23,29,37}-r{1,2,3} and klc-sessions-a1/{n23,n37}
```
