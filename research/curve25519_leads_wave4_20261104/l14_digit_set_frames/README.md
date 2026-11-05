# L14: digit-set frames for the bits factor base (Curve25519 k = 20) — census result

**Class:** exploratory mechanism census in exact counts (pure Python, no engine, no
ecbench, no timing). It measures exact m = 2 subgroup coverage of candidate factor
bases on the family miniature k = 20 (A = 926, p = 1,048,573, r = 131,203, n = 8r).
It does not solve a discrete log, produces no F4 mul-adds and is not a leaderboard
row. **Rho remains ahead** (T12: the best PKM arm is 6.06·10⁴× behind rho-neg at
w = 0.1 on k = 20); nothing here changes that order.

Protocol: [PROTOCOL.md](PROTOCOL.md), scripts frozen at commit
`5a800bb3d06c69b32f32195c3aebccdfb3fee31e` (per the wave-4 brief; no git use by this
session). Run: 2026-11-04 06:39 UTC, `scripts/run.sh`, one Python process, 11 s
after a 0.8 s log-table build (sizing only, not a result). No deviation from the
frozen command; `data/run.log` and `data/analysis.txt` are the verbatim outputs.

## Answer

- **The lead's success rule held (P4):** the greedy digit set (replicate 0) covers
  **522** subgroup points at t = 5 and **1268** at t = 6, against the best of 2000
  random translation frames (**248** and **570**): ratios **2.10×** and **2.22×**,
  both ≥ 1.3. Per the protocol the lead stays **OPEN** and the next step is the
  engine confirmation of section 7 of PROTOCOL.md (not done here; f4cost cannot take
  digit sets today).
- The point prediction P5 (ratio in [1.10, 1.45]) **failed on the high side**: the
  greedy does much better than the frozen extreme-value reasoning allowed. The
  coset histograms (exploratory, below) show why: the greedy drives the base into a
  single cofactor coset (16 of 22 points in coset 0 mod 8 at t = 5), a multiplicative
  effect on J, not the additive one the protocol assumed.
- Random digit sets are not a new object (Q-A): the mean ratio is 0.985 / 0.989 /
  1.015 (P2 held); the max ratio failed its band at t = 5 only (0.734 < 0.75; P3
  failed), driven by one outlying translation frame (248; the next is 220).
- The greedy is **not** stable across replicates at t = 4 and t = 5 (P9 failed:
  spread 1.52 and 1.48; 1.19 at t = 6 would have held). One replicate (t = 5 rep 3)
  started from a `lo` whose point is not in coset 0/4 and never recovered (392 with
  29 columns).
- Cost model direction (P8) held strongly: covered/column of the greedy set is
  2.3–2.5× that of the best translation frame (23.7 vs 10.3 at t = 5; 29.5 vs 11.9
  at t = 6), so under the T10 model (E ∝ columns/covered at unchanged mpt) a digit
  set would cut a *bits* arm's modelled F4 cost by about 2.3× (t = 5) and 2.5×
  (t = 6). That is a bits-arm statement; the measured bits-form mpt is 17–64×
  the isogeny tower's at equal (t, G) (see "Distance remaining"), so this does not
  overtake the T10 winner on the model.

## Instrument check (P1, must hold first)

`bits lo = 0` reproduced the #3359 census exactly: covered **12 / 42 / 162** at
t = 4 / 5 / 6 (7 / 11 / 25 columns). The K = 2000 translation means **19.112 /
70.924 / 266.545** are within −0.8 % / +0.03 % / −1.5 % of the 20261124-seed
reference (19.26 / 70.90 / 270.54). **P1 held.** `frame_census.py` sha256
`afcf2cc3…31e3d` (listed in `data/SHA256SUMS`).

## Results (verbatim from `data/summary.json`)

### (a) Random families, K = 2000 each, rng `random.Random(20261104 + t)`; sd is the population standard deviation (sample sd for translation: 10.619 / 26.933 / 69.562)

| t | translation mean / sd (population, ddof = 0) / max (cols of best) | digit-set mean / sd / max | mean ratio | max ratio | distinct_x min (digits) |
|--:|:--|:--|--:|--:|--:|
| 4 | 19.112 / 10.616 / **74** (12 cols, L = 250777) | 18.822 / 10.686 / **74** | 0.985 | 1.000 | 16 |
| 5 | 70.924 / 26.926 / **248** (24 cols, L = 815271) | 70.157 / 26.019 / **182** | 0.989 | 0.734 | 32 |
| 6 | 266.545 / 69.545 / **570** (48 cols, L = 889850) | 270.497 / 68.919 / **594** | 1.015 | 1.042 | 64 |

No random digit set had a repeated abscissa (`distinct_x` = 2^t throughout).
Exploratory (no rule): two-sample KS statistics 0.0255 / 0.0185 / 0.0355, all below
the 5 % critical value 0.0429 for 2000 vs 2000; quantiles q50/q90/q99 are 16/34/52
vs 16/34/52 (t = 4), 68/106/146 vs 68/106/140 (t = 5), 262/356/448 vs 264/362/466
(t = 6); mean columns 8.00 vs 7.96, 16.03 vs 16.01, 31.84 vs 32.20. The translation
top five at t = 5 are 182, 184, 186, 220, 248; the digit top five 154, 160, 164,
172, 182.

### (b) Greedy construction, B = 2000 candidates per digit, R = 5 replicates

Replicate 0 (primary; seed 20261104 + t + 1000):

| t | lo | digits c_1..c_t | cols | covered | J | covered / trans max | covered / trans mean | covered/col (greedy vs best translation) | trajectory (cols, covered) per step |
|--:|--:|:--|--:|--:|--:|--:|--:|:--|:--|
| 4 | 270158 | 44640, 482808, 833512, 163143 | 13 | **208** | 208 | **2.811** | 10.88 | 16.00 vs 6.17 | (1,2) (3,18) (7,72) (13,208) |
| 5 | 113971 | 289589, 761384, 273167, 963514, 602728 | 22 | **522** | 522 | **2.105** | 7.36 | 23.73 vs 10.33 | (1,2) (3,18) (6,72) (12,242) (22,522) |
| 6 | 51134 | 293482, 1015139, 374525, 914144, 700453, 505371 | 43 | **1268** | 1272 | **2.225** | 4.76 | 29.49 vs 11.88 | (1,2) (3,18) (6,72) (11,202) (21,464) (43,1268) |

All five replicates (covered, cols): t = 4: 208/13, 202/12, 250/14, 164/12, 250/14
(spread 1.524); t = 5: 522/22, 524/22, 582/21, 392/29, 474/23 (spread 1.485);
t = 6: 1268/43, 1364/45, 1326/42, 1142/45, 1148/46 (spread 1.194). Median replicate
over translation max: 2.811 / 2.105 / 2.225. Every replicate at every t exceeds the
translation max (minimum ratio 392/248 = 1.58 at t = 5).

The full selected sets (all 15, with `lo`, digits, seed, cols, covered, J) are in
`data/selected_digit_sets.json`; the per-step records in `data/k20_t{4,5,6}.json`.

### Search cost (counted; P7)

| t | greedy rep 0: Legendre / [r]P labels / group ops (1.5·log₂ r = 25.50 per label) | translation census K = 2000: Legendre / labels / group ops | Legendre ratio | label ratio |
|--:|:--|:--|--:|--:|
| 4 | 30,000 / 15,096 / 384,981 | 32,000 / 15,999 / 408,009 | 0.9375 | 0.944 |
| 5 | 62,000 / 31,034 / 791,434 | 64,000 / 32,054 / 817,446 | 0.96875 | 0.968 |
| 6 | 126,000 / 62,902 / 1,604,137 | 128,000 / 63,679 / 1,623,952 | 0.984375 | 0.988 |

The Legendre ratios are exactly (2^t − 1)/2^t, as the analytic bound says; labels
are ≈ 0.50 of the Legendre tests in both families (0.4975–0.5032). The five replicates
cost within 0.4 % of five times the rep-0 figure but not exactly that: exact sums are 150,000 /
309,999 / 629,998 Legendre tests (one abscissa collision each in t = 5 rep 3 and t = 6 rep 2)
and 75,223 / 154,705 / 314,858 labels at t = 4 / 5 / 6. For scale, the
isogeny seed census that produced the T10 winner recorded 6.94·10⁶ group operations
at t = 5 (build + selector over 2,000 seeds), about 8.8× this greedy's search.

## Predictions (decided by the frozen `scripts/analyse.py`; verbatim `data/analysis.txt`)

| | rule (PROTOCOL §5) | outcome | evidence |
|:--|:--|:--|:--|
| P1 | bits lo=0 = 12/42/162; translation mean within ±10 % of 19.26/70.90/270.54 | **held** | 12/42/162; 19.112/70.924/266.545 |
| P2 | digit mean / translation mean ∈ [0.95, 1.05] at every t | **held** | 0.985, 0.989, 1.015 |
| P3 | digit max / translation max ∈ [0.75, 1.33] at every t | **FAILED** | 1.000, **0.734**, 1.042; fails at t = 5 only, translation side higher |
| P4 | greedy rep 0 ≥ 1.3 × translation max at t = 5 and t = 6 | **held** | 2.105, 2.225 (lead stays open) |
| P5 | greedy rep 0 / translation max ∈ [1.10, 1.45] at t = 5 and 6 | **FAILED** | 2.105, 2.225: above the band at both t (under-prediction) |
| P6 | greedy / translation mean ≥ 3.0 (t = 4), ≥ 2.0 (t = 5, 6) | **held** | 10.88, 7.36, 4.76 |
| P7 | greedy Legendre ≤ 1.0×, labels ∈ [0.8, 1.2]× the translation census | **held** | 0.9375/0.969/0.984; 0.944/0.968/0.988 |
| P8 | covered/col (greedy) ≥ 0.9 × covered/col (best translation) at t = 5, 6 | **held** | 23.73 vs 10.33; 29.49 vs 11.88 |
| P9 | greedy replicate spread max/min ≤ 1.3 at t = 5 and 6 | **FAILED** | 1.485 (t = 5), 1.194 (t = 6); t = 4 (no rule) 1.524 |

Six held, three failed; the failed ones stay failed. No threshold, seed, K, B or
tie rule was changed. `lead_status` from the script: "OPEN: digit-set greedy >= 1.3x
translation best (P4 held)".

## Exploratory observations (not preregistered, no rule)

- **Mechanism: coset alignment.** Coset histograms (full log mod 8 of the base
  points; sign-folded) of the selected sets, rep 0: t = 4 `[2,0,0,0,10,0,0,1]`
  (10/13 in coset 4); t = 5 `[16,2,0,0,1,0,2,1]` (16/22 in coset 0); t = 6
  `[6,3,3,2,24,2,3,0]` (24/43 in coset 4). Bins 0 and 4 and the fold-pair totals are exact; the other bins depend on the generator that
  `frame_census.build` picks (`random.Random(1)`) and would permute with another generator
  (e.g. t = 5 rep 0 `[16,3,1,0,1,0,1,0]` instead of `[16,2,0,0,1,0,2,1]`); the mechanism claim
  rests on bins 0 and 4. Every replicate concentrates in coset 0
  or 4 (the two cosets whose points pair with themselves and each other into E[r]).
  The best translation frames are spread over the cosets (t = 5: `[0,3,6,0,6,0,7,2]`;
  t = 6: `[3,8,7,9,5,5,4,7]`). The early greedy steps are perfect: 1, 3, 6 columns
  give 2, 18, 72 = 2c² covered, the maximum for c points in one coset; saturation
  sets in from step 4 (242 of 288 at 12 columns, 522 of 968 at 22, 1268 of 3698 at
  43). The selected digit sets are therefore partial "coset-0 towers" built by
  additive translation rather than by isogeny.
- **The failed replicate** (t = 5 rep 3: lo = 226727, step 0, the base {lo}, is 1 column and 0
  covered; step 1 is (2, 2)) started from a point outside cosets 0/4 and the greedy, which cannot
  revisit `lo`, stayed spread (`[2,0,13,4,0,3,6,1]`, 29 columns, 392). The start
  matters in the other direction too: the two t = 4 replicates whose `lo` already
  carried a coset-0/4 point (step 0 = (1, 2), reps 2 and 4) ended highest (250).
  An off-coset start is not always fatal: t = 6 reps 1 and 4 also started at
  (1, 0), and rep 1 ended highest (1364). Choosing `lo` by coset, or scanning `lo`
  as a first step, is an obvious fix, but it would be a new lead with its own rule;
  the P9 failure stands as recorded.
- **J vs covered.** J equals covered exactly in every random row and in all t = 4
  and t = 5 greedy sets; at t = 6 the greedy sets have J − covered = 4, 8, 6, 6, 14
  (0.3–1.2 %): at that density some signed pair sums coincide, so J slightly
  overstates covered and would mis-rank near-ties at t ≥ 6.
- **Modelled F4 cost ratio** E(greedy)/E(best translation) =
  (cols_g/cov_g)/(cols_tr/cov_tr), *assuming mpt unchanged* (bits-form system
  shape is identical by construction; this assumption is exactly what the engine
  confirmation would test): **0.436** at t = 5 (22/522 vs 24/248) and **0.403** at
  t = 6 (43/1268 vs 48/570), i.e. 2.30× and 2.48× less modelled F4 work.
- **Against the T10 winner (isogeny tower, t = 5, G = 4, seed 1307: 22 columns,
  210 covered, mpt 52,816).** The t = 5 greedy digit set has the same 22 columns
  and 2.49× its coverage. But the measured bits-form mpt is much larger than the
  isogeny form's at equal (t, G): 3,036,916.11 vs 177,112 at (5, 3) (17.1×,
  `frame_search/engine/frame_t5_g3.json` vs T10) and 45,194,767 vs 710,094 at
  (6, 4) (63.6×, `t7_t0_frames/data/engine/bitsbest_t6_g4.json` vs T10). No bits
  cell at (5, 4) exists. At (5, 3), with mpt(digits) = mpt(bits) by the shape
  argument, E(digit set)/E(isogeny winner) = (22/522 · 3,036,916.11)/(22/210 ·
  177,112) ≈ **6.9×** behind the winner. For comparison, the engine-measured frame
  `bits@976503` (24 columns, 198 covered, the cell behind the 3,036,916.11) is 19.8× behind
  under the same model, (24/198 · 3,036,916.11)/(22/210 · 177,112); this run's best
  translation frame (L = 815271, 24 columns, 248 covered) would be 15.8× behind. The digit set closes most of the bits arm's coverage gap to the
  isogeny tower but not the form's per-target F4 gap.

## What an engine confirmation needs (not done; no Rust written by L14)

Verified against the current source: `src/bin/curve25519_family.rs` form parser
(`"bits"` arms at lines 1645 and 1651) accepts `bits[@LO]` only;
`src/cryptanalysis/ic_framework/pkm_semaev.rs` has `bits_factor_base` at line 449
(`xs = (lo + k) mod p, k < 2^t`), `new_bits` at 821 (`Membership::Bits { lo }`),
the abscissa enumeration at 1427 (`(0..(1<<t)).map(|k| (lo + k) % p)`), and the
membership description at 1619 (`x = lo + Σ 2^j b_j`). A `digits@LO:c1,…,ct` form
would carry the digit vector in the membership, enumerate subset sums in
`bits_factor_base` and at 1427, and scale `Pol::var(n, i·t + j)` by `c_j` instead
of `(1 << j) mod p` in the polynomial build; for ecbench, the same digits on the
`prime-interval` base and the `pkm-semaev` oracle params. Frozen prediction for
that confirmation (PROTOCOL §7): `mpt(digits) = mpt(bits@LO)` within 1 % at equal
(t, G), and `covered_points` equal to this census (522 at t = 5 rep 0, 1268 at
t = 6 rep 0). Candidate cells: t = 5 rep 0 and rep 2 (582 covered, 21 columns),
t = 6 rep 1 (1364 covered, 45 columns), with their `bits@L` controls L = 815271
(t = 5) and L = 889850 (t = 6) from this run.

## Distance remaining to rho-neg

Measured (T12, k = 20, 16 targets): the best PKM arm (isogeny t = 5, G = 4, seed
1307) is 17.1× [13.3, 22.0] behind rho-neg at w = 0, 6.06·10⁴× [4.55·10⁴, 8.07·10⁴]
at w = 0.1 and 3.78·10⁵× at w = 1. L14 measured nothing end-to-end. Under the
T10 model, a bits arm built on the t = 5 greedy digit set would sit about 6.9×
behind that winner at equal G (above), i.e. roughly 4·10⁵× behind rho-neg at
w = 0.1 — further than the protocol's §8 best case (4.2·10⁴×), because §8 did not
price the bits-form mpt penalty. Within the bits family the modelled gain is
2.3–2.5× over the best translation frame. **Rho remains ahead at every weight;
nothing here is an improvement or leaderboard claim.** A verified improvement
would require the engine change above plus an ecbench session priced at
w = 0, 0.1, 1.

## Deviations

- None from the frozen command, seeds, K, B, R or rules. The run was launched
  with `nohup … &` (stdout copied to `data/run_stdout.log`) per the shared-host
  rule; it finished in 11 s.
- `data/SHA256SUMS` as written by the frozen `run.sh` lists the two scripts, the
  instrument and the three census files; this session appended the derived files
  (`summary.json`, `selected_digit_sets.json`, `analysis.txt`, `run.log`,
  `run_stdout.log`) to it without touching the entries the runner wrote. The
  README itself is not listed.
- No Conductor task, resume checkpoint, visual report, benchmarks entry or git
  operation: the wave-4 brief confines writes to this lead directory, forbids git,
  and the result is an exploratory census, not a benchmark row.
- Host: shared 4-core container; other leads' ecbench sessions (T11, L12, L13)
  were running. Counted units do not depend on this.

## Files

- `PROTOCOL.md`, `scripts/{run.sh,digit_census.py,analyse.py}` (frozen).
- `data/k20_t{4,5,6}.json` (raw census: part (a) rows, greedy steps, costs),
  `data/summary.json` (cells and P-table), `data/selected_digit_sets.json` (15
  sets), `data/analysis.txt`, `data/run.log`, `data/run_stdout.log`,
  `data/SHA256SUMS` (`sha256sum -c` passes).

## Validator errata (applied)

Source: `validator-report.json` (pass_with_errata, 2026-11-04T07:08:48Z). Only this README was edited. Prediction verdicts are unchanged (P1, P2, P4, P6, P7, P8 held; P3, P5, P9 failed; lead stays OPEN).

- **Erratum 1 (mpt value), applied.** The bits mpt is 3,036,916.11 (`frame_t5_g3.json`) in both the text and the formula; the text earlier showed 3,036,916 and the formula 3,036,910 (`bits_t5_g3.json` has 3,036,911.76). The 6.9x result is unaffected.
- **Erratum 2 (19.8x vs 15.8x), applied.** 19.8x uses the engine cell bits@976503 (24 columns, 198 covered); this run's best translation frame (L = 815271, 24 columns, 248 covered) gives 15.8x under the same model. Both bases now stated.
- **Erratum 3 (step 0 / step 1), applied.** For t = 5 rep 3, step 0 (the base {lo}) is (1 column, 0 covered); step 1 is (2, 2). The claim that lo carries a point outside cosets 0/4 stands.
- **Erratum 4 (labels ratio), applied.** Labels are about 0.50 of the Legendre tests (0.4975-0.5032), not 0.49.
- **Erratum 5 (replicate sums), applied.** Exact five-replicate sums: 150,000 / 309,999 / 629,998 Legendre and 75,223 / 154,705 / 314,858 labels, within 0.4 % of 5x rep 0 but not equal.
- **Erratum 6 (coset bins), applied.** Bins other than 0 and 4 depend on the generator chosen by `frame_census.build` (`random.Random(1)`) and are now stated as such; bins 0 and 4 and fold-pair totals reproduce exactly.
- **Erratum 7 (sd), applied.** The tables' sd is the population standard deviation (ddof = 0); sample sd for translation would be 10.619 / 26.933 / 69.562. Immaterial to every rule.
