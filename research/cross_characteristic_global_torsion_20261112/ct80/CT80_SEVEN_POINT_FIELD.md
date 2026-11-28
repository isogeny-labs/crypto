# CT80 — An explicit degree-48 seven-torsion point field and characteristic-two residue [pcc:unverified]

## Claim and exact scope

For Cremona 11a1, use the rational curve

\[
E:y^2+y=x^3-x^2-10x-20.
\]

Construct the global algebra

\[
K=\mathbb Q[X,Y]/(\psi_7(X),\,Y^2+Y-X^3+X^2+10X+20).
\]

The standard seven-division polynomial has degree 24, leading coefficient 7, and largest absolute coefficient 2,207,441,511,783,053 (51 bits). Its reduction modulo 3 is irreducible, and the ordinate equation remains irreducible over the degree-24 x-coordinate field. Consequently this quotient is a degree-48 **field**, not merely a product algebra. This is an explicit fixed-curve point-field presentation, not a new or scalable ECDLP attack. [pcc:unverified]

## Polynomial and field certificates

Let

\[
D=4X^3-4X^2-40X-79=\psi_2^2,\quad
\psi_3=3X^4-4X^3-60X^2-237X-21,
\]

and

\[
f_4=2X^6-4X^5-100X^4-790X^3-210X^2-1496X-5821=\psi_4/\psi_2.
\]

The classical recurrences produce

\[
\psi_5=D^2 f_4-\psi_3^3,\qquad
\psi_7=\psi_5\psi_3^3-D^2f_4^3.
\]

Python constructs the integer coefficients by convolution and runs Rabin's irreducibility test modulo 3, checking that X to the power 3^24 equals X modulo the polynomial and that the gcd gates at exponents 3^12 and 3^8 equal 1. JavaScript independently reconstructs the polynomial using BigInt and computes the modular Euclidean gates. Irreducibility modulo 3 implies the x-coordinate field has degree 24 over Q. [pcc:unverified]

At the binary prime, exact factorization is

\[
\psi_7(X)\bmod2 =
(X^{12}+X^9+X^5+X^2+1)
(X^{12}+X^9+X^5+X^4+X^2+X+1).
\]

Both factors are irreducible: Python uses Rabin's criterion, while JavaScript independently tests every monic possible divisor of degree at most six. Define alpha by the first factor, and let g=alpha^3+alpha^2 in F_(2^12). The directly computed absolute Artin–Schreier trace of g is **1**, so Y^2+Y+g is irreducible over F_(2^12). Hence the target field is exactly F_(2^24), represented by this tower.

The binary point U=(alpha,Y) has order seven; raising both coordinates to 2^12 fixes alpha but sends Y to Y+1, so it negates U. Since the defining polynomial for Y is monic and X is integral at this unramified binary prime, if Y lay in Q(X), reduction would give an impossible root of the trace-one equation in F_(2^12). Therefore the number field of the oriented point has degree **48**. This is a restriction for this fixed E/Q and its binary reduction, not for arbitrary variable binary elliptic targets or genus-two/higher targets. [pcc:unverified]

## Source place and end-to-end public toy

At p=37, the curve has 35 points, and its seven-division polynomial is squarefree with factor-degree pattern 1^3 3^7; its linear roots are 9, 27 and 28. The public source point P=(9,4) has exact order seven and selects a degree-one source prime via X→9, Y→4. The same global point T=(X,Y) specializes to U in the binary degree-24 field.

| k | Source public point | Binary target coordinates as pairs of 12-bit polynomial encodings |
|---:|---|---|
| 0 | O | O |
| 1 | (9,4) | ((2,0),(0,1)) |
| 2 | (27,9) | ((17,0),(77,1)) |
| 3 | (28,0) | ((1562,0),(2879,1)) |
| 4 | (28,36) | ((1562,0),(2878,1)) |
| 5 | (27,27) | ((17,0),(76,1)) |
| 6 | (9,32) | ((2,0),(1,1)) |

The Python and JavaScript programs independently evaluate the source and target group laws and check all seven recovered target logarithms. Applying 2^12-Frobenius to only Q, leaving the generator fixed, changes the logarithm; this is the wrong-orientation negative control.

**Essential limitation:** source-to-global recognition still enumerates [k]P. For seven independently repeated exhaustive scans, the additive source cost would be 21 group additions and the target recovery cost another 21 target additions, before global construction. The implementation reuses its seven-entry tables, so these are counterfactual scan counts, not timed operations. There is no non-enumerative encoded-input algorithm and no complete rho/index-calculus speedup. [pcc:unverified]

## Reproduce

Run from the repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct80
    python3 -B "$D/ct80_seven_pointfield.py" >ct80-py.jsonl
    python3 -O -B "$D/ct80_seven_pointfield.py" >ct80-pyopt.jsonl
    node "$D/ct80_seven_pointfield_independent.js" >ct80-js.jsonl
    node --no-opt "$D/ct80_seven_pointfield_independent.js" >ct80-jsopt.jsonl
    cmp ct80-py.jsonl ct80-pyopt.jsonl
    cmp ct80-py.jsonl ct80-js.jsonl
    cmp ct80-py.jsonl ct80-jsopt.jsonl
    echo '65451a652799895b61587f085914e0343da6dc79fbac3535caf292711ac0414c  ct80-py.jsonl' | sha256sum -c

Local Python, optimized Python, Node, and no-opt Node output were identical with the above SHA-256 in the standalone test. This is producer-side execution evidence, not independent signed PCC certification. Full Rust/Lean/conductor gates and EKS were not run in this connector context. [pcc:unverified]

## Sources and open obligations

Prior repository CT69, CT75, CT76, CT78, CT79; root [crypto PR #3877](https://github.com/isogeny-labs/crypto/pull/3877); stacked [#4015](https://github.com/isogeny-labs/crypto/pull/4015); distinct ongoing native [cryptanalysis PR #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523). Primary documentation: [Sage elliptic division polynomials](https://doc-gitlab.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_generic.html) and [Snowden, elliptic curves over DVRs](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html). The underlying theorems are classical; no novelty claim. Research KB MCP, local CLI, and Conductor were unavailable, so the literature review is incomplete. [pcc:unverified]

Next: derive a non-scalar-enumerating algorithm to recognize globally compatible torsion representatives for arbitrary encoded P and Q at one source and one target place. Charge number-field construction, coefficient growth, factorization, root selection, local-to-global identification and target solving, then compare the *complete cold cost* with optimized rho and current matched IC baselines. Neither a general cross-characteristic transfer nor a computational advantage is established. No paid compute and no merge bypass. [pcc:unverified]
