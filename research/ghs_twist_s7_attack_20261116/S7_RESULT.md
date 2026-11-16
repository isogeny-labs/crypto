# Full s=7 GHS solve on the negative-trace twist

16 November 2026 UTC. This result solves one saved 48-bit source subgroup target through a degree-seven cover and genus-five Jacobian relation pipeline. The same-target Frobenius-aware rho run is still in progress. Therefore the present record does not establish an observed speedup or a practical vulnerability. Finite claims remain [pcc:unverified] until PCC admission.

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

## Matched rho and claim status

The source-group rho script uses the same saved P,Q and the checked order-seven F_(3^5)-Frobenius eigenvalue 173,253,049,031,852 modulo r. It canonicalizes each Frobenius orbit and uses distinguished points to bound memory. Its first 10,000-step run was censored at its stated cap; the optimized run measured 0.145212 s setup and 4.555784 s walk CPU. The complete one-solve rho cost is pending; that 10,000-step timing must not be promoted to a solve or speedup.

The [2015 Iijima–Momose–Chao classification](https://eprint.iacr.org/2015/805.pdf), Section 5 Case 13, concerns a direct degree-five genus-five (2,...,2) construction with full rational 2-torsion. This exact source has order 2 modulo 4 and a degree-seven cover, outside that specified direct case. Its cover arises by an ordinary quadratic twist of the positive-trace cover, so the mechanism and twist closure are prior art. The exact negative-trace cover and subgroup solve are new explicit records in this project; global publication priority remains unestablished. An independent new eligible family or new attack method is not claimed.
