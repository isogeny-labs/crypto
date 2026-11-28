# CT87 — Explicit 8 × 3 × 2 point-field tower for 11a1[7]

**[pcc:unverified].** Exact fixed-fixture algebraic field presentation, not an arbitrary-input cross-characteristic transport algorithm or measured DLP improvement.

## Result

CT86 established s=x(T)+x([2]T)+x([3]T) with irreducible degree-eight polynomial

    g(S)=S^8−8S^7−840S^6−29862S^5−39690S^4−1696464S^3
         −6423347S^2+3828478S−2102915.

The projective subgroup field F=Q(s) is the fixed field of the six multiplication automorphisms of the oriented point field K=Q(T). The x-coordinate field L=Q(α) has [L:F]=3 and K=L(Y) has [K:L]=2. CT87 makes the middle **relative cubic completely explicit in this same generator s**, rather than referring to unspecified coefficients in F.

Let the subgroup polynomial be

    H_T(Z)=(Z−α)(Z−x([2]T))(Z−x([3]T))
          =Z³−sZ²+e₂Z−e₃.

The following integer coefficient vectors are in **ascending** degree order:

    q₂ = 23624165350186
    C₂ = [51746818064829, -330291288560641, -95029425174429,
          -1711075375204, -1596513316848, -44914399152,
          -455319459, 54044109]

    q₃ = 10252887761980724
    C₃ = [-201756977283948362, 563360952055619683,
          149521192983511524, 3495575179577110, 2638006865239263,
          74263835359077, 711827220769, -88442189137].

Write c₂(S)=∑ C₂[i]S^i and c₃(S)=∑ C₃[i]S^i. Then the exact relative polynomial is

    h(Z)=Z³−s Z²+(c₂(s)/q₂)Z−c₃(s)/q₃ ∈ F[Z].

**Theorem (conditional only on already recorded CT80/82 premises):** L≅F[Z]/(h) and K≅L[Y]/(Y²+Y−Z³+Z²+10Z+20), and the three successive degrees are 8,3,2. Irreducibility of g is certified modulo 3 by CT86, the exact cubic relation is certified below, and the degrees [L:F]=3 and [K:L]=2 were proved by CT86 and CT80 respectively. No genus-two or prime-field ECDLP speed claim follows from this field decomposition.

## Independently reproducible exact arithmetic certificate

As in CT86, define

    D=4X³−4X²−40X−79, B₃=ψ₃²,
    A₂=(3X²−2X−10)²+(1−2X)D,
    A₃=Xψ₃²−D f₄,
    B=D B₃, N=X B + A₂ B₃ + A₃ D.

Thus s=N(α)/B(α). Define E₂=X A₂ B₃ + X A₃ D + A₂ A₃ and E₃=X A₂ A₃. Then e₂=E₂(α)/B(α), e₃=E₃(α)/B(α).

Each of the standalone Python and independently written JavaScript programs constructs ψ₇, N, B, E₂ and E₃ from exact integer polynomial arithmetic and checks these three **unconditional integer divisibility identities**:

    ψ₇(X) | q₂ E₂ B⁶ − ∑_{i=0}⁷ C₂[i] N^i B^(7−i)
    ψ₇(X) | q₃ E₃ B⁶ − ∑_{i=0}⁷ C₃[i] N^i B^(7−i)

    ψ₇(X) | q₂q₃ B⁷ X³ − q₂q₃ N B⁶ X²
             + q₃ X ∑ C₂[i] N^i B^(7−i)
             − q₂ ∑ C₃[i] N^i B^(7−i).

Both use integer pseudo-division by ψ₇, so no floating-point root recognition or assumed scalar target data is involved in **verification**. The coefficients were initially discovered using rational linear algebra over Q[X]/ψ₇, but neither CI verifier depends on SymPy. Note that discovery of the subgroup polynomial *does* use the classical [2] and [3] multiplication maps; this is not an unlabelled, selective lift algorithm.

From the repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct87
    python3 -B "$D/ct87_tower.py" >/tmp/ct87-py.txt
    python3 -O -B "$D/ct87_tower.py" >/tmp/ct87-opt.txt
    node "$D/ct87_tower_independent.js" >/tmp/ct87-js.txt
    node --no-opt "$D/ct87_tower_independent.js" >/tmp/ct87-js-noopt.txt
    cmp /tmp/ct87-py.txt /tmp/ct87-opt.txt
    cmp /tmp/ct87-py.txt /tmp/ct87-js.txt
    cmp /tmp/ct87-py.txt /tmp/ct87-js-noopt.txt
    printf '%s  %s\n' 9ae4951f266e6b9c2ef8e146ab9b82014e7753868f13c4324da368cd8bce371c /tmp/ct87-py.txt | sha256sum -c

All four modes passed in local execution, byte-identically. **CI exact-head and independent scientific PCC signing are separate gates.** [pcc:unverified]

## Research implications and limits

This exhibits a concrete algebraic representation in which global degree 48 separates into subgroup-selection (8), x-orientation modulo sign (3), and ordinate sign (2). The relative cubic coefficients have substantial height/denominators, so a bare field-degree comparison cannot be treated as a runtime or bit-complexity improvement. Passing from K to the degree-eight invariant field F discards the oriented element needed to distinguish Q=[m]P; recovering that information is exactly the remaining selective-recognition challenge, not free transfer.

The next experiment should feed an **unlabelled input** Q at the single fixed source prime into a certified factorization/root-selection procedure for h over F, retaining the ordinate sign and charging polynomial factorization, denominator clearing, number-field embedding, residue transport and target solve. A matched full cold rho/index-calculus comparison is still unperformed.

Sources checked: CT80–86 and root [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877); [PARI number-field roots/factorization](https://pari.math.u-bordeaux.fr/dochtml/html-stable/General_number_fields.html), [Magma factorization over number fields](https://docs.magma-maths.org/BasicRings/UnivariatePolynomialRings/factorization.html), classical finite étale prime-to-p torsion reduction. Research-KB MCP and checked-out wrapper are absent from this tool environment; this is an incomplete literature/Kb survey, not a novelty assertion.

No paid cloud compute, protected merge, toolchain-wide Rust/Lean run or independently signed PCC verdict is claimed.