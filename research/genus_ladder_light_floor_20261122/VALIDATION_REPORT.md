# Validation report: `research/genus_ladder_light_floor_20261122` (validator, 2026-11-22)

Status of this report: `[pcc:unverified]` (a validator's report is an assertion until a pcc claim carries it). Gate standing of the study: **pre-gate**
("unvalidated (pre-gate)"): there is no `VALIDATION.json`, no pinned replay-certificate or validator-report file, and no §5a band (findings F1, F2); this report
is the input for those files, not them.

## Disposition

| scope | disposition |
|---|---|
| All measured cells (3 primes x 6 curves x 2 seeds x 5 arms = 180 IC runs, 18 instance lines) re-run in full with the recorded binary: every row equal field for field to the producer's except wall times; the 36 reused rho rows re-run as well, equal to R2's | **validated** |
| The claim as worded (paired `FPM_mid` of `--excess E,0.01` over plain SGE(8,3): E = 0.2: 0.860 / 0.848 / 0.829, E = 0.3: 0.857 / 0.841 / 0.809; `la_mul` 0.64-0.65 at E = 0.2; best arm over R2's rho rows 0.226 / 0.303 / 0.356), recomputed by my own script | **validated** to 3 significant digits, in the unit the study declares (`FPM_mid`, I = 10, kappa = g^2 = 9) |
| The improvement as a unit-independent statement; a §5a-banded ranking against rho; leaderboard or `benchmarks/README.md` admission | **insufficient_evidence** (F1, F2: in the study's own rows the sign of the gain depends on the weight of a mod-r multiplication; no band is reported) |
| Certificate by code independent of the solver | **insufficient_evidence** (F3: rows carry `certified` and `ok` only; the logarithm, G and T are not in the artifacts) |
| Frozen pre-fix `analyze.py` | not recoverable; shown not to matter for the reported ratios (item 4) |

No run is `invalid_measurement`. Nothing was refused.

## 1. Provenance and bindings

* Dispatch HEAD `e593da5e23`; the working tree was clean (`git status --short` empty) at the start and at the end. **HEAD moved during the validation** to
  `c1b4c0189e` (three R5 commits by other work in this checkout). `git diff e593da5e23 HEAD` touches only a new directory `research/genus_ladder_light_floor_g45_20261122`,
  `research/hecdlp_campaign_20261121/FRONTIER.md` (18 + 1 files); `git diff e593da5e23 -- src Cargo.toml Cargo.lock research/genus_ladder_light_floor_20261122
  research/genus_ladder_g3_highp_20261121` is empty. The artifact set I validated is therefore the one committed at `e593da5e23` (sha `e593da5e235a4ca86d17f1093ede3ff9c2805980`), unchanged throughout.
* Binary: copies of `.../scratchpad/r4/genus_ladder` and `genus_ladder_analyze` placed in `.../scratchpad/validator_r6/bin/`; sha256 `b47dbfa33bdd8ae2abf7e0cba08fc439d27021c1f672954e20b0898656130e35` and
  `943d7f65c9f1ffebaa43a7788b0b46ba91d4b9172aa53afb02d7eef1d4049b23`, equal to `BUILD_DONE3` (commit `d046bb88ac4b919114a195004b1a24cbd06019ef`, `dirty_src: []` at start and end, `test exit 0`) and to the study's `binary.sha256`.
  The repository's own `target/release/genus_ladder` has the same hash (mtime 01:52:36Z); no file under `src/`, `Cargo.toml` or `Cargo.lock` is newer than it; the marker's 3-second "build" at 02:16:46Z is consistent with a no-op re-check of an up-to-date tree (only `genus_ladder_analyze` carries a 02:16:49Z mtime; its hash equals R2's analyzer).
  I did not rebuild (not allowed), so the binary-to-commit binding rests on the marker, the mtimes and the committed sources, not on a reproduced build.
* `git show d046bb88ac --stat`: R3 commit, 17 files, of which only `src/bin/genus_ladder.rs` (+60 -1) and `src/cryptanalysis/genus_ladder.rs` (+363) are code; the rest is `research/genus_ladder_light_rows_20261121/`.
  `git diff d046bb88ac HEAD -- src/ Cargo.toml Cargo.lock` is **empty** at `e593da5e23` and at `c1b4c0189e`: no source change after that commit. The R3 diff, read in full, adds the `--excess` option, new result fields and tests; the unselected path is unchanged
  (the test `excess_none_and_noop_reproduce_baseline_counts` pins counts for `None`, and my field-for-field check of the plain and unmerged arms against R2's rows, below, shows it).
* Declared artifacts exist and hash as recorded: `data.sha256` 8/8 (hashes are of the decompressed files; I checked `zcat | sha256sum` of `run_{65537,131101,262147}.jsonl.gz`, `joined.jsonl.gz`, and `partial2/*`). One label is wrong: the last line of `data.sha256` is named `joined.jsonl` but is the hash of `partial2/joined.jsonl.gz` (412d96b3...), not of `joined.jsonl.gz` (dc0c5755...). Cosmetic.
  `frozen.sha256`: PROTOCOL.md, AMENDMENT.md and the post-amendment `run_all.sh` match; the two failing entries are the disclosed ones (see item 4).
* Seeds and commands: the study's `run_all.sh` as amended (it carries `--curves 6`), with only the binary path and output directory changed:
  `genus_ladder run --primes P --genera 3 --curves 6 --curve-seeds P=<six seeds> --seeds 2 --arms c --sge 8,3 --excess 0.1,0.01 --excess 0.2,0.01 --excess 0.3,0.01 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --order-method bsgs1 --out run_P.jsonl`
  (curve seeds 158/217/250/254/255/283, 41/58/61/79/84/101, 69/113/206/207/241/261; run seeds 1, 2). Script: `.../scratchpad/validator_r6/replay.sh`; rows: `.../scratchpad/validator_r6/out/run_{65537,131101,262147}.jsonl`.
* Host: Linux 6.18.44 firecracker VM, 4 vCPU Intel Xeon @ 2.10 GHz, 16 GB, a **shared container, not isolated**. Replay `done` walls 106 / 351 / 745 s against the producer's 104 / 317 / 747 s. Walls are not used for any claim.

## 2. What was replayed and how rows were compared

All three primes in full, including all six curves at 262147 (so the minimum asked, curves 4 and 5, is covered), plus, as an extra control, the reference arm: `genus_ladder run ... --arms R --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --seed-search 600 --order-method bsgs1 --rho-cap 3e9`
on the same curve seeds (`.../validator_r6/rho/`), compared with R2's `R_rho_neg` rows.

Comparison script `.../validator_r6/compare.py` (own code; keys indexed with `[]`; every field except those whose name contains `wall` is compared, and the field sets must be equal both ways). Negative control: tampering one `la_mul`, one `mean_weight_after` by 1e-12 and deleting one `la_sel_add` in a copy
is detected in all three cases.

| comparison | rows compared | mismatching (row, field) pairs | field-set differences |
|---|--:|--:|--:|
| replay vs producer `run_65537.jsonl.gz` (config, 6 instance, cell, 60 run rows) | 68 | **0** | 0 |
| replay vs producer `run_131101.jsonl.gz` | 68 | **0** | 0 |
| replay vs producer `run_262147.jsonl.gz` | 68 | **0** | 0 |
| producer plain and unmerged rows + instance lines vs R2's rows (L1; 24 + 6 per prime) | 90 | **0** | 0 (the new rows have no extra fields on these arms) |
| `partial2/` (curves 4 and 5 of each prime: 60 run + 6 instance rows) vs the full run's rows | 66 | **0** | 0 |
| replayed rho rows (12 per prime) + 6 instance lines per prime vs R2's | 54 | **0** | 0 |

Wall-type fields excluded: `wall_ns`, `order_search_wall_s`, `search_wall_s` (and the `done` row's `wall_s`). Mismatches listed: none. The replay's own `run_P.log` files are empty.
All 180 replayed runs have `certified = true`, `ok = true`, `exhausted = false`, `la_fallbacks = 0`, `attempts = 1`; in the 108 excess runs `ex_fallback = 0` (no selection was undone).

## 3. Recomputed ratios against the producer's

Own script `.../validator_r6/my_analysis.py`, written before I opened `analyze.py`; it gives numerically identical output on the replay rows and on the producer's rows. FPM_mid = `cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul`, rho = `cantor_mul + 10 cantor_inv`;
paired ratio = mean over the 12 (curve, seed) pairs of excess/plain; rho ratio = mean FPM of the arm over 12 runs / mean FPM of the 12 R2 rho runs on the same curves (instance lines checked equal first).

| quantity | 65537 | 131101 | 262147 | producer / claim |
|---|--:|--:|--:|---|
| FPM_mid paired, E = 0.1 | 0.883571 | 0.879038 | 0.869730 | 0.884 / 0.879 / 0.870 |
| FPM_mid paired, **E = 0.2** | **0.860249** | **0.848005** | **0.828866** | 0.860 / 0.848 / 0.829 |
| FPM_mid paired, **E = 0.3** | **0.856870** | **0.841376** | **0.809200** | 0.857 / 0.841 / 0.809 |
| `la_mul` paired, E = 0.1 / 0.2 / 0.3 | 0.7422 / **0.6388** / 0.5682 | 0.7539 / **0.6489** / 0.5820 | 0.7580 / **0.6487** / 0.5709 | 0.64-0.65 at E = 0.2 |
| relation phase paired, E = 0.1 / 0.2 / 0.3 | 1.1006 / 1.2001 / 1.2998 | 1.1007 / 1.2005 / 1.3006 | 1.0997 / 1.1997 / 1.2997 | "exactly 1 + E" (within 0.002) |
| FPM_mid / rho, plain | 0.263248 | 0.360129 | 0.439361 | 0.263 / 0.360 / 0.439 |
| FPM_mid / rho, E = 0.1 / 0.2 / 0.3 | 0.2326 / 0.2265 / **0.2256** | 0.3165 / 0.3054 / **0.3030** | 0.3821 / 0.3642 / **0.3555** | best arm 0.226 / 0.303 / 0.356 |
| unmerged arm / rho | 0.468825 | 0.602640 | 0.693939 | 0.469 / 0.603 / 0.694 |
| mean row weight after / plain (L4, E = 0.2), from `sge_nnz_in / sge_rows_in` of the plain arm | 0.770 | 0.749 | 0.729 | 0.770 / 0.749 / 0.729 |
| `la_mul` exponent in p (3 primes, least squares): plain, E = 0.1, 0.2, 0.3 | 1.506, 1.522, 1.518, 1.510 | | | same; FPM exponent 1.430 plain, 1.389 for E = 0.3 |

Every number agrees with `RESULTS.md` to the digits it prints (the 262147 best-arm value 0.355505 rounds to 0.356; at 65537 E = 0.2 and E = 0.3 both round to 0.226). The paired means are bit-equal to `analysis.json` (max absolute difference 0). `python3 -I analyze.py` run in scratch on the
decompressed rows reproduces `analysis.json`, `analyze.out` and `joined.jsonl` (sha256 `dc0c5755...`) exactly. Scorecard L1-L5 recomputed: L1 held (90/90 rows equal to R2's, 180/180 certified), L2 held (0.860 / 0.848 / 0.829 against 0.90 / 0.87 / 0.85; `la_mul` <= 0.649 against 0.70),
L3 held (relation phase 0.1 < 0.2 < 0.3 and `la_mul` 0.3 < 0.2 < 0.1 at all primes; best E at 262147 is 0.3: 0.809 against 0.829 and 0.870), L4 held, **L5 falsified** (0.3555 against 0.33), as `RESULTS.md` says.
My own hierarchical bootstrap (4000 draws, seed 777) gives FPM/plain E = 0.2: [0.855, 0.865], [0.843, 0.853], [0.825, 0.833]; E = 0.3: [0.852, 0.861], [0.836, 0.847], [0.805, 0.814]; FPM/rho for E = 0.3: [0.163, 0.352], [0.200, 0.413], [0.253, 0.575]
(producer: [0.163, 0.345], [0.199, 0.415], [0.254, 0.588]; the intervals differ at the third digit by bootstrap seed only). Every one of the 12 pairs per prime and E lies below 1 in FPM_mid (min-max at E = 0.2, 262147: 0.820 to 0.842).

## 4. Integrity of the excess mechanism (source at `d046bb88ac`, identical at HEAD)

File `src/cryptanalysis/genus_ladder.rs` unless named.

1. **Extra relations are charged like the others.** `run_ic_ext_excess` (1707-2152). On the first stop of the rule the hook (1940-1949) sets `next_check = ceil(rels.len() * (1 + extra))` and `continue`s into the *same* collection loop (1799-1930): `res.trials += 1` (1828), the field work of the walk
   and root finding goes to `res.rel` (1737, 1753-1754, 1836, 1852, 1859, 1881-1882, 1907-1909) and to the curve's counters, `res.relations += 1` (1928). `cantor_mul` and `cantor_inv` are the curve counters' whole-run difference (2143-2144), so they include the extra walk. The `max_trials` guard (1800) applies. Data confirm it:
   in all 108 excess rows `excess_rows_collected == relations == ceil(plain relations * (1 + E))`, and the relation phase is 1 + E times plain (section 3).
2. **Selection cannot drop the target column's last row, with a fallback.** `light_survivors` (1661-1698): rows are dropped heaviest first, but a row is skipped if *any* of its columns has live weight < 2 (1687-1689); the target column is a column of the peeled core (index `ct`, last), so its last holder is never dropped. The floor is `max(ceil(keep * rows), cols + margin(cols))` (1668), and `target >= rows.len()` returns "keep all" at no cost (1669-1671).
   After the drop, `select_light_rows` (1630-1657) re-peels (1648) and **checks** `holds_t = core has a row with column ct` and `core.len() >= cc + margin(cc)` (1651-1656); otherwise `Err(true)` and the caller keeps the original core and counts `ex_fallback` (1993-1996). `Err(false)` (nothing dropped) also keeps it (1639-1641). Data: `ex_fallback = 0` in all 108 runs, `rows_kept >= core_cols` everywhere, so the fallback path was not exercised; it is covered only by reading.
3. **The certificate still gates `certified`.** The closure `certify` (2017-2026) computes `curve.mul(G, t) == T` (left-to-right, charged to `verify_ops`) and `curve.mul_rtl(G, t) == T`, and restores the field-work counters after the second ladder. Every path to an answer goes through it (2038, 2045, 2079, 2118) and `res.certified = true` is set only after an answer exists (2086-2090, 2125-2129). The excess option does not touch this code (R3 diff).
   The solver does not read the planted secret: `t_secret` does not occur in 1707-2152; the driver's `ok = (recovered == Some(inst.t_secret))` (`src/bin/genus_ladder.rs:681`, the branch that writes the excess rows) compares against it afterwards. The secret is planted at 1070-1071 (`target = mul_rtl(g, t_secret)`).
4. **`la_sel_*` is in the LA total, and is zero multiplications.** `res.la_sel.merge(sel); res.la.merge(sel)` (1985-1986) runs before the result is inspected, so the selection's work is charged on both the `Ok` and the `Err` branch, together with the first peel (1965-1968), SGE (2033-2034, 2067-2068) and Wiedemann (2084-2085). `select_light_rows` and `peel` (`src/cryptanalysis/chained_transfer/ic_control.rs:1134`) increment only `Nat::add`;
   `la_sel_mul = 0` in all 108 rows, `la_sel_add` is 0.1-0.6 M against `la_add` of 2.7e8 to 3.6e9 (at most 0.06% of it), and `la_add >= la_sel_add + la_sge_add + la_wied_add` holds in every excess row. The driver writes `la_mul = o.la.mul` (`src/bin/genus_ladder.rs:690`). **Consequence for the unit:** FPM_mid prices `la_mul` only, so the selection, the re-peel and all native additions are free in FPM_mid (they are small, but see F1).
   One pre-existing oddity, not on the exercised path: the second solve block (2092-2122) repeats the first when it fails and `w_nat` is merged into `la_wied` twice (2084, 2123), which would over-charge a failed attempt; no run reached it (`attempts = 1`, `la_fallbacks = 0` everywhere).

## 5. Disclosed deviations

* **`frozen.sha256` entries that fail.** (a) `analyze.py` (frozen `aa203b20...`, now `9630d4d8...`) and (b) the pre-amendment `run_all.sh` (`c0a40e4f...`, superseded by the second entry `eb5b0c1e...`, which matches, together with the matching `AMENDMENT.md`).
  The pre-fix `analyze.py` **is not recoverable**: git has one commit containing the file (`a974ae6b36`, the final version) and `partial2/analyze_as_run.py` is byte-identical to the final one (`9630d4d8...`, so it is already the fixed version). Argument that the fix cannot move the reported ratios: in the current file the weights are stored under `mean_mean_weight_before/after`
  (`c["mean_" + k]`) and read back as `c["mean_mean_weight_*"]`; a read of `c["mean_weight_*"]` is a `KeyError` that stops the script before `analysis.json` is written, so a pre-fix run could not have produced a wrong number. `mean_wt_*` feeds only the L4 weight ratio and the weight columns, not the FPM, `la_mul`, relation-phase or rho ratios. Independently,
  my script (no use of `analyze.py`) reproduces every reported ratio and, from raw rows, the L4 weights. This is an inference from the final code, not a diff of the frozen version.
* **`--curves 6` omitted from `PROTOCOL.md`'s command** (AMENDMENT.md): the first launch measured curves 4 and 5 only. `partial2/` rows (66) equal the full run's rows for those curves on every field but walls. The partial run was analysed (a peek at 2 of the 6 curves: L2 0.860 / 0.849 / 0.828, best E = 0.3) before the amendment; the thresholds and settings are the PROTOCOL's (the L3 "my pick 0.3" is in the frozen PROTOCOL text), and
  the full-data numbers (0.860 / 0.848 / 0.829) are what I reproduced. The full result is not independent of that peek for curves 4 and 5; that is disclosed.

## 6. Findings that limit what the numbers may be used for

**F1. The gain exists in `FPM_mid` (and `FPM_high`) and not in the other readings the repository's own analyzer defines.** From the study's own rows (my script; the analyzer's definitions of the cost readings, `src/bin/genus_ladder_analyze.rs:9-19` (doc) and `49-60` (`costs`)), paired excess/plain:

| reading | E = 0.2: 65537 / 131101 / 262147 | E = 0.3: 65537 / 131101 / 262147 |
|---|--|--|
| fpm_low (I = 1, kappa = 1) | 1.112 / 1.103 / 1.090 | 1.185 / 1.173 / 1.154 |
| **fpm_mid (I = 10, kappa = g^2 = 9), the study's unit** | **0.860 / 0.848 / 0.829** | **0.857 / 0.841 / 0.809** |
| fpm_high (I = 30, kappa = 9) | 0.884 / 0.870 / 0.850 | 0.888 / 0.871 / 0.838 |
| gae_w0 (group operations only) | 1.200 / 1.200 / 1.200 | 1.300 / 1.301 / 1.300 |
| gae_w0.1 (group ops + 0.1 per native mul) | 1.056 / 1.047 / 1.030 | 1.113 / 1.101 / 1.075 |
| gae_w1 | 1.052 / 1.043 / 1.026 | 1.107 / 1.096 / 1.070 |

The break-even weight of one mod-r multiplication (kappa*, I = 10, cell means) is 3.25 / 2.90 / 2.48 at E = 0.2 and 4.07 / 3.65 / 3.05 at E = 0.3 (65537 / 131101 / 262147); the study fixes kappa = 9. Below that weight the option costs more than the plain arm, because it pays `1 + E` in the relation phase for a smaller linear algebra.
So the order of plain and excess arms is not stable across the cost readings the repository itself provides; AGENTS.md §5a ranks arms only where the order holds across the band. `RESULTS.md` states the unit and notes that the g^2 conversion is "the one of R2/R3" but does not say that the sign of the gain depends on it.
The claim as worded ("FPM_mid of ... 0.860") is correct; any wording like "the excess option is cheaper" without the unit is not supported by these rows.

**F2. No §5a band is reported for the new arms.** `RESULTS.md` and `analysis.json` give FPM ratios only (R2's `RESULTS.md` disclosed "10-13 times rho under GAE w = 1, unresolved"; this study omits that sentence). I ran the repository's `genus_ladder_analyze` (the copy, hash `943d7f65...`) on the study's `joined.jsonl`
(216 runs, 0 not certified; output `.../validator_r6/an/band_out.json`). Ratio to rho at w = 0 / 0.1 / 1 GAE: plain 0.054 / 1.69 / 9.07, E = 0.2 0.065 / 1.79 / 9.54, E = 0.3 0.071 / 1.88 / 10.04 at 65537; 0.066 / 2.25 / 12.08, 0.079 / 2.36 / 12.60, 0.086 / 2.48 / 13.24 at 131101;
0.073 / 2.60 / 13.99, 0.087 / 2.68 / 14.35, 0.094 / 2.80 / 14.96 at 262147. The analyzer's `band_gae` verdict is `unresolved` for every IC arm at every prime, and `band_fpm` is `below` rho (except the unmerged arm at 262147, `unresolved`).
This is **not** the canonical rule: that analyzer prices only `rel_mul + la_mul + native` at w, so IC arms' `rel_add`/`la_add` and rho's `coef_adds_uncharged`, `compares_uncharged`, `hash_uncharged` (R2 rows, 1.9e8 to 8.3e8 compares per run on average) stay at 0 for both arms, and it is not `ecbench band`/`band.rs`. I report it as the repository's reading, not as a replacement for the band the gate needs.
The matched-rho requirement of the gate (same binary or process, band at 0 / 0.1 / 1) is not met by the study's own files; my rho replay with the study's binary reproduces R2's rho rows exactly, which addresses reproducibility, not the band.

**F3. Certificate.** The rows carry `certified` and `ok` but not the recovered logarithm, G or T, and the Cantor arithmetic that checks `[t]G = T` is the solver's own (two ladders over the same code). What I could verify: `ok` equals the planted secret, which the solver never reads (item 3 above), and the run is deterministic and replays field for field.
A check by an independent genus-3 implementation was not possible from the artifacts; the dispatch did not ask for one, but the certificate-discipline rule would need it for the gate.

**F4. The rho reference is a single realisation per run and was lucky or unlucky by prime.** Mean rho steps / sqrt(r) on these 12 curves per prime: 1.014 / 1.027 / 0.673 (R2: 18 runs, 0.994 / 0.886 / 0.778; range per run 0.105 to 2.59). Measured over the expected rho cost (0.886 sqrt(r) steps at the measured cost per step): 1.13 / 1.17 / 0.73.
The best-arm ratios against *expected* rho are 0.255 / 0.355 / 0.260 for E = 0.3, not 0.226 / 0.303 / 0.356: the rise of the ratio with p in the claimed row (0.226 -> 0.356) is partly rho's draw (the 262147 reference is 27% under its expectation). The claim as worded uses measured rho and is validated as that; "the ratio grows with p" is not established by it.
(R2 reports the same effect, 0.86 of expectation at 262147 over 18 runs.)

## 7. Requirement checklist

| requirement | result | evidence |
|---|---|---|
| frozen, committed artifact set; commit recorded; no moving tree | pass, with a note (HEAD moved, frozen set unchanged) | section 1; `git diff e593da5e23 -- <paths>` empty |
| binary hash = marker = `binary.sha256`; source unchanged since `d046bb88ac` | pass (hashes); binding to commit not reproduced (no rebuild allowed) | section 1 |
| replay of 65537, 131101 in full; 262147 at least curves 4, 5 | pass: all three in full, all six curves | section 2, `.../validator_r6/out/` |
| every `run` and `instance` row equal but walls | pass: 204 rows (180 run, 18 instance, 3 config, 3 cell), 0 mismatches | `compare.py` |
| independent recomputation of ratios, to 3 significant digits | pass | section 3, `my_analysis.py`, `my_replay.json` |
| excess mechanism integrity | pass (1-4 above) | line numbers in section 4 |
| disclosed deviations | pass: pre-fix `analyze.py` not recoverable, argued immaterial; `partial2` = full rows | section 5 |
| controls: null, reference, cheaper baseline | reference arm (rho) replayed and equal to R2; plain SGE(8,3) is the cheaper baseline and is in every pair; the unmerged arm is in the table; the null (no-excess identity) is the L1 check and passes | sections 2, 3 |
| reporting contract: one table, one unit, all arms priced incl. rho, band | **partial**: one unit (FPM_mid) and one table; no §5a band; unit-dependent sign (F1, F2) | section 6 |
| certificate by independent code | **not possible from the artifacts** (F3) | section 6 |
| m = 83 / ECC2-130 disclosure rules | not applicable (genus 3, p = 65537 to 262147) | |

## 8. Limits of this validation

Shared 4-vCPU container, not isolated (walls unused); the replay is the same binary on the same inputs, so it establishes determinism and absence of copy errors, not independence of the algorithm's counters from the solver; the binary was not rebuilt; the rho rows' original binary (R2, `6f40afb3...`, commit `98005699ee`)
differs from the IC binary (`b47dbfa3...`), although the rho rows reproduce under the latter; six fresh curves per prime, one SGE setting, genus 3 only; the best E at 262147 is chosen on the data it is scored on (as the study says); the three-prime exponent fits are descriptive.

## 9. Candidate lessons (proposed text only; the ledger was not edited, the dispatch allows one output file)

* validator, research: when a study's headline is a paired ratio in one weighted unit (here FPM_mid with kappa = g^2) -> recompute the same pairs under the repository's other cost readings (fpm_low, gae_w0.1, gae_w1) and report the break-even weight before accepting "cheaper"; here the sign flips at kappa < 2.5-4.
* validator: when the dispatch HEAD can move (other sessions commit in the same checkout) -> record the sha at the start, and at the end `git diff <sha> -- <artifact paths> src Cargo.*` to show the validated set is unchanged.

Scratch (not evidence, reproducible from the commands above): `/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/validator_r6/` (`replay.sh`, `compare.py`, `my_analysis.py`, `out/`, `rho/`, `an/`).
