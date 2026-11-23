# Algebraic relation oracles against the rho-parity budget (k = 20, 2026-11-04)

Follow-up to [README.md](README.md). The generic floor rules out parity for any factor base with
unknown logarithms [pcc:unverified]. The only escape left is an oracle that finds relations
algebraically, with its native work priced at w GAE per operation [pcc:unverified]. This screen
asks how cheap such an oracle would have to be, and how far every known family is from that.
It uses no ecbench run; `scripts/screen.py` computes everything, and `data/screen.json` holds it.

## Budget

A single-target solve needs F + 1 relations. With coverage C out of r points, it needs
(F + 1)·r/C trials. Each trial costs 1 group operation plus c native operations. The model is:

S(w) = (F + 1)·(r/C)·(1 + w·c) / √r

Base construction and setup are ignored, which favours index calculus. Parity needs S(w) at or
below the measured rho-neg S at every weight: 1.600, 1.716 and 2.764 at w = 0, 0.1 and 1 (T13
band). At k = 20 the whole w = 1 budget is about 1,000 GAE [pcc:unverified].

## Screen

Gap = S(model) ÷ S(rho-neg). "m" is the arm's measured or derived source.

| family | F | C | trials | c per trial | gap w = 0 | gap w = 0.1 | gap w = 1 | source |
|:--|--:|--:|--:|--:|--:|--:|--:|:--|
| isogeny t = 4, G = 3, seed 1193 + trace (T13) | 8 | 52 | 22,708 | 8,339 | 39 | 3.1·10⁴ | 1.9·10⁵ | m: L10, T13 |
| isogeny t = 5, G = 4, seed 1307 + trace (L12) | 22 | 210 | 14,370 | 24,912 | 25 | 5.8·10⁴ | 3.6·10⁵ | m: L12 |
| same base, exhaustive grid (L13) | 22 | 210 | 14,370 | 1,248 | 25 | 2.9·10³ | 1.8·10⁴ | m: L13 |
| digit set t = 5, G = 4 (L15) | 22 | 522 | 5,781 | 1.4·10⁶ | 10 | 1.3·10⁶ | 7.9·10⁶ | m: L15 |
| isogeny t = 4, m = 3 (T3) | 8 | ≤ 960 | 1,230 | 4.0·10⁷ | 2.1 | 8.0·10⁶ | 5.0·10⁷ | m: T3 cost; C derived |
| m = 2, full coverage, F = 256, one root-find per base point | 256 | r | 257 | 6,400 | 0.44 | **265** | **1.6·10³** | derived |
| m = 2, full coverage, F = 256, exhaustive pair scan | 256 | r | 257 | 2.0·10⁵ | 0.44 | 8.1·10³ | 5.1·10⁴ | derived |
| m = 3, full coverage, F = 46, F4 on S₄ | 46 | r | 47 | ≥ 4.0·10⁷ | 0.08 | 3.1·10⁵ | 1.9·10⁶ | derived; T3 cost at F = 8 as lower bound |
| multiplicative subgroup x ∈ μ_d, d = 2,394 (L11), scan | 1,197 | r | 1,198 | 23,940 | 2.1 | 4.6·10³ | 2.9·10⁴ | derived |

All table values are model outputs from measured or derived inputs [pcc:unverified].

## Reading

- Every measured family is limited by coverage before native cost counts. Its trials alone exceed
  rho's whole budget, so it fails even at w = 0 [pcc:unverified].
- The full-coverage designs go below rho at w = 0 (0.44× and 0.08×). That is only because native
  work is free at w = 0; the band rule (AGENTS.md §5a) does not rank on one weight
  [pcc:unverified].
- The smallest gap is the m = 2 full-coverage design with one root-find per base point: 265× at
  w = 0.1 and 1.6·10³× at w = 1. Parity would need c ≤ 14 native operations per trial at w = 0.1
  and c ≤ 3 at w = 1. That means deciding whether R is a signed pair-sum over 256 base points in
  about three field operations; no known structure of these curves does this [pcc:unverified].
- The gap grows with the curve. At full coverage F ≈ √(r/2) and c ≈ 25·F, so
  S(w) ≈ 12.5·w·√r: exponent 1 in r against rho's 1/2 [pcc:unverified].
- No family is within 10× at w = 0.1, so this screen proposes no single-target test. The strongest
  obstruction: the w = 1 budget of about 1,000 GAE leaves an oracle at most 1,000/(F + 1) native
  operations per relation at full coverage, and the cheapest known decision over a base of the
  needed size costs about 6,400 per trial [pcc:unverified].

## Limits

The model ignores setup and linear algebra, which only lowers index calculus's cost. Coverage
for m = 3 and μ_d is an upper bound. The root-find cost of about 25 multiplications per base point
is a derived estimate, not a measurement. The literature has no prime-field method that changes
these numbers (Semaev 2004; Petit, Kosters and Messeng, PKC 2016).
