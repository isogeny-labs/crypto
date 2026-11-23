#!/usr/bin/env python3
"""Independent base, sumset, target, and witness audit for H4 n29 instance one."""
from sage.all import *
import hashlib, json, random
from pathlib import Path
HERE = Path(__file__).resolve().parent
spec = json.loads((HERE / "protocol.json").read_text())
out = json.loads((HERE / "result.json").read_text())
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
assert out["status"] == "complete" and len(out["targets"]) == 32
assert sha(HERE / "protocol.json") == out["protocol_sha256"]
assert sha(HERE / "run.py") == out["runner_sha256"]
n, idx = 29, 1
rng = random.Random(20261022 * 1000 + n * 10 + idx)
set_random_seed(20261022 + n * 10 + idx)
mod = PolynomialRing(GF(2), "x").irreducible_element(n)
K = GF(2**n, "z", modulus=mod)
def elt(v): return K([(v >> i) & 1 for i in range(n)])
def enc(e): return sum(int(c) << i for i,c in enumerate(e.polynomial().list()))
def rec(P): return [enc(P[0]), enc(P[1])]
a2, a6 = elt(rng.getrandbits(n)), elt(rng.getrandbits(n))
while a6 == 0: a6 = elt(rng.getrandbits(n))
E = EllipticCurve(K, [1,a2,0,0,a6])
assert int(E.cardinality()) == out["group_order"] == 536895614
assert [[int(p),int(e)] for p,e in factor(out["group_order"])] == out["order_factorization"]
assert out["subgroup_order"] == 268447807 and out["cofactor"] == 2
assert [enc(a2),enc(a6)] == [out["curve_a2"],out["curve_a6"]]
basis = out["subspace_basis"]
while True:
    candidate = [rng.getrandbits(n) for _ in range(10)]
    rows=list(candidate); rank=0
    for bit in range(n):
        pivot=next((v for v in rows if (v>>bit)&1),None)
        if pivot is None: continue
        rows.remove(pivot);rank+=1
        rows=[v^pivot if (v>>bit)&1 else v for v in rows]
    if rank==10: break
assert candidate == basis
raw=[]
for bits in range(1<<10):
    x=0
    for j,v in enumerate(basis):
        if (bits>>j)&1: x^=v
    raw.extend(E.lift_x(elt(x),all=True))
zero=E(0); cleared=[2*P for P in raw]
base=sorted(set(cleared)-{zero},key=rec)
assert len(raw)==out["raw_count"] and len(base)==out["base_count"]
assert sum(out["subgroup_order"]*P==zero for P in raw)==out["direct_subgroup_points"]
assert cleared.count(zero)==out["cleared_identity_images"]
assert len(cleared)-len(set(cleared))==out["cleared_duplicate_images"]
assert hashlib.sha256(json.dumps([rec(P) for P in base],separators=(",",":")).encode()).hexdigest()==out["base_sha256"]
G=next(P for P in cleared if P!=zero)
assert rec(G)==out["generator"] and out["subgroup_order"]*G==zero
# Independent sumset membership audit: retain only points, not the runner's first-pair map.
pair_sums={A+B for A in base for B in base}
assert len(pair_sums)==out["distinct_pair_sums"] and out["pair_attempts"]==len(base)**2
verified=0
for i,row in enumerate(out["targets"]):
    label=f"h4-n29-instance1-target-{i}"
    scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),"big")%(out["subgroup_order"]-1)+1
    Q=scalar*G
    assert row["index"]==i and row["label"]==label
    assert row["fixture_scalar"]==scalar and row["target"]==rec(Q)
    found=any(Q-C in pair_sums for C in base)
    assert found == (row["status"]=="verified")
    if found:
        a,b,c=row["witness_indices"]
        assert base[a]+base[b]+base[c]==Q
        assert 1<=row["attempts"]<=len(base)
        verified+=1
    else:
        assert row["witness_indices"] is None and row["attempts"]==len(base)
audit={"schema":"h4-n29-instance-one-pair-audit-v1","status":"passed",
       "base_count":len(base),"distinct_pair_sums":len(pair_sums),
       "verified_targets":verified,"exhausted_targets":32-verified,
       "result_sha256":sha(HERE / "result.json")}
(HERE / "audit.json").write_text(json.dumps(audit,indent=2,sort_keys=True)+"\n")
print(json.dumps(audit,sort_keys=True))
