# Prime leading-syzygy first fall: candidate exact certificate argument

This follow-up targets the standard first-fall invariant: the least total degree of a nontrivial syzygy of the highest homogeneous parts, modulo Koszul syzygies (and characteristic-p self syzygies, which occur far above this range). It is Definition 1.3 of [Caminata and Gorla](https://arxiv.org/html/2112.05579). The R3 binary value2 agrees with this invariant and with the earliest new affine degree fall. For prime systems the earliest new affine fall remains a separate quantity; a nonzero lower-degree product need not be new in V(F,d). Do not silently identify those quantities.

Let ell=floor(83/m), h=floor(ell/2), and A_i=k[b_i0,...,b_i(ell-1)]/(b_ij^2), with L_i=sum_j 2^j b_ij. Let A be the tensor product of all A_i and a polynomial ring in the m-2 intermediate coordinates U_j. This is the quotient by the leading selector equations. These equations form a regular sequence, so their Koszul complex resolves A; first Koszul homology of all leading equations is the first Koszul homology of the quartics over A. No characteristic-p self syzygy occurs at the degrees considered.

The leading S3 form is H(X,Y,Z)=X²Y²+X²Z²+Y²Z²−2XYZ(X+Y+Z); the fixed-target endpoint has leading form X²Y². The saved formulas and field are already checked by R3 and must be checked again before accepting a certificate.

## Lower bound: no nontrivial syzygy below h+4

Theorem2.7 of [Phuong and Tran](https://arxiv.org/html/2211.13548) proves the strong Lefschetz property of k[y_1,...,y_ell]/(y_i²) in characteristic zero or characteristic greater than ell. Our prime is greater than83, and the variable scaling y_j=2^j b_j is invertible. Multiplication by L_i² has rank min(C(ell,r),C(ell,r+2)) on degree r. It is injective for r<h and has a kernel at r=h.

Use a homogeneous Lefschetz-string basis for each A_i: each string starts in degree r and consists of v,L_i v,...,L_i^(ell−2r)v. Filter this basis by the exponent of L_i. Order the abstract raising variables lexicographically as L0 > L1 > U0 > L2 > U1 > L3 > ... > U_(m−3) > L_(m−1). Give each Koszul generator the weight of its quartic's leading raising monomial. On each fixed total-degree strand this is a finite filtration.

The associated graded differential is the Koszul differential for:

- G0=L0² L1²;
- Gk=U_(k−1)² L_(k+1)² for k=1,...,m−2.

These operators involve disjoint tensor factors. Therefore their Koszul complex is the tensor product of the corresponding one-element complexes. By the Kunneth formula, first homology is a sum of an annihilator of one Gi tensored with the quotients for the other Gj. Every Gi is injective on multiplier degrees below h: polynomial multiplication by U² is injective, and both L² factors in G0 are injective in each bidegree of total degree below h. Thus first homology vanishes in every total degree below h+4.

Vanishing passes to the original filtered complex. Explicitly, a leading syzygy at such a degree is a Koszul boundary for the Gi; subtract the corresponding boundary for the Hi and descend in the finite filtration. The process terminates at zero. Hence every syzygy below h+4 is trivial. This is a proof-based lower bound, not a measured full-matrix rank on n83.

## Upper bound: explicit nontrivial syzygy at h+4

In the final selector block, put y_j=2^j b_j and g=product_(a=0..h−1)(y_(2a)−y_(2a+1)). For each pair (y−z)(y+z)=0 in the square-zero quotient. Consequently Lg is g times the sum of the unpaired variables. There are zero or one such variables, so L²g=0. The endpoint leading form is U²L² (or L0²L1² for m2), so g times that leading form is zero.

The product g is nonzero: the monomial selecting the even variable of every pair has nonzero coefficient. This syzygy is not Koszul. For m2 there is only one quartic. For m>=3, if g e_last were a Koszul boundary, g would belong to the ideal of all other quartics. Setting every earlier selector block and every intermediate to zero kills those quartics but preserves nonzero g, a contradiction.

Therefore the standard leading-syzygy first-fall degree is exactly h+4. The native certificate will record the pair identities, field conditions, rank formula data, disjoint factor allocation and nontriviality specialization, and cross-check small examples with explicit finite-field matrices before any n83 follow-up run.

## Affine product check and its limit

The affine product g F_endpoint reduces modulo the selector relations to degree at most h+3, by the displayed leading cancellation. It is nonzero if some Boolean selector assignment with g!=0 has L!=target_x: the coefficient of U² is g(L−target_x)². Two pair choices give distinct L in [0,2^ell), so at least one differs from the target. For m2, F_endpoint is a nonzero quadratic in L0 at that assignment, so among L0=0,1,2 at least one evaluates nonzero. This supplies a checked nonzero degree-lowering product. It does not by itself certify that this affine polynomial is absent from V(F,h+3). Record earliest_new_affine_fall as unresolved for prime.

This argument is a producer derivation with machine-checkable hypotheses and explicit witnesses, not a Lean proof or independent research review. The controlled small-matrix checks are required before using its numeric formula on the requested grid.
