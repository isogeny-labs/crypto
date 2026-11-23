# Validator report VAL-20261118-43658a: generator-orbit IC levers at K1 n29

This report independently validates `research/koblitz_n29_orbit_levers_20261118/` on branch
`claude/f4-f5-algorithm-speed-bvp7qd`. The committed snapshot is `6ba49f66db` (results). It
rests on `0ae90a4812` (registration) and `33dccd579d` (code). The validator did not design, write
or run any part of the study, and did not edit any producer file.

Sources read:

- the frozen protocol and README;
- the diff of `33dccd579d`;
- the band and compare source (`ecbench/band.rs`, `ecbench/methods.rs::PRICED_NATIVE`);
- `docs/ic/calibration.json`;
- the raw session named by the README, via a byte-identical copy;
- the A0 n29 session of `koblitz_known_log_columns_20261118`, used for the cross-binary replay.

Validator nonce: `b363e6ac3d1deca45290c0759496ee2b`, drawn 2026-11-18T16:06:05Z. That is after
the results commit (15:53:09Z) and after its push (15:53:49Z).

## Verdicts

"Passed" means the receipt is admissible evidence for the claim as stated, at K1 a=1 n=29 only.
That claim is a T0-scope engineering gain of the validated generator-orbit IC arm `gen-c1`. It is
not an ECDLP claim and not a claim below rho. It does not change any exponent, and it does not
authorize a leaderboard or `benchmarks/README.md` row by itself (see section 7).

| item | claim | recomputed (w = 0 / 0.1 / 1, 95% CI) | verdict |
|:--|:--|:--|:--|
| **P1: lv-all / gen-c1** | 0.713 [0.679, 0.749] / 0.614 [0.580, 0.649] / 0.487 [0.462, 0.514] | identical (ecbench rerun: bit-identical; independent Python: point estimates within 1.4e-15) | **passed**: improvement at all three weights |
| `lean_layout=1` (lv-lean / gen-c1) | 1.000 / 0.762 [0.741, 0.783] / 0.459 [0.432, 0.487] | identical | **passed**. Performed work is really removed, and the change is sound. It is a tie at w = 0 by construction, so it is an improvement at w = 0.1 and 1 only. |
| `sib=1` (lv-sib / gen-c1) | 0.834 [0.815, 0.854] / 0.907 [0.898, 0.915] / 0.998 [0.998, 0.999] | identical | **passed** at w = 0 and 0.1 (14 field inversions removed per solve). The w = 1 residual 0.998 is entirely the removed charge for `P + (−P)`, which is accounting. Read it as neutral at w = 1. |
| `tau_verify=1` (lv-tau / gen-c1) | 0.879 [0.863, 0.895] / 0.945 [0.939, 0.952] / 1.030 [1.028, 1.032] | identical | **passed as a sound, fully counted check. Failed as a stand-alone improvement:** the order changes inside the band (crossover near w = 0.36), it is worse at w = 1, and its verify phase is about 1.6x slower in wall time. The README reports these facts; the lever must not be booked as an improvement on its own. |
| gap: lv-all / rho-v5 | 0.877 [0.736, 1.046] / 1.116 [0.956, 1.304] / 2.262 [2.002, 2.555]; unresolved | identical; band order flips (lv-all below at w = 0, above at 0.1 and 1) | **passed** (unresolved inside the band, as stated) |
| gap: lv-all / rho-strong | 0.070 / 0.094 / 0.273, below at all weights | identical | passed (rho-strong is the leaderboard's reference, not the strongest measured rho) |
| gap before: gen-c1 / rho-v5, gen-c1 / rho-strong | 1.231 / 1.819 / 4.644; 0.098 / 0.153 / 0.560 | identical | passed |

**Overall: passed**, for lv-all (P1) and for the gap statements, with the readings and
limitations below. Two readings matter most:

- The w = 0 headline 0.713 contains the tau-adic check's −11 GAE per solve. That part is not a
  cost reduction once its recoding is priced as it executes on this host (section 3).
- lean + sib without tau (a derived, unmeasured counterfactual) would read 0.834 / 0.668 / 0.457.
  lv-lean alone already beats lv-all at w = 1, with band S 1.426 against 1.515.

## 1. `lean_layout=1`: real work removed, nothing still needed is skipped

What is skipped. The skipped work was read in the diff (`plugins.rs`, `ic_boundary.rs`) and then
checked on every record. Factor-base unpriced native counts fall from 377 to 30 per solve, on all
192 records:

| counter | before | after |
|:--|--:|--:|
| `hash_probes_uncharged` | 174 | 0 |
| `factor_point_inserts_uncharged` | 58 | 0 |
| `point_negations_uncharged` | 87 | 29 (the orbit loop's own negations stay) |
| `curve_equation_checks_uncharged` | 30 | 1 |
| `min_key_field_squares_uncharged` | 28 | 0 |

The 29 calibrated Frobenius maps, `points_sha256`, columns, coefficients, `abscissae` = 29,
`signed_points` = 58 and `factor_base_vec_bytes_floor_uncharged` = 2784 are all unchanged.

- **Is the index still needed anywhere?** No.
  - `FactorBase::from_signed_layout` sets `neg_index[i] = i ^ 1`. The builder pushes each orbit
    as `(τ^t(G), −τ^t(G))` (plugins.rs, the generator loop), so this is exactly the negation map
    `from_column_map` derived.
  - The only consumer, `FrobeniusPairTable::build_layout_with` / `probe` (layout branch), names
    points by `layout_member` arithmetic and `neg_index`. It never reads `point_index`,
    `x_index` or `abscissa_list`. The validator grepped every reader of those three fields; the
    readers are the other oracles, the non-layout table builds and `archive.rs`.
  - An index-less base is refused in two places. The Frobenius MITM oracle's `prepare` refuses it
    unless in layout mode (`fb.index_elided() && !self.layout_members`), and
    `run_pipeline_inner` refuses it for any other oracle name. The unit test exercises the
    refusal.
  - Nothing falls back silently. Every lv-lean and lv-all record has the same `targets_tried`,
    the same relations-phase counts and the same linear-algebra counts as gen-c1 (192/192), and
    recovers the same `d`.
- **Is skipping the per-image on-curve check sound?** Yes.
  - The incumbent's generator path records no reason for the check. It copies the scan path's
    defensive check of the Frobenius implementation.
  - The images are `frobenius_k(member, 1) = (x², y²)`. On `y² + xy = x³ + ax² + 1` with `a, b`
    in `F_2`, the Frobenius maps the curve to itself.
  - The seed `G` is still checked (the 1 remaining equation check). The orbit-closure check
    `σ^n(G) = G` is kept.
  - A defect in the map would make the final `[d]G = Q` fail. For the double-and-add arms that
    check does not use the base. For the tau arms a defective orbit would pass only by
    coincidence (probability about 1/r). A defect would also fail the harness's own
    `matches_target` audit and the validator's independent check of all 1728 answers
    (section 5).
  - No soundness is lost in any reported result.
- **Is the single column's min key needed?** No. It only feeds `seen` to detect a repeated column.
  With `generator_multiples = columns = 1` there is no earlier column and no scan
  (`seen.len() == columns` breaks the raw-x loop at once). With `generator_multiples > 1` the key
  is still computed.
- **Is anything still executed but uncounted?** Only index arithmetic of the same class the
  incumbent also leaves uncounted:
  - `i ^ 1` for 58 entries;
  - one `seen.insert(seed.x)`, which replaces the incumbent's uncounted `seen.insert(key)`.
- **Real reduction or relabel?** A real reduction.
  - At w = 0 it is unchanged by construction.
  - At w > 0, 347 performed operations go per solve. The factor-base phase wall time falls from
    37.6 to 26.4 µs (median; lv-all 25.6). That is about 32 ns per removed operation, roughly
    0.2 GAE at this host's about 155 ns per group operation (descriptive, L0).
  - The removed hash-map operations, negations, curve checks and squarings were executed by the
    incumbent binary. That is shown by the cross-binary replay in section 5 and by the wall time.

## 2. `sib=1`: the convention, the band, and `P + (−P)`

- **Formula.** On `y² + xy = x³ + ax² + b`, `−X = (x_X, x_X + y_X)`. So
  `λ₋ = λ₊ + x_X/(x_P + x_X)`, and both abscissae come from one inversion:
  `x = λ² + λ + x_P + x_X + a`. The validator recomputed `x(G ± τ^t G)` for all 14 sibling pairs
  with its own GF(2^29) arithmetic (field polynomial `x^29 + x^2 + 1`), once by the sibling
  formula and once by two full additions. There were 0 mismatches.
  - Fallbacks: an equal abscissa, the identity, or an unpaired last index all take the full
    addition (`p.x != x.x`, `js.peek() == Some(&(j + 1))`).
- **Same table.** The canonicalisations (29), table inserts (29) and `pair_table_entries` (26)
  are identical on every record. The unit test probes 3,000 walk points against both tables.
  Every lv-sib and lv-all record has the same trials and relation as gen-c1.
- **Convention.** It is the one `KSumTable::build_negation_folded_full` (`sib` on k-sum builds)
  and `wwalk` use: `build_ops.adds += 1` plus `sib_second_points += 1` per pair. The counter is
  emitted as `work_native.sibling_second_points` behind `counted:sibling_second_points`.
- **Priced in the band.** `band::table_work_counts` reads `SIBLING_WORK_COUNTERS`
  (`sibling_second_points`, `sibling_lift_products`) behind their marker. `priced_counts_with`
  adds them to `U` under the default pricing. The validator's independent band recompute
  includes the 14 sibling points per solve and reproduces `ecbench compare` to 1.4e-15.
  - Charging one unit at w is conservative here. The binary second point costs one
    multiplication and one squaring against `I + 2M + S` for an addition.
- **`P + (−P)`.** It is correctly excluded: `points[rep + 1] = −points[rep]` by the layout, and
  the incumbent inserted no key for it (`is_identity → continue`). But the incumbent charged it
  as one group addition (`BinaryGroup::add` counts `adds += 1`, and `FastCurve::add` returns `O`
  after an x/y comparison, with no field arithmetic). Its removal therefore takes 1 GAE per solve
  off the count while removing almost no work.
  - At w = 1 this is the whole of lv-sib's 0.998: with that charge restored, lv-sib / gen-c1 is
    1.0000.
  - For lv-all, restoring the charge gives 0.724 / 0.620 / 0.489 instead of 0.713 / 0.614 /
    0.487. P1 is unaffected.
  - The real saving of `sib` is 14 field inversions per solve. That is too small to show in the
    set-up phase's wall time (60.0 against 61.4 µs median), which canonicalisation dominates.

## 3. `tau_verify=1`: a complete check, fully counted, but not a band-robust saving

- **Complete and λ-free.**
  - `tau_orbit_mul` sums `±τ^{i mod n}(G)` over the τ-adic NAF of the integer `d`. The orbit is
    `fb.points[2t]` and `fb.points[2t + 1]`; `prepare` checks that column 0 starts at `G`.
  - The result is compared with the target by `check == target`. `FastPoint` derives
    `PartialEq` on `(x, y, infinity)`, so this is a full point comparison.
  - The argument uses only `τ² − μτ + 2 = 0` and `τ^n = 1` on `E(F_{2^n})`. No eigenvalue, log or
    group order is used, so a wrong `λ` still fails.
  - `μ = 1` for a = 1. The trace-1 digit strings reproduce every recorded count, which shows the
    code used `μ = 1`.
- **Correct for every `d` in range.** With independent code (Python, own field arithmetic), the
  validator checked every d in [0, r − 1] (all 42,457 values, `d = 0` included):
  - the digits evaluate to `(d, 0)` in `Z[τ]`;
  - no two non-zero digits are adjacent;
  - `Σ u_i τ^{i mod 29}(G) = [d]G` on the curve, with 0 mismatches. Lengths run from 1 to 32
    digits.
  - A literal transcription of the producer's `tau_naf`, with Rust's truncating `i128` division
    and `rem_euclid`, gives digit strings identical to the validator's on 524,300 inputs: both
    `μ`, `d < 2^18`, and `u64::MAX`, `2^63`, `r`, `r − 1`, `#E`.
  - `τ^29(G) = G` and `τ²(G) − τ(G) + 2G = O` were checked directly.
- **Every operation counted.** τ-NAFs are unique, so the counts do not depend on the
  implementation. On all 384 lv-tau and lv-all records:
  - verify adds = (non-zero digits − 1), with 0 doublings;
  - `tau_naf_recode_steps_uncharged` = number of digits;
  - verify GAE = adds;
  - there were 0 mismatches.

  On all 960 double-and-add IC records (gen-c1, -aa, -off, lv-lean, lv-sib): adds = popcount(d) − 1
  and doublings = bitlen(d) − 1, with 0 mismatches. The recode counter is in `cost.unpriced`, so
  the band prices it at w.
- **Is the native pricing reasonable?** The band is honest about it. The README's word
  "conservatively" is not supported on this host.
  - The verify phase takes 3.15 µs (gen-c1, 20.3 group operations) against 5.02 µs (lv-all, 9.35
    additions and 30.25 recode steps), medians.
  - That puts the implemented recoding at about 118-125 ns per step, about 0.8 GAE-equivalent
    (descriptive, L0).
  - At realistic per-step prices, then, the lever is about neutral to negative. Its band order
    against gen-c1 crosses near w = 0.36, and it is 1.030 at w = 1.
  - By AGENTS.md "What counts", a lever whose headline count falls while its priced total (w = 1)
    and wall time rise is a relabelling on its own.
  - Inside lv-all the band order still holds at every weight, because lean dominates at w = 1.

## 4. Protocol before data, and the off-state deviation

- **Order.**
  - Code `33dccd579d` was committed at 15:01:33Z and registration `0ae90a4812` at 15:01:41Z.
    The HEAD reflog has the same times.
  - The binary file is from 15:48:59Z. The session ran 15:49:13–15:49:25Z (`session.json`). The
    results were committed at 15:53:09Z.
  - The five registered files hash to `REGISTRATION.sha256` and are unchanged at `6ba49f66db`.
  - The session's `spec.json` is byte-identical to the registered `specs/n29.json` (`16227bfb…`).
  - `src/`, `crates/`, `Cargo.toml` and `Cargo.lock` are identical between `33dccd579d` and
    `6ba49f66db`.
  - The shared-target lib-test binary is from 14:59:55Z, built from sources last modified before
    14:21Z. That fits the protocol's disclosure that the unit test ran before the freeze.
  - Among today's session specs in the scratchpad, only this session contains `lean_layout`,
    `sib=` or `tau_verify`. No undisclosed candidate-arm session was found.
  - **Caveat:** the registration was not pushed before the data. The remote-tracking ref moved
    from `c0ebd66001` (13:15:44Z) straight to `6ba49f66db` (15:53:49Z). There is also no `pcc`
    preregistration claim. The order therefore rests on the producer's local commit and reflog
    times, not on an externally witnessed freeze. This is a limitation for any tier claim
    (`docs/ic/SUCCESS_CRITERIA.md` asks for a pcc preregistration).
- **Prediction.** The frozen model prediction was ~0.71 / ~0.61 / ~0.49. It was derived from the
  disclosed A0 profile and code reading, and the observed values match it. For deterministic
  operation counts with an unchanged walk, this agreement is expected. It is not evidence of
  data peeking.
- **The off-state identity correction is legitimate.**
  - The frozen `analyze.py` strip includes the whole `factor_base` object. That object echoes the
    arm's parameters (`params.lean_layout`) and its FB1 configuration digests (`fb_id`,
    `fb_sha256`, a hash of curve, family, parameters and points). Both differ by construction
    when `lean_layout=0` is written out.
  - The validator diffed every field of all 192 gen-c1-off records (warm-ups included) against
    gen-c1, excluding only volatile telemetry (`time`, `online`, `isolation`, `allocator`,
    `wall_ns`, record and run ids). The only differing fields are parameter echoes:
    - `method.params.{factor_base,oracle}`, `method.method_id`, `method.method_sha256`;
    - `fbm_id`;
    - `factor_base.{params.lean_layout,fb_id,fb_sha256}`;
    - `detail.label`.
  - The cost, every phase's counts and native and work_native maps, the counters, the outcome,
    `algorithm_seed` and `points_sha256` are identical on all 192 records.
  - `ecbench compare` of gen-c1-off against gen-c1 gives 1.000 [1.000, 1.000] at all three
    weights.
  - Minor: the README names `detail.label` as one of the fields that tripped the frozen check.
    `detail` is not in the frozen strip; `fb_sha256` is. This has no effect.

## 5. Recomputation, replay, digests, answers

- **Digests.**
  - The raw `records.jsonl` (scratchpad, gunzipped) hashes to `d9b45b5d…6351d`. That matches
    `summary.json`, `session.json` and the replay receipt.
  - The binary `ecbench-n29lv-33dccd579d` hashes to `43f44fec…3e65`, matching
    `logs/binary.sha256` and `session.json`.
  - `results.json.gz`, `summary.json`, `tables/*`, `audits/*`, `REPORT.pdf` and `figures/*.svg`
    match the sha256 in `benchmarks/records/20261118T1512Z-koblitz-n29-orbit-levers.json`.
  - `tables/records-n29.csv.gz` equals the raw records on all 1296 measured rows (0
    mismatches).
- **Headlines.**
  - `ecbench compare --weight {0,0.1,1} --resamples 4000 --seed 20261002` was rerun with the
    pinned binary for the 10 pairs in the table above. Every point estimate and CI end is
    bit-identical to `summary.json`.
  - Independent Python, sharing no code with the producer, reproduces each record's
    `priced_gae(w) = G − C + w(U + n_C)`, the per-arm band S and every paired point estimate
    (max relative difference 1.4e-15). It uses `cost.unpriced` lookup, `NON_OPERATION_COUNTERS`
    excluded, table and sibling work behind markers, and the pinned K_1/GF(2^29) calibration.
  - Its own target-cluster bootstrap (different RNG) moves CI ends by at most 0.029 (gen-c1 /
    rho-v5 at w = 1) and changes no conclusion.
  - Band S at w = 0 / 0.1 / 1:

    | arm | w = 0 | w = 0.1 | w = 1 |
    |:--|--:|--:|--:|
    | gen-c1 | 0.4393 | 0.7064 | 3.1104 |
    | lv-all | 0.3132 | 0.4334 | 1.5150 |
    | rho-v5 | 0.3570 | 0.3883 | 0.6698 |
    | rho-strong | 4.5054 | 4.6106 | 5.5574 |
- **Multiplicity.** There is one primary here. Over this thread's five primaries (the four
  known-log tests and this P1), Bonferroni gives 99% intervals for lv-all / gen-c1 of
  [0.665, 0.754] / [0.568, 0.656] / [0.455, 0.521], all below 1. For lv-all / rho-v5 they are
  [0.700, 1.094] / [0.918, 1.361] / [1.958, 2.649], still unresolved.
- **Replay.**
  - `verify --replay-all` on a copy of the session: 1728 records, 1728 verified, 1296 of 1296
    replays reproduced, 0 problems.
  - Cross-binary: the pinned binary replays the A0 n29 session of the known-log study (records
    `154318…`, as validated in VAL-20261118-4ae594) on 864 of 864 runs identically, with 0
    problems. The incumbent and both rho references are therefore unchanged.
- **Answers.**
  - The curve is K1 over `F_2[x]/(x^29 + x^2 + 1)`. This is the only trinomial or pentanomial
    that puts `G` and the first target on the curve; `[r]G = O`, and `r · h = #E`.
  - For all 1728 records, the validator checked with its own arithmetic that `[d]G = Q`, that
    `Q` is on the curve and that `[r]Q = O`. There were 0 mismatches.
  - On the 32 records drawn by the nonce (seqs 107, 122, 148, 154, 193, 205, 380, 409, 421, 501,
    657, 682, 760, 768, 773, 835, 881, 885, 950, 979, 980, 1075, 1095, 1153, 1221, 1237, 1264,
    1294, 1312, 1316, 1326, 1535), it re-checked `[d]G = Q` by a second scalar-multiplication
    route (a ladder). There were 0 mismatches.
  - All 1728 records have `status = verified` and `matches_target = true`.
- **Same walk.** lv-all, lv-lean, lv-sib and lv-tau match gen-c1 on all 192 (round, target)
  pairs, warm-ups included. They have the same `targets_tried` (mean 40.19), the same relations
  phase (wall excluded) and the same recovered `d`.
- **Cold, single target.**
  - Each record is one public target (`hash_to_subgroup_v1`, seed 20261118404, 48 targets × 3
    rounds plus 1 warm-up).
  - Each IC record charges its own factor base, pair-table set-up, relations, linear algebra and
    verify. rho-v5 charges its set-up (9 adds and 13.65 doublings) per target, and rho-strong
    its 32-scalar-multiplication set-up (about 738 GAE) per target.
  - No unpriced counter lacks a count on any arm.
- **Memory.** Median max RSS is 21.2 MiB (gen-c1) and 21.0 MiB (lv-all). The pair table has 26
  entries in both. Memory is not changed by the levers.

## 6. Unit test `generator_orbit_lean_sib_tau_levers_keep_the_solve_and_remove_work`

The validator read the test in `33dccd579d` (ic_framework/mod.rs). It asserts:

- the τ-NAF's value in `Z[τ]` and its non-adjacency for both `μ` (d < 2000 and four large
  values);
- the lean base equals the indexed base in points, columns, coefficients, `neg_index` and
  layout, with the expected counters and refusal of bad parameter sets;
- the sibling table matches the plain table in entries, canonicalisations, insert attempts and
  3,000 probes, with `adds_on + siblings + orbits = adds_off`;
- `tau_orbit_mul = [d]G` on about 1000 scalars per curve, at a lower GAE;
- end to end on n23, n29 and n37 with 2 seeds each: the incumbent equals `lean_layout=0,sib=0,
  tau_verify=0` (decomposition and total GAE, wall excluded); the levers on recover the planted
  log with the same `targets_tried` and a lower total; the lean base is refused by
  `mitm-frobenius-counted`.

The test asserts properties, not ratios. Its assertions match what the session shows.

- Prebuilt lib-test binary in the shared `target/` (`crypto_lib-b4fa3a3eb9ba3f87`, built
  14:59:55Z, fresh for the committed source): **1 passed** in 0.52 s.
- Rebuilt by the validator from the committed source (`cargo test --release --locked --lib
  --config 'profile.release.package.crypto.opt-level=2'`, which forces a new artifact for the
  crate only; `CARGO_BUILD_JOBS=1`, under the CPU lock): **1 passed** in 0.53 s. The build took
  37 min 20 s and produced `crypto_lib-cf1f48aedc699661` (sha256 `e44dc70b…7031`). The validator
  deleted that artifact afterwards, and its dependencies were reused unchanged.

The ecbench binary itself was not rebuilt. The producer's build took 46.5 minutes, and only 4.1 GB
of disk was free. Its binding to `33dccd579d` therefore rests on:

- the producer's build log and hash;
- the rebuilt lib test above;
- this validator's code-independent checks of every answer and every verify-phase count.

## 7. Classification, gate, limitations

- **Classification.** lv-all is a T0 engineering gain of the named, validated generator-orbit
  IC arm at K1 n29.
  - That arm is a Frobenius- and negation-folded BSGS under the IC harness. Its asymptotic
    ceiling of about `2√(r/2n)` against `√(πr/4n)` for folded rho is unchanged.
  - The gain is not relabelling as a whole. The boundary, unit, targets, walk, table and
    pricing are the same, and the removed work is executed work.
  - Two parts of the count are accounting or relabelling, and should be booked so: the `sib`
    `P + (−P)` charge (about 1 GAE per solve), and `tau_verify` on its own (section 3).
  - The wall time (descriptive, L0) agrees in direction for lv-all: median solve 155.2 against
    162.7 µs. That is about 5%, mostly the lean base.
- **Gap to rho.** Against rho-v5, the strongest measured reference (same session, same binary),
  lv-all is unresolved inside the band.
  - v5 leaves its jump-index selection untallied, which makes v5 slightly optimistic for rho at
    w > 0. That does not change "unresolved".
  - Nothing here is below rho.
- **Gate (docs/VALIDATION_GATE.md).** Not satisfiable yet.
  - The raw session (`records.jsonl`, 1728 records) exists only in the producer scratchpad. It is
    not in git and not in the store (`uploaded: false` in the benchmark record). The committed
    CSV has only `total_gae` and `S`, so no band headline can be recomputed from in-repository
    data.
  - A `VALIDATION.json` needs four things: the raw session archived (store by sha256 or a git
    archive), a replay certificate, this report as the validator report, and a rho-reference
    descriptor (`negation_frobenius`, rho-v5, `order_holds_across_band: false` for lv-all
    against v5).
- **Limitations.**
  - Single curve (n29), local L0 shared host, `--cpus none --allow-busy`, no Iso Lab, EKS or S3,
    as declared. The metric is a deterministic count (every replay identical); wall times are
    descriptive.
  - No pcc preregistration, and the freeze was not pushed before the run (section 4).
  - The ecbench binary was not rebuilt by the validator (section 6).
  - The README cites `figures/*.png`, but only the SVGs are committed.
  - lean + sib without tau was not an arm. The 0.834 / 0.668 / 0.457 above is derived from
    phase counts, not measured.

## validation_report

```yaml
validation_report:
  id: VAL-20261118-43658a
  task_id: null  # dispatched by the producing session; no TASK card was named
  snapshot: 6ba49f66db4381c2cc7e710d843cd00a8515ca27
  code_commit: 33dccd579d0e7e2e93835ceb519b86885be24c19
  registration_commit: 0ae90a4812327d9585f89a924f7363c8ea9f20d9
  run_ids:
    - ECBS1h034591a60e85  # n29 levers session, records d9b45b5ddac78276e9ed14c654fca0fc94c5598a756abc844da1c715dc96351d
    - ECBS1hc77953b5e57b  # A0 n29 known-log session (cross-binary replay), records 154318391f4df4f0779f0bbd4259a4de89b069c85a414ce009e05ae03f1ec2ab
  artifact_checks:
    - records.jsonl sha256 = summary.json = session.json = replay receipt
    - binary sha256 43f44fec44b902f22753c2b44fd10d0567ad327cf73c9298c3558de3ff5c3e65 = logs/binary.sha256 = session.json
    - REGISTRATION.sha256 = registered blobs, unchanged at 6ba49f66db; session spec.json = specs/n29.json (16227bfb...)
    - src/, crates/, Cargo.toml, Cargo.lock identical between 33dccd579d and 6ba49f66db
    - committed results/summary/tables/audits/report/figure digests = benchmark record; records-n29.csv.gz = raw (0/1296 mismatches)
    - order: code 15:01:33Z < registration 15:01:41Z < session 15:49:13Z < results 15:53:09Z (local commits; first push 15:53:49Z)
  metric_recomputations:
    - P1 lv-all/gen-c1 w=0/0.1/1: 0.713003 [0.679142,0.748553] / 0.613528 [0.580090,0.648894] / 0.487079 [0.461944,0.513580]
    - lv-lean, lv-sib, lv-tau / gen-c1; lv-all and gen-c1 / rho-v5 and rho-strong (section-0 table)
    - method: ecbench compare rerun (bit-identical to summary.json) and independent Python band and ratio (max rel diff 1.4e-15)
    - Bonferroni(5) 99% for P1 upper ends 0.754 / 0.656 / 0.521
  control_checks:
    - default-off gen-c1-off = gen-c1 on every non-echo field, 192/192 records; ratio 1.000 [1,1] at all weights
    - A/A gen-c1-aa = gen-c1 (only volatile telemetry differs); ratio 1.000 [1,1]
    - same targets_tried, relations phase and recovered d for every candidate on 192/192 (round, target)
    - every record verified; [d]G = Q, Q on curve, [r]Q = O recomputed with independent arithmetic on 1728/1728
    - nonce b363e6ac3d1deca45290c0759496ee2b sample of 32 re-verified by a second scalar-multiplication route
    - verify --replay-all 1296/1296 identical; cross-binary replay of A0 n29 864/864 identical
    - tau-NAF: exhaustive d in [0, r-1] (Z[tau] value, non-adjacency, point equality); verify counts = unique tau-NAF on 384 records; double-and-add counts on 960
    - sibling abscissae = full additions on all 14 pairs (independent arithmetic)
    - unit test generator_orbit_lean_sib_tau_levers_keep_the_solve_and_remove_work passed on the prebuilt lib-test binary and on the validator's rebuild from committed source (opt-level override, sha256 e44dc70b...7031, deleted after)
  heuristic_validation_checks:
    - model prediction ~0.71/~0.61/~0.49 frozen before data (from disclosed A0 profile); observed 0.713/0.614/0.487
    - no correlation/bias claim; the null objects are the A/A and default-off arms (both exactly 1.000)
    - asymptotic ceiling unchanged (folded BSGS-type, ~1.6x folded rho); no extrapolation claimed
  cost_model_checks:
    - unit ecbench.gae; S = GAE/sqrt(r); band w = 0, 0.1, 1, table and sibling work priced; no missing unpriced count on any arm
    - lean removes 347 executed native ops per solve (wall time corroborates); nothing executed newly uncounted
    - sib charged one addition + sibling_second_points (k-sum/wwalk convention), priced by the band; P+(-P) removal is a ~1 GAE accounting component
    - tau recode steps priced at w as tau_naf_recode_steps_uncharged; measured about 0.8 GAE-equivalent per step on this host (descriptive), so tau alone is not a band-robust saving
    - cold single-target: every phase charged per record for IC and both rho arms; memory unchanged (median max RSS ~21 MiB)
  proof_architecture_checks: []
  verdict:
    P1 lv-all/gen-c1: passed
    lean_layout: passed (improvement at w = 0.1 and 1; tie at w = 0 by construction)
    sib: passed (improvement at w = 0 and 0.1; neutral at w = 1 once the P+(-P) charge is read as accounting)
    tau_verify: passed as a sound, fully counted check; failed as a stand-alone improvement (band-unresolved, worse at w = 1, slower in wall time)
    gap lv-all/rho-v5: passed (unresolved inside the band)
    overall: passed
  classification: T0 engineering gain of the validated generator-orbit IC arm at K1 n29; not below rho; not a relabelling as a whole (sib P+(-P) charge and tau alone are accounting/relabelling components)
  limitations:
    - raw session only in the producer scratchpad (not in git, not in the store); the gate needs it archived before VALIDATION.json
    - no pcc preregistration; the registration commit was not pushed before the data
    - ecbench binary not rebuilt by the validator (disk); binding rests on the producer's hash, the validator's lib-test rebuild and independent checks
    - single curve, L0 shared local host; counts deterministic, wall times descriptive
    - lean + sib without tau not measured (derived 0.834 / 0.668 / 0.457)
    - rho-v5 leaves its jump-index selection untallied (optimistic for rho at w > 0)
  artifact_paths:
    - research/koblitz_n29_orbit_levers_20261118/PROTOCOL.md
    - research/koblitz_n29_orbit_levers_20261118/README.md
    - research/koblitz_n29_orbit_levers_20261118/summary.json
    - research/koblitz_n29_orbit_levers_20261118/tables/records-n29.csv.gz
    - research/koblitz_n29_orbit_levers_20261118/audits/audit-n29.json.gz
    - research/koblitz_n29_orbit_levers_20261118/audits/xbin-a0-n29.json.gz
    - benchmarks/records/20261118T1512Z-koblitz-n29-orbit-levers.json
    - (scratchpad, not durable) n29lv-sessions/n29/records.jsonl.gz; bins/ecbench-n29lv-33dccd579d
  review_attestation:
    joints_owned: [whole receipt, as tasked]
    read_sibling_reports: false  # research/koblitz_known_log_columns_20261118/VALIDATOR_REPORT.md (VAL-20261118-4ae594) was read for format and its A0 n29 digest only; it is the prior study's report, not a sibling in this round
    sources_read:
      - research/koblitz_n29_orbit_levers_20261118/{PROTOCOL.md,README.md,REGISTRATION.sha256,registered-at.txt,run.sh,analyze.py,summarize.py,make_specs.py,summary.json,tables/records-n29.csv.gz,audits/xbin-a0-n29.json.gz,logs/binary.sha256,logs/run-n29.time}
      - git show 33dccd579d (plugins.rs, ic_boundary.rs, ic_framework/mod.rs, stages.rs); git log/reflog of the branch
      - src/cryptanalysis/ic_framework/plugins.rs (compact-orbit-scan generator and scan loops)
      - src/cryptanalysis/ic_boundary.rs (FactorBase, from_column_map, count_index_build, FrobeniusPairTable probe/build_inner, KSumTable and wwalk sibling code, BinaryGroup, recovery check)
      - src/cryptanalysis/koblitz_fast.rs (FastPoint, FastCurve::add, frobenius_k)
      - src/cryptanalysis/ecbench/band.rs, methods.rs (PRICED_NATIVE), fbm.rs (FB1 identity)
      - docs/ic/calibration.json; docs/VALIDATION_GATE.md
      - AGENTS.md: Success criterion, section 5a, section 6, IC measurement rules
      - crypto-autoresearcher docs/agent-runtime-core.md, agents/validator.md
      - benchmarks/records/20261118T1512Z-koblitz-n29-orbit-levers.json
      - research/koblitz_known_log_columns_20261118/summary.json (A0 n29 digest) and VALIDATOR_REPORT.md (format)
      - scratchpad copies of n29lv-sessions/n29 and klc-sessions/n29
```
