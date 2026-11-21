# Validation report: genus 3 at p = 8209 ... 32771, double large prime + prefilter + SGE against rho (Study B)

Validator: independent replay, 2026-11-21 (agent role `validator`). Study under test:
`research/genus_ladder_sge_scale_20261115/` (PROTOCOL.md, RESULTS.md, `sge_g3.jsonl`, `joined_analysis.json`,
`data.sha256`, `binary.sha256`; rho reference rows `research/genus_ladder_scale_20261115/scale_g3.jsonl`).
Every figure below is `[pcc:unverified]` (the validator files no pcc claim).

## Disposition

| item | disposition |
|---|---|
| p = 8209 and p = 16411: IC arms (unmerged, SGE(8,3)), instance lines, admission cells | **validated** (row-by-row identical) |
| p = 8209 and p = 16411: rho rows (`--arms R`, the extra check) | **validated** (row-by-row identical to the reference rows) |
| p = 32771 (the headline cell, ratio 0.283): IC arms, rho, instances | **validated** (the 50-minute instance search was run; row-by-row identical) |
| analysis (cell means, ratios to rho, GAE band numbers, exponents, bootstrap intervals) | **validated as arithmetic**: recomputed from the producer's raw rows and from my replayed rows; the unchanged analyzer reproduces `joined_analysis.json` bit for bit (section 4). Two reporting findings (section 7) |
| the sentence "best measured arm of the program" and the headline "about 0.3 of rho" as a ranking | **not supported as stated** (section 7, finding 2); the FPM figures themselves are validated |
| standing under the validation gate | **pre-gate**: same shared 4-vCPU container, not an isolated host; no `VALIDATION.json`; the rho reference was not measured in this study (reused from the scaling study; I re-ran it, section 3); the section 5a band is unresolved; the bench record has `validation: null` and the raw file `uploaded: false`. Does not qualify for the IC leaderboard or `benchmarks/README.md` |

Refusal check: frozen, committed artifact set and clean-tree build both present (marker below). Not refused.

## 1. Provenance and binding

- Build marker (`.../scratchpad/BUILD_DONE`): `build started 2026-11-21T19:34:42Z HEAD c9749cbffae41c9420cbf43e8c40b0475e254124 dirty_src: []`, `exit 0`, `build finished 2026-11-21T19:43:37Z dirty_src: []`,
  `cc69ca6bccc0d719309d5ee9b5d78fcf019b64e16ec1138844258bed04910b0f  target/release/genus_ladder`,
  `e8db68c7db36be4dc245128f90e17ecd5e95f8453d06cba7633134871e0fe342  target/release/genus_ladder_analyze`.
- Copies used: `/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/validator_c5/genus_ladder` and `genus_ladder_analyze`; re-hashed, both equal the marker. `genus_ladder_analyze` also equals the producer's analyzer binary (`scratchpad/sge/genus_ladder_analyze`, `e8db68c7...`), so the analysis tool is the same bytes.
- Validated commit `c9749cbffa`. `HEAD` moved during the work (`ce87d83cec`, later `8b825e96fe`) and other agents have uncommitted edits in `src/bin/genus_ladder.rs`, `src/cryptanalysis/genus_ladder.rs`, `src/cryptanalysis/quartic_ladder.rs`; the binary copy is unaffected, line numbers are read from `git show c9749cbffa:...`. Commits through `ce87d83cec` did not change `src/` or this study directory.
- Producer binary: `binary.sha256` = `b0c619a8...  /tmp/claude-0/.../scratchpad/sgescale/genus_ladder` (a scratch path, not durable); the file exists and hashes to that value. The bench record `benchmarks/records/20261115T0609Z-genus-ladder-sge-scale.json` binds commit `64e5c0e415` (`commit_dirty: false`); at that commit `src/cryptanalysis/genus_ladder.rs` hashes to `f70a41b9...` and `src/bin/genus_ladder.rs` to `0ca5a69f...`, the same two sources as Study A's freeze. `HEAD` `c9749cbffa` differs from that source by one visibility edit (`pub(crate) fn sge_reduce`) and by other crate files (for example performance edits in `ic_boundary.rs`, including the rho walk). The rebuilt binary has a different hash (cc69ca6b...) and reproduces every count (section 3), so those edits moved no count.
- `frozen.sha256` (PROTOCOL.md): OK. `data.sha256` lists only `sge_g3.jsonl` (`c6e9548c...`) and `joined_analysis.json` (`4cd14266...`): both hash correctly. RESULTS.md, `tables.md`, figures are not hashed. The rho reference `scale_g3.jsonl` (`ed825804...`) equals the hash in `research/genus_ladder_scale_20261115/data.sha256` and the scratch copy I used.
- Host: same shared container (Linux 6.18.44-fc, `nproc` 4), load average 9.5 at the first launch, 14 to 20 for the rest (other agents ran their own `genus_ladder` jobs, including the same instance searches). Not isolated; counts are the measurement, walls are informational only.

## 2. What was replayed (commands verbatim except `--arms` and `--out`; scratch outputs in `.../scratchpad/validator_c5/out/`)

| run | command (common flags `--genera 3 --curves 3 --seeds 2 --sge 8,3 --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600`) | start (UTC) | wall | exit | output sha256 |
|---|---|---|--:|--:|---|
| B_c | `--primes 8209,16411 --arms c` | 19:45:47 | 1,025.0 s | 0 | `006f691d...` |
| B_R | `--primes 8209,16411 --arms R` | 20:00:04 (parallel with B_c's tail, because of load) | 1,207.8 s | 0 | `94438e42...` |
| B_cR_32771 | `--primes 32771 --arms cR` | 20:03:37 | 3,029.6 s | 0 | `41f6c370...` |

Deviation, disclosed: p = 32771 was run with both arms (`cR`) in one invocation, so its instance search (about 35 to 50 minutes) ran once instead of twice. Rows of different arms are independent functions of (instance, arm, seed); the comparison below is per arm. The producer's single invocation covered all three primes with `--arms c`. Output rows are written in thread-completion order, so file order differs from the producer's.

## 3. Rows compared and mismatches (every field of every row, except `wall_ns`, `search_wall_s`, `wall_s`)

Comparator `tools/rowdiff.py` (keys: instance `(p, g, curve_seed)`, cell `(p, g)`, run `(p, g, curve_seed, seed, arm)`).

| comparison | rows | fields | mismatches |
|---|--:|--:|--:|
| replay `run` rows of arms `C_double_lp+pre` and `C_double_lp+pre+sge(8,3)` vs `sge_g3.jsonl`: p = 8209, 16411 (24 rows) and p = 32771 (12 rows) | 36 of 36 | 1,512 | **0** |
| replay unmerged `C_double_lp+pre` rows vs `scale_g3.jsonl` (the producer's Z1): p = 8209, 16411 (12) and 32771 (6) | 18 of 18 | 594 | **0** |
| replay `R_rho_neg` rows vs `scale_g3.jsonl`: p = 8209, 16411 (12, from the `--arms R` run) and 32771 (6, from the `cR` run) | 18 of 18 | 306 | **0** |
| `instance` lines (9 distinct; replayed 15 times across the three invocations) vs `sge_g3.jsonl` and `scale_g3.jsonl` | 15 | 135 | **0** (9 / 9 instances; curve seeds 8209: 6, 18, 23; 16411: 8, 27, 43; 32771: 8, 16, 31) |
| `cell` (admission) lines: `tried`, `admitted`, `rejects` | 5 (3 distinct primes) | 35 | **0** (tried 23, 43, 31; rejects `order_not_near_prime` 20, 40, 28) |
| producer's own Z1 re-checked on its two raw files (`sge_g3.jsonl` unmerged vs `scale_g3.jsonl`) | 18 rows + 9 instances | 594 + 81 | 0 |

All 54 replayed `run` rows (36 IC rows and 18 rho rows) have `certified: true` and `ok: true`. Outputs: `out/B_c_rowdiff_vs_sge_g3.txt`, `out/B_c_rowdiff_vs_scale_g3.txt`, `out/B_R_rowdiff_vs_scale_g3.txt`, `out/B_32771_rowdiff_vs_sge_g3.txt`, `out/B_32771_rowdiff_vs_scale_g3.txt`, `out/B_producer_Z1_check.txt`.

## 4. Recomputed means and ratios (own code, `tools/b_recompute.py`, from the producer's raw rows first, then from the replay)

Per (p, arm), 6 runs each (3 curves x 2 seeds). `FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul`, g^2 = 9; rho rows have no `rel_mul` or `la_mul`. Values from the producer's rows and from the full replay are identical.

| p | arm | mean `la_mul` | mean `FPM_mid` | `FPM_mid` / rho | producer's RESULTS.md |
|--:|---|--:|--:|--:|--:|
| 8209 | unmerged | 6.9758e6 | 8.4247e7 | 0.5911 | 0.591 |
| 8209 | SGE(8,3) | 2.1994e6 | 4.1259e7 | **0.2895** | 0.290 |
| 8209 | rho | n/a | 1.4252e8 | 1 | |
| 16411 | unmerged | 1.7977e7 | 2.1403e8 | 0.7415 | 0.741 |
| 16411 | SGE(8,3) | 6.4669e6 | 1.1044e8 | **0.3826** | 0.383 |
| 16411 | rho | n/a | 2.8866e8 | 1 | |
| 32771 | unmerged | 4.6430e7 | 5.4176e8 | 0.5372 | 0.537 |
| 32771 | SGE(8,3) | 1.7977e7 | 2.8569e8 | **0.2833** | 0.283 |
| 32771 | rho | n/a | 1.0084e9 | 1 | |

SGE arm `la_mul` / unmerged `la_mul` (ratio of means = mean of per-pair ratios): 0.3153, 0.3597, 0.3872 (RESULTS.md 0.32, 0.36, 0.39).

- Producer raw rows vs `joined_analysis.json`: 51 / 51 values (`mean_la_mul`, `fpm_mid.mean`, `mean_group_ops`, `fpm_mid_over_rho`, GAE ratios at 0, 0.1, 1) agree to 4 s.f. Replayed rows vs `joined_analysis.json`: 51 / 51 agree (`out/B_compare_full_replay_vs_joined.txt`).
- The unchanged analyzer (`e8db68c7...`) on the producer's concatenation (`scale_g3.jsonl` then `sge_g3.jsonl`, byte-equal to the scratch `joined.jsonl`) reproduces `joined_analysis.json`: 702 / 702 cell numbers and 108 / 108 fit numbers bit-identical. With the 72 replayed IC and rho rows substituted key by key into that input (the 18 `A_full+pre` rows, not replayed, kept), the analyzer output equals `joined_analysis.json` in all 702 cell numbers and 108 fit numbers, bit for bit (only `mean_wall_ns` ignored): the analysis depends on counted fields only, and the replayed counts are the producer's.
- GAE band (section 5a, recomputed: cost = `group_ops + c (rel_mul + la_mul + native)`, c in {0, 0.1, 1}), SGE arm over rho: p = 8209: 0.0862, 2.024, 10.75; p = 16411: 0.0991, 2.634, 14.04; p = 32771: 0.0669, 1.862, 9.942. Below rho at c = 0, above at 0.1 and 1: the order does not hold across the band, so `unresolved` (producer's `band_gae`, and Z5) is right. RESULTS.md prints the c = 0 and c = 1 ends only ("0.07 to 0.10", "10 to 14x"); the c = 0.1 values (1.86 to 2.63) are in `tables.md`.
- FPM band verdict for the SGE arm, from the unchanged analyzer: `below` at all three primes (upper interval ends 0.428, 0.726, 0.361), as claimed (Z3).
- Exponents (log-log least squares of mean `FPM_mid` on p, three primes): unmerged 1.3444, SGE 1.3978, rho 1.4134, equal to the producer's `fits` (intervals 1.335 to 1.356, 1.385 to 1.410, 1.143 to 1.740).

Scorecard recomputed: Z1 held (9 / 9 instance lines, 18 / 18 unmerged rows; also reproduced by replay). Z2 held (36 / 36, plus 18 / 18 rho and the replay). Z3 held (0.2833 in [0.22, 0.42]; `below` at all three primes in the FPM reading). Z4 held (0.3153 to 0.3872 in [0.20, 0.40]; the scorecard text "0.31" is 0.3153 truncated, the table says 0.32). Z5 held (`band_gae` unresolved at all three primes).

## 5. Certificates

From `git show c9749cbffa:src/cryptanalysis/genus_ladder.rs`:
- `run_ic`: line 1512 `let chk = curve.mul(&mut ops, inst.g, t);`; line 1515 `let ok = chk == inst.target && curve.mul_rtl(inst.g, t) == inst.target;`; lines 1520 to 1522 `if ok { res.recovered = Some(t); res.certified = true; ...`.
- Rho arm (`run_rho`): lines 1927 to 1930 `certified = rr.recovered.map(|d| inst.curve.mul_rtl(inst.g, d) == inst.target)`.
- Driver `src/bin/genus_ladder.rs`: line 399 (IC arms) and line 352 (rho): `rec["ok"] = json!(o.recovered == Some(inst.t_secret));`.
- `run_ic` and `run_rho` never read `t_secret` (it appears only at lines 836, 887, 888, 897 and in tests), so `ok` compares the answer with a secret the solver cannot see.
- `mul_rtl` (line 604) is a right-to-left double-and-add calling `cantor` directly and uncounted; `mul` is the trait default of `ic_boundary.rs` line 1529, a left-to-right double-and-add through the counted `add`/`double`.

Limits, stated plainly: both paths use the same `cantor` group law and the target is built by `mul_rtl` (line 888), so the check certifies the logarithm under the implemented law. Run rows do not record `G`, `T` or the recovered `t`; I could not re-verify individual logarithms with code outside the binary, and instead replayed and diffed the flags (54 / 54 true). What I could do independently: a pure-Python Cantor implementation (`tools/indep_group_check.py`, no shared code) checks Hasse-Weil, `[order]D = 0`, `[r]([h]D) = 0` and `[h]D != 0` on three random divisors per instance: **9 / 9 genus-3 instances (p = 8209, 16411, 32771) pass**, `r` prime, `order = h r`, `h <= 8` on all 9 (`out/B_producer_indep_group_check.txt`; the replayed instance lines are identical). No run was classed `invalid_measurement`.

## 6. Controls and reporting contract

- Reference arm: rho (negation map, `R_rho_neg`) on the same 9 instances and the same 2 seeds, priced in the same FPM unit and in the GAE band; the unmerged arm is also priced (so the SGE gain is visible inside the same table). The rho rows were measured by the scaling study with a different binary (`61ccabd2...`) at another time; I re-ran them with the current build and they are identical, which closes the "reused reference" gap for these three primes.
- Priced work: FPM prices multiplications only (additions free); `la_add` is not in the study's table. Checked on the producer's rows: the SGE arm's `la_add` over the unmerged arm's is 0.606, 0.691, 0.752 at p = 8209, 16411, 32771 (below 1 but rising with p) and `rel_add` is identical in every pair, so the free-additions convention does not favour the SGE arm at these primes; the rising ratio means that margin shrinks with p.
- One table, one unit, ratio, phases separated: RESULTS.md's table is in FPM units with a ratio column and intervals; GAE band numbers are in `tables.md`. The section 5a band is stated as unresolved, as it is.
- m = 83 / ECC2K-130 rules: not applicable.

## 7. Findings

1. **The unmerged arm is counted twice in `joined_analysis.json`.** `genus_ladder_analyze` does not de-duplicate rows; the joined input holds the unmerged `C_double_lp+pre` rows twice (once in `scale_g3.jsonl`, once in `sge_g3.jsonl`), so those cells show `runs: 12` (also visible in `tables.md` T2; RESULTS.md says "36 runs"). Means and ratios are unaffected. Intervals are: re-running the same analyzer on de-duplicated rows (`out/B_analysis_dedup_producer_raw.json`) widens the unmerged arm's `FPM_mid`/rho interval from [0.457, 0.846] to [0.437, 0.870] at 8209, from [0.534, 1.175] to [0.499, 1.422] at 16411, and from [0.422, 0.687] to [0.407, 0.686] at 32771. The SGE arm's own cells have 6 runs; their intervals shift only through the analyzer's shared resampling stream: 8209 [0.224, 0.428] unchanged, 16411 [0.266, 0.726] to [0.261, 0.726], 32771 [0.221, 0.361] to [0.214, 0.361]. FPM band and GAE band verdicts are unchanged at every prime and arm; the exponent intervals of the two IC arms move by at most 0.003, rho's lower end from 1.1425 to 1.1309. RESULTS.md's interval figures for the unmerged arm are therefore too narrow; the point estimates and the verdicts stand.
2. **"This is the best measured arm of the program at genus 3 over F_p" is not supported by the study's artifacts.** No cross-program table is in the directory, and under the section 5a band the SGE arm and rho cannot be ranked (0.07 to 0.10 of rho at c = 0, 1.9 to 2.6x at c = 0.1, 9.9 to 14.0x at c = 1). "About 0.3 of rho" holds only in the FPM reading (`I = 10`, `g^2` per multiplication modulo r, additions free), which RESULTS.md names but its first sentence of "Reading" does not scope. Keep the figure, scope the sentence as an FPM-reading result.
3. **Sample size.** Six runs per cell from 3 curves; the intervals resample 3 curves, and the exponents are three-point fits (rho's interval is 1.14 to 1.74, so the three exponents cannot be told apart; RESULTS.md makes no exponent claim, but its explanation of the SGE exponent being higher than the unmerged one is an interpretation not tested here).
4. **Provenance**: producer binary is a scratch-path file built from a working tree (commit bound only through the bench record); the bench record's raw file has `uploaded: false` (the 29 KB file is in Git, so nothing is lost).
5. **Certificate scope** as in section 5 (shared group law; `G`, `T`, `t` not recorded).

## 8. Not verified

- The rho reference was not independently re-derived beyond rerunning it with the same program; no external rho implementation was compared. The 3-curve sample is the producer's design.
- Wall times (informational only); no isolation level; the g^2 and `I = 10` price conventions; the Wiedemann operation counting convention (same function in both arms, not re-derived); `REPORT.pdf`, `REPORT.html`, `fig_ratio.*` (not checked against data).

## 9. Reproduce

```sh
# binaries: /tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/validator_c5/{genus_ladder,genus_ladder_analyze}
genus_ladder run --primes 8209,16411 --genera 3 --curves 3 --seeds 2 --arms c --sge 8,3 --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --out out/B_c_8209_16411.jsonl
genus_ladder run --primes 8209,16411 --genera 3 --curves 3 --seeds 2 --arms R --sge 8,3 --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --out out/B_R_8209_16411.jsonl
genus_ladder run --primes 32771 --genera 3 --curves 3 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --out out/B_cR_32771.jsonl
python3 -I tools/rowdiff.py sge_g3.jsonl out/B_c_8209_16411.jsonl            # also: scale_g3.jsonl vs the R and cR outputs
python3 -I tools/b_recompute.py scale_g3.jsonl sge_g3.jsonl ; python3 -I tools/b_gae.py scale_g3.jsonl sge_g3.jsonl
python3 -I tools/b_compare.py joined_analysis.json out/B_joined_replay.jsonl
genus_ladder_analyze <scale_g3.jsonl + sge_g3.jsonl> out.json && python3 -I tools/deepdiff.py joined_analysis.json out.json
```
Scripts (sha256, in `.../scratchpad/validator_c5/tools/`): `rowdiff.py` 3b40c82c..., `b_recompute.py` 887703ad..., `b_gae.py` d8b8002a..., `b_compare.py` 5d41192a..., `deepdiff.py` 591a04b8..., `indep_group_check.py` 2c8f6c9e.... Other output hashes: `out/B_joined_replay.jsonl` 8fa9d41b..., `out/B_analysis_exact_substitution.json` 113bf7e0..., `out/B_analysis_dedup_producer_raw.json` aab50a7e..., `out/B_runs.log` dee834e3.... The scratch directory is not durable; the hashes above identify the files if they are preserved.
