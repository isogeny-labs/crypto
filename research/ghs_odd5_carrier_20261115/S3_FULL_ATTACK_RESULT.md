# Complete s=3 GHS transfer and matched rho on one trace-+26 curve

16 November 2026 UTC. This result is a measured small-field calibration for the exact mask-30 source, not a practical-speedup claim. The attack and baseline sources were frozen before their final receipts, and earlier failed or imprecise runs remain in `S3_FULL_ATTACK_PROTOCOL.md` and `s3_full_attack_attempt*.json`. All finite claims remain [pcc:unverified] until PCC admission.

## Exact instance

Let `K=F3[a]/(a^5+2a+1)`, `L=F_(3^15)` with the exact field polynomial in `s3_full_attack.json`, and `k=F_(3^3)`. The elliptic source is `E_30/L: v²=u³+2u²+a+a³`; the carrier is the fixed `Y/F3` in `DEGREE7_FINDING.md`. The order is `#E_30(L)=#J(Y)(k)=14350286=2*109*65827`, where `r=65827` is prime. The saved P has order r and the public validation target is `Q=12345P`. Full coordinate vectors, exact field moduli and embeddings are in the final receipt.

The degree-seven cover's corrected pullback and `L/k` trace gave nonzero `G=T(P)` and `H=T(Q)` of order r. A separately evaluated `37P` checked scalar preservation. The independent auditor reconstructed the cover from its saved F243 coefficient vectors, recomputed G and H from P and Q, and matched both saved Mumford representatives.

## Full measured solve

The factor base used 12 nonzero projected classes from 27 finite rational points. Shifted relation walks attempted 3,406 candidate divisors and accepted 52 whose reduced Mumford U split over k. The modular relation matrix had full rank 12. Every solved factor-base logarithm was checked by a Jacobian scalar equality. Target descent succeeded after 138 offsets and recovered scalar 12,345, verified on both E and J. `audit_s3_full_attack.py` independently checked the recorded 52 modular equations, rank 12, target recovery, all factor-base group equalities, and the 30 rho candidate scalars.

| Measured CPU phase | Seconds |
|---|---:|
| GHS setup and factor base | 0.235315 |
| Pullback and trace transfer | 0.081890 |
| Relations and modular linear algebra | 1.288679 |
| Target descent and final verification | 0.046729 |
| **GHS total, including its setup** | **1.652613** |
| Standalone matched rho setup | 0.105799 |
| Median rho trial, 30 verified solves | 0.038295 |
| **Median one-solve rho total, including its setup** | **0.144094** |

The measured ratio is `1.652613/0.144094 = 11.469`: GHS was about 11.5 times slower on this exact r-subgroup. The matched rho implementation canonicalizes the orbit under the order-three `F_(3^5)` Frobenius, whose checked eigenvalue is 2447 modulo r. Its median collision took 148 walk steps; every trial checked the recovered scalar on the same saved P and Q. Both in-process CPU totals exclude Sage interpreter startup. The GHS total charges its field, curve, factor-base and transfer setup; the standalone rho total charges its field, source curve, eigenvalue and walk setup. The wall-time outputs include interpreter startup and should not be pooled with these CPU phase measurements.

This executable pipeline at `s=3` demonstrates target recovery but does not measure the 69-bit `s=11` subgroup. Diem's fixed-genus expected `O-tilde(q^(8/5))` theorem is an asymptotic result under its cyclicity or known-group-structure hypothesis; it does not supply an observed `s=11` attack cost. The five `s=11` exact transfer candidates and the full-degree-55 primitive-j source remain **map-verified, cost-unresolved**. No new attack method or practical vulnerability is established by this slower `s=3` run.
