# Frobenius-folded k-sum on Koblitz K1 n23: sizing pilot (2026-11-15) `[pcc:unverified]`

Lever 1 of `research/noneric_lane_survey_20261115/README.md`. **Sizing pilot, not data**
(`BENCHREC_SKIP="sizing pilot, not data"`): one curve, 8 public targets, 1 round, L0, local host, no
validation-gate evidence, no leaderboard or `benchmarks/` row. Every number here is an unverified
assertion until a frozen study reproduces it.

## Sources checked

- research-kb: not available in this session (no MCP tool); knowledge-base pass still owed before a study card.
- `research/noneric_lane_survey_20261115/README.md` (lever 1, decision rule).
- Code read: `src/cryptanalysis/ic_boundary.rs` (`KSumTable`, `FrobeniusPairTable`, `decompose_mitm_frobenius`),
  `src/cryptanalysis/koblitz_fast.rs` (`FrobeniusCanon::canon_with_shift`),
  `src/cryptanalysis/ic_framework/plugins.rs` (`KSumOracle`, `FrobeniusMitmOracle`, `KoblitzClosureBase`,
  `CompactOrbitScanBase`), `src/bin/ecbench/pdp3_koblitz.rs` (K1 recipe).

## Step A: decision gate (no code)

Spec `pilot_gate_spec.json`; session `ECBS1h77f98d347fab`, 88/88 verified;
`records.jsonl` sha256 `ba236f13f80f04eb29b70fb84fe0c83223a02d229137c43b419b1f6c32692042` (scratch only).
Curve `icv1-f2m23-tm5197-1f85e9e1` (K1, a = 1, n = 23, log2 r = 22.00). Mean S (w = 0, ecbench record):

| arm | recipe | S |
|:--|:--|--:|
| rho-strong | `rho.signed_frobenius_strong` | 0.881 |
| rho-neg | `rho.negation` | 1.245 |
| k1-l8 / l9 / l10 | `koblitz-standard-subspace:dimension=8/9/10` + `pdp3-koblitz:m=3,engine=rootfind-negfold` | 50.16 / **43.63** / 126.04 |
| ksum4-n10 / n12 / n14 | `binary-multiplier:size=N,mult=1` + `ksum:m=4` | 7.80 / **5.80** / 7.30 |
| ksum5-n9 / n10 / n12 | `binary-multiplier:size=N,mult=1` + `ksum:m=5` | 8.47 / **6.54** / 10.24 |

Gate: best ksum5 S / best K1 S = 6.54 / 43.63 = **0.15** (the rule stops at > 2). Pass. The *unfolded*
generic k-sum already beats the validated K1 recipe by about 6.7x at w = 0 on this curve.

## Step B: what was built

- `FrobeniusKSumTable` and `FrobeniusKSumOracle` (`src/cryptanalysis/ic_framework/plugins.rs`, appended):
  a table of every signed m-sum (m = 3, 4, 5) of a Frobenius-closed base, one entry per `<sigma, -1>` orbit,
  keyed by the normal-basis canonical name of the sum's abscissa (`FrobeniusCanon::canon_with_shift`, the
  canonicalisation `FrobeniusPairTable` / `mitm-frobenius*` use). The first summand is the representative of
  the least orbit the sum meets; the others are later points of the orbit-sorted base, distinct abscissae.
  A probe canonicalises the target once, the two shifts give the rotation t, the summands
  `sigma^t(P_i)` come from an orbit-member index, and m − 1 additions confirm `±R`. The relation is
  `Relation::Summands` over base points; the base's column map supplies `±lambda^t` coefficients
  (as `FrobeniusMitmOracle` does).
- Accounting: every build and probe addition in `GroupOps`; build canonicalisations (orbit pass + one per
  completed sum), Frobenius maps and lookups of the member index in `setup_native`
  (`canonicalisations`, `frobenius_maps`, `lookups`, all in `PRICED_NATIVE`); probe canonicalisations and
  lookups in `OracleCounters`; insert attempts and normal-basis build work in `setup_work_native` behind
  the `counted:` marker, priced in the band.
- Dispatch: `ksum-frobenius:m=3|4|5` in the binary match of `src/cryptanalysis/ecbench/methods.rs`,
  with help text. Refuses a base not closed under the Frobenius (e.g. `koblitz-standard-subspace`) and
  more than 2^28 completions.
- Tests (`ksum_frobenius_tests`, `cargo test --release --lib ksum_frobenius`: 2 passed): on K1 n17
  (`koblitz-closure:dimension=2`) the folded table finds every decomposition the unfolded `KSumTable`
  finds for m = 3, 4, with no fold mismatches and every hit re-summing to its target; the pipeline
  recovers planted logs on K1 n17 (`compact-orbit-scan:columns=2`) for m = 3, 4, three seeds each.

## Step B: pilot with the folded arm

Spec `pilot_fold_spec.json` (Step A arms + 12 folded arms on `compact-orbit-scan:columns=C,raw_x_cap=1000000`,
`g` = `generator_orbit=1`); session `ECBS1h7ba97825e983`, 184/184 verified;
`ecbench verify --replay-all`: 184 replays reproduced, 0 problems. `records.jsonl` sha256
`1775d0adebcd7a11a80089a9b6cb23f9b7c57af457bda02f184c16c68df13cb8` (scratch only). §5a band
(`band_fold.txt`, `band_fold.json`, table work priced):

| arm | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|
| rho-strong | 0.881 | 0.944 | 1.517 |
| rho-neg | 1.245 | 1.352 | 2.313 |
| k1-l8 | 49.70 | 55.42 | 106.83 |
| k1-l9 | 43.08 | 51.78 | 130.09 |
| ksum4-n12 | 5.75 | 6.49 | 13.21 |
| ksum5-n10 | 6.49 | 7.26 | 14.18 |
| **kf3-c1g** (`ksum-frobenius:m=3`, 1 orbit, generator orbit) | **1.135** | **1.384** | **3.626** |
| kf3-c1 (1 orbit) | 1.384 | 1.703 | 4.575 |
| kf3-c2g / kf3-c2 | 3.03 / 3.08 | 3.77 / 3.82 | 10.41 / 10.51 |
| kf4-c1g / kf4-c1 | 7.04 / 7.09 | 8.37 / 8.44 | 20.34 / 20.52 |
| kf5-c1g, kf4-c2(g) | ~64 | ~76 | ~180-190 |
| kf5-c2(g) | 1285 | 1530 | 3729 |

Band verdicts: kf3-c1g < every K1 arm, < every unfolded ksum arm, < kf3-c1, at all three weights;
rho-strong < kf3-c1g at all three weights; kf3-c1g vs rho-neg unresolved inside the band.

Ratios of the best folded arm (kf3-c1g) to the best K1 arm at each w: 1.135/43.08 = 0.026,
1.384/51.78 = 0.027, 3.626/106.83 = 0.034 (29-38x lower S). Distance remaining to matched strong rho:
1.29x (w = 0), 1.47x (w = 0.1), 2.39x (w = 1).

## Reading

- The winning configuration is the smallest base: one signed Frobenius orbit (46 points) and m = 3,
  table 229 entries, about 515 probes for the single relation needed when column 0 is the generator's own
  orbit. Larger m or more orbits overshoot the optimal table size at r ≈ 2^22 and cost more. This is a
  Frobenius-folded baby-step/giant-step in IC clothing: generic in the sqrt(r/n) class, as the survey
  predicted. It narrows the gap to rho; it cannot cross it (BSGS constant above rho's).
- The S gain over K1 is a real measured improvement over the current K1 index-calculus recipe on this
  curve, but it is not a non-generic lever: it says the K1 IC recipe is far from the generic floor at
  n23, not that IC structure helps.
- Unpriced lower-bound counters remain on the folded arms (factor-base scan items, `hash_probes`,
  `point_negations`, all small in the factor-base phase); the band prices them alike on every arm.

## Recommendation

The pre-set condition holds (folded ksum beats the best K1 recipe at w = 0, 0.1 and 1 in the pilot), so
**freeze a study**: K1 n23 and n29 (the two validated K1 GAE rows), preregistered arms
`compact-orbit-scan:columns=1[,generator_orbit=1]` + `ksum-frobenius:m=3`, plus columns 1-2 × m = 3-4
sizing re-picked per size from this pilot, against the K1 recipe, unfolded ksum and matched
`rho.signed_frobenius_strong`, with the validation gate's replay certificate, validator report and §5a
band. Report it as a generic-class (sqrt(r/n)) improvement of the K1 row, not as a non-generic IC result,
and state the remaining distance to rho. Before the card: research-kb pass, agent-bus note to the K1
lane (ml-cryptanalysis), and the slope at n29 (whether the rho gap stays ~1.3-2.4x).
