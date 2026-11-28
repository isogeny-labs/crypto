# Validator report: genus-4 follow-up (`research/genus4_followup_20261118`)

- **Disposition: validated** (`validator-report.json`: `status: pass`, `independent: true`).
- **Validated commit:** `6213ecbc0da0ed4917ab75805258201d6629b477` on `claude/hyperelliptic-index-calculus-td1fdn` (PR isogeny-labs/crypto#4693).
  Every raw row was read as a git blob at that commit. The working tree was clean apart from untracked, ignored `.jsonl`/`.log` copies, which were not used.
- **Freeze:** `ae36cdc6e4`.
- **Validator:** a Validator subagent, 2026-11-18. The recompute, the certificates and the replay comparison use the validator's own code (`research/genus4_followup_20261118/validation/scripts/`). The producer's `analyze.py`, `select.py` and `sensitivity.py` were read but never imported or run.
- **Standing:** every number in the study is still `[pcc:unverified]` until a pcc claim records it. This report checks the numbers. It does not issue a claim.

Machine-readable: `validator-report.json` (sha256 `8df892413ddd3628c0b811e1fc169b5871224ada8dc591fd496224345e718d89`), `replay-certificate.json` (sha256 `e4df255739e12d5c729b159e4e606ab0fc56750a3ae3d5b427a815a280c2912d`), `VALIDATION.json`; `python3 scripts/validation_gate.py evidence benchmarks/20261118-genus4-followup` says `valid`. Working files: `research/genus4_followup_20261118/validation/` (`scripts/`, `out/`). Paths below starting `validation/` are under `research/genus4_followup_20261118/`.

## Verdict in one paragraph

Every number in `analysis.json`, `sensitivity.json` and `select.json` recomputes from the committed raw rows. The match holds to at most 2e-16 relative difference over 104 cells, bootstrap intervals included, with the declared design and seed. The gates hold with real keys. Four further facts check:

- The frozen files hash to `frozen.sha256` and predate the data.
- The rebuilt binary has the producer's code.
- 46 nonce-drawn replayed run rows are identical in every field except `wall_ns`.
- The validator's own Jacobian arithmetic certifies all 54 instances and 33 recovered logarithms.

The frozen decision "arm x is below rho at w = 1" is met at the frozen seed by a margin smaller than Monte Carlo noise. It holds only against the measured plain rho, in Q3. On the two h = 2 curves arm x is at parity with the plain-rho expectation, so the result rests on the one prime-order Jacobian. Against the strongest available reference, the ideal negation floor, the §5a band does not keep one order: arm x is below at w = 0 and 0.1 and 1.240× at w = 1. RESULTS.md says this, except for the per-curve decomposition (F7). Two small factual errors need errata (F1, F2).

## Checks

| id | requirement | outcome | evidence |
|---|---|---|---|
| C1 | `PROTOCOL.md`, `run.sh`, `select.py`, `analyze.py` hash to `frozen.sha256` at the freeze commit and at the validated commit. `frozen.sha256` and `binary.sha256` are unchanged since the freeze. The freeze commit has no data, and its parent has no study files. Part A data came in `f4853db923` and part B data in `6213ecbc0d`, both later. No `src/` or `Cargo.*` change after the freeze. The freeze commit's only source change is `--target-from` (9 lines of `src/bin/genus_ladder.rs`) | pass | `validation/out/freeze_check.json` |
| C2 | Producer binary `74f02ea1…` (hashed at dispatch, before any validator build) equals `binary.sha256`. Validator rebuild from `git archive ae36cdc6e4` (5,521 paths) in a validator-owned tree and target directory: `90128561…`. `.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame`, `.gcc_except_table` and `.got` are byte-identical, and all 57,424 symbol addresses and types are equal. Only the GNU build-id and the `.llvm.<hash>` suffix of 35 promoted local symbols differ: a build-path difference, as the earlier genus-4 validator found | pass (the hash does not reproduce across build paths; the code does) | `validation/out/binary_diff.txt`, `validation/out/archive_paths.sha256.txt` |
| C3 | Replay of a nonce-drawn subset with the validator build. Nonce `40984f0a…` (`validation/out/nonce.txt`), plan `validation/out/plan.json`. Every replayed row equals its committed row in every field except `wall_ns` | pass: 46/46 run rows and 16/16 instance rows identical: 16 arm-x and 16 arm-w rows at p = 16411 (8 cells, all 3 curves); 10 part-A arm-x rows at p = 8209, one cell at each cc = 0.5, 0.7, 1.0, 1.4, 2.0; 4 Q rows: (172, k = 1, s 1) from the lean-rho study, (180, k = 7, s 1 and s 2) from `b_q`, (2640, k = 9, s 1) from the resumed block `b_q.jsonl.r1` | `validation/out/replay_compare.json`, `validation/out/replay/`, `validation/scripts/replay_ic.sh`, `replay_q.sh` |
| C4 | Independent certificates, validator arithmetic (`jac.py`, reviewed against the Rust at the freeze commit). For the 3 curves: f matches the row; f is monic of degree 9 and squarefree; r is prime; order = h·r; h·r lies inside the exact Hasse–Weil interval and r is larger than its width, so #J = h·r; [order]D = 0 for 6 random divisors. The only F_p-automorphisms are the identity and the hyperelliptic involution: an affine scan, plus a scan of maps swapping ∞ with a rational root (curve 180 has one, x = 12246). Positive controls detect planted automorphisms. For all 54 targets: G re-derived from the seed, G ≠ 0, [r]G = 0, T = [t]G, checked again by a second addition chain | pass (3/3 curves, 54/54 targets) | `validation/out/certs.json`, `validation/scripts/certs.py`, `validation/out/automorphism_selftest.txt` |
| C5 | The producer's library (`val_fudump`, built after `genus_ladder` from the same tree, library reused) prints f, G, T, t for all 54 targets: every value equals the validator's derivation. 32 IC runs (16 x, 16 w; the 8 replay cells) and 1 Q run (172, k = 1, seed 1, a lean-rho row) print their recovered log: it equals t, [recovered]G = T holds in the validator's arithmetic, and the counts equal the committed row. A tamper control (one log +1, one count +1, one T coefficient, one t) is caught 4/4 | pass (54/54 instances, 33/33 logs) | `validation/out/verify_dump_main.json`, `verify_dump_rho.json`, `verify_dump_tamper_control.txt` |
| C6 | Headline numbers recomputed by the validator's own code from the raw rows: ratios of means, the PROTOCOL §6 bootstrap (targets then seeds, 2000 resamples, seed 20261123), part-A selection, post-hoc theory ratios, 50-seed stability. 104 cells compared with `analysis.json` and `sensitivity.json`: 82 exact, the rest ≤ 1.9e-16 (summation order) | pass | `validation/out/recompute.json`, `compare_claims.json` |
| C7 | G1: 216/216 IC runs certified, ok and not exhausted, and 108/108 Q runs certified and ok. G2: 36/36 cheap-IC rows equal in the 7 named fields and in every field except `wall_ns`. All 7 fields exist in every compared row (indexed with `[]`), so `analyze.py`'s `.get` comparison was not vacuous. G3: the re-run row (172, 172, 1) equals the lean row in every field. Resume duplicates: 2 keys, (2640, k = 4, s 1) and (2640, k = 3, s 2), identical in every field except `wall_ns`. Key sets: w, x and Q each cover exactly the 108 keys; old Q is k = 0..2 and new Q is k = 3..17, with no overlap | pass | `validation/out/recompute.json` (`checks`) |
| C8 | §5a band against the strongest reference, every listed unpriced counter shown | pass, with F3 | `validation/out/band.json` |
| C9 | Claims of `RESULTS.md`, caveats included (below) | pass, with findings F1–F7 | this file |

## Headline numbers (p = 16411, 54 targets, 108 keys per arm; recomputed = claimed to ≤ 2e-16)

| reading | arm x / rho | arm w / rho |
|---|---|---|
| Q3, w = 1, measured Q | 0.87395 [0.76659, 0.99960] | 1.60530 [1.40698, 1.83734] |
| Q4, w = 1, measured Q | 0.21850 [0.19166, 0.24991] | 0.40134 [0.35176, 0.45936] |
| Q3, w = 0.1 | 0.08881 [0.07790, 0.10158] | 0.16194 [0.14194, 0.18535] |
| Q4, w = 0.1 | 0.06831 [0.05992, 0.07814] | 0.12457 |
| w = 0 (Q3 = Q4) | 0.0015685 | 0.0015681 |
| new targets only (Q3, w = 1) | 0.87348 [0.76166, 1.01349] | 1.60392 |
| curve-level bootstrap (Q3, w = 1) | [0.67133, 1.15045] | [1.22904, 2.11809] |
| vs √(πr/2), w = 1 (post hoc) | 0.87714 [0.83790, 0.91989] | 1.61115 |
| vs ideal negation √(πr/4), w = 1 (post hoc) | 1.24046 [1.18497, 1.30091] | 2.27851 [2.17534, 2.39381] |
| vs ideal negation, w = 0.1 (post hoc) | 0.12605 [0.12041, 0.13220] | 0.22985 |
| Q3 w = 1 upper end over bootstrap seeds 1–50 | min 0.99400, median 1.00296, max 1.01229, below 1 for 18/50 | min 1.8255, max 1.8588, 0/50 |

Measured Q mean / √(πr/2) is 1.00365. Arm x mean cost at w = 1 is 2.3823e8 GAE, with LA 76.6% of native work; arm w's is 4.3759e8.

Part A (p = 8209, arm x, 36 keys per cc, all certified and correct). Mean cost at w = 1 is 3.0613e8, 1.8239e8, 1.1783e8, 9.3901e7 and 9.0896e7 for cc = 0.5, 0.7, 1.0, 1.4 and 2.0 (nb = 432, 604, 863, 1208, 1725). The ratios to cc = 2.0 are 3.368, 2.007, 1.296, 1.033 and 1.000. The frozen rule (`select.py`) selects **cc\* = 2.0**, at the edge of the grid. Trials scale as nb^−1.396 (cc 1.0→2.0) and nb^−1.662 (0.5→1.0). LA is 60.6% of native work at cc = 2.0.

## Rho reference and the §5a band

**Which reference.** All three curves have only the identity and the hyperelliptic involution as F_p-automorphisms, and Frobenius is the identity on J(F_p). So **negation (A = 2) is the strongest available symmetry**. The strongest reference that can be justified is the **ideal negation floor √(πr/4)** group operations, with nothing charged. Every negation walk costs more:

- The lab's measured negation walk B (lean-rho study, k = 0..2) takes 1.85× √(πr/4) steps, with 3.00 counted compares per step.
- Plain Q costs 1.42× the floor in group operations alone.

VALIDATION.json uses that floor. Band, mean GAE per key, arm x:

| w | arm x | negation floor √(πr/4) | arm x / floor | measured Q, Q3 | measured Q, Q4 |
|---|---|---|---|---|---|
| 0 | 4.2757e5 | 1.9205e8 | 0.0022 | 2.7259e8 (0.0016) | 2.7259e8 (0.0016) |
| 0.1 | 2.4208e7 | 1.9205e8 | 0.1261 | 2.7259e8 (0.0888) | 3.5436e8 (0.0683) |
| 1 | 2.3823e8 | 1.9205e8 | **1.2405** | 2.7259e8 (0.8740) | 1.0903e9 (0.2185) |

**Order across the band against the negation floor: does not hold** (below, below, above). Arm x is not ranked against negation rho. Arm w is the same: 0.0022, 0.2299, 2.2785.

What changes the w = 1 order is how rho's per-step native work is priced:

- The floor charges none.
- A negation walk at the ideal step count, charged its minimal per-step work, puts arm x below at every weight. With one canonicalisation per step arm x is at 0.620 of it at w = 1. With a hash and two coefficient additions as well it is at 0.248. These are models, not measurements: no measured negation walk here reaches the ideal step count.

**Is plain rho Q a defensible measured reference?** Yes, as the measured reference.

- **Cheapest measured rho.** On the 18 keys at k = 0..2, B costs 1.36×, 1.89× and 6.64× Q at w = 0, 0.1 and 1 (Q3), and 2.64× Q at w = 1 (Q4). Q is the cheapest measured rho at every weight in both readings.
- **Pricing.** Q's walk does no canonicalisation (`PointClasses::trivial`), so pricing its C at 0 is right; the `native` field is only the one-per-step floor (lesson L-527c1f013131). Q has no window: `compares_uncharged` is 0 on all 108 keys.
- **Not the strongest available.** That is the negation floor, and the claim "below rho" must name Q.
- **Two binaries.** Q for k = 0..2 ran in the lean-rho binary. The validator's rebuild of the frozen source reproduces those rows: the G3 key, plus the nonce-drawn key (172, k = 1, s 1) in C3. Its recovered log is certified in C5.

**Unpriced counters, every arm.**

- IC: `rel_mul`, `rel_add`, `la_mul`, `la_add` (priced). The walk's per-step partition hash and two coefficient additions are not emitted (F3).
- Q: `compares_uncharged` = 0. `hash_uncharged` = steps and `coef_adds_uncharged` = 2·steps, priced in Q4 only. Canonicalisations are 0.
- Negation floor: none.

## The claims of RESULTS.md

1. **"Below rho at w = 1", and its fragility.** The frozen rule (Q3 and Q4 upper ends < 1) is met: 0.99960 and 0.24991. RESULTS states the fragility correctly and prominently: "That verdict is not robust", the 50-seed spread 0.994–1.012 with 36% below 1, the new-targets-only 1.013 and the curve-level 1.150 upper ends, all of which recompute. The validator adds three points:
   - An equally valid resampling order (draw a target, then its seeds) gives 0.99946.
   - Interpolated quantiles give 0.99962.
   - The verdict rests on curve 2640 (F7).

   The P1 scorecard ("above 1 for 64% of seeds") recomputes: 32/50.
2. **Dropping "Ferrari inside the sieve".** Justified by `src/cryptanalysis/genus_ladder_sieve.rs` at the freeze commit. The module header says "There is no splitting test". `extract` finds the roots of R_t from the counter-array hits and reads a degree-1 cofactor as `e1 − Σ hits` (lines ~316–372). Only lp = 2 uses a quadratic discriminant from a table. At g = 4, `lp_eff = 1` because 2g − 1 = 7 > `REL_FB` = 5 (line ~516). A family's g fixed places enter through one family vertex, so a relation carries 2g + 1 places. R_t has degree g + 1 = 5, and no quartic splitting decision exists for Ferrari to replace. The "1.4–1.9× arm w's LA" figure is the cheap-IC study's 1.410 / 1.938 (its RESULTS table). The change was made in the frozen protocol before any data.
3. **Q as reference.** Defensible as the measured reference, not as the strongest (above).
4. **The `rho_reference` block.** `kind: negation`, the ideal negation floor, with the band as tabulated and `order_holds_across_band: false`. `same_binary_or_process: true` is stated on this basis: the floor is a closed form of r of instances emitted by the same binary and certified independently, so no second implementation's operation counts enter.
5. **Other prose.** All of these recompute:
   - Part A table, nb^−1.4 and nb^−1.66, LA share 76% at p = 16411 against 61% at p = 8209.
   - Mean cost 2.38e8 against a mean √(πr/2) of 2.72e8; r ≈ 7.3e16 (h = 1) and 3.6e16 (h = 2).
   - Q mean 1.004× √(πr/2); "1.24× the ideal negation bound"; "0.126× at w = 0.1"; arm w 1.605 [1.407, 1.837] and 2.28×.
   - The 9-target 0.876 (cheap-IC `analysis.json` 0.87632).
   - The genus-3 L03 figures (36–40% at cc = 1.0, slope −0.81 to −0.84).
   - The deviations: 62 of 90 Q rows before the kill, and `resume.sh` output read by the frozen glob.

## Findings

- **F1 (minor, prose; needs an erratum).** PROTOCOL §7 P1 says the 9-target Q mean was "about 14% above √(πr/2)", and RESULTS repeats "because the 9-target rho mean was high". The 18 Q rows at k = 0..2 give 1.0009 (ratio of means) and 0.976 (mean of per-run ratios). No committed file supports 14%. The conclusion RESULTS draws, that the old ratio was not an unlucky rho sample, is consistent with the data. `PROTOCOL.md` is frozen, so the correction belongs in an `ERRATA.md` (lesson L-8d86e8fc352c).
- **F2 (minor, record; needs a correction).** In `benchmarks/records/20261118T0710Z-genus4-followup.json`, metric `w_over_plain_rho_q3_w1_p16411` is 1.6046 [1.4069, 1.8371]. `analysis.json` and the recompute give 1.6053 [1.4070, 1.8373], and no computation tried reproduces 1.6046. The record is also still open: `status: open`, `closed_utc` null, `commit_dirty: true`, `reports` empty. Its raw entries carry the right SHA-256 but `uploaded: false` (no S3 transport, disclosed).
- **F3 (minor, pricing symmetry).** Q4 charges rho one hash and two coefficient additions per step. The IC walk does the same per-step work (`genus_ladder.rs` ~1508–1512: `mix64` of the key and two `addm`) and is not charged for it. Charging 3·trials moves arm x Q4 from 0.2185 to 0.2197 at w = 1 and from 0.0683 to 0.0687 at w = 0.1. No verdict changes. Q3 is symmetric.
- **F4 (interpretation).** See claim 1. "Below rho" must name the reference (measured plain Q) and the reading (Q3 at Monte Carlo margin; Q4 robust). Against negation, the order does not hold across the band.
- **F5 (note).** `resume.sh`'s statement that it was written before any Q row was read cannot be verified from git, because the rows were on disk uncommitted. Consistent with it: the 2 re-computed keys are identical, and a resumed-block key (2640, k = 9) replays identically.
- **F6 (note).** The headline row "w = 0.1, measured rho 0.089" does not name its reading. It is Q3; Q4 gives 0.068.
- **F7 (interpretation; the most consequential).** Arm x's cost hardly depends on r: its coefficient of variation over the 108 keys is 1.2%, while rho scales as √r. Per curve at w = 1:

  | curve (h) | arm x / √(πr/2) | arm x / negation floor | arm x / measured Q (Q3) |
  |---|---|---|---|
  | 172 (h = 2) | 0.998 | 1.412 | 1.150 |
  | 180 (h = 2) | 0.998 | 1.412 | 0.907 |
  | 2640 (h = 1) | 0.707 | 1.000 | 0.685 |

  The pooled "below plain rho at w = 1" therefore rests on the one prime-order Jacobian. On the two cofactor-2 curves arm x is at parity with the plain-rho expectation. The paired per-key view agrees: the median per-key x/Q is 0.935, but the mean per-key ratio is 1.265, and arm x is cheaper on 56 of 108 keys. RESULTS gives h and r per curve but not this decomposition. The preregistered curve-level bootstrap [0.671, 1.150] is its trace.

## Reproducibility

- **Host.** 4-core Intel Xeon @ 2.30 GHz, Linux 6.18.44.
- **Toolchain.** rustc 1.97.0 (2d8144b78 2026-07-07), cargo 1.97.0.
- **Build.** `cargo build --release --locked --offline --bin genus_ladder` in `…/scratchpad/fuval/src_ae36cdc6e4`, with a validator-owned `CARGO_TARGET_DIR`. Sources were touched first, the log shows `Compiling crypto v0.1.0 (<that tree>)`, and the build took 9 m 55 s. No other `crypto_lib` compile was running.
- **Dumper.** `val_fudump` (sha256 `f2c37e92…`) was built afterwards in 1.1 s: `rustc --crate-name val_fudump` only, with the library reused.
- **Replay commands.** `validation/scripts/replay_ic.sh` and `replay_q.sh`, with the same flags as `run.sh` and `resume.sh`, one cell per invocation (`--curves 1 --curve-seeds p=cs --targets 1 --target-from k`).
- **Not done.**
  - No replay with the producer's binary: its code sections equal the rebuild's.
  - S3 copies not checked: none were uploaded.
  - Not on isolab-eks: counts are deterministic, and the producer's local host is a disclosed deviation.
  - No `ecbench band` run: these are `genus_ladder` rows, not ecbench sessions. The band was recomputed with the same rule (every listed counter priced), lesson L-546d37ca7750.
- **Lessons applied.** L-546d37ca7750, L-99ee952350b2, L-527c1f013131, L-7f8877029be4, L-ce5e4e4b764b, L-ca1840e60dc0, L-b0626ec2a4c0, L-3f39e1d8b3a9, L-26895912385b, L-e33c4f6ece95.
