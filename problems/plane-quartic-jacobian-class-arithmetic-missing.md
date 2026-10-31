# No class arithmetic on the Jacobian of a plane quartic

- **Status:** `resolved` (toy smokes)
- **Kind:** implementation gap
- **Affected work:** `research/ghs_cubic_genus3_build_20261012/` (Conductor T-2267); follow-up IC study `research/ghs_cubic_quartic_ic_20261012/` (T-2280 / T-2491, PR #1768)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-21

## Problem

We could not test numerically that the transfer `E0(F_{q³}) → Jac(C)(F_q)` is a homomorphism injective on the order-`ℓ` subgroup, nor run a discrete logarithm in `Jac(C)(F_q)`, because the repository had no arithmetic on divisor classes of a non-hyperelliptic genus-3 curve (a smooth plane quartic).

The instances were the nine `F_q` plane quartics of run `20261013T0320Z` (`q ∈ {5, 7, 11}`, `#E0(K) = 4ℓ`). The study verified the descent, smoothness, the L-polynomial `1 − t_E T³ + q³T⁶` and that every transfer divisor is an `F_q`-rational effective divisor of degree 6 on the quartic; the homomorphism and kernel statements rested on a derivation only. `#Jac(C)(F_q) = #E0(K)` held on 9 of 9 cases from point counts over `F_q`, `F_{q²}`, `F_{q³}`.

## What unblocked it (2026-10-14)

`research/ghs_cubic_quartic_ic_20261012/quartic_ic.rs` now provides:

1. **Pic / linear-equivalence checks** on `F_q`-rational transfer divisors (typically two Gal(`K/F_q`) orbits): `--check-lin-equiv` runs `pic_jacobian` (homomorphism and associativity via deg-6 divisor keys, `#Jac` / `ℓ` annihilators, cubic residual reduction smoke). Passes on `q ∈ {7,11}`.
2. **Pure IC target path** keyed by those Gal-orbit deg-6 divisors: planted-scalar recovery with `recovery:ic` and **no** `transfer_pushforward_dlog` fallback on `q=7` and `q=11` smokes.

Evidence: `research/ghs_cubic_quartic_ic_20261012/results/run-20261014T0200Z.jsonl` (smoke) and `run-20261014T0241Z.jsonl` (10 targets per variant per field, `q ∈ {5, 7, 11}`, 90/90 correct), README tables.

## Different route: the hyperelliptic `m = 3, r = 5` type (2026-10-13, T-2463)

`research/ghs_cubic_hyperelliptic_genus3_20261013/` builds the other genus-3 type of the cover census, which is hyperelliptic, as an explicit `w² = h(z)` over `F_q` (degree 7 when `{a, b} ⊂ F_q`), and executes the transfer `φ = (1 + Fr + Fr²)∘π0^*` with Cantor arithmetic (run `20261014T0029Z`, `q = 5, 7, 11, 13`, 12 instances): homomorphism on 400/400 random pairs per instance, `φ(G) ≠ 0` and `ℓ·φ(G) = 0` for a generator of `E0(K)[ℓ]`, `π0_*∘φ = [4]` on every point of `E0(K)`, and `ker φ` of order 2 or 4 (so `φ` is injective on the `ℓ`-part but not an isomorphism). That route executes a genus-3 transfer for its type only; it never supplied class arithmetic on plane quartics, which is what the section above closes for the `m = 2` covers.

## Remaining limitations

- Canonical class-unique keys now exist (2026-10-18, W1, `src/cryptanalysis/plane_quartic_pic.rs`, study `research/quartic_pic_canonical_20261018/`): `Pic^0` of a smooth plane quartic over `F_q`, `q < 2^31`, with add, neg, double, scalar multiplication and a key that is a function of the class (echelon conics of the unique effective degree-3 divisor, or the point `P` of `K - P`). Validated by brute-force class counts against `#Jac` at `q` in {5, 7, 11, 13} on 21 instances, group-law tests, and on the three split GHS transfers at `q` in {5, 7, 11}, where the old degree-3 reduction merged 1, 7 and 20 pairs of distinct classes and the canonical keys separate all of them. Generic add costs 3523 multiplications + 44 inversions for `q` from 2^12 to 2^31 (not compared with any external implementation).
- Still open: the other six GHS quartics are not covered by the transfer-agreement test (only the first split instance per `q` is reachable from `quartic_ic.rs` without modifying it), and `quartic_ic.rs` itself still keys its dlog on deg-6 orbit pairs (W2b is to switch to the canonical keys).
- Dense FB-only line IC does not by itself reach the `ℓ`-part on these toys (`C(F_q)` is often 4-primary under `λ`).
- No deployed-curve or sub-ρ claims.

## Resolution and history

2026-10-12: opened with PR for T-2267. The index row in `problems/README.md` is not added in that PR because the file is held by Conductor T-2240; add it when that reservation clears.
2026-10-13: partial progress — pushforward identity / `transfer_pushforward_dlog` only (not full Pic; not pure IC).
2026-10-13: different route executed for the hyperelliptic type only (T-2463, PR #1796).
2026-10-14: Pic lin-equiv checks + orbit-aware pure IC on toys (T-2491 / PR #1768). Status → `resolved` for the discriminating toy checks named in "What would unblock it."
2026-10-18: W1 canonical `Pic^0` keys and arithmetic (PR for branch `claude/quartic-pic-canonical-20261018`).
2026-10-19 (N4, `research/quartic_n4_20261019/`): the 2026-10-14 "pure IC" smokes were subgroup enumeration (99.8-99.99% of their cost was the loop over every multiple of the generator; relation rows discarded; 0, 1 and 3 relations at q = 5, 7, 11), so they are withdrawn as index-calculus evidence; their recovered scalars and deg-6 table are correct, their degree-3 reduction merged distinct classes (1, 7, 20 pairs). A real relation-collection pipeline on canonical `Pic^0` (no enumeration) now exists (q up to 262111, 51 of 51 instances verified) and measures an exponent of 1.083 [1.040, 1.128] against matched rho's 1.485 on q >= 4051 for the GHS-weak family; lead N4 stays open. The "FB points are 4-primary" note above is not supported at q = 5, 7, 11 (all points but `P0` have a nonzero `ell`-part).
2026-10-21 (N4 iteration 2, `research/quartic_n4_la_20261021/`): double large primes (B2) cut the cold cost of the plane-quartic relation-collection solve to 0.22 to 0.36 of the recorded B0 baseline at q >= 4051 and give an exponent of 1.045 [1.037, 1.052] against rho's 1.475 on that range (IC/rho 0.024 at q = 1 048 507, GHS-weak family only); structured elimination (B1) was a small gain and a regression at q = 262 111. B2 is the new baseline; lead N4 stays open and is now ahead of rho in the whole measured range q >= 991.
