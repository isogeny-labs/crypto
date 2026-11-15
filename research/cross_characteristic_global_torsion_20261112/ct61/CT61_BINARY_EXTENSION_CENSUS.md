# CT61 — binary elliptic base-extension census and fixed genus-two comparison

**Status:** bounded exact arithmetic; scientific standing [pcc:unverified]. This is a subgroup-*availability* test, not a transfer algorithm.

## Question and experiment
CT60 enumerated all five elliptic traces over F2. Does permitting binary target curves defined over F4 and F8 change the minimum total residue degree in which some odd-prime-order subgroup exists?

Use generalized Weierstrass models
`y²+a1*x*y+a3*y=x³+a2*x²+a4*x+a6`
over F_(2^k), k=1,2,3, with polynomial moduli `x+1`, `x²+x+1`, `x³+x+1`. Enumerate all 2^(5k) coefficient tuples; explicitly reject singular models by solving the simultaneous partial-derivative equations, count every rational affine point plus infinity, and collect all achieved Frobenius traces `t=q+1-#E(Fq)`.

For each trace and listed prime ell, use the exact recurrence `S0=2, S1=t, Sn=t*S(n-1)-q*S(n-2)`, and `#E(F_(q^f))=q^f+1-Sf`. Search f<=256. Report the minimal *total binary field degree* d=k*f. This tests divisibility of group order and, for prime ell, implies an element of order ell exists by Cauchy's theorem. It does not construct a global point compatible with the source input.

## Locally executed result

All traces realized by a smooth model:
- F2: -2,-1,0,1,2.
- F4: -4,-3,-2,-1,0,1,2,3,4.
- F8: -5,-4,-3,-1,0,1,3,4,5.

| ell | best total d | (base degree k, extension f, trace t) | fixed CT56 genus-two/F2 least f (if <=256) |
|---:|---:|---|---:|
| 3 | 1 | (1,1,0) | 4 |
| 5 | 1 | (1,1,-2) | 24 |
| 7 | 2 | (2,1,-2) | 16 |
| 11 | 5 | (1,5,-1) | 40 |
| 17 | 16 | (1,16,-2) | 96 |
| 19 | 6 | (2,3,1) | 1 |
| 23 | 11 | (1,11,-1) | 132 |
| 37 | 6 | (2,3,3) | >256 (not resolved within cap) |
| 43 | 7 | (1,7,0) | >256 |
| 163 | 27 | (1,27,-1) | >256 |
| 257 | 32 | (1,32,-2) | >256 |

Independent hand recurrence controls: `#E(F4)=7` for t=-2; `#E(F64)=76` for q=4,t=1; `#E(F64)=74` for q=4,t=3; `#E(F128)=129` for q=2,t=0.

The fixed genus-two column uses the **specific CT56 witness**, `#C(F2)=#C(F4)=6`, giving Jacobian Frobenius polynomial `T⁴+3T³+5T²+6T+4`. For each ell, a modular companion-matrix determinant test finds the first f<=256 with ell dividing `det(I-F^f)`. This is NOT a genus-two census and does NOT give a genus-two lower bound. The genus-two witness admits 19-torsion over F2 but not the other listed large primes over F2.

## Scope and limitations

- Search is exhaustive over all generalized elliptic models for k=1..3, but not over k>=4 and not over genus-two curves.
- The table's minimum is only among the enumerated fields and f<=256; a censored fixed-genus-two entry means no result in that window, not nonexistence.
- Existing original ECDLP points P,Q are *not* lifted or mapped by the census.
- A global number-field degree >=d remains necessary for an actual point specialization at binary residue degree d. The test does not bound bit complexity or construct a compatible characteristic-zero lift.
- No cold solve, no rho comparison, no cost improvement and no paid compute.

## Sources and execution receipt

Prior internal CT55–60 and `problems/lift-reduce-second-prime-transfer.md`. External math references: [Magma handbook on L-polynomials](https://docs.magma-maths.org/AlgebraicGeometry/AlgebraicCurves/function_field.html), and [A Comparison of Point Counting Methods for Genus 2 Curves](https://eprint.iacr.org/2004/241.pdf). Shared research-KB MCP is unavailable in this session; fallback GitHub research inspected. No novelty claim. Local command: `python3 ct61_census.py`; it completed successfully. Production CI/PCC has not yet confirmed this head.

## Next unblocked step

Census *smooth genus-two models* over F2 and F4 using independent point counts over the first two extensions and a full Jacobian group order recurrence; evaluate whether any small genus-two target beats this elliptic residue-degree frontier. Only then attempt explicit source-to-target globally compatible point lifting, which remains the essential unsolved algorithmic gate.

[pcc:unverified]
