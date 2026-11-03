# E5c protocol: is G a small multiple of the first point on P-192 or P-224?

**Frozen 2026-10-19, before any E5c run.** Committed alone, before the
example that runs the scan exists. Any later change is an amendment with its
own date, listed below the original text.

**No ECDLP run. No security claim.** E5c is an exact equality test of a stated
derivation rule, not a statistic.

## The rule tested

Trapdoor program `nist_trapdoor_slot_program_20261011/PROTOCOL.md` §10, slot
S4, experiment E4a, rule "`G = [h]·(first point)`". E5a excluded only
`P_first = ±[i]G` for `i ≤ 2^24`, that is `h ≡ ±i⁻¹ (mod n)`; it does not
cover small `h > 1`. E5c tests the direct direction:

> **R(P, H):** there is an `h` with `1 ≤ h ≤ H` and `[h]·P = ±G`.

## Curves and base points

`CurveParams::p192()` and `CurveParams::p224()` in `src/ecc/curve_zoo.rs`.
Both have cofactor 1, so every affine point generates the group.

- **Primary.** `P_first`: the point with the smallest canonical abscissa
  `x ∈ [0, p)` for which `x³ − 3x + b` is a square (zero included) in `F_p`
  (`x = 0` on P-192, `x = 3` on P-224, recorded by the census and by E5a).
  Both roots `y` and `p − y`. Since `[h](−P) = −[h]P`, comparing with `±G`
  covers both roots in one scan; the program still reports both roots.
- **Extension.** The first 16 valid abscissae of each curve (the primary
  point included), each with one root; the other root is covered by `±G` as
  above.

`H = 2^24` for every base point.

## Method

`Q_1 = P`, `Q_h = Q_{h−1} + P` in Jacobian coordinates (Montgomery field
arithmetic of `examples/generator_effect_e5a.rs`), split into chunks of `2^16`
consecutive `h`; each chunk starts from `[h_0]P` by double-and-add and is
normalised with one batched inversion. Every affine `x([h]P)` is compared with
`G.x`; on equality `y` is compared with `G.y` and `p − G.y`.

Recorded per base point: the matches `(h, sign)`; and, as a check that the
scan visited `H` distinct-looking points, the histogram of
`c(h) = ℓ − bitlen(x([h]P) XOR G.x)`, the number of leading bits shared with
`G.x` (expected `P(c ≥ k) ≈ 2^{−k}`, so the maximum over `2^24` values is
about 24; an exact match has `c = ℓ`).

## Checks that must pass before the result is read

1. `G` and every base point on the curve; `[n]G = O` (BigUint arithmetic of
   `src/ecc`).
2. The fast arithmetic agrees with `Point::scalar_mul_vartime` on
   `x([h]P_first)` and `y([h]P_first)` for
   `h ∈ {1, 2, 3, 1000, 12345, 2^16, 2^16 + 1, 2^20, 2^24}`, on both curves.
3. **Positive control.** On each curve, the same scan of `P_first` against
   the planted target `T = [12345]·P_first` (computed with the BigUint
   arithmetic, not the fast path) must report exactly one match, `h = 12345`
   with sign `+`; against `−T` exactly one match, `h = 12345`, sign `−`.

If a check fails, the result is not read and the failure is reported.

## Prediction

**No match** for any base point on either curve: `G ≠ ±[h]·P` for
`1 ≤ h ≤ 2^24` and every base point above. Under a random `G` the chance of
any match is about `16 · 2 · 2^24 / n ≈ 2^{−163}` on P-192, smaller on P-224.

A match would mean `G` was derived from the first point (or an early valid
abscissa) by a small scalar: the mundane direction of S4 for that curve, to be
reported with `h` and the sign.

## What a null result closes

Rule E4a "`G = [h]·(first point)`" for `1 ≤ h ≤ 2^24`, with either root of the
first point, on P-192 and P-224. Together with E5a (`h ≡ ±i⁻¹ (mod n)`,
`i ≤ 2^24`) this covers both small `h` and small `h⁻¹`. It closes nothing
about larger `h`, the SHA-1 rules of E4a, or E4b.

## Cross-reference

E5a protocol `e5/PROTOCOL.md`; report `E5.md`, "Cross-reference: trapdoor
program slot S4".
