"""Independent graph and column-pivot rank replay of complete fan basis."""

import hashlib
import json
from collections import Counter
from itertools import combinations
from math import comb
from pathlib import Path

from verify_adaptive_private_columns import add, on_curve

HERE=Path(__file__).resolve().parent
path=HERE/"type_i_full_cross_pivot_cycle_rank.json"
source=json.loads(path.read_text())
sha=lambda item:hashlib.sha256(Path(item).read_bytes()).hexdigest()
assert source["source_sha256"]==sha(HERE/"line_full_cross_pivot_cycle_rank.py")
for name,digest in source["input_sha256"].items():
    assert sha(HERE/name)==digest
p,r=int(source["field_prime"]),int(source["subgroup_order"])
names=["type_i_residual_pivot000_three.json",
       "type_i_residual_pivot001_cross.json",
       "type_i_residual_pivot002_cross.json"]
fans=[json.loads((HERE/name).read_text()) for name in names]
incidence={}
quotient_ids={}
duplicate=[]
endpoint_sets=[]
for pivot,fan in enumerate(fans):
    endpoints=set()
    for i,row in enumerate(fan["rows"]):
        endpoint=int(row["selected_index"])
        assert endpoint not in endpoints
        endpoints.add(endpoint)
        ids=[]
        for side,encoded in enumerate(row["projected_residuals"]):
            Q=tuple(map(int,encoded))
            assert on_curve(Q,p)
            if Q in quotient_ids:
                duplicate.append([quotient_ids[Q][1],[pivot,i,side]])
            else:
                quotient_ids[Q]=(len(quotient_ids),[pivot,i,side])
            ids.append(quotient_ids[Q][0])
        assert ids[0]!=ids[1]
        incidence[pivot,endpoint]=ids
    endpoint_sets.append(endpoints)
assert len(incidence)==source["split_cut_edges"]==15034
assert len(quotient_ids)==source["exact_distinct_projected_columns"]==30065
assert len(duplicate)==len(source["duplicate_projected_presentations"])==3
assert len(set.union(*endpoint_sets))==source["selected_endpoints_union"]==8718
overlaps={pair:endpoint_sets[pair[0]]&endpoint_sets[pair[1]]
          for pair in combinations(range(3),2)}
assert {f"{a}_{b}":len(v) for (a,b),v in overlaps.items()}== \
    source["pair_overlap_counts"]
triple=set.intersection(*endpoint_sets)
assert len(triple)==source["triple_overlap_count"]==1295
anchor=min(triple)
assert anchor==source["common_anchor_endpoint"]
assert sum(comb(len(v),2) for v in overlaps.values())== \
    source["implicit_rectangle_presentations"]==9652897

# A graph edge closes a cycle precisely when its endpoints are already
# connected.  This counts the cycle-space dimension independently of the
# edge/vertex formula in the producer.
nodes={("p",pivot) for pivot in range(3)}
nodes|={("s",j) for j in set.union(*endpoint_sets)}
parent={v:v for v in nodes}


def find(v):
    while parent[v]!=v:
        parent[v]=parent[parent[v]]
        v=parent[v]
    return v


non_tree=0
for pivot,endpoint in sorted(incidence):
    a,b=find(("p",pivot)),find(("s",endpoint))
    if a==b:non_tree+=1
    else:parent[a]=b
assert len({find(v) for v in parent})==1
assert non_tree==source["combinatorial_cycle_rank"]==6314

expected=[]
for pivot_pair in ((0,1),(0,2)):
    expected.extend([list((*pivot_pair,j,anchor))
                     for j in sorted(overlaps[pivot_pair]-{anchor})])
expected.extend([[1,2,j,anchor]
                 for j in sorted(overlaps[1,2]-endpoint_sets[0])])
assert expected==source["basis_witnesses"]
assert len(expected)==source["basis_rows"]==non_tree
assert hashlib.sha256(json.dumps(expected,separators=(",",":"),
                                 sort_keys=True).encode()).hexdigest()== \
    source["basis_witnesses_sha256"]


row_vectors=[]
column_occurrence=Counter()
for u,v,j,k in expected:
    row=Counter()
    for q,t,sign in ((u,j,1),(v,k,1),(u,k,-1),(v,j,-1)):
        for index in incidence[q,t]:row[index]+=sign
    normalized={i:value%r for i,value in row.items() if value%r}
    row_vectors.append(normalized)
    column_occurrence.update(normalized.keys())
private=source["basis_private_pivot_columns"]
assert len(private)==len(set(private))==len(row_vectors)==6314
for index,(pivot,row) in enumerate(zip(private,row_vectors)):
    assert row.get(pivot,0)!=0
    assert column_occurrence[pivot]==1
rank=len(private)
assert rank==source["coalesced_projected_column_rank"]==6314
active_edges=[(a,j) for a,j in incidence
              if sum(j in group for group in endpoint_sets)>1]
active_ids={index for edge in active_edges for index in incidence[edge]}
assert len(active_edges)==source["active_edge_presentations"]==11337
assert len(active_ids)==source["active_distinct_projected_columns"]
minimum=min(sum(len(overlaps[min(a,b),max(a,b)])-1
                for b in range(3)
                if b!=a and j in endpoint_sets[b])
            for a,j in active_edges)
assert minimum==source["minimum_active_column_degree_in_implicit_rows"]
assert minimum>=2

non_source=source["non_source_policy"]
kept=[set(group)-set(fans[pivot]["source_line_selected_pair"])
      for pivot,group in enumerate(endpoint_sets)]
assert [len(group) for group in kept]==non_source["fan_non_source_rows"]
assert sum(map(len,kept))==non_source["split_cut_edges"]==15028
pair_kept={pair:kept[pair[0]]&kept[pair[1]]
           for pair in combinations(range(3),2)}
assert {f"{a}_{b}":len(group) for (a,b),group in pair_kept.items()}== \
    non_source["pair_overlap_counts"]
assert len(set.intersection(*kept))==non_source["triple_overlap_count"]
assert sum(comb(len(group),2) for group in pair_kept.values())== \
    non_source["implicit_rectangle_presentations"]==9635113
local_anchor=min(set.intersection(*kept))
assert local_anchor==non_source["common_anchor_endpoint"]
local=[]
for a,b in ((0,1),(0,2)):
    local.extend([[a,b,j,local_anchor]
                  for j in sorted(pair_kept[a,b]-{local_anchor})])
local.extend([[1,2,j,local_anchor]
              for j in sorted(pair_kept[1,2]-kept[0])])
assert local==non_source["basis_witnesses"]
assert len(local)==non_source["basis_rank"]==6309
local_rows=[]
local_occurrence=Counter()
for u,v,j,k in local:
    row=Counter()
    for a,t,sign in ((u,j,1),(v,k,1),(u,k,-1),(v,j,-1)):
        for index in incidence[a,t]:row[index]+=sign
    normalized={i:value%r for i,value in row.items() if value%r}
    local_rows.append(normalized)
    local_occurrence.update(normalized.keys())
local_pivots=non_source["basis_private_pivot_columns"]
assert len(local_pivots)==len(set(local_pivots))==len(local_rows)
assert all(row.get(pivot,0) and local_occurrence[pivot]==1
           for pivot,row in zip(local_pivots,local_rows))
local_columns=[index for a,j in incidence
               if j in kept[a] for index in incidence[a,j]]
assert len(local_columns)==non_source["projected_column_presentations"]==30056
assert len(set(local_columns))== \
    non_source["exact_distinct_projected_columns"]==30056
local_active=[(a,j) for a,j in incidence
              if j in kept[a] and sum(j in group for group in kept)>1]
local_active_ids={index for edge in local_active for index in incidence[edge]}
assert len(local_active_ids)== \
    non_source["active_distinct_projected_columns"]==22658

# A different deterministic spread of line and rectangle equations is
# evaluated directly on the integer P-192 group.
selected_guide=json.loads((HERE/"type_i_line_guided_collector.json").read_text())
selected_new=json.loads((HERE/"type_i_line_full_new_quotient_points.json").read_text())
selected=[tuple(map(int,Q)) for Q in selected_guide["base_points_fp"]+selected_new]
S=tuple(map(int,source["line_section_constant"]))
line_checks=[]
for pivot,fan in enumerate(fans):
    pivot_Q=tuple(map(int,fan["pivot_quotient_point"]))
    for i in sorted({(i*len(fan["rows"])//37)%len(fan["rows"])
                     for i in range(37)}):
        row=fan["rows"][i]
        Qs=[tuple(map(int,Q)) for Q in row["projected_residuals"]]
        Q=add(add(add(pivot_Q,selected[row["selected_index"]],p),Qs[0],p),Qs[1],p)
        assert Q==S
        line_checks.append([pivot,i])
assert len(line_checks)==111
rect_checks=[]
for i in sorted({(i*len(expected)//71)%len(expected) for i in range(71)}):
    u,v,j,k=expected[i]
    left=[tuple(map(int,Q)) for Q in
          fans[u]["rows"][next(a for a,row in enumerate(fans[u]["rows"])
                                      if row["selected_index"]==j)]
          ["projected_residuals"]]
    left += [tuple(map(int,Q)) for Q in
             fans[v]["rows"][next(a for a,row in enumerate(fans[v]["rows"])
                                         if row["selected_index"]==k)]
             ["projected_residuals"]]
    right=[tuple(map(int,Q)) for Q in
           fans[u]["rows"][next(a for a,row in enumerate(fans[u]["rows"])
                                       if row["selected_index"]==k)]
           ["projected_residuals"]]
    right += [tuple(map(int,Q)) for Q in
              fans[v]["rows"][next(a for a,row in enumerate(fans[v]["rows"])
                                          if row["selected_index"]==j)]
              ["projected_residuals"]]
    total_left=total_right=None
    for Q in left:total_left=add(total_left,Q,p)
    for Q in right:total_right=add(total_right,Q,p)
    assert total_left==total_right
    rect_checks.append(i)
assert len(rect_checks)==71

receipt={
    "status":"independent_full_cross_pivot_cycle_replay_pass",
    "producer_sha256":sha(path),
    "source_sha256":sha(__file__),
    "graph_non_tree_edges":non_tree,
    "independent_private_column_rank_certificate":rank,
    "active_distinct_columns":len(active_ids),
    "minimum_implicit_column_degree":minimum,
    "non_source_rank_certificate":len(local_pivots),
    "non_source_active_columns":len(local_active_ids),
    "sampled_line_checks":line_checks,
    "sampled_rectangle_checks":rect_checks,
}
output=HERE/"type_i_full_cross_pivot_cycle_rank_replay.json"
output.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n")
print("FULL_CROSS_PIVOT_CYCLE_REPLAY_PASS",json.dumps({
    "rank":rank,"active_columns":len(active_ids),
    "line_checks":len(line_checks),
    "rectangle_checks":len(rect_checks)},sort_keys=True),flush=True)
