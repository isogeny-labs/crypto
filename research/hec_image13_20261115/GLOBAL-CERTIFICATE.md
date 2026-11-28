# Global cubic and singular-quartic certificate

Post-run mathematical supplement, 2026-11-15. This does not rerun or rewrite
the frozen interpolation gate. Its raw global_identity_certified=false flag
records what that empirical gate alone establishes. The additional theorem
below uses its exact ranks plus a source-checked global dimension bound.
It certifies identities, not equality of the gradient zero scheme with the
entire coordinate image, curve recovery or independent validation.

## 1. Precise input
For each V, previous source26a74be6430e and its report construct A=J/V and a
symmetric principal line bundle M with psi*M=O(3W). Eight descended sections
s0..s7 form a complete basis of H0(A,2M). Opposite character products make
these sections even for the square of a symmetric linearization of M.
Their recorded evaluations are in F127; all arguments below base-change to
an algebraic closure. No curve model for A is assumed.

The frozen image sourceb1fb249426b6 gives ranks36and112 for the quadratic
and cubic evaluation matrices, a one-dimensional quartic-gradient kernel,
and exact coefficient membership of all eight quartic partials in the
fitted eight-dimensional cubic space. These are deterministic finite-field
linear-algebra inputs, not an inference from holdout agreement.

## 2. Tame trace lemma, including characteristic127
Choose a normalized symmetric linearization iota*(-) on M for inversion
 iota=[-1]. It squares to the identity. At a fixed point T in A[2], its
fiber eigenvalue is plus or minus1. On N=M^6 this eigenvalue is therefore1.
Inversion has exactly2^6=64geometric fixed points, reduced because127!=2.
Its tangent and conormal eigenvalues are all-1 in dimension3.

Apply Donovan, The Lefschetz-Riemann-Roch formula, Section3(Brauertrace),
Section4(conormal exterior-class denominator), Corollary5.5 and its finite-
group application, https://www.numdam.org/article/BSMF_1969__97__257_0.pdf.
The hypotheses are smooth projective over an algebraically closed field,
periodic automorphism with order prime to characteristic, and an equivariant
coherent sheaf. Here A is an abelian variety, order2 is prime to127, and N
is a linearized line bundle. Every fixed component is a point. The local
Todd factor is1 and the alternating conormal exterior trace is(1-(-1))^3=8.
Teichmüller lifts of plus/minus1 are those same integers. Thus the alternating
Brauer trace is exactly64/8=8 in the characteristic-zero Witt fraction field,
not merely a trace congruence modulo127. This avoids the potential ambiguity
of recovering integral eigenspace dimensions from field traces.

Higher coherent cohomology of N vanishes: for arbitrarily large m prime to
127, trace for the finite étale map[m] splits O_A off[m]_*O_A, since its
rank m^6 is invertible. Projection formula makes H^i(N) a direct summand of
H^i([m]*N). Symmetry gives[m]*N=N^(m²); Serre vanishing kills the latter
for i>0 and sufficiently large m. Riemann-Roch for a principal polarization
gives h0(N)=chi(N)=6^3=216. For this dimension formula see also Kieffer,
Isogeny graphs of abelian varieties over finite fields, Theorem1.1.16/proof,
https://members.loria.fr/JKieffer/files/isogenycourse.pdf (Mumford references).

Because iota has order2 in odd characteristic, H0(N) is the direct sum of
its plus/minus eigenspaces. If their integer dimensions are a,b, its Brauer
trace is a-b, so

    a+b=216,  a-b=8,  hence a=112 and b=104.

This argument applies to the symmetric ppav itself. It does not require
A to be a hyperelliptic/nonhyperelliptic Jacobian. Müller's Section3 formula
for even sections corroborates112 but is not used to smuggle a curve
hypothesis into the argument.

## 3. Rank sandwich proves all eight global cubic identities
Let R3 be the120-dimensional space of homogeneous cubics in eightvariables.
Multiplication of the eight even sections defines

    mu: R3 -> H0(A,6M)^+.

The lemma bounds its rank by112, hence dim ker(mu)>=8. Evaluate sections
at the160regular trainingpoints using the same nonzero frames as recorded.
Every global relation evaluates to zero, so

    ker(mu) is a subspace of ker(E3).

The exact rank E3=112 means dim ker(E3)=8. Thus both kernels have dimension8
and are equal. Every one of the eight fitted cubic coefficient vectors is
therefore a GLOBAL identity on the quotient image. This does not assume
projective normality or use a probability argument. Rank is unchanged after
extending F127 to its algebraic closure; the conclusion includes geometric
relations. Holdouts remain additional implementation checks.

The quadratic matrix has full rank36. Its global kernel is contained in
its zero evaluation kernel, so there is no nonzero global quadric relation,
over F127 or its algebraic closure.

## 4. The unique quartic is globally singular along the image
Each fitted quartic partial is an exact linear combination of the now-
certified global cubics. Every partial thus vanishes globally on the image.
Euler's homogeneous identity sum Xi*dF/dXi=4F, with4invertible in F127,
shows F vanishes there too. Conversely, any quartic singular along the
image belongs to the one-dimensional sampledgradientkernel. Therefore for
each of the two kernels the recorded quartic is, up to scalar, the UNIQUE
nonzero quartic globally singular along its coordinate image. Its gradient
span is the full global cubic space, by recorded exact coefficient rank8.

## 5. Scope, provenance and next step
The global identities and unique singular quartic now follow from the
mathematical supplement plus pinned exact ranks; the standalone executable
still correctly labels its own empirical output as uncertified. This is
an explicit proof extension, not a change to that run or a validation
signature. No claim that the gradient equations define exactly the image
scheme, or that no extra components occur, is made. Indecomposability,
quotient curve/descent/Serre and divisor transport still need their own
geometric argument/construction. Complete costs remain unmeasured here;
full mathematical10xgenus2/3/4scope retained.

Donovan's theorem and setup were read directly in the primary PDF, including
positive-characteristic Brauer lifting and the prime-to-characteristic order
condition. Initial web refetches timed out; archive.numdam.org TLS mismatch
was not bypassed. The www.numdam.org PDF downloaded with certificate checks
and was read via pdftotext. No scientific computation or source rerun was
required for this proof. Sources and original false raw flag retained.
