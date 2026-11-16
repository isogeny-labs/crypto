#!/usr/bin/env sage -python
"""Independent small-field census for the triangle/Hess overlap formula."""
import hashlib
import json
from pathlib import Path

from sage.all import GF

HERE=Path(__file__).resolve().parent


def one(m):
    q=2**m
    K=GF(2**(3*m),"z")
    V=[a for a in K if a and a+a**q+a**(q*q)==0]
    P={a*b for a in V for b in V}
    e=pow(q+1,-1,q**3-1)
    Z={s for s in P if (s**e)+(s**e)**q+(s**e)**(q*q)==0}
    assert Z=={t**(q+1) for t in V}

    def absolute_trace(t):
        y=K(0)
        for i in range(3*m):
            y+=t**(2**i)
        return y

    outside=sum(1 for s in P if absolute_trace(s**e)==1)
    assert len(P)==(q*q-1)*(q+2)//2
    assert outside==q*q*(q+1)//4
    return {"m":m,"q":q,"field_modulus":str(K.modulus()),
            "relative_trace_zero_nonzero":len(V),"hess_products":len(P),
            "projective_trace_zero_products":len(Z)//(q-1),
            "hess_outside_triangle":outside,
            "union_models":q**3-1+outside,
            "all_normalized_ordinary_models":2*(q**3-1)}


rows=[one(m) for m in range(1,6)]
out={"schema":"ghs-triangle-hess-overlap-census-v1",
     "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     "rows":rows}
(HERE/"overlap_census.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps([{k:r[k] for k in ("q","hess_products",
                                  "hess_outside_triangle","union_models")}
                  for r in rows]))
