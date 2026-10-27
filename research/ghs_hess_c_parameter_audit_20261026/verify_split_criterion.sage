"""Independent finite-field check of the trace-zero product criterion."""

import json
from pathlib import Path

root=Path(__file__).resolve().parent
records=[]
for ell in (1,2,3,4):
    q=2**ell
    K=GF(q**3,'w',modulus='conway')
    w=K.gen()
    a=w**(q*q+q+1)
    scalar=[K(0)]+[a**i for i in range(q-1)]
    assert len(set(scalar))==q
    def trace(v): return v+v**q+v**(q*q)
    U=[v for v in K if v and trace(v)==0]
    assert len(U)==q*q-1
    u=U[0]
    v=next(t for t in U if t/u not in scalar)
    coeffs={}
    for A in scalar:
        for B in scalar:
            for C in scalar:
                delta=A*u*u+B*u*v+C*v*v
                assert delta not in coeffs
                coeffs[delta]=(A,B,C)
    assert len(coeffs)==q**3
    products={x*y for x in U for y in U}
    def split(A,B,C):
        if B==0: return True
        z=A*C/B**2
        return sum((z**(2**i) for i in range(ell)),K(0))==0
    splits={delta for delta,(A,B,C) in coeffs.items()
            if delta and split(A,B,C)}
    assert products==splits
    expected=(q-1)*(q+1)*(q+2)//2
    assert len(products)==expected
    expected_subfield=(q-1) if (q-1)%3==0 else 0
    subfield_nonzero=set(scalar)-{K(0)}
    assert len(products & subfield_nonzero)==expected_subfield
    records.append({'ell':int(ell),'q':int(q),'trace_zero_nonzero':int(len(U)),
                    'product_count':int(len(products)),
                    'formula_count':int(expected),
                    'subfield_product_count':int(expected_subfield),
                    'full_degree_product_count':int(len(products)-expected_subfield),
                    'split_criterion_matches':True})
    print('q',q,'products',len(products),'subfield',expected_subfield,flush=True)

(root/'split_criterion_receipt.json').write_text(json.dumps({
    'schema':'ghs-hess-cubic-product-formula-check/v1',
    'records':records},indent=2)+'\n')
