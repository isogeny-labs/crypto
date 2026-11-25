# L25 — matched-count k=30 window vs pullback (2026-11-24)

**Lead:** L25 (spawned by L13) in `crypto-autoresearcher/docs/open-leads-20261123.md`.
**Question:** the L13 k=30 timings (window 30.4 s, ideal degree 49; pullback
24.5 s, ideal degree 37) were not controlled, because each family used its own
decomposable target. Does the window/pullback time gap survive a controlled
comparison at k=30?

**Answer:** yes, once the control variable is chosen correctly. The k=30 gap
disappears when both families use the **same target and the same x-space solution
count** (30): window 17.99 s mean vs pullback 18.32 s mean (ratio 1.02, and the
ordering flips between reps). Matching only the msolve quotient degree
(60 vs 60) is **not** sufficient: the x-space counts are then 27 vs 6 and a
1.23x gap remains. The control variable is x-space solution count, not quotient
degree.

This confirms, at k=30 rather than k=20, the result already recorded in the
leads doc: `matchedx` at k=20 (count 15, degree 15 both) gave 3.32 s vs 3.28 s.

## Cell and instrument

- Field `p = 4099`; curve `y^2 = x^3 + x + 1`; `m = 3` (S4 + three membership
  polynomials of degree `k = 30`); one seed, one curve.
- `msolve 0.11.0`, `-t 8`, `-v 2`, wall-clock seconds on a 14-core macOS host.
- **`msolve` "degree of ideal" is the quotient dimension / solution count, not
  `d_reg`.** `d_reg` remains the calibrated `f4_fp` metric (8/8 semi-regular
  cells, `crypto/tests/f4_fp_calibration.rs`, PR #5134). No `d_reg` claim is made
  here.
- "x-space solution count" = ordered triples `(x1,x2,x3)` of base x-values with
  `S4(x1,x2,x3,xQ) = 0` over `F_p`, computed directly from the curve arithmetic.

## Primary result: matched x-space count 30, same target

Both families use `xQ = 3106`; each has 30 x-space solutions; msolve reports
ideal degree 30 for both. Two reps, families alternated, same host, `-t 8`:

| rep | family | wall s | msolve elapsed s | msolve CPU s | ideal degree |
|---:|---|---:|---:|---:|---:|
| 0 | window | 18.11 | 18.07 | 21.34 | 30 |
| 0 | pullback | 17.31 | 17.30 | 20.97 | 30 |
| 1 | window | 17.87 | 17.86 | 21.16 | 30 |
| 1 | pullback | 19.33 | 19.32 | 21.83 | 30 |
| mean | window | **17.99** | 17.97 | 21.25 | 30 |
| mean | pullback | **18.32** | 18.31 | 21.40 | 30 |

Ratio of means: `18.32 / 17.99 = 1.018`. The per-rep ordering is not stable
(rep 0 pullback faster, rep 1 window faster), so the gap is within run-to-run
noise at this cell. The uncontrolled L13 ratio was `30.4 / 24.5 = 1.24`.

| family | target `xQ` | x-space count | input SHA-256 |
|---|---:|---:|---|
| window | 3106 | 30 | `f57631c10d0c8687f9f02563608be80abef8e8fae668b7277338cae027c5b60d` |
| pullback | 3106 | 30 | `bcad84a2979b4b9e6e06f4fbccd6218af2ef745cea80ea48b1005bded0f2404e` |

## Control: matching quotient degree alone is insufficient

A separate search selected targets with the same msolve ideal degree (60) but
different x-space counts (window 27, pullback 6). That pair still shows a gap:

| family | target `xQ` | x-space triples | mean wall (2 reps, `-t 8`) | ideal degree |
|---|---:|---:|---:|---:|
| window | 2037 | 27 | 28.07 s | 60 |
| pullback | 2575 | 6 | 22.91 s | 60 |

Ratio `28.07 / 22.91 = 1.225`. So matching the quotient dimension does not
control the comparison; L25's "solution count" must mean the x-space count.

The degree scan that found the degree-60 pair (selection run only, `-t 2`,
four concurrent jobs) also shows that quotient degree is not monotone in x-space
count:

| family | x-space triples | ideal degree |
|---|---:|---:|
| window | 6 | 42 |
| window | 13 | 49 |
| window | 27 | **60** |
| window | 48 | 105 |
| pullback | 6 | **60** |
| pullback | 13 | 37 |
| pullback | 27 | 69 |
| pullback | 48 | 84 |

## Exact commands

```sh
# Primary: matched x-space count 30 at k=30, 2 alternating reps, -t 8.
python3 scripts/l25_matched_count.py runx

# Control: matched quotient degree 60 with different x-space counts.
python3 scripts/l25_matched_count.py run

# Selection scan for the degree-60 control (8 runs, -t 2, 4 concurrent).
python3 scripts/l25_matched_count.py scan

# Direct reproduction of the primary pair without Python:
msolve -f research_notes/l25_matched_count_k30_20261124/window_k30_xQ3106.ms \
       -o /tmp/win.out -t 8 -v 2
msolve -f research_notes/l25_matched_count_k30_20261124/pullback_k30_xQ3106.ms \
       -o /tmp/pull.out -t 8 -v 2
```

The system builder in `scripts/l25_matched_count.py` is the same S4/membership
construction as `scripts/ic_semaev_msolve.py` (PR #5137); it is repeated here so
this record does not depend on an unmerged branch.

## What this establishes

- At `p = 4099`, `k = 30`, one curve, matching the **same target and x-space
  solution count** removes the window/pullback timing gap (17.99 vs 18.32 s,
  ratio 1.02).
- The original 30.4-vs-24.5 s gap is explained by the uncontrolled solution
  count, not by a structural advantage of the pullback base.
- Matching the quotient degree alone is not enough: degree 60/60 with x-space
  counts 27/6 still leaves a 1.23x gap.

## What this does not establish

- Not a scaling law: one prime, one curve, one seed, one `k`. No claim at
  `k = 40/50` or at cryptographic `p`.
- Not a `d_reg` measurement; `msolve` ideal degree is not `d_reg`.
- Not a statement about relation yield or the full IC cost; only the dense
  Gröbner-basis step is timed.
- Wall times are host/load dependent; paired alternation controls drift, not
  absolute throughput.

## Consequence for L5

L5's target `k = 50` still exceeds the 15-minute budget on the default window
target. At fixed k, target choice moves the quotient degree by >2x (37 to 105 at
k=30), but the primary result says the *x-space solution count*, not the quotient
degree, is the cost-relevant control. The next k=50 attempt should first select a
low-x-space-count target, then price that cell; k=50 is still unmeasured and a
run would exceed this cycle's 30-minute cap.
