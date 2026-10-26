"""Choose a source-free three-pivot star under a 96-class reduction budget."""

import hashlib
import json
from collections import Counter
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import GX, GY, add, mul, neg, on_curve

HERE = Path(__file__).resolve().parent
FAN_NAMES = ["type_i_residual_pivot000_three.json",
             "type_i_residual_pivot001_cross.json",
             "type_i_residual_pivot002_cross.json"]
GUIDE_NAMES = ["type_i_line_guided_collector.json",
               "type_i_line_full_new_quotient_points.json"]
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
fans = [json.loads((HERE / name).read_text()) for name in FAN_NAMES]
guide, new = [json.loads((HERE / name).read_text()) for name in GUIDE_NAMES]
p, r = int(guide["field_prime"]), int(guide["subgroup_order"])
selected = [tuple(map(int, Q)) for Q in guide["base_points_fp"] + new]
assert len(selected) == len(set(selected)) == 9925
assert all(int(fan["field_prime"]) == p and
           int(fan["subgroup_order"]) == r for fan in fans)

rows_by_endpoint = [{int(row["selected_index"]): row for row in fan["rows"]}
                    for fan in fans]
eligible = [set(rows) - set(map(int, fan["source_line_selected_pair"]))
            for rows, fan in zip(rows_by_endpoint, fans)]
endpoints = sorted(set.intersection(*eligible))[:16]
assert len(endpoints) == 16
assert endpoints == [19, 30, 33, 35, 48, 52, 63, 66,
                     72, 73, 75, 80, 83, 91, 116, 128]
anchor = endpoints[0]

columns = []
edge = {}
S = None
for pivot, fan in enumerate(fans):
    P = tuple(map(int, fan["pivot_quotient_point"]))
    for endpoint in endpoints:
        row = rows_by_endpoint[pivot][endpoint]
        Qs = [tuple(map(int, Q)) for Q in row["projected_residuals"]]
        assert len(Qs) == 2 and all(on_curve(Q, p) for Q in Qs)
        value = add(add(add(P, selected[endpoint], p), Qs[0], p), Qs[1], p)
        if S is None:
            S = value
        assert value == S
        edge[pivot, endpoint] = tuple(range(len(columns), len(columns) + 2))
        columns.extend(Qs)
assert len(edge) == 48 and len(columns) == len(set(columns)) == 96


def rectangle(a, b, u, v):
    row = Counter()
    positive = []
    negative = []
    for pivot, endpoint, sign in ((a, u, 1), (b, v, 1),
                                  (a, v, -1), (b, u, -1)):
        for index in edge[pivot, endpoint]:
            row[index] += sign
            (positive if sign == 1 else negative).append(columns[index])
    left = right = None
    for Q in positive:
        left = add(left, Q, p)
    for Q in negative:
        right = add(right, Q, p)
    assert left == right
    return {index: value % r for index, value in row.items() if value % r}, left


basis = []
for a, b in ((0, 1), (0, 2)):
    for endpoint in endpoints[1:]:
        basis.append([a, b, endpoint, anchor])
assert len(basis) == 30
basis_rows = [rectangle(*w)[0] for w in basis]
occurrence = Counter(index for row in basis_rows for index in row)
private = []
for witness, row in zip(basis, basis_rows):
    a, b, endpoint, _ = witness
    assert a == 0
    options = [index for index in edge[b, endpoint]
               if row.get(index, 0) and occurrence[index] == 1]
    assert options
    private.append(min(options))
assert len(set(private)) == 30

targets = {}
for a, b in combinations(range(3), 2):
    for u, v in combinations(endpoints, 2):
        row, target = rectangle(a, b, u, v)
        targets.setdefault(target, []).append([a, b, u, v])
        if a == 1:
            ab, _ = rectangle(0, 1, u, v)
            ac, _ = rectangle(0, 2, u, v)
            assert all((ac.get(k, 0) - ab.get(k, 0)) % r == row.get(k, 0)
                       for k in set(ab) | set(ac) | set(row))
assert sum(map(len, targets.values())) == 360

G = (GX, GY)
assert mul(r, G, p) is None
query_laws = {}
for name, scalars in (("uniform", guide["uniform_scalars"]),
                      ("character_guided", guide["guided_scalars"])):
    hits = []
    for i, encoded in enumerate(scalars):
        scalar = int(encoded)
        assert 0 < scalar < r
        P = mul(scalar, G, p)
        Q = neg(add(P, P, p), p)
        if Q in targets:
            hits.append([i, str(scalar), targets[Q]])
    query_laws[name] = {"queries": len(scalars), "hits": hits}

out = {
    "status": "verified_p192_source_free_triple_star_screen",
    "candidate_id": None,
    "field_prime": str(p), "subgroup_order": str(r),
    "selected_endpoints": endpoints, "anchor_endpoint": anchor,
    "cut_edges": len(edge), "projected_classes": len(columns),
    "distinct_projected_classes": len(set(columns)),
    "implicit_rectangle_presentations": 360,
    "distinct_rectangle_targets": len(targets),
    "basis_witnesses": basis, "basis_private_columns": private,
    "basis_rank": len(private), "query_laws": query_laws,
    "line_section_constant": list(S),
    "source_sha256": sha(__file__),
    "input_sha256": {name: sha(HERE / name) for name in FAN_NAMES + GUIDE_NAMES},
    "boundary": "Source-free exact quotient-class rank and 128-query support screen; fixed-origin reduction and independent cubic replay are separate gates.",
}
path = HERE / "type_i_triple_star_screen.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print("TRIPLE_STAR_SCREEN_PASS", json.dumps({
    "classes": len(columns), "rows": 360, "rank": len(private),
    "targets": len(targets),
    "query_hits": {name: len(law["hits"]) for name, law in query_laws.items()},
}, sort_keys=True), flush=True)
