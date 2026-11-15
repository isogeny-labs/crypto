---
id: LEAD-ICP-gamma0-binary-forms
type: lead
title: Gamma_0(p) representation problem for binary quadratic forms
tags: [prime-field, ecdlp, semaev, binary-quadratic-forms, gamma0, m2]
confidence: measured
status: closed-obstructed
source_refs: []
added: 2026-11-04
superseded_by: null
next_action: none; census run 2026-11-05, ratio 0.22 -> 0.015 and structured = decompositions in every cell
---

## Statement
A 2-decomposition of R is a form `g(T) = S_3(x1,x2,T) in Z[T]` with leading coefficient
`(x1-x2)^2` a square, discriminant `16 f(x1) f(x2)`, and root `x_R mod p`. Decompositions are
therefore vectors of `L_R = {(c0,c1,c2) : c0 + c1 x_R + c2 x_R^2 = 0 mod p}` (det p) lying in a
structured set. Question: can the Gamma_0(p)-reduction / representation-number theory enumerate them
in sub-B time?

## Corrections from the audit
* `D = 16 f(x1) f(x2) > 0` for `x_i >= 1`, so the forms are **indefinite**: SL_2(Z)-orbits are
  infinite. The finite object is the set of Gamma_0(p)-reduced representatives of bounded size
  (equivalently, class-group cosets meeting the root constraint).
* The census as originally written is infeasible where it matters: the box `|c_i| <= B^4` in a
  3-dimensional lattice of determinant p contains about `8 B^12 / p` points -- O(1) at delta =
  1/12, p^2 at delta = 1/4. A census exists only for `delta <= 0.10`.

## Census result (2026-11-05)
At p = 2^20, 2^22, 2^24 with delta = 0.10, 0.12 the structured forms are *exactly* the decompositions
(2 = 2 in every cell), the box holds 8B^12/p points, and the ratio decomposition forms / square-leading
forms falls 0.22 -> 0.026 -> 0.015. The discriminant route costs B^8/p against B^2 direct:
closed-obstructed.

## Next action (executable)
Run the census at p in {2^20, 2^24} and delta <= 0.10, counting forms with square leading
coefficient, with D in S_B, and with both, against the number of decompositions; then test whether
sieving D by a small prime q beats its cost. Report the ratio and the work exponent.

## Kill
Ratio decreasing like `B^-c`, c >= 1, with no sub-B enumeration: file the Gamma_0(p) correspondence
as a known result and close.
