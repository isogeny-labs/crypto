# Next assembly: actual coefficient pencil and branch

Proposed proof route, not a completed result [pcc:unverified].

Keep F,H monic with a=deg F, b=deg H=a-2>0. Supplied full-degree norm
witness A0=alpha F, B0=beta H has alpha,beta nonzero. Bezout gives
UF+V(fH)=1 and mu(Z)=coeff_(a-1)((ZV) mod F). The present theorem gives
radical directions(F,0),(0,H) for the full block pairing.

1. Define mu as a genuine K-linear map on polynomials. Define bounded
   coefficient spaces and their basis maps; retain actual degree proofs.
2. Suppose A0^2-f B0^2=cG, c nonzero. Mu(A0^2)=0 by divisibility by F.
   Mu(f B0^2)=0 by the lower radical and the full-degree test H. Apply the
   linear map to the actual norm equation to obtain mu(G)=0. Do not assume
   pencil membership as an extra hypothesis [pcc:unverified].
3. For monic G of degree d=2a, define the actual coefficient constraints
   q_j(A,B)=coeff_j(A^2-fB^2)-coeff_j(G)*coeff_d(A^2-fB^2), j<d.
   Expand bounded polynomials in the monomial basis. Mu(G)=0 should identify
   mu(A^2-fB^2)=sum_(j<d)mu(X^j)*q_j(A,B), including the leading coefficient
   cancellation. This supplies the actual dual-pencil coefficients.
4. Build the symmetric matrix of mu(AQ-fBT) in those coefficient bases;
   match quadratic evaluation and polarized pairing explicitly. Characteristic
   not2 is available in the intended prime-field domain; keep assumptions
   explicit if a factor2 is used.
5. At target(q,y), branch l(A,B)=A(q)-yB(q). If F(q)!=0, k=(F,0) is a
   right-radical vector with l(k)!=0. Instantiate branchTestsDetectFull,
   including scalar-space maps. From alpha F(q)=y beta H(q), nonzero alpha
   and nonzero F(q), prove the radical intersection is span(alpha F,beta H).
6. Transfer exact radical dimension1 to matrix corank1, then apply the earlier
   adjugate recovery theorem with all nonzero/stationarity guards proved.

Each step still needs its own exact formal statement and proof. Even after
assembly, V and the moments above are constructed using the supplied witness;
a public-input way to find them, with bit complexity 2^{o(log r)}, remains
essential. No search-time gain follows solely from this plan [pcc:unverified].

## 2026-11-13 functional continuation

NormPencilFunctional now supplies the genuine quotient linear map, derives
mu(G)=0 from the scaled norm equation, and gives the actual trial-norm
coefficient-pencil identity using the proved degree bounds. See
../functional-report/README.md for exact scope, source and checks. This
completes the functional obligations in steps1–3 above; concrete bounded
basis/matrix, branch intersection and corank steps remain open. No public-input
finder or search bound is supplied [pcc:unverified].
