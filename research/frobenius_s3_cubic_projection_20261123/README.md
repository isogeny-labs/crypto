# Cubic trace multipliers for the binary S3 selector

At field degree 23, cubic Frobenius-equivariant trace multipliers span the 13-dimensional left kernel of the normalized two-coordinate equation at every nonzero sum, for both tested Frobenius subspaces. At field degrees 47, 71, and 79 the entire cubic multiplier space has pointwise rank 4, while the left kernels have dimensions 25, 37, and 41. The unrestricted result follows from spectral block ranks and a span certificate in both subspaces at each size. [pcc:unverified]

| Field degree | Selector dimension | Cubic equivariant space | Pointwise rank | Full left-kernel dimension |
| ---: | ---: | ---: | ---: | ---: |
| 7 | 3 | 6 | 5 | 5 |
| 23 | 11 | 35 | 13 | 13 |
| 47 | 23 | 50 | 4 | 25 |

## Construction

Let \(\tau(u)=u^2\) on \(K=\mathbb F_{2^n}\). For each tested field, take \(U=\ker g(\tau)\), where \(g\) is one of the two odd-degree factors of \((z^n+1)/(z+1)\) over \(\mathbb F_2\). Write \(k=\dim U=(n-1)/2\). The verifier checks that \(U\) is trace zero and totally isotropic under the trace pairing, so \(U^\perp=U+\mathbb F_2\). The Frobenius coordinate functions are \(Z_i(s)=s^{2^i}\) for \(0\le i<k\). Moore invertibility makes them a basis for linear coordinate functions over \(K\). Products with distinct indices give a basis for the filtered polynomial functions, since squares reduce to linear forms. Frobenius equivariance restricts their coefficients to \(\mathbb F_2\).

A multiplier \(\alpha\) annihilates the unknown \(x\) in \(x^2+sx=p(s)\) exactly when \(\sqrt\alpha+s\alpha\in U^\perp\). The symbolic calculation squares this expression and applies \(g(\tau)(\tau+1)\). It uses the function-ring reductions \(Z_i^2=Z_{i+1}\) and \(Z_{k-1}^2=\sum_{i<k}g_iZ_i\). The resulting matrix has 232 columns at degree 23 and 2,048 columns at degree 47. Construction uses these polynomial coordinates without enumerating the elements of \(U\). The field-arithmetic check independently verifies the multiplier condition at every evaluated representative. [pcc:unverified]

For \(s\ne0\), the kernel of \(x\mapsto x^2+sx\) on \(U\) is \(\{0,s\}\), so the left kernel has dimension \(n-k+1\). The pointwise rank is constant on each Frobenius orbit by equivariance. At 7 and 23 bits the verifier visits every nonzero orbit. At 47 bits it samples 128 nonzero sums per subspace and checks the following symbolic certificate over the entire function space. [pcc:unverified]

## Span certificate at 47 bits

Write \(\alpha_0=1\) and, for \(1\le r\le3\),

\[
\alpha_r(s)=\sum_{j=0}^{r-1} Z_{-j}(s)\prod_{i=1}^{r-j-1}Z_i(s),
\qquad Z_i(s)=\tau^i(s).
\]

The script forms the span of each \(\alpha_r\) multiplied by the Frobenius-invariant scalar functions of degree at most \(3-r\). A Frobenius-invariant scalar takes values in \(\mathbb F_2\). For both 47-bit factors, this span has dimension 50 and equals the entire equivariant cubic space. At every sum its evaluations are therefore in the span of four values. [pcc:unverified]

For the lower bound on rank, \(T_s(a)=\sqrt a+sa\) sends \(\alpha_r(s)\) to \((\tau^r+\tau^{-r})s\) for \(r>0\) by cancellation, and sends 1 to \(1+s\). On \(U\), \(u=\tau+\tau^{-1}\) has minimal polynomial of degree 23. If a Frobenius conjugate of an eigenvalue \(\zeta\) fixes \(\zeta+\zeta^{-1}\), it sends \(\zeta\) to itself or its inverse. The powers of 2 modulo 47 have odd order 23, so inversion is excluded. The three operators are \(u,u^2,u^3+u\), which are independent on every nonzero \(s\in U\). The value \(1+s\) has trace one. This gives rank four at every nonzero sum. [pcc:unverified]

## Unrestricted cubic multipliers

The coefficient Frobenius \(\sigma\) on \(K\) has a normal basis and therefore has each \(n\)-th root of unity as an eigenvalue once after extending scalars to \(E=\mathbb F_{2^k}\). Let \(\zeta\in E\) be a root of \(g\). Write \(Q=\{2^i\bmod n:0\le i<k\}\), let \(\phi\) act on the coordinate functions, and set

\[
Y_j=\sum_{r=0}^{n-1}\zeta^{-jr}\phi^r(Z_0),\qquad j\in Q.
\]

These satisfy \(\phi(Y_j)=\zeta^jY_j\), \(Z_1=\sum_{j\in Q}\zeta^jY_j\), and \(Y_j^2=\zeta^{2j}Y_{2j}\). In the coefficient block \(\sigma=\zeta^\ell\), the squared multiplier test applies \(g(\zeta^\ell\phi)(1+\zeta^\ell\phi)\) to \(\alpha+\zeta^\ell Z_1\phi(\alpha)\). The first operator is diagonal on products of distinct \(Y_j\). A monomial's coefficient must vanish when its weight plus \(\ell\) lies outside \(Q\cup\{0\}\), where the weight is the sum of its indices modulo \(n\). For each degree-four monomial, all contributing terms add one previously absent index, and hence have the same nonzero coefficient. The verifier first solves those rows over \(\mathbb F_2\), then handles the lower-degree rows over \(E\), including repeated-index reductions. [pcc:unverified]

The blocks for \(\ell=0,1,-1\) give the three Frobenius orbit representatives. If their kernel dimensions are \(d_0,d_+,d_-\), the unrestricted binary dimension is \(d_0+k(d_++d_-)\). The two choices of \(g\) gave the same dimensions at each field degree. [pcc:unverified]

| Field degree | Block dimensions | Unrestricted dimension | Scalar-multiple dimension |
| ---: | ---: | ---: | ---: |
| 7 | 6, 6, 5 | 39 | — |
| 23 | 35, 33, 33 | 761 | 312 |
| 47 | 50, 50, 50 | 2,350 | 2,350 |
| 71 | 109, 111, 110 | 7,844 | 7,844 |
| 79 | 134, 137, 135 | 10,742 | 10,742 |

For \(k>3\), multiplying the four displayed \(\alpha_r\) by arbitrary Boolean scalars of degree at most \(3-r\) gives an admissible subspace of dimension \(\sum_{j=0}^3(4-j)\binom{k}{j}\). Indeed, the four \(\alpha_r(s)\) are independent at every nonzero sum by the argument above, which also holds at 71 and 79 bits, where the orders of 2 are 35 and 39. Any polynomial dependency would force each scalar to vanish off zero. The indicator of zero has degree \(k\), so the allowed scalar degrees exclude such a dependency. Equality of dimensions at 47, 71, and 79 bits proves this subspace is the entire cubic family. In particular, unrestricted coefficients still give only four pointwise directions. [pcc:unverified]

For each of the six subspaces at these sizes, the verifier also finds a rational target on \(y^2+xy=x^3+x^2+1\) and a nonzero sum that passes all four checks. Adding the actual normalized S3 right-hand side raises the image rank from \(k-1\) to \(k\). These false positives rule out sufficiency of the cubic projection even on the S3 target family, in the tested geometries. Higher-degree multipliers, additional target-dependent equations, and other formulations remain open. Rank coverage, the solving degree of a projected presentation, and the index-calculus exponent are separate quantities. [pcc:unverified]

## Compiled quadratic selector projection at 23 bits

For a fixed Frobenius geometry, write \(A\) for the packed row space of coefficient constraints that force \(\operatorname{Tr}(\alpha(s)(x^2+sx))=0\). For a target \(t\), let \(v_m(t)\) be the linear functional on the multiplier coefficients that returns the coefficient of \(s^m\) in \(\operatorname{Tr}(\alpha(s)p_t(s))\). A multiplier produces a selector equation of degree at most two precisely when it annihilates \(A\) and all the \(v_m(t)\) with \(|m|>2\). Compile the quotient by \(A\) once, then reduce by the high-degree functionals after specializing each target. Dualizing the images of the remaining functionals gives exactly the degree-at-most-two selector equations in the projected span. This builds the quadratic equations without forming or closing a system of quartic polynomials. The coefficient stage still uses the cubic and quartic supports in the selector variables. [pcc:unverified]

The comparison uses four shared targets, a planted target with two distinct solution sums, and eight seeded additional targets for each 23-bit factor. The unrestricted cubic multiplier space has dimension 761. The compiler uses 562 selector monomial coordinates in its coefficient stage, including the 495 high-degree coefficient functionals. It outputs 33 independent quadratic selector equations for every target. The other controls, unrestricted quadratic multipliers and equivariant cubic multipliers, give one quadratic selector equation each. Adding the controls' consequences through degree three leaves the original system's solving degree at four. [pcc:unverified]

| Presentation | Variables | First fall degree | Last fall degree | Solving degree | Columns at solving degree |
| --- | ---: | ---: | ---: | ---: | ---: |
| Direct normalized equations | 22 | 3 | 4 | 4 | 9,109 |
| Direct equations plus the 33 compiled quadratics | 22 | 3 | 3 | 3 | 1,794 |
| 33 compiled selector quadratics alone | 11 | 3 | 3 | 3 | 232 |

The direct and augmented systems have reduced basis degree zero on the 13 misses and degree two on the 13 hits. The selector-only system has reduced basis degree zero on the misses, degree one on the 11 singleton sums, and degree two on the two multiple-sum targets. For the online check, bounded closure either produces the constant 1 or yields an affine slice of at most eight assignments. Evaluation of the input quadratics validates the slice, then half trace recovers both orientations of every nonzero sum. The extraction code receives no root count or reference points, and the verifier compares its output with the independent rank-and-recovery reference. [pcc:unverified]

On this bounded-closure benchmark, the original two-coordinate extraction takes 15.30 s over the 26 targets. The compiled extraction takes 2.56 s, including 1.85 s of fixed setup across the two factors, target specialization, closure, and recovery of the original orientations. Both totals include normalization and exclude the independent reference computation. The complete toy comparison below also runs the compiled method. [pcc:unverified]

The degree reduction is for the specialized presentation at 23 bits. At 47 bits the unrestricted cubic multiplier span fails on rational targets even if all its projected equations are retained, so the quadratic subspace of that span also fails. [pcc:unverified]

## Quartic multipliers at 47 bits

For the two 47-bit subspaces, increasing the multiplier degree to four gives block dimensions 281, 281, and 283. The unrestricted dimension is 13,253, which equals \(\sum_{j=0}^4(5-j)\binom{23}{j}\), the scalar-multiple span of \(\alpha_0,\ldots,\alpha_4\). The fourth telescoping image adds \(u^4s\), so these five values are independent at every nonzero sum. The same span argument now gives pointwise rank five. Each subspace again has a rational target and a nonzero sum passing all five checks with inconsistent augmented rank. An exact projection using only target-independent trace multipliers of this form therefore requires multiplier degree at least five on these geometries. [pcc:unverified]

## Affine charts and mixed-coordinate elimination

Let a selector chart \(C\subset U\) have codimension \(r\). Extend its free Boolean coordinates by affine functions on \(U\). A degree-\(d\) multiplier on \(C\), multiplied by the indicator of \(C\), extends by zero to a global multiplier of degree at most \(d+r\). On a chart excluding zero, the unknown \(x\) may be restricted at each \(s\) to a complement of \(\langle s\rangle\), since \(x\mapsto x^2+sx\) has the same image on that complement. For a chart containing zero, the extension also requires the cancellation condition at zero. Thus a chart containing a published quartic false positive cannot reject it using only trace multipliers with \(d+r\le4\). [pcc:unverified]

The mixed-coordinate experiment uses the full Boolean equations in \(x,s\) at the two 47-bit witness targets. It first takes the degree-capped Macaulay rows and then optionally closes their span under multiplication by a single variable whenever the input row has degree below the cap. Both operations use the Boolean field equations to reduce powers. To test whether the row space separates a witness, the verifier keeps each monomial involving \(x\) as a separate column and evaluates only the pure selector terms at that witness. A row with zero \(x\)-part and nonzero evaluation separates the witness. The small built-in control has a witness separated only after closure. [pcc:unverified]

Degree-three closure completed in both 46-variable systems. Its selector-only subspace has dimension 25 and equals the span of \(f_2\), \(f_1\), and \(s_i f_1\), where \(f_j(s)=\operatorname{Tr}(\alpha_j(s)p(s))\) for \(j=1,2\). This equality was checked by taking the rank of the union of the computed and independently generated rows. Both witnesses survive. The degree-two trace multipliers already put the same 25 rows in the degree-four Macaulay selector span, whose computed dimension is 25. Hence the mixed-coordinate products give no additional selector rows there. The complete degree-four Macaulay spans leave both witnesses alive, including after setting \(s_0=1\) and \(x_0=0\) in their symmetry charts. Both degree-four closure pilots reached the time cap, so they leave the full degree-four closure question open. [pcc:unverified]

The closure verifier tracks the span of rows whose variable products have already been inserted. It initializes that span with all raw sources of nominal degree below the cap, since the raw matrix already contains their products. Before multiplying a low-degree pivot, it reduces the pivot by that span. Multiplying the remainder gives the same enlarged span, and a zero remainder requires no new products. On both degree-three witness runs the rank and the selector span agree with the repeated-product algorithm. The number of submitted rows drops from 8,602 to 6,486. In two capped degree-four runs the partial selector rank reached 272 and neither witness separated before the 360-second cap. Full degree-four closure remains open. [pcc:unverified]

## Quadratic field multiples

Write \(F=x^2+sx+p(s)\), where \(p\) is affine. Over Boolean coordinates, field squaring is linear. Consequently both \(sF=sx^2+s^2x+sp(s)\) and \(xF=x^3+sx^2+xp(s)\) are quadratic, using \(x^3=x\cdot x^2\). Their binary coordinates belong to \(V_3(F)\), since each is a linear combination of products of coordinates of \(F\) by variables. They preserve the solution set and can be generated directly as quadratics. If \(G\) is the system obtained by adjoining these coordinates, then \(V_D(G)=V_D(F)\) for \(D\ge3\) because every input row of \(G\) lies in \(V_D(F)\), which is already closed under the permitted products. The construction can still change \(V_2\). [pcc:unverified]

For the telescoping multiplier \(\alpha_r\) with \(r\ge2\), the term with \(j=0\) has a factor \(s\). Multiply \(sF\) by the remaining \(r-1\) Frobenius-linear factors. The other terms in \(\alpha_r\) have degree at most \(r-1\). This derives \(\operatorname{Tr}(\alpha_r p)\) in \(V_{r+1}(F)\) and saves one degree relative to the raw bound \(r+2\) for these consequences. The left-kernel coverage still depends on the multiplier family. [pcc:unverified]

On the same 28 seven-bit targets used for the quotient comparison, adjoining both multiples lowers the last fall degree and solving degree from three to two in all 28 cases. The first fall degree also drops from three to two. The mixed-coordinate column count at the solving degree drops from 42 to 22. With \(sF\) alone the solving degree drops on 21 targets. On the four 23-bit targets, the solving degree and last fall degree remain four, as implied by the \(V_D\) equality. This establishes a degree reduction on the seven-bit panel. A reduction at larger field sizes or in the decomposition exponent remains open. [pcc:unverified]

## Quotient coordinates

For nonzero \(s\), introduce \(w=p(s)/s^2\) in the trace-zero subspace of the coefficient field. Let \(H\) be the odd-degree half trace. The two constraints are

\[
s^2 w=p(s),\qquad sH(w)\in U.
\]

Both are bilinear in the two coordinate blocks apart from the affine right-hand side. They give an exact test for a nonzero sum. If \(x^2+sx=p(s)\), then \(w=(x/s)^2+x/s\), and \(sH(w)\) equals one of \(x,x+s\). Conversely \(H(w)^2+H(w)=w\), so the second constraint supplies a solution \(x=sH(w)\in U\) to the original equation. For a nonzero S3 target, \(p(0)\ne0\). Otherwise the original product at sum zero would be 1, and substituting into S3 would force the target abscissa to be zero. Thus the first quotient constraint excludes the zero sum. [pcc:unverified]

The degree comparisons use the same target abscissae within each pair. They omit targets admitting the diagonal \(s=0\) in the original presentation. The original presentation retains two orientations for each admissible sum and the quotient presentation gives one point per sum. A linear-image rank test determines the nonzero sums before computing the reduced basis from those points. The verifier measures closure membership of that basis. [pcc:unverified]

| Field degree | Targets | Original and quotient variables | First fall in both | Last fall in both | Solving degree in both | Columns at solving degree |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 7 | 28 | 6, 9 | 3 | 3 | 3 | 42, 130 |
| 23 | 4 | 22, 33 | 3 | 4 | 4 | 9,109, 46,938 |

At 23 bits the two planted targets have reduced basis degree two in the original presentation and one in the quotient presentation. The two refuted targets have basis degree zero in both. Degree-three closure of the quotient equations at the 47-bit witnesses still gives the same 25-dimensional selector space from the first two telescoping trace equations. Both witnesses survive. [pcc:unverified]

For both Frobenius subspaces at 23, 47, 71, and 79 bits, the verifier also constructs independent quotient values from pairs of distinct rational factor-base abscissae. Their span is the entire trace-zero subspace. A fixed linear subspace for \(w\) that covers all these rational pairs must therefore retain \(n-1\) coordinates. [pcc:unverified]

## Subspace resultant

Let \(L_U(X)=\sum_{j=0}^k c_j X^{2^j}\) be the monic subspace polynomial with roots \(U\). In these Frobenius geometries its coefficients are the binary coefficients of \(g\), so \(c_0=c_k=1\). For fixed \(s\in U\setminus\{0\}\), define \(P_s(Y)=\sum_{i=0}^{k-1}d_i(s)Y^{2^i}\) by

\[
d_{k-1}=1,\qquad d_{i-1}=c_i+d_i s^{2^i}\quad(1\le i<k).
\]

The recurrence matches every coefficient of \(P_s(X^2+sX)\) with \(L_U(X)\) except that of \(X\). Evaluating at the nonzero \(s\) matches the last coefficient. The map \(x\mapsto x^2+sx\) on \(U\) has two-element fibers, so its image has \(2^{k-1}\) distinct points. These are exactly the roots of \(P_s\). The resultant is therefore

\[
\operatorname{Res}_X(L_U(X),X^2+sX+p(s))=P_s(p(s))^2 .
\]

The unsquared value is a division-free consistency check. At \(s=0\) the correct test is \(L_U(p(0))=0\), because squaring is a permutation of \(U\). The pointwise recurrence rejects both published 47-bit quartic witness sums. [pcc:unverified]

For a fixed target, write \(p(S)\) as an affine linearized polynomial modulo \(L_U(S)\). In \(d_i\), each power of \(S\) has exponent \(\sum_{r=i+1}^{j-1}2^r\) for some \(j\le k\). Each \(p(S)^{2^i}\) reduces to at most \(k+1\) terms before multiplying. Only the terms with \(j=k\) can overrun \(2^k-1\), and their exponents are below \(2^k+2^{k-1}\). Replacing \(S^{2^k}\) by the lower terms of \(L_U\) therefore needs just one step per overrun. There are at most \(O(k^2)\) such overruns, and each contributes at most \(k\) terms. The other products contribute at most \(O(k^3)\) terms. Adding a scalar multiple of \(L_U(S)/S\), the indicator of zero on \(U\), sets the value at \(s=0\). This gives an exact polynomial with \(O(k^3)\) terms in the univariate power basis and ordinary degree below \(2^k\). [pcc:unverified]

The monomials \(S^e\) with \(0\le e<2^k\) form a basis for \(K\)-valued functions on \(U\), and those with \(\operatorname{popcount}(e)\le d\) form a basis of the degree-\(d\) Boolean filtration over \(K\). Each \(S^{2^i}\) is linear on \(U\), and the independent powers with the required popcounts have the same count as the squarefree Boolean monomials. The scalar-coordinate span over \(\mathbb F_2\) is measured separately after expanding in Boolean coordinates. [pcc:unverified]

| Field degree | Targets across both factors | Nonzero univariate terms per residual | Residual Boolean degree |
| ---: | ---: | ---: | ---: |
| 7 | 56 | 8 | 3 |
| 23 | 8 | 200 | 11 |
| 47 | 2 | 1,822 | 23 |
| 71 | 2 | 5,034 | 35 |
| 79 | 2 | 8,138 | 39 |

At 23 bits the ordinary degree is 2,047 and the Boolean expansion contains 2,047 or 2,048 nonzero monomials. The span of its binary coordinate equations is zero through degree eight and has dimensions 11, 22, and 23 in degrees nine, ten, and eleven. Those are input-row degrees. The direct, quotient, and quadratically augmented systems have measured solving degree four on the four shared first-factor targets. [pcc:unverified]

For 7 and 23 bits the independent rank-and-recovery check, pointwise recurrence, and dense polynomial GCD return the same solution sums. The dense GCD test records sparse construction, dense conversion, GCD, and isolation separately. It expands the polynomial to \(2^k\) coefficient slots at 23 bits. Scanning the recurrence also visits \(2^k\) sums. Both extraction methods have work that scales with \(2^k\). Sparse root isolation remains open. [pcc:unverified]

## Three-summand join by fixing the third point

Take signed factor-base points with abscissae in \(1+U\) and a target point \(R\) on the toy curve. For each allowed third point \(C\), compute \(W=R-C\) and solve the two-coordinate equation at \(x(W)\). The normalized unknown \(x\) is the first abscissa translated by 1. For every recovered nonzero selector sum, the half trace gives the two orientations of the same pair of abscissae. Look up their rational lifts and check \(P+Q=W\) for each possible signing. Record each unordered signed triple once per target with three distinct abscissae. An identity intermediate can only give a cancelling pair, which repeats an abscissa. [pcc:unverified]

All factor-base points lie in \(2E(K)\), so both solvers reject targets outside that subgroup up front. Membership for a finite point \(W=(t,y)\) is equivalent to \(\operatorname{Tr}(t)=1\). Doubling gives \(t=\lambda^2+\lambda+1\), hence the forward implication. For the converse, take a solution \(\lambda\) to this equation, let \(u^2=y+(\lambda+1)t\), and set \(v=(\lambda+u)u\). The curve equation at \(W\) gives \(u^4+tu^2+1=0\), so \(u\ne0\) and \((u,v)\) lies on the curve. Its double is \(W\). The two solvers share this gate and the intermediate-point calculations. The reference enumerates one signed pair point and computes its complement by group arithmetic with batched inversion. [pcc:unverified]

The image coefficients depend on the sum \(s\) alone and can be cached across targets. A fixed \(R\) and an intermediate abscissa \(t\) allow at most two different third points because \(W\) has at most two signs and \(C=R-W\) is unique for each. With \(B\) factor-base abscissae and all \(2B\) thirds allowed, an eligible target has between \(B\) and \(2B\) nonidentity intermediate abscissae. The cached recurrence must therefore visit \(\Theta(B2^k)\) selectors per target in this sliced join. The individual GCD arm constructs a sparse residual for each distinct intermediate abscissa, then materializes its dense coefficient array and isolates the common roots. [pcc:unverified]

The batch arm builds a balanced product tree of the residuals modulo \(L_U\). A single GCD of the root polynomial with \(L_U\) has exactly the union of the selector roots. To recover the intermediate labels, take the GCD of the surviving root polynomial with each child product, descending only into children with a nonconstant GCD. Because \(L_U\) is squarefree, the resulting leaf polynomials give the full root sets, including any root shared by multiple leaves. [pcc:unverified]

All four methods produced identical signed triple sets. The three resultant arms also agreed on the selector roots for every intermediate. At 7 bits the panel exhausts the finite targets and signed thirds for each subspace, and independently checks the 142 subgroup rejections by group arithmetic. At 23 bits it uses eight signed thirds per subspace and five targets, one planted and four shared with the two-coordinate comparison. Of the 80 intermediate abscissae, 47 give no selector root. Each GCD leaf constructs 200 nonzero terms and materializes 2,048 coefficient slots. The batch arm stores seven internal polynomials with 2,048 slots each per target. [pcc:unverified]

| Field degree | Targets across both factors | Verified signed triples | Direct group join | Cached recurrence | Individual dense GCD | Batch dense GCD |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 7 | 282 | 560 | 0.160 s | 0.254 s | 0.486 s | 0.415 s |
| 23 | 10 | 37 | 3.01 s | 9.62 s | 15.11 s | 3.92 s |

Totals include shared factor-base setup once per factor, intermediate construction, failed searches, and pair recovery. The scan total also includes its image cache. The supplied target and third-summand panels are fixed across arms. At 23 bits, direct arithmetic takes 0.081 s per verified triple and batched GCD takes 0.106 s. This counts triples with distinct abscissae. [pcc:unverified]

In a separate planted panel of eight intermediates per factor, the reduced 23-bit products become fully dense after combining four 200-term residuals. At 47 bits, each input has 1,822 terms and a reduced two-leaf product has 229,090 or 265,541 terms depending on the factor. The next schoolbook products would require over 52 billion coefficient pairs each. They were capped before multiplication. This measures the support of explicit product representations. A joint method that avoids forming the growing products remains open. [pcc:unverified]

## Relation rank and complete toy solves

The doubling subgroups at field degrees 7 and 23 have prime orders 71 and 4,196,903. To derive these orders, let \(a_0=2\), \(a_1=1\), and \(a_j=a_{j-1}-2a_{j-2}\), the Frobenius trace recurrence for this curve over \(\mathbb F_2\). The subgroup order is \((2^n+1-a_n)/2\). The verifier checks that a nonidentity generator has this order. On the subgroup, Frobenius acts by a scalar \(\mu\) satisfying \(\mu^2-\mu+2=0\) modulo the order. The verifier identifies the root by comparing the Frobenius image of the generator with its scalar multiple. [pcc:unverified]

Choose one signed representative per Frobenius orbit of factor-base abscissae. A point equal to the \(j\)-th Frobenius image of a representative contributes \(\mu^j\) in its column, or \(-\mu^j\) for the opposite sign. This reduces 7 columns to 1 at 7 bits and 943 columns to 41 at 23 bits. The collection target is a random known multiple of the generator. For every verified triple, accumulate its signed weights, reduce the row over the prime order, and retain a pivot only if it raises rank. After triangular back substitution, search for a triple of the challenge shifted by another known multiple of the generator. Subtract the known shift from the triple's computed logarithm. The verifier checks the resulting scalar and every recovered representative against point multiplication. [pcc:unverified]

The direct group join, the batch resultant join, and the compiled quadratic join use the same deterministic offsets, signed thirds, and challenge for each factor. At 23 bits they attain rank 41 after 18 and 16 collection attempts, with 59 and 52 verified collection triples, respectively. Three attempts across the two factors have no triples. Each solve then succeeds on its first descent attempt. The compiled join verifies every proposed selector with the exact pointwise resultant before sign recovery. It falls back to individual GCD extraction if bounded closure leaves more than eight affine candidates or times out. The measured runs have zero fallbacks and zero false positive selectors. The 7-bit direct and batch controls attain rank one and recover the challenge on each factor. [pcc:unverified]

| Field degree | Factor \(g\) encoded as an integer | Independent rows | Verified collection triples | Direct-join total | Batch-join total | Compiled-join total | Generic search |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 23 | 2,787 | 41 | 59 | 5.13 s | 6.51 s | 5.28 s | 0.32–0.33 s |
| 23 | 3,189 | 41 | 52 | 4.38 s | 5.68 s | 5.02 s | 0.25–0.28 s |

The join totals include factor-base and orbit setup, fixed compilation where used, target generation, unsuccessful searches, sign recovery, modular row reduction, back substitution, descent, and verification. The generic measurement is an independent baby-step giant-step search on the supplied challenge, including its baby-table build. The compiled join's search phase takes 4.01 s and 3.68 s, compared with 4.68 s and 3.96 s for the direct join. The fixed setup offsets that gain in the complete run on both factors. These measurements leave the asymptotic decomposition question open. [pcc:unverified]

## Sum–product linearization in a quadratic extension

Let \(K=\mathbb F_{q^2}\), \(q=2^k\), and take the factor-base abscissae in \(a+\mathbb F_q\) with \(a\notin\mathbb F_q\). For \(x_1=a+u_1\), \(x_2=a+u_2\), put \(s=u_1+u_2\) and \(p=u_1u_2\). The original product is \(z=a^2+as+p\). For a finite target abscissa \(t\) on an ordinary binary curve with coefficient \(b_E\ne0\), S3 is

\[
z^2+tz+t^2s^2+b_E=0.
\]

After substitution this equation is affine linear over \(\mathbb F_2\) in the \(2k\) binary coordinates of \(s,p\). The number of solutions is at most four, uniformly in the target. To see this, write \(a^2=Aa+B\) with \(A\ne0\) and \(t=b+ca\), with \(A,B,b,c\in\mathbb F_q\). The \(a\)-coordinate of the S3 equation has nonconstant part

\[
A(1+c^2)s^2+(b+Ac)s+cp.
\]

If \(c=0\), this gives at most two values of \(s\). For each value the constant-coordinate equation is monic quadratic in \(p\). If \(c\notin\{0,1\}\), solve the \(a\)-coordinate equation for \(p\) and substitute into the constant coordinate. The \(p^2\) term supplies a nonzero coefficient of \(s^4\), and the other terms have degree at most two. If \(c=1\), the substitution makes \(p\) affine linear in \(s\) with slope \(b+A\). The surviving coefficient of \(s^2\) is \(b^2+(b+A)^2=A^2\ne0\). These bounds hold for the affine system because the constant terms do not affect the stated leading coefficients. [pcc:unverified]

Solve the binary linear system, enumerate its at most four solutions, and recover \(u_1,u_2\) from \(U^2+sU+p=0\) in \(\mathbb F_q\). With \(s\) fixed this is another binary linear system. Rational lifting and a constant number of signed group checks give every verified pair. For a given field representation the per-target decomposition uses polynomial work in \(k\), replacing enumeration over \(2^k\) candidates. The sliced three-summand join uses \(B_0\operatorname{poly}(k)\) work with \(B_0\) abscissae in the rational factor base, by fixing each signed third point. The direct complement-lookup implementation uses \(\Theta(B_0^2)\) checks when all thirds and all signed left points are included. A fixed bound on the number of two-coordinate candidates is what changes the enumeration exponent in this comparison. [pcc:unverified]

Exhaustive controls over \(\mathbb F_{2^6}\) and \(\mathbb F_{2^{10}}\) agree with independent S3 pair enumeration for every field abscissa and with direct signed group sums for every represented point. The sampled controls at field degrees 14, 22, and 34 verify planted pairs and check that the candidate bound holds. The signed three-summand comparison includes 12 planted targets at each of field degrees 10, 14, and 22. The 10-bit run uses every signed third, and the other runs use a bounded panel of signed thirds. [pcc:unverified]

| Field degree | Verified triples in both arms | Direct pair checks | Linear parameter candidates |
| ---: | ---: | ---: | ---: |
| 10 | 286 | 23,232 | 1,296 |
| 14 | 145 | 23,316 | 384 |
| 22 | 218 | 431,664 | 432 |

### Obstruction in the prime-degree Frobenius bases

For the earlier prime-degree geometries, put \(W=\operatorname{span}_{\mathbb F_2}\{uv:u,v\in U\}\). Then \(W=\ker\operatorname{Tr}\). To see this, extend scalars to \(\mathbb F_{2^k}\). The tensor product with the coefficient field \(\mathbb F_{2^n}\) is a field because \(\gcd(n,k)=1\). Products of nonzero Frobenius eigenvectors are nonzero and add their weights modulo \(n\). The eigenweights of \(U\) are one of the two size-\(k\) doubling orbits \(Q\). The set \(Q+Q\) is stable under doubling and excludes zero because \(-Q\) is the other orbit. Cauchy–Davenport gives \(|Q+Q|\ge 2k-1>k\) for the geometries here, so the sumset includes both nonzero orbits. The product span is therefore \(n-1\) dimensional, and trace isotropy identifies it with the trace-zero space. The verifier reproduces these dimensions by direct binary rank in both factors at all five tested prime degrees. [pcc:unverified]

Replacing \(p=u_1u_2\) by \(p\in W\) leaves every sum \(s\in U\) alive at each nonzero trace-compatible rational target on the toy curve. Since the shift is 1, the original product \(z=1+s+p\) has trace one. The equation \(z^2+tz=t^2s^2+1\) has two roots because \(\operatorname{Tr}(s)=\operatorname{Tr}(1/t^2)=0\). They differ by \(t\), which has trace one. Exactly one root has trace one and yields \(p\in W\). The relaxed linear system thus has \(2^k\) solutions, one per sum, and the omitted product-membership condition still has to be enforced. This rules out the product-span relaxation as a bounded-candidate solver for these prime-degree geometries. [pcc:unverified]

The quadratic-extension result changes the factor-base geometry. It gives a decomposition bound for composite field degrees. In the coset controls the relative trace of \(a\) is 1 and \(k\) is odd, so every factor-base point has absolute trace-one abscissa. The finite doubles on this even-degree toy curve have trace-zero abscissa by the same doubling criterion. Since the doubling subgroup has index two, every three-term sum in the coset controls lies outside it. The native recovery experiment accounts for this constraint by shifting its targets. [pcc:unverified]

## Native Rust comparison and larger extension degrees

The four-candidate bound holds for any proper subfield \(F\subset K\) and shift \(a\in K\setminus F\). The nonconstant part of the equation is \(A s^2+D s+p^2+tp\), with \(A=a^2+t^2\) and \(D=ta\). If \(t\in F\), reduce modulo \(F\). Since \(A\notin F\), a scalar coordinate of the resulting equation gives at most two sums, and the equation in \(p\) gives at most two candidates for each sum. If \(t\notin F\), reduce modulo \(F+Ft\). A nonzero projected coefficient of \(s^2\) or \(s\) gives at most two sums. For each sum the map \(p\mapsto p^2+tp\) is injective on \(F\). [pcc:unverified]

If both projected coefficients vanish, write \(A=A_0+A_1t\) and \(D=D_0+D_1t\). The \(t\)-coordinate expresses \(p\) as an affine quadratic in \(s\). If \(A_1\ne0\), the other coordinate has leading term \(A_1^2s^4\), giving at most four sums. If \(A_1=0\), then \(a+t\in F\). The condition \(ta\in F+Ft\) now implies \(a^2\in F+Fa\), so \(a\) has degree two over \(F\). The preceding quadratic-extension calculation gives the bound in this last case. This extends polynomial-time S3 extraction and the one-pass sliced join to cubic extensions and other proper-subfield cosets. [pcc:unverified]

The native harness uses the pinned `Gf2` field kernel from `src/cryptanalysis/semaev_decomp.rs`. It builds three arms using identical field arithmetic: direct signed complement lookup with batched inversion, sum–product linearization with per-candidate recovery, and the same linearization with a fixed subfield Artin–Schreier map for recovery. The latter solves \((u/s)^2+u/s=p/s^2\) for nonzero \(s\). The harness checks all three signed triple sets on every trial. It also compares both S3 extractors with exhaustive algebraic pairs on every target at field degrees 6, 9, and 10. [pcc:unverified]

Each seed supplies 24 uniform finite curve targets and 24 planted three-summand targets per geometry. Every signed third is included. A shared modular eliminator tracks cumulative rank of signed relation coefficients modulo the largest prime factor of the toy curve order. It stops processing rows at full column rank. The full-rank prefix charges factor-base and map setup, target and intermediate generation, unsuccessful searches, sign recovery, and rank reduction. This measures relation coefficient rank rather than discrete-log recovery. [pcc:unverified]

| Uniform panel | Rank reached | First full-rank target | Direct time to full rank | Linearized time to full rank with cached recovery |
| --- | ---: | ---: | ---: | ---: |
| 22-bit quadratic, seed 11781661 | 1,058 / 1,058 | 5 | 0.817 s | 0.142 s |
| 22-bit quadratic, seed 12675811 | 1,058 / 1,058 | 6 | 0.972 s | 0.131 s |
| 18-bit cubic, either seed | 3 / 32 | — | — | — |

At 22 bits, each full uniform panel performs about 107 million checks in the direct complement join. The linearized join examines 48,046 and 56,680 sum–product candidates and achieves the same full-rank prefixes as the direct join. At 18 bits the quadratic coset reaches rank 254 of 254 on both uniform panels. The cubic coset has 32 factor-base abscissae and reaches rank 3 of 32 on both. This matched field-degree comparison shows how a cheaper decomposition can still lose relation yield when the factor base shrinks. The modular rank reduction uses prime 991 at 22 bits. [pcc:unverified]

The JSON field "full_rank_prefix" records the number of targets and the elapsed work for the direct, linearized, and cached linearized arms, in that order. Its value is null when the panel does not reach full rank.

### Native prime-subgroup recovery

Let \(G\) generate the largest odd prime-order subgroup of order \(r\) in the toy instance, and let \(T\) have maximal 2-power order. The relation targets are \(jG+T\), and the descent targets are \(Q+jG+T\). The point \(T\) places the targets in the three-summand coset and vanishes under projection to the odd subgroup. The coefficient matrix uses nonzero projected factor-base points up to sign as its columns. The experiment charges target generation, decomposition, elimination, descent, and point verification of the representative logs and recovered scalar. [pcc:unverified]

The Frobenius compression uses the scalar \(\mu\) satisfying \(\mu G=\tau G\), so signed orbit images contribute weights \(\pm\mu^i\). The split projection uses \(\sigma=\tau^k\) and chooses \(\epsilon\in\{1,-1\}\) by \(\sigma G=\epsilon G\). Then \(P+\epsilon\sigma P\) belongs to the chosen eigensubgroup of order \(N_\epsilon\). Multiplication by \(N_\epsilon/r\) maps it into the order-\(r\) subgroup. This replaces scalar multiplication by the full curve cofactor with one point addition and a smaller scalar multiplication. The projected generator is scaled by \(2N_\epsilon/r\), which is invertible modulo \(r\) in the measured groups. [pcc:unverified]

All four methods recover the same challenge scalars in both seeded runs at 14, 18, and 22 bits. The 22-bit curve has 1,058 factor-base abscissae. It gives 264 distinct nonzero projected columns up to sign and 24 columns after Frobenius compression. Each arm reaches full rank on its first collection target and descends on its first shifted challenge. [pcc:unverified]

| 22-bit seed | Direct group join | Linearized join | Linearized with Frobenius compression | Linearized with compression and split projection |
| ---: | ---: | ---: | ---: | ---: |
| 11781661 | 270.32 ms | 12.14 ms | 9.15 ms | 7.74 ms |
| 12675811 | 271.99 ms | 11.08 ms | 8.85 ms | 8.06 ms |

The times include setup and verification. The split projection reduces setup from approximately 1.9 ms to 0.8 ms within the compressed arm. An independent baby-step giant-step check takes about 7–11 microseconds on the supplied 991-order subgroup challenges, including its baby table. The result establishes a faster complete relation-based recovery on these toys. The largest subgroup and generic baseline remain relevant to any scaling claim for this curve family. [pcc:unverified]

## Solving-degree ceiling for the direct coset S3 baseline

The constant-size sum–product fiber also bounds the solving degree of the direct equations. Work in the Boolean quotient and use a degree-compatible monomial order. Write the input in the variables \(x=u_1\) and \(s=u_1+u_2\), with \(p=x^2+sx\). The binary coordinates of the direct input are the affine linear sum–product equations with this substitution, and they have degree at most two. If the linear sum–product equations are inconsistent, a binary combination of the input already gives 1. Otherwise row reduction eliminates \(p\) and gives affine equations in \(s\) alone. These equations are binary combinations of the direct input. Their affine slice of sums has dimension \(d\le2\), because it is a projection of an affine fiber with at most four points. [pcc:unverified]

Use \(d\) free Boolean coordinates for that affine slice. Its \(2^d\) point indicators \(\chi\) have degree at most two. For each sum \(s_\chi\), the row \(\chi F_j(x,s_\chi)\) can be derived by multiplying the original input by \(\chi\) and reducing the affine selector equations. This costs degree at most four. At the fixed sum, \(F_j(x,s_\chi)\) is affine linear in \(x\). For any polynomial \(g\) of degree at most three that vanishes on the solutions, \(g(x,s_\chi)\) belongs to this affine ideal, including when the fiber is empty. Linear substitution expresses it as a combination of the affine equations with product degrees at most three. After multiplying by the point indicator, these products have degree at most five. Adding them over the affine slice gives \(g\) modulo the affine selector equations, again with products of degree at most five. Thus \(g\) lies in the degree-five multiplication closure. [pcc:unverified]

There are at most eight oriented solutions, since every sum–product candidate gives at most two roots. The standard monomial set of their Boolean point ideal has size at most eight and is closed under divisibility. A squarefree minimal leading monomial of degree four or greater would require at least 15 distinct proper divisors in that set. Hence the reduced basis has degree at most three, including the possible square relations of degree two before passing to the Boolean quotient. The closure argument gives solving degree at most five for the direct coset S3 presentation. In particular, a tenfold reduction in solving degree from this baseline is impossible for a nontrivial satisfiable instance with positive solving degree. [pcc:unverified]

An exact membership check on 56 toy instances across seven quadratic and cubic geometries gives direct solving degree at most three. Every nonempty tested fiber has reduced basis degree at most two. The bound and the certificates concern solving degree. They are separate from the earlier measured exponent for pair extraction and the wall-clock gain in native recovery. [pcc:unverified]

## Higher-summand degree certificates

The native probe constructs the direct higher Semaev polynomials by iterating a homogeneous quadratic resultant. For fixed abscissa \(u\) and remaining variable \(X\), the quadratic in the intermediate \(T\) has coefficients \(A=u^2+X^2\), \(B=uX\), \(C=u^2X^2+1\). For a previous polynomial \(P(T)\) of prescribed homogeneous degree \(d\), set \(Z=AT\) and reduce \(A^dP(Z/A)\) modulo \(Z^2+BZ+AC\). If the remainder is \(UZ+V\), the new polynomial is

\[
  \frac{V^2+BUV+ACU^2}{A^d}.
\]

The division is exact in the polynomial ring before any evaluation. Keeping the prescribed degree on rows with zero leading coefficient accounts for roots at infinity in the next step. Controls compare the first resultant with S3 on every small coset pair and check reversed input order and signed group sums, including repeated abscissae. [pcc:unverified]

For each selected target, the verifier obtains the exact Boolean coefficients by a Möbius transform of the direct evaluation table. For every candidate degree \(D\), let \(H_D\) be the binary span of the coefficient words of degree at least \(D\). Any binary linear combination of input coordinates of degree below \(D\) is a functional annihilating \(H_D\). A point with nonzero evaluation in \(H_D\) satisfies every such combination. This certifies solving degree at least \(D\) under bounded multiplication closure, even allowing degree-based linear row preprocessing of the direct input. [pcc:unverified]

| Field degree | Summands | Boolean variables | Certified direct solving-degree lower bounds |
| ---: | ---: | ---: | ---: |
| 9 | 3 | 9 | 5–6 |
| 9 | 4 | 12 | 11–12 |
| 9 | 5 | 15 | 15 |
| 9 | 6 | 18 | 18 |
| 12 | 4 | 16 | 12 |
| 12 | 5 | 20 | 20 |

The measured solution counts are nonpowers of two, so an affine linear system in the original variables cannot describe these fibers. For the four 20-variable instances, three Boolean nonsolutions satisfy every quadratic that vanishes on the true points, one witness per target. The fourth instance instead has a cubic minimal leading term in degree reverse lexicographic order, certified by evaluating consecutive monomials on its 300 points. Its leading monomial has mask 917504. Therefore each 20-variable instance needs a basis of degree at least three in those variables and that order. The direct systems solve in degree at most \(w+1\) for \(w\) Boolean variables because this cap permits every variable product of every multilinear row. Combining these bounds rules out a tenfold drop for a single exact solution-ideal basis on these panels in the original variables and fixed order. [pcc:unverified]

### Charted resultant with a quadratic basis

An auxiliary-variable formulation reaches degree two on all four of the certified degree-20 baselines. It constructs an exact Boolean graph over the original 20 variables. The construction and the complete solution reconstruction are included in the degree accounting. The original-coordinate basis obstruction above still applies if one eliminates the auxiliary variables symbolically. [pcc:unverified]

For a five-summand target, lift the 16 factor coset abscissae to the curve over the quadratic extension of the coefficient field. Each lift exists there because its defining Artin–Schreier equation has coefficients in the original field. Enumerate the signed sums of unordered three-abscissa prefixes and subtract each signed sum from a fixed lift of the target. For a finite residual apply the sum–product S3 linearization and recover the two roots in the base coset. All field products, inversions, and point operations on this branch use scalar values. The binary equations for extracting the remaining pair on a fixed branch are affine linear. Verify the signs at the residual and restore the prefix permutations. An identity residual contributes the entire affine chart with equal suffix abscissae. This also handles cancellation through infinity. [pcc:unverified]

The resulting charts are disjoint. An unordered suffix pair with distinct abscissae gives an affine line with one orientation bit. A suffix pair with equal abscissae gives a point, unless the identity-residual chart already contains it. Parallel affine charts can be merged without splitting them into point lists: the disjoint union of \(b+V\) and \(c+V\) is \(b+(V+\langle b+c\rangle)\). The verifier normalizes the bases and repeats these mergers. [pcc:unverified]

Use a selector \(z_i\) for every chart except chart zero, and one variable per local axis. Chart zero is active when all explicit selectors are zero. Emit zero products between variables owned by different charts, the within-chart gates \(z_i r_{ij}+r_{ij}\), and all auxiliary Boolean square equations. Add one affine equation per original coordinate expressing it as a function of the selectors and local axes. In a degree-compatible order with the original variables ahead of the auxiliaries, the leading terms are the original-coordinate variables, the forbidden and gated products, and the auxiliary squares. The surviving squarefree monomials consist of the empty monomial, each explicit selector, and nonempty products within one chart's axes. Their count is \(\sum_i 2^{\dim V_i}\), the exact number of graph points. The leading terms therefore generate the initial ideal, and the emitted degree-two rows form a Gröbner basis. Reconstruction evaluates the linear coordinate graph on each chart. [pcc:unverified]

| Target | Reconstructed points | Charts after merging | Auxiliary variables | Sparse basis rows | Direct lower bound → charted degree |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 13 | 1,460 | 283 | 910 | 414,096 | ≥20 → 2 |
| 24 | 300 | 81 | 227 | 25,829 | ≥20 → 2 |
| 34 | 350 | 91 | 259 | 33,606 | ≥20 → 2 |
| 82 | 1,236 | 263 | 815 | 332,192 | ≥20 → 2 |

The independent direct-resultant computation confirms all four complete solution sets over the original variables. Each candidate run visits 5,984 distinct signed-prefix sums counted within their prefix groups and solves 5,093–5,232 distinct intermediate systems. A separate parser verifies the emitted leading terms, their standard-monomial counts, and the sparse row counts. With a disk export, candidate construction and reconstruction take 5–28 ms on this machine, excluding the independent reference and parser. The tenfold bound is a solving-degree result for the lifted presentation. Its cost includes the signed-prefix search and tens to hundreds of thousands of sparse basis rows. [pcc:unverified]

## Reproduction

Run from the repository root with Python and SymPy installed.

```sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/profile.py 7 23 47
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/profile.py 23 --degree 2
c++ -std=c++20 -O3 research/frobenius_s3_cubic_projection_20261123/extension_rank.cpp -o /tmp/s3-extension-rank
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/unrestricted.py 7 23 47 71 79 --ranker /tmp/s3-extension-rank --verify
```

The following build uses the carryless-multiply instruction available on supported x86 CPUs for the larger quartic rank calculation.

```sh
c++ -std=c++20 -O3 -mpclmul research/frobenius_s3_cubic_projection_20261123/extension_rank.cpp -o /tmp/s3-extension-rank-fast
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/unrestricted.py 47 --ranker /tmp/s3-extension-rank-fast --degree 4
```

Reproduce the witness checks as follows. The degree-four runs need time to insert all their Macaulay rows. The `--closure` switch includes the bounded multiplication closure.

```sh
c++ -std=c++20 -O3 research/frobenius_s3_cubic_projection_20261123/selector_elimination.cpp -o /tmp/s3-selector-elimination
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/selector_elimination.py --ranker /tmp/s3-selector-elimination --degree 3 --closure
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/selector_elimination.py --ranker /tmp/s3-selector-elimination --degree 4
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/selector_elimination.py --ranker /tmp/s3-selector-elimination --degree 4 --chart
```

The same ranker measures quotient-coordinate closure. Use a C++20 compiler with 128-bit integer support for the 69-variable witness checks.

```sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/quotient.py degrees --ranker /tmp/s3-selector-elimination
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/quotient.py witnesses --ranker /tmp/s3-selector-elimination
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/quotient.py spans
```

The same build also runs the quadratic augmentation comparison. The degree-four closure command reproduces the capped experiment, whose partial ranks depend on hardware speed.

```sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/quadratic_multiples.py --ranker /tmp/s3-selector-elimination
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/selector_elimination.py --ranker /tmp/s3-selector-elimination --degree 4 --closure --limit 360
```

The quadratic comparison and capped closure records are in [quadratic-multiples.jsonl](quadratic-multiples.jsonl) and [selector-closure-d4-partial.jsonl](selector-closure-d4-partial.jsonl).

The quotient results are in [quotient-degrees.jsonl](quotient-degrees.jsonl), [quotient-witnesses.jsonl](quotient-witnesses.jsonl), and [quotient-spans.jsonl](quotient-spans.jsonl).

Use the same ranker to replay the coefficient compilation and degree comparison.

~~~sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/compiled_projection.py --ranker /tmp/s3-selector-elimination
~~~

The specialization and recovery records are in [compiled-projection.jsonl](compiled-projection.jsonl).

The elimination records are in [selector-closure-d3.jsonl](selector-closure-d3.jsonl), [selector-macaulay-d4.jsonl](selector-macaulay-d4.jsonl), and [selector-chart-d4.jsonl](selector-chart-d4.jsonl).

Reproduce the sparse resultant with Python and SymPy. Add `--dense-gcd` with python-flint installed to check exact root isolation through 23 bits.

```sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/subspace_resultant.py 7 23 47 71 79 --dense-gcd
```

The resultant results are in [subspace-resultant.jsonl](subspace-resultant.jsonl).

Run the three-summand comparison with the same dependencies.

~~~sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/three_summand_join.py 7 23 --dense-gcd --batch-gcd
~~~

The join results are in [three-summand-join.jsonl](three-summand-join.jsonl).

Measure the sparse product support with a standard C++20 build. A build using the optional carryless-multiply instruction can be cross-checked coefficient by coefficient with the portable build.

~~~sh
c++ -O3 -std=c++20 research/frobenius_s3_cubic_projection_20261123/sparse_product.cpp -o /tmp/s3-sparse-portable
c++ -O3 -std=c++20 -mpclmul research/frobenius_s3_cubic_projection_20261123/sparse_product.cpp -o /tmp/s3-sparse-product
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/batch_support.py 23 47 --multiplier /tmp/s3-sparse-product --control /tmp/s3-sparse-portable
~~~

The support results are in [batch-product-support.jsonl](batch-product-support.jsonl).

Replay the end-to-end comparison. The batch comparison also needs python-flint.

~~~sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/toy_solve.py 7 23 --method direct
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/toy_solve.py 7 23 --method batch
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/toy_solve.py 23 --method compiled --ranker /tmp/s3-selector-elimination
~~~

The rank and recovery records are in [toy-recovery.jsonl](toy-recovery.jsonl).

Replay the sum–product checks with Python and SymPy.

~~~sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/product_linearization.py
~~~

The coset and prime-degree results are in [product-linearization.jsonl](product-linearization.jsonl).

Run the native comparison with Rust and Python available. The shell runner extracts the pinned field kernel into a temporary build directory.

~~~sh
bash research/frobenius_s3_cubic_projection_20261123/run_native_coset.sh 24 11781661
bash research/frobenius_s3_cubic_projection_20261123/run_native_coset.sh 24 12675811
bash research/frobenius_s3_cubic_projection_20261123/run_native_coset.sh recovery
~~~

The native records are in [native-coset.jsonl](native-coset.jsonl) and [native-recovery.jsonl](native-recovery.jsonl).

Replay the coset solving-degree baselines with the C++ ranker built above.

~~~sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/coset_degree.py --ranker /tmp/s3-selector-elimination
~~~

The solving-degree records are in [coset-degrees.jsonl](coset-degrees.jsonl).

Replay the native higher-summand certificates.

~~~sh
bash research/frobenius_s3_cubic_projection_20261123/run_native_coset.sh higher-degree
~~~

The witness records are in [higher-degree.jsonl](higher-degree.jsonl).

Run the charted resultant with a directory for the emitted bases, then verify their leading terms.

~~~sh
mkdir -p /tmp/s3-charted-bases
bash research/frobenius_s3_cubic_projection_20261123/run_native_coset.sh charted /tmp/s3-charted-bases > /tmp/s3-charted.jsonl
python3 research/frobenius_s3_cubic_projection_20261123/verify_chart_export.py /tmp/s3-charted.jsonl /tmp/s3-charted-bases
~~~

The chart construction, export, and reconstruction records are in [charted-resultant.jsonl](charted-resultant.jsonl).

The multiplier records are in [cubic-ranks.jsonl](cubic-ranks.jsonl), [quadratic-ranks.jsonl](quadratic-ranks.jsonl), [unrestricted-ranks.jsonl](unrestricted-ranks.jsonl), and [unrestricted-quartic-ranks.jsonl](unrestricted-quartic-ranks.jsonl). The unrestricted verifier checks direct binary-coordinate ranks at 7 and 23 bits, and compares logarithm-table arithmetic against polynomial arithmetic for the extension blocks through 47 bits. The last-fall conventions and references used for the broader experiments are recorded in the [LFD research note](../lfd_lower_bound_20261111/NOTE.md), including the relationship between last fall degree, reduced basis degree, and solving degree under a degree-compatible term order. [Caminata and Gorla](https://arxiv.org/abs/2112.05579)
