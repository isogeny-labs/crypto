# Validation report: genus ladder follow-up 2, structured Gaussian elimination before Wiedemann (Study A)

Validator: independent replay, 2026-11-21 (agent role `validator`). Study under test:
`research/genus_ladder_sge_20261115/` (PROTOCOL.md, RESULTS.md, `data.sha256`, `binary.sha256`).
Every figure below is `[pcc:unverified]` (no pcc claim was filed; the validator does not file claims).

## Disposition

| item | disposition |
|---|---|
| genus-3 and genus-4 cells (p = 1031, 2053; 16 instances, 192 runs, 4 arms) | **validated** (replay reproduces every counted figure; see section 5) |
| genus-5 cell (p = 1031) | **insufficient_evidence** for a replay (not replayed, about 15 minutes of instance search). Recomputed from the producer's raw rows in the checkout: agrees with `analysis.json` and `paired.json` (section 6). That is a re-derivation, not a replay |
| row-by-row comparison with the producer's raw `sge_g34.jsonl` | **insufficient_evidence** (file only in the artifact store; not reachable, section 4) |
| matched rho reference with the section 5a band | **not present** (the study states it ran no rho); not required for a component comparison, required by the validation gate |
| standing under the validation gate | **pre-gate**: shared 4-vCPU container, not an isolated host; no matched rho; no `VALIDATION.json`; counts only, no wall-time claim. Does not qualify for the IC leaderboard or `benchmarks/README.md` |

Refusal check: the dispatch named a frozen, committed artifact set and a clean-tree build; the marker shows both. Not refused.

## 1. Provenance and binding

- Build marker (`.../scratchpad/BUILD_DONE`, written by the main session): `build started 2026-11-21T19:34:42Z HEAD c9749cbffae41c9420cbf43e8c40b0475e254124 dirty_src: []`, `exit 0`, `build finished 2026-11-21T19:43:37Z dirty_src: []`,
  `cc69ca6bccc0d719309d5ee9b5d78fcf019b64e16ec1138844258bed04910b0f  target/release/genus_ladder`,
  `e8db68c7db36be4dc245128f90e17ecd5e95f8453d06cba7633134871e0fe342  target/release/genus_ladder_analyze`.
- Copies used: `/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/validator_c5/genus_ladder` and `genus_ladder_analyze`; re-hashed after the copy, both equal the marker (cc69ca6b..., e8db68c7...).
- Validated commit: `c9749cbffa`. During the validation `HEAD` moved on (`ce87d83cec`, later `8b825e96fe`) and other agents left uncommitted edits in `src/bin/genus_ladder.rs`, `src/cryptanalysis/genus_ladder.rs`, `src/cryptanalysis/quartic_ladder.rs`. None of that touches the validated binary copy. Quoted line numbers below are read from `git show c9749cbffa:...`, not from the working tree. Commits after `c9749cbffa` up to `ce87d83cec` did not change `src/` or this study directory; I did not audit `8b825e96fe`.
- Source freeze check (`frozen.sha256`, run from the repo root): `PROTOCOL.md`, `paired.py`, `src/bin/genus_ladder.rs` OK; `src/cryptanalysis/genus_ladder.rs` **FAILED** (frozen `f70a41b9...`, committed `0586377e...`). Cause found: `git show a552dc2ee8:src/cryptanalysis/genus_ladder.rs` hashes to the frozen `f70a41b9...`, and the only difference to `c9749cbffa` in that file is the visibility of one function (`fn sge_reduce` became `pub(crate) fn sge_reduce`, in the composition-study commit). No behavioural change. The other crate files changed between the freeze and `c9749cbffa` (30 files, among them `ic_boundary.rs` performance edits); the replay is the test that they did not move any count (section 5).
- Producer binary: `binary.sha256` = `50e5986e...`, equals `scratchpad/sge/genus_ladder.sge` (exists; hash verified). It was built from a working tree and records no commit. The rebuilt binary has a different hash (different sources), as expected. I did not run the producer binary.
- `data.sha256` check on files in the checkout: `sge_g5.jsonl`, `paired.json`, `analysis.json`, `smoke.jsonl`, `run_g34.log`, `run_g5.log`, `tables.md`, `fig_sge.svg`, `render.py`, `RESULTS.md` all OK. `sge_g34.jsonl`, `sge.jsonl`, `repro.jsonl` are absent (listed in `.artifacts.json`, bytes in S3): hash not verifiable here.
- Host: same shared container (Linux 6.18.44-fc, `nproc` 4, load average 3.4 at the start of the replay and 15 to 20 later, other agents active). Not isolated. Counts are the measurement.
- Environment for the run: validator copies in `scratchpad/validator_c5/`; outputs in `scratchpad/validator_c5/out/` (scratch space, not durable; hashes below).

## 2. What was replayed

```
genus_ladder run --primes 1031,2053 --genera 3,4 --curves 4 --seeds 3 --arms c --cc 2.0 --max-h 8 --threads 3 \
    --sge 2,3 --sge 4,3 --sge 8,3 --out out/A_g34.jsonl
```
Start 2026-11-21T19:44:17Z, end 19:45:37Z, exit 0, `real 1m20.636s` (producer: 58.6 s; informational). 214 lines: config, 16 instance, 4 cell, 192 run, done. 192 / 192 runs `certified` and `ok`.
Output SHA-256 `c863d72747a1a929cb00a3f8303695c62455d4166757981a8f627dbe6ba2e227` (`out/A_g34.jsonl`; row order is thread dependent).

## 3. Requirements and evidence

| requirement | result | evidence |
|---|---|---|
| frozen committed artifacts, commit recorded | pass (with the `frozen.sha256` finding above) | marker; `git show a552dc2ee8:...` |
| clean-tree build, binary hash bound | pass | marker, `validator_c5/` hashes |
| seeds, commands, tool versions | pass: command from PROTOCOL.md section 3 run verbatim; solver seeds 1, 2, 3 are inside the driver | `out/A_g34.env.txt` |
| instance lines equal `analysis.json` `instances` | pass: 16 / 16 identical (all fields, incl. `f`, `order`, `r`, `h`); curve seeds p=1031 g3 2,8,9,10; g4 35,59,61,80; p=2053 g3 3,21,25,36; g4 2,24,33,40 as expected | `out/A_cells.txt`, `out/A_extra_claims.txt` |
| cell means: `mean_la_mul`, `mean_group_ops`, `fpm_mid.mean` vs `analysis.json` | pass: 48 / 48 equal, relative difference 0.0 (bit-identical) | `out/A_cells.txt` (`tools/a_cells.py`, own code, not the analyzer) |
| paired ratios vs `paired.json` | pass: 12 per-cell and 6 per-genus means equal to 4 s.f.; with rows sorted by key, the frozen `paired.py` on the replay reproduces all 162 values (means and bootstrap endpoints, 3 ratios x 18 groups x mean/lo/hi) bit for bit; all 300 `structure` numbers (merged, wt-1, refused, nnz, rows/cols in and out, ...) equal | `out/A_paired_replay_sorted.json` |
| relation phase untouched (P1) on replayed pairs | pass: 144 / 144 pairs identical on all 18 fields (frozen `paired.py` output) | `out/A_paired_replay_sorted.json` |
| all runs certified with planted t (P2) | pass: 192 / 192 | replay rows |
| certificate re-verified with independent code | partial, see section 7 | `out/A_indep_group_check.txt` |
| controls | see section 8 | |
| reporting contract | see section 8 | |

## 4. Rows compared and mismatches

- Instance lines: 16 replayed vs `analysis.json`: 16 identical, 0 mismatches.
- Run rows, merged arms (144 rows): no raw producer rows available. Cell means and paired statistics, which the producer derived from those rows, agree (section 3). The producer's `sge_g34.jsonl` (sha `f838b6a2...`) is only in the artifact store (S3); no copy exists anywhere on this machine (searched `/` for `sge_g34.jsonl` and `sge.jsonl`), and this sandbox has no AWS CLI or credentials. Row-by-row comparison with it: **insufficient_evidence**.
- Run rows, unmerged arm (48 rows) vs the first study's `C_double_lp+pre` rows (`scratchpad/run/part_a.jsonl` sha `32061f65...` and `part_b.jsonl` sha `20ca1553...`, both equal to `research/genus_ladder_20261114/data.sha256`): **48 rows, 1,584 fields (all but `wall_ns`), 0 mismatches**; 4 cell lines (28 fields) 0 mismatches; 16 instance lines compared with `smith` excluded (the first study ran with `--smith`, so its genus-3 instance lines carry an extra `smith` object that a run without `--smith` does not write; 8 lines): 144 fields, 0 mismatches. This reproduces the producer's own "60 of 60" statement for the 48 rows of genus 3 and 4 (`out/A_rowdiff_vs_first_study_ignore_smith.txt`).
- Unchanged analyzer on the replay: 16 cells, 480 non-interval leaves, 479 bit-identical; the one difference is `gae_w01.mean` of `sge(4,3)` p=1031 g3, relative 1.7e-16 (floating-point summation order of the rows). Bootstrap endpoints of the analyzer are not comparable (shared resampling stream over all 20 cells; the genus-5 cells are not in the replay).

## 5. Recomputed cell means vs the producer's `analysis.json` (replayed rows; independent code)

All values equal to full double precision. `la` = mean `la_mul`, `grp` = mean `group_ops`, `FPM` = mean `FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul`.

| p | g | arm | la | grp | FPM |
|--:|--:|---|--:|--:|--:|
| 1031 | 3 | none | 404,821 | 4,480.8 | 5.4957e6 |
| 1031 | 3 | sge(2,3) | 295,550 | 4,480.8 | 4.5123e6 |
| 1031 | 3 | sge(4,3) | 164,717 | 4,480.8 | 3.3348e6 |
| 1031 | 3 | sge(8,3) | 98,750 | 4,480.8 | 2.7411e6 |
| 1031 | 4 | none | 1.4817e6 | 14,343.5 | 3.3177e7 |
| 1031 | 4 | sge(2,3) | 1.2153e6 | 14,343.5 | 2.8915e7 |
| 1031 | 4 | sge(4,3) | 760,877 | 14,343.5 | 2.1643e7 |
| 1031 | 4 | sge(8,3) | 494,938 | 14,343.5 | 1.7388e7 |
| 2053 | 3 | none | 1.0448e6 | 9,193.3 | 1.3373e7 |
| 2053 | 3 | sge(2,3) | 771,008 | 9,193.3 | 1.0909e7 |
| 2053 | 3 | sge(4,3) | 437,156 | 9,193.3 | 7.9043e6 |
| 2053 | 3 | sge(8,3) | 277,747 | 9,193.3 | 6.4696e6 |
| 2053 | 4 | none | 4.2011e6 | 32,045.6 | 8.8895e7 |
| 2053 | 4 | sge(2,3) | 3.5430e6 | 32,045.6 | 7.8366e7 |
| 2053 | 4 | sge(4,3) | 2.3396e6 | 32,045.6 | 5.9111e7 |
| 2053 | 4 | sge(8,3) | 1.5144e6 | 32,045.6 | 4.5908e7 |

Paired ratios (mean of per-pair ratios; replay = producer), headline setting `sge(8,3)`: `la_mul` 0.2554 (g3, 24 pairs), 0.3473 (g4, 24); `FPM_mid` 0.4918 (g3), 0.5204 (g4). Bootstrap 95% (sorted rows, bit-identical to `paired.json`): `la_mul` [0.2422, 0.2691] (g3), [0.3341, 0.3584] (g4); RESULTS.md prints 0.255 [0.242, 0.269] and 0.347 [0.334, 0.359]. Other settings: `sge(2,3)` la 0.7340 / 0.8318, FPM 0.8185 / 0.8765; `sge(4,3)` la 0.4135 / 0.5353, FPM 0.5996 / 0.6587 (g3 / g4).

## 6. Genus 5 (not replayed)

`sge_g5.jsonl` (hash OK, in the checkout): independent recomputation of 4 cell means each for `mean_la_mul`, `mean_group_ops`, `fpm_mid` against `analysis.json` (12 values) and of the 3 paired means (la, FPM) against `paired.json`: all equal (`out/A_g5_recompute_from_producer_rows.txt`). 48 / 48 runs certified and ok in those rows. `sge(8,3)` g5: `la_mul` ratio 0.2742, `FPM_mid` ratio 0.6245, as in RESULTS.md. This checks arithmetic on the producer's rows only; the rows themselves were not reproduced: **insufficient_evidence** for genus 5.

## 7. Certificates

From `git show c9749cbffa:src/cryptanalysis/genus_ladder.rs`:
- `run_ic`: line 1512 `let chk = curve.mul(&mut ops, inst.g, t);`; line 1515 `let ok = chk == inst.target && curve.mul_rtl(inst.g, t) == inst.target;`; lines 1520 to 1522 `if ok { res.recovered = Some(t); res.certified = true; ...`. `certified` is set only after both comparisons to the target hold.
- Driver `src/bin/genus_ladder.rs` line 399 (IC arms; line 352 for rho): `rec["ok"] = json!(o.recovered == Some(inst.t_secret));`.
- `t_secret` is read only at instance construction (`genus_ladder.rs` lines 836, 887, 888, 897) and in tests (2352, 2514, 2532); `run_ic` never reads it, so `ok` compares the solver's answer with a secret it cannot see.
- `mul_rtl` (line 604, comment "the independent path of the certificates") is a right-to-left double-and-add that calls `cantor` directly and is not counted, whereas `mul` is the trait default of `ic_boundary.rs` line 1529, a left-to-right double-and-add through the counted `add`/`double`.

Limits of that certificate, stated plainly: both paths use the same `cantor` group law, and the target itself is built by `mul_rtl` (line 888), so the check certifies the logarithm under the implemented law, not the law. The run rows do not record `G`, `T` or the recovered `t`, so I could not re-verify each logarithm with code outside the binary; I replayed and diffed the `certified`/`ok` flags (192 / 192 true).
Independent check that I could do: a pure-Python Cantor implementation (`tools/indep_group_check.py`, shares no code with the solver) takes each instance's `f`, `p`, `h`, `r`, `order`, builds three random divisors from random points and checks Hasse-Weil, `[order]D = 0`, `[r]([h]D) = 0`, `[h]D != 0`: **16 / 16 replayed instances pass** (genus 3, 4) and **4 / 4 genus-5 instances** of the producer's file pass; `r` prime and `order = h r`, `h <= 8` on all 20 (`out/A_indep_group_check.txt`, `out/A_g5_indep_group_check.txt`). That supports the group law and the stated orders on which the certificates rest. It does not re-derive any individual logarithm. No run was classed `invalid_measurement`.

## 8. Controls and reporting contract

- Reference arm: the unmerged arm is priced in every paired ratio, on identical relations (P1 holds on every replayed pair), so the gain is a component gain (linear algebra), not a relabelling. Phases are separated (`la_mul`, `rel_mul`, `cantor_*`).
- Unpriced work: the study counts the merge's own multiplications and additions in `la_mul`/`la_add`. `FPM_mid` prices multiplications only, so additions are free in that reading; the paired `la_add` ratio for `sge(8,3)` is 0.360, 0.421, 0.576, 0.657 (g3 p1031, g3 p2053, g4 p1031, g4 p2053) and 0.626 (g5), all below 1, so the omission does not favour the merge. The `g^2` price of a multiplication modulo r and `I = 10` are stated conventions of the first study, not measurements; no ranking against another method is made here.
- Matched rho: none, by the study's own statement ("no rho was rerun here, so no band verdict is claimed"). Cheaper baselines or an external fastest implementation: not applicable to a component comparison; none claimed.
- One unit: ratios in counted units with 95% intervals; the ratio is stated as a ratio. m = 83 / ECC2K-130 rules: not applicable.
- Predictions: P1 held (144 / 144 replayed, 180 / 180 reported), P2 held (192 / 192), P3 and P4 hold at genus 4 in the replay (0.347, 0.520) and at genus 5 on the producer's rows (0.274, 0.624), **P5 failed** exactly as the producer reports (FPM ratio g5 0.6245 > g4 0.5204 > g3 0.4918, recomputed). The study does not hide it.

## 9. Findings (none changes a counted result)

1. **Frozen source hash does not verify at HEAD** (section 1). Explained by one visibility edit; the frozen blob exists at `a552dc2ee8`.
2. **Raw rows not available to a validator.** `sge_g34.jsonl`, `sge.jsonl`, `repro.jsonl` live in S3 only; the row-level check for the merged arms was replaced by comparison with the producer's aggregate files (which agree) and with the first study's rows for the unmerged arm. A dispatch that stages the three files would close this.
3. **Bootstrap intervals depend on input row order.** `paired.py` feeds `random.Random(20261115)` through per-curve lists in file order. On my replay file (thread-completion order) the by-genus interval endpoints differ from `paired.json` by up to 2.3e-3 relative; with rows sorted by `(p, g, curve_seed, seed, arm)` all 162 values are bit-identical. Point estimates do not depend on order. Reproducing the published intervals needs the sorted order, which RESULTS.md does not state.
4. **Three narrative figures in RESULTS.md "What the merge did" are not supported by the cell data** (the conclusions they serve still hold): (a) "merge's own multiplications ... at most 2.5e-4 of the merged `la_mul`": cell means of merge mul / `la_mul` for `sge(8,3)` are 3.84e-4, 1.13e-4, 9.8e-5, 2.7e-5 (g3 p1031, g3 p2053, g4 p1031, g4 p2053), 6.4e-6 (g5); the largest single run is 1.62e-3. (b) "its additions are 0.7% to 2.3% of the merged `la_add`": 2.86%, 1.60%, 0.92%, 0.46% (g3, g4 cells), 0.65% (g5). (c) "`la_add` falls (ratio 0.39 to 0.63 at `w_max = 8`)": 0.360 to 0.657 over the four replayed cells, 0.626 at g5. (The text "85 x 65" for genus 3, p = 1031 is the rounding of 84.5 x 65.2.)
5. **Provenance of the producer binary**: built from a working tree, no commit recorded in `binary.sha256`; bound only by source hashes (one of which now differs, finding 1) and by the replay.
6. **Certificate scope** as in section 7: group law shared by `mul`, `mul_rtl` and the instance's target; rows do not carry `G`, `T`, `t`.

## 10. Not verified

- Genus-5 replay; the producer's raw genus-3/4 rows (row by row); the producer's `sge_g34.jsonl`, `sge.jsonl`, `repro.jsonl` hashes (absent); the first-study reproduction check of RESULTS.md ("240 rows, 7,152 fields", `repro.jsonl` absent); `fig_sge.svg` and `render.py`; the `g^2` and `I = 10` price conventions; the Wiedemann operation counting convention (same function in both arms, not re-derived).
- Wall times: informational only, shared host. No isolation level was established.

## 11. Reproduce

```sh
# binaries: /tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/validator_c5/{genus_ladder,genus_ladder_analyze}
genus_ladder run --primes 1031,2053 --genera 3,4 --curves 4 --seeds 3 --arms c --cc 2.0 --max-h 8 --threads 3 \
    --sge 2,3 --sge 4,3 --sge 8,3 --out out/A_g34.jsonl
python3 -I tools/a_cells.py out/A_g34.jsonl analysis.json paired.json        # cells, paired, instances
python3 -I tools/rowdiff.py out/A_ref_first_study_subset.jsonl out/A_g34.jsonl --arms-new C_double_lp+pre --ignore smith
python3 -I tools/indep_group_check.py out/A_g34.jsonl 3
```
Validator scripts (sha256): `a_cells.py` 373dc9ce..., `rowdiff.py` 3b40c82c..., `indep_group_check.py` 2c8f6c9e..., `deepdiff.py` 591a04b8...; all in `/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/validator_c5/tools/`. Output hashes: `out/A_cells.txt` 345c8c3b..., `out/A_paired_replay_sorted.json` 525c9831..., `out/A_rowdiff_vs_first_study_ignore_smith.txt` 69f5f4d5....
