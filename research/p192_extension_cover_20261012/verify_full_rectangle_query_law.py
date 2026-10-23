"""Independent chunked x-first replay of all frozen rectangle query tests."""

import hashlib
import json
from collections import defaultdict
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import GX,GY,add,mul,neg,on_curve

HERE=Path(__file__).resolve().parent
path=HERE/"type_i_full_rectangle_query_law.json"
receipt=json.loads(path.read_text())
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert receipt["source_sha256"]==sha(HERE/"line_full_rectangle_query_law.py")
for name,digest in receipt["input_sha256"].items():
    assert sha(HERE/name)==digest
full=json.loads((HERE/"type_i_full_cross_pivot_cycle_rank.json").read_text())
guide=json.loads((HERE/"type_i_line_guided_collector.json").read_text())
new=json.loads((HERE/"type_i_line_full_new_quotient_points.json").read_text())
fans=[json.loads((HERE/name).read_text()) for name in (
    "type_i_residual_pivot000_three.json",
    "type_i_residual_pivot001_cross.json",
    "type_i_residual_pivot002_cross.json")]
p,r=int(full["field_prime"]),int(full["subgroup_order"])
selected=[tuple(map(int,Q)) for Q in guide["base_points_fp"]+new]
assert len(selected)==9925 and len(set(selected))==9925
S=tuple(map(int,full["line_section_constant"]))
G=(GX,GY)
assert on_curve(S,p) and on_curve(G,p)
pair_data={}
for a,b in combinations(range(3),2):
    left={row["selected_index"] for row in fans[a]["rows"]}
    right={row["selected_index"] for row in fans[b]["rows"]}
    endpoints=sorted(left&right)
    points=[selected[j] for j in endpoints]
    by_x=defaultdict(list)
    for j,P in zip(endpoints,points):by_x[P[0]].append((j,P))
    pair_data[a,b]=(endpoints,points,by_x)
    assert len(endpoints)==full["pair_overlap_counts"][f"{a}_{b}"]


def find_pairs(target,endpoints,points,by_x):
    """Chunked reciprocal lookup, filtering by x before computing y."""
    hits=[]
    for start in range(0,len(points),96):
        chunk=points[start:start+96]
        ids=endpoints[start:start+96]
        if target is None:
            for j,P in zip(ids,chunk):
                V=neg(P,p)
                for k,Q in by_x.get(V[0],()):
                    if k>j and Q==V:hits.append((j,k))
            continue
        tx,ty=target
        generic=[]
        exceptional=[]
        for j,(x,y) in zip(ids,chunk):
            den=(x-tx)%p
            if den:generic.append((j,x,y,den))
            else:exceptional.append((j,(x,y)))
        prefix=[1]
        for _,_,_,den in generic:prefix.append(prefix[-1]*den%p)
        inverse=pow(prefix[-1],-1,p)
        reciprocal=[0]*len(generic)
        for i in range(len(generic)-1,-1,-1):
            reciprocal[i]=inverse*prefix[i]%p
            inverse=inverse*generic[i][3]%p
        assert inverse==1
        for (j,x,y,_),inv in zip(generic,reciprocal):
            slope=(-y-ty)*inv%p
            rx=(slope*slope-tx-x)%p
            candidates=by_x.get(rx,())
            if candidates:
                ry=(slope*(tx-rx)-ty)%p
                for k,Q in candidates:
                    if k>j and Q==(rx,ry):hits.append((j,k))
        for j,P in exceptional:
            V=add(target,neg(P,p),p)
            if V is not None:
                for k,Q in by_x.get(V[0],()):
                    if k>j and Q==V:hits.append((j,k))
    return hits


assert receipt["implicit_rectangle_presentations"]== \
    full["implicit_rectangle_presentations"]==9652897
summary={}
for name,law in receipt["query_laws"].items():
    scalars=guide["uniform_scalars" if name=="uniform" else "guided_scalars"]
    assert law["queries"]==len(scalars)==64
    assert law["scalar_sha256"]==hashlib.sha256(
        "\n".join(map(str,scalars)).encode()).hexdigest()
    hits=[]
    for index,encoded in enumerate(scalars):
        a=int(encoded)
        assert 0<a<r
        P=mul(a,G,p)
        Q=neg(add(P,P,p),p)
        for i,j in combinations(range(3),2):
            endpoints,points,by_x=pair_data[i,j]
            piv_i=tuple(map(int,fans[i]["pivot_quotient_point"]))
            piv_j=tuple(map(int,fans[j]["pivot_quotient_point"]))
            T=add(add(add(add(S,S,p),neg(piv_i,p),p),
                          neg(piv_j,p),p),neg(Q,p),p)
            for u,v in find_pairs(T,endpoints,points,by_x):
                assert add(selected[u],selected[v],p)==T
                hits.append({"query_index":index,"scalar":str(a),
                             "pivot_pair":[i,j],
                             "selected_endpoints":[u,v]})
    assert hits==law["hits"]
    summary[name]={"queries":len(scalars),"hits":len(hits)}
    print("FULL_RECTANGLE_QUERY_REPLAY_PROGRESS",name,len(hits),flush=True)

out={
    "status":"independent_full_rectangle_query_replay_pass",
    "producer_sha256":sha(path),
    "source_sha256":sha(__file__),
    "laws":summary,
}
output=HERE/"type_i_full_rectangle_query_law_replay.json"
output.write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
print("FULL_RECTANGLE_QUERY_REPLAY_PASS",json.dumps(summary,sort_keys=True),flush=True)
