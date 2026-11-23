# Protocol: a novel genus-3 cover of a NIST-shaped curve, its explicit same-field quartic, and the checked transfer map

Study `nist_cover_quartic_map_20261116`. Module
`src/cryptanalysis/nist_cover_quartic.rs`. Binary
`src/bin/nist_cover_quartic_map.rs`.

Standing: this is an **exact-correctness construction study**, not a speed or
novelty-of-complexity claim. Every statement produced by the run is
`[pcc:unverified]` until an independent validator replays it. Toy primes.
No leaderboard row, no `benchmarks/README.md` row, no bench record: there is
no timing or cost measurement here.

## Question

Given the existing Smith trigonal pipeline (`smith_trigonal`,
`smith_canonical`, `smith_transfer_cost_20261115`) that turns a Smith-positive
genus-3 *hyperelliptic* curve `H` into an explicit same-field plane quartic `C`
with a checked transfer `Psi: Pic^0(H) -> Pic^0(C)`:

Can we exhibit a **novel genus-3 hyperelliptic cover of a NIST-shaped elliptic
curve** `E` (short Weierstrass, `a = -3`, prime field, cofactor 1), run Smith's
trigonal method on it, and **check the full chain**

```
E(F_p)  --phi^*-->  J_H(F_p)  --Psi-->  Pic^0(C)
```

carrying a planted discrete logarithm from the elliptic curve all the way onto
the plane quartic, where the non-hyperelliptic index-calculus stage runs?

This connects the (cover / transfer) stage to the non-hyperelliptic
index-calculus stage. It is the milestone artifact, distinct from Tian's
degree-27 (3,3,3) route (a separate construction).

## The novel cover family

For `E: y^2 = x^3 + a x + b` over `F_p` and a free parameter `gamma != 0`, the
degree-2 map `phi(u,v) = ((u^2+gamma)/u, v/u^2)` pulls `E` back to

```
H_gamma: v^2 = f(u) = u * [ (u^2+gamma)^3 + a u^2 (u^2+gamma) + b u^3 ],
```

with ascending coefficients `f = [0, gamma^3, 0, 3 gamma^2 + a gamma, b,
3 gamma + a, 0, 1]` (monic degree 7 => genus-3, one point at infinity).

* At `gamma = 1, a = -3` this is `v^2 = u^7 + b u^4 + u`, the known degree-2
  genus-3 cover of the `a=-3` curve (`research/p256_genus_dlp_20261115`,
  genus-3 row). `gamma` free is the **novel one-parameter family**; it is the
  search space for a Smith-positive member of a *fixed* `E`.
* The construction is polynomial over the prime field, hence field-agnostic:
  it specializes to the actual NIST primes symbolically. The run here is at
  tooling scale (`3 < p < 2^31`, the bound of `smith_prop24`/`smith_canonical`);
  a bignum port for the full NIST primes is out of scope and is recorded as a
  gap, not done.

## Pullback homomorphism (exact)

`phi^*([P]-[O_E])` for `P=(x0,y0)`: roots of `u^2 - x0 u + gamma`, ordinate
`w = y0 u^2`. The identity `f(u)/u^4 = x0^3 + a x0 + b = y0^2` makes every
pulled-back point lie on `H`. In the one-infinity model
(`HJac::point(x,y)=[(x,y)-infty]`), with `phi^{-1}(O_E) = (0,0) + infty`:

```
phi^*([P]-[O_E]) = pair(P) - point(0,0),
```

`pair(P) = point(u1,w1)+point(u2,w2)` when `x0` splits, else the Mumford class
`U = u^2 - x0 u + gamma`, `V = y0 x0 u - y0 gamma`. `phi` has degree 2, the
transferred subgroup has odd prime order `r`, so `gcd(2,r)=1` and `phi^*` is
injective on it (`phi_* phi^* = [2]`).

## Instance

`Chain::build(p, a=-3, b, gamma_max, require_prime_e=true)`:

1. `#E` by brute force; require prime (cofactor 1).
2. For `gamma = 1..gamma_max`: build `f`, require `HJac` (deg 7); find a
   Smith-positive normal-form trigonal map (`smith_canonical::find_trigonal`);
   build `CanonicalMap`; require the quartic smooth and `#C(F_p)=#H(F_p)`;
   build `Transfer`; require `>= 8` usable points of `H` over `U`.
3. First such `gamma` is the instance.

`b` is the first `a=-3` nonsingular curve of prime order found by ascending
search at each `p`; `gamma` is the first admissible one. Primes: the fixed
grid `{1009, 2003, 4001, 10007}`, `gamma_max = 800`, planted `k = r/3 + 1`.
Larger primes are not run here: `Transfer::usable_points` enumerates the
~`p` affine points of `H` over `U`, so instance build is `O(p)`; the prior
Smith trigonal studies (`smith_canonical_20261115`, `smith_transfer_cost_20261115`)
stopped at `p = 211` and `p = 8209` respectively, and `p = 10007` already
exceeds both. A larger grid needs a faster usable-point pass, recorded as a
gap, not done.

## Checks (per instance)

Positive:

1. `#E` prime (cofactor 1); `H_gamma` genus 3, monic deg 7, squarefree; the
   quartic `C` smooth; `#C(F_p) = #H(F_p)`.
2. `phi^*` injective on `<P>`: `ord(phi^*(P)) = r` for a generator `P`.
3. `phi^*` homomorphism on the planted pair: `phi^*([k]P) = [k] phi^*(P)` in
   `J_H`, for a planted `k` and `Q=[k]P`.
4. Transfer order: `Psi(phi^*(P))` nonzero in `Pic^0(C)` and of order `r` on
   the odd part (Smith kernel `S` has exponent 2, coprime to `r`).
5. End-to-end: BSGS on `C` for `log_{Psi(phi^*(P))} Psi(phi^*(Q))` returns `k`.

Negative controls:

6. A wrong scalar is not accepted by the end-to-end check.
7. (module tests) the pulled-back points lie on `H` across the split fibre.

## Baseline / distance to rho (honest accounting)

The reference is the matched negation-map rho on `E` on the order-`r` subgroup
(`O~(sqrt r)` group operations). Index calculus on a same-field genus-3 `C` is
`O~(p) = O~(r^{1/1})` group operations (Diem), i.e. **above** rho for a
prime-field target. This study therefore reports the gap-to-rho as the distance
remaining and makes **no** improvement claim. The deliverable is the verified
explicit artifact and the checked map, which the Tian (3,3,3) route does not
supply on this family.

## What is and is not established

Established (by exact computation on each instance): the novel cover, its
explicit same-field quartic, the explicit pullback homomorphism, and the
end-to-end transfer of a planted logarithm `E -> J_H -> Pic^0(C)`.

Not established: any speed or complexity improvement; anything at the real NIST
prime sizes (bignum port not done); Smith's isogeny theorem itself (assumed, as
in `smith_canonical_20261115`); correctness of the reused
`plane_quartic_pic`/`smith_*` modules beyond their own studies.

## Deviations

Recorded in RESULTS.md after the run.
