# Timeout note: re-sized card timeouts for lane A, P3 (2026-11-02)

This file is bound by no card. It exists because the P3 cards bind `PROTOCOL.md`, whose amendment A1 (section 10) fixed the
timeout policy "the larger of 1800 seconds and 4 times the section 7 CPU estimate of that command". The note changes no
protocol value, no argv, no env, no binary, no pin (`d97f5953b24e05096208a54df6e699827d021541`, binary sha256
`73f1ff63410bdf819b8c8e691e91f3739714b390c6a154737289a2320aed3299`). It is filed before any data of the re-sized cards exist.
It reads no result: only the `build_s`, `dreg_s`, `solve_s` timing fields and the `k` of the partial rows, and the wall seconds
of the receipts, were used.

## 1. What happened

`CARD-isogeny-ic-yield-p3-b-badapt-20261102-1` (lane A, position A9) ran 1872.104 s, hit its card timeout (1872 s = 4 x 468) and
left 1713 of 2112 rows (264 per (k, attempt) cell: 263 curves, E0 with two borrowed presentations; k = 3,4,5,6 x attempts 0,1).
Partial output (not pooled, not analysed, not cited): `p3_ecc2k130/full/_failed/CARD-isogeny-ic-yield-p3-b-badapt-20261102-1-attempt1/p3b_badapt.jsonl`,
sha256 `eaf22cc4a5202ea5e1fc6b3078b5a6909d943a12f44f93ed73feb31d32a16ebc` (the receipt's output digest).

## 2. Cause: untimed solution count

The section 7 estimate (468 s) is the sum of the timed fields (`build_s` + `dreg_s`). `measure_instance`
(`harness/src/main.rs` lines 452-475) also counts the solutions of each system before it starts the clock for `dreg_s`
(`solutions` column, `--count` default 1). For m = 3, n = 131 and 3k <= 21 that is `count_m3_numeric`, which enumerates all
(2^k)^3 triples with about 7 field multiplications each. Its cost is in no timed field, grows by 8 with every k, and is large
when the modulus is dense (a b-adapted presentation has modulus weight 59-75; the base modulus has weight 5).

Evidence from the receipts (wall seconds minus the sum of the timed fields of the rows of the same card):

| card | wall s | timed s | wall / timed | moduli |
| --- | --- | --- | --- | --- |
| b-main-k34 (k=3,4, 1052 rows) | 1435.422 | 1433.1 | 1.00 | base (weight 5) |
| b-ctrl (k=3,4,5, 216 rows) | 1018.437 | 1015.9 | 1.00 | base |
| a-main-k5 (n=43) | 564.765 | 563.9 | 1.00 | n=43, fast count |
| b-stdv s0a (k=3..6, 528 rows) | about 2652 (file times 07:50:23 to 08:34:35) | 2629.4 | 1.01 | sparse alternative moduli |
| b-stdv s0b | about 2824 | 2799.8 | 1.01 | sparse alternative moduli |
| b-badapt attempt 1 (1713 rows) | 1872.104 | 226.5 | 8.27 | dense (weight 59-75) |

Timed seconds of the partial rows (mean per row): k=3 0.016, k=4 0.063, k=5 0.208, k=6 0.584 (129 rows). Their sum 226.5 s;
untimed 1872.104 - 226.5 = 1645.6 s. Rows done: k=3 528, k=4 528, k=5 528, k=6 129.

## 3. Re-sized estimate and timeouts

Untimed cost per row modelled as u6 / 8^(6-k) (the loop count of `count_m3_numeric`), u6 = untimed seconds of one k=6 row:

- 1645.6 = u6 x (129 + 528/8 + 528/64 + 528/512) = u6 x 204.28, so u6 = 8.056 s (k=5: 1.007, k=4: 0.126, k=3: 0.016).
- Full b-badapt line, 528 rows per k: timed 8.1 + 33.3 + 109.7 + 308.4 (528 x 0.584) = 459.6 s (the section 7 figure 468 is this);
  untimed 528 x (8.056 + 1.007 + 0.126 + 0.016) = 4859.8 s; estimate 5319.4 s, taken as 5320 s.
- Bound if the model is wrong and all untimed time sat in the k=6 rows: u6 = 1645.6 / 129 = 12.76 s, total 459.6 + 528 x 12.76 =
  7195 s; the timeout below is 2.96 x that bound.

Timeouts by the A1 policy (4 x estimate), estimates re-based:

| card | tag | position | old timeout | estimate (s) | new timeout (s) |
| --- | --- | --- | --- | --- | --- |
| CARD-isogeny-ic-yield-p3-b-badapt-20261102-2 | b-badapt | A9 | 1872 | 5320 | 4 x 5320 = 21280 |
| CARD-isogeny-ic-yield-p3-b-bsparse-20261102-2 | b-bsparse | A10 | 2304 | 1482 | 4 x 1482 = 5928 |
| CARD-isogeny-ic-yield-p3-b-main-k5-s1-20261102-1 | b-main-k5-s1 | A11 | 10800 | 2700 | unchanged, 10800 |

- b-bsparse: section 7 timed estimate 576 s (4 cells k=5,6 x attempts 0,1). Rows per cell: 7 curves x 6 presentations + 8
  nbadapt = 50, all with dense moduli except the E0 `z` row (ignored, an over-count). Untimed: 100 k=6 rows x 8.056 + 100 k=5
  rows x 1.007 = 806.0 + 100.7 = 906.3 s. Estimate 576 + 906.3 = 1482.3 s, taken as 1482. The old timeout 2304 is 1.55 x that.
- b-main-k5-s1: base modulus (weight 5) and random V; the wall / timed ratio of the base-modulus cards above is 1.00 (b-ctrl,
  72 k=5 rows, untimed 2.5 s in all). Untimed estimate 263 rows x 0.035 s = 9 s, negligible against the 2700 s estimate; no
  n=131 k=5 dmax 7 receipt exists yet (lane B card b-main-k5-s0 has not run), so 2700 s stands, and 10800 s is 4.0 x it. Card unchanged.

The two new cards differ from their `-1` cards only in `card_id`, `receipt`, `stdout`, `stderr` (all derived from the card id)
and `timeout_seconds`. The `CARD-isogeny-ic-yield-p3-b-bsparse-20261102-1` card was never run; it is superseded by its `-2` and
is not to be launched. The `CARD-isogeny-ic-yield-p3-b-badapt-20261102-1` card is spent (its receipt exists).

## 4. Queue for lane A

A9' = `CARD-isogeny-ic-yield-p3-b-badapt-20261102-2`, then A10 = `CARD-isogeny-ic-yield-p3-b-bsparse-20261102-2`, then A11 =
`CARD-isogeny-ic-yield-p3-b-main-k5-s1-20261102-1`; one at a time, launched detached as in `LAUNCH_NOTE.md`. Lane B and P2 are
unaffected. Expected CPU seconds of lane A from here: about 5320 + 1482 + 2700 = 9502 s (2.6 h), against 3170 s in the
section 10 table. Other cards still to run were checked against the same table: `b-stdv-s1` (sparse alternative moduli, ratio
1.01 as s0) and `b-main-k5-s0` (base modulus) have no untimed-count problem.
