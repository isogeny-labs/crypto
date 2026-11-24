# L25 — matched-count k=30 window vs pullback (2026-11-24)

**Lead:** L25 (spawned by L13) in `crypto-autoresearcher/docs/open-leads-20261123.md`.
**Question:** the L13 k=30 timings (window 30.4 s, ideal degree 49; pullback
24.5 s, ideal degree 37) were not controlled, because each family used its own
decomposable target. Does the window/pullback time gap survive at a **matched
msolve ideal degree**?

**Answer:** yes at this cell. At matched ideal degree 60, pullback is
22.91 s mean wall vs window 28.07 s (8 threads, two alternating reps), a factor
1.225. The uncontrolled L13 ratio was 1.24. Matching the degree does not remove
the gap, so pullback is a genuine cheaper decomposition at k=30 — not an artifact
of the degree mismatch.

## Cell and instrument

- Field `p = 4099`; curve `y^2 = x^3 + x + 1`; `m = 3` (S4 + three membership
  polynomials of degree `k = 30`); one seed, one curve.
- `msolve 0.11.0`, `-t 8`, `-v 2`, wall-clock seconds on a 14-core macOS host.
- **`msolve` "degree of ideal" is the quotient dimension / solution count, not
  `d_reg`.** `d_reg` remains the calibrated `f4_fp` metric (8/8 semi-regular
  cells, `crypto/tests/f4_fp_calibration.rs`, PR #5134). No `d_reg` claim is made
  here.
- The two systems are byte-identical except for the base x-set and target:

| family | target `xQ` | ordered decomposition triples | input SHA-256 |
|---|---:|---:|---|
| window | 2037 | 27 | `ccd70352f57afb1cb860c9d29290315ce0423cfd648b84226895e9cc8b26f4a6` |
| pullback | 2575 | 6 | `2ae4a67c4e63c591b8eed3a6bb8ccbb44c9218e2d8b436f564e593df228e7fa3` |

Both systems report `degree of ideal = 60` (checked from `msolve` stderr and the
degree line in the output file).

## Target selection (why these two)

We enumerated all ordered triples of base points for each family, grouped
reachable targets by their ordered-triple count, then ran one representative per
count. Degrees are not monotone in the triple count, and 60 occurs in both
families:

| family | triples | ideal degree |
|---|---:|---:|
| window | 6 | 42 |
| window | 13 | 49 |
| window | 27 | **60** |
| window | 48 | 105 |
| pullback | 6 | **60** |
| pullback | 13 | 37 |
| pullback | 27 | 69 |
| pullback | 48 | 84 |

The scan used `-t 2` with four concurrent jobs and is a **selection** run, not a
timing run. Only the matched pair below is used for timing.

## Matched-degree timing

Two reps, families alternated, same host, same `-t 8`:

| rep | family | wall s | msolve elapsed s | msolve CPU s | ideal degree |
|---:|---|---:|---:|---:|---:|
| 0 | window | 28.15 | 28.04 | 34.49 | 60 |
| 0 | pullback | 23.16 | 23.13 | 28.11 | 60 |
| 1 | window | 27.98 | 27.94 | 34.16 | 60 |
| 1 | pullback | 22.65 | 22.61 | 27.78 | 60 |
| mean | window | **28.07** | 27.99 | 34.33 | 60 |
| mean | pullback | **22.91** | 22.87 | 27.95 | 60 |

Ratio of means: `28.07 / 22.91 = 1.225`. Wall-time difference: 5.16 s.

## Exact commands

```sh
# Selection scan (8 runs): produces the degree table above.
python3 scripts/l25_matched_count.py scan

# Matched pair: 2 alternating reps, writes matched60.json.
python3 scripts/l25_matched_count.py run

# Direct reproduction of either system without Python:
msolve -f research_notes/l25_matched_count_k30_20261124/window_k30_xQ2037.ms \
       -o /tmp/win.out -t 8 -v 2
msolve -f research_notes/l25_matched_count_k30_20261124/pullback_k30_xQ2575.ms \
       -o /tmp/pull.out -t 8 -v 2
```

The system builder in `scripts/l25_matched_count.py` is the same S4/membership
construction as `scripts/ic_semaev_msolve.py` (PR #5137); it is repeated here so
this record does not depend on an unmerged branch.

## What this establishes

- At `p = 4099`, `k = 30`, one curve, the window/pullback gap is **not** explained
  by the 49-vs-37 ideal-degree mismatch: at matched degree 60 the pullback target
  is still ~1.23x faster.
- Ideal degree varies by a factor >2 across targets at fixed `k` (37 to 105), so
  any future window-vs-pullback timing must name the target and degree.

## What this does not establish

- Not a scaling law: one prime, one curve, one seed, one `k`. No claim at
  `k = 40/50` or at cryptographic `p`.
- Not a `d_reg` measurement; `msolve` ideal degree is not `d_reg`.
- Not a statement about relation yield or the full IC cost; only the dense
  Gröbner-basis step is timed.
- Wall times are host/load dependent; the paired alternation controls drift, not
  absolute throughput.

## Consequence for L5

L5's target `k = 50` still exceeds the 15-minute budget on the default window
target. This result says a cheaper target at the same `k` can change the degree
substantially (37 to 105 at `k = 30`), so the next L5 attempt should first select
a low-degree target, not simply buy a longer timeout. A `k = 50` run remains
unmeasured here and would exceed the 30-minute per-cycle compute cap.
