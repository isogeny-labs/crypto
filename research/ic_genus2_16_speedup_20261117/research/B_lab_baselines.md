# B. Lab baselines for index calculus on genus 2..16 (survey, 2026-11-17)

Status: complete; sections were appended in reading order (1, 1.1b and 1.2 are instruments; 2.x measurements).
Sources: pre-deletion tree `c0b657b6d5` (read via git show), current tree.
Citations: all numbers below are lab measurements quoted from files; none were
re-run here, so each is `[pcc:unverified]` unless a pcc claim id is quoted.

## Outline
1. IC instruments for genus >= 2
2. Measured baselines per genus
3. Tried and closed
4. Cost unit and matched rho
5. Gaps (genera 2..16 without instrument/measurement)
6. Summary table

## 1. IC instruments for genus >= 2

### 1.1 genus_ladder (the lab's main cross-genus instrument)

Files: `c0b657b6d5:src/cryptanalysis/genus_ladder.rs` (2536 lines, deleted on
main by 27b3d06b51), `c0b657b6d5:src/bin/genus_ladder.rs` (559 lines),
`c0b657b6d5:src/bin/genus_ladder_analyze.rs`,
`c0b657b6d5:src/cryptanalysis/genus_ladder_harvest.rs` +
`src/bin/genus_ladder_harvest.rs`, plane-quartic arm in
`c0b657b6d5:src/cryptanalysis/quartic_ladder.rs`. Protocol
`research/genus_ladder_20261114/PROTOCOL.md` (frozen at commit `6e937a2a60`).

- **Curves:** random imaginary hyperelliptic `y^2 = f(x)`, `deg f = 2g+1`,
  `g in {2,3,4,5}` (`MAX_GENUS = 5`, fixed arrays `CAP = 24`), all over one
  `F_p`; odd prime `p < 2^31` with `(sqrt p + 1)^{2g} < 2^63`; admitted only if
  `#J = h r`, `r` prime, `h <= --max-h` (default 64; the 1114 grid used h <= 8).
  Exact `#J` computed (tested against brute-force zeta at g = 2,3,4).
- **Group law:** generic Cantor by extended gcd (not explicit formulas);
  `JCurve` tallies every `F_p` mul and inv. Measured cost per Cantor step:
  90, 219, 393, 693 field mults at g = 2,3,4,5 (RESULTS.md 1114 sec. 1).
- **Arms** (`enum Arm`): `Full` ("A_full", factor base = every degree-1 place,
  0 large primes) and `DoubleLp` ("C_double_lp", reduced base, <= 2 large
  primes). CLI letters: `A` full, `a` full+prefilter, `C` 2LP, `c` 2LP+prefilter,
  `R` negation-map rho. `--quartic` adds three plane-quartic arms
  (Diem line relations full/2LP, and `Q_rho_neg`) from `quartic_ladder.rs`.
  `--smith` / `--smith-census N` runs Smith's Prop. 2/4 test
  (`smith_prop24::analyze_curve`) on genus-3 curves.
- **Factor base:** abscissae `x` with `f(x)` a non-zero square (one column per
  pair `(x, +-y)`, sign `eps` from `y > (p-1)/2`), taken in increasing `x`
  until `nb` columns. 2LP size `nb = ceil(c * p^{1-1/g})`, capped at p
  (`fb_size_2lp`, "Gaudry, Thome, Theriault, Diem"); `--cc` default 2.0 for
  hyperelliptic, `--qc` default 3.0 for quartics.
- **How relations are found:** a random walk `R <- R + M_j` with 16 jumps
  `M_j = a_j G + b_j T` (short coefficients < 2^12), index `mix64(key) % 16`;
  restart every `RESTART = 4096` steps with fresh `aG + bT` (amendment flag
  `--restart-sqrt`: `min(4096, max(16, floor(sqrt(r)/2)))`). Each walk element
  of full degree `g` is a candidate. No sieving, no single-large-prime arm.
- **Split test:** optional parity prefilter `chi(disc u) = (-1)^{d-k}`
  (Pellet/Stickelberger) via pseudo-remainder chain + table look-up, decides
  outright at g = 2; then `splits_completely` (`x^p mod u` test), then
  `roots_of` (randomised root finding); evaluate `v` at each root; y = 0
  places dropped (2-torsion). Relation kept if #large primes <= `max_lp`.
- **Large primes:** large-prime vertices in a union-find (`Dsu`) graph;
  need cycles >= `nunk + 8 + nunk/100` before LA (2LP arm).
- **Linear algebra:** shared code of the genus-3 positive control
  (`chained_transfer::ic_control`: `build_rows`, `peel`, `wiedemann`) mod r;
  optional `--sge W,F` structured Gaussian elimination (`sge_reduce`, columns
  of weight 2..W merged while nnz <= F x core) between peel and Wiedemann.
  Every log certified twice (`[t]G = T` by two independent multiplications).
- **Rho:** `run_rho` = `rho_reference_walk(..., RhoWalk::negation())`, the
  lab's negation-map rho, max steps `60 sqrt(r) + 10000`, `--rho-cap 3e7`.
- **Cost formula (primary FPM reading):**
  `cantor_mul + 10 * cantor_inv + rel_mul + g^2 * la_mul`
  (RESULTS 1114, fig caption); i.e. inversion priced I = 10 mults and an
  LA multiplication mod r priced kappa = g^2 (r ~ p^g). Secondary: GAE band,
  `w = 0, 0.1, 1` per native mult (AGENTS.md sec. 5a).
- **CLI:** `genus_ladder run --primes P,.. --genera 2,3,4,5 --curves 4
  --seeds 3 [--arms AaCcR] [--cc 2.0] [--max-h 64] [--threads 4]
  [--rho-cap 3e7] [--seed-search 600] [--max-trials 5e7] [--smith]
  [--smith-census N] [--quartic] [--qc 3.0] [--restart-sqrt] [--sge W,F]...
  --out FILE.jsonl`.
- Self-declared limits (RESULTS 1114 "Limits"): generic Cantor not explicit
  formulas; Smith map not built; non-hyperelliptic genus 4/5 not covered;
  single large prime and sieving not implemented.

## 2. Measured baselines (genus_ladder family first)

All `[pcc:unverified]` as the source files themselves state (shared 4-vCPU
host, no independent replay, no pcc claim). FPM = F_p-multiplication
equivalents, `cantor_mul + 10 inv + rel_mul + g^2 la_mul`.

### 2.1 genus_ladder_20261114 (`c0b657b6d5:research/genus_ladder_20261114/RESULTS.md`)

1,809 runs, all certified; 7 primes 67..4099 (genus 5 to 1031); 4 curves x 3
seeds; h <= 8. At p = 1031 (FPM, 95% CI):

| kind | ln r | best IC arm | rho (neg.) | IC/rho |
|---|--:|--:|--:|--:|
| hyperelliptic g=2 | 12.3 | 2.74e5 (C+pre) | 9.74e4 | 2.82 [2.09, 4.03] |
| hyperelliptic g=3 | 20.1 | 5.50e6 (C+pre) | 7.32e6 | 0.75 [0.46, 1.40] |
| hyperelliptic g=4 | 26.9 | 3.32e7 (C+pre) | 2.84e8 | 0.117 [0.077, 0.216] |
| hyperelliptic g=5 | 33.6 | 1.50e8 (C+pre) | 1.80e10 (3 curves) | 0.0084 [0.0047, 0.020] |
| plane quartic g=3 | 19.5 | 4.58e6 (2LP) | 1.72e8 | 0.027 [0.019, 0.037] |

Fitted exponents in p (FPM; 7 primes, 5 at g=5):

| | rho | IC full | IC 2LP | theory rho/2LP |
|---|--:|--:|--:|---|
| g=2 | 0.77 [0.66,0.87] | 1.45 | 0.50 [0.37,0.59] | 1 / 1 |
| g=3 | 1.47 | 1.59 | 1.08 [1.04,1.11] | 1.5 / 1.33 |
| g=4 | 1.90 | 1.55 | 1.29 [1.27,1.30] | 2 / 1.5 |
| g=5 | 2.69 | 1.03 | 1.02 [0.93,1.12] | 2.5 / 1.6 |
| quartic | 1.36 | 1.67 | 0.69 [0.63,0.76] | 1.5 / 1 |

Exponents below theory are set-up dominated (stated in the file). Other
findings: g=4 at p=4099 IC/rho 0.064 [0.033, 0.135]; g=5 at 1031 in the GAE
band 0.0030 / 0.090 / 0.48 [0.27, 1.14] at w = 0 / 0.1 / 1, i.e. **no cell is
below rho across the whole GAE band**. At p=4099 quartic best arm 1.33e7 vs
hyperelliptic g=3 3.32e7 FPM; quartic rho ~8,900 field mults per Pic addition
vs 219 per Cantor step (44x dearer rho). LA share of best arm at p=1031: about
two thirds at g=3, half at g=5; Cantor arithmetic about a fifth to a third.
Split rate = `C(n_x, g) 2^g / #J` (obs/model 0.967..1.020 for p >= 257);
trials scale as `n_x^{1-g}`. Parity prefilter: identical counts, `rel_mul`
x0.60..0.76, whole-log saving 1-9% FPM. Smith census share 0.169 (2,364/14,000);
only 1/28 admitted near-prime curves pass (tail ~0.04, flagged as a lead).

### 2.2 genus_ladder_scale_20261115 (`c0b657b6d5:research/genus_ladder_scale_20261115/RESULTS.md`)

234 runs, all certified. Bench record `benchmarks/records/20261115T0317Z-genus-ladder-scale.json`.

- **g=2, p = 8209 .. 16,777,259 (2^24):** exponents rho 0.975 [0.908, 1.037],
  2LP+pre 1.004 [0.991, 1.020], full base 1.97. 2LP/rho ratios 3.3, 2.0, 2.2,
  4.6, 3.9, 5.7, 3.0, 3.7, 3.6, 4.6, 1.7, 5.1 (8209 .. 2^24), always > 1, no
  trend. **LA share 0.08-0.20.** Cost is the relation walk (one Cantor step per
  trial, half the trials split, each split one edge of a ~p/2-vertex LP graph).
  "Nothing here suggests a crossing at any size."
- **g=3:**

| p | 2LP+pre FPM | rho FPM | 2LP/rho [95%] | LA share |
|--:|--:|--:|--:|--:|
| 8209 | 8.42e7 | 1.43e8 | 0.591 [0.437, 0.870] | 0.75 |
| 16411 | 2.14e8 | 2.89e8 | 0.741 [0.492, 1.42] | 0.76 |
| 32771 | 5.42e8 | 1.01e9 | 0.537 [0.407, 0.686] | 0.77 |

  Exponents 2LP+pre 1.344 [1.334, 1.358] (theory 4/3), rho 1.413 [1.145, 1.751].
  GAE band unresolved at every prime (above at w = 1).

### 2.3 genus_ladder_sge_20261115 (`c0b657b6d5:research/genus_ladder_sge_20261115/RESULTS.md`)

240 runs, arm C+pre, cc = 2.0, all certified, identical relation phases.
Paired ratios with/without SGE (best `sge(8,3)`): `la_mul` 0.255 (g=3), 0.347
(g=4), 0.274 (g=5); whole-log `FPM_mid` 0.492, 0.520, 0.624. Absolute: g=3
5.50e6 -> 2.74e6 (p=1031), 1.34e7 -> 6.47e6 (2053); g=4 3.32e7 -> 1.74e7 (1031),
8.89e7 -> 4.59e7 (2053); g=5 1.50e8 -> 9.37e7 (1031). LA share of FPM falls
from 0.66-0.76 (g=3,4) and 0.52 (g=5) to 0.32-0.53 and 0.23. Fill bound never
bound; `w_max > 8`, Markowitz on non-+-1 entries, and dropping surplus rows
untried. No rho rerun, so no band verdict claimed. Implied (arithmetic only,
not measured): g=3 p=1031 IC/rho ~0.75 x 0.49 ~ 0.37.

### 2.4 genus_ladder_sge_scale_20261115

Only `PROTOCOL.md`, `binary.sha256`, `frozen.sha256` exist at c0b657b6d5
(predictions Z3: SGE arm/rho in [0.22, 0.42] at 32771). **No RESULTS or data
found** in any commit (`git log --all` on RESULTS.md/*.jsonl is empty). The
composition 2LP+pre+SGE vs rho at g=3, p ~ 2^13..2^15 is unmeasured.

### 2.5 genus_ladder_bound_r_20261114 (`c0b657b6d5:research/genus_ladder_bound_r_20261114/RESULTS.md`)

g=4 hyperelliptic and plane quartics, h <= 2, p = 2053, 4099, 8209, 3 curves x 2
seeds; 108 runs certified; validator replay identical for p = 2053, 4099
(72/72 runs, 1,752 fields; `VALIDATION_REPORT.md`, disposition `validated` for
those 12 cells, `insufficient_evidence` at 8209). Bench record
`benchmarks/records/20261115T0031Z-genus-ladder-bound-r.json`.

| p | g4 arm c | g4 rho | c/rho | quartic 2LP | quartic rho | 2LP/rho |
|--:|--:|--:|--:|--:|--:|--:|
| 2053 | 8.93e7 | 1.81e9 | 0.049 [0.041, 0.062] | 7.0e6 | 7.9e8 | 0.0089 [0.0056, 0.018] |
| 4099 | 2.40e8 | 6.42e9 | 0.038 [0.022, 0.077] | 1.6e7 | 1.8e9 | 0.0086 [0.0042, 0.020] |
| 8209 | 6.74e8 | 2.16e10 | 0.031 [0.018, 0.062] | 6.1e7 | 4.7e9 | 0.013 [0.0064, 0.035] |

g=4 exponents: arm c 1.458 [1.449, 1.467] (theory 1.5), rho 1.79 [1.20, 2.22].
GAE band at g=4 unresolved (w=1, 8209: [0.48, 1.64]). Quartic 2LP FPM exponent
1.56 [0.97, 1.96]; **quartic `rel_mul` (conic evaluations of decomposition
rows) is 70-80% of its FPM**; full-base Diem grows as 1.98. Quartic below rho
only at GAE w=0. NB: quartic ratios are against the lab's quartic rho that L10
found 1.73x overpriced (section 4).

### 2.6 genus_ladder_harvest_20261115 (`c0b657b6d5:research/genus_ladder_harvest_20261115/RESULTS.md`)

Harvesting Cantor's first-reduction intermediate (degree g+1) as extra
relation candidates. g=3: trials x0.80 (full), x0.86/0.88/0.90 (2LP at
1031/2053/4099); but `rel_mul` x1.79-2.02 (degree-4 smoothness test) and
`FPM_mid` x1.12-1.15, GAE w=0.1 x1.57. **Verdict: not an improvement (closed).**
g=2: identical (no intermediate); g=4 splits at 1/720, trials change <= 3%.
Distance to rho (cell means): g=3 C+pre FPM 0.661 (p=1031), 0.890 (4099); GAE
w=0/0.1/1 at 4099: 0.144 / 4.27 / 22.9. g=4 C+pre at 4099: FPM 0.102; GAE
0.014 / 0.52 / 2.81.

### 2.7 Genus-4 positive-control family (current main, `research/genus4_*_20261116/`)

Instances = the bound_r genus-4 curves (h <= 2). Arms: `c` = 2LP+prefilter
(genus_ladder baseline); `v` = c + coordinate Wiedemann on a square row subset;
`w` = v + unit-T walk with capped restarts (PROTOCOL.md lines 31-32). These
are the genus-3 positive-control speed-ups ported to genus 4.

- `research/genus4_positive_control_20261116/RESULTS.md` (144 runs, 3 curves x 4
  seeds/prime): LA ratio v/c 0.330-0.331, w/c 0.222-0.233 at p = 2053, 4099,
  8209; total w/c 0.66-0.70. **Relation phase is 84-88% of arm w's native
  multiplications.** FPM w/rho: 0.021 (2053), 0.015 (4099), 0.011 (8209).
  GAE w=1 (mults only) w/rho 1.018, 0.708, 0.528 [0.382, 0.754]; red-team: with
  additions priced 1.057 [0.765, 1.499].
- `research/genus4_rho_refs_20261116/RESULTS.md` (p=8209, 9 targets): plain rho
  = 1.30x negation rho; Q3 reading (owner's primary, every emitted native op on
  both sides) w/rho = 0.003 / 0.098 / 0.142 at w = 0 / 0.1 / 1; rho does ~18
  window/Brent compares per step.
- `research/genus4_scale_20261116/RESULTS.md`: against window-free plain rho
  (P0f) at w=1: **1.050 [0.683, 2.006]** (8209 new), 1.049 [0.785, 1.513]
  (8209 pooled, 18 targets), **1.153 [0.772, 2.252]** (16411); unfloored P0u
  2.1-2.3x above. At w = 0.1: 0.193-0.212 (below). Lab negation rho needs 1.42x
  its expected steps at p = 16411 (plain rho cheaper there, P/R 0.67).
- `research/genus4_independent_audit_20261116/REPORT.md`: 144 runs replayed,
  0 mismatches; "unqualified complete-cost advantage against the strongest rho
  is not established". Arm w mean counters at 8209: G = 172,424 group ops;
  rel mults 43.4M; LA mults 8.0M; rel adds 39.1M; LA adds 60.1M. Pricing an
  addition above 0.465 of a mult crosses parity at w=1.
- `research/genus4_lean_rho_20261116/`: frozen protocol (arms w, B = negation
  rho window-free + Brent + look-ahead, Q = plain window-free) at p = 8209,
  16411, 32771; JSONL data exist (`p8209.jsonl`, `p16411.jsonl`,
  `p32771.jsonl`) but **no RESULTS.md found**; outcome not reported.
- `research/hecdlp_{fastgroup,sge_coord,sieve,smith_isogeny}_20261116/`: NOTES/smoke
  only (see section 2.9).

### 1.1b Newer genus_ladder (IcExt) — NOT the c0b657b6d5 copy

Important provenance note: `c0b657b6d5:src/cryptanalysis/genus_ladder.rs`
(2536 lines) is the SGE-era version (last touched a552dc2ee8). A later
3033-line version with `IcExt` and `run_ic_ext` exists at
**`fffa1b1243:src/cryptanalysis/genus_ladder.rs`** (2026-11-16, an ancestor of
current HEAD; deleted by 27b3d06b51). Its `IcExt` flags:
`coord_square` (coordinate Wiedemann on a random square row subset, only the
target coordinate, fallback to baseline Wiedemann), `unit_t` (walk starts
`aG + T`, jumps `alpha G`, segment capped at `max(64, floor(sqrt r / 16))`),
`ferrari` (Ferrari quartic splitting test after the prefilter, genus 4),
`sge_coord` (SGE then coordinate Wiedemann). Driver
`fffa1b1243:src/bin/genus_ladder.rs` arms: `A a C c` (as before), `v` = c +
coord, `w` = v + unit-T, `f` = w + Ferrari, `s` = w + SGE + coord (default
`--sge 4,1.5`), `R` negation rho, `P` plain rho (`gl::run_rho_walk`, with
closed-form per-step hash + 2 coefficient adds for symmetric pricing). New
flags: `--targets`, `--curve-seeds p=s1/s2/..`, `--calibrate`.
Companion modules on current main (still present):
`src/cryptanalysis/genus_ladder_fastgroup.rs` (Lange/Nagao-style g=3,4
explicit addition), `src/cryptanalysis/genus_ladder_sieve.rs` (sieved
`y - v_t(x)` relations), `src/cryptanalysis/smith_isogeny.rs`.

### 2.8 Other genus-3/4 measurements (c0b657b6d5)

**hec_bitop_20261020** (`c0b657b6d5:research/hec_bitop_20261020/README.md`):
round-seven rows of `examples/hyperelliptic_ic_vs_rho_bitops.rs` re-priced in
`bitop/v1` (`docs/units/BITOPS.md`), full-width rho, 18 rows, IC 5 runs/row,
rho 40 runs/row. **Genus 4, p = 61 (N = 16,790,591 ~ 2^24): IC/rho 0.114
[0.096, 0.138]** (variants ~0.13). Slopes of log2(IC/rho) vs log2 N: g=4
-0.232 [-0.320, -0.082]; g=3 +0.095 [+0.050, +0.148]; g=2 +0.269 [+0.138,
+0.523]. Rho fixed precomputation is 68% of rho bitops at g=2, p=251; without
it every g=2 row and the small g=3/g=4 rows read above 1. Self-classed
**"accounting", not an improvement**. One curve per row.

**Plane quartic programme** (`c0b657b6d5:research/plane_quartic_program_20261018/CLOSING.md`,
2026-10-26), GHS-weak family `E/F_{q^3}` -> plane quartic over F_q, Diem IC:
- `plane_quartic_pic.rs` (W1): generic Pic^0 addition 3,523 mults + 44 invs,
  flat in q.
- N4 (`research/quartic_n4_20261019/`): B0 pipeline 0.122 [0.10, 0.16] of plain
  rho at q = 262,111 (M-eq); exponent 1.083 [1.040, 1.129] (q >= 4,051) vs rho
  1.485.
- N4 v2 (`research/quartic_n4v2_20261020/`, q to 4,194,271, 501 IC units,
  2,442 rho runs): best pre-registered arm DL (2LP + SGE + block Lanczos);
  crossover vs negation rho q* = 555 [389, 752] (M-eq, w=0), 1,889 (CPU), 5,704
  (wall); at q = 2,097,091 DL = **0.005 [0.004, 0.008] of negation rho**;
  exponent gap 0.560 [0.510, 0.600]; DL exponent 0.988 [0.986, 0.989] (k =
  17..22). Relative to B0 at q = 2,097,091: S 1.001, L (SGE+BL) 0.784, D (2LP)
  0.081, DL 0.042, DL1 (D + SGE + scalar Lanczos, not pre-registered) 0.022.
  Negation rho = 0.715 [0.656, 0.777] of plain rho (theory 0.707).
  SGE shrank only to 0.74-0.78; block Lanczos 3.6x scalar.
- Status: N4 "open by frozen rule; no further work planned". Larger q blocked
  by 64-bit packing (q < 2^21) and l ~ 2^64 overflow.
- Smith route (W3a/W3b): psi verified; 91,944 planted toy ECDLPs carried, DL
  on J_X brute force (not IC); psi costs ~1e5-1e6 F_q mults per divisor.

**quartic_dl1_cd_sqrt_20261113** (`c0b657b6d5:research/quartic_dl1_cd_sqrt_20261113/README.md`):
GHS-weak quartics, k = 12..22 (q ~ 2^k), 116 units verified. Smaller 2LP base
(c_D rule 1.55) + fixed-exponent square root chain: DL paired ratio 0.684,
0.677, 0.627, 0.630, 0.584 at k = 18..22. DL vs frozen matched rho at k = 22:
0.0036 -> 0.0021 of plain rho; 0.0040 -> **0.0023 of negation rho** (w = 0).
Caveat (sensitivity.md, post hoc): additions unpriced; at w_add = 1.0 the gain
vanishes (1.02-0.98); CPU total ~1.0 at k = 18..21, 0.77-0.87 at k = 22.
Follow-up `cd_sweep/`: c_D = 1.7 recommended for k >= 18 (0.87-0.88 with
additions at 1 M-eq).

### 2.9 Genus-3 positive-control lineage (ic_control, GAE band unit)

Instrument: `c0b657b6d5:src/cryptanalysis/chained_transfer/ic_control.rs`,
`src/bin/ic_positive_control.rs`; genus-3 imaginary hyperelliptic, full
Jacobian, h <= 64. Cost unit here is the **GAE band** (group operations +
w x native mults, w = 0, 0.1, 1), not FPM -- which is why its genus-3 ratios
are much worse than genus_ladder's FPM ratios on the same kind of curve.

- `research/ic_positive_control_20261112/RESULTS.md`: p = 131..65537, 572 runs.
  Arm C (2LP, nb = 2 p^{2/3}) vs negation rho R: w=0 C/R 0.114 at 65537;
  **w=0.1 5.82 [3.84, 9.37]; w=1 31.5 [20.8, 52.7]** at 65537. Cause: the
  smoothness test (`x^p mod u`) plus roots ~362 native mults per trial (2.09e8
  relation mults vs 1.20e8 LA mults at 65537, 5.79e5 trials). Calibrated
  timing reading: C/R 0.58 at 65537 [0.38, 0.94].
- `ic_positive_control_lucas_20261116/RESULTS.md` (c0b657b6d5): Cardano-Lucas
  cubic splitting test, relation mults per trial L/C 0.45-0.49, M/C (with
  Cardano roots) 0.20-0.22 where a single-exponent cube root exists. End to
  end best new arm vs R at 65537: w=0.1 3.97, w=1 21.3.
- `ic_positive_control_la_20261116/RESULTS.md` (current main): coordinate
  Wiedemann N 0.667x LA, square-subset S 0.32-0.33x (0.39-0.45 with adds). S/R
  at 32771: w=0.1 **1.92 [1.08, 3.71]**, w=1 9.85.
- `ic_positive_control_unit_t_20261116/RESULTS.md`: finding F1 -- mixed-walk
  IC arms partly solve by rho collisions inside the walk when sqrt r ~ 4096
  (affects p <~ 521 rows of all genus-3 controls and genus_ladder small cells).
- `ic_positive_control_capped_20261116/RESULTS.md`: arm W (capped unit-T +
  deterministic fallback roots) = current genus-3 operating point. W/R at
  w=0.1: 1.77 (16411), **1.72 [0.95, 3.32] (32771)**, 2.11 (65537), 1.78
  (131101); at w=1: 8.97, 8.75, 11.1, 9.17. Still above rho in the GAE band.

### 1.2 Other IC instruments for genus >= 2 (c0b657b6d5)

| file | lines | what |
|---|--:|---|
| `src/cryptanalysis/hyperelliptic_index_calculus.rs` | 3181 | ADH/Gaudry IC on `Jac(C)(F_p)`, any genus `deg f = 2g+1`, on `prime_hyperelliptic` Mumford/Cantor substrate; degree-1 factor base, factor-base walk, sparse elimination mod r, optional single large primes (Theriault, line 1232) |
| `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs` | 1470 | char-2 companion: `y^2 + h y = f`, `deg f = 2g+1` over `F_{2^m}`, degree-1 factor base |
| `src/cryptanalysis/hec_dlp.rs` | 708 | front door: `HecGroup` abstraction, order finding, BSGS, Pohlig-Hellman over composite orders, IC only in large prime components; both field families |
| `src/cryptanalysis/hyperelliptic_ic_bench.rs` | 2390 | IC vs rho on the same Jacobian; unit `S = group ops / sqrt(N)` |
| `src/cryptanalysis/genus2_fp.rs` | 1372 | genus-2 u64 toolkit (`y^2 = f`, deg 5, p < 2^31), from `examples/r4_16_6_genus2_census.rs`; census tool, not an IC solver |
| `src/cryptanalysis/ecbench/hyper_ic.rs` | 1319 | ecbench-counted hyperelliptic IC on word-size Cantor (`HCurve`/`HDiv`): reduced base (y <= (p-1)/2, increasing x, truncated), smoothness by splitting u, optional single LP |
| `src/cryptanalysis/ecbench/plane_quartic.rs` | 1570 | smooth plane quartic Jacobians as ecbench workloads (section spaces S_n) |
| `src/cryptanalysis/plane_quartic_pic.rs` | 3528 | canonical Pic^0 on smooth plane quartics (W1); generic add 3,523 mults + 44 invs |
| `src/cryptanalysis/diem_quartic.rs` | 3538 | W2a: Diem line relations on m=2 GHS quartics, LA mod l, priced rho on E0/F_{q^3} |
| `src/cryptanalysis/quartic_ladder.rs` | 1718 | generic plane quartic rung of genus_ladder: Diem full/2LP + negation rho |
| `crates/hec-native-ic/` | -- | native hyperelliptic IC crate (`ic.rs`, `large_prime.rs`, `table_factor.rs`, `interpolation.rs`, `hybrid.rs`), examples anchor/principal relations, table oracle |
| `src/cryptanalysis/gaudry_quartic*.rs`, `jv_quartic.rs`, `bielliptic_quartic.rs`, `quartic_folds.rs`, `nist_cover_quartic.rs`, `chained_transfer/quartic_ic.rs`, `chained_transfer/full_jacobian_ic.rs` | -- | genus-3 quartic/decomposition IC variants (EC-cover oriented) |
| `examples/hyperelliptic_ic_vs_rho*.rs`, `hyperelliptic_ic_large_prime.rs`, `ic_improve*_genus3_*.rs` | -- | older genus 2-4 IC-vs-rho harnesses (round seven; bitop study) |
| `research/humbert_genus5_cover_20261021/src/*` | -- | genus-5 cover study crate with its own LA (`la.rs`), fast relations (`relfast.rs`), GHS filter |

**Genus >= 6:** no executable IC solver or measurement found for genus 6..16
on prime fields. What exists is formula-level:
`c0b657b6d5:src/cryptanalysis/jacobian_transfer/cost.rs` (frozen cost model:
`hyperelliptic_index_calculus` = `g! q + g^2 q^{2-2/g}` Jacobian ops, g >= 2;
`enge_gaudry_smoothness` = explicit count up to genus 1024 with places of
degree <= b, sparse LA `g F^2`, minimised over b; `enge_gaudry` =
`L_N(1/2, sqrt 2)` above genus 1024), documented in
`c0b657b6d5:docs/curves/JACOBIAN_TRANSFERS.md` line ~136 ("a row is a
formula, never a measurement"); genus-9 `cover_descent` rows of
`GF(p^6)` curves there are formula prices. The only *measured* genus >= 6
quantity is the Cantor operation count,
`c0b657b6d5:research/jacobian_op_count_20261014/summary.csv` (textbook Cantor,
p = 1009, mean mults+10.. total per add): g=2 164.7, 3 338.7, 4 569.6, 5 937.1,
**6 1354.9, 8 2620.9, 10 4585.6, 12 7236.2, 14 10843.5, 16 15343.5**, 20
27984.7, 32 102282.5; fit exponent 2.347 [2.346, 2.349]. (Different counter
from genus_ladder's 90/219/393/693.) GHS genus-16 references in
`examples/ghs_attack_demo.rs` (line ~322: "converting to a single
hyperelliptic equation of genus 16 is ... research-grade (Diem/GTTD;
Enge-Gaudry") and `examples/ghs_relation_constants.rs` (g = 16.0 constant)
are notes/constants, not solvers. No degree-b (Enge-Gaudry) factor base,
no distinct-degree-factorisation smoothness test found in any IC solver.

### 2.10 Genus 5 (cover route): humbert_genus5_cover_20261021

`c0b657b6d5:research/humbert_genus5_cover_20261021/README.md` (+
`results/summary.md`, `dlp_20261113/`, `dlp_filter_20261115/`,
`la_solver_20261116/`, `levers_20261116/`). Genus-5 cover C/F_q of an
elliptic curve over the quartic extension (toy q = 17..32003, r < 2^62);
own native crate (`src/la.rs`, `relfast.rs`, ...). **Unit: unisolated wall
time** (L0), rho = matched rho on the source subgroup (crate's scalar rho,
"expected-steps" and negation variants). Not the genus_ladder FPM unit.
- v1 sweep (90 runs, q = 61..8009, full and iota arms): rho faster at every q;
  e.g. q = 8009 iota IC/rho(expected) 2.37 [1.96, 4.17].
- dlp (2LP) 2026-11-13: median IC/expected-rho 2.18, 2.85, 1.65 at q = 8009,
  12007, 16001; exponent of t_ic 1.68 vs baseline 1.77.
- dlp_filter 2026-11-15: dlpf IC/expected-rho 2.38, 1.47, 1.21 (q = 12007,
  16001, 24001).
- la_solver 2026-11-16 (scalar Lanczos one-solve, LA 0.203x): best median
  **IC/expected-steps rho 0.55 (q = 24001), 0.36 (q = 32003); vs negation rho
  0.78 and 0.51**; IC slope 1.36 vs rho ~2. "Expected toy crossover ... not a
  break". Relation collection then 62% of time.
- levers 2026-11-16: relation phase x0.042, LA x0.29; comb/base t_ic 0.184,
  0.182, 0.170 at q = 16001, 24001, 32003 (no new rho ratio quoted in README).
Genus 5 *generic hyperelliptic* data is only in genus_ladder (sec. 2.1, 2.3).

### 2.11 Older genus-2 reference

`c0b657b6d5:research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`
(`hyperelliptic_index_calculus.rs` + `hyperelliptic_ic_bench.rs`, unit
`S = group ops / sqrt N`, LA converted at measured `c` ~2,200-2,660 mul-mods
per group op; rho = 16-adding Teske walk + Floyd): genus 2, p = 41..251,
S_ic/S_rho = 3.0, 4.2, 5.5, 7.9, 8.8, 8.5 -- "loses harder as p grows";
S_ic/floor 32-49. Class: accounting.

### 2.12 Yesterday's leads that carry a measurement (`research/g2plus_leads_20261116/`)

All Python or standalone instruments, `[pcc:unverified]`, counted FPM:
- L01 (g=2, 3): relation-phase FPM/relation 30-70x below generic Cantor walk,
  5-12x below explicit formulas, 3.3-3.9x with pencil set-up charged; descent
  and LA not charged; pencil rows 16-465x dearer to eliminate. Follow-up F3
  (`research/g2plus_followups_20261117/F3_L01_pencil_end_to_end/`) frozen,
  pilot only, no RESULTS yet.
- L02: `y^2 = x^5 + a` IC/rho improves 2.46x [2.23, 2.75] (orbit base, rho
  also uses the automorphism, rho_gain 1.61 pooled); `y^2 = x^7 + a` 1.10x
  [0.91, 1.34] (killed). Setup-free: 1.6-2.0x.
- L03 (g=3): 2LP base constant cc 2 -> 1 cuts counted cost 36-40% in a
  simulator; F1 (`.../F1_L03_base_constant_lab_instrument/`) re-measures in
  genus_ladder; frozen, pilot only, no RESULTS yet.
- L08 (nodal plane quintic g=4, sextic g=6 `d6_m4`, Python): A_2LP slopes
  e.g. 0.981 (g=4 one model), 1.11-1.17 (g=6) -- **void**: duplicate lines
  counted as relations (INCONCLUSIVE-instrument). This is the only genus-6
  relation collection found anywhere, and it is invalid.

## 3. Tried and closed (genus >= 2)

From the closed-routes map `c0b657b6d5:research/ic_mega_pass_20261031/brief/MAP_closed.json`
(116 items; schema MAP_closed.v1):
- `TREAT-frontier-measurement`: 4,848 priced executions, no IC arm below rho
  on any prime-field or prime-degree binary *elliptic* curve; "weak
  extension-field families DO cross rho: genus-3 GHS (exponent ~1.356 vs 1.5),
  plane quartics ...".
- `LEAD-TRF-same-field-genus-g-embedding` (closed_obstructed, theorem):
  `#Jac(C)(F_p) = N * #A(F_p)`, `#A = Theta(N^{g-1})`; same-field genus-g
  embeddings never help ("never revisit for same-field covers").
- `LEAD-EXO-split-jacobian-projection` (closed_obstructed).
- `LEAD-TRF-small-genus-cover-general` (closed_obstructed, proof): only the JV
  configuration gives g <= 3 among 2-torsion Kummer covers of E/F_{q^3}
  (others g in {1,7,9,13}); for E/GF(p^k) every GHS cover has genus >= 9, 49,
  41 at k = 6, 10, 14.
- `LEAD-TRF-jv-nagao-genus2-cover-decomp` (closed_dominated): Nagao on genus-2
  cover of E(F_{p^6}) at S/rho 151-293.
- `KR-IC-a4dd54` (known bound): 2LP IC `O~(q^{2-2/g})` beats rho `q^{g/2}` for
  g >= 3; Diem plane quartic `O~(q)`.
`research/leads_20261111/EXISTING_LEADS.txt` (173 lines) is a name list only;
genus-related names: LEAD-EXO-split-jacobian-projection,
LEAD-ICB-ecc2k130-jacobian-carrier, LEAD-TRF-cover-catalog-jacobian-transfer,
LEAD-TRF-jv-nagao-genus2-cover-decomp, LEAD-TRF-odd-char-descent-smith-quartic,
LEAD-TRF-same-field-genus-g-embedding, LEAD-TRF-small-genus-cover-general,
LEAD-TRF-weil-restriction-jacobian-class.

Closed within the genus_ladder programme (measured):
- Cantor-intermediate harvesting (genus_ladder_harvest_20261115): FPM x1.12-1.15, closed.
- Parity prefilter: kept, but only 1-9% of total (exact, no further gain).
- Genus 2 2LP crossing rho: none up to 2^24 ("nothing suggests a crossing").
- `leads.log` (c0b657b6d5): HEC-NATIVE genus-2 dense step-column solve
  4.04-5.75x vs Fast (more mulmods); GENUS5 cover 2LP "keep cover family
  restrictions explicit".

Yesterday's twelve (`research/g2plus_leads_20261116/README.md`, each
`L*/ASSESSMENT.md`):

| lead | verdict | headline |
|---|---|---|
| L01 anchored pencil sieve | supported | g=2 relation phase 3-5x vs strongest baseline (30-70x vs generic Cantor walk); not a solve |
| L02 automorphism orbits | g=2 supported, g=3 killed | x^5+a: 2.46x [2.23, 2.75]; x^7+a: 1.10x |
| L03 base re-balance | supported | g=3 cc 2 -> 1: -36..40% counted cost (simulator) |
| L04 batch smooth-part | **killed** | batch arms 9-44x dearer per candidate at g=3 |
| L05 even-degree models | supported (parity) | cost 0.995-1.022 of odd model; reach only |
| L06 odd-l isogeny census | void (instrument), rerun; F2 re-freeze | kernel criterion fires on 27.8/26.8/24.0% at l = 3,5,7 |
| L07 Prym IC (g 4-5) | **killed** | 2LP exponent 16/9, 20/11 > 3/2, 8/5 |
| L08 non-hyperelliptic g4-g6 | inconclusive (instrument) | duplicate lines counted |
| L09 gonal pencil seeds | inconclusive | proxy saving 31% -> 7% as p grows |
| L10 C34/Picard matched rho | orbit inconclusive; rho accounting supported | orbit fold R_T ~9.0; lab quartic rho overpriced 1.73x |
| L11 Laine-Lauter memory | inconclusive | halving memory costs 2.1-2.4x trials |
| L12 bielliptic orbits | void (instrument) | lab fold already beats orbit arm 1.4-1.6x; "no reason to rerun" |

## 4. Cost units and how rho is matched

Units in use (they are not interchangeable; ratios differ by >10x between them):
1. **FPM** (genus_ladder, L-leads): `cantor_mul + 10 inv + rel_mul + g^2
   la_mul` -- one F_p mult = 1, inversion I = 10, a mult mod r (r ~ p^g)
   kappa = g^2. "FPM_mid" / band brackets I and kappa (L03 used (10, 9) at
   g=3; L02 (10, g^2), (1,1), (30, g^2)).
2. **GAE band** (AGENTS.md sec. 5a, `c0b657b6d5:AGENTS.md` line 1214): group
   ops + w x every native op, w in {0, 0.1, 1}; rank only if the order holds
   at all three; plus `w_cal` (measured mulmod/group-add, ~0.005-0.013 GAE on
   lab hosts). Owner decision 2026-11-16 (genus4_rho_refs): primary reading
   Q3 = every emitted native op on both sides. The genus-3 positive control
   and genus-4 studies use this.
3. **bitop/v1** (`docs/units/BITOPS.md`; hec_bitop_20261020).
4. **M-eq** (`mul + r_inv inv + r_lmul lmul`) in the plane-quartic programme
   (additions unpriced; quartic_dl1 shows the gain vanishes at w_add = 1).
5. **Wall time** (humbert genus-5, unisolated L0).
6. Formula model `log2` bit-ops (`jacobian_transfer/cost.rs`), never measured.

Matched rho per genus:
- **Hyperelliptic g = 2..5 (genus_ladder):** `run_rho` =
  `rho_reference_walk(..., RhoWalk::negation())` on the order-r subgroup, max
  steps `60 sqrt(r) + 10^4`, certified once; negation in Mumford form is free
  (v -> -v). Expected steps `sqrt(pi r / 4)`. No other automorphisms (random
  curves). The walk carries a 16-key fruitless-cycle window and Brent check:
  ~18 compares per step (genus4_rho_refs), DP bits `floor(bits(r)/4)`.
  Known defects: at g=4, p=16411 it needs **1.42x its expected steps**
  (genus4_scale), and plain rho is then cheaper (P/R 0.67); against a
  window-free plain rho (P0f) arm w goes from 0.149 to 1.05 at w=1. A lean
  negation rho (`B`: window-free, Brent, Wiener-Zuccherato look-ahead) is in
  genus4_lean_rho_20261116, results not written up.
- **Genus 2 rho reference** has heavy fixed set-up (16 full-width jump
  points; 68% of rho bitops at g=2, p=251 in hec_bitop), so small-p genus-2
  ratios understate rho.
- **Plane quartics (quartic_ladder, genus_ladder `--quartic`, bound_r):**
  `ql::run_rho` negation walk. **L10 finding** (`research/g2plus_leads_20261116/L10_c34_picard_matched_rho/{PROTOCOL,RESULTS,ASSESSMENT}.md`):
  each negation-walk step costs one Pic addition A ~ 3,521 mults + 44 inv
  plus one negation N ~ 5,400 mults + 35 inv (sum ~8,921 + 79, matching the
  observed ~8,900/op in bound_r.jsonl). Since a quartic negation is dearer
  than the sqrt 2 it saves, `F_Q = (A + N)/(sqrt 2 A)` = **1.7338, 1.7269,
  1.7322** at p = 2053, 4099, 8209 (model value: ideal step counts x measured
  step costs; not an executed walk). Corrected ratios: bound_r 2LP/rho 0.0089,
  0.0086, 0.013 -> ~0.0154, 0.0149, 0.0225; genus_ladder p=1031 quartic 0.027
  -> ~0.047. For Picard curves (C_{3,4}, extra order-3 automorphism) the
  cheapest matched rho is the `<rho>` class walk S3 = 0.447 x S2.
  The GHS-weak quartic rows (plane_quartic_program, quartic_dl1) use an
  *elliptic* reference (rho on E0/F_{q^3}; negation 0.715 of plain) and are
  unaffected.
- **Genus-5 cover (humbert):** rho on the elliptic source subgroup, the
  crate's scalar rho, reported against expected steps and negation variant.
- **L02 (y^2 = x^5 + a):** rho also folded by the automorphism (rho_gain 1.61).

## 5. Gaps in genus 2..16

- **Genus 2, 3, 4:** instrumented and measured (several instruments, several units).
- **Genus 5:** hyperelliptic only in genus_ladder at p <= 1031 (5 primes,
  4 curves; SGE at 1031 only); no scale study, no IcExt arms (v/w/s), no
  explicit group law (`genus_ladder_fastgroup.rs` covers g = 3, 4 only), no
  `h <= 2` bound-r run; `Rel` holds 5 base entries (REL_FB = 5), so the
  harvest and sieve variants overflow at g >= 4/5. Cover-route genus 5
  (humbert) measured in wall time only. Non-hyperelliptic genus 5: none.
- **Genus 6:** no working instrument. `genus_ladder` stops at `MAX_GENUS = 5`
  (and `(sqrt p + 1)^{2g} < 2^63` caps p; u64 group orders). The generic
  `hyperelliptic_index_calculus.rs` has no genus bound in its header but no
  genus-6 measurement was found. Only L08's Python plane-sextic arm, voided.
- **Genus 7..16:** no instrument and no IC measurement at all. Only (a) the
  textbook-Cantor op counts at g = 8, 10, 12, 14, 16 (jacobian_op_count),
  (b) the formula cost model (`hyperelliptic_index_calculus` `g! q + g^2
  q^{2-2/g}`; `enge_gaudry_smoothness` with degree <= b factor base, to genus
  1024), (c) GHS genus-9/15/16 cover rows priced by formula.
  No degree-b (Enge-Gaudry/Gaudry-style higher-degree) factor base, no
  distinct-degree-factorisation smoothness test, no subexponential arm
  exists in any executed IC instrument.
- **Missing results inside measured genera:** genus_ladder_sge_scale_20261115
  (2LP+SGE vs rho at g=3, p = 2^13..2^15) has protocol only;
  genus4_lean_rho_20261116 has data but no RESULTS; follow-ups F1/F2/F3 frozen,
  no results; no isolated-host validation for any genus >= 2 row; genus 4
  w = 1 against the strongest rho is open (1.05-1.15 vs P0f).

## 6. Summary table

Ratios are IC/rho, `[pcc:unverified]` throughout. "Unit" matters: FPM ratios
are far more favourable to IC than GAE-band ratios.

| genus | instrument(s) | best measured IC/rho (unit, p) | exponent (IC; rho) | main cost component | status | path |
|---|---|---|---|---|---|---|
| 2 | genus_ladder (A/a/C/c), hyperelliptic_index_calculus + ic_bench, ecbench hyper_ic, hec-native-ic, L01/L02 (Python) | 2LP+pre: 1.7 (FPM, 11th of 12 primes ~2^23); typically 3-5x, 3.3 at 8209, 5.1 at 2^24; L02 orbit x^5+a 2.46x better | 1.004 [0.991, 1.020]; rho 0.975 | relation walk (LA 0.08-0.20) | open (L01 F3 pending); no crossing seen | `c0b657b6d5:research/genus_ladder_scale_20261115/RESULTS.md` |
| 3 hyperelliptic | genus_ladder (+SGE), ic_control positive control (L, M, N, S, W arms), IcExt v/w at fffa1b1243 | FPM: **0.537 [0.407, 0.686] at p=32771** (2LP+pre); SGE(8,3) halves FPM at 1031 (composition at scale unmeasured). GAE: W/R 1.72 [0.95, 3.32] (w=0.1), 8.75 (w=1) at 32771 | 1.344 [1.334, 1.358]; rho 1.413 | LA 0.75-0.77 of FPM (2LP); in GAE the relation phase (smoothness test ~362 mults/trial originally) | open | `c0b657b6d5:research/genus_ladder_scale_20261115/RESULTS.md`; `research/ic_positive_control_capped_20261116/RESULTS.md` |
| 3 plane quartic | quartic_ladder, plane_quartic_pic, diem_quartic, ecbench plane_quartic | generic: 2LP 0.0086 at 4099 (FPM) -> ~0.0149 after L10 1.73x; GHS-weak DL 0.0023 of negation rho at q~2^22 (M-eq, w=0) | generic 2LP 0.69 (ladder) / 1.56 [0.97, 1.96] (bound_r); GHS DL 0.988; rho 1.36 / 1.485 | relation phase (conic evaluations 70-80% of FPM) | N4 open, "no further work planned"; L10 correction pending to row owners | `c0b657b6d5:research/genus_ladder_bound_r_20261114/RESULTS.md`; `c0b657b6d5:research/plane_quartic_program_20261018/CLOSING.md`; `research/g2plus_leads_20261116/L10_c34_picard_matched_rho/ASSESSMENT.md` |
| 4 hyperelliptic | genus_ladder c, IcExt v/w/f/s, fastgroup, sieve (smoke), round-seven bitops harness | FPM: c 0.031 at 8209; w 0.011 at 8209. GAE Q3 w=1: 0.142 (frozen negation refs), but 1.05 [0.785, 1.513] (8209 pooled) and 1.15 (16411) vs window-free plain rho. bitop/v1: 0.114 at p=61 | c 1.458 [1.449, 1.467]; rho 1.79 [1.20, 2.22] | relation phase 84-88% of arm w native mults | open (w=1 vs strongest rho unresolved; lean-rho results unwritten) | `research/genus4_scale_20261116/RESULTS.md`; `c0b657b6d5:research/genus_ladder_bound_r_20261114/RESULTS.md` |
| 5 hyperelliptic | genus_ladder (A/a/C/c, SGE) | FPM 0.0084 [0.0047, 0.020] at 1031 (C+pre); SGE(8,3) x0.624; GAE w=1 0.48 [0.27, 1.14] | 1.02 [0.93, 1.12] (set-up dominated; theory 1.6); rho 2.69 | LA 0.52 of FPM (0.23 after SGE), then relations + Cantor | open, thinly measured | `c0b657b6d5:research/genus_ladder_20261114/RESULTS.md`; `.../genus_ladder_sge_20261115/RESULTS.md` |
| 5 cover | humbert_genus5_cover crate | 0.36 vs expected-steps rho, 0.51 vs negation rho at q=32003 (wall time) | t_ic slope 1.36; rho ~2 | relation collection 62% (before levers) | open (toy crossover) | `c0b657b6d5:research/humbert_genus5_cover_20261021/README.md` |
| 6 | none working (L08 Python sextic, void) | not found | L08 A_2LP 1.11-1.17 (void) | -- | no instrument | `research/g2plus_leads_20261116/L08_nonhyperelliptic_ladder_g4_g6/ASSESSMENT.md` |
| 7-16 | none (formula model only; Cantor op counts) | not found | not found | -- | no instrument, no measurement | `c0b657b6d5:src/cryptanalysis/jacobian_transfer/cost.rs`; `c0b657b6d5:research/jacobian_op_count_20261014/summary.csv` |

Status: COMPLETE (2026-11-17). All figures quoted from the named files; none re-run.
