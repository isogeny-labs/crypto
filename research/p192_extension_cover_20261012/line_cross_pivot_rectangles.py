"""Cross-pivot rectangles among reduced P-192 quartic quotient classes.

Each fan edge is a split line through a pivot and a selected endpoint.  Four
edges from two pivots and two endpoints cancel their shared line-section
constant, both pivots, and both endpoints.  The resulting eight-term row
uses only fixed-origin degree-three classes already reduced in frozen runs.
"""

import hashlib
import json
from collections import Counter, defaultdict, deque
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import GX, GY, add, key, mul, neg, on_curve

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
fan_names = ["type_i_residual_pivot000_three.json",
             "type_i_residual_pivot001_cross.json",
             "type_i_residual_pivot002_cross.json"]
reduction_names = ["type_i_two_pivot_reduced64.json",
                   "type_i_pivot002_reduced32.json"]
source_names = ["type_i_line_guided_collector.json",
                "type_i_line_full_new_quotient_points.json",
                "type_i_base_points_legacy_fixture.json"]
fans = [json.loads((HERE/name).read_text()) for name in fan_names]
first_two = json.loads((HERE/reduction_names[0]).read_text())
third = json.loads((HERE/reduction_names[1]).read_text())
guide = json.loads((HERE/source_names[0]).read_text())
new_selected = json.loads((HERE/source_names[1]).read_text())
p,r = int(guide["field_prime"]),int(guide["subgroup_order"])
assert all(int(f["field_prime"])==p and int(f["subgroup_order"])==r
           for f in fans)
assert p==int(first_two["field_prime"])==int(third["field_prime"])
assert r==int(first_two["subgroup_order"])==int(third["subgroup_order"])
selected = [tuple(map(int,Q)) for Q in guide["base_points_fp"]+new_selected]
assert len(selected)==9925 and all(on_curve(Q,p) for Q in selected[:141])
S = None
edges = []
columns = []
for pivot in range(3):
    fan = fans[pivot]
    pivot_Q = tuple(map(int,fan["pivot_quotient_point"]))
    rows = (first_two["rows"][pivot*32:(pivot+1)*32]
            if pivot<2 else third["rows"])
    reps = (first_two["representatives"][pivot*32:(pivot+1)*32]
            if pivot<2 else third["representatives"])
    assert len(rows)==len(reps)==32
    assert all(rep["status"]=="verified_fixed_origin_degree_three"
               for rep in reps)
    for local in range(16):
        a,b=rows[2*local:2*local+2]
        assert a["pivot"]==b["pivot"]==pivot
        assert a["fan_row_index"]==b["fan_row_index"]
        assert (a["residual_side"],b["residual_side"])==(0,1)
        source_row=fan["rows"][a["fan_row_index"]]
        endpoint=int(source_row["selected_index"])
        Qa,Qb=(tuple(map(int,row["quotient"])) for row in (a,b))
        assert [Qa,Qb]==[tuple(map(int,Q)) for Q in
                         source_row["projected_residuals"]]
        assert on_curve(Qa,p) and on_curve(Qb,p)
        line_sum=add(add(add(pivot_Q,selected[endpoint],p),Qa,p),Qb,p)
        if S is None:S=line_sum
        assert line_sum==S
        columns.extend((Qa,Qb))
        edges.append({"pivot":pivot,"endpoint":endpoint,
                      "fan_row_index":a["fan_row_index"],
                      "column_indices":[2*len(edges),2*len(edges)+1]})
assert len(edges)==48 and len(columns)==len(set(columns))==96
assert all(mul(r,Q,p) is None for Q in columns)


def rank(vectors):
    pivots={}
    for raw in vectors:
        v=[x%r for x in raw]
        for k,q in sorted(pivots.items()):
            if v[k]:
                t=v[k]
                v=[(x-t*y)%r for x,y in zip(v,q)]
        k=next((i for i,x in enumerate(v) if x),None)
        if k is not None:
            inverse=pow(v[k],-1,r)
            pivots[k]=[x*inverse%r for x in v]
    return len(pivots)


def case(name,edge_limit,exclude_source=False):
    chosen=[e for e in edges if e["pivot"]<edge_limit[0]
            and (edge_limit[1] is None or
                 e["column_indices"][0]//32<edge_limit[0]
                 and (e["column_indices"][0]%32)//2<edge_limit[1])
            and (not exclude_source or e["endpoint"] not in
                 set(fans[e["pivot"]]["source_line_selected_pair"]))]
    edge_by_key={(e["pivot"],e["endpoint"]):e for e in chosen}
    endpoint_by_pivot=[{e["endpoint"] for e in chosen if e["pivot"]==v}
                       for v in range(edge_limit[0])]
    local_columns=sorted({k for e in chosen for k in e["column_indices"]})
    remap={k:i for i,k in enumerate(local_columns)}
    equations=[]
    witnesses=[]
    targets=[]
    for u,v in combinations(range(edge_limit[0]),2):
        for j,k in combinations(sorted(endpoint_by_pivot[u]&endpoint_by_pivot[v]),2):
            # R_u,j + R_v,k = R_u,k + R_v,j, where R is the
            # projected sum of the two residual classes on a split cut.
            row=[0]*len(local_columns)
            positive=[]
            negative=[]
            for pivot,endpoint,sign in ((u,j,1),(v,k,1),(u,k,-1),(v,j,-1)):
                edge=edge_by_key[pivot,endpoint]
                for global_index in edge["column_indices"]:
                    row[remap[global_index]]+=sign
                    (positive if sign==1 else negative).append(columns[global_index])
            pos=sum_group(positive)
            assert pos==sum_group(negative)
            equations.append(row)
            witnesses.append([u,v,j,k])
            targets.append(pos)
    degrees=[sum(row[k]!=0 for row in equations)
             for k in range(len(local_columns))]
    active={k for k,d in enumerate(degrees) if d}
    row_neighbors=[{k for k,x in enumerate(row) if x} for row in equations]
    col_neighbors=[{j for j,ns in enumerate(row_neighbors) if k in ns}
                   for k in range(len(local_columns))]
    core_rows=set(range(len(equations)))
    core_cols=set(active)
    queue=deque(k for k in active if len(col_neighbors[k])<2)
    while queue:
        k=queue.popleft()
        if k not in core_cols or len(col_neighbors[k]&core_rows)>=2:continue
        core_cols.remove(k)
        for j in col_neighbors[k]&core_rows:
            core_rows.remove(j)
            for l in row_neighbors[j]&core_cols:
                if len(col_neighbors[l]&core_rows)<2:queue.append(l)
    incidence_nodes=set()
    for e in chosen:
        incidence_nodes.add(("p",e["pivot"]))
        incidence_nodes.add(("s",e["endpoint"]))
    parent={v:v for v in incidence_nodes}
    def find(v):
        while parent[v]!=v:
            parent[v]=parent[parent[v]];v=parent[v]
        return v
    for e in chosen:
        parent[find(("p",e["pivot"]))]=find(("s",e["endpoint"]))
    components=len({find(v) for v in parent})
    cycle_rank=len(chosen)-len(incidence_nodes)+components
    actual_rank=rank(equations)
    assert actual_rank==cycle_rank
    query_hits={}
    G=(GX,GY)
    assert on_curve(G,p) and mul(r,G,p) is None
    target_map=defaultdict(list)
    for i,Q in enumerate(targets):target_map[Q].append(i)
    for label,scalars in (("uniform",guide["uniform_scalars"]),
                          ("character_guided",guide["guided_scalars"])):
        hits=[]
        for i,encoded in enumerate(scalars):
            a=int(encoded)
            assert 0<a<r
            P=mul(a,G,p)
            Q=neg(add(P,P,p),p)
            if Q in target_map:
                hits.append([i,str(a),target_map[Q]])
        query_hits[label]={"queries":len(scalars),"hits":hits}
    return {
        "name":name,"edges":len(chosen),"selected_endpoints":
            len({e["endpoint"] for e in chosen}),
        "incidence_vertices":len(incidence_nodes),
        "incidence_components":components,
        "cycle_rank":cycle_rank,"matrix_rank":actual_rank,
        "relation_rows":len(equations),
        "total_columns":len(local_columns),
        "active_columns":len(active),
        "active_column_min_degree":min((degrees[k] for k in active),default=0),
        "two_core_rows":len(core_rows),"two_core_columns":len(core_cols),
        "edge_selected_indices":[e["endpoint"] for e in chosen],
        "column_indices_global":local_columns,
        "rectangle_witnesses":witnesses,
        "rectangle_rows":equations,
        "rectangle_target_keys":[list(Q) if Q else None for Q in targets],
        "distinct_rectangle_targets":len(target_map),
        "known_scalar_query_laws":query_hits,
    }


def sum_group(points):
    total=None
    for point in points:total=add(total,point,p)
    return total


cases=[case("two_pivots_32_each",(2,None)),
       case("three_pivots_16_each",(3,8)),
       case("three_pivots_32_each",(3,None)),
       case("three_pivots_32_each_non_source",(3,None),True)]
out={
    "status":"verified_p192_cross_pivot_rectangle_rank",
    "candidate_id":None,
    "field_prime":str(p),"subgroup_order":str(r),
    "line_section_constant":list(S) if S else None,
    "edges":edges,
    "quotient_columns":[list(Q) for Q in columns],
    "cases":cases,
    "input_sha256":{name:sha(HERE/name) for name in
                    fan_names+reduction_names+source_names},
    "source_sha256":sha(__file__),
    "boundary":"All rows are exact four-cut identities among independently reduced degree-three classes. Matrix ranks constrain those base-class logarithms; the 64-query laws only test the rectangle four-sum targets shown here. Full relation collection and target descent remain separate measurements.",
}
path=HERE/"type_i_cross_pivot_rectangles.json"
path.write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
for c in cases:
    print("CROSS_PIVOT_RECTANGLE_PASS",c["name"],
          "edges",c["edges"],"rows",c["relation_rows"],
          "rank",c["matrix_rank"],"core",c["two_core_rows"],
          c["two_core_columns"],"queries",
          {k:len(v["hits"]) for k,v in c["known_scalar_query_laws"].items()},
          flush=True)
