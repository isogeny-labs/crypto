# CT109 — Exact characteristic-zero line-field polynomial and 2-adic index

**2026-11-14. [pcc:unverified].** This improves CT106/107's number-field presentation. It is not an arbitrary-input cross-characteristic transfer or DLP speedup.

## Theorem: explicit defining polynomial over Q

For E: y²+y=x³−x²−10x−20, T a nonzero global eleven-torsion point, and

    S = x(T)+x([2]T)+x([3]T)+x([4]T)+x([5]T),

CT106 proved [Q(S):Q]=12. **CT109 now proves the following exact monic characteristic-zero minimal polynomial:**

    q(Z) = Z^12 − 20 Z^11 − 5500 Z^10 − 543125 Z^9 − 2165625 Z^8
           − 308550000 Z^7 − 4064649325 Z^6 + 8753368250 Z^5
           − 179918683945 Z^4 − 501858246395 Z^3
           − 4153362124197 Z^2 − 9133681615853 Z − 5490195752091.

This is **not** the modular degree-12 polynomial of CT106. All coefficients are integers; the maximum absolute coefficient is 9,133,681,615,853.

### Global exact certificate

Let psi11∈Z[X] be the degree-60 division polynomial from CT101, with leading coefficient 11; define the usual x-coordinate multiplication rational functions phi_m/psi_m², 1≤m≤5. Their denominator factors are [1,D,f3²,D f4²,f5²], D=4X³−4X²−40X−79. Their sum is **A/B**, A,B∈Z[X] with (deg A,deg B)=(51,50).

The exact homogenized polynomial

    H(X) = B(X)^12 q(A(X)/B(X))

has degree 612. **Exact integer long division proves psi11 divides H in Z[X], with quotient degree 552 and ZERO remainder**; all division coefficients are verified integral. Since psi11 is irreducible of degree 60 over Q (CT102) and deg B=50, B(x(T))≠0, hence q(S)=0. Combined with CT106's degree-12 field theorem, q is the exact minimal polynomial.

Python independently constructs the recurrence, discovers the candidate by CRT and checks the homogeneous identity by Horner evaluation. BigInt JavaScript independently reconstructs the integer division polynomial, assembles H as the sum of 13 terms c_k A^k B^(12−k) and performs separate exact integer polynomial division. Both verify the discriminant by fraction-free Sylvester determinants.

### Discovery and independent residue controls

The Python modular probe works in Fp[X]/(psi11) via extended Euclid and Krylov linear dependence. Eleven primes **13,17,19,23,29,31,37,41,43,47,59** produce degree-12 relations whose coefficientwise CRT, with product **15,704,977,131,048,049**, proposes the displayed polynomial. Three held-out primes **61,79,101** reproduce its reductions. These congruences **do not** certify a characteristic-zero height bound; the exact integer identity above is the proof.

Modulo 13, q agrees with CT106. At 59, q has linear roots 23 and 32 (CT107). At 2, q=(Z⁴+Z+1)³, as predicted by CT107's rank-four reduction defect.

### Exact 2-adic index (strengthening CT107)

Two independent 23×23 fraction-free Sylvester determinant implementations show:

    disc(q)<0,  |disc(q)| has 195 decimal digits,  v_2(disc(q))=24.

E has good reduction at 2; 11 is prime to 2. By the Néron–Ogg–Shafarevich good-reduction criterion, the complete 11-division field and its subfield M=Q(S) are unramified at 2. Hence v₂(disc(O_M))=0 and the discriminant/index formula gives

    v₂([O_M : Z[S]]) = (24−0)/2 = 12.

This **sharpens the CT107 lower bound ≥8 to the exact valuation 12**. It confirms that the power basis 1,S,…,S¹¹ is nonmaximal at the binary prime and cannot distinguish all three binary degree-four residue primes. It does **not** certify CT108's separately developing (S,e₂) basis; those checks are independent.

### Reproduction

From repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct109
    python3 -B "$D/ct109_exact_minpoly.py" > /tmp/ct109-py.json
    python3 -O -B "$D/ct109_exact_minpoly.py" > /tmp/ct109-pyopt.json
    node "$D/ct109_exact_minpoly_independent.js" > /tmp/ct109-js.json
    node --no-opt "$D/ct109_exact_minpoly_independent.js" > /tmp/ct109-jsopt.json
    cmp /tmp/ct109-py.json /tmp/ct109-pyopt.json
    cmp /tmp/ct109-py.json /tmp/ct109-js.json
    cmp /tmp/ct109-py.json /tmp/ct109-jsopt.json
    printf '%s  %s\n' 54bd0e9ea52e9054c0638a869e57e50bf16339acf0701475e31d2f32cba1b8b1 /tmp/ct109-py.json | sha256sum -c

Four locally executed modes matched the digest above using the separately tested implementations of the exact theorem; the committed-code exact-head run is a distinct CI gate. Neither producer uses optimization-removable Python assert checks. A passing policy/PCC linter is not an independent scientific signature.

### Sources and boundaries

Repository prior work: CT101–107, CT108 preregistration, root PR #3877, stacked PR #4102, AGENTS.md and the 2026-11-14 session log. Methodological references: [Magma division polynomials](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurves/function_field.html); [PARI relative number-field algorithms](https://pari.math.u-bordeaux.fr/dochtml/ref/Relative_extensions_of_number_fields_and__backslashZ_K_minusmodules.html). Research-KB MCP and checkout wrapper were unavailable in this connector runtime; no novelty claim. Concurrent cryptanalysis/crypto-autoresearcher Jacobian work and CT108 were preserved.

Remaining gates: produce the relative quintic H_T over the exact Q[S] basis, diagnose denominators at 2; join the independent binary saturation work; construct a scalar-independent **global** lift of public source Q with fixed target residue pairing and exact certificates; then measure full cold costs against strongest matched rho/BSGS/IC. No Rust/Conductor full-suite validation, paid EKS run, protected merge, PCC signature, complete ECDLP transfer or end-to-end advantage is claimed. [pcc:unverified]
