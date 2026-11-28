# Six polar equations for the regular RR5 Graeffe chart

Date: 2026-11-24 UTC. Scope: the original three-parameter RR5 chart over
M31, with four distinct original abscissae in the admitted depth-ten
Dickson fiber. The principal result is a written finite-field argument:
the six total second derivatives of the third centered elementary
residual vanish at every admitted solution and have Jacobian rank three
there. Retaining the original equations therefore removes their local
multiplicity without discarding any regular solution. The argument is
unsigned and has no Lean-kernel or signed independent standing.
[pcc:unverified]

This is a specialization of differential deflation, an established
technique. The result below is derived directly in positive
characteristic; the characteristic-zero literature is context, not its
proof. No extraction result, solving-degree bound, or speed improvement is
asserted. Fixed native controls and their execution status belong to the
[protocol](PROTOCOL.md) and report.

## 1. Assumptions and the exact equations

Let \(K=\mathbb F_p\), \(p>3\), and let
\[
E:y^2=f(x)=x^3+Ax+B
\]
be nonsingular. The M31 fixture uses
\[
p=2147483647,\qquad A=847514846,\qquad B=2087444724.
\]
Its full rational group has recorded odd prime order \(2147446831\);
the existing curve/base admission checks remain required. The local
linear-algebra theorem below needs nonzero leaf ordinates rather than a
group-order assumption, but odd full curve order provides this condition
for the admitted split selector.

Fix a finite nonidentity target \(R=(r,s)\). Write
\[
\theta=(b,c,d),\quad V=X+b,\quad L=cX+d,\quad
D_R=X^2+rX+r^2+A,
\]
\[
\ell=(X-r)L+(s+y)V,\qquad
U=D_RV^2-(X-r)L^2-2sLV.
\]
Then \(U=X^4+u_3X^3+u_2X^2+u_1X+u_0\) is monic and
\[
fV^2-\big((X-r)L+sV\big)^2=(X-r)U.                 \tag{1}
\]
The coefficients, fixing the exact parameter convention, are
\[
\begin{aligned}
u_3&=2b+r-c^2,\\
u_2&=b^2+2rb+r^2+A+rc^2-2cd-2sc,\\
u_1&=rb^2+2b(r^2+A)+2rcd-d^2-2s(cb+d),\\
u_0&=(r^2+A)b^2+rd^2-2sbd.
\end{aligned}                                                   \tag{2}
\]
These are the same norm and chart as the
[RR5 note](../prime31_four_summand_20261122/MATHEMATICS.md) and its
[structured-selector continuation](../prime31_four_summand_20261122/scaling-b677/MATHEMATICS.md).

For this study put
\[
D_1(X)=X,\qquad D_{2m}(X)=D_m(X)^2-2,\qquad
\phi(X)=D_{1024}(X)-\tau.
\]
Admission requires that \(\phi\) have 1024 distinct roots in \(K\),
that \(\tau\ne\pm2\), and that \(\phi(r)\ne0\). Its curve-liftable
roots, with both signs, are the admitted pure Dickson base. This study
does not replace the frozen B677 union selector or install a new B677
extraction path.

Let
\[
\Delta(\theta)=\operatorname{disc}_X U_\theta.
\]
The **regular chart is \(\Delta\ne0\)**. Over an algebraic closure,
write the four roots of \(U\) as \(x_1,\ldots,x_4\), and put
\[
z_i=\phi(x_i),\qquad F_j(\theta)=e_j(z_1,z_2,z_3,z_4)
\quad(1\le j\le4).                                      \tag{3}
\]
Although individual root functions require local choices, each \(F_j\)
is a polynomial in \(b,c,d\): it is symmetric in the roots of the monic
quartic. Equivalently,
\[
\det\big(TI-\phi(M_U)\big)
 =T^4-F_1T^3+F_2T^2-F_3T+F_4,                           \tag{4}
\]
where \(M_U\) is multiplication by \(X\) in \(K[X]/(U)\).

At a regular point,
\[
F_1=F_2=F_3=F_4=0
\quad\Longleftrightarrow\quad
z_1=z_2=z_3=z_4=0
\quad\Longleftrightarrow\quad U\mid\phi.                 \tag{5}
\]
The first equivalence follows from the polynomial in (4) being \(T^4\);
the second uses the distinct original roots of \(U\).

Repeated original roots require an explicit guard. Without
\(\Delta\ne0\), the characteristic polynomial can be \(T^4\) while
\(\phi(M_U)\) is a nonzero nilpotent matrix, so (5) need not hold.
Neither the root-coordinate proof nor the deflation conclusion below is
a claim about that stratum. A solver using the ideal equality below must
localize or saturate by \(\Delta\), or provide an exactly equivalent
regular-chart construction.

No distinctness condition is imposed on the intermediate Dickson
images. The quartics representing those images may have repeated roots,
including complete terminal collapse to \(\tau\). Only the original
four \(x_i\) must be distinct.

## 2. Graeffe coefficient map and the centered third residual

Define
\[
Q_j(T)=\prod_{i=1}^4\big(T-D_{2^j}(x_i)\big),\qquad Q_0=U.
\]
For a monic quartic
\[
Q(T)=T^4+a_3T^3+a_2T^2+a_1T+a_0
\]
write
\[
\begin{aligned}
k_3&=2a_2-a_3^2,\\
k_2&=a_2^2+2a_0-2a_3a_1,\\
k_1&=2a_2a_0-a_1^2,\qquad k_0=a_0^2.
\end{aligned}
\]
The quartic whose roots are \(x_i^2-2\) has coefficients
\[
\begin{aligned}
a'_3&=k_3+8,\\
a'_2&=k_2+6k_3+24,\\
a'_1&=k_1+4k_2+12k_3+32,\\
a'_0&=k_0+2k_1+4k_2+8k_3+16.                           \tag{6}
\end{aligned}
\]
Indeed, with \(v=T+2\), this quartic is
\[
(v^2+a_2v+a_0)^2-v(a_3v+a_1)^2.
\]
Thus (6) is a polynomial graph map, valid over arbitrary coefficient
algebras, with no choice of square root or unknown-dependent division.
Iterating it ten times gives
\[
Q_{10}(T)=T^4+g_3T^3+g_2T^2+g_1T+g_0.
\]

Centering is essential:
\[
Q_{10}(T+\tau)=\prod_i(T-z_i)
              =T^4-F_1T^3+F_2T^2-F_3T+F_4.
\]
Consequently,
\[
\begin{aligned}
F_1&=-g_3-4\tau,\\
F_2&=g_2+3\tau g_3+6\tau^2,\\
\boxed{F_3}&=\boxed{-g_1-2\tau g_2-3\tau^2g_3-4\tau^3},\\
F_4&=g_0+\tau g_1+\tau^2g_2+\tau^3g_3+\tau^4.
\end{aligned}                                                   \tag{7}
\]
The uncentered coefficient \(g_1\), even with its terminal constant
subtracted, does not have the order-three vanishing property used below.

All derivatives in this note are **total derivatives in the original
parameters \(b,c,d\)**. In particular, the \(g_i\) in (7) are the
composed polynomial functions obtained from (2) and all ten instances of
(6). Holding intermediate or terminal graph variables independent and
differentiating (7) is a different operation. The ambient Hessian of its
affine expression for \(F_3\) is identically zero; it supplies no
deflation. Likewise, the ambient derivative of \(F_2\) with respect to
\(g_2\) equals one, so constraining that derivative to zero would delete
every solution.

## 3. The root tangent space and its nonzero normal

Fix a geometric regular solution \(\theta_0\). Squarefreeness of
\(\phi\) and (5) imply that \(U\) is a squarefree divisor of \(\phi\).
The denominator argument from the structured-selector note is short
enough to retain here:
\[
U(-b)=(b+r)(d-bc)^2.
\]
If \(U(-b)=0\), then either \(b=-r\), making \(r\) a root of \(U\)
despite \(\phi(r)\ne0\), or \(d=bc\). In the latter case \(L=cV\) and
\[
U=V^2\big(D_R-(X-r)c^2-2sc\big),
\]
contradicting squarefreeness. Hence \(V(x_i)\ne0\), and the signed
ordinates
\[
y_i=-s-\frac{(x_i-r)L(x_i)}{V(x_i)}                    \tag{8}
\]
are well defined. Equation (1) gives \(y_i^2=f(x_i)\).

No \(y_i\) is zero under the admitted split-selector and odd-full-group
conditions: such an \(x_i\in K\) would give rational nonidentity
two-torsion. The divisor of \(\ell\) is
\[
\sum_{i=1}^4(P_i)+(-R)-5(O),\qquad P_i=(x_i,y_i),
\]
so \(\sum_iP_i=R\). This identity holds in a formal neighborhood with
these four simple local roots, nonzero denominators, and fixed target.

### Rank of the parameter-to-root differential

Let a parameter tangent satisfy \(\delta U=0\). Differentiating (1) at
each fixed leaf gives
\[
0=2y_iV(x_i)\,\delta\ell(P_i).
\]
Thus \(\delta\ell\) vanishes at the four leaves and at \(-R\). Since the
normalized coefficient of \(xy\) cannot vary, \(\delta\ell\in L(4O)\).
Its five distinct finite zeros force \(\delta\ell=0\). Comparing the
\(y,x^2,x\) coefficients then gives
\(\delta b=\delta c=\delta d=0\). Therefore
\(\theta\mapsto(u_0,u_1,u_2,u_3)\) has differential of rank three here.

The coefficient-to-ordered-root differential is invertible when
\(\Delta\ne0\). Multiplication of each root differential by
\(\phi'(x_i)\ne0\) is also invertible. Consequently
\[
\mathcal L=d(z_1,z_2,z_3,z_4)_{\theta_0}: \overline K^3
 \longrightarrow \overline K^4
\]
has rank three. For the Dickson selector one can also see nonvanishing
directly from
\[
D'_{1024}(x)=2^{10}\prod_{j=0}^{9}D_{2^j}(x).
\]
Any zero factor would force the terminal value to be \(2\) or \(-2\),
which admission excludes.

### The Abel differential

The invariant differential on \(E\) is \(dx/(2y)\). Differentiating the
fixed sum gives
\[
\sum_{i=1}^4\frac{dx_i}{2y_i}=0.
\]
Since \(dz_i=\phi'(x_i)\,dx_i\), define
\[
n_i=\frac{1}{2y_i\phi'(x_i)}\ne0,\qquad
W=\operatorname{im}\mathcal L.
\]
Then
\[
\boxed{W=n^\perp,\qquad n=(n_1,n_2,n_3,n_4),\quad
       n_i\ne0\text{ for every }i.}                    \tag{9}
\]
The pairing is the ordinary nondegenerate bilinear dot product over
\(\overline K\); it has no positivity property. The scalar choice of
normal is immaterial. Formula (9) is for the proof and independent
diagnostics: the proposed equations do not need to know any solution
root, signed ordinate, or normal in advance.

## 4. Six equations give full rank at every regular solution

Let \(\theta_1=b,\theta_2=c,\theta_3=d\), and append
\[
H_{ab}(\theta)=
\frac{\partial^2F_3}{\partial\theta_a\partial\theta_b}(\theta)=0,
\qquad 1\le a\le b\le3.                               \tag{10}
\]
These are six equations, retaining all four \(F_j=0\).

**Theorem.** At every geometric regular solution admitted above, all six
\(H_{ab}\) vanish, and the \(6\times3\) Jacobian of the \(H_{ab}\)
has rank three. In particular, the augmented system has a reduced
isolated point there. [pcc:unverified]

**Proof of vanishing.** Label the simple roots in the completed local
ring at \(\theta_0\). Put \(t=\theta-\theta_0\). Since \(z(\theta_0)=0\),
\[
z(\theta_0+t)=\mathcal L t+O(t^2),\qquad
F_3(\theta_0+t)=C(t)+O(t^4),\quad C(t)=e_3(\mathcal L t).
                                                               \tag{11}
\]
Thus \(F_3\) vanishes to order at least three, proving (10).
The linear parts of its six Hessian entries are exactly the six
second derivatives of the cubic \(C\).

**Proof of rank.** Suppose a parameter direction \(v\in\overline K^3\)
is killed by all six of these linear parts. Set \(w=\mathcal L v\in W\).
All second derivatives of the quadratic \(\partial_v C\) vanish. Since
two is invertible and this polynomial is homogeneous of degree two,
\(\partial_v C=0\). Equivalently, the homogeneous quadratic
\[
Q(z)=\partial_w e_3(z)
\]
vanishes identically on the hyperplane \(W=n^\perp\).
Its defining linear form therefore divides \(Q\), so for some vector
\(a\in\overline K^4\),
\[
Q(z)=(n\cdot z)(a\cdot z).                             \tag{12}
\]
The polynomial \(e_3\) is squarefree in each coordinate. Hence \(Q\)
has no terms \(z_i^2\). The coefficient of \(z_i^2\) on the right of
(12) is \(n_i a_i\). By (9), every \(a_i=0\), so \(Q\) is the zero
polynomial on all of \(\overline K^4\).

Write \(S=\sum_iw_i\). For each \(i<j\), the coefficient of \(z_i z_j\)
in \(Q\) is
\[
S-w_i-w_j.                                             \tag{13}
\]
Its vanishing for all six pairs implies that all four \(w_i\) are
equal to some \(c\). Then (13) gives \(2c=0\), so \(w=0\).
Finally \(\mathcal L\) is injective, giving \(v=0\).
The six linear forms therefore span the whole three-dimensional
cotangent space. This proves the theorem.

This proof uses neither a generic-normal assumption nor a choice of a
nonvanishing three-by-three minor that must be known before solving.
Different triples of the six rows may carry the rank at different
solutions; retaining all six handles them uniformly.

The nonzero-coordinate condition in (9) matters. For example, with
\(n=(1,0,0,1)\), write \(W\) as \(z=(a,b,c,-a)\). Then
\[
e_3(z)=-a^2(b+c).
\]
Its Hessian entries span only \(a\) and \(b+c\), giving rank two.
This is a synthetic out-of-scope counterexample, not an admitted RR5
normal.

## 5. Exact support and the regular-chart radical

Let \(B=K[b,c,d]\), and define
\[
I_{\rm coll}=(F_1,F_2,F_3,F_4),\qquad
J=I_{\rm coll}+(H_{11},H_{12},H_{13},H_{22},H_{23},H_{33}).
\]
On the regular open chart the two ideals have exactly the same
geometric zero set: every added equation vanishes at every original
regular solution, and every zero of \(J\) already satisfies the
original equations. Keeping the original equations is essential;
the six Hessian equations alone need not have the intended global
zero set.

There are finitely many possible four-element root sets of the fixed
selector and finitely many choices of signed lifts. Each admitted
signed divisor determines a unique normalized RR5 function. Hence the
regular zero set is finite. The rank theorem makes every local
Artinian ring of \(B[\Delta^{-1}]/J\) a field after algebraic closure:
its maximal ideal has zero cotangent space and is zero by Nakayama's
lemma. Thus
\[
J\,B[\Delta^{-1}]
   =\sqrt{I_{\rm coll}\,B[\Delta^{-1}]}.                 \tag{14}
\]

For comparison let \(I_{\rm rem}\) be generated by the four
coefficients of \(\phi\bmod U\). The existing radicality argument
also applies here. At a solution \(\phi=UQ\), the factors \(U,Q\)
are coprime. A tangent preserving the remainder equations satisfies
\[
\delta U\,Q+U\,\delta Q=0.
\]
Reduction modulo \(U\) forces \(\delta U=0\); Section 3 then forces
\(\delta\theta=0\). The finite remainder ideal is consequently
geometrically radical. Its roots all have \(\Delta\ne0\).
Equality of zero sets and reducedness, followed by field-extension
contraction, gives the precise saturated identity
\[
\boxed{J:\Delta^\infty=I_{\rm rem}.}                    \tag{15}
\]
This is an equality with the radical remainder presentation, not a
claim that the unsaturated collapsed ideal was already reduced.
It covers every regular solution, including those with intermediate
Dickson-image collisions. [pcc:unverified]

For the admitted split M31 selector, the four-leaf anti-Frobenius lemma
in the structured-selector note shows that all geometric remainder
solutions have rational signed leaves and rational \(b,c,d\).
Its hypotheses are unchanged: full odd rational group order, four
distinct abscissae, and target abscissa outside the selector. Equation
(15) uses that established chart; it does not add field equations or
extend that rationality lemma to longer sums.

## 6. Optional three-equation arm and its exact exception

The first derivatives of \(F_2\) also vanish at every regular solution.
They are a smaller conditional alternative, not required by (10).
Let \({\bf1}=(1,1,1,1)^T\). Since
\[
e_2(z)=\big(({\bf1}\cdot z)^2-z\cdot z\big)/2,
\]
at a solution
\[
dF_1={\bf1}^T\mathcal L,\qquad
\operatorname{Hess}F_2
 =\mathcal L^T({\bf1}{\bf1}^T-I)\mathcal L.             \tag{16}
\]
For a common kernel direction \(v\), write \(w=\mathcal L v\in W\).
The first row gives \({\bf1}\cdot w=0\). The Hessian rows then give
\(\mathcal L^Tw=0\), hence \(w\in W^\perp=\langle n\rangle\).
A nonzero common kernel exists exactly when
\[
\boxed{{\bf1}\cdot n=0\quad\hbox{and}\quad n\cdot n=0.} \tag{17}
\]
Its dimension is one; thus the Jacobian of the original equations
together with the three first derivatives of \(F_2\) has rank two in
(17), and rank three otherwise. These two equations describe a
codimension-two condition in the space of nonzero normals; no
frequency statement about the actual RR5 normal population follows.
Finite-field isotropy cannot be excluded by real-positive-definiteness
reasoning. In particular \(n\parallel{\bf1}\) is not exceptional:
then \({\bf1}\cdot n\ne0\) since \(4\ne0\).

For another view of how \(F_3\) repairs (17), its third polar form is
\[
\begin{aligned}
T_3(w,u,v)={}&({\bf1}\cdot w)({\bf1}\cdot u)({\bf1}\cdot v)
-({\bf1}\cdot w)(u\cdot v)\\
&-({\bf1}\cdot u)(w\cdot v)
-({\bf1}\cdot v)(w\cdot u)
+2\sum_iw_i u_i v_i.
\end{aligned}
\]
For the exceptional remaining direction \(w=n\) and \(u,v\in W\),
this reduces to \(2\sum_i n_i u_i v_i\). Since every \(n_i\ne0\),
this nondegenerate diagonal bilinear form cannot vanish identically
on a three-dimensional hyperplane in a four-dimensional space.
This establishes a nine-derivative fallback directly, while Section 4
proves the stronger result that the six second derivatives suffice by
themselves. No third derivatives of \(F_4\) are required.

## 7. Exact graph differentiation and implementation consequences

One exact construction substitutes
\[
(b,c,d)\longmapsto(b+\varepsilon_1,c+\varepsilon_2,d+\varepsilon_3)
\]
in (2), propagates the complete truncated Taylor polynomials through
all gates (6), and applies (7). Addition and multiplication in the
truncated polynomial ring enforce the chain rule exactly.
For a multi-index \(\alpha\), its Taylor coefficient is
\(\partial^\alpha F_j/\alpha!\); conversion to actual repeated
partials must retain the factorial. Characteristic \(p>3\) makes all
factorials through order three invertible.

Order-two jets suffice to construct the six equations as polynomial
functions of the parameters. Order-three jets also give their
Jacobian and are used by the fixed development check. In a lifted
solver system, every jet output must be tied to all coefficients of
its preceding gate, and the initial parameter jets must be fixed.
The resulting acyclic lift is a graph extension, with uniquely
determined derivative wires over any coefficient algebra. Free
derivative wires or a missing coefficient equation do not implement
(10). Full graph equations plus terminal constraints are retained.

An independent point check may reconstruct each simple root as a
formal jet using \(U(x_i(\varepsilon),\theta+\varepsilon)=0\), then
evaluate scalar Dickson composition and elementary symmetric
functions. Its linear term is
\[
\frac{\partial x_i}{\partial\theta_a}
 =-\frac{\partial_{\theta_a}U(x_i)}{U'(x_i)},\qquad
\mathcal L_{ia}
 =-\phi'(x_i)\frac{\partial_{\theta_a}U(x_i)}{U'(x_i)}.  \tag{18}
\]
All inverses here are valid only at admitted simple known-answer
points. This is an independent diagnostic path, not a way for the
candidate solver to receive or enumerate unknown leaves.

The concrete potential advantage is removal of artificial local
multiplicity in the collapsed presentation. At a regular solution the
original four equations have Jacobian rank at most one, because
\(F_j\) vanishes to order at least \(j\). The augmented system has
rank three. The construction is deterministic and the same six
polynomials cover every admitted regular root; it does not select
favorable collision branches or require an approximate root first.

This local fact does not bound the effort to discover the roots.
Automatic differentiation introduces additional arithmetic and, in a
lifted representation, more graph variables and equations. It may
increase matrix sizes or storage. The fully expanded terminal
coefficient degrees have the elementary upper bound \(2^{11}=2048\)
from the quadratic initial map and ten quadratic Graeffe maps;
differentiating twice reduces that bound by two, not to a small
constant. These are polynomial-degree bounds, not solving-degree
bounds. No bounded-width elimination, small quotient, reduced F4
work, or complete relation extraction follows from (15).

The discriminating next comparison is the same admitted target and
regular solution set under the remainder, collapsed, and six-Hessian
presentations, charging construction and derivative work as well as
complete solving and replay. The current protocol stops at exact
finite M31 construction controls; no such discovery comparison is
recorded by this note.

## 8. Conditional extension to more leaves

The linear-algebra argument has a useful general form. Let \(m\ge3\),
let \(p>3\) with \(p\nmid(m-2)\), and suppose a local parameter map
\[
z:K^{m-1}\longrightarrow K^m
\]
has \(z(\theta_0)=0\), differential \(\mathcal L\) of rank \(m-1\),
and image \(W=n^\perp\) with every \(n_i\ne0\).
Then all
\[
\binom{m}{2}
\]
second partial derivatives of \(e_3(z(\theta))\), with respect to the
\(m-1\) parameters, vanish at \(\theta_0\) and have joint Jacobian
rank \(m-1\).

The proof of Section 4 is unchanged through (12): absence of squares
forces \(Q=\partial_we_3\) to be identically zero. Its coefficient
of \(z_i z_j\) is still \(S-w_i-w_j\), now forcing every \(w_i=c\)
and \((m-2)c=0\). The characteristic condition then gives \(w=0\).
The condition cannot simply be dropped: if \(p\mid(m-2)\), then
\(\partial_{\bf1}e_3=(m-2)e_2=0\), so a hyperplane containing
\({\bf1}\) has an undetected tangent direction.

For an elliptic fixed-sum fiber with distinct simple abscissae,
nonzero ordinates, and a squarefree selector, its invariant
differential gives the same candidate normal
\(n_i=(2y_i\phi'(x_i))^{-1}\). A valid \(m-1\)-parameter chart must
still be constructed and its rank established. At \(m=8\) the
conditional statement calls for 28 second-derivative equations,
always using \(e_3\). This is a derived extension, with no
implementation, curve control, or extraction claim here. The
four-leaf rationality argument and frozen M31 controls do not
automatically extend to it. [pcc:unverified]

## 9. Sources checked and standing

- The two RR5 notes linked in Section 1 supply the existing norm chart,
  denominator lemma, four-leaf rationality argument, and radical
  remainder ideal. The exact M31 arithmetic/base source is
  [the existing native library](../prime31_ic_20261115/native/lib.rs).
- The earlier
  [first-collision study](../prime31_ic_20261115/native/DICKSON_FIRST_COLLISION_S3_20261115.md)
  and
  [quotient/GCD study](../prime31_ic_20261115/native/S3_QUOTIENT_GCD_ORACLE_20261115.md)
  are prior mechanisms, not new discoveries of this task. The present
  rule keeps all regular support in one collapsed presentation and
  does not repeat their pair-branch construction.
- J. D. Hauenstein and C. W. Wampler, *Isosingular Sets and Deflation*,
  Foundations of Computational Mathematics 13 (2013), 371–403,
  [author preprint](https://academicweb.nd.edu/~cwample1/Preprints/hwIsosingular.pdf).
  Sections 2–4 describe differential/determinantal deflation over
  complex polynomial systems and distinguish regularizing selected
  components from computing an entire radical. They motivate the
  explicit support and rank obligations proved here.
- J. D. Hauenstein, B. Mourrain, and A. Szanto, *On deflation and
  multiplicity structure*, Journal of Symbolic Computation 83 (2017),
  228–253,
  [author preprint](https://aszanto.math.ncsu.edu/Deflation-preprint.pdf),
  [arXiv:1601.00611](https://arxiv.org/abs/1601.00611).
  Section 2 assumes a coefficient field contained in \(\mathbb C\).
  Section 3 treats first-differential constructions; its introductory
  discussion also identifies adding partial derivatives that vanish
  at a root as an existing preprocessing step. Those results do not
  by themselves certify the M31 construction or its simultaneous
  preservation of every regular RR5 root.
- Sources checked (KB): no KB MCP tool was exposed. The parent
  session records that the repository wrapper exited 127 because
  its executable was unavailable. The fallback was named repository
  sources and the accessible primary papers above. This was not an
  exhaustive novelty search.

All new theorem, ideal-equality, and extension statements in this file
remain written, unsigned derivations. Any native finite-panel result
must retain its separate scope and evidence; neither self-checks nor
these calculations provide signed theorem standing. [pcc:unverified]

