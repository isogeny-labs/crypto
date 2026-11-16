# Degree-seven cover transfer: complete s=7 legacy run

## Exact instance and map

Let `K=F3[a]/(a^5+2a+1)`, `L=F_(3^35)`, and `k=F_(3^7)` with the embeddings pinned in `s7_transfer.json`. The source is

`E/L: v^2=u^3+2u^2+a+a^3`,

and the carrier is

`Y/k: y^2=x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1`.

The degree-seven cover has `u=A(x)/B(x)` and `v=C(x)y/B(x)^2`; its exact polynomial identity and coefficients are checked in `s7_transfer.sage` and pinned by `degree7_quadratic_square_30.json`. The map first pulls a point to `J(Y)(L)`, subtracts the fixed two-torsion pole divisor, then sums the five `3^7`-Frobenius conjugates and descends the fixed image to `J(Y)(k)`. `audit_s7_transfer.sage` independently reconstructed the saved images. [pcc:unverified]

`#E(L)=50031545359078046=2*29*109*7393*1070455751`. For the prime `r=1070455751`, the saved `P` has order `r`; the image `G` is nonzero of order `r`, and the saved target `Q=123456789 P` maps to `H=123456789 G`. The same map preserves `37P`; the receipt contains exact coordinates for all points and images. [pcc:unverified]

## Complete historical Sage execution and cost

The third frozen attack attempt obtained rank 1093 on a factor base of 1093, using 2186 accepted relations in 269998 attempts. It completed target decomposition and recovered `123456789` in both the Jacobian and the source subgroup. `audit_s7_attack.py` checked all saved modular relation equations and the descent row. The first attempt was interrupted after generic sparse linear algebra exceeded 16 minutes; the second failed on JSON serialization; both logs are retained. [pcc:unverified]

The source-bound Sage CPU record charges 422.170193 s to the full GHS path, including setup, transfer, factor base, relation acquisition, linear algebra, descent and verification. The corrected Frobenius-quotient rho arm completed 30 fixed-seed same-target solves, all with the same scalar and no restarts; its median one-solve CPU is 19.4710205 s including setup. The recorded ratio is 21.6819757, with GHS slower. Wall time was not separately recorded. The original rho arm used negation and encountered short cycles; its failure logs are retained. [pcc:unverified]

These are **historical Sage/Python-source timings**. The saved receipts do not record host identity or isolation level. The repository's current native-implementation rule requires a native rerun before a new performance claim or leaderboard entry. The modular audit and transfer reconstruction are producer-side checks, and no independent signed PCC verdict exists for the timing. This result is therefore a source-bound development measurement, not a validated benchmark. [pcc:unverified]

## Prior work and novelty status

The cover belongs to the already investigated odd-characteristic degree-seven cover orbit in this repository. The exact base-change instance and subgroup receipt are new to this search; the eligible cover family and transfer method are not established as new. Iijima--Momose--Chao (2015) classified the `(2,...,2)` odd-characteristic prime-degree cover setting through degree seven under its stated conditions. An equation alone gives no claim of a new vulnerable family, and this measured instance does not satisfy a below-rho cost criterion. [pcc:unverified]

Primary prior work: https://eprint.iacr.org/2015/805.pdf . Local source: `../ghs_odd5_carrier_20261115/DEGREE7_FINDING.md`. Source files and receipts in this directory reproduce the exact instance and phase accounting. A native matched implementation is the remaining performance validation step.

The [cost chart](cost.svg) shows the two recorded CPU totals, and [RESULT.pdf](RESULT.pdf) contains the source-linked visual report. No canonical vulnerability graph is changed because the comparison is legacy, unvalidated, and below no rho threshold.
