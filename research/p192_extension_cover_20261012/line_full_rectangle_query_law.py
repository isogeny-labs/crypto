"""Exact frozen known-scalar membership in all implicit four-sum rectangles.

For two pivots a,b and shared endpoints u,v, the positive four-sum target
is 2S-a-b-U_u-U_v.  A batch-inversion pair lookup tests each frozen query
against the complete target family without materializing 9.65M rows.
"""

import hashlib
import json
import time
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import GX,GY,add,mul,neg,on_curve

HERE=Path(__file__).resolve().parent
sha=lambda path:hashlib.sha256(Path(path).read_bytes()).hexdigest()
full_path=HERE/"type_i_full_cross_pivot_cycle_rank.json"
guide_path=HERE/"type_i_line_guided_collector.json"
new_path=HERE/"type_i_line_full_new_quotient_points.json"
fan_names=["type_i_residual_pivot000_three.json",
           "type_i_residual_pivot001_cross.json",
           "type_i_residual_pivot002_cross.json"]
full=json.loads(full_path.read_text())
guide=json.loads(guide_path.read_text())
new=json.loads(new_path.read_text())
fans=[json.loads((HERE/name).read_text()) for name in fan_names]
p,r=int(full["field_prime"]),int(full["subgroup_order"])
assert p==int(guide["field_prime"]) and r==int(guide["subgroup_order"])
selected=[tuple(map(int,Q)) for Q in guide["base_points_fp"]+new]
assert len(selected)==len(set(selected))==9925
S=tuple(map(int,full["line_section_constant"]))
pivots=[tuple(map(int,fan["pivot_quotient_point"])) for fan in fans]
overlap={}
for a,b in combinations(range(3),2):
    left={row["selected_index"] for row in fans[a]["rows"]}
    right={row["selected_index"] for row in fans[b]["rows"]}
    indices=sorted(left&right)
    assert len(indices)==full["pair_overlap_counts"][f"{a}_{b}"]
    points=[selected[i] for i in indices]
    assert len(points)==len(set(points))
    overlap[a,b]=(indices,points,{Q:i for i,Q in zip(indices,points)})
G=(GX,GY)
assert on_curve(G,p) and mul(r,G,p) is None
assert on_curve(S,p)


def batch_differences(target,points):
    """Compute target-P for every prime-field affine P with one inversion."""
    if target is None:
        return [neg(P,p) for P in points]
    tx,ty=target
    denominators=[]
    indices=[]
    exceptional={}
    for index,(x,y) in enumerate(points):
        den=(x-tx)%p
        if den==0:
            exceptional[index]=add(target,neg((x,y),p),p)
        else:
            indices.append(index)
            denominators.append(den)
    products=[]
    product=1
    for den in denominators:
        product=product*den%p
        products.append(product)
    inverses=[0]*len(indices)
    if denominators:
        inverse=pow(product,-1,p)
        for k in range(len(indices)-1,-1,-1):
            inverses[k]=inverse*(products[k-1] if k else 1)%p
            inverse=inverse*denominators[k]%p
        assert inverse==1
    result=[None]*len(points)
    for index,Q in exceptional.items():result[index]=Q
    for index,inv in zip(indices,inverses):
        x,y=points[index]
        slope=(-y-ty)*inv%p
        rx=(slope*slope-tx-x)%p
        ry=(slope*(tx-rx)-ty)%p
        result[index]=(rx,ry)
    for index in sorted({0,len(points)//2,len(points)-1}):
        assert result[index]==add(target,neg(points[index],p),p)
    return result


start=time.monotonic_ns()
query_laws={}
for name,scalars in (("uniform",guide["uniform_scalars"]),
                     ("character_guided",guide["guided_scalars"])):
    hits=[]
    for index,encoded in enumerate(scalars):
        scalar=int(encoded)
        assert 0<scalar<r
        P=mul(scalar,G,p)
        Q=neg(add(P,P,p),p)
        if index%8==0:
            assert Q==mul(scalar,neg(add(G,G,p),p),p)
        for a,b in combinations(range(3),2):
            endpoints,points,lookup=overlap[a,b]
            target=add(add(add(add(S,S,p),neg(pivots[a],p),p),
                           neg(pivots[b],p),p),neg(Q,p),p)
            for endpoint,U,V in zip(endpoints,points,
                                    batch_differences(target,points)):
                other=lookup.get(V)
                if other is None or other<=endpoint:continue
                assert add(U,selected[other],p)==target
                # The cut-sum identity makes this exact positive four-sum.
                Ra=add(add(S,neg(pivots[a],p),p),neg(U,p),p)
                Rb=add(add(S,neg(pivots[b],p),p),
                       neg(selected[other],p),p)
                assert add(Ra,Rb,p)==Q
                hits.append({"query_index":index,"scalar":str(scalar),
                             "pivot_pair":[a,b],
                             "selected_endpoints":[endpoint,other]})
    query_laws[name]={
        "queries":len(scalars),
        "scalar_sha256":hashlib.sha256("\n".join(map(str,scalars)).encode()).hexdigest(),
        "implicit_four_sum_targets_per_query":full["implicit_rectangle_presentations"],
        "selected_point_subtractions_per_query":sum(len(v[0]) for v in overlap.values()),
        "hits":hits,
    }
    print("FULL_RECTANGLE_QUERY_PROGRESS",name,len(scalars),len(hits),flush=True)

receipt={
    "status":"verified_complete_rectangle_known_scalar_query_laws",
    "candidate_id":None,
    "field_prime":str(p),"subgroup_order":str(r),
    "implicit_rectangle_presentations":full["implicit_rectangle_presentations"],
    "query_laws":query_laws,
    "elapsed_ns_unisolated":time.monotonic_ns()-start,
    "input_sha256":{path.name:sha(path) for path in
                    (full_path,guide_path,new_path)+tuple(HERE/name for name in fan_names)},
    "source_sha256":sha(__file__),
    "boundary":"Exact membership test against every implicit rectangle target for each frozen known-scalar query. The batch inversion avoids materializing the 9.65M rows; elapsed wall time is unisolated. Target-descent and relation-stage costs are separate.",
}
path=HERE/"type_i_full_rectangle_query_law.json"
path.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n")
print("FULL_RECTANGLE_QUERY_PASS",{name:len(law["hits"])
                                   for name,law in query_laws.items()},flush=True)
