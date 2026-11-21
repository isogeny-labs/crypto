"""Independent finite-group, linear-system, and timing-record audit."""

import hashlib
import json
import statistics
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent
O=None


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def add(A,B):
    if A is O:return B
    if B is O:return A
    x1,y1=A;x2,y2=B
    if x1==x2 and (y1+y2)%47==0:return O
    slope=((3*x1*x1-1)*pow(2*y1,-1,47) if A==B else
           (y2-y1)*pow((x2-x1)%47,-1,47))%47
    x3=(slope*slope-x1-x2)%47
    return (x3,(slope*(x1-x3)-y1)%47)


def mul(n,A):
    R=O
    while n:
        if n&1:R=add(R,A)
        A=add(A,A);n//=2
    return R


def fold(A):
    B=(A[0],(-A[1])%47)
    return min(A,B),1 if A<=B else -1


def solve(A,b):
    rows=[[v%43 for v in row]+[value%43] for row,value in zip(A,b)]
    pivots=[]
    r=0
    for c in range(21):
        pivot=next((i for i in range(r,len(rows)) if rows[i][c]),None)
        if pivot is None:continue
        rows[r],rows[pivot]=rows[pivot],rows[r]
        inv=pow(rows[r][c],-1,43)
        rows[r]=[(x*inv)%43 for x in rows[r]]
        for i in range(len(rows)):
            if i==r:continue
            factor=rows[i][c]
            if factor:rows[i]=[(x-factor*y)%43 for x,y in zip(rows[i],rows[r])]
        pivots.append(c);r+=1
    assert len(pivots)==21 and all(any(row[:21]) or row[21]==0 for row in rows)
    solution=[0]*21
    for i,c in enumerate(pivots):solution[c]=rows[i][21]
    assert all(sum(x*y for x,y in zip(row,solution))%43==v%43 for row,v in zip(A,b))
    return solution


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(ROOT/"xedni_fixed_rank1_height_20261121"/"summary.json")==p["source_height_summary_sha256"]
    assert sha(ROOT/"xedni_endpoint_count_bound_20261121"/"result.json")==p["source_endpoint_result_sha256"]
    result=json.loads((HERE/"result.json").read_text())
    raw=(HERE/"arm-rows.jsonl").read_bytes()
    assert hashlib.sha256(raw).hexdigest()==result["arm_rows_sha256"]
    arms=[json.loads(s) for s in raw.splitlines()]
    assert len(arms)==6
    digest=hashlib.sha256(p["target_seed"].encode()).digest()
    scalar=1+int.from_bytes(digest[:8],"big")%42
    G=(17,10)
    target=mul(scalar,G)
    assert target==tuple(result["target"]) and scalar==result["fixture_scalar_for_replay"]
    assert mul(43,G) is O and len({mul(n,G) for n in range(1,43)})==42
    endpoints=[]
    for n in range(1,15):
        B=mul(n,G)
        for m in (2,6):endpoints.append((mul((-m*n)%43,G),B,m))
    keys=sorted({fold(P)[0] for P,B,m in endpoints}|{fold(B)[0] for P,B,m in endpoints})
    assert len(keys)==21
    col={xy:i for i,xy in enumerate(keys)}
    relations=[]
    for P,B,m in endpoints:
        row=[0]*21
        for point,weight in ((P,1),(B,m)):
            xy,sign=fold(point);row[col[xy]]+=sign*weight
        relations.append(row)
    anchor=[0]*21
    key,sign=fold(G);anchor[col[key]]=sign
    logs=solve(relations+[anchor],[0]*28+[1])
    key,sign=fold(target)
    assert sign*logs[col[key]]%43==scalar
    for rep in range(1,4):
        pair=[r for r in arms if r["repetition"]==rep]
        assert [r["arm"] for r in pair]==p["arm_order"][rep-1].split(",")
        assert [r["position"] for r in pair]==[1,2]
        for r in pair:
            assert r["status"]=="verified" and r["scalar"]==scalar
            assert tuple(r["target"])==target and mul(r["scalar"],G)==target
            assert r["start_ns"]<r["stop_ns"] and r["online_ns"]==r["stop_ns"]-r["start_ns"]
            if r["arm"]=="table":
                s=r["setup"]
                assert s["relation_rows"]==28 and s["folded_columns"]==21
                assert s["relation_rank"]==20 and s["anchored_rank"]==21
                assert s["max_coordinate_bits"]==7241
                assert s["setup_ns"]==sum(s["phases"].values())
                assert r["online_ns"]==r["target_query_ns"]+r["target_recovery_check_ns"]
            else:
                assert r["setup"] is None and r["walk_updates"]>0
    table=[r for r in arms if r["arm"]=="table"]
    rho=[r for r in arms if r["arm"]=="rho"]
    assert result["table_online_median_ns"]==statistics.median(r["online_ns"] for r in table)
    assert result["rho_online_median_ns"]==statistics.median(r["online_ns"] for r in rho)
    assert result["table_setup_median_ns"]==statistics.median(r["setup"]["setup_ns"] for r in table)
    assert result["table_cold_median_ns"]==statistics.median(r["setup"]["setup_ns"]+r["online_ns"] for r in table)
    print(json.dumps({"schema":"xedni-fixed-curve-target-accounting-audit-v1","status":"PASS",
                      "target":target,"verified_arms":len(arms),"anchored_rank":21,
                      "independent_scalar":scalar},indent=2,sort_keys=True))


if __name__=="__main__":main()
