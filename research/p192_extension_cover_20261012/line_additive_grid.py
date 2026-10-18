"""A 4x4 grid of fixed-origin quartic classes with repeated quotient columns.

The eight seed classes are independently frozen prior reductions.  For each
sum A_i+B_j, the existing two-cubic constructor supplies a new degree-three
representative D_ij-T.  The grid equations and adjacent rectangles form a
column-nonprivate control; their rank is measured over the exact r-subgroup.
"""

import hashlib
import json
import sys
import time
from collections import Counter
from itertools import combinations_with_replacement
from pathlib import Path

from sage.all import GF, Integer, matrix

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def prelude(name):
    path = HERE / name
    source = path.read_text()
    marker = "start = time.monotonic_ns()"
    assert source.count(marker) == 1
    namespace = {"__file__": str(path), "__name__": name[:-3] + "_prelude"}
    exec(compile(source.split(marker)[0], str(path), "exec"), namespace)
    return namespace, source


def key(point):
    return None if not point else (str(int(point[0])), str(int(point[1])))


def main():
    prior_path = HERE / "type_i_two_pivot_reduced64.json"
    fixture_path = HERE / "type_i_base_points_legacy_fixture.json"
    guide_path = HERE / "type_i_line_guided_collector.json"
    density_path = HERE / "type_i_character_density.json"
    runtime_path = HERE / "type_i_additive_grid_runtime_info.json"
    assert runtime_path.is_file()
    prior = json.loads(prior_path.read_text())
    fixture = json.loads(fixture_path.read_text())
    guide = json.loads(guide_path.read_text())
    density = json.loads(density_path.read_text())
    quotient, _ = prelude("line_quotient_cubic_probe.py")
    reduction, reduction_source = prelude("line_quotient_fixed_origin.py")
    E, L, p, r = (quotient[name] for name in ("E", "L", "p", "r"))
    O, inv2 = quotient["O"], quotient["inv2"]
    assert int(p) == int(prior["field_prime"]) == int(fixture["field_prime"])
    assert int(r) == int(prior["subgroup_order"]) == int(fixture["subgroup_order"])
    assert len(prior["rows"]) == len(prior["representatives"]) == 64
    seed_rows = prior["rows"][:4] + prior["rows"][32:36]
    seed_reps = prior["representatives"][:4] + prior["representatives"][32:36]
    assert all(row["status"] == "verified_fixed_origin_degree_three"
               for row in seed_reps)
    seeds = [E(L(Integer(row["quotient"][0])),
               L(Integer(row["quotient"][1]))) for row in seed_rows]
    assert len(set(seeds)) == 8 and all(r*Q == O and Q != O for Q in seeds)
    sums = [seeds[i] + seeds[4+j] for i in range(4) for j in range(4)]
    assert len(set(sums)) == 16 and all(Q != O and r*Q == O for Q in sums)
    assert not set(sums).intersection(seeds)
    sextic = [int(c) for c in density["sextic_coefficients_decimal"]]

    def character(Q):
        x = int(Q[0])
        value = 0
        for coefficient in reversed(sextic):
            value = (value*x + coefficient) % int(p)
        assert value != 0
        return "+" if pow(value, (int(p)-1)//2, int(p)) == 1 else "-"

    def cell(Q):
        H = inv2*Q
        assert 2*H == Q
        return character(H) + character(Q)

    rows = []
    start = time.monotonic_ns()
    for index, Q in enumerate(sums):
        H = inv2*Q
        rows.append({"index": index, "grid_index": [index//4, index%4],
                     "quotient": list(key(Q)), "cell": cell(Q),
                     "positive_input_residual": quotient["residual"](Q),
                     "negative_input_residual": quotient["residual"](H),
                     "status": "verified_prime_field_cubic_pair"})
        if (index+1) % 4 == 0:
            print("ADDITIVE_GRID_CUBIC_PROGRESS", index+1, flush=True)
    reduction["source"] = {"rows": rows}
    reduction["representatives"] = []
    loop = reduction_source[
        reduction_source.index('for item in source["rows"]:'):
        reduction_source.index("\nsuccesses =")]
    exec(compile(loop, str(HERE / "line_quotient_fixed_origin.py"), "exec"), reduction)
    reps = reduction["representatives"]
    assert len(reps) == 16
    successes = sum(row["status"] == "verified_fixed_origin_degree_three"
                    for row in reps)

    # Column order: four A seeds, four B seeds, sixteen derived C classes.
    relation_rows = []
    for i in range(4):
        for j in range(4):
            vec = [0]*24
            vec[i], vec[4+j], vec[8+4*i+j] = -1, -1, 1
            assert sums[4*i+j] - seeds[i] - seeds[4+j] == O
            relation_rows.append(vec)
    for i in range(3):
        for j in range(3):
            vec = [0]*24
            for a, b, sign in ((i,j,1),(i+1,j+1,1),
                               (i+1,j,-1),(i,j+1,-1)):
                vec[8+4*a+b] += sign
            assert sums[4*i+j]+sums[4*(i+1)+j+1] == \
                sums[4*(i+1)+j]+sums[4*i+j+1]
            relation_rows.append(vec)
    degrees = [sum(v != 0 for row in relation_rows for v in [row[k]])
               for k in range(24)]
    rank = int(matrix(GF(r), relation_rows).rank())
    assert min(degrees) >= 2 and rank == 16

    old = [E(L(Integer(item["x"])), L(Integer(item["y"])))
           for item in fixture["points"]]
    base = old + seeds + sums
    assert len(old) == 16 and len(set(base)) == 40
    targets = {}
    collisions = []
    target_cells = Counter()
    for indices in combinations_with_replacement(range(len(base)), 3):
        Q = sum((base[i] for i in indices), O)
        name = key(Q)
        if name in targets:
            collisions.append([list(targets[name]), list(indices)])
        else:
            targets[name] = indices
            if Q != O:
                target_cells[cell(Q)] += 1
    query_laws = {}
    G = E(L(Integer("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012", 16)),
          L(Integer("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811", 16)))
    assert r*G == O
    for law, scalars in (("uniform", guide["uniform_scalars"]),
                         ("character_guided", guide["guided_scalars"])):
        hit, query_cells = [], Counter()
        for index, encoded in enumerate(scalars):
            a = Integer(encoded)
            assert 0 < a < r
            Q = -2*a*G
            query_cells[cell(Q)] += 1
            witness = targets.get(key(Q))
            if witness is not None:
                assert sum((base[i] for i in witness), O) == Q
                hit.append({"index": index, "scalar": str(a),
                            "witness": list(witness)})
        query_laws[law] = {"queries": len(scalars),
                           "cells": dict(sorted(query_cells.items())),
                           "hits": hit}

    output = {
        "status": "verified_p192_additive_grid_fixed_origin_probe"
                  if successes == 16 else "partial_p192_additive_grid_reduction",
        "candidate_id": None,
        "field_prime": str(p), "subgroup_order": str(r),
        "seed_source_indices": [0,1,2,3,32,33,34,35],
        "seed_quotients": [list(key(Q)) for Q in seeds],
        "grid_quotients": [list(key(Q)) for Q in sums],
        "rows": rows, "representatives": reps,
        "reduced_classes": successes,
        "origin_x_polynomial": reduction["encode_poly"](reduction["origin_h"]),
        "origin_y_polynomial": reduction["encode_poly"](reduction["origin_y"]),
        "relation_rows": relation_rows,
        "relation_count": len(relation_rows), "column_count": 24,
        "column_degrees": degrees, "two_core_rows": len(relation_rows),
        "two_core_columns": 24, "relation_rank": rank,
        "seed_constraint_rank_after_derived_elimination": 0,
        "three_sum_base_quotients": [list(key(Q)) for Q in base],
        "unordered_three_sums": len(targets)+len(collisions),
        "distinct_three_sum_targets": len(targets),
        "three_sum_collisions": collisions,
        "three_sum_character_cells": dict(sorted(target_cells.items())),
        "known_scalar_query_laws": query_laws,
        "elapsed_ns_unisolated": time.monotonic_ns()-start,
        "source_sha256": sha(__file__),
        "input_sha256": {path.name: sha(path) for path in
                         (prior_path, fixture_path, guide_path,
                          density_path, runtime_path)},
        "constructor_sha256": {name: sha(HERE/name) for name in
                              ("line_quotient_cubic_probe.py",
                               "line_quotient_fixed_origin.py")},
        "boundary": "The column-nonprivate grid is produced by exact additions of the eight seeds; its rectangle rows are algebraic consequences. Rank after eliminating all derived classes constrains no seed logarithm. Triple support and frozen ordinary-query hits are measured separately.",
    }
    path = HERE / "type_i_additive_grid.json"
    path.write_text(json.dumps(output, indent=2, sort_keys=True)+"\n")
    print("ADDITIVE_GRID_PASS", json.dumps({
        "reduced": successes, "two_core": [len(relation_rows),24],
        "rank": rank, "seed_rank": 0,
        "triple_targets": len(targets),
        "triple_collisions": len(collisions),
        "queries": {name: len(law["hits"]) for name,law in query_laws.items()},
    }, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
