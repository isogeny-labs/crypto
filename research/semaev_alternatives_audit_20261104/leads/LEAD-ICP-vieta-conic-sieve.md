---
id: LEAD-ICP-vieta-conic-sieve
type: lead
title: Vieta-conic + integer-square sieve for 2-decomposition
tags: [prime-field, ecdlp, semaev, vieta, conic, closed, m2]
confidence: measured
status: closed-obstructed
source_refs: []
added: 2026-11-04
superseded_by: null
next_action: none; cite the O(B) baseline in any future m = 2 comparison
---

## Statement (as proposed)
2-decompositions of R are points (e1,e2) on the conic `Q_R(e1,e2) = 0` with
`e1^2 - 4 e2 = u^2`, `u = x1 - x2`. Sieve the square condition first, then batch the resulting
univariate small-root problems with a lattice over (e1, u).

## Why it is closed (strict standard: Lean `m2_never`)
0. **Arity 2 cannot win at all** (`lean/Routes.lean`, `m2_never`): no m = 2 oracle beats rho, so the
   route is excluded independently of the points below, which are measurements.
1. **It is already O(B), with no lattice (measured).** `Q_R` is quadratic in e2, so each `e1 in [1,2B]` yields
   at most two conic points; enumeration is `O(B)` field operations and the square test is O(1) per
   point. Measured: planted 2-decompositions recovered at `B = p^(1/2)` in `O(B)` (5/5 at p = 2^28,
   8.4e4 e1-steps, 9.4 s; `scripts/vieta_conic_enum.py`). This is the generic sqrt(p) bound.
2. **m = 2 cannot change the exponent.** With `C = p T / B + B^2`, even a free oracle gives
   `p / B + B^2 >= p^(2/3)` at `B = p^(1/3)`; arity 2 needs `B = p^(1/2)` for relations to exist.
3. **The proposed reduction is quartic, not quadratic.** Substituting `e2 = (e1^2 - u^2)/4` into
   `Q_R` gives a quartic in e1, so univariate Coppersmith reaches `e1 <= p^(1/4)`, not `p^(1/2)`.

## Retained result
Lemma: "the symmetric quotient is a conic, and the genus-1 obstruction is exactly the integer
square condition." True and worth citing; not a lead.
