"""Independent exact rank and certificate replay for the reduced triple star."""

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
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
screen_path = HERE / "type_i_triple_star_screen.json"
reduced_path = HERE / "type_i_triple_star_reduced96.json"
prepared_path = HERE / "type_i_triple_star_reduced96_input.json"
gp_input_path = HERE / "type_i_triple_star_reduced96_input.gp"
gp_log_path = HERE / "type_i_triple_star_reduced96_native.log"
screen, reduced, prepared = [json.loads(path.read_text()) for path in
                             (screen_path, reduced_path, prepared_path)]
assert screen["source_sha256"] == sha(HERE / "line_triple_star_screen.py")
assert reduced["source_sha256"] == sha(HERE / "line_triple_star_reduced_classes.py")
assert reduced["screen_sha256"] == sha(screen_path)
assert prepared["source_sha256"] == sha(HERE / "prepare_triple_star_gp.py")
assert prepared["reduction_sha256"] == sha(reduced_path)
assert prepared["gp_input_sha256"] == sha(gp_input_path)
assert gp_log_path.read_text().strip() == (
    "TRIPLE_STAR_FIXED_ORIGIN_NATIVE_PASS cases=96 prime_bits=192")
assert reduced["runtime_sha256"] == sha(HERE / "type_i_triple_star_runtime_info.json")
assert reduced["input_classes"] == reduced["reduced_classes"] == 96
assert reduced["unreduced_classes"] == 0

fans = [json.loads((HERE / name).read_text()) for name in FAN_NAMES]
guide = json.loads((HERE / "type_i_line_guided_collector.json").read_text())
new = json.loads((HERE / "type_i_line_full_new_quotient_points.json").read_text())
p, r = int(screen["field_prime"]), int(screen["subgroup_order"])
assert p == int(reduced["field_prime"]) and r == int(reduced["subgroup_order"])
selected = [tuple(map(int, Q)) for Q in guide["base_points_fp"] + new]
assert len(selected) == len(set(selected)) == 9925
maps = [{int(row["selected_index"]): (i, row)
         for i, row in enumerate(fan["rows"])} for fan in fans]
eligible = [set(mapping) - set(map(int, fan["source_line_selected_pair"]))
            for mapping, fan in zip(maps, fans)]
endpoints = sorted(set.intersection(*eligible))[:16]
assert endpoints == screen["selected_endpoints"] == reduced["selected_endpoints"]
assert len(endpoints) == 16

columns = []
edge = {}
S = None
for pivot, fan in enumerate(fans):
    assert reduced["fan_sha256"][FAN_NAMES[pivot]] == sha(HERE / FAN_NAMES[pivot])
    P = tuple(map(int, fan["pivot_quotient_point"]))
    for endpoint in endpoints:
        fan_row_index, row = maps[pivot][endpoint]
        Qs = [tuple(map(int, Q)) for Q in row["projected_residuals"]]
        for side, Q in enumerate(Qs):
            index = len(columns)
            encoded = reduced["rows"][index]
            representative = reduced["representatives"][index]
            assert (encoded["index"], encoded["pivot"],
                    encoded["selected_endpoint"], encoded["fan_row_index"],
                    encoded["residual_side"]) == (
                        index, pivot, endpoint, fan_row_index, side)
            assert tuple(map(int, encoded["quotient"])) == Q
            assert representative["quotient"] == encoded["quotient"]
            assert representative["status"] == "verified_fixed_origin_degree_three"
            assert on_curve(Q, p)
            columns.append(Q)
        edge[pivot, endpoint] = (len(columns) - 2, len(columns) - 1)
        value = add(add(add(P, selected[endpoint], p), Qs[0], p), Qs[1], p)
        if S is None:
            S = value
        assert value == S
assert list(S) == screen["line_section_constant"]
assert len(columns) == len(set(columns)) == 96


def rectangle(a, b, u, v):
    row = Counter()
    positive = negative = None
    for pivot, endpoint, sign in ((a, u, 1), (b, v, 1),
                                  (a, v, -1), (b, u, -1)):
        for index in edge[pivot, endpoint]:
            row[index] += sign
            if sign == 1:
                positive = add(positive, columns[index], p)
            else:
                negative = add(negative, columns[index], p)
    assert positive == negative
    return [row[i] % r for i in range(96)], positive


rows = []
targets = {}
for a, b in combinations(range(3), 2):
    for u, v in combinations(endpoints, 2):
        row, target = rectangle(a, b, u, v)
        rows.append(row)
        targets.setdefault(target, []).append([a, b, u, v])
assert len(rows) == 360
assert len(targets) == screen["distinct_rectangle_targets"] == 360
active = {i for i in range(96) if any(row[i] for row in rows)}
degrees = [sum(bool(row[i]) for row in rows) for i in range(96)]
assert len(active) == 96 and min(degrees) == max(degrees) == 30

# RREF over the exact P-192 subgroup order, independent of the producer's
# private-column witness selection and graph cycle count.
pivots = {}
for raw in rows:
    row = raw[:]
    for lead, pivot in sorted(pivots.items()):
        if row[lead]:
            factor = row[lead]
            row = [(x - factor * y) % r for x, y in zip(row, pivot)]
    lead = next((i for i, value in enumerate(row) if value), None)
    if lead is not None:
        inverse = pow(row[lead], -1, r)
        row = [value * inverse % r for value in row]
        for earlier_lead, pivot in pivots.items():
            if pivot[lead]:
                factor = pivot[lead]
                pivots[earlier_lead] = [
                    (x - factor * y) % r for x, y in zip(pivot, row)]
        pivots[lead] = row
assert len(pivots) == screen["basis_rank"] == 30

query_hits = {}
for name, scalars in (("uniform", guide["uniform_scalars"]),
                      ("character_guided", guide["guided_scalars"])):
    hits = []
    for i, encoded in enumerate(scalars):
        Q = mul(int(encoded), neg(add((GX, GY), (GX, GY), p), p), p)
        if Q in targets:
            hits.append([i, str(encoded), targets[Q]])
    assert hits == screen["query_laws"][name]["hits"]
    query_hits[name] = len(hits)

out = {
    "status": "independent_triple_star_rank_and_cubic_replay_pass",
    "classes": len(columns), "reduced_classes": reduced["reduced_classes"],
    "line_cuts_checked": len(edge), "rectangle_rows_checked": len(rows),
    "exact_rank": len(pivots), "active_columns": len(active),
    "minimum_column_degree": min(degrees),
    "maximum_column_degree": max(degrees),
    "distinct_targets": len(targets), "query_hits": query_hits,
    "screen_sha256": sha(screen_path), "reduction_sha256": sha(reduced_path),
    "gp_input_sha256": sha(gp_input_path), "gp_log_sha256": sha(gp_log_path),
    "source_sha256": sha(__file__),
}
path = HERE / "type_i_triple_star_replay.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print("TRIPLE_STAR_REPLAY_PASS", json.dumps({
    "classes": 96, "rank": len(pivots), "rows": len(rows),
    "query_hits": query_hits}, sort_keys=True), flush=True)
