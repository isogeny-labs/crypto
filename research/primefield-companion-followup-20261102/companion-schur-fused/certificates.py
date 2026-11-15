"""Independent exact residual and normalized witness checks, unexecuted.

Plain Python integers, no source operator, factorization, oracle or hidden
scalar. A and S are explicit validation references, not production inputs.
"""
def product(A,x,r,transpose=False):
    if transpose:
        assert len(x)==len(A)
        return [sum(A[i][j]*x[i] for i in range(len(A)))%r for j in range(len(A[0]))]
    assert all(len(row)==len(x) for row in A)
    return [sum(a*b for a,b in zip(row,x))%r for row in A]

def pairing(a,b,r):
    assert len(a)==len(b)
    return sum(x*y for x,y in zip(a,b))%r

def check_residuals(A,S,b,z,h,x,v,w,r):
    N=len(A);m=len(S);n=N-m
    assert n>0 and m>0 and all(len(row)==N for row in A)
    assert all(len(row)==m for row in S)
    assert len(b)==len(x)==len(w)==N
    assert len(z)==len(h)==len(v)==m
    assert all(type(t)is int and 0<=t<r for vec in [b,z,h,x,v,w] for t in vec)
    assert x[n:]==list(z) and w[n:]==list(v)
    primal=[(u-t)%r for u,t in zip(product(A,x,r),b)]
    reduced=[(u-t)%r for u,t in zip(product(S,z,r),h)]
    dual=product(A,w,r,True);small_dual=product(S,v,r,True)
    full_pair=pairing(w,b,r);small_pair=pairing(v,h,r)
    assert primal==[0]*n+reduced
    assert dual==[0]*n+small_dual
    assert full_pair==small_pair
    normalized=not any(small_dual) and small_pair==1
    if normalized:assert not any(dual) and full_pair==1 and any(w)
    return dict(primal_residual=primal,reduced_residual=reduced,
                dual_residual=dual,reduced_dual_residual=small_dual,
                full_pairing=full_pair,reduced_pairing=small_pair,
                normalized_exclusion=normalized)

def check_feature_annihilation(F,w,r):
    """A separate condition: never inferred from nullness of a Gram matrix."""
    result=product(F,w,r,True)
    assert not any(result)
    return dict(feature_annihilation=True,feature_columns=len(F[0]))
