"""Single-public-target table accounting with same-point Floyd rho."""

import hashlib
import json
import os
import platform
import resource
import statistics
import time
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, matrix, vector

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fold(P):
    xy=tuple(map(int,P.xy()))
    neg=tuple(map(int,(-P).xy()))
    return min(xy,neg),1 if xy<=neg else -1


def build_table(protocol):
    t0=time.monotonic_ns()
    E=EllipticCurve(QQ,[QQ(s) for s in protocol["rational_curve_a_invariants"]])
    Q=E([QQ(s) for s in protocol["rational_point_Q"]])
    E0=EllipticCurve(GF(47),protocol["reduced_curve_a_invariants"])
    G=E0([GF(47)(s) for s in protocol["rational_point_Q"]])
    assert E0.cardinality()==43 and G.order()==43
    t1=time.monotonic_ns()
    raw=[]
    endpoint_keys=set()
    max_bits=0
    for n in range(1,protocol["rational_multiple_prefix"]+1):
        B=n*Q
        B0=E0([GF(47)(v) for v in B.xy()])
        max_bits=max(max_bits,*[max(abs(v.numerator()).nbits(),v.denominator().nbits()) for v in B.xy()])
        for m in protocol["relation_coefficients"]:
            P=-m*B
            assert P+m*B==E(0)
            P0=E0([GF(47)(v) for v in P.xy()])
            assert P0+m*B0==E0(0)
            raw.append((P0,B0,m))
            endpoint_keys.update((fold(P0)[0],fold(B0)[0]))
            max_bits=max(max_bits,*[max(abs(v.numerator()).nbits(),v.denominator().nbits()) for v in P.xy()])
    assert len(raw)==28 and len(endpoint_keys)==21
    t2=time.monotonic_ns()
    columns=sorted(endpoint_keys)
    col={xy:i for i,xy in enumerate(columns)}
    relations=[]
    for P,B,m in raw:
        row=[0]*21
        for point,c in ((P,1),(B,m)):
            xy,sign=fold(point)
            row[col[xy]]+=sign*c
        relations.append(row)
    A=matrix(GF(43),relations)
    assert A.rank()==20
    anchor_key,anchor_sign=fold(G)
    anchor=[0]*21
    anchor[col[anchor_key]]=anchor_sign
    A=A.stack(matrix(GF(43),[anchor]))
    rhs=vector(GF(43),[0]*28+[1])
    assert A.rank()==21
    t3=time.monotonic_ns()
    logs=A.solve_right(rhs)
    assert A*logs==rhs
    t4=time.monotonic_ns()
    table={xy:int(logs[i]) for i,xy in enumerate(columns)}
    assert len(table)==21
    t5=time.monotonic_ns()
    phases={"curve_setup_ns":t1-t0,"relation_generation_ns":t2-t1,
            "matrix_build_ns":t3-t2,"relation_LA_ns":t4-t3,"table_build_ns":t5-t4}
    assert sum(phases.values())==t5-t0
    return E0,G,table,{"phases":phases,"setup_ns":t5-t0,
                      "relation_rows":len(raw),"folded_columns":len(columns),
                      "relation_rank":20,"anchored_rank":21,"max_coordinate_bits":max_bits}


def table_solve(G,T,table):
    start=time.monotonic_ns()
    key,sign=fold(T)
    k=(sign*table[key])%43
    query_stop=time.monotonic_ns()
    assert k*G==T
    stop=time.monotonic_ns()
    return {"status":"verified","scalar":k,"start_ns":start,"stop_ns":stop,
            "online_ns":stop-start,"target_query_ns":query_stop-start,
            "target_recovery_check_ns":stop-query_stop}


def rho_solve(E,G,T,pair_index):
    start=time.monotonic_ns()
    updates=0
    for restart in range(1,44):
        seed=pair_index*17+restart
        a=seed%43
        b=(7*seed+3)%43
        X=a*G+b*T
        def step(state):
            nonlocal updates
            P,u,v=state
            choice=0 if P==E(0) else int(P[0])%3
            updates+=1
            if choice==0:return (P+G,(u+1)%43,v)
            if choice==1:return (P+T,u,(v+1)%43)
            return (2*P,(2*u)%43,(2*v)%43)
        slow=(X,a,b);fast=(X,a,b)
        for iteration in range(1,10001):
            slow=step(slow);fast=step(step(fast))
            if slow[0]!=fast[0]:continue
            den=(slow[2]-fast[2])%43
            if den==0:break
            k=(fast[1]-slow[1])*pow(den,-1,43)%43
            assert k*G==T
            stop=time.monotonic_ns()
            return {"status":"verified","scalar":k,"seed_mod_43":seed%43,
                    "walk_updates":updates,"iterations":iteration,"restarts":restart-1,
                    "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    stop=time.monotonic_ns()
    return {"status":"rho_cap_exhausted","scalar":None,"walk_updates":updates,
            "restarts":43,"start_ns":start,"stop_ns":stop,"online_ns":stop-start}


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(ROOT/"xedni_fixed_rank1_height_20261121"/"summary.json")==p["source_height_summary_sha256"]
    assert sha(ROOT/"xedni_endpoint_count_bound_20261121"/"result.json")==p["source_endpoint_result_sha256"]
    digest=hashlib.sha256(p["target_seed"].encode()).digest()
    fixture_k=1+int.from_bytes(digest[:8],"big")%42
    Ef=EllipticCurve(GF(47),p["reduced_curve_a_invariants"])
    Gf=Ef([GF(47)(s) for s in p["rational_point_Q"]])
    target=fixture_k*Gf
    target_xy=tuple(map(int,target.xy()))
    records=[]
    for rep,order in enumerate(p["arm_order"],1):
        E,G,table,setup=build_table(p)
        T=E(target_xy)
        for position,arm in enumerate(order.split(","),1):
            result=table_solve(G,T,table) if arm=="table" else rho_solve(E,G,T,rep)
            records.append({"repetition":rep,"position":position,"arm":arm,
                            "target":target_xy,"setup":setup if arm=="table" else None,**result})
    assert all(r["status"]=="verified" and r["scalar"]==fixture_k for r in records)
    raw="".join(json.dumps(r,sort_keys=True)+"\n" for r in records).encode()
    (HERE/"arm-rows.jsonl").write_bytes(raw)
    table_rows=[r for r in records if r["arm"]=="table"]
    rho_rows=[r for r in records if r["arm"]=="rho"]
    return {"schema":"xedni-fixed-curve-target-accounting-result-v1","status":"PASS",
            "target":target_xy,"fixture_scalar_for_replay":fixture_k,
            "repetitions":len(table_rows),"verified_arms":len(records),
            "arm_rows_sha256":hashlib.sha256(raw).hexdigest(),
            "table_online_median_ns":statistics.median(r["online_ns"] for r in table_rows),
            "rho_online_median_ns":statistics.median(r["online_ns"] for r in rho_rows),
            "table_setup_median_ns":statistics.median(r["setup"]["setup_ns"] for r in table_rows),
            "table_cold_median_ns":statistics.median(r["setup"]["setup_ns"]+r["online_ns"] for r in table_rows),
            "host":{"system":platform.system(),"machine":platform.machine(),
                    "cpu_count":os.cpu_count(),"maxrss":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__=="__main__":
    print(json.dumps(main(),indent=2,sort_keys=True))
