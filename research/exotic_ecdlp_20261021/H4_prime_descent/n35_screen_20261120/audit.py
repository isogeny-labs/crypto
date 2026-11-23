#!/usr/bin/env python3
"""Independent reconstruction of the five frozen n35 curve orders."""
from sage.all import *
import hashlib,json,random
from pathlib import Path
HERE=Path(__file__).resolve().parent
SPEC=json.loads((HERE/"protocol.json").read_text())
RESULT_PATH=HERE/"result.json"
RESULT=json.loads(RESULT_PATH.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def enc(e):return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
assert RESULT["status"]=="complete" and len(RESULT["instances"])==5
assert sha(HERE/"protocol.json")==RESULT["protocol_sha256"]
assert sha(HERE/"run.py")==RESULT["runner_sha256"]
assert SPEC["instance_indices"]==[0,1,2,3,4]
checks=[]
for idx,entry in enumerate(RESULT["instances"]):
    n=35;l=12
    rng=random.Random(20261022*1000+n*10+idx)
    set_random_seed(20261022+n*10+idx)
    mod=PolynomialRing(GF(2),"x").irreducible_element(n)
    K=GF(2**n,"z",modulus=mod)
    def elt(v):return K([(v>>i)&1 for i in range(n)])
    a2=elt(rng.getrandbits(n));a6=elt(rng.getrandbits(n))
    while a6==0:a6=elt(rng.getrandbits(n))
    E=EllipticCurve(K,[1,a2,0,0,a6])
    while True:
        basis=[rng.getrandbits(n) for _ in range(l)]
        mat=matrix(GF(2),[[((v>>bit)&1) for bit in range(n)] for v in basis])
        if mat.rank()==l:break
    order=int(E.cardinality())
    fac=[[int(p),int(e)] for p,e in factor(order)]
    r=max(p for p,e in fac)
    assert entry["index"]==idx
    assert entry["field_modulus"]==sum(int(c)<<i for i,c in enumerate(mod.list()))
    assert [entry["curve_a2"],entry["curve_a6"]]==[enc(a2),enc(a6)]
    assert entry["subspace_basis"]==basis
    assert entry["group_order"]==order and entry["factorization"]==fac
    assert entry["largest_prime_divisor"]==r and is_prime(r)
    assert entry["cofactor"]==order//r and entry["eligible"]==(r>=2**30)
    assert (order-(1<<n)-1)**2<=4*(1<<n)
    checks.append({"index":idx,"order":order,"largest_prime":r,"eligible":entry["eligible"]})
assert RESULT["selected_index"]==next((x["index"] for x in checks if x["eligible"]),None)
audit={"schema":"h4-n35-screen-audit-v1","status":"passed",
       "result_sha256":sha(RESULT_PATH),"selected_index":RESULT["selected_index"],
       "instances":checks}
(HERE/"audit.json").write_text(json.dumps(audit,indent=2,sort_keys=True)+"\n")
print(json.dumps(audit,sort_keys=True))
