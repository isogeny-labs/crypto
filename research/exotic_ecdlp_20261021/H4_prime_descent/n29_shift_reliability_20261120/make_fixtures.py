#!/usr/bin/env python3
"""Freeze 256 subgroup targets disjoint from previous n29 queries and target panels."""
from sage.all import *
import hashlib, json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
P=ROOT/"research/exotic_ecdlp_20261021/H4_prime_descent"
C=json.loads((P/"n29_full_pipeline_20261120/candidate.json").read_text())
PREP=json.loads((P/"n29_full_pipeline_20261120/result.json").read_text())
PANEL=json.loads((P/"n29_local_panel_20261120/fixtures.json").read_text())
A=json.loads((P/"n29_instance1_pair_20261120/result.json").read_text())
B=json.loads((P/"n29_instance1_yield_20261120/result.json").read_text())
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
field=C["record"]["field"];n,poly=field["n"],field["defining_polynomial_int"]
F2=GF(2);PR=PolynomialRing(F2,"z");z=PR.gen()
mod=sum(F2((poly>>i)&1)*z**i for i in range(n+1))
K=GF(1<<n,"z",modulus=mod)
def elt(v):return K([(v>>i)&1 for i in range(n)])
def enc(e):return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
def rec(Q):return [enc(Q[0]),enc(Q[1])]
coeff=C["record"]["curve"]["coefficients"]
E=EllipticCurve(K,[1,elt(coeff["a2"]),0,0,elt(coeff["a6"])])
G=E(elt(C["record"]["curve"]["G"][0]),elt(C["record"]["curve"]["G"][1]))
r=C["record"]["curve"]["r"]
assert r*G==E(0)
used={x["known_scalar"] for x in PREP["relation_trace"]}
used.add(PREP["target_fixture_scalar"])
used.update(x["fixture_scalar"] for x in PANEL["targets"])
used.update(x["fixture_scalar"] for x in A["targets"])
used.update(x["fixture_scalar"] for x in B["targets"])
excluded=len(used)
targets=[]
for i in range(256):
    counter=0
    while True:
        label=f"h4-n29-shift-reliability-target-{i}-{counter}"
        scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),"big")%(r-1)+1
        if scalar not in used:break
        counter+=1
    used.add(scalar)
    Q=scalar*G
    workload={"curve_id":C["record"]["curve"]["curve_id"],
              "G":rec(G),"target":rec(Q),"target_count":1,
              "target_law":"sha256_nonzero_scalar_times_G_excluding_prior_n29_queries_and_targets",
              "target_label":label,"precomputation_state":"ready"}
    targets.append({"index":i,"fixture_scalar":scalar,"target":rec(Q),
                    "target_point_sha256":digest(rec(Q)),"target_label":label,
                    "workload_id":"W"+digest(workload)[:12],"workload_record":workload})
fixture={"schema":"h4-n29-shift-reliability-fixture-v1",
         "candidate_id":C["candidate_id"],"curve_id":C["record"]["curve"]["curve_id"],
         "generator":rec(G),"subgroup_order":r,"excluded_prior_scalars":excluded,
         "selection_law":"first SHA256-derived nonzero scalar per indexed label excluding all prior n29 relation and target scalars and earlier selected targets",
         "targets":targets}
(HERE/"fixtures.json").write_text(json.dumps(fixture,indent=2,sort_keys=True)+"\n")
print(json.dumps({"candidate_id":fixture["candidate_id"],"target_count":len(targets),
                  "excluded_prior_scalars":excluded,"first_target":targets[0]["target"]},sort_keys=True))
