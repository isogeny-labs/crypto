#!/usr/bin/env python3
"""Independent prime-field audit of saved first-pencil witnesses and totals.

The native producer checks every accepted Jacobian relation.  This separate
implementation checks one saved relation per walk as a polynomial identity,
and recomputes all aggregate counters.  It does not reimplement Jacobian
arithmetic or claim independent validation of all unsaved relations.
"""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = ROOT / "research/more_leads_20261019/target_labelled_pencil_v1.jsonl"
FIXTURE = ROOT / "research/three_leads_e2e_20261018/genus-order-fixtures-v1/order-genus-16.json"
RHO = ROOT / "research/more_leads_20261019/source_rho_controls_v1.jsonl"


def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def multiply(a, b, p):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = (out[i + j] + x * y) % p
    return trim(out)


def quotient(a, b, p):
    a = trim(a[:])
    assert b[-1] == 1
    q = [0] * max(1, len(a) - len(b) + 1)
    while len(a) >= len(b):
        shift = len(a) - len(b)
        c = a[-1]
        q[shift] = c
        for i, bi in enumerate(b):
            a[shift + i] = (a[shift + i] - c * bi) % p
        trim(a)
    return trim(q), a


def evaluate(a, x, p):
    out = 0
    for c in reversed(a):
        out = (out * x + c) % p
    return out


def difference(a, b, p):
    return trim([((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p
                 for i in range(max(len(a), len(b)))])


def audit_elimination(target, walks, h, p, r):
    roots_by_square = [[] for _ in range(p)]
    for y in range(p):
        roots_by_square[y * y % p].append(y)
    node_xs = [x for x in range(p) if any(y != 0 for y in roots_by_square[evaluate(h, x, p)])]
    nodes = {x: i for i, x in enumerate(node_xs)}
    n = len(nodes)
    assert n == target["solver_nodes"]
    pivots = {}
    updates = inversions = dependent = 0
    recovered = None
    walks_by_index = {i: w for i, w in enumerate(walks)}
    keys = target["solver_relation_keys"]
    assert len(keys) == target["solver_inserted_relations"]
    for position, (walk_index, lam) in enumerate(keys, 1):
        walk = walks_by_index[walk_index]
        d = walk["divisor"]
        u, v = d["u"][:3], d["v"][:2]
        profile = [((v[i] if i < len(v) else 0) + lam * u[i]) % p for i in range(3)]
        q, rem = quotient(difference(multiply(profile, profile, p), h, p), u, p)
        assert not rem and len(q) == 4
        xs = [x for x in range(p) if evaluate(q, x, p) == 0]
        assert len(xs) == 3
        row = [0] * (n + 2)
        for x in xs:
            y = evaluate(profile, x, p)
            assert y * y % p == evaluate(h, x, p)
            if y:
                col = nodes[x]
                row[col] = (row[col] + (1 if y <= p // 2 else -1)) % r
        row[n] = walk["b"]
        row[n + 1] = -walk["a"] % r
        for col in range(n):
            if not row[col]:
                continue
            if col in pivots:
                factor = row[col]
                pivot = pivots[col]
                for j in range(col, n + 2):
                    row[j] = (row[j] - factor * pivot[j]) % r
                    updates += 1
            else:
                inverse = pow(row[col], -1, r)
                inversions += 1
                for j in range(col, n + 2):
                    row[j] = row[j] * inverse % r
                    updates += 1
                pivots[col] = row
                break
        else:
            dependent += 1
            if row[n]:
                inversions += 1
                recovered = row[n + 1] * pow(row[n], -1, r) % r
                assert position == target["solve_witness"]["inserted_relations"]
                assert [walk_index, lam] == [target["solve_witness"]["walk_index"],
                                             target["solve_witness"]["lambda"]]
                break
            assert row[n + 1] == 0
    assert len(pivots) == target["solver_rank"]
    assert dependent == target["solver_dependent_relations"]
    assert updates == target["solver_row_updates"]
    assert inversions == target["solver_inversions"]
    assert recovered == target["recovered"] == target["scalar"]
    return len(keys)


def audit():
    rows = [json.loads(line) for line in RECEIPT.open()]
    summary = rows[-1]
    assert summary["schema"] == "pencil-study-summary/v1"
    p, r = summary["p"], summary["r"]
    h = summary["quintic_coefficients_low_to_high"]
    assert p == 739 and p % 4 == 3 and r == 34147
    assert h == [c * pow(12, -1, p) % p for c in [12, 100, 299, 380, 187, 30]]
    fixture = json.loads(FIXTURE.read_text())["workloads"]["curves"][0]
    assert (fixture["p"], fixture["r"]) == (p, r)
    walks = [x for x in rows if x["schema"] == "pencil-target-walk/v1"]
    targets = [x for x in rows if x["schema"] == "pencil-target-summary/v1"]
    assert len(walks) == 100 and len(targets) == 20 and len(rows) == 121
    for t in targets:
        ti = t["target_index"]
        group = [w for w in walks if w["target_index"] == ti]
        assert len(group) == t["walks"] == 5
        assert t["scalar"] == (123 + 1777 * ti) % r
        assert t["complete_relations"] == sum(w["complete_three_root_relations"] for w in group)
        assert [w["a"] for w in group] == [
            (2695 + 7919 * i + 1741 * ti) % r for i in range(5)]
        assert [w["b"] for w in group] == [
            (409 + 3151 * i + 857 * ti) % r for i in range(5)]
    for w in walks:
        ti = w["target_index"]
        assert w["scalar"] == (123 + 1777 * ti) % r
        d = w["divisor"]
        assert d["degree"] == 2 and d["u"][2] == 1
        u, v = d["u"][:3], d["v"][:2]
        assert quotient(difference(multiply(v, v, p), h, p), u, p)[1] == []
        first = w["first_relation"]
        lam = first["lambda"]
        profile = [(v[i] if i < len(v) else 0) + lam * u[i] for i in range(3)]
        profile = [c % p for c in profile]
        q, rem = quotient(difference(multiply(profile, profile, p), h, p), u, p)
        assert rem == [] and q == first["quotient"]
        roots = [x for x in range(p) if evaluate(q, x, p) == 0]
        assert len(roots) == 3
        assert roots == [point[0] for point in first["points"]]
        for x, y in first["points"]:
            assert y == evaluate(profile, x, p)
            assert y * y % p == evaluate(h, x, p)
        assert w["point_sieve_batch_multiplies"] == 3 * (
            w["point_sieve_ordinary_denominators"] +
            w["point_sieve_exceptional_denominators"])
        assert w["point_sieve_batch_inversions"] == 1
        assert w["complete_three_root_relations"] <= w["point_sieve_candidates"]
    summed = {
        "complete_three_root_relations": sum(w["complete_three_root_relations"] for w in walks),
        "point_sieve_candidates": sum(w["point_sieve_candidates"] for w in walks),
        "point_sieve_ordinary_denominators": sum(w["point_sieve_ordinary_denominators"] for w in walks),
        "point_sieve_exceptional_denominators": sum(w["point_sieve_exceptional_denominators"] for w in walks),
        "point_sieve_batch_inversions": sum(w["point_sieve_batch_inversions"] for w in walks),
        "point_sieve_batch_multiplies": sum(w["point_sieve_batch_multiplies"] for w in walks),
        "point_sieve_point_probes": sum(w["point_sieve_point_probes"] for w in walks),
    }
    assert all(summary[key] == value for key, value in summed.items())
    assert summary["pencil_members"] == p * len(walks)
    assert summary["baseline_split_walks"] == sum(w["baseline_two_distinct_roots"] for w in walks)
    assert summary["repeated_root_u_walks"] == sum(w["u_distinct_rational_roots"] == 1 for w in walks)
    independently_replayed_rows = sum(audit_elimination(
        t, [w for w in walks if w["target_index"] == t["target_index"]], h, p, r)
        for t in targets)
    assert summary["targets_solved_by_three_point_elimination"] == 20
    assert summary["projected_factor_base_points_checked"] == summary["rational_curve_points"]
    rho = [json.loads(line) for line in RHO.open()]
    assert len(rho) == len(targets)
    for index, control in enumerate(rho):
        assert control["schema"] == "pencil-matched-source-rho/v1"
        assert control["target_index"] == index
        assert control["scalar"] == control["recovered"] == targets[index]["recovered"]
        assert control["seed"] == 2026102001 + index
        assert 0 < control["native"]["walk_steps"] < 128700
    return {"schema": "pencil-independent-witness-audit/v1", "walks": len(walks),
            "first_witnesses_checked": len(walks), "aggregate_counters_checked": True,
            "polynomial_identities_checked": True, "p": p, "r": r,
            "complete_relations_reported_by_producer": summed["complete_three_root_relations"],
            "solver_rows_independently_replayed": independently_replayed_rows,
            "targets_recovered_by_independent_elimination": len(targets),
            "matched_source_rho_solves_checked": len(rho)}


if __name__ == "__main__":
    print(json.dumps(audit(), sort_keys=True))
