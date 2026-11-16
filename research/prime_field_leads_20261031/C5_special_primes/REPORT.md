# C5: Special prime shapes, closure note (write-up)

**Lead.** C5 of `research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md`. **Decision.** writeup.
**Purpose.** Record why 2^255−19 and the Solinas primes of the NIST curves touch field arithmetic only,
so the idea is not reopened. Derivation only; no counted measurement in this directory.

## Claims

1. **Doubling does not preserve "x has small high bits".** For y² = x³ + ax + b,
   x(2P) = ((x² − a)² − 8bx) / (4(x³ + ax + b)), a degree-4 rational map. The set B_k of x with top k bits zero
   has density 2^{−k}, and the preimage of B_k under this map is not B_k. Nothing in the map keeps x(2P) in B_k.
2. **Multiplication by 2 on x is not a curve map.** x ↦ 2x is a field operation that ignores the curve equation.
   A value x is the x-coordinate of a point iff x³ + ax + b is a square, which holds for about half of F_p.
   So 2x leaves x(E) for many x.
3. **No digit-pattern base is closed under the group law.** In a prime-order group of order n, a nonempty subset closed
   under addition contains (n−1)P = −P for every element, hence the identity, and is a subgroup; a prime-order
   group has no proper nontrivial subgroup. So an addition-closed base is {O} or the whole group. Negation
   preserves x, so an x-defined base is negation-closed; that gives the known factor 2 and no more.

## Exceptions that are not reopened here

- **Negation.** Preserves x; the factor 2 in the table above.
- **Rational 2-torsion translation.** Acts as a Möbius map on the x-line (FGHR effect; `RESEARCH_EXOTIC_COORDINATES.md`
  §1.3). This is the symmetry side and belongs to the card `LEAD-ICP-xline-symmetry-special-j`, whose disposition is
  `open_unexamined`. This note does not close that question.
- **x ↦ ζx (j = 0) and x ↦ −x (j = 1728).** These are curve automorphisms. P-256 has a, b ≠ 0, so j ∉ {0, 1728}.
  ECCp-131 has a, b ≠ 0 (from `src/ecc/curve.rs`, `eccp131()`), so the same holds. Curve25519 in Montgomery form has
  A = 486662, and its j-invariant over Q, 256(A²−3)³/(A²−4), is about 1.4·10²⁵, so it is neither 0 nor 1728 over Q.
  The reduction mod p was not separately checked here.

## Cited sources

- `research/notes/cm-isogeny/RESEARCH_NIST_CYCLOTOMIC_POINT_HEIGHTS.md`, §3. Only the corrected statements are used:
  small naive height is necessary but not sufficient for a small-height lift, and the canonical height is invariant
  under Q-isomorphism (correction of 2026-10-11).
- `research/notes/cm-isogeny/RESEARCH_NIST_SOLINAS_STRUCTURE.md` for the Solinas shape context.
- `/Volumes/SSD990-2/crypto-autoresearcher/knowledge/leads/ecdlp/ic-prime/LEAD-ICP-xline-symmetry-special-j.yaml`
  for the symmetry side, which is that card's domain.

## Diagram

`closure_diagram.svg` shows the three failed closure properties. Data: `closure_checks.csv`, which records each
claim, its status and whether it was measured (none were).

## Limits

- No counted measurement; the heuristic densities (2^{−k}, about one half) are stated as densities, not measured.
- The j-invariant checks for Curve25519 are over Q only.
- Solinas reduction changes the cost of one field multiplication, not the number of them. That is the counted unit
  and is not a closure property, so it is not counted here.

## Reading

The three closure properties fail for the same reason: the group law and the bit-level structure of a field element
are different structures. Digit patterns survive only the field operations they are written for.
