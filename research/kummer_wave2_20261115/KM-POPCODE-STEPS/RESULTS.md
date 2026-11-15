# RESULTS: KM-POPCODE-STEPS, lab D of wave 2 (2026-11-15)

Every number below is `[pcc:unverified]` (Python + numpy development check on one core; not admitted
evidence). `PROTOCOL.md` sha256 `14acb70ef76b708bd9d860d9d1507d14f7d4b0356fc76a0768f197731a333ebd`
(frozen before any sigma datum). The repository copy of `PROTOCOL.md` has `[pcc:unverified]` tags appended to five lines for the prose lint (sha256 `7f117fec3a89052f37430690a3aa6b8cff236bcc021f47e36d3cda76d7217ed1`); no definition or rule changed. `sigma.py` sha256
`5b14e67419142eeccf7ced321e961e3df3742c30f8311158d5e2a95e3ea8c3a3`.

## Headline

Computed with the literal definition of `PROTOCOL.md` §1 (finite window `s <= s_max`):

| quantity | registered | computed (draw 0; draw 1 agrees where run) | verdict |
|:--|:--|:--|:--|
| sigma_1(2), positive control | 4 | 4 (p = 40961, 12289, and the on-disk f1 at p = 786433) | reproduces |
| sigma_1(3) | 8 | **7** (2 draws at p = 40961, 1 at p = 12289; nu_3(8) = 37, nu_3(9..13) = 0) | **fails by 1** |
| sigma_1(4) | 12 | **11** (1 draw; nu_4(12) = 394, nu_4(13) = 0; s = 14 not run) | **fails by 1** |
| sigma_2(3) | 6 | 6 (2 draws; nu_3(7) = 11, nu_3(8..12) = 0) | holds |
| sigma_2(4) | 10 | 10 (2 draws; nu_4(11) = 128, nu_4(12) = 0) | holds, window ends one level later |
| sigma_3(4) | 9 | 9 (2 draws; nu_4(10) = 153, nu_4(11) = nu_4(12) = 0) | holds, window 2 levels |
| sigma_2(2) <= 1 | <= 1 | **2** (nu_2(3) = 1) | fails by 1 (does not enter d_F for t >= 2) |
| sigma_3(3) undefined | no degree-3 multiple | **5** (dim M_3(s) = 0,0,7,20,37,40,40,...; nu_3(6) = 3) | fails: degree-3 multiples exist |
| period 4 | sigma(d+1) - sigma(d) = 4 | W=1: 4,7,11 (gaps 3, 4); W=2: 2,6,10 (gaps 4, 4); W=3: 5,9 (gap 4) | holds at W=2, 3; not at W=1 d=2 -> 3 |

Per `PROTOCOL.md` §4 the claim dies on a mismatch at any registered value; it is dead at W = 1
(`sigma_1(3) = 7 != 8`, `sigma_1(4) = 11 != 12`). The W = 2 and W = 3 values match, with the
W = 3 `sigma_3(3)` and the W = 2 `sigma_2(2)` registrations also wrong. Partial match: period 4 and
the values `sigma_2(3), sigma_2(4), sigma_3(4)` are right; the W = 1 offset is off by one level
after the first step. This is reported as partial and does not count as support.

## Which values were tests (stated in PROTOCOL §3 before computing)

All six registered values are fitted by inversion of a measured on-disk `d_F` step, so none is an
out-of-sample test of its value. The computation tests the definition (nu through `E_d`), without
being shown the `d_F` steps. Result: `sigma_2(3), sigma_2(4), sigma_3(4)` come out as the fitted values;
`sigma_1(3), sigma_1(4)` do not; the control `sigma_1(2)` does.

## nu tables (draw 0, p = 40961; entries for s = 1, 2, 3, ...; `dim M_d(s)` in `raw/*.jsonl`)

| W, d | nu_d(s) | last positive s | sigma = s - 1 | s_max |
|:--|:--|--:|--:|--:|
| 1, 2 | 1,3,5,5,1,0,0,0,... | 5 | 4 | 13 |
| 1, 3 | 1,0,0,0,15,41,51,37,0,0,0,0,0 | 8 | 7 | 13 |
| 1, 4 | 1,0,0,0,0,0,1,4,364,581,673,394,0 | 12 | 11 | 13 |
| 2, 2 | 0,1,1,0,... | 3 | 2 | 12 |
| 2, 3 | 0,1,5,19,29,33,11,0,0,0,0,0 | 7 | 6 | 12 |
| 2, 4 | 0,1,0,0,1,1,63,356,484,490,128,0 | 11 | 10 | 12 |
| 3, 2 | all 0 (`M_2 = 0`) | none | none | 12 |
| 3, 3 | 0,0,7,13,17,3,0,0,0,0,0,0 | 6 | 5 | 12 |
| 3, 4 | 0,1,0,1,1,116,277,367,395,153,0,0 | 10 | 9 | 12 |

Windows: the finite-window caveat applies to every row; W = 1, d = 4 has one zero (s = 13) after
its last positive value, W = 2, d = 4 one zero (s = 12), W = 3, d = 4 two zeros, W = 1, d = 3 five
zeros. `nu_d(s) = 0` beyond `s_max` is not shown.

Second draws (seed offset 1) of W = 2 and W = 3 (d <= 4, s <= 12) and W = 1 (d <= 3, s <= 12) gave
tables identical to draw 0 in every entry. The second prime 12289 (W = 1, d <= 3, s <= 12) is
identical to p = 40961. W = 1, d = 4 was run once (draw 0); the second draw was stopped for time.

## Comparison with the measured step positions (rule `d_F = 1 + min{d : sigma_W(d) > t}`)

Using the computed sigma (W=1: 4,7,11; W=2: 2,6,10; W=3: 5,9), predicted vs measured `d_F`
(`growth_law/README.md` §5; `>=` where sigma of the next degree is not computed):

| W | t | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 1 | measured | 3 | 3 | 4 | 4 | 4 | 4 | 5 | 5 | 5 | 5 |
| 1 | predicted | 3 | 3 | 4 | 4 | 4 | **5** | 5 | 5 | 5 | **>=6** |
| 2 | measured | 4 | 4 | 4 | 4 | 5 | 5 | 5 | 5 | 6 | 6 |
| 2 | predicted | 4 | 4 | 4 | 4 | 5 | 5 | 5 | 5 | >=6 | >=6 |
| 3 | measured | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 6 | | |
| 3 | predicted | **4** | **4** | **4** | 5 | 5 | 5 | 5 | >=6 | | |

W = 2 matches every cell. W = 1 predicts its steps at t = 7 and t = 11, one level before the
measured steps at t = 8 and t = 12 (2 of 10 cells off). W = 3: because degree-3 multiples exist
(`sigma_3(3) = 5`), the rule predicts 4 at t = 2..4 where the measured `d_F` is 5 (the first fall
degree, `ffd(3) = 5`, is a floor the rule does not contain); 3 of 8 cells off.

## Controls

- Positive control: `sigma_1(2) = 4` reproduces; `dim M_2(s) = 4,9,14,15,15,15` and [pcc:unverified]
  `dim M_3(s) = 8,27,64,121,194` (s = 2..6) reproduce the on-disk Rust values on the on-disk
  instance `f1 = 3 + 1234 z1 + 5678 z2 + 9012 z1 z2` over F_786433 (`raw/f1.jsonl`) and on every
  generic W = 1 draw at p = 40961 and 12289. The monomial counts n match too (11, 22, 37, 56, 79, 106
  and 15, 42, 93, 176, 299).
- Negative control N1 (planted grid zero, `f(1,1) = 0`, f irreducible so a unit of no `R_t`):
  its nu tables are **identical** to the generic W = 1 tables (d = 2: 1,3,5,5,1,0..; d = 3:
  1,0,0,0,15,41,51,37,0,..; sigma = 4, 7). So sigma does not see the grid zero. The control does
  not differ; it is a finding that sigma (defined in the infinite tower, with no K) cannot detect
  non-unit-ness, so the rule can only describe units and `d_F = infinity` for N1 is invisible to
  it. As a detector-of-genericity the control is vacuous for the grid-zero property.
- Negative control N2 (lead's own, `(1+z1)(1+z2)`): differs strongly: `dim M_2(s) = s^2`
  (nu_2(s) = 2s - 1 for all s <= 11, no finite sigma in the window), `dim M_3(s) = s^3`
  (1,8,27,...,1331), nu_3 = 0 for s >= 2. The detector sees the reducible f.
- Validity checks: the elimination routine matches a naive rank on 18 random low-rank matrices at
  three primes (`test_la.py`); every kernel was verified against all `W*2^(s+1)` Bezout points
  (`verified: true` in every row); the full-evaluation rank certificate `(N+1)^2 - (N+1-W)^2`
  holds at s = 3, 4 for the three W instances (`raw/cert.log`), and point counts were 40960,
  40683 and 24677 (>= `W*2^(s_max+1)`).

## Commands and runtime

    cd research/kummer_wave2_20261115/KM-POPCODE-STEPS   # code here; cwd-independent otherwise
    python3 test_la.py
    python3 chk.py > raw/cert.log
    python3 run_cell.py KIND W DRAW P D SMAX OUT.jsonl   # see queue.sh for the exact list
    ./queue.sh ; python3 aggregate.py ; python3 fig.py ; python3 summ.py raw/*.jsonl

Environment: `OMP_NUM_THREADS=OPENBLAS_NUM_THREADS=1`, Python 3.13, numpy 2.5.3, host shared with
three other labs. Wall clock about 1.6 h including development; compute: the W=1 d<=4 s<=13 run
about 27 min (s = 13 alone about 11 min dominated by one 17,902 x 16,384 elimination), W=2 d<=4 s<=12
about 11 min per draw, W=3 about 10 min per draw, all d <= 3 ladders under 1 min each.

## Deviations

- Python + numpy (as instructed), not Rust; engine is my own blocked mod-p elimination, not
  `diem_quartic::dense_rank`. Not admitted evidence.
- Coefficients are from `numpy.random.default_rng(0x504B4D54 + 100 W + draw)` at p = 40961 / 12289,
  not the pilot's draws (the primes differ from the ladder primes).
- W = 1, d = 4 ran one draw and stopped at s = 13 (the lead's bound); s = 14 (n = 24,158) was not run,
  so `sigma_1(4) = 11` means "last positive s in the window is 12 and s = 13 is zero".
- The `queue.sh` entry for the second W = 1 d = 4 draw was killed at start; its (empty) files were
  removed.
- Second-prime check is at d <= 3 only.

## What is and is not shown

Shown (`[pcc:unverified]`): under the literal definition, `sigma` is finite within the windows
above, equals the registered value at W = 2 (d = 3, 4) and W = 3 (d = 4), the control, and not at
W = 1 (d = 3, 4), and the period-4 gaps hold at W = 2 and W = 3. The rule-implied `d_F` matches the
measured steps at W = 2 and fails by one tower level at W = 1 and by the first-fall floor at W = 3.
Not shown: that `nu = 0` for all `s` beyond the window; any value at `d = 5` (the `W = 2, t = 14`
prediction needs `sigma_2(5)`, out of reach here); that another reading of `E_d` (the lead's
"primitive" is one choice) would not give 8 and 12 at W = 1; anything about `d_F` itself.
No improvement or speedup claim is made.
