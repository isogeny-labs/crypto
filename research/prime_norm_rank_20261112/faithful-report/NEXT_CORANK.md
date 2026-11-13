# Explicit public branch basis for the next corank proof

Unproved continuation plan, not an additional formal result or search-cost
claim [pcc:unverified].

The pinned degreeLT.basis_val and basisProd_castAdd definitions identify the
first full coordinate with the pair(1,0). Its actual branch coefficient is1.
Let N=(a+1)+(b+1), and let ell_i be the actual branch coefficients. Instead
of choosing an unspecified basis of ker(ell), use the explicit N-by-(N-1)
public matrix T with columns

    T_j = e_(j+1) - ell_(j+1) e_0,   j=0,...,N-2.

Prove the first-coordinate assertion from the actual product basis, then
check the following identities [pcc:unverified]:

1. ell(T z)=0 for every z.
2. Tail deletion L satisfies L(T z)=z.
3. T(L w)=w for every actual branch vector w.

These exact identities supply an injective map onto the branch and its inverse,
rather than assuming a branch basis. T uses only q,y and the public monomial
families. It does not require the unknown norm witness or quotient moments.
The source may use Fin.cases/succ; read the pinned APIs before coding.

Define the actual reduced moment and coefficient matrices by

    S_red = T^transpose S T,   Q_red,j = T^transpose Q_j T.

Transfer the existing arbitrary-pair identity through T. The current theorem
coordinateBranchRadical should then give the exact reduced kernel

    ker(S_red) = span(L w0).

Prove L w0!=0 using T(L w0)=w0 and the existing nonzero witness theorem.
Use rank-nullity for corank1 and an actual nonzero maximal minor for the
adjugate guard. The witness norm supplies every reduced coefficient quadratic;
rank-one adjugate factorization then supplies the stationarity pairings.
Apply the existing guarded recovery only after all these bridges are checked
[pcc:unverified].

Predictions: no literal rank assumption, no division by2, no search for a basis
and no nonzero branch-coefficient oracle. A mismatch should expose an index,
transpose, left/right-kernel or degree-chart error, not be bypassed by a numeric
rank assertion. Keep failed proof logs. Constant-H/degree drops remain separate.
The public-input certificate finder and its uniform bit cost remain additional
obligations. No timing experiment is admitted by this plan.
