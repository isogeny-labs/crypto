# Conditional trace-fiber structure and its exact premises

All algebra below is in a finite field K of characteristic two. The fixed
normalized binary S3 polynomial is used; no field/curve or subgroup transport
is inferred from a shared field degree. These are written arguments with
finite native controls, not independently signed or Lean-checked proofs.
[pcc:unverified]

## Trace compatibility and operator structure are distinct

For the ordinary normalized curve, Kosters–Yeo's known homomorphism sends
a finite P to Tr(x(P)+a2), with O mapped to zero. An odd-order subgroup lies
in its kernel. Thus its finite x-coordinates belong to H_tau, tau=Tr(a2).
Trace alone says H_tau is an affine hyperplane. It is not generally a
subfield or a multiplicatively closed set.

Fix a,b nonzero and distinct. In S3(a,b,u), let A=(a+b)^2, B=ab,
C=(ab)^2+beta. The operator L(u)=Au^2+Bu is GF(2)-linear. Its kernel is
exactly {0,B/A}, since L(u)=u*(Au+B). Define delta=B/A and s=a/(a+b).
Then s^2+s=ab/(a+b)^2=delta, so Tr(delta)=0. Delta is nonzero by the premises.
Rank-nullity on ker(Tr) gives rank n-2. Its translate H_tau maps to a
codimension-two affine image; a change of basis cannot alter this rank.
[pcc:unverified]

For n odd define H(h)=sum_{i=0}^{(n-1)/2} h^(2^(2i)). Then
H(h)^2+H(h)=h+Tr(h). With h=AC/B^2 and Tr(h)=0, the two roots are
delta*H(h) and delta*(H(h)+1). They have the same trace because delta has
trace zero. The trace check is therefore independent of the one-bit root
choice. Both roots or neither survive a specified trace class.
[pcc:unverified]

The two image tests can be written Tr(h)=0 and Tr(delta*H(h))=tau for
fixed A,B and odd n. They are constraints on the right-hand side for that
conditional linear problem. They are not new independent subgroup filters
on leaves whose rational subgroup membership has already been established.

For a conditioned pair, take delta as the first basis vector of ker(Tr)
and extend it to a full basis. The corresponding first matrix column is
zero, and the remaining n-2 columns are independent: any dependence would
give a kernel element outside the span of delta. The native chart constructor
and rank/span controls expose this structure. This is conditional basis
alignment, not a reduction of the invariant rank or a gain over an arbitrary
basis whose kernel has already been eliminated. [pcc:unverified]

## Coupled blocks and all singular branches

Write P_i(u)=A_i u^2+B_i u+C_i. The combination
A1*P2+A2*P1 cancels u^2 and gives D*u+N, where
D=A1*B2+A2*B1, N=A1*C2+A2*C1. Thus:

1. If D!=0, any common root must be u=N/D. Substitute it into BOTH P_i.
2. If D=0,N!=0, there is no common root.
3. If D=N=0, solve P1 completely and filter by P2. This is necessary even
   when both A_i vanish: then D=N=0 does not establish proportionality or
   consistency of the remaining linear equations.

This algorithm is complete over K, including zero inputs and all constant
branches. It says nothing about rational point signs or identity partial sums.
[pcc:unverified]

For inputs (a,b,z,t), expansion gives

    D=(a+b)^2*z*t+(z+t)^2*a*b
     =(a*z+b*t)*(a*t+b*z).

If both input pairs are nonzero/distinct, D=0 iff their nonzero kernel
directions agree. Otherwise their only common homogeneous kernel element is
zero. The stacked map restricted to ker(Tr) therefore has rank n-2 when
D=0 and n-1 when D!=0. The two blocks usually remove the root ambiguity;
trace does not do so by itself. [pcc:unverified]

A further native identity control checks

    N=(e3+sqrt(beta)*e1)^2,
    e1=a+b+z+t,
    e3=abz+abt+azt+bzt.

It is a characteristic-two expansion, not a new identity certificate or a
reason to omit the D=0 branches. Eliminating the auxiliary symbolically can
raise polynomial degree; this is the usual resultant tradeoff. Cheap
fixed-coefficient elimination does not imply cheap solving for unknown leaves.
[pcc:unverified]

## The reusable structure that an actual set can supply

For a=r*b and b!=0, division by b^2 yields

    L_r(u)=(r+1)^2*u^2+r*u = r^2*b^2+beta/b^2.

The left map is fixed for all pairs with the same r. Its Gaussian transform
and right-inverse/image-test data can therefore be cached. The kernel
delta(r)=r/(r+1)^2 is invariant under r -> 1/r. For nonzero r,s different
from one, clearing denominators shows delta(r)=delta(s) iff
(r+s)*(rs+1)=0, i.e. s=r or s=1/r. [pcc:unverified]

If a nonzero coordinate set S has a small quotient set S/S, it gives few
different normalized maps. Intersecting a length-L multiplicative progression
with the trace class and actual subgroup x-inventory preserves the bound
|S/S|<=2L-1. No density of the retained set is assumed. Its membership and
ratio search can be expensive, and enumerating all pairs remains quadratic.
A precomputation saving must be compared with the strongest fixed-base
algebraic or exact pair reference, including all setup and failed queries.
[pcc:unverified]

## Proof map

| Statement | Evidence in this component | Remaining scope |
|---|---|---|
| Trace kernel for subgroup points | published Kosters–Yeo result reused | actual inventory/sign adapter is not here |
| delta has trace zero; restricted rank n-2 | factorization and rank-nullity above, finite rank controls | no Lean kernel proof |
| Half-trace roots | standard identity, compared with binary elimination and exhaustive roots | odd degree only for that route |
| Complete coupled scalar roots | three-branch argument, exhaustive small-field oracle | no unknown-leaf search or group relation claim |
| Ratio-class operator reuse | exact scaling identity and native controls | cost saving not measured |
| Low quotient-set candidate yields better relations | proposed test criterion only | inventory density, solving, rank and full cost unresolved |

The native controls use small supported fields and exact arithmetic, with
source-bound command logs in evidence/. The current IDC1v1 odd-prime format
cannot represent these characteristic-two/Boolean statements; no incompatible
certificate is created. Lean/lake are still absent. A faithful formal bridge
must state the field, nonzero/distinct premises, finite trace and rank maps,
and the scalar-fiber versus group-relation distinction explicitly.
