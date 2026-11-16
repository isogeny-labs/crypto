# Normalized fixed-index transport across public shifts

2026-11-05. Prime-field source construction with bounded native checks.

Define T_n(R)=psi_n(R)c(R)^(n^2), where c is the normalization seed already used by the source. The new module transports the specific n-values consumed by its half-product calculation: (r+1)/2, 2w, and s+e when s is nonzero. It takes public Q and produces homogeneous pairs for those functions on Q+aP. It does not receive or recover the scalar of Q. [pcc:unverified]

## Conditional identity and cancellation

For ordinary division polynomials on y^2=x^3+Ax+B, the addition identity is

    psi_n(R+P) psi_n(R-P)
      = psi_n(R)^2 psi_n(P)^2
        * (x([n]P)-x([n]R)) / (x(P)-x(R))^(n^2).

One derivation uses sigma(nz)/sigma(z)^(n^2), with the conventional constant sign for psi_n, and the sigma addition formula. The sign cancels in this product identity. Clearing rational denominators gives a polynomial identity valid after specialization to the supported prime-field domain wherever the displayed quotients are defined. This is an algebraic identity; no analytic evaluation or characteristic-changing map is used by the code. [pcc:unverified]

Assume the source's seed normalization law

    c(R+P)c(R-P) = (x(P)-x(R))c(R)^2 c(P)^2.

Multiplying its n^2-th power into the division identity cancels the entire denominator power, giving

    T_n(R+P)T_n(R-P)
      = (x([n]P)-x([n]R)) T_n(P)^2 T_n(R)^2.

This note proves the transport reduction from that seed law. It does not reprove the derivative-normalization law or remove the inherited phase assumptions: prime p>3, odd prime subgroup order r>=7, r!=p and gcd(r,p-1)=1, with the existing word-size constraints. Native checks compare the chosen derivative normalization directly on the stated fixtures. [pcc:unverified]

With T_n(R) represented by [A,B] and T_n(R-P) by [C,D], let delta=x([n]P)-x([n]R) and K=T_n(P)^2. The next homogeneous pair is

    [delta*K*A^2*D, B^2*C].

This step uses six modular products and one coordinate subtraction; no inverse or large exponent is evaluated in the recurrence step. It rejects noncanonical, zero or degenerate operands. Independent scaling of either input pair preserves the represented output. [pcc:unverified]

## Anchors, geometry and initialization

Set T_n(O)=0 as a source convention; no division-polynomial value at infinity is evaluated. The P and 2P vectors are computed during public setup. Since psi_n(-P)=(-1)^(n+1)psi_n(P) and c(-P)=-c(P), T_n(-P)=-T_n(P) for every n. The anchors O,P,-P,2P are handled before recurrence. In particular, the P and 2P anchors prevent an identity/zero predecessor from entering a quotient after crossing O. [pcc:unverified]

The existing prepared geometry already contains [n](Q+aP) for each selected n. The transport reads its affine x coordinates; it does not add new multiplier lanes. For a non-anchor starting point, only offsets0 and1 use direct phase and value-division initialization. Later non-anchor rows use the recurrence, across all selected columns. Public setup builds the base grid, two anchor vectors and the base phase; query geometry, allocations and the initial evaluations remain costs. Window and lane limits are explicit. [pcc:unverified]

## Native evidence

Fifteen tests pass with eight recursive source/data hashes and four compiler warnings retained. Complete subgroup windows over F5/r7, F7/r13 and F509/r479 check1,490 normalized values against freshly evaluated frozen jets, including identity rows. Eleven length16 windows at public points on the F509 fixture check528 additional transported values, including crossings of O and starts at each anchor. Test-only phase/value-initialization counters match the actual direct branches; each tested window needs at most two direct initializations. Complete O-start windows use only anchors and recurrence after setup. [pcc:unverified]

Homogeneous scale tests vary both predecessor scales over three small fields. Domain/size/invalid-point and zero-operand rejection tests are included. The separate check.rs imports no transport implementation; it reconstructs each complete public subgroup by affine addition and recomputes values with the frozen jet evaluator. It accepts all1,490 rows and rejects a changed finite numerator. The checker was authored in the producing session, not an independent review. [pcc:unverified]

The complete windows are checker fixtures, not runtime scalar-answer tables. No target scalar is supplied to the production Source/Prepared API. Original4,563-case cohort is unchanged. No cloud job or timing panel ran.

## Next consumer construction

The current production character residual consumes unscaled division values with factored seed dependence. It cannot simply receive T_n in place of psi_n. The next step is to factor the residual's dependence on the seed and the three initial division values, then consume their transported homogeneous pairs without undoing normalization. Shared initial phases and geometry must be reused rather than duplicated by separate transport/source calls. This integration and complete-cost accounting remain to be implemented; the present artifact is the transport primitive.

## Sources checked

Sources checked (KB): tools unavailable; scripts/research-kb status found no executable. Fallback: saved shared-seed derivation, source residual and prepared geometry; NIST DLMF23.10.3 (https://dlmf.nist.gov/23.10.E3); Katherine Stange, Elliptic nets and elliptic curves, Algebra & Number Theory5(2011), Section3 (sigma construction and addition identity) and Section4 (algebraic specialization), https://msp.org/ant/2011/5-2/ant-v5-n2-s.pdf, printed pages210-217. Sources retrieved2026-11-05. No novelty-priority claim. No native failure recorded; corrupted-column rejection is intentional. [pcc:unverified]
