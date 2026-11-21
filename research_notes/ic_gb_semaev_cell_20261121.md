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

## Next

1. Calibrate `solving_degree` on a cell with a known published value.
2. Move to |B| ≈ (30p)^{1/3} with msolve for the dense steps, or raise the F4
   degree bound and budget.
3. Read the PKM paper before any further parametrized-base compute.
