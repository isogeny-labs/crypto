# LEAD-C-quartic-n4v2-k26: N4v2 double-large-prime IC past the 64-bit group-order wall

2026-11-12. Lane C (`research/leads_20261111/lane_C_prime_ext_cover.md`, lead
`LEAD-C-quartic-n4v2-k26`). Local part only. This is a scale-up of an existing L0 result. It is
not a new improvement, and it is not a validated row: there is no replay certificate, no isolated
host and no measured rho at the new sizes. Every result statement below is **[pcc:unverified]**.

## Answer

* **The 128-bit support is in place and the old records reproduce [pcc:unverified].** It is an
  opt-in fork, `src/v2w.rs`. The committed v2 sources are untouched. `ell`, the scalars, the matrix
  coefficients, the Lanczos vectors and the `kG = Q` check are all u128. For `ell < 2^64` the fork
  runs the original arithmetic value for value. For `ell >= 2^64` it uses Barrett reduction.
  Controls:
  * The committed v2 binary and the fork produce identical records on 18 units (k = 12..22),
    5642 fields compared, in both memory modes.
  * The fork regenerates all 63 committed instances.
  * It reproduces every committed per-size mean for D/DL1/DL at k = 17..22: 18 cells, counts
    exact, phase M-eq within 3.4e-6.
* **Largest size reached: k = 25 (q = 33554383, `ell` about 2^73), 3 seeds [pcc:unverified].**
  All 81 IC records at k = 17..25 recover the planted logarithm, and `kG = Q` holds for each. That
  includes 27 records with `ell > 2^64`: 3 arms x 3 seeds at each of k = 23, 24 and 25.
* **k = 26 was not run.** One D-family unit at k = 26 is projected at about 5.7 GiB peak RSS, which
  is over the 3 GB job limit. It is also projected at about 1000-1150 CPU-s, close to the 20-minute
  limit. k = 25 needed a memory mode (described below) to fit under 3 GB at all.
* **DL exponent, frozen measure (M-eq at w = 0, this host's `calib` medians per size)
  [pcc:unverified]:**

  | range | value [95% interval] |
  |:--|:--|
  | k = 22..25 | 1.0993 [1.0993, 1.0995] |
  | k = 17..25 | 1.099 [1.098, 1.100] |

  The rise comes from the arithmetic step at k = 23: `r_lmul` goes from 2.19 to 3.15 when the u64
  path gives way to 128-bit Barrett. Measured without that step:

  | measure | k = 22..25 | k = 17..25 |
  |:--|:--|:--|
  | `r_lmul` held at its k = 22 value | 1.011 | 1.016 |
  | raw mul + lmul counts | 1.018 | 1.028 |

* **DL / rho-neg, against the calibrated rho-neg model (not measured rho) [pcc:unverified]:**

  | size | w = 0 [band] |
  |:--|:--|
  | k = 22 | 0.0049 [0.0039, 0.0061] |
  | k = 23 | 0.0045 [0.0036, 0.0055] |
  | k = 24 | 0.0031 [0.0025, 0.0038] |
  | k = 25 | 0.0021 [0.0017, 0.0026] |
  | k = 26, projected | 0.0016 |

  The projected k = 26 value is above the predicted 0.00085 [0.0006, 0.0014] and well below the
  0.01 kill line. The remaining distance to rho at k = 25 is a factor of about 470 at w = 0 and
  about 850 at w = 1.

## Verdict against the preregistered falsifiers

| falsifier / prediction (lane file, frozen) | outcome | status |
|:--|:--|:--|
| Killed if the DL exponent over **k = 22..26** has a lower 95% bound above 1.10 | k = 26 not measured, so the test as written is not evaluated. Partial range k = 22..25: e = 1.0993, lower bound **1.0993**, 0.0007 below the kill line. A k = 26 point on the k = 22..25 trend leaves the fit at about 1.099. | **not triggered on the partial range; at the edge** |
| Killed if DL / rho-neg at k = 26 has an upper bound above 0.01 | k = 26 not measured. At k = 25, upper bound 0.0026 against the model. Projected k = 26 point value 0.0016. | not triggered (k = 26 projected only) |
| Killed on any unverified logarithm | 81/81 verified, 27 of them with `ell > 2^64` | not triggered |
| Prediction: exponent over k = 17..26 of 1.00 [0.95, 1.06] (M-eq, w = 0) | k = 17..25: **1.099 [1.098, 1.100]**, outside the band. Counts: 1.028. Fixed `r_lmul`: 1.016, inside. | **prediction missed on the frozen measure** |
| Prediction: DL / rho-neg at k = 26 of 0.00085 [0.0006, 0.0014] | projected 0.0016 (k = 22..25 fit) | **missed (about 1.9x higher)** |
| Prediction: DL wall at k = 26 of 190-400 s | projected DL-arm CPU 449-486 s on this host (DL at k = 22: 16.9 s here, 12.3 s on the M4) | above the band |

The exponent miss has two parts:

1. **Host calibration.** The same counts (identical records) give a k = 17..22 exponent of
   **1.077** with this host's `calib` constants. With the committed M4 constants they give
   0.988. Here `r_inv` rises from 14.9 to 24.0 and `r_lmul` from 1.65 to 2.19 over k = 17..22. On
   the M4, `r_lmul` falls from 2.37 to 2.01.
2. **The arithmetic step at the 64-bit wall.** `r_lmul` goes from 2.19 to 3.15 at k = 23. This is
   the effect the lead's "up to 2x for wider arithmetic" anticipated.

Neither part is a change in the algorithm's operation counts, which grow as q^1.02-1.03. The
bootstrap intervals cover unit sampling only (3 seeds, low spread). They do not cover
conversion-constant uncertainty, as Deviation 6 of the N4v2 README already notes.

The overall status is **partial**:

* The implementation step and its controls are complete.
* k = 23 and 24 were run as the lead's local plan asked, and k = 25 as well.
* The k = 22..26 falsifiers cannot be evaluated without k = 26 and a measured rho reference.

## What was run (exact commands)

All runs are single-threaded (`RAYON_NUM_THREADS=1`) on the shared 4-vCPU container, with load
average 1.2-6 from other agents. Each job ran under `runjob.py`, which records `wait4` user/sys CPU
and peak RSS (`ru_maxrss`) in `*.res.json`, because there is no `/usr/bin/time`. The code base is
Git HEAD `6b7fc4520e`.

```bash
cd research/leads_20261111/runs/C_quartic_k26
# builds (binaries are .gitignored; hashes in raw/SHA256SUMS.txt, raw/SHA256SUMS_v2w2.txt)
(cd ../../../quartic_n4v2_20261020 && rustc --edition 2021 -O v2.rs -o $OLDPWD/bin/v2_orig)  # committed v2, unmodified
rustc --edition 2021 -O src_v1/v2w.rs -o bin/v2w      # fork used for k = 17..24 and the controls
rustc --edition 2021 -O src/v2w.rs    -o bin/v2w2     # same + packed relations (k = 25)
./bin/v2w selftest 50000                              # Barrett vs shift-and-add, inverses, Miller-Rabin
./run_controls.sh     # orig vs v2w (default and V2W_COMPACT=1) on 18 units; instance list k = 8..22
./run_wide.sh         # V2W_COMPACT=1: ic q s D for k = 23, 24 (seeds 1-3), k = 17..22 (seeds 1-3); calib x3 per size
./run_v2w2.sh         # v2w2 identity checks; k = 25 seeds 1-3 under RSS_CAP_KIB=3145728; calib x3
python3 compare.py raw/controls_final; python3 compare.py raw/controls_compact
python3 analyze.py > analysis.md     # writes results.json
python3 plot.py                      # fig_dl_scaling.{png,svg}
```

The unit command is the committed grid's own: `ic <q> <seed> D 3.0 2.0 4 4`, the D family with
c_D = 2.0, gamma = 4 and block = 4, as in `params.sh`. It runs D, DL1 and DL on the same relations.

## What the fork changes (`src/`, from `research/quartic_n4_20261019` and `research/quartic_n4v2_20261020`)

* **`common.rs`:**
  * Adds `type L = u128`.
  * Adds `Barrett` (128x128 -> 256-bit product, mu = floor(2^(2n)/ell), at most 2 corrections),
    `mulmod_l`, `inv_mod_l`, `is_prime_l` (bases 2..41, deterministic below 3.3e24) and
    `Rng::below_l`.
  * Each of these calls the original u64 code when the modulus is below 2^64, so the rng stream and
    every value are unchanged for k <= 22.
* **`ghs.rs`:** `ell: L`. The `ell <= u64::MAX` cap is lifted. The cap never bound for q < 2^22,
  which is why the instance list is identical.
* **`ic.rs`:** `Row = Vec<(u32, L)>`. N4's B0 Lanczos is removed, because arm B0 is not part of the
  fork.
* **`ic2.rs`, `la2.rs`, `solver2.rs`, `rho2.rs`:** every mod-`ell` residue becomes L, with the same
  operation sequence and the same counters.
* **Memory mode, opt-in through `V2W_COMPACT=1`:**
  * Graph labels are recomputed from the relations instead of being stored.
  * Anchors are kept in a map.
  * A sentinel log vector replaces `Option`.
  * A sealed sorted log table replaces the HashMap.
  * The edge list is reserved up front.
* **Always on:** relations hold inline (index, multiplicity) lists instead of heap Vecs, and
  large-prime points are stored as `pack2` keys.
* **`v2w2` only:** packed `Cs` and u32 adjacency offsets.
* **Peak RSS at k = 22 seed 3:** 1.16 GB in default mode, 0.45 GB in memory mode. At k = 24 seed 2:
  1.71 GB with `v2w`, 1.52 GB with `v2w2`.
* **Diagnostics:** stderr carries `mem phase=... VmHWM_kB=...` lines. The JSONL records are not
  affected.

## Results

### Controls (lead step 2) [pcc:unverified]

| control | result |
|:--|:--|
| `selftest`: Barrett vs shift-and-add, 14 moduli of 64-126 bits, 2.8M products; inverses mod 7 wide primes; 200 semiprimes | SELFTEST OK |
| committed v2 vs `v2w` default, units: D at k = 12..21 seed 1, D at k = 22 seeds 1-3, S/L0/L1/L at k = 12..16 seed 1 (`raw/controls_final/COMPARE.txt`) | 18/18 identical, 5642 non-timing fields |
| committed v2 vs `v2w` with `V2W_COMPACT=1`, same units (`raw/controls_compact/COMPARE.txt`) | 18/18 identical |
| builder: `v2w instances` for k = 8..22 vs `results/instances.jsonl` (ell, order, tries, f) | 63/63 identical |
| committed per-size means (`summary.json`, ic_size): every count statistic, and the 7 phase M-eq recomputed with the committed r_inv / r_lmul, for D/DL1/DL at k = 17..22 over seeds 1-3 | 18/18 cells match (counts exact, phase M-eq within 3.4e-6) |
| wide regime: preliminary default-mode build vs final memory-mode `v2w`, k = 23 seed 1 (`raw/wide_identity`) | identical |
| `v2w2` vs committed v2 (k = 12, 14, 16, 18) and vs `v2w` (k = 23 seed 1, k = 24 seed 2) | 4/4 and 2/2 identical |

The committed per-unit raw records (`results/grid-20261025/units.tar.gz`) are an S3 artifact that
returns 403 here. In place of a direct record-for-record match against the grid, the controls use
the committed binary rebuilt from Git, plus the per-size means and the instance list that are in
Git.

### DL per size on this host (memory mode; M-eq with this host's `calib` medians) [pcc:unverified]

| k | q | log2 ell | ok | r_inv | r_lmul | DL M-eq w=0 | w=0.1 | w=1 | DL CPU s | D-family unit CPU s (3 seeds) | peak RSS GiB |
|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|:--|--:|
| 22 | 4194271 | 64.0 | 3/3 | 24.04 | 2.194 | 1.615e9 | 1.843e9 | 3.898e9 | 16.9 | 63, 43, 41 | 0.43 |
| 23 | 8388571 | 67.0 | 3/3 | 25.33 | 3.153 | 4.225e9 | 4.689e9 | 8.866e9 | 43.8 | 102, 109, 123 | 0.84 |
| 24 | 16777183 | 70.0 | 3/3 | 26.33 | 3.058 | 8.253e9 | 9.192e9 | 1.765e10 | 95.7 | 242, 268, 221 | 1.63 |
| 25 | 33554383 | 73.0 | 3/3 | 27.30 | 2.995 | 1.638e10 | 1.828e10 | 3.539e10 | 204.4 | 497, 451, 523 | 2.87 |

The k = 17..21 rows, and the D and DL1 arms, are in `analysis.md`. DL1, the post-hoc ratchet
baseline, is about 2.0-2.1x below DL at k = 23..25.

### Exponents (ln cost on ln q, 95% bootstrap over units) [pcc:unverified]

| arm | range | e M-eq w=0 | e w=1 | e CPU | e w=0, r_lmul fixed (k=22) | e mul+lmul counts |
|:--|:--|:--|:--|:--|:--|:--|
| DL | k = 22..25 | 1.0993 [1.0993, 1.0995] | 1.054 | 1.192 [1.178, 1.206] | 1.011 | 1.018 |
| DL | k = 17..25 | 1.099 [1.098, 1.100] | 1.059 | 1.134 | 1.016 | 1.028 |
| DL | k = 17..22 (same records as the committed grid) | 1.077 [1.076, 1.079] | 1.052 | 1.064 | 1.018 | 1.032 |
| DL1 | k = 17..25 | 1.077 [1.076, 1.079] | 1.042 | 1.092 | 1.031 | 1.043 |

For comparison, the committed fit for DL over k = 17..22 (M4 constants) is 0.988 [0.986, 0.989].

### DL / rho-neg (rho-neg = calibrated model, not measured) [pcc:unverified]

The model is built as follows:

* **Steps:** 0.9175 sqrt(ell). This is the committed pooled steps/sqrt(ell) over 1225 runs.
* **Priced work:** 1.0354 GAE per step, with GAE = 56 mul + r_inv inv at this host's r_inv. Both
  come from the committed k = 22 supplement.
* **Unpriced work:** 3.04 ops per step for the w-band.
* **Band:** the calibration constant ranges over the frozen admissibility range [0.75, 1.15],
  crossed with the seed range of DL.

| k | rho-neg model M-eq w=0 | DL/rho w=0 | w=0.1 | w=1 |
|--:|--:|:--|:--|:--|
| 22 | 3.27e11 | 0.0049 [0.0039, 0.0061] | 0.0044 | 0.0030 |
| 23 | 9.39e11 | 0.0045 [0.0036, 0.0055] | 0.0039 | 0.0024 |
| 24 | 2.69e12 | 0.0031 [0.0025, 0.0038] | 0.0026 | 0.0017 |
| 25 | 7.69e12 | 0.0021 [0.0017, 0.0026] | 0.0018 | 0.0012 |
| 26 (projected, k = 22..25 fit) | 2.18e13 | 0.0016 | | |

The order (DL below rho-neg) holds across the whole w = 0 / 0.1 / 1 band at every size. It is a
ratio to a model, not to measured rho runs. The figure is `fig_dl_scaling.png`.

## What is blocked

* **k = 26 locally.** Projected peak RSS is about 5.7 GiB (2.87 GiB at k = 25 even with packed
  relations), over the 3 GB job limit. A D-family unit takes about 1000-1150 CPU-s. The run needs a
  host with at least 8 GB free, or further memory work.
* **The k = 25 memory figure is at the cap.** Peak RSS was 2.874 GiB (3.09 GB decimal) under a
  3 GiB kill guard. It stayed under 3 GiB, but not under 3.0e9 bytes.
* **Measured rho-neg at k >= 23.** One run is projected above 20 minutes: 616 CPU-s per run at
  k = 22 on the M4, and about 2.8x more per k. The ratios above therefore use the calibrated model
  only, which is what the lead's `rho_reference` allows.
* **The isolab job** (k = 25, 26 with 3 seeds, rho-neg at k = 23, 24, replay certificate). The AWS
  identity is rejected in this container. There is no validation file, so this remains an L0 local
  result.
* **The committed per-unit grid records.** The S3 artifact returns 403. Section "Controls" lists
  what was used instead.
* **research-kb.** The MCP server is not available to this subagent, so no knowledge-base receipts
  were recorded. Sources read: the lane file, `research/quartic_n4v2_20261020/README.md`,
  `summary*.json` and `instances.jsonl`.

## Deviations

1. `V2W_COMPACT=1` (memory mode) was used for every k >= 17 run. Default mode reproduces it exactly
   (controls), but it cannot fit k >= 24 under 3 GB: 2.55 GB at k = 23.
2. k = 25 used `bin/v2w2`, which adds packed relations and u32 adjacency offsets. Its identity with
   `v2w` and with the committed v2 was checked on 6 units (table above), not on the full 18.
3. The calibration is this host's (Linux x86, shared), not the M4 of the committed grid. The
   per-size `calib` medians are the frozen M-eq rule. Their effect on the exponent is about +0.09
   over k = 17..22 (1.077 vs 0.988 on identical counts).
4. Only the D family ran past k = 22. The S family and B0 are not runnable or not needed there.
5. The fork's files are not rustfmt-formatted, because the originals are not either. Formatting
   would bury the diff.
6. The bootstrap intervals cover unit sampling only (3 seeds, very low spread).

## Next step

* **Run the isolab job as the lead specifies.** That means k = 25 and 26 with 3 seeds each
  (`bin/v2w2`, `V2W_COMPACT=1`, at least 8 GB memory, about 20 min per unit at k = 26), plus
  measured rho-neg at k = 23 and 24. Use one host's calibration for all sizes and produce a replay
  certificate. This is what evaluates the k = 22..26 falsifiers as written.
* **Treat the `r_lmul` step at k = 23 as an engineering target, not a method property.** A
  Montgomery or 2-limb `mulmod` for `ell` < 2^80 should bring r_lmul back toward the u64 path. That
  would move the frozen-measure exponent toward the counts exponent of 1.02. It would be an
  arithmetic (constant-factor) improvement, measured separately.
