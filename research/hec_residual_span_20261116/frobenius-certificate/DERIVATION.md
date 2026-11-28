# Frobenius action from twisted fixed-point traces

Mathematical deduction [pcc:unverified]; no novelty/formalverification claim.
Let C:y²=x^n+c overFq, oddcharacteristic notdividing oddprime n, c!=0,
q=1modn, genusg=(n-1)/2. Let sigma(x,y)=(z*x,y), z primitive n-root,
and pi the q-power Frobenius endomorphism (not inverse arithmetic action).
Use an auxiliary Tateprime L distinct from fieldcharacteristic and n.

The quotient bysigma is rational, so Phi_n(sigma)=0 on J. Since sigma
is notidentity, Q(zeta_n) embeds in End0(J), degree n-1=2g. Its rational
Tatemodule has rank1 over K tensor Q_L (Milne AV Proposition10.23),
so every nontrivial sigma eigenvalue occurs once. Pi commutes withsigma;
the centralizer is K tensor Q_L and its tracepairing is nondegenerate.

Set chi(t)=t^((q-1)/n), S_j=sum_{t!=0,chi(t)=z^j} eta(t+c),
P(X)=-sum_j S_j X^j. Then sum_j S_j=-eta(c).
Fixedpoints of sigma^a*pi have y inFq. For x!=0, t=x^n lies inFq*,
and chi(t)=z^-a; every such t has n corresponding x. Include x0 and
infinity to obtain

    #Fix(sigma^a*pi)=q+1+eta(c)+n*S_(-a).

The derivative of Frobenius iszero, so these fixedpoints have multiplicity1.
Lefschetz gives trace(sigma^a*pi)=-eta(c)-n*S_(-a). On the otherhand,
trace(sigma^k)=n-1 if n|k, else -1, hence

    trace(sigma^a*P(sigma))=-n*S_(-a)-eta(c).

Equality for alla and nondegenerate tracepairing in the centralizer imply
pi=P(sigma) on the rational Tatemodule; faithfulness gives the equality
of actual endomorphisms. This step needs fulltwistedtrace information,
not just P's characteristic polynomial, norm or one rationalpoint count.
H1pullback and Tate action conventions are identified by the usual
Jacobian/cohomology duality; traces agree, no z->z^-1 orientation swap.

On rational elltorsion, pi=1. If an exact modular identity is checked

    U(X)Phi_n(X)+V(X)(P(X)-1)=X-lambda overFell,

then sigma-lambda annihilates every rational elltorsion point. No native
lambda*C multiplication is required by this argument. In the fixed
wholeprime-order instances all rational points are elltorsion. Similarly
[ell]C=0 follows from that SAME external wholeorder premise by Lagrange;
checking it anew is redundant, not an additional assumption. NonzeroC
and publicsource [sm]C=D still checked. Do not infer wholeorder merely
from [ell]C=0, or apply this fielddegree proof to composite n=9/genus4.

Primary sources read: Milne LEC Theorem25.1 (fixedpointformula),
https://www.jmilne.org/math/CourseNotes/LEC.pdf ; Milne AV Proposition10.23
and faithful Tateaction background, https://www.jmilne.org/math/CourseNotes/AV.pdf .
Own specialization above, not a theorem attributed to those texts.
KB query found source3e1e01117261f169a97967304713c645, unrelated proposed
ArtinSchreiermeasurement, not support for this proof; primary sources used.
Independent mathematical review/formalization remains required.
