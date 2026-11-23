#!/usr/bin/env python3
"""Fail-closed count-only audit of frozen CT82 selected-call vectors.

This is not a native evaluator replay, calibrated benchmark, PCC certificate,
or ECDLP advantage. Uses only Python's standard library.
"""
import argparse
import copy
import json
from collections import Counter
from pathlib import Path

FIXTURE = Path("research/chained_jacobian_transfers_20261110/evidence/public-n4-complete-chain-checkpoint-01/01-literal-setup-and-selected-vectors.json")
DIRECT = "direct_composite"
LITERAL = "literal_sequential_Jacobian_maps"
RETAINED = "strongest_retained_lift_chain"
ROUTES = (DIRECT, LITERAL, RETAINED)
C13 = "degree13_elliptic_evaluation"
C169 = "degree169_elliptic_evaluation"
DBL = "norm_point_doubling"
COUNTERS = (
    "correction_dictionary_lookup", "correction_difference", C13, C169,
    "explicit_half_check", "external_full_order", "external_norm_and_geometry",
    "external_packet_parse", "final_actual_norm", "final_full_order",
    "generic_halving_factorization", "generic_ordinate", "infinity_halves",
    "inverse_full_group_scalar", "matching_corrections_checked",
    "quadratic_source_phi", "quadratic_target_phi", "source_lift_addition",
    "target_correction_addition", "two_torsion_halves",
)
REUSE = {DIRECT: (C169, 2, 1, 1),
         LITERAL: (C13, 4, 2, 2),
         RETAINED: (C13, 4, 2, 1)}


def require(ok, message):
    if not ok:
        raise ValueError(message)


def audit(data):
    require(set(data) == {"actual_setup", "all48_actual_selected_vectors", "scope"}, "top-level schema")
    rows = data["all48_actual_selected_vectors"]["all48"]
    require(type(rows) is list and len(rows) == 48, "48 whole selected records required")
    directions, sectors, seen, savings = Counter(), Counter(), set(), Counter()
    gaps = {name: set() for name in (*COUNTERS, DBL)}
    for i, row in enumerate(rows):
        require(set(row) == set(ROUTES) | {"from", "to", "input", "sector", "scalar_OUTSIDE_runtime", "whole_outputs_equal"}, f"row schema {i}")
        direction = (row["from"], row["to"])
        require(direction in {(0, 2), (2, 0)}, f"direction {i}")
        require(type(row["sector"]) is int and row["sector"] in range(4), f"sector {i}")
        require(type(row["scalar_OUTSIDE_runtime"]) is int, f"scalar diagnostic {i}")
        require(type(row["input"]) is list and type(row["whole_outputs_equal"]) is list, f"encoded data {i}")
        identity = (direction, row["sector"], row["scalar_OUTSIDE_runtime"], tuple(row["input"]))
        require(identity not in seen, f"duplicate record {i}")
        seen.add(identity)
        directions[direction] += 1
        sectors[row["sector"]] += 1
        old, new = {}, {}
        for route in ROUTES:
            values = row[route]
            require(type(values) is dict and set(values) == set(COUNTERS), f"missing or added count {i}:{route}")
            require(all(type(v) is int and v >= 0 for v in values.values()), f"invalid count {i}:{route}")
            field, before, after, doubles = REUSE[route]
            require(values[field] == before, f"baseline changed {i}:{route}")
            other = C169 if field == C13 else C13
            require(values[other] == 0, f"unexpected map {i}:{route}")
            old[route] = values
            v = dict(values)
            v[field] = after
            v[DBL] = doubles
            new[route] = v
            savings[route] += before - after
        require(all(old[DIRECT][k] == old[RETAINED][k] for k in COUNTERS if k not in (C13, C169)), f"unequal common costs {i}")
        for k in (*COUNTERS, DBL):
            require(new[LITERAL][k] >= new[RETAINED][k], f"literal lower than retained {i}:{k}")
            gaps[k].add(new[LITERAL][k] - new[RETAINED][k])
            if k not in (C13, C169):
                require(new[DIRECT][k] == new[RETAINED][k], f"unexpected direct gap {i}:{k}")
        require(new[DIRECT][C169] == 1 and new[RETAINED][C13] == 2, f"map crossover {i}")
    require(directions == Counter({(0, 2): 24, (2, 0): 24}), "direction balance")
    require(sectors == Counter({k: 12 for k in range(4)}), "sector balance")
    require(savings == Counter({DIRECT: 48, LITERAL: 96, RETAINED: 96}), "incorrect savings")
    return {
        "source": "frozen CT82 selected-call counters only",
        "vectors": len(rows),
        "directions": {"0->2": 24, "2->0": 24},
        "sectors": {str(k): 12 for k in range(4)},
        "saved_map_evaluations": dict(sorted(savings.items())),
        "direct_minus_retained": "C169 - 2*C13, all other recorded counters equal",
        "literal_componentwise_dominated": True,
        "literal_minus_retained_ranges": {k: [min(s), max(s)] for k, s in gaps.items() if max(s) > 0},
        "conditional_reuse_gate": {DIRECT: "C169 > D", LITERAL: "C13 > D", RETAINED: "2*C13 > D"},
        "status": "[pcc:unverified]; prospective counts; no setup, primitive prices, wall clock, ECDLP or independent native replay",
    }


def self_test():
    primitive = {k: 0 for k in COUNTERS}
    primitive.update({"explicit_half_check": 4, "correction_dictionary_lookup": 1,
                      "correction_difference": 1, "matching_corrections_checked": 2,
                      "quadratic_source_phi": 1, "quadratic_target_phi": 1,
                      "source_lift_addition": 1, "final_actual_norm": 1})
    rows = []
    for fr, to in ((0, 2), (2, 0)):
        for sector in range(4):
            for seed in range(6):
                direct = dict(primitive, **{C169: 2})
                retained = dict(primitive, **{C13: 4})
                literal = dict(retained)
                literal.update({"explicit_half_check": 8, "correction_dictionary_lookup": 2,
                                "correction_difference": 2, "matching_corrections_checked": 4,
                                "quadratic_source_phi": 2, "quadratic_target_phi": 2,
                                "source_lift_addition": 2})
                rows.append({"from": fr, "to": to, "sector": sector,
                             "scalar_OUTSIDE_runtime": seed, "input": [fr, to, seed, sector],
                             "whole_outputs_equal": [0], DIRECT: direct, LITERAL: literal,
                             RETAINED: retained})
    baseline = {"actual_setup": {}, "scope": "", "all48_actual_selected_vectors": {"all48": rows}}
    require(audit(baseline)["vectors"] == 48, "synthetic control")
    mutations = []
    a = copy.deepcopy(baseline)
    a["all48_actual_selected_vectors"]["all48"].pop()
    mutations.append(a)
    b = copy.deepcopy(baseline)
    b["all48_actual_selected_vectors"]["all48"][0][DIRECT][C169] = 1
    mutations.append(b)
    c = copy.deepcopy(baseline)
    c["all48_actual_selected_vectors"]["all48"][0][LITERAL]["quadratic_source_phi"] = 0
    mutations.append(c)
    d = copy.deepcopy(baseline)
    d["all48_actual_selected_vectors"]["all48"][0][RETAINED]["silent_new_counter"] = 1
    mutations.append(d)
    e = copy.deepcopy(baseline)
    e["all48_actual_selected_vectors"]["all48"][0]["sector"] = 9
    mutations.append(e)
    for i, mutant in enumerate(mutations):
        try:
            audit(mutant)
        except ValueError:
            continue
        raise AssertionError(f"mutation {i} wrongly accepted")
    return {"self_test": "passed", "synthetic_control": 48, "mutations_rejected": 5}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixture", type=Path, default=FIXTURE)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    result = self_test() if args.self_test else audit(json.loads(args.fixture.read_text("utf-8")))
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
