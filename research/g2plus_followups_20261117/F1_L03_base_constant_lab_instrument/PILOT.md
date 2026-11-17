# F1 pilot (run by the coordinator before freezing)

Everything here is `[pcc:unverified]`. The pilot touched gate quantities and instrument sanity
on the decision primes (instance search; at p = 8209 the cc = 2 seeds 1-2 runs, used only for the
G6 reproduction counts `trials` and `core_cols`), and the full pipeline only on **non-decision
primes 8219 and 32779**. No cc = 1 run was made on a decision curve, so no value of the decision
metric G on a decision instance was seen.

## 0. Files and hashes

| file | sha256 |
|---|---|
| binary `/home/user/crypto-c0b657b/target/release/genus_ladder` | `50e5986e8e80ee98d7d80c9cbedc169f7b732c84d571bee9b74acbd123a030ff` |
| `ref/instances_L03.jsonl` (copy of L03 `instances.jsonl`, the only file taken from yesterday's leads; no code copied) | `91ddea89649a852f9b8c73e6a7affa4e2adde701624db0700e282f0846517148` |
| `f1.py` | see `frozen.sha256` companion line in §7 |
| `tools/rt.py` = `pilot/rt.py` | `875de0d5f6781b92f7cf9f53fa9eff768c460f888ae25e1be0c0e36f34f485d8` |
| `pilot/pilot_gates.py` | `3d853652ce88e37712f4245aee94e07d267a4abac6aa1716772c58908e831a3c` |

`/usr/bin/time` is not installed; `rt.py` (Python `resource.getrusage(RUSAGE_CHILDREN)`) records
CPU seconds and peak RSS instead. No hook refused any pilot command.

## 1. G1: instance search reproduces the nine curves

Commands (search only, `--seeds 0`, so no solver run):

```
$B run --primes P --genera 3 --curves 3 --seeds 0 --arms c --cc 2.0 --max-h 8 --threads 1 \
   --rho-cap 3e7 --max-trials 2e8 --seed-search 600 --out pilot/search_pP.jsonl
```

| p | curve seeds admitted | equal to `ref/instances_L03.jsonl` (f, order, h, r) | search CPU-s | peak RSS |
|---|---|---|---|---|
| 8209 | 6, 18, 23 (23 tried) | yes | 87.6 | 13 MB |
| 16411 | 8, 27, 43 (43 tried) | yes | 643.6 | 13 MB |
| 32771 | SEARCH32771 | SEARCH32771_EQ | SEARCH32771_CPU | SEARCH32771_RSS |

## 2. G6: this build reproduces the scale study's counts (p = 8209, cc = 2, seeds 1-2)

`$B run --primes 8209 --genera 3 --curves 3 --seeds 2 --arms c --cc 2.0 … --out pilot/g5_full_p8209_cc2.jsonl`
(87.4 CPU-s, almost all of it the search). Only `trials` and `core_cols` were read:

| curve | seed | trials | core_cols | certified, ok, attempts |
|---|---|---|---|---|
| 6 | 1 | **42975** | 741 | true, true, 1 |
| 6 | 2 | 44577 | 740 | true, true, 1 |
| 18 | 1 | 43863 | 716 | true, true, 1 |
| 18 | 2 | 43224 | 751 | true, true, 1 |
| 23 | 1 | 44984 | 750 | true, true, 1 |
| 23 | 2 | 44160 | 748 | true, true, 1 |

Mean trials 43963.83 (scale study as quoted in L03 Q7: 43964); mean core_cols 741.0 (L03 Q7:
741.0); (curve 6, seed 1) trials 42975 (L03 ASSESSMENT item 4: 42975). **G6 passes.** So the
scratch build at `a552dc2ee8` counts exactly as the scale study's binary `61ccabd2…` did on these
runs, and `--threads 1` gives the counts the 4-thread study gave.

## 3. G2, G3, G4, G5 on single runs at p = 8219 (non-decision)

`--primes 8219 --curves 1 --seeds 1 --arms cR`, cc = 0.5 and 3.0 (12 CPU-s each):

| cc | fb_cols (expected) | trials | la_mul | core rows / cols | attempts | certified, ok |
|---|---|---|---|---|---|---|
| 0.5 | 204 (204) | 134137 | 589997 | 212 / 202 | 1 | true, true |
| 3.0 | 1222 (1222) | 33682 | 14185784 | 1124 / 1055 | 1 | true, true |

- G2: `fb_cols = ceil(cc · 8219^{2/3})` at both ends. The cc = 0.5 core sits exactly at the
  margin (212 = 202 + 8 + 2), and still certifies on the first attempt.
- G4: an identical re-run of the cc = 0.5 command (`pilot/pilot_p8219_cc0.5_rerun.jsonl`) is
  identical on every field except `wall_ns`/`wall_s`/`search_wall_s`: **True**.
- G5: the rho rows of the cc = 0.5 and cc = 3.0 processes are identical: **True**.

## 4. Full dress rehearsal at p = 8219 (non-decision): every gate through `f1.py`

The study's exact process layout, `--curves 3 --seeds 8`, files `pilot/dress8219/p8219_cc1.0_main`,
`p8219_cc2.0_main` (`--arms cR`) and `p8219_cc2.0_rerun` (`--arms c`). CPU-s: 101.8, 103.1, 90.9
(about 90 of each is the instance search); peak RSS 13 MB; files 21 KB, 21 KB, 15 KB.
`python3 pilot/pilot_gates.py pilot/dress8219` (calls `f1.gates` with this prime's own instances
as reference and G6 off) printed:

```
gates: {"G0_files_present": true, "G1_instances": true, "G2_cc_honoured": true, "G3_certified_complete": true, "G4_determinism": true, "G5_rho_independent_of_cc": true, "G7_tradeoff_visible": true, "G8_ops_identity": true, "G9_aa_noise_floor": true} all_pass True
curves (3, 18, 21) pairs 24 attempts max 1
G lo ['-0.2888', '-0.2972', '-0.2805']
G mid ['0.3612', '0.3550', '0.3670']
G hi ['0.3099', '0.3035', '0.3160']
G_AA mid ['0.0084', '-0.0075', '0.0241']
cc1 mean trials 72313 LA share mid 0.339 IC/rho mid 0.539
cc2 mean trials 43864 LA share mid 0.743 IC/rho mid 0.843
```

(Triples are point, 95% lo, 95% hi.) All 48 IC runs and 48 rho runs certified on the first
attempt. G7 (more trials, fewer `la_mul` at cc = 1) and G9 (A/A 0.0084, limit 0.05) pass.

## 5. What the dress rehearsal changed in the design (before freezing)

1. **Band dependence.** At 8219 the gain is +0.36 at mid and **−0.29 at lo**: with kappa = 1 the
   relation phase dominates and the larger base wins. The decision stays on the instrument's
   primary band (mid), unchanged, but PROTOCOL §8 now makes a band label mandatory in the
   verdict, so that a mid-band "supported" cannot be read as unconditional.
2. **A/A gate G9 added** (|G_AA_mid| ≤ 0.05), after seeing it pass at 0.0084 on the 8219 rehearsal.
3. Seeds raised from the scale study's 2 to 8 per curve: solver runs cost seconds once the
   search is done; the search dominates.

## 6. Largest size: G3 at p = 32779 (non-decision), both ends of the sweep

`--primes 32779 --curves 1 --seeds 2 --arms cR`, cc = 0.5 and 3.0:

P32779_TABLE

## 7. CPU budget for the study

Per process the instance search dominates: 88 s at 8209, 644 s at 16411, SEARCH32771_CPU s at
32771; solver runs add seconds to a few minutes (P32779_RUNCPU). Study total ≈ 6 processes per
prime: about BUDGET_TOTAL CPU-hours, of which p = 32771 is the bulk. **Each p = 32771 process
exceeds the brief's "about 15 CPU-minutes"** because the search cannot be skipped or shared
across `--cc` values in this binary; PROTOCOL §9 declares this. Memory: 13 MB peak.
