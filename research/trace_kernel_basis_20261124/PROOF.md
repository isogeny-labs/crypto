# Derivation, algorithm contract and proof map

Status: written arguments applying known trace and matroid results, plus
finite native development controls. No Lean-checked or independently signed
proof standing is asserted. [pcc:unverified]

## 1. Trace-compatible coordinates

For E:y^2+xy=x^3+a2*x^2+c over K=GF(2^n), the known map
P -> Tr(x(P)+a2), with O mapped to zero, has kernel 2E(K). Thus any
odd-order subgroup is in the kernel. A finite subgroup point has
Tr(x)=tau=Tr(a2). This is Kosters–Yeo Proposition4.2. It is necessary in
general, and need not isolate one particular odd-order subgroup.

Let v0,...,v(n-1) be any binary field basis; choose k with Tr(vk)=1.
Eliminating the trace equation from u=sum_i yi*vi gives

    yk = tau + sum_{i != k} Tr(vi)*yi,
    u  = tau*vk + sum_{i != k} (vi+Tr(vi)*vk)*yi.

The remaining coefficient vectors have trace zero. Expanding any dependence
in the original basis proves their independence, so they span ker(Tr).
This is a bijective affine parameterization of the required trace hyperplane.
Direct parameterization and trace elimination give identical polynomial
coefficients when the pivot and remaining coordinate labels agree.
[pcc:unverified]

The necessary condition applies to finite auxiliary partial sums of rational
subgroup points. Infinity has no affine x-coordinate. A complete collection
adapter must account for identity partial sums and check rational lifting,
sign consistency and the final group relation. This component constructs
polynomials; it does not claim those semantics are supplied by every root.

## 2. Additive auxiliary-column support

Use one auxiliary u in the ordered chain

    S3(x1,x2,u)=0, S3(u,x3,t)=0,
    S3(x,y,z) = x^2*y^2 + z*x*y + z^2*(x^2+y^2) + c.

For fixed leaf polynomials, target t and offset o, substitute
u=o+sum_j b_j*z_j into the Boolean quotient z_j^2=z_j. Squaring is
GF(2)-linear. The equations become F10+sum_j z_j*G1(b_j) and
F20+sum_j z_j*G2(b_j), where

    G1(b) = b^2*(x1^2+x2^2) + b*x1*x2,
    G2(b) = b^2*(x3^2+t^2) + b*x3*t.

Each G is GF(2)-linear in the field vector b. Let support count coefficient
ones after descent in the SAME fixed output field basis. Different auxiliary
bits occur in disjoint monomials and never occur in F10,F20. Therefore

    support(F1,F2) = C + sum_j w_t(b_j),
    w_t(b) = support(G1(b),G2(b)).

Summing over a fixed training set preserves additivity. Constants and leaf
terms belong to C and are included in reported whole-chain support.
The code checks this identity against full polynomial construction.
[pcc:unverified]

This separation is for ONE auxiliary basis. In a longer chain an equation
containing TWO variable auxiliaries can couple their basis choices; this
argument does not justify jointly optimizing all of them by independent
greedy passes. It also does not optimize an output equation basis, perform
Gaussian elimination, or imply anything about elimination fill or degree.

## 3. Minimum-weight basis: standard greedy application

The ground set is ker(Tr) without zero. Independent sets are the linearly
independent field vectors over GF(2), so they form a vector matroid.
Sort all candidates by (training weight,encoded value). Keep a vector if it
increases rank, stopping at n-1. This is the classical minimum-weight basis
algorithm, not a new matroid result.

Exchange argument: maintain a minimum-weight basis B containing the current
greedy prefix. The next greedy vector v is independent of the prefix. If it
is not already in B, the fundamental dependence in B union {v} contains a
vector b outside the prefix that can be exchanged for v. Such a b was also
eligible at this greedy step; hence w(v)<=w(b). The replacement basis has
no higher weight and remains minimum. Induction gives a minimum-weight basis
at termination. This is the standard argument in Edmonds1971.
[pcc:unverified]

Full-kernel optimality requires ALL nonzero kernel vectors. A restricted
candidate pool only gives the optimum inside that pool. Dependence is tested
on b, not on its coefficient image G(b); the image need not be injective.
The enumeration has 2^(n-1)-1 candidates and is not a large-field algorithm.

## 4. Proof map and evidence boundary

| Statement | Basis / check | Formal and independent status |
|---|---|---|
| Odd subgroup trace condition | Kosters–Yeo Proposition4.2 | Published result reused, not re-proved by this code |
| Direct chart equals trace elimination | linear substitution above; exact coefficient test | Written; no Lean build |
| Same hyperplane for each kernel basis | rank and exhaustive small span controls | Finite producer checks |
| Additive single-auxiliary support | explicit G1,G2 expansion; direct support comparison | Written; finite controls |
| Minimum additive support basis | Edmonds vector-matroid greedy; exhaustive rank-three controls | Published general method applied; no Lean bridge |
| Lower actual solving/collection cost | no evidence yet | Open |

The formalization preflight found no `lean` or `lake` executable; invoking
`lake --version` returned command-not-found. No theorem module was built and
no axiom audit is claimed. A faithful future Lean bridge must connect the
Boolean quotient, field-valued coefficient map, chosen trace kernel, support
function, and the matroid minimum-basis statement; an unrelated arithmetic
lemma would not certify this derivation.

The current repository `identity.certificate/v1` format evaluates ONLY over
the fixed odd prime 2^61-1. It cannot certify characteristic-two Frobenius
linearity or Boolean reduction here. No incompatible IDC1 certificate is
fabricated. A binary-field-aware identity verifier or faithful formal proof
is a specific remaining evidence dependency.
