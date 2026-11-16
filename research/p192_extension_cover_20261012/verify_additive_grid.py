"""Independent integer-curve replay of the P-192 additive grid and queries."""

import hashlib
import json
from collections import Counter
from itertools import combinations_with_replacement
from pathlib import Path

from verify_adaptive_private_columns import GX, GY, add, cell, key, mul, neg, on_curve

HERE = Path(__file__).resolve().parent
producer_path = HERE / "type_i_additive_grid.json"
source = json.loads(producer_path.read_text())
prior = json.loads((HERE / "type_i_two_pivot_reduced64.json").read_text())
guide = json.loads((HERE / "type_i_line_guided_collector.json").read_text())
density = json.loads((HERE / "type_i_character_density.json").read_text())
fixture = json.loads((HERE / "type_i_base_points_legacy_fixture.json").read_text())
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
assert source["source_sha256"] == sha(HERE / "line_additive_grid.py")
for name, digest in source["input_sha256"].items():
    assert sha(HERE / name) == digest
for name, digest in source["constructor_sha256"].items():
    assert sha(HERE / name) == digest
p, r = int(source["field_prime"]), int(source["subgroup_order"])
assert p == int(guide["field_prime"]) == int(density["p"])
assert r == int(guide["subgroup_order"]) == int(density["r"])
coefficients = [int(c) for c in density["sextic_coefficients_decimal"]]
assert source["seed_source_indices"] == [0,1,2,3,32,33,34,35]
seeds = [tuple(map(int, prior["rows"][i]["quotient"]))
         for i in source["seed_source_indices"]]
grid = [tuple(map(int, pair)) for pair in source["grid_quotients"]]
assert seeds == [tuple(map(int, pair)) for pair in source["seed_quotients"]]
assert len(seeds) == 8 and len(grid) == 16
assert all(on_curve(Q, p) and mul(r, Q, p) is None for Q in seeds+grid)
assert len(set(seeds+grid)) == 24
for i in range(4):
    for j in range(4):
        assert add(seeds[i], seeds[4+j], p) == grid[4*i+j]
        assert tuple(map(int, source["rows"][4*i+j]["quotient"])) == grid[4*i+j]
assert source["reduced_classes"] == sum(
    row["status"] == "verified_fixed_origin_degree_three"
    for row in source["representatives"])
assert all(row["quotient"] == source["rows"][i]["quotient"]
           for i,row in enumerate(source["representatives"]))

rows = []
for i in range(4):
    for j in range(4):
        v = [0]*24
        v[i],v[4+j],v[8+4*i+j] = -1,-1,1
        rows.append(v)
for i in range(3):
    for j in range(3):
        v = [0]*24
        for a,b,s in ((i,j,1),(i+1,j+1,1),(i+1,j,-1),(i,j+1,-1)):
            v[8+4*a+b] += s
        rows.append(v)
assert rows == source["relation_rows"]
assert len(rows) == source["relation_count"] == 25
degrees = [sum(row[k] != 0 for row in rows) for k in range(24)]
assert degrees == source["column_degrees"] and min(degrees) >= 2
assert source["two_core_rows"] == 25 and source["two_core_columns"] == 24


def rank(vectors):
    pivots = {}
    for raw in vectors:
        row = [x % r for x in raw]
        for k in sorted(pivots):
            if row[k]:
                factor = row[k]
                row = [(a-factor*b) % r for a,b in zip(row,pivots[k])]
        k = next((i for i,x in enumerate(row) if x), None)
        if k is not None:
            inverse = pow(row[k], -1, r)
            pivots[k] = [x*inverse % r for x in row]
    return len(pivots)


assert rank(rows[:16]) == rank(rows) == source["relation_rank"] == 16
# Eliminating each distinct C pivot from every rectangle leaves the zero row.
for row in rows[16:]:
    reduced = [x % r for x in row]
    for k in range(16):
        factor = reduced[8+k]
        if factor:
            reduced = [(a-factor*b) % r for a,b in zip(reduced, rows[k])]
    assert all(x == 0 for x in reduced)
assert source["seed_constraint_rank_after_derived_elimination"] == 0

old = [(int(item["x"]), int(item["y"])) for item in fixture["points"]]
base = old + seeds + grid
assert len(base) == len(set(base)) == 40
assert base == [tuple(map(int,Q)) for Q in source["three_sum_base_quotients"]]
targets, collisions, cells = {}, [], Counter()
for triple in combinations_with_replacement(range(40),3):
    Q = add(add(base[triple[0]],base[triple[1]],p),base[triple[2]],p)
    if Q in targets:
        collisions.append([list(targets[Q]),list(triple)])
    else:
        targets[Q] = triple
        if Q is not None:
            H = neg(mul(pow(2,-1,r),Q,p),p)
            cells[cell(H,coefficients,p)] += 1
assert len(targets)+len(collisions) == source["unordered_three_sums"]
assert len(targets) == source["distinct_three_sum_targets"]
assert collisions == source["three_sum_collisions"]
assert dict(sorted(cells.items())) == source["three_sum_character_cells"]
assert on_curve((GX,GY),p) and mul(r,(GX,GY),p) is None
for law_name,law in source["known_scalar_query_laws"].items():
    scalars = guide["uniform_scalars" if law_name=="uniform" else "guided_scalars"]
    observed,hits = Counter(),[]
    for index,encoded in enumerate(scalars):
        a=int(encoded)
        assert 0<a<r
        P=mul(a,(GX,GY),p)
        Q=neg(add(P,P,p),p)
        observed[cell(P,coefficients,p)] += 1
        if Q in targets:
            hits.append({"index":index,"scalar":str(a),
                         "witness":list(targets[Q])})
    assert law["queries"] == len(scalars) == 64
    assert law["cells"] == dict(sorted(observed.items()))
    assert law["hits"] == hits

receipt = {
    "status":"independent_additive_grid_replay_pass",
    "producer_sha256":sha(producer_path),
    "source_sha256":sha(__file__),
    "matrix_rank":rank(rows),
    "seed_constraint_rank":0,
    "three_sum_targets":len(targets),
    "query_hits":{name:len(law["hits"])
                  for name,law in source["known_scalar_query_laws"].items()},
}
path=HERE/"type_i_additive_grid_replay.json"
path.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n")
print("ADDITIVE_GRID_REPLAY_PASS",json.dumps(receipt,sort_keys=True),flush=True)
