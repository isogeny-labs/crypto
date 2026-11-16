# Full s=7 GHS solve on the negative-trace twist

16 November 2026 UTC. This result solves one saved 48-bit source subgroup target through a degree-seven cover and genus-five Jacobian relation pipeline. A completed, independently audited, same-target Frobenius-aware rho run measures a 14.14-fold CPU advantage for this GHS implementation on this instance. This is an observed one-solve result, not a guarantee across targets or implementations. Finite claims remain [pcc:unverified] until PCC admission.

## Exact target and transfer

Set K = F3[a]/(a^5+2a+1), L = F_(3^35), k = F_(3^7), and b = a+a^3. The source and carrier are

```text
E^-/L: v^2 = u^3 + u^2 + 2b
Y^-/k: y^2 = 2(x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1).
```

The exact L modulus, embedding of a, source point P, and Mumford transfer image are in the independently audited [transfer receipt](../ghs_twist_extension_20261116/twist_s7_transfer.json). The cover is u=2A/B, v=yC/B^2 with the complete A,B,C coefficient vectors in the [positive-orbit cover receipt](../ghs_odd5_carrier_20261115/degree7_quadratic_square_30.json). The polynomial identity and pole-fiber correction are checked by the producer and an independent auditor.

The source order is 50,031,544,838,921,370 = 270 * r, where r = 185,302,017,921,931 is prime. The saved P has order r, Q=12345P is the validation target, and the corrected pullback plus five-term L/k trace maps P to nonzero G of order r and Q to H=12345G. The [full attack receipt](s7_full_attack.json) saves P, Q, G, H, the exact relation and target records, and phase timings.

## Relation solve and target descent

The factor base has 1,092 projected classes from 2,187 finite rational points of Y^-(k). A bounded walk of 250,000 multiples of G yielded 2,069 split relations and covered every factor-base column. The modular matrix over F_r has rank 1,092. FLINT solved all 1,092 factor-base logs, each verified by a Jacobian scalar equality. Target descent found a split row at offset 124, recovered 12,345, and checked it on both E^- and J(Y^-).

The independent [audit receipt](audit_s7_full_attack.json) rebuilt the cover from saved coefficient vectors, reevaluated P and Q, checked all 1,092 individual logs, checked all 2,069 relation rows as Jacobian group equalities, recomputed rank and target descent, and verified the scalar. This replay used its own code path and took 112.11 CPU seconds; audit time is outside the attack total.

| GHS phase | CPU seconds |
|---|---:|
| Source and curve setup | 0.107019 |
| Pullback, trace, and target transfer | 0.168780 |
| Factor-base setup | 6.361242 |
| Relation collection | 222.494982 |
| Matrix construction, rank, and solve | 1.863915 |
| Target descent and final verification | 0.129193 |
| **Stitched one-solve total** | **231.125131** |

The relation stage was checkpointed across two processes. The total adds its cumulative in-process CPU to one measured setup, transfer, FLINT linear-algebra run, and target descent. It excludes Sage interpreter startup, validation-target generation (0.061730 s), and the separate 55.232059 s producer audit of all factor-base logs. A first generic sparse-matrix rank attempt took 191.298066 CPU seconds to reach the same rank and was interrupted during the subsequent solve. It is an incomplete implementation arm and is not pooled into the FLINT total.

## Bound on new s=7 isogeny models

Let pi be the 243-Frobenius of E^- over K. Its characteristic equation is pi^2+26*pi+243=0. Define U_0=0, U_1=1, and U_(n+1)=-26*U_n-243*U_(n-1). Induction gives pi^n=U_n*pi-243*U_(n-1). Here U_7=-21,156,227, and 21,156,227 is prime. The recurrence and all trial divisors through 4,600, above its square root, are checked in [TwistOrbitArithmetic.lean](TwistOrbitArithmetic.lean); [Sage's independent receipt](isogeny_orbit_bound.json) checks primality and enumerates small-degree isogenies.

Suppose H is a finite subgroup of E^- of order n coprime to 3 and 21,156,227. If H is stable under pi^7, then it is already stable under pi: solve the displayed identity for pi modulo n, since U_7 is invertible modulo n. Consequently, every separable L-isogeny with such a kernel descends to K and its codomain j-invariant stays in K. A single separable isogeny of degree prime to 3 that makes j have full degree 35 must therefore have degree divisible by 21,156,227. This is a bound for this fixed source and sevenfold base change, not a universal GHS bound. Direct tests of degrees 2, 3, 5, 7, 13, 29, 41 and 43 found no codomain j of degree 35; the degree-three test is an observed check outside the prime-to-three proof.

## Matched rho and claim status

The [source-group rho receipt](rho_s7_result.json) uses the same saved P,Q and the checked order-seven F_(3^5)-Frobenius eigenvalue 173,253,049,031,852 modulo r. It canonicalizes each Frobenius orbit and uses distinguished points to bound memory. The walk found a verified collision at step 7,761,729, recovered scalar 12,345, and used 0.294091 s setup plus 3,267.740265 s walk CPU, for **3,268.034356 s total**. The [independent rho audit](audit_rho_s7.json) rebuilt E, P, Q, checked the eigenvalue, both collision representations, the scalar and the CPU sum. The first 10,000-step rho calibration was censored at its stated cap and is separate from this complete run.

| Complete same-target solve | CPU seconds | Relative cost |
|---|---:|---:|
| GHS cover, relation solve, target descent | 231.125131 | 1.00 |
| Order-seven Frobenius rho | 3,268.034356 | 14.14 |

Thus the measured GHS solve is **14.14 times faster** than this rho run on the exact same 48-bit subgroup target. The GHS total stitches checkpointed relation collection to separately timed setup, linear algebra and descent. Rho is one checkpointed run. Both exclude Sage interpreter startup. Rho collision time has substantial single-run variance; the source implementation uses the order-seven Frobenius quotient but does not claim to exhaust every possible rho optimization. Optimized implementations on either side could change the ratio. This finite result supports a potentially vulnerable **exact curve instance** under the stated cost model; it does not establish a new eligible family or a new attack method.

The [2015 Iijima–Momose–Chao classification](https://eprint.iacr.org/2015/805.pdf), Section 5 Case 13, concerns a direct degree-five genus-five (2,...,2) construction with full rational 2-torsion. This exact source has order 2 modulo 4 and a degree-seven cover, outside that specified direct case. [Joux and Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) already describe cover plus decomposition index calculus, including covers found after an isogeny walk. Our cover arises by an ordinary quadratic twist of the positive-trace cover, so the mechanism and twist closure are prior art. The exact negative-trace cover and subgroup solve are new explicit records in this project; global publication priority remains unestablished. An independent new eligible family or new attack method is not claimed.
