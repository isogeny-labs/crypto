# Next transfer: actual branch and corank

Unproved plan, not a completed matrix rank or recovery theorem
[pcc:unverified].

Use the genuine product basis already defining upperFamily/lowerFamily.
For a vector w, its basis sum is a point x_w in
(degreeLT K(a+1)) x (degreeLT K(b+1)). Projecting x_w gives the assembled
polynomials. This should supply three explicit bridges:

1. Both assembled polynomials satisfy their bounds for every w.
2. If both assembled polynomials of w,z coincide, product/subtype extensionality
   gives x_w=x_z; applying the basis coordinate equivalence yields w=z.
3. Every bounded polynomial pair has coordinates (already familiesComplete).

For arbitrary w,z, derive the bilinear identity, not only its diagonal:

    Matrix.toBilin'(momentMatrix mu f A B) w z
      = mu(A_w*A_z - f*(B_w*B_z)).

Expand finite sums using actual multiplication/functional linearity, keeping
factor order and avoiding division by2. Matrix.sum_apply/smul_apply are the
correct matrix APIs. Do not infer the arbitrary-pair identity by an unsupported
polarization assumption in characteristic2 [pcc:unverified].

Define l(w)=A_w(q)-yB_w(q) as an actual linear map; the existing faithful
NormLift.branchEval gives its coefficient sum. If F(q)!=0, familiesComplete
gives k with assembled polynomials(F,0), hence l(k)!=0. For every v the
bilinear evaluation against k vanishes by F-divisibility. This instantiates
branchTestsDetectFull with the actual coordinate form and target branch.

The complete/full polynomial radical theorem, arbitrary-pair identity,
coordinate bounds and surjectivity should give

    B(w,v)=0 for every v iff A_w=cF and B_w=dH for some c,d.

On the branch, cF(q)=y*dH(q). A supplied actual norm witness also satisfies
alpha F(q)=y*beta H(q). Under alpha!=0 and F(q)!=0, solve the two scalar
equations to obtain(c,d)=t(alpha,beta). Coordinate injectivity then identifies
the branch radical with the line of the actual witness coefficient vector.
Prove both directions and its nonzeroness explicitly [pcc:unverified].

Only after these transfers: use a genuine basis of ker(l), show its restricted
form has radical dimension1, derive exact matrix corank1, nonzero adjugate and
all coefficient-form stationarity guards, then apply existing guarded recovery.
A known witness still supplies these moments; no public-input finder or
2^{o(log r)} search cost follows just from this certificate-existence route.
