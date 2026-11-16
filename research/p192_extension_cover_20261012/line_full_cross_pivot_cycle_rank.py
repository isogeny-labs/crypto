"""Implicit rectangle basis and exact rank on three complete P-192 fans.

The all-rectangle family has millions of rows.  A common triple-overlap
endpoint gives a compact 6,314-row basis.  We replay its sparse coefficient
rank after coalescing equal exact projected subgroup columns.
"""

import hashlib
import json
from collections import Counter
from itertools import combinations
from math import comb
from pathlib import Path

from verify_adaptive_private_columns import add, on_curve

HERE=Path(__file__).resolve().parent
names=["type_i_residual_pivot000_three.json",
       "type_i_residual_pivot001_cross.json",
       "type_i_residual_pivot002_cross.json"]
selected_names=["type_i_line_guided_collector.json",
                "type_i_line_full_new_quotient_points.json"]
sha=lambda path:hashlib.sha256(Path(path).read_bytes()).hexdigest()
fans=[json.loads((HERE/name).read_text()) for name in names]
guide=json.loads((HERE/selected_names[0]).read_text())
new=json.loads((HERE/selected_names[1]).read_text())
p,r=int(guide["field_prime"]),int(guide["subgroup_order"])
selected=[tuple(map(int,Q)) for Q in guide["base_points_fp"]+new]
assert len(selected)==len(set(selected))==9925
set_selected=set(selected)
assert all(int(f["field_prime"])==p and int(f["subgroup_order"])==r
           for f in fans)

sets=[]
edge_by_key={}
columns=[]
column_index={}
edge_index={}
duplicate_presentations=[]
selected_hits=[]
for pivot,fan in enumerate(fans):
    endpoints=set()
    for row_index,row in enumerate(fan["rows"]):
        endpoint=int(row["selected_index"])
        assert endpoint not in endpoints
        endpoints.add(endpoint)
        local=[]
        for side,encoded in enumerate(row["projected_residuals"]):
            Q=tuple(map(int,encoded))
            assert Q is not None and on_curve(Q,p)
            if Q in column_index:
                duplicate_presentations.append({
                    "first":list(column_index[Q][1]),
                    "second":[pivot,row_index,side],
                    "quotient":list(Q)})
                index=column_index[Q][0]
            else:
                index=len(columns)
                columns.append(Q)
                column_index[Q]=(index,(pivot,row_index,side))
            if Q in set_selected:
                selected_hits.append([pivot,row_index,side])
            local.append(index)
        assert len(local)==2 and local[0]!=local[1]
        edge_by_key[pivot,endpoint]=tuple(local)
        edge_index[pivot,endpoint]=row_index
    sets.append(endpoints)

assert [len(s) for s in sets]==[5037,4971,5026]
assert len(edge_by_key)==sum(map(len,sets))==15034
print("FULL_CROSS_PIVOT_SCAN_PASS",[len(s) for s in sets],
      len(columns),flush=True)
pair_overlaps={(u,v):sets[u]&sets[v]
               for u,v in combinations(range(3),2)}
assert [len(pair_overlaps[pair]) for pair in ((0,1),(0,2),(1,2))]== \
    [2574,2552,2485]
triple=set.intersection(*sets)
assert len(triple)==1295
anchor=min(triple)
union=set.union(*sets)
vertices=3+len(union)
cycle_rank=len(edge_by_key)-vertices+1
assert vertices==8721 and cycle_rank==6314


def basis_witnesses():
    for u,v in ((0,1),(0,2)):
        for endpoint in sorted(pair_overlaps[u,v]-{anchor}):
            yield u,v,endpoint,anchor
    # At a triple-overlap endpoint, the BC row is the difference of its
    # AB and AC rows.  BC-only overlap supplies the remaining basis rows.
    for endpoint in sorted(pair_overlaps[1,2]-sets[0]):
        yield 1,2,endpoint,anchor


def sparse_row(witness):
    u,v,j,k=witness
    raw=Counter()
    for pivot,endpoint,sign in ((u,j,1),(v,k,1),(u,k,-1),(v,j,-1)):
        for index in edge_by_key[pivot,endpoint]:raw[index]+=sign
    return {index:value%r for index,value in raw.items() if value%r}


witnesses=list(basis_witnesses())
assert len(witnesses)==cycle_rank
rows=[sparse_row(w) for w in witnesses]
assert all(1<=len(row)<=8 for row in rows)
# Each selected AB row owns a B-endpoint residual column; each selected AC
# row owns a C-endpoint column; each BC-only row owns a B-endpoint column.
# Source-fan duplicate presentations are coalesced first.  A coefficient
# occurring in exactly one basis row gives a direct rank certificate over
# every coefficient field, including the exact prime-order subgroup field.
occurrence=Counter(index for row in rows for index in row)
private_pivots=[]
for witness,row in zip(witnesses,rows):
    u,v,j,_=witness
    owner=1 if (u,v) in ((0,1),(1,2)) else 2
    choices=[index for index in edge_by_key[owner,j]
             if row.get(index,0) and occurrence[index]==1]
    assert choices
    private_pivots.append(min(choices))
assert len(set(private_pivots))==len(witnesses)
actual_rank=len(private_pivots)
print("FULL_CROSS_PIVOT_BASIS_PASS",actual_rank,flush=True)
rectangles=sum(comb(len(v),2) for v in pair_overlaps.values())
assert rectangles==9652897
active_edges=[(pivot,endpoint) for pivot in range(3)
              for endpoint in sets[pivot]
              if sum(endpoint in s for s in sets)>=2]
assert len(active_edges)==11337
active_columns={index for edge in active_edges
                for index in edge_by_key[edge]}
degrees=[]
for pivot,endpoint in active_edges:
    degree=sum(len(pair_overlaps[min(pivot,other),max(pivot,other)])-1
               for other in range(3)
               if other!=pivot and endpoint in sets[other])
    degrees.append(degree)
assert min(degrees)>=2

# Independent integer group checks on a frozen spread of source cuts and
# compact-basis rectangles.  The complete fan producers already assert the
# line-section identity on every row; the SHA-pinned inputs retain them.
S=None
checked_line_rows=[]
for pivot,fan in enumerate(fans):
    pivot_Q=tuple(map(int,fan["pivot_quotient_point"]))
    sampled=sorted({i*(len(fan["rows"])-1)//31 for i in range(32)})
    for row_index in sampled:
        row=fan["rows"][row_index]
        endpoint=int(row["selected_index"])
        Qa,Qb=(tuple(map(int,Q)) for Q in row["projected_residuals"])
        value=add(add(add(pivot_Q,selected[endpoint],p),Qa,p),Qb,p)
        if S is None:S=value
        assert value==S
        checked_line_rows.append([pivot,row_index,endpoint])
assert len(checked_line_rows)==96
checked_rectangles=[]
for index in sorted({i*(len(witnesses)-1)//63 for i in range(64)}):
    u,v,j,k=witnesses[index]
    left=[];right=[]
    for pivot,endpoint,sign in ((u,j,1),(v,k,1),(u,k,-1),(v,j,-1)):
        pair=[columns[z] for z in edge_by_key[pivot,endpoint]]
        (left if sign==1 else right).extend(pair)
    total_left=None
    total_right=None
    for Q in left:total_left=add(total_left,Q,p)
    for Q in right:total_right=add(total_right,Q,p)
    assert total_left==total_right
    checked_rectangles.append(index)
assert len(checked_rectangles)==64


def non_source_policy():
    kept=[set(group)-set(fans[pivot]["source_line_selected_pair"])
          for pivot,group in enumerate(sets)]
    pair={(u,v):kept[u]&kept[v]
          for u,v in combinations(range(3),2)}
    triple_kept=set.intersection(*kept)
    assert triple_kept
    local_anchor=min(triple_kept)
    basis=[]
    for u,v in ((0,1),(0,2)):
        basis.extend((u,v,j,local_anchor) for j in sorted(pair[u,v]-{local_anchor}))
    basis.extend((1,2,j,local_anchor)
                 for j in sorted(pair[1,2]-kept[0]))
    edge_count=sum(map(len,kept))
    vertex_count=3+len(set.union(*kept))
    graph_rank=edge_count-vertex_count+1
    assert len(basis)==graph_rank
    sparse=[sparse_row(w) for w in basis]
    use=Counter(index for row in sparse for index in row)
    pivots=[]
    for (u,v,j,_),row in zip(basis,sparse):
        owner=1 if (u,v) in ((0,1),(1,2)) else 2
        options=[index for index in edge_by_key[owner,j]
                 if row.get(index,0) and use[index]==1]
        assert options
        pivots.append(min(options))
    assert len(set(pivots))==graph_rank
    active=[(pivot,j) for pivot in range(3) for j in kept[pivot]
            if sum(j in group for group in kept)>=2]
    active_ids={index for key in active for index in edge_by_key[key]}
    column_presentations=[index for pivot in range(3) for j in kept[pivot]
                          for index in edge_by_key[pivot,j]]
    minimum=min(sum(len(pair[min(pivot,other),max(pivot,other)])-1
                    for other in range(3)
                    if other!=pivot and j in kept[other])
                for pivot,j in active)
    return {
        "fan_non_source_rows":[len(group) for group in kept],
        "split_cut_edges":edge_count,
        "selected_endpoints_union":len(set.union(*kept)),
        "incidence_vertices":vertex_count,
        "pair_overlap_counts":{f"{u}_{v}":len(group)
                               for (u,v),group in pair.items()},
        "triple_overlap_count":len(triple_kept),
        "common_anchor_endpoint":local_anchor,
        "implicit_rectangle_presentations":sum(comb(len(group),2)
                                                for group in pair.values()),
        "basis_witnesses":[list(w) for w in basis],
        "basis_private_pivot_columns":pivots,
        "basis_rank":graph_rank,
        "projected_column_presentations":len(column_presentations),
        "exact_distinct_projected_columns":len(set(column_presentations)),
        "active_edge_presentations":len(active),
        "active_column_presentations":2*len(active),
        "active_distinct_projected_columns":len(active_ids),
        "minimum_active_column_degree_in_implicit_rows":minimum,
    }


non_source=non_source_policy()
assert non_source["split_cut_edges"]==15028
assert non_source["basis_rank"]==6309
assert non_source["implicit_rectangle_presentations"]==9635113
assert non_source["exact_distinct_projected_columns"]==30056
assert non_source["active_distinct_projected_columns"]==22658

raw_basis=json.dumps(witnesses,separators=(",",":"),sort_keys=True).encode()
out={
    "status":"verified_p192_full_cross_pivot_cycle_basis",
    "candidate_id":None,
    "field_prime":str(p),"subgroup_order":str(r),
    "fan_rows":[len(f["rows"]) for f in fans],
    "split_cut_edges":len(edge_by_key),
    "selected_endpoints_union":len(union),
    "incidence_vertices":vertices,"incidence_components":1,
    "pair_overlap_counts":{f"{u}_{v}":len(vs)
                           for (u,v),vs in pair_overlaps.items()},
    "triple_overlap_count":len(triple),
    "common_anchor_endpoint":anchor,
    "implicit_rectangle_presentations":rectangles,
    "basis_witnesses":witnesses,
    "basis_witnesses_sha256":hashlib.sha256(raw_basis).hexdigest(),
    "basis_rows":len(rows),
    "basis_private_pivot_columns":private_pivots,
    "combinatorial_cycle_rank":cycle_rank,
    "coalesced_projected_column_rank":actual_rank,
    "projected_column_presentations":2*len(edge_by_key),
    "exact_distinct_projected_columns":len(columns),
    "duplicate_projected_presentations":duplicate_presentations,
    "selected_column_hits":selected_hits,
    "active_edge_presentations":len(active_edges),
    "active_column_presentations":2*len(active_edges),
    "active_distinct_projected_columns":len(active_columns),
    "minimum_active_column_degree_in_implicit_rows":min(degrees),
    "implicit_two_core_rows":rectangles,
    "implicit_two_core_columns":len(active_columns),
    "independent_group_line_checks":checked_line_rows,
    "independent_group_rectangle_basis_checks":checked_rectangles,
    "line_section_constant":list(S) if S else None,
    "non_source_policy":non_source,
    "source_sha256":sha(__file__),
    "input_sha256":{name:sha(HERE/name) for name in names+selected_names},
    "boundary":"The 9,652,897 rectangle presentations are counted and represented by a 6,314-row exact sparse basis, not materialized. All cuts were checked by frozen fan producers; this replay independently checks a spread of 96 cuts and 64 basis rectangles. Only the 96-class slice has frozen fixed-origin reduced representatives so far.",
}
path=HERE/"type_i_full_cross_pivot_cycle_rank.json"
path.write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
print("FULL_CROSS_PIVOT_CYCLE_PASS",json.dumps({
    "edges":out["split_cut_edges"],
    "implicit_rows":rectangles,
    "rank":actual_rank,
    "active_columns":len(active_columns),
    "duplicate_presentations":len(duplicate_presentations)},sort_keys=True),flush=True)
print("NON_SOURCE_CROSS_PIVOT_CYCLE_PASS",json.dumps({
    "edges":non_source["split_cut_edges"],
    "implicit_rows":non_source["implicit_rectangle_presentations"],
    "rank":non_source["basis_rank"],
    "active_columns":non_source["active_distinct_projected_columns"]},
    sort_keys=True),flush=True)
