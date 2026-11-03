"""Independent integer-curve and quartic replay of the third residual pivot."""

import hashlib
import json
from collections import Counter, defaultdict, deque
from math import comb
from pathlib import Path

from verify_adaptive_private_columns import add, neg, on_curve

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
receipt_path = HERE / "type_i_residual_pivot002_cross.json"
receipt = json.loads(receipt_path.read_text())
assert receipt["status"] == "p192_cross_pivot_three_step_sampled_closure_fan"
assert receipt["pivot_index"] == 2
assert receipt["source_sha256"] == sha(HERE / "line_residual_pivot_cross_fan.py")
assert all(sha(HERE / name) == digest
           for name, digest in receipt["input_sha256"].items())
assert receipt["runtime_info_sha256"] == sha(
    HERE / "type_i_residual_pivot002_cross_runtime_info.json")
assert receipt["comparison_fan_sha256"] == sha(
    HERE / "type_i_residual_pivot001_cross.json")
plane = json.loads((HERE / "type_i_plane_quartic.json").read_text())
p = int(plane["p"])
assert p == int(receipt["field_prime"]) and p % 4 == 3
terms = [(tuple(map(int, label.split(","))), int(value) % p)
         for label, value in plane["quartic_coefficients"].items()]
points = [tuple(map(int, line.split()[1:])) for line in
          (HERE / "type_i_line_base_points.tsv").read_text().splitlines()]
points += [tuple(map(int, line.split())) for line in
           (HERE / "type_i_line_new_points.tsv").read_text().splitlines()]
selected = [tuple(map(int, x)) for x in json.loads(
    (HERE / "type_i_line_guided_collector.json").read_text())["base_points_fp"]]
selected += [tuple(map(int, x)) for x in json.loads(
    (HERE / "type_i_line_full_new_quotient_points.json").read_text())]
assert len(points) == len(set(points)) == len(selected) == len(set(selected)) == 9925
assert all(on_curve(q, p) for q in selected)
selected_keys = set(selected)
selected_signed = {min(q, neg(q, p)) for q in selected}
S = tuple(map(int, json.loads((HERE / "type_i_line_full_constant.json")
                              .read_text())["constant_fp"]))
pivot = tuple(map(int, receipt["pivot_quartic_point"]))
pivot_q = tuple(map(int, receipt["pivot_quotient_point"]))
assert on_curve(S, p) and on_curve(pivot_q, p)


def quartic(x, y):
    return sum(c * pow(x, a, p) * pow(y, b, p)
               for (a, b, _), c in terms) % p


def signed(key):
    return None if key is None else min(key, neg(key, p))


assert quartic(*pivot) == 0 and pivot not in set(points)
inv2, inv3, inv4, inv8 = (pow(k, -1, p) for k in (2, 3, 4, 8))
rows = receipt["rows"]
assert receipt["rows_sha256"] == hashlib.sha256(json.dumps(
    rows, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
counts = Counter()
seen_geometry, seen_exact, seen_signed = {}, {}, {}
next_index = 0
for j, (u, v) in enumerate(points):
    counts["selected_pairs"] += 1
    d = (u - pivot[0]) % p
    assert d
    m = (v - pivot[1]) * pow(d, -1, p) % p

    def at(t):
        return quartic((pivot[0] + t) % p, (pivot[1] + m*t) % p)

    fp1, fm1, fp2, fm2 = at(1), at(-1), at(2), at(-2)
    odd1, even1 = (fp1-fm1)*inv2 % p, (fp1+fm1)*inv2 % p
    odd2, even2 = (fp2-fm2)*inv4 % p, (fp2+fm2)*inv8 % p
    c3 = (odd2-odd1)*inv3 % p
    c1 = (odd1-c3) % p
    A = (even2-even1)*inv3 % p
    c2 = (even1-A) % p
    B, C = (c3+d*A) % p, (c2+d*(c3+d*A)) % p
    assert A and C and (c1+d*C) % p == 0
    discr = (B*B-4*A*C) % p
    assert discr
    symbol = pow(discr, (p-1)//2, p)
    if symbol == p-1:
        counts["nonsplit"] += 1
        continue
    assert symbol == 1
    row = rows[next_index]
    assert row["selected_index"] == j
    sq = pow(discr, (p+1)//4, p)
    den = pow(2*A, -1, p)
    roots = [(-B+sq)*den % p, (-B-sq)*den % p]
    expected = [((pivot[0]+z) % p, (pivot[1]+m*z) % p) for z in roots]
    geometry = [tuple(map(int, point)) for point in row["residual_points"]]
    assert geometry == expected and len(set(geometry)) == 2
    assert all(quartic(*point) == 0 for point in geometry)
    projected = [tuple(map(int, pair)) if pair is not None else None
                 for pair in row["projected_residuals"]]
    assert all(on_curve(q, p) for q in projected)
    assert add(add(add(pivot_q, selected[j], p), projected[0], p),
               projected[1], p) == S
    for side, (point, key) in enumerate(zip(geometry, projected)):
        if point in points:
            counts["geometric_selected_hits"] += 1
        if point in seen_geometry:
            counts["geometric_repeats"] += 1
        seen_geometry.setdefault(point, [next_index, side])
        if key is None:
            counts["identity_columns"] += 1
        if key in selected_keys:
            counts["selected_hits"] += 1
        skey = signed(key)
        if skey in selected_signed:
            counts["signed_selected_hits"] += 1
        if key in seen_exact:
            counts["exact_repeats"] += 1
        if skey in seen_signed:
            counts["signed_repeats"] += 1
        seen_exact.setdefault(key, [next_index, side])
        seen_signed.setdefault(skey, [next_index, side])
    counts["split"] += 1
    next_index += 1
assert next_index == len(rows)
assert dict(counts) == receipt["counts"]
assert len(seen_geometry) == receipt["distinct_geometry"]
assert len(seen_exact) == receipt["distinct_exact_columns"]
assert len(seen_signed) == receipt["distinct_signed_columns"]
assert receipt["six_step_control_row_indices"] == list(range(0, len(rows), 256))

# Independently scan the old family and both earlier residual-pivot fans.
old_exact_hits, old_signed_hits = [], []
old_columns = 0
for i in range(141):
    old_chunk = json.loads((HERE / "type_i_line_trace_full_chunks" /
                            f"pivot-{i:03d}.json").read_text())
    for row_index, old_row in enumerate(old_chunk["rows"]):
        for side, pair in enumerate(old_row["projected_residuals"]):
            key = tuple(map(int, pair))
            old_columns += 1
            if key in seen_exact:
                old_exact_hits.append([i, row_index, side, seen_exact[key]])
            if signed(key) in seen_signed:
                old_signed_hits.append([i, row_index, side, seen_signed[signed(key)]])
assert old_columns == receipt["old_family_columns_scanned"] == 1360948
assert old_exact_hits == receipt["old_exact_hits"]
assert old_signed_hits == receipt["old_signed_hits"]

comparison_hits = {}
fans = []
for pivot_index in range(3):
    path = HERE / ("type_i_residual_pivot000_three.json" if pivot_index == 0 else
                   f"type_i_residual_pivot{pivot_index:03d}_cross.json")
    fans.append(json.loads(path.read_text()))
    if pivot_index == 2:
        continue
    exact, folded = {}, {}
    for first_index, first_row in enumerate(fans[-1]["rows"]):
        for side, pair in enumerate(first_row["projected_residuals"]):
            key = tuple(map(int, pair))
            exact.setdefault(key, [first_index, side])
            folded.setdefault(signed(key), [first_index, side])
    comparison_hits[pivot_index] = {
        "exact": [[index, side, exact[key]]
                  for index, row in enumerate(rows)
                  for side, pair in enumerate(row["projected_residuals"])
                  if (key := tuple(map(int, pair))) in exact],
        "signed": [[index, side, folded[signed(key)]]
                   for index, row in enumerate(rows)
                   for side, pair in enumerate(row["projected_residuals"])
                   if (key := tuple(map(int, pair))) is not None
                   and signed(key) in folded],
    }
assert comparison_hits[1]["exact"] == receipt["cross_fan_exact_hits"]
assert comparison_hits[1]["signed"] == receipt["cross_fan_signed_hits"]

# Eliminate any row containing a private nonselected signed column.
occurrences = defaultdict(list)
row_columns = []
source_rows = []
for fan_index, fan in enumerate(fans):
    sources = set(fan["source_line_selected_pair"])
    for row_index, row in enumerate(fan["rows"]):
        if row["selected_index"] in sources:
            source_rows.append([fan_index, row_index, row["selected_index"]])
            continue
        columns = set()
        for pair in row["projected_residuals"]:
            key = signed(tuple(map(int, pair)))
            if key not in selected_signed:
                columns.add(key)
        index = len(row_columns)
        row_columns.append(columns)
        for key in columns:
            occurrences[key].append(index)
degrees = {key: len(indices) for key, indices in occurrences.items()}
active = set(range(len(row_columns)))
queue = deque(i for i, columns in enumerate(row_columns)
              if any(degrees[key] == 1 for key in columns))
while queue:
    index = queue.popleft()
    if index not in active or not any(degrees[key] == 1 for key in row_columns[index]):
        continue
    active.remove(index)
    for key in row_columns[index]:
        degrees[key] -= 1
        if degrees[key] == 1:
            queue.extend(occurrences[key])

two_private_rows = sum(
    len(columns) == 2 and all(len(occurrences[key]) == 1 for key in columns)
    for columns in row_columns)
assert two_private_rows == len(row_columns) and not active
exact_union = set(selected)
signed_union = set(selected_signed)
for fan in fans:
    for row in fan["rows"]:
        for pair in row["projected_residuals"]:
            assert pair is not None
            key = tuple(map(int, pair))
            exact_union.add(key)
            signed_union.add(signed(key))
accepted_lower = int(json.loads((HERE / "type_i_character_density.json")
                                .read_text())["accepted_nonzero_scalar_lower_bound"])
capacity = comb(len(exact_union)+2, 3)
exponent = 0
while capacity * 2**(exponent+1) < accepted_lower:
    exponent += 1

out = {
    "status": "independent_pivot002_integer_curve_and_cross_fan_replay",
    "pivot_index": 2,
    "counts": dict(counts),
    "old_family_exact_hits": old_exact_hits,
    "old_family_signed_hits": old_signed_hits,
    "comparison_hits": comparison_hits,
    "source_rows": source_rows,
    "non_source_rows": len(row_columns),
    "non_source_rows_with_two_private_signed_columns": two_private_rows,
    "signed_core_rows_after_private_column_peeling": len(active),
    "exact_selected_plus_three_fan_union": len(exact_union),
    "signed_selected_plus_three_fan_union": len(signed_union),
    "unordered_three_sum_capacity": capacity,
    "ideal_uniform_accepted_query_hit_less_than_two_to_minus": exponent,
    "receipt_sha256": sha(receipt_path),
    "fan_sha256": [sha(HERE / name) for name in
                   ("type_i_residual_pivot000_three.json",
                    "type_i_residual_pivot001_cross.json",
                    "type_i_residual_pivot002_cross.json")],
    "source_sha256": sha(Path(__file__)),
    "boundary": "Full quartic and prime-field group-sum replay for pivot 002; old-family and two-fan signed-column scans; leaf peeling of non-source three-fan rows. Projected input construction has spaced direct six-step controls in the producer.",
}
path = HERE / "type_i_residual_pivot002_cross_replay.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True)+"\n")
print("PIVOT002_CROSS_REPLAY_PASS", len(rows), len(active), comparison_hits,
      flush=True)
