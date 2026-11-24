# Semaev + membership cell through the repo's own F4 — first numbers (2026-11-21)

**Status:** experiment note, unreviewed. Toy scale. This is the *first* run of the
prime-field Semaev decomposition system through `cryptanalysis::f4_fp`; it is
not a degree-of-regularity result and does not claim one.

## Why this exists

`crypto-autoresearcher`'s G30-004 records that no true d_reg backend exists and
that it must be commissioned. That premise is wrong for this workspace:
`src/cryptanalysis/f4_fp.rs` **is** a degree-bounded F4 over `F_p` that reports
`solving_degree`, `degree_reached`, `max_rows/max_cols`, and enforces a deadline.
Its own tests pass (`cargo test --lib f4_fp`: 5/5, including agreement with
brute force on random zero-dimensional systems and degree-bound truncation
reporting). So the blocker is integration and cell design, not availability.

## What was run

`examples/ic_semaev_gb.rs`:

- `S4(x1,x2,x3,xQ)` built by the two-quadratic resultant identity
  `Res_D(S3(x1,x2,D), S3(x3,x4,D)) = (A1C2 - A2C1)^2 - (A1B2 - A2B1)(B1C2 - B2C1)`
  with `S3 = A D^2 + B D + C` per pair. Validated: **20/20 genuine 4-sums vanish**
  (points chosen with distinct abscissae so the two-quadratic formula applies).
- Membership polynomials `h(x_i) = prod (x_i - w)` over the base x-set.
- Two bases of the *same* x-count: window (k smallest x) and pullback
  (`x^2 in T`, `|T| = k/2`, so `h` is even and has the same degree k).
- Per-family decomposable target (a target from one base need not decompose in
  the other), so both systems are consistent in every cell.

## Results (p = 4099, y² = x³ + x + 1, #E = 4198)

| k | family | deg h | inconsistent | solving_degree | degree_reached | rows | cols | ms |
|---|---|---|---|---|---|---|---|---|
| 6 | window | 6 | false | 3 | 16 | 792 | 639 | 1972 |
| 6 | pullback | 6 | false | 3 | 16 | 795 | 656 | 1718 |
| 8 | window | 8 | false | 3 | 21 | 1832 | 1533 | 10399 |
| 8 | pullback | 8 | false | 3 | 21 | 1832 | 1555 | 10731 |

**Reading:** at matched membership degree the two bases are indistinguishable —
same solving degree, same reached degree, same matrix shape within noise. That
is what the degree bookkeeping predicts: composing a degree-2 map with a
degree-|T| target polynomial gives membership degree `2|T| = k`, exactly the
window's degree. So *this* reconstruction of the parametrized-base idea has no
Gröbner-side advantage, consistent with the sumset null found in
`g32-001-pullback-pilot-20261121` (identical densities and sumset sizes).

## Limitations (stated, not hidden)

- k ≤ 8. k ≥ 10 exceeded a 120 s budget (rows ~3–5k, degree reached ~22–24).
- No calibration of `solving_degree` against a system with a published value;
  the small values (3) suggest the bounded F4 is resolving through a low-degree
  path in these near-empty ideals, so the metric should be treated as a
  relative counter here, not as d_reg.
- The base is small relative to p, so each ideal has few solutions and every
  degree statistic is in its degenerate regime (cf. the G30-004 status note:
  a non-degenerate cell needs |B| ≈ (30p)^{1/3} ≈ 50 at this prime).
- The reconstruction of the Petit–Kosters–Messeng construction is *ours*; the
  paper has not been read. If their map/target set differ, this cell tests the
  wrong object.

## Bug worth recording

The first version wrote the `C` coefficient of S3 with `+4b(x1+x2)` instead of
`-4b(x1+x2)`, which produced a polynomial that failed on genuine 4-sums
(assert caught it immediately). Anyone re-deriving S4 from the resultant should
check against the group law before trusting a basis.

## Calibration (same day): `solving_degree` is trustworthy

`examples/ic_gb_calibrate.rs` runs homogeneous semi-regular systems (m quadrics
in n variables, m = n+1 and n+2) and compares `solving_degree` with the
Bardet–Faugère–Salvy prediction (first non-positive coefficient of
`(1-t^2)^m/(1-t)^n`, computed in exact integer arithmetic).

| n | m | predicted d_reg | solving_degree |
|---|---|---|---|
| 3 | 4 | 3 | 3 |
| 3 | 5 | 3 | 3 |
| 4 | 5 | 3 | 3 |
| 4 | 6 | 3 | 3 |
| 5 | 6 | 4 | 4 |
| 5 | 7 | 3 | 3 |
| 6 | 7 | 4 | 4 |
| 6 | 8 | 4 | 4 |

**8/8 exact.** The instrument is calibrated against the standard prediction, so
the small values in the table above are real properties of those (near-empty)
ideals, not a bug. This is the first calibration of a prime-field d_reg
instrument in this workspace and it should be quoted whenever the metric is
used.

## Scaling wall (same day)

The dense F4 times out at k = 12 and 16 with a 120 s budget (degree reached
21–24, ~4,600 rows):
`k=12 window solving_degree=5 timed_out=true`, `k=12 pullback 3 timed_out=true`,
`k=16 window 5 timed_out=true`, `k=16 pullback 3 timed_out=true`.
The differing `solving_degree` values under timeout are not evidence — both
runs were truncated.

So the instrument as written covers membership degrees up to ~10 at this prime.
Reaching |B| ≈ (30p)^{1/3} ≈ 50 needs the sparse/dense split moved to `msolve`
(F4 over F_p, available at `/opt/homebrew/bin/msolve`), or a much larger budget
with the degree bound raised.

## msolve cross-check: the wall is the in-repo F4, not the system

`IC_GB_DUMP=<dir> cargo run --release --example ic_semaev_gb -- k...` now
writes the same four polynomials in msolve syntax (variables, characteristic,
comma-separated equations) and skips the in-repo solve. Running msolve 0.11
with `-t 4` on those exact files:

| k | family | in-repo F4 | msolve | ideal degree (window) |
|--:|--------|-----------:|-------:|----------------------:|
| 8 | window | 10.4 s | 0.03 s | 1 |
| 10 | window | — | 0.05 s | 1 |
| 12 | window | timeout 120 s | 0.42 s | 13 |
| 12 | pullback | timeout 120 s | 0.10 s | 1 |
| 16 | window | timeout 120 s | 1.11 s | 13 |
| 16 | pullback | timeout 120 s | 0.20 s | 1 |
| 24 | window | — | 6.3 s | 19 |
| 32 | window | — | 66 s | 49 |
| 50 | window | — | timeout 180 s | (350 seen mid-run) |
| 50 | pullback | — | timeout 180 s | (73 seen mid-run) |

The k = 12/16 timeouts are an implementation wall in the repo's degree-bounded
F4 (full Macaulay expansion to a degree bound), not a property of this system:
msolve solves the identical polynomials in under a second. On msolve the wall
moves to about k = 32 (66 s, ideal degree 49); k = 50 is still out of reach at
180 s, with the membership degree dominating (the pullback basis reaches
degree 73 against the window's 350 at the same k).

Two cautions for anyone quoting this: the degree shown is the ideal degree of
the msolve output; at k ≥ 12 msolve introduces an auxiliary variable `A`, so
the numbers are not directly comparable to the F4 report's `degree_reached`.
And every cell here is one planted target on one small curve, so the growth
`1 → 13 → 19 → 49` over k = 8 → 32 is a scaling record, not an exponent.

## Next

1. The |B| ≈ (30p)^{1/3} cell still needs a backend: msolve carries k = 32 in
   66 s and neither backend reaches k = 50 in 180 s. Try a sparser
   representation (the pullback basis is already cheaper at k = 50: degree 73
   against 350) or a smaller prime where (30p)^{1/3} is reachable, then
   separate the effect of |B| from the effect of p.
2. Read the PKM paper before any further parametrized-base compute.
3. The calibration is now `f4_fp::tests::solving_degree_tracks_the_semiregular_prediction`
   (8/8 cells), so the metric cannot drift silently.

## msolve port (L13, 2026-11-23)

`scripts/ic_semaev_msolve.py` emits the same system (S₄ via the two-quadratic
resultant, plus `h(x_i)` membership polynomials) in msolve's input format and
times msolve's F4. p = 4099, y² = x³ + x + 1, 8 threads:

| k (membership degree) | family | msolve wall | ideal degree | in-repo F4 |
|---:|---|---:|---:|---|
| 8 | window | 0.04 s | 1 | 1.9 s |
| 12 | window | 0.62 s | 13 | **timed out at 120 s** |
| 20 | window | 3.71 s | 13 | — |
| 20 | pullback | 3.59 s | 13 | — |
| 30 | window | 30.4 s | 49 | — |
| 30 | pullback | 24.5 s | 37 | — |
| 40 | window | 164.7 s | 85 | — |
| 50 | window | **> 900 s (timeout)** | — | — |

**What this establishes:** the dense in-repo F4 covers k ≤ ~10 at this prime;
msolve covers k ≤ 40 in under three minutes, and the target cell
|B| ≈ (30p)^{1/3} ≈ 50 is just beyond the 15-minute budget. The frontier is a
factor ~4 in k, not an asymptotic change.

**What it does not establish:** the k=30 window-vs-pullback difference
(30.4 s vs 24.5 s) is *not* a controlled comparison — the two runs have
different ideal degrees (49 vs 37) because each family uses its own
decomposable target. Comparing solver cost across families requires matching
the solution count, which is the next design step.

Reproduce:

```sh
python3 scripts/ic_semaev_msolve.py 8 12 20 30
```
(add `40` or `50` for the long cells; each run prints k, family, actual base
size, wall time, and msolve's reported ideal degree).
