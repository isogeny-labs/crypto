# Trace elimination and its boundary

This is a derivation, pending independent standing. Let F be a field with
2 nonzero, q a positive integer, and omega=(c,d) have c^2-2d^2=1 with d!=0.
Set T=T_q(x), U=U_(q-1)(x), A=x^2-1. Classical polynomial identities give
T^2-A*U^2=1. If T=c, then A*U^2=2d^2, so U and A are nonzero.
Define b=A*U/(2d). Then b*U=d and 2b^2=A. Hence z=(x,b) has norm one and
z^q=(T,bU)=(c,d). Conversely an original norm-one root has T=c and bU=d,
and A*U^2=2d^2 forces that same value of b. This is a bijection of roots
on the stated chart; forgetting b without the specified lift is incomplete.

The cleared residuals make the inference auditable. With
R=T^2-AU^2-1, W=c^2-2d^2-1, C=(T-c)(T+c), we have
AU^2-2d^2=C-R+W and
(2d)^2*A-2*(AU)^2=-2*A*(C-R+W).
Native identity records check these polynomial equalities at fixed streams;
they are not formal proofs of their use on the whole curve instance.

Differentiation supplies U=T'_q/q only when q is nonzero in F. This extra
implementation restriction does not silently alter the underlying lemma.
The d=0 chart must be handled separately: q3 at x=-1/2 has T=1,U=0 in
characteristic101, and the lift formula is undefined. The API rejects it.

For the original many-coset contract, omega=h^(qj), h has odd order aq,
and 1<=j<=k<a/2. Thus omega is neither1 nor-1 and d!=0. Subject to the
original exact-order/extension hypotheses, each qth-power fibre has q
elements. Real-part equality for norm-one elements identifies z with z^-1;
both cannot belong to this fibre because omega!=omega^-1. Its q real
parts are distinct. They are precisely the roots of T_q(X)-c, whose degree
is q in characteristic different from2. This gives squarefree splitting
for that declared fibre, not for arbitrary q/omega over an arbitrary field.

The scalar equation is a candidate solver preconditioner. Expanding it or
constructing its full coordinate algebra costs something not measured by
the recurrence's evaluation size. No generic solving-degree bound, useful
relation yield, lower attack exponent or root-discovery algorithm follows.
