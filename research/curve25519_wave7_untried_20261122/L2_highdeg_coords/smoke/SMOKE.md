# L2 smoke test: SMOKE, NOT DATA

**SMOKE, NOT DATA.** The smoke test runs the smallest case (k = 16) with tiny K and
target counts. It checks that the pipeline runs end to end and gives the runtime
estimate. **It decides no prediction.**

- Run: `bash smoke/run_smoke.sh`.
- CPU: about 41 s user (48 s wall), within the 60 s cap.
- Peak RSS: 199 MB.
- Date: 2026-11-22.
- It ran after `PROTOCOL.md` was frozen, and the predictions were not edited afterwards.

## What ran

| step | command (abridged) | CPU |
|:--|:--|--:|
| sumpoly | `sumpoly.py --k 16 --funcs x ye xe v q xeye --verify 200` | 0.4 s |
| census | `census.py --k 16 --t 4 --K 20` (all 10 arms) | 0.4 s |
| macaulay | `macaulay.py --k 16 --funcs x q xeye --T 4 5 --targets 1 --covered 1` | 39 s |
| engine | `engine_calib.sh` with KS = 16, TS = 4, NT = 2 | < 0.1 s |
| analyse | `analyse.py --data smoke --out smoke` | < 1 s |

## Pipeline checks

These are checks of the code, not results.

- **Curve facts** (`curve_facts.json`): every assertion in `Curve._verify` holds on k = 16,
  18 and 20:
  - the exact point count equals 8r, and r is prime;
  - the twist order is 4 × prime;
  - A² − 4 is a non-square, and p ≡ 5 (mod 8);
  - a generator of order exactly 8r exists.
- **PC3**: the interpolated S^x is proportional to the explicit Weierstrass S₃. It passed
  after the overflow fix in PROTOCOL §0, fix 1.
- **Interpolation**: the kernel dimension is 1 for every f run, and 200 of 200 fresh zeros
  vanish.
- **Ŝ_x terms**: 121 at T = 4 and 256 at T = 5, which is C1's form.
- **Census against the polynomial system**: they agree on every smoke target
  (`consistency_census_vs_system`).
- **M2 rank check**: rank V_n = 2^n − |Z| holds wherever M2 reached D = n.
- **Engine**: `f4cost 16 bits 4 0 2 …` runs, and its JSON carries `solving_degree_max` and
  `poly_terms_per_specialisation`.
- **analyse.py** writes `summary.json` and `summary.md`. In the smoke, the k = 20 controls
  (PC1) and the ye, xe and arm-dependent verdicts are absent or meaningless by design.

## Values exposed at k = 16 (recorded for blindness; not evidence)

- **S^f** (per-variable degree / total degree / terms):

  | f | per-variable | total | terms |
  |:--|--:|--:|--:|
  | x | 2 | 4 | 13 |
  | ye | 2 | 6 | 9 |
  | xe | 4 | 12 | 11 |
  | v | 9 | 18 | 315 |
  | q | 9 | 21 | 429 |
  | xeye | 4 | 12 | 11 |

  - The split-fibre counts match the per-variable degrees.
  - The total degrees of xe, q and xeye exceed the frozen P4 bounds. P4-total will be decided
    by the full run as frozen.
- **Ŝ degree e:**
  - x: 4 at T = 4 and 5;
  - q: 8 at T = 4 and 10 at T = 5 (= n);
  - xeye: 6 at T = 4 (P5a predicted 8) and 8 at T = 5.
- **M2 D_ref / D_root** (one target each):
  - x: 6 / 6 at T = 4 and 6 / 6 at T = 5;
  - q: 8 / 8 at T = 4 and 10 / 10 at T = 5;
  - xeye: 7 / 7 at T = 4 and 9 / 9 at T = 5.
- **M1 D:**
  - x: 12 and 14;
  - q: 16 and 20;
  - xeye: 12 and 16.
- **Engine** (x, T = 4, 2 targets): `solving_degree_max` = 6.
- **Census at K = 20, t = 4** (mean covered for f against cols-matched x):

  | arm | f | x |
  |:--|--:|--:|
  | v | 88.0 | 80.8 |
  | q | 67.6 | 69.4 |
  | uv | 67.7 | 70.5 |
  | xeye | 29.7 | 102.6 |
  | jac6 | 22.5 | 18.3 |
  | ye | 14.2 | 16.4 |
  | xe | 39.8 | 72.4 |
  | rand | 66.4 | 69.2 |
  | xAA | 72.6 | 72.4 |
  | even | 138.3 | 68.6 |

## Runtime estimate for the full run

See PROTOCOL §9.

| step | basis | estimate |
|:--|:--|--:|
| macaulay | M2 at n = 10: about 15 s per target for dense f (v, q, uv, jac6), about 4.5 s for x and ye, about 4 s for xe and xeye; T = 4 and T = 6 under 0.5 s each. 13 targets × 8 f per curve | ≈ 1000 s per curve, ≈ 50 min |
| census | 1.8 ms per frame at t = 4; larger t costs more | 240k frames, ≈ 16 min |
| sumpoly | the uv and jac6 kernels dominate (synthetic RREF timing: 1400 × 1330 mod p takes 18 s) | ≈ 12 min |
| engine | | ≈ 2 min |
| **total** | | **≈ 80 min** |

Memory stays under 0.3 GB.
