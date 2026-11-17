# Radical (closed-form) low-degree root finding over F_p vs Cantor–Zassenhaus

**Date.** 2026-11-17. **Runtime.** Claude (local Rust toolchain). **Status.**
Prototype, standalone (no crate dependency), public synthetic only. The
quadratic solver is exhaustively validated and measured; the quartic is
analyzed with a predicted win but **not yet implemented** (the cube-root
primitive it needs is the open piece). `[pcc:unverified]` — field-op counts are
exact (deterministic), not wall-time; a wall-time claim would go through the
`controlled-experiment` protocol.

## Idea

`prime_root_trace` finds roots with generic Cantor–Zassenhaus (Shoup 20.4),
whose dominant cost is `X^p mod f` ≈ `Θ(d² log p)` **polynomial-coefficient**
field multiplications (+ equal-degree splitting). The univariate polynomials in
prime-field Semaev/index-calculus root finding have degree `2^{m-1}` in the
eliminated variable — **d = 2 (S₃, m=2), 4 (S₄, m=3), 8 (S₅, m=4)** — and
d ≤ 4 is **solvable by radicals**: roots via modular square/cube roots, each a
single scalar modular exponentiation (`Θ(log p)` scalar muls). So for d ∈ {2,4}
the work drops from `Θ(d² log p)` polynomial-coefficient muls to `Θ(log p)`
scalar muls.

## Quadratic (d = 2, the S₃ / m=2 case) — validated + measured

`prototype/radical_vs_cz_quadratic.rs`. Radical: `disc = b²−4ac`; Legendre to
decide 0 / 1 / 2 roots; Tonelli–Shanks sqrt (fast path `a^((p+1)/4)` for
p≡3 mod 4); roots `(−b±√disc)/(2a)`.

- **Correctness:** exhaustively validated against brute force on **374 566
  (a,b,c) triples over 13 small primes** (both p≡1 and p≡3 mod 4), and agreeing
  with the reference Cantor–Zassenhaus on **2000 random polynomials per prime**
  across 6 primes up to 2⁶¹−1, on both the split (two-root) and no-root paths.
- **Field multiplications, radical vs CZ** (`prototype/quadratic_measurement.txt`):

| p | p%4 | case | radical muls | CZ muls | ratio |
|--:|--:|:--|--:|--:|--:|
| 1 000 003 | 3 | split | 114 | 3 358 | **29.5×** |
| 1 000 003 | 3 | no-root | 59 | 1 185 | **20.1×** |
| 1 000 033 | 1 | split | 283 | 3 743 | 13.2× |
| 1 000 000 007 | 3 | split | 181 | 7 802 | **43.1×** |
| 2 147 483 647 | 3 | split | 218 | 13 261 | **60.8×** |
| 2 305 843 009 213 693 951 (2⁶¹−1) | 3 | split | 428 | 48 440 | **113.2×** |
| 2⁶¹−1 | 3 | no-root | 245 | 16 331 | **66.7×** |

**The radical quadratic uses 13–113× fewer field multiplications than CZ, and
the margin grows with p** (CZ pays `Θ(d² log p)` coefficient muls; radical pays
`Θ(log p)` scalar muls, so the `d²` constant compounds against CZ as log p
grows). p≡1 mod 4 (full Tonelli–Shanks) costs more than p≡3 mod 4 (single
exponentiation) but still wins 13×+. The **no-root / rejection path** — the
common case in an IC relation sweep — is 20–67×, because one Legendre test
rejects a non-residue discriminant while CZ still computes a full `X^p mod f`.

## Quartic (d = 4, the S₄ / m=3 case) — analyzed, not yet implemented

CZ's cost scales as `d²`, so for d = 4 the coefficient-mul count is ~4× the
quadratic's gap over its own radical: the predicted win is the **largest** of
the practical degrees (order ~50–400× on the split path at 32–61 bits by the
same `d² log p` vs `log p` accounting). Ferrari's method solves the quartic by
radicals: depress, solve a **resolvent cubic**, then two quadratics (the
quadratic solver above). The open piece is a correct, fast **modular cube
root** — bijective `a^((2p−1)/3)` for p≡2 mod 3, but AMM
(Adleman–Manders–Miller) for p≡1 mod 3, and **cube roots over F_{p²}** for the
casus irreducibilis (three F_p roots whose Cardano radicals live in F_{p²}).
`prototype/cbrt_amm_validation.rs` validates an F_p cube root across both
classes (8 270 (a,p) pairs) but currently leans on a brute fallback, so it is
not yet measurement-grade. Implementing a clean F_{p²} AMM cube root (validated
exhaustively on small fields, isolated from the Cardano/Ferrari logic) is the
next step; d = 8 (S₅) is not solvable by radicals and keeps CZ.

## Scope / honesty

- Field-op counts are exact; the comparison is apples-to-apples (same counted
  `mul`/`sqr` primitive on both solvers). Not a wall-time claim.
- Solving low-degree polynomials by formula instead of generic factorization is
  a **known** computer-algebra technique; the contribution here is the measured
  magnitude in the prime-field IC degree regime and (pending) wiring it into the
  Semaev-polynomial / certificate root-finding path. It is **off the `prime_fast`
  subtraction hot path** (which does no univariate root finding); it speeds the
  Semaev-polynomial IC variant and the `prime_root_trace` certificate path.
- KB novelty check (`research-kb`) and the `controlled-experiment` protocol
  apply before any in-repo comparative/novelty claim; recorded here as prototype
  evidence, not yet a landed claim.

## Reproduce

```bash
rustc -O -o radtest   prototype/radical_vs_cz_quadratic.rs && ./radtest
rustc -O -o cbrttest  prototype/cbrt_amm_validation.rs     && ./cbrttest
```
