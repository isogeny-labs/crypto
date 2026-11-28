# CT95 — Exact rational-degree gate for global torsion recognition

**Scientific standing: [pcc:unverified].** This is a bounded proof and development reproduction; no cryptanalytic speedup or general impossibility theorem.

## Starting point and prior work

CT80 constructed a degree-24 x-coordinate field `L=Q(alpha)` for the nonzero seven-torsion of 11a1, with source place alpha=9 modulo 37. CT84 reconstructed the doubling image of the public point (27,9) using a rational Padé template with numerator degree <=4 and denominator degree <=3, over 18 source digits. CT85 identified the endomorphism rigidity limitation. CT93–94 fixed the consistent characteristic-two embedding and established a necessary degree-24 target field. No CT84 degree template is automatically valid for arbitrary source targets.

## New rigorous scoped obstruction

For the same global generator T with x(T)=alpha, the tripling rational function is

  x([3]T)=phi_3(alpha)/psi_3(alpha)^2.

With D=4X^3−4X²−40X−79 and f4=psi4/psi2 from CT84, the **exact** polynomial is

  phi_3(X)=X*psi_3(X)^2−D(X)*f4(X).

The reduced degrees of (phi_3,psi_3²) are (9,8); an independent rational polynomial Euclidean calculation gives gcd(phi_3,psi_3²)=1. The source x-coordinate satisfies x([3]P)=28 mod 37.

Suppose A,B in Q[X], B(alpha) !=0, deg A<=4 and deg B<=3, satisfy

  A(alpha)/B(alpha)=phi_3(alpha)/psi_3(alpha)^2.

Then the polynomial A*psi_3²−B*phi_3 vanishes at alpha and has degree at most max(4+8,3+9)=12. Since alpha has minimal polynomial of degree **24**, that polynomial must be identically zero. Coprimality then forces deg A>=9 and deg B>=8 unless both are zero, contradicting the budget. Thus:

> **No rational function with numerator degree <=4 and denominator degree <=3 can represent x([3]T) globally in L.**

This is stronger than an LLL failure, independent of source precision and candidate coefficient height. It does not exclude a higher-degree representation, a non-rational-function root representation, or a uniform global-algebraization algorithm.

## Developmental Padé falsifier

`ct95_degree_gate.py` reuses the CT84 exact-integer LLL implementation, with the same source root alpha=9 and a source target chosen only by its x-coordinate. It calculates independently Hensel-lifted source roots and reruns the **same** (4,3) budget. The following are maximum absolute coefficients of the first admissible reduced basis row, **not** certified shortest norms or lower bounds.

| Source target x | Precision 18 | Precision 24 | Precision 36 |
|---|---:|---:|---:|
| 27 (doubling) | 158 | 158 | 158 |
| 28 (tripling) | 905 | 8,539 | 802,789 |

The tripling coefficient growth agrees with the exact no-(4,3)-representation theorem; it cannot be interpreted as a general runtime lower bound. The result for doubling is a positive control.

Local execution ran equivalent CT84 exact-rational LLL routines, `ct95_degree_gate.py` with the original CT84 algorithm reproduced, and exact rational gcd checks. The committed CI must check the actual repository import before the result is promoted. No full Rust/Conductor run or independent PCC signature.

## General symbolic criterion

Let alpha be algebraic of degree D and let a reduced rational function r=A_m/B_m have degrees u,v with gcd(A_m,B_m)=1. A candidate A/B of degrees (a,b) satisfying r(alpha)=A(alpha)/B(alpha), with nonzero denominators, **must equal r as a rational function** whenever max(a+v,b+u)<D. This follows immediately from minimal-polynomial divisibility. Consequently, if a<u or b<v, such a candidate cannot exist.

This criterion is an exact algebraic filter to run **before** spending precision or lattice reduction on a fixed template.

## Acceptance / next experiment

1. Use the degree gate to rule out inadmissible templates before local lifting.
2. Test a *scalar-independent* expanding-degree search (including tripling's (9,8) budget) with actual number-field point and y-coordinate certification, and without declaring success based solely on a local congruence.
3. Charge the degree-24 field construction, coefficient height, LLL, source precision, target arithmetic, and final log recovery against direct BSGS/rho, on the same public inputs and metric.
4. Larger orders require a uniform degree/height stopping rule; neither this example nor fixed-scale LLL solves that problem.

Sources: repo CT80, CT84, CT85, CT93, CT94; classical division polynomials and elementary minimal-polynomial theory. Research-KB MCP/checkout CLI unavailable in this connector context. No novelty claim.
