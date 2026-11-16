# Amendment S56: the same construction at the 56-bit ladder prime

Amendment to `PROTOCOL.md` of study `nist_cover_quartic_map_20261116`, written
and frozen before any 56-bit data was taken. Owner request (2026-11-16):
"Try s56 prime". Exact-correctness construction; no timing or cost claim; no
leaderboard, `benchmarks/README.md` row or bench record. Every result is
`[pcc:unverified]` until independently replayed.

## Target

The 56-bit rung of the lab's deterministic prime-field ladder,
`prime_fast::find_a3_curve(56, 3)`: `p` the largest prime `= 3 mod 4` below
`2^56` (`p = 2^56 - 5`), `a = -3`, the smallest `b` giving prime order `n`,
the smallest-`x` generator `G`. NIST shape: prime field, `a = -3`, cofactor 1.

## What changes in the code (semantics-preserving below 2^31)

The Smith stack was bounded to `p < 2^31` (u64 products). Widened to
`p < 2^62`:

* `md` (modular product) in `smith_prop24`, `smith_trigonal`,
  `smith_canonical`: the original u64 expression when `p < 2^32`, a u128
  product above. Identical values and identical operation sequence below 2^32.
* `smith_prop24::edf` and `kp_one_root`: the exponent `(p^d - 1)/2` overflows
  u128 for `d >= 3` at 56 bits. When (and only when) `p^d` does not fit, the
  power is computed exactly as
  `a^((p^d-1)/2) = (prod_{i<d} a^(p^i))^((p-1)/2)`, from
  `p^d - 1 = (p - 1)(1 + p + ... + p^(d-1))`. Same random draws.
* The `p < 2^31` bounds of `analyze_curve` and `Trigonal::new` are raised to
  `2^62`.
* `fit_quartic` gains a `u64` core `fit_quartic_u64`; the `u32` function is a
  wrapper (asserts `p < 2^32`).

Regression gate before the run: the existing `smith_prop24`, `smith_trigonal`,
`smith_canonical` and `nist_cover_quartic` unit tests pass on the widened
source (Smith's Example 1 positive control among them).

Not ported: `plane_quartic_pic` (u32 field, 171 sites). Hence `Psi` into
`Pic^0(C)` and the planted-log recovery on `C` are **not run** at 56 bits; that
is a separate port, recorded as the remaining gap.

## Run

`nist_cover_quartic_map wide --bits 56 --gamma-max 400 --seed 1592612438
--out runs_s56.json` (seed `0x5eed5656`). One deterministic instance.

1. `E`: `[k]G` for a planted `k` from the seed, computed by two independent
   implementations (`nist_cover_quartic::Ecc`, u128 affine; `prime_fast`,
   Montgomery Jacobian) must agree; `[n]G = O`.
2. `gamma = 1, 2, ...` up to 400: the first `gamma` for which
   `smith_canonical::find_trigonal` succeeds (squarefree `f`, Smith-positive,
   normal-form map with square leading coefficient of `s`). Every tried
   `gamma` and its outcome is logged.
3. `phi^*` in `J_H` (Cantor, u128): `phi^*(G) != 0`; `[n] phi^*(G) = 0` (with
   `n` prime, the order is exactly `n`); `phi^*([k]G) = [k] phi^*(G)`;
   `phi^*(G + Q) = phi^*(G) + phi^*(Q)`. Negative control:
   `[k+1] phi^*(G) != phi^*(Q)`.
4. The canonical quartic `C`: images under `psi` of the points of `X` over
   **randomly sampled** affine points of `H` (no enumeration). Fit on 40
   distinct image points (`fit_quartic_u64` requires: the three coordinates
   independent, no conic, no cubic, exactly one quartic). Verify on 120
   further, independently sampled image points: all must lie on `C`.
   Negative control: `C` with one coefficient perturbed must contain none of
   them.
5. Smoothness of `C` without point counting: after a random invertible
   coordinate change with nonzero `y^4`, `z^4` coefficients, in the charts
   `z = 1` and `y = 1`, interpolate `R_a = Res(F, F_u)` and `R_b = Res(F, F_v)`
   (Sylvester determinants at 30 points, formal degrees 4 and 3, degree
   `<= 12` by Bezout, checked); a singular point in the chart is a common root,
   so `gcd(R_a, R_b) = 1` with both nonzero certifies the chart; plus the point
   `(1:0:0)` directly. Controls (unit tests, at `p = 1009` and `p = 2^56 - 5`):
   the Fermat quartic certifies; a quartic with a node at `(0:0:1)` does not.

Pass: every check in 1-5 holds. A failure is recorded as such, not rerun with
other parameters.

## What it establishes, and does not

Establishes (if it passes): at the 56-bit NIST-shaped ladder curve, the novel
cover `H_gamma`, a Smith-positive `gamma`, the explicit trigonal map, the
checked pullback `phi^*` on the full prime-order group, and an explicit,
held-out-verified, certified-smooth same-field plane quartic `C`.

Does not establish: `Psi: J_H -> Pic^0(C)` or a transferred logarithm at 56
bits (not run); `#C = #H` (not countable at this size; the held-out test and
the canonical-map construction stand in for the fit, not for the zeta
function); anything about cost; anything at 256 bits.
