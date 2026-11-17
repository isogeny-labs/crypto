# Amendment S56b: the transfer `Psi` into `Pic^0(C)` at the 56-bit ladder prime

Amendment to `PROTOCOL.md` and `PROTOCOL_S56.md` of study
`nist_cover_quartic_map_20261116`, written and frozen before any 56-bit `Psi`
data. Owner request (2026-11-17): port the plane-quartic `Pic^0` arithmetic so
that the planted logarithm is carried onto `C` at 56 bits. Exact-correctness
construction; no timing or cost claim; no leaderboard, `benchmarks/README.md`
row or bench record. Every result is `[pcc:unverified]` until independently
replayed.

## Code

* `plane_quartic_pic64`: a 64-bit copy of `plane_quartic_pic` (`q < 2^62`).
  Field elements `u64`, products through `u128` (the `Fq` multiply and the
  `poly_roots` helpers), the two long accumulators reduced at every step,
  primality by deterministic Miller-Rabin. Algorithms unchanged. A separate
  module: the `u32` original, its callers and its frozen studies are untouched.
* `nist_cover_quartic::wide::Transfer64`: `Psi = psi_* . phi` ported from
  `smith_canonical::Transfer`, with no enumeration: the base point and the
  auxiliary bases are rational image points of `C` from the sampled canonical
  map; the pool of points of `H` is sampled; a split representative is
  accepted when its `e_class` is defined (not when it lies in a pre-enumerated
  list).

Regression gate before the run (unit tests):

1. `plane_quartic_pic64` against `plane_quartic_pic` on random smooth quartics
   at `q = 1009, 65521, 2^31 - 1`: identical canonical keys for point classes,
   sums, differences, negation and scalar multiples (`k` up to `~2^32`).
2. `Transfer64` against `smith_canonical::Transfer` at the `p = 1009` toy
   instance (same trigonal map, canonical map, base point): identical class of
   `Psi(phi^*(G))`, and `Psi([77] D) = [77] Psi(D)`.
3. The existing `nist_cover_quartic`, `smith_canonical` tests.

## Run

`nist_cover_quartic_map wide --bits 56 --gamma-max 400 --seed 1592612438 --psi
--out runs_s56b.json`: the S56 instance (same seed, so the same `E`, planted
`k`, `gamma = 15`, `C`), followed by:

1. `Transfer64` on `C` with base points from the rational held-out images; a
   pool of 48 sampled points of `H` over `U`.
2. `Psi(phi^*(G))` twice, from two independent random representatives: the
   two classes must be equal (well defined).
3. `Psi(phi^*(G)) != 0` and `[n] Psi(phi^*(G)) = 0` (`n` prime, so the order is
   exactly `n`: the order-`n` subgroup survives the transfer).
4. `Psi(phi^*(Q)) = [k] Psi(phi^*(G))` for the planted `k`: the planted
   logarithm carried onto `C`.
5. `Psi(phi^*(G + Q)) = Psi(phi^*(G)) + Psi(phi^*(Q))`.
6. Negative control: `[k+1] Psi(phi^*(G)) != Psi(phi^*(Q))`.

Recovering `k` from the classes on `C` by search is not attempted: at 56 bits
that is a generic search of `~2^28` `Pic^0` operations and says nothing beyond
check 4. Pass: every check in 2-6 holds. A failure is recorded as such, not
rerun with other parameters.

## What it establishes, and does not

Establishes (if it passes): at the 56-bit NIST-shaped ladder curve the full
chain `E(F_p) -> J_H(F_p) -> Pic^0(C)` carries the planted logarithm, with the
order-`n` subgroup preserved, on the explicit same-field quartic `C` of S56.

Does not establish: any cost or speed (index calculus on `C` is `O~(p)`, far
above rho on `E`); `#C = #H`; anything at 256 bits.
