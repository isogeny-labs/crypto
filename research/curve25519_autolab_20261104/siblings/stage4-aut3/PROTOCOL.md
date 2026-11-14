# SIBLING-POINTS Stage 4: order-3 automorphism keys (`aut=3`) on j = 0 curves

Status: frozen before any data of this stage (2026-11-11). Every number below is an expectation or
a prior measurement, labelled `[pcc:unverified]`.

## Question

On j = 0, b = 7 curves (the secp256k1 shape), the matched rho is `rho.cubic`. It walks on orbits
of the order-3 automorphism and negation (A = 6). Stage 3 measured the best sibling arm at 1.48 to
1.54 times rho-cubic at w = 0, flat in r [pcc:unverified]. That gap is the sqrt(3) the
index-calculus walk did not use.

The engine change on this branch adds `wwalk ... aut=3`, which works as follows:
- Every stored or probed point is keyed by min(x, ωx, ω²x), where ζ₃(x, y) = (ωx, y) = [κ](x, y).
- The rotation is stored with the entry.
- A collision at rotation difference d is the row of E_t = ±[κ^d]E_s.
- Each canonicalisation charges two field multiplications, as rho.cubic charges its rotations.
- With `aut` off, all 5,100 replayable runs of four existing sessions reproduce exactly
  [pcc:unverified].

Because the target walk's jumps do not depend on the point, keying by orbit triples the number of
matching pairs without any fruitless cycles. The expected number of stored points before full rank
falls by sqrt(3).

## Design

- Specs `spec-j0b7-{34,37,40}.json`: five prime-order `prime_orbit_shape` j0_b7 curves per size,
  6 public targets per curve, 6 rounds plus 1 warmup, alternating order, L0, timeout 900 s.
- The panel runs locally on this session's host (no AWS identity), with `ecbench` built from
  `b460422af6`, the commit that adds `aut=3`.
- Arms:
  - `rho-neg`, the reference.
  - `rho-cubic`, the matched rho.
  - `rho-neg-aa`, the A/A control.
  - `tw2cj-lazy`.
  - `tw2cj-lazy-aut3`.
  - `sib8-p512`.
  - `sib8-p512-aut3`.
- Analysis, as in Stage 3:
  - `ecbench compare --save` at w = 0, 0.1 and 1.
  - `ecbench band --ranks`.
  - `stats_bitops.py`.
  - `scaling_fit.py` (Stage 3's script with this stage's arm pairs), as a derived statistic.

## Pre-registered expectations [pcc:unverified]

- **H1.** tw2cj-lazy-aut3 / tw2cj-lazy, GAE at w = 0: in [0.50, 0.66] at every size. 1/sqrt(3) is
  0.577 on the walk; the set-up is unchanged.
- **H2.** sib8-p512-aut3 / sib8-p512, GAE at w = 0: in [0.50, 0.66] at every size.
- **H3.** sib8-p512-aut3 / rho-cubic, GAE at w = 0: the 95 percent interval's upper end is below
  1.05 at every size. The point estimate is expected near 1.52 × 0.58 ≈ 0.88. The band order against
  rho-cubic is expected to be unresolved: the second points are priced at w.
- **H4.** The slope of ln(sib8-p512-aut3 / rho-cubic) on ln r has an interval containing 0, so the
  result is a constant factor.

## Admission

Each item is checked per spec. A failure voids that spec; it is reported, not re-run under the same
name.
- Every record verified.
- `verify --replay 8` reproduces.
- The A/A ratio is 1.

## Not claimed

- No leaderboard or `benchmarks/README.md` row: that needs the validation gate.
- No claim about secp256k1 itself.
- A ratio below 1 against rho-cubic at w = 0 only would be "unresolved in the band", not a win
  (§5a).
