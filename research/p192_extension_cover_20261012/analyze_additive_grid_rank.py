"""Eliminate derived grid columns from exact three-sum collision rows."""

import hashlib
import json
from collections import Counter
from pathlib import Path

from verify_adaptive_private_columns import add, neg

HERE=Path(__file__).resolve().parent
path=HERE/"type_i_additive_grid.json"
source=json.loads(path.read_text())
r=int(source["subgroup_order"])
base=[tuple(map(int,Q)) for Q in source["three_sum_base_quotients"]]
p=int(source["field_prime"])
assert len(base)==40
grid=[[0]*16+row for row in source["relation_rows"][:16]]
collisions=[]
residuals=Counter()
first={}
for index,(left,right) in enumerate(source["three_sum_collisions"]):
    row=[0]*40
    for k in left:row[k]+=1
    for k in right:row[k]-=1
    collisions.append(row)
    reduced=row[:]
    for k in range(16):
        factor=reduced[24+k]
        if factor:
            reduced=[a-factor*b for a,b in zip(reduced,grid[k])]
    assert reduced[24:]==[0]*16
    key=tuple(reduced[:24])
    residuals[key]+=1
    first.setdefault(key,[index,left,right])


def rank(rows):
    pivots={}
    for raw in rows:
        v=[x%r for x in raw]
        for k,q in sorted(pivots.items()):
            if v[k]:
                t=v[k]
                v=[(x-t*y)%r for x,y in zip(v,q)]
        k=next((i for i,x in enumerate(v) if x),None)
        if k is not None:
            z=pow(v[k],-1,r)
            pivots[k]=[x*z%r for x in v]
    return len(pivots)


nonzero={v:count for v,count in residuals.items() if any(v)}
assert len(collisions)==2805
assert residuals[(0,)*24]==2688
assert sum(nonzero.values())==117
assert rank(grid)==16
assert rank(collisions)==16
assert rank(grid+collisions)==17
assert rank([list(v) for v in nonzero])==1
for vector in nonzero:
    total=None
    for k,coefficient in enumerate(vector):
        assert coefficient in (-1,0,1)
        if coefficient==1:total=add(total,base[16+k],p)
        elif coefficient==-1:total=add(total,neg(base[16+k],p),p)
    assert total is None
example=min(nonzero,key=lambda v:first[v][0])
receipt={
    "status":"verified_additive_grid_collision_elimination",
    "producer_sha256":hashlib.sha256(path.read_bytes()).hexdigest(),
    "source_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "collision_rows":len(collisions),
    "collision_row_rank":rank(collisions),
    "grid_row_rank":rank(grid),
    "combined_rank":rank(grid+collisions),
    "seed_constraint_rank":1,
    "zero_reduced_rows":residuals[(0,)*24],
    "nonzero_reduced_rows":sum(nonzero.values()),
    "nonzero_residual_vectors":len(nonzero),
    "first_seed_relation_witness":first[example],
    "first_seed_relation_vector":list(example),
}
out=HERE/"type_i_additive_grid_rank.json"
out.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n")
print("ADDITIVE_GRID_RANK_PASS",json.dumps({
    "grid":receipt["grid_row_rank"],
    "triple_collisions":receipt["collision_row_rank"],
    "combined":receipt["combined_rank"],
    "seed":receipt["seed_constraint_rank"],
    "nonzero_rows":receipt["nonzero_reduced_rows"]},sort_keys=True),flush=True)
