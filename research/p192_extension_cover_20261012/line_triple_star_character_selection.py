"""Optimize admitted four-sum support at a fixed 96-class star budget.

The search universe is frozen before scoring: the first 32 source-free
endpoints common to all three complete pivot fans. The objective counts
exact ++ character-cell targets across the three pivot-pair rectangles.
"""

import hashlib
import json
import time
from collections import Counter
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import add, mul, neg, on_curve

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
fan_names = ["type_i_residual_pivot000_three.json",
             "type_i_residual_pivot001_cross.json",
             "type_i_residual_pivot002_cross.json"]
inputs = [HERE / name for name in fan_names]
fans = [json.loads(path.read_text()) for path in inputs]
guide_path = HERE / "type_i_line_guided_collector.json"
new_path = HERE / "type_i_line_full_new_quotient_points.json"
density_path = HERE / "type_i_character_density.json"
screen_path = HERE / "type_i_triple_star_screen.json"
guide, new, density, screen = [json.loads(path.read_text()) for path in
                               (guide_path, new_path, density_path, screen_path)]
p, r = int(screen["field_prime"]), int(screen["subgroup_order"])
assert p == int(guide["field_prime"]) == int(density["p"])
assert r == int(guide["subgroup_order"]) == int(density["r"])
selected = [tuple(map(int, Q)) for Q in guide["base_points_fp"] + new]
eligible = [set(int(row["selected_index"]) for row in fan["rows"]) -
            set(map(int, fan["source_line_selected_pair"])) for fan in fans]
universe = sorted(set.intersection(*eligible))[:32]
baseline = list(map(int, screen["selected_endpoints"]))
assert universe[:16] == baseline
assert len(universe) == 32 and len(set(universe)) == 32
S = tuple(map(int, screen["line_section_constant"]))
pivots = [tuple(map(int, fan["pivot_quotient_point"])) for fan in fans]
assert all(on_curve(Q, p) for Q in [S] + pivots)
coefficients = [int(v) for v in density["sextic_coefficients_decimal"]]
inv2 = pow(2, -1, r)


def character(Q):
    assert Q is not None
    x = Q[0]
    value = 0
    for coefficient in reversed(coefficients):
        value = (value * x + coefficient) % p
    assert value != 0
    return "+" if pow(value, (p - 1) // 2, p) == 1 else "-"


def cell(Q):
    H = mul(inv2, Q, p)
    assert H is not None and add(H, H, p) == Q
    return character(H) + character(Q)


start = time.monotonic_ns()
pair_data = {}
for a, b in combinations(range(3), 2):
    constant = add(add(add(S, S, p), neg(pivots[a], p), p),
                   neg(pivots[b], p), p)
    first = {u: add(constant, neg(selected[u], p), p) for u in universe}
    for u, v in combinations(universe, 2):
        Q = add(first[u], neg(selected[v], p), p)
        assert Q is not None and on_curve(Q, p)
        pair_data[a, b, u, v] = (Q, cell(Q))
    print("TRIPLE_STAR_CHARACTER_PAIR_PROGRESS", a, b,
          len(pair_data), flush=True)
assert len(pair_data) == 3 * 496

weights = {(u, v): sum(pair_data[a, b, u, v][1] == "++"
                       for a, b in combinations(range(3), 2))
           for u, v in combinations(universe, 2)}


def score(chosen):
    return sum(weights[u, v] for u, v in combinations(sorted(chosen), 2))


def improve(starting):
    chosen = set(starting)
    assert len(chosen) == 16 and chosen <= set(universe)
    while True:
        current = score(chosen)
        moves = []
        for old in sorted(chosen):
            for new in sorted(set(universe) - chosen):
                candidate = (chosen - {old}) | {new}
                gain = score(candidate) - current
                if gain > 0:
                    moves.append((-gain, old, new))
        if not moves:
            return sorted(chosen), current
        _, old, new = min(moves)
        chosen.remove(old)
        chosen.add(new)


starts = [universe[:16], universe[16:]]
for offset in range(1, 16):
    starts.append(sorted((universe[offset:] + universe[:offset])[:16]))
starts.append(sorted(universe[::2]))
starts.append(sorted(universe[1::2]))
assert all(len(set(start)) == 16 for start in starts)
results = [improve(start) for start in starts]
optimized, objective = min(results, key=lambda result: (-result[1], result[0]))
assert objective >= score(baseline)


def describe(chosen):
    targets = {}
    cells = Counter()
    for a, b in combinations(range(3), 2):
        for u, v in combinations(sorted(chosen), 2):
            Q, label = pair_data[a, b, u, v]
            cells[label] += 1
            targets.setdefault(Q, []).append([a, b, u, v])
    assert sum(cells.values()) == 360
    return {"endpoints": sorted(chosen), "character_cells": dict(sorted(cells.items())),
            "positive_presentations": 360,
            "distinct_positive_targets": len(targets),
            "duplicate_positive_presentations": 360 - len(targets)}


baseline_result = describe(baseline)
optimized_result = describe(optimized)
assert baseline_result["character_cells"].get("++", 0) == score(baseline)
assert optimized_result["character_cells"].get("++", 0) == objective
elapsed_s = round((time.monotonic_ns() - start) / 1e9, 3)
out = {
    "status": "verified_triple_star_character_selection_screen",
    "candidate_id": None,
    "field_prime": str(p), "subgroup_order": str(r),
    "universe": universe, "selection_size": 16,
    "candidate_pair_targets_scored": len(pair_data),
    "optimization": "deterministic best-improvement one-swap ascent from frozen starts",
    "starts_considered": len(starts),
    "baseline": baseline_result, "optimized": optimized_result,
    "source_sha256": sha(__file__),
    "input_sha256": {path.name: sha(path) for path in
                     inputs + [guide_path, new_path, density_path, screen_path]},
    "boundary": "Exact ++ character-cell support on a frozen first-32 common-endpoint universe. Selection is fixed-geometry precomputation; reduction certificates, ordinary-query hits, and target-dependent costs are separate gates.",
}
path = HERE / "type_i_triple_star_character_selection.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print("TRIPLE_STAR_CHARACTER_SELECTION_PASS", json.dumps({
    "baseline_admitted": score(baseline), "optimized_admitted": objective,
    "optimized_endpoints": optimized,
    "elapsed_s_unisolated": elapsed_s,
}, sort_keys=True), flush=True)
