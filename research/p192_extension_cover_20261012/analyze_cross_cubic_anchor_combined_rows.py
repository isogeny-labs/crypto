"""Combine the anchored degree-eight fan with known-scalar cubic triples."""

import hashlib
import json
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from analyze_cross_endpoint_query_triples import rank_sparse
from verify_adaptive_private_columns import GX, GY, add, mul, neg

HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


def rank_sparse_min(rows, modulus):
    """Independent min-pivot elimination, paired with the max-pivot rank."""
    basis = {}
    for original in rows:
        row = dict(original)
        for lead, vector in sorted(basis.items()):
            factor = row.pop(lead, 0)
            if factor:
                for column, value in vector.items():
                    if column == lead:
                        continue
                    updated = (row.get(column, 0) - factor * value) % modulus
                    if updated:
                        row[column] = updated
                    else:
                        row.pop(column, None)
        if row:
            lead = min(row)
            inverse = pow(row[lead], -1, modulus)
            basis[lead] = {column: value * inverse % modulus
                           for column, value in row.items()}
    return len(basis)


def main():
    source = load("type_i_known_scalar_query_cross_fan.json")
    transfer = load("type_i_known_scalar_query_transfer.json")
    triples = load("type_i_cross_endpoint_query_triples.json")
    base = load("type_i_quotient_base_probe.json")
    p, r, h = int(source["field_prime"]), int(source["subgroup_order"]), int(base["candidate_cofactor"])
    G = GX, GY
    scalars = [int(row["scalar"]) for row in transfer["rows"]]
    policy = triples["policies"]["first"]
    config = {row["triple"]: row["bundle_query_indices"] for row in policy["configuration"]}
    anchor = policy["anchor"]
    assert len(config) == 23 and anchor in config

    def point(raw):
        return tuple(map(int, raw))

    def sum_points(values):
        total = None
        for value in values:
            total = add(total, value, p)
        return total

    outputs = {}
    for name, filename in (
        ("cross_cubic_anchor_8_endpoints", "type_i_cross_cubic_anchor_degree_eight.json"),
    ):
        receipt = load(filename)
        assert receipt["status"] == "verified_cross_cubic_anchor_degree_eight"
        new_count = len(receipt["new_endpoint_geometry"])
        edges = {}
        for edge in source["edges"] + receipt["new_split_edges"]:
            key = edge["query_index"], edge["root_index"], edge["selected_endpoint"]
            assert key not in edges
            edges[key] = tuple(point(Q) for Q in edge["projected_residuals"])
        bundles = {}
        by_endpoint = defaultdict(list)
        for u in range(81 + new_count):
            for i in range(27):
                if all((i, root, u) in edges for root in range(3)):
                    bundles[i, u] = tuple(Q for root in range(3) for Q in edges[i, root, u])
                    by_endpoint[u].append(i)
        assert len(bundles) == receipt["all_complete_bundles"]
        old_active = {(i, u) for u, items in by_endpoint.items()
                      if len(items) >= 2 for i in items}
        cross_bundle_keys = {(config[j][root], 3 * j + root)
                             for j in config for root in range(3)}
        assert cross_bundle_keys <= bundles.keys()
        selected_keys = old_active | cross_bundle_keys
        values = sorted({Q for key in selected_keys for Q in bundles[key]})
        index = {Q: k for k, Q in enumerate(values)}

        def row_for(left, right):
            coefficients = Counter(index[Q] for Q in left)
            coefficients.subtract(index[Q] for Q in right)
            return {k: value % r for k, value in coefficients.items() if value % r}

        endpoint_rows = [row_for(bundles[i, u], bundles[j, u])
                         for u in range(81 + new_count)
                         for i, j in combinations(by_endpoint[u], 2)]
        assert len(endpoint_rows) == receipt["endpointwise_rows"]

        def triple_values(j):
            return tuple(Q for root in range(3)
                         for Q in bundles[config[j][root], 3 * j + root])

        cross_rows = []
        for j in sorted(config):
            if j == anchor:
                continue
            left, right = triple_values(j), triple_values(anchor)
            weight_left = 3 * scalars[j] + sum(scalars[i] for i in config[j])
            weight_right = 3 * scalars[anchor] + sum(scalars[i] for i in config[anchor])
            scalar = (-4 * h * (weight_left - weight_right)) % r
            assert add(sum_points(left), neg(sum_points(right), p), p) == mul(scalar, G, p)
            cross_rows.append(row_for(left, right))
        assert len(cross_rows) == 22
        endpoint_rank = rank_sparse(endpoint_rows, r)
        total_rank = rank_sparse(endpoint_rows + cross_rows, r)
        assert rank_sparse_min(endpoint_rows, r) == endpoint_rank
        assert rank_sparse_min(endpoint_rows + cross_rows, r) == total_rank
        assert endpoint_rank == receipt["exact_rank"]
        degree = Counter(Q for key in selected_keys for Q in set(bundles[key]))
        active = {values[k] for row in endpoint_rows + cross_rows for k in row}
        incidences = sum(len(set(bundles[key])) for key in selected_keys)
        assert incidences == sum(k * count for k, count in Counter(degree.values()).items())
        assert max(degree.values()) >= 8
        assert len(receipt["anchor_root_active_bundle_hits"]) == 8
        outputs[name] = {
            "receipt_sha256": sha(HERE / filename),
            "endpointwise_rows": len(endpoint_rows),
            "cross_cubic_rows": len(cross_rows),
            "endpointwise_rank": endpoint_rank,
            "combined_rank": total_rank,
            "active_columns": len(active),
            "selected_support_columns": len(values),
            "selected_bundle_count": len(selected_keys),
            "distinct_bundle_column_incidences": incidences,
            "necessary_max_degree_for_closure_at_frozen_rank":
                (incidences + total_rank - 1) // total_rank,
            "max_selected_bundle_degree": max(degree.values()),
            "selected_bundle_degree_histogram": dict(sorted((str(k), count)
                for k, count in Counter(degree.values()).items())),
        }
    out = {"status": "verified_cross_cubic_anchor_combined_rows",
           "candidate_id": None, "source_sha256": sha(__file__),
           "input_sha256": {name: sha(HERE / name) for name in (
               "type_i_known_scalar_query_cross_fan.json",
               "type_i_known_scalar_query_transfer.json",
               "type_i_cross_endpoint_query_triples.json",
               "type_i_quotient_base_probe.json",
               "type_i_cross_cubic_anchor_degree_eight.json")},
           "policies": outputs}
    path = HERE / "type_i_cross_cubic_anchor_combined_rows.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("CROSS_CUBIC_ANCHOR_COMBINED_ROWS_PASS", json.dumps(outputs, sort_keys=True))


if __name__ == "__main__":
    main()
