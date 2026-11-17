# Horizontal transport of a marked transverse norm

Derived, 2026-11-08, T-1942. The formulas below are written algebra; native qualification is separately bounded by scope.json. Scientific standing is unverified. [pcc:unverified]

Let E,E'/F_p have prime rational group order r, with p>3 and r odd and distinct from p. Let f:E->E' be a separable F_p-isogeny of degree d coprime to r. Set G'=f(G), Q'=f(Q), Q=[k]G with k not 0,+1,-1. Write x(f(A))=F(x(A)), where F=A_0/D_0 is reduced. All equalities are first rational-function identities on E x E. Maps in the implementation are normalized before any explicitly recorded closure.

Define psi=1-pi on E and psi'=1-pi' on E'. Both have degree r, kernel the rational group, and differential 1. Since f is defined over F_p, f psi=psi' f. The group map f restricts to a bijection on rational r-torsion: its kernel has order coprime to r. Consequently the Round60 cross-norm identity applies on both curves:

N(P,R)=c*(x(psi P)-x(psi R));
N'(fP,fR)=c'*(x(psi' fP)-x(psi' fR)).

Here c=product_(j=1..r-1)(x(jG)-x(jQ)) and c' is the corresponding destination constant. Their construction is part of setup, unless a different explicit evaluator is provided. This proof does not assert that they are independent of k. [pcc:unverified]

Divided-difference transfer. Put

h(X,Y)=(F(X)-F(Y))/(X-Y), with h(X,X)=F'(X).

a_j=x(P+jG), b_j=x(R+jQ), d_j=a_j-b_j, and h_j=h(a_j,b_j). Then destination factors satisfy d'_j=h_j*d_j. Hence, on any denominator-safe chart,

H(P,R)=product_j h_j=(c'/c)*h(x(psi P),x(psi R)).

The right side follows by dividing the two cross-norm identities and using f psi=psi' f. Equality extends as a rational function through zeros of N; one need not divide by the numerical zero at a marked specialization. This compresses this multiplier, conditional on c'/c, to the isogeny circuit and two known Frobenius quotient points. It does not compute the transverse coefficients themselves. [pcc:unverified]

Exact weighted jet transfer. Work over K[s,t]/(s^2,st,t^2). On a chart with all h_j units,

product_j(d'_j+s+t*F(a_j))
 = H * product_j(d_j+s/h_j+t*F(a_j)/h_j).

This is a factor-by-factor identity. If the weighted product has coefficients (A_w,B_w,C_w), the destination coefficients are H*(A_w,B_w,C_w). The source unweighted pair B,C is insufficient to replace the two nonuniform weights at a generic input. The native implementation contracts all r factors for both sides; it makes no compressed weighted-norm assumption. [pcc:unverified]

Marked divisor. Choose P with 2f(P) nonrational and U in E(F_p), and put R=P+U. This condition implies 2P is nonrational. Every translated input and its f-image is finite, since an infinite f-image would make f(P) rational. In each graph there is exactly one zero factor, at j such that [j](G-Q)=U. Minus equality on the destination would make 2f(P) rational; plus equality differs by a rational r-torsion point and cannot enter the prime-to-r kernel of f. Thus both graphs have the same unique vanishing index. Also y(P+jG) and y(f(P+jG)) are nonzero. For a normalized f the relation y(f(A))=y(A)*F'(x(A)) shows h_j=F'(a_j) is nonzero at that index. All other h_j are nonzero because both d_j and d'_j are nonzero. [pcc:unverified]

Let the unweighted source coefficients be A=0,B,C and the destination ones be A'=0,B',C'. The unique-zero argument gives

x_* = C/B = x(P+[j]G),
H = (c'/c)*F'(x(psi P)),
B' = H*B/F'(x_*),
C' = B'*F(x_*).

In particular C'/B'=F(C/B). This proves why omitting the derivative factor generally changes B'. The source/destination two-choice lifts subtract P/f(P), respectively, so the decoded points satisfy V'=f(V), with V=[(1-k)^-1]U. This is signed point transport; the offset x_* is not the x-coordinate of V itself. [pcc:unverified]

Projective encoding. If A_0^h,D_0^h are homogenized to degree d, the marked ratio [C:B] maps to [A_0^h(C,B):D_0^h(C,B)]. Ordinary rational-map composition evaluates this without expanding the composite degree. The absolute B' still requires H and F'(C/B); obtaining C/B initially is precisely the source marked-action dependency. Thus this transfer is a small circuit conditional on an already represented source jet. It is not a seed for that jet. Denominators, derivative nonvanishing and the nonrational-image guard above are part of the interface. [pcc:unverified]

A bidirectional oracle transfer without kernel-branch enumeration. If a marked-action evaluator is available on E', transfer G,Q,U through f, obtain V'=M_(Q')^(G')(f(U)), and return

V=[d^(-1) mod r] * dual(f)(V').

Indeed dual(f) f=[d], and V is rational r-torsion. This needs an explicit dual and scalar multiplication by a known d-inverse. No choice among d geometric preimages is required. The reduction works in the other direction by swapping the maps. For a chain use its product degree and reverse closed dual chain; actual construction and all map evaluations remain charged. An endpoint with a cheaper target-specific norm representation can therefore be used through this interface, but an isogenous endpoint alone supplies no such representation. [pcc:unverified]

Chain costs. A supplied chain of rational maps of degrees ell_i can transport a represented point, ratio, derivative product and multiplier in work proportional to its evaluator cost, for dense Horner O(sum ell_i) field operations, plus normalization constants. Fully expanding its kernel fibers instead produces product ell_i branches in that literal expansion. These are different representations, not a lower bound on arbitrary norm circuits. The actual Round25 relation has 53 edges and degree a 292-bit integer; its prime degrees sum to 1,151,567. Only the 47-edge prefix and its closed duals are supplied here. The native P192 checker replays those maps, compares dense and compact representations, then retracts with the inverse of the 205-bit prefix degree modulo r. Its six remaining global relation maps are not inferred from a subgroup correcting scalar. [pcc:unverified]

Why closure does not yet determine the mark. For a closed CM map alpha=a+b*pi, the rational subgroup scalar is lambda=a+b mod r. Naturality gives alpha M_Q(U)=M_Q(alpha U), after consistent generator transport, which is already the linearity of [(1-k)^-1]. The projective jet rule transports an existing hidden coordinate through alpha; neither this equation nor the compact H formula gives a starting C/B. A useful next circuit must provide graph-dependent weighted first coefficients from public G,Q and U with a smaller charged constructor. The present result provides exact transport guards and equations for testing such a circuit, rather than dropping its mark during CM reduction. [pcc:unverified]

Sources checked. Round60 cross-norm-proof.md gives the coupled norm and unique-zero lift; Round25/27/28 supply the actual P192 CM relation and map certificates. Bernstein-De Feo-Leroux-Smith, arxiv:2003.10118 section4.15, treats isogeny-composed coordinates and does not automatically supply this coupled weighted deformation. Banegas-Gilchrist-Le Devehát-Smith, arxiv:2306.16072 sections2-3 and5.2, supplies normalized-map and Frobenius-kernel evaluation background. Robert HDR section4.7.3 distinguishes compact isogeny description from its reconstruction. The earlier local marked-variation result is rechecked in the live Round60 sign-ambiguity proof: ordinary point derivatives do not determine graph marks. KB tools and the wrapper are unavailable. No novelty/absence or universal complexity lower-bound claim is made. [pcc:unverified]
