"""Replay triple-endpoint, known-scalar rows on the frozen P-192 cover graph.

Each complete six-column bundle at endpoint u has sum 3S-3u-C+kq_i.
Taking one bundle at each of the three endpoints of query cubic j makes the
otherwise unknown endpoint sum C-kq_j cancel between two configurations.
"""

import hashlib
import json
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import GX, GY, add, mul, neg, on_curve

HERE = Path(__file__).resolve().parent


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


def point(raw):
    return tuple(map(int, raw))


def sum_points(values, p):
    result = None
    for value in values:
        result = add(result, value, p)
    return result


def rank_sparse(rows, r):
    basis = {}
    for original in rows:
        row = dict(original)
        for lead, vector in sorted(basis.items(), reverse=True):
            factor = row.pop(lead, 0)
            if factor:
                for index, value in vector.items():
                    if index == lead:
                        continue
                    updated = (row.get(index, 0) - factor * value) % r
                    if updated:
                        row[index] = updated
                    else:
                        row.pop(index, None)
        if row:
            lead = max(row)
            inverse = pow(row[lead], -1, r)
            basis[lead] = {index: value * inverse % r for index, value in row.items()}
    return len(basis)


def main():
    source = load('type_i_known_scalar_query_cross_fan.json')
    replay = load('type_i_known_scalar_query_cross_fan_replay.json')
    transfer = load('type_i_known_scalar_query_transfer.json')
    base = load('type_i_quotient_base_probe.json')
    screen = load('type_i_triple_star_screen.json')
    assert replay['screen_sha256'] == digest(HERE / 'type_i_known_scalar_query_cross_fan.json')
    assert source['source_sha256'] == digest(HERE / 'line_known_scalar_query_cross_fan.py')
    assert replay['exact_rank'] == 181
    p, r, h = int(source['field_prime']), int(source['subgroup_order']), int(base['candidate_cofactor'])
    G, C, S = (GX, GY), point(load('type_i_known_scalar_query_transfer_replay.json')['shared_constant']), point(screen['line_section_constant'])
    assert on_curve(G, p) and mul(r, G, p) is None
    scalars = [int(item['scalar']) for item in transfer['rows']]
    quotients = [point(item['quotient']) for item in transfer['rows']]
    endpoints = [point(item) for item in source['endpoint_quotients']]
    assert len(scalars) == len(quotients) == 27 and len(endpoints) == 81
    for j, item in enumerate(transfer['rows']):
        sigma = sum_points(endpoints[3 * j:3 * j + 3], p)
        assert sigma == point(item['projected_sum'])
        assert add(sigma, mul(2 * h % r, quotients[j], p), p) == C
        assert quotients[j] == mul(-2 * scalars[j] % r, G, p)
    edge = {}
    for item in source['edges']:
        key = (item['query_index'], item['root_index'], item['selected_endpoint'])
        assert key not in edge
        edge[key] = tuple(point(value) for value in item['projected_residuals'])
    bundles = {}
    by_endpoint = defaultdict(list)
    for u in range(81):
        for i in range(27):
            if all((i, root, u) in edge for root in range(3)):
                values = tuple(value for root in range(3) for value in edge[i, root, u])
                assert len(values) == 6 and len(set(values)) == 6
                sigma = point(transfer['rows'][i]['projected_sum'])
                expected = add(add(mul(3, S, p), neg(sigma, p), p), neg(mul(3, endpoints[u], p), p), p)
                assert sum_points(values, p) == expected
                bundles[i, u] = values
                by_endpoint[u].append(i)
    assert len(bundles) == source['complete_three_cut_bundles'] == 258
    eligible = [j for j in range(27) if all(by_endpoint[3 * j + t] for t in range(3))]
    assert len(eligible) > 1
    baseline_endpoints = [u for u in range(81) if len(by_endpoint[u]) >= 2]
    baseline_bundles = {(i, u) for u in baseline_endpoints for i in by_endpoint[u]}
    baseline_columns = set().union(*(set(bundles[key]) for key in baseline_bundles))
    all_columns = sorted({value for values in bundles.values() for value in values})
    column_index = {value: n for n, value in enumerate(all_columns)}

    def difference(left, right):
        coefficients = Counter(column_index[value] for value in left)
        coefficients.subtract(column_index[value] for value in right)
        return {index: coefficient % r for index, coefficient in coefficients.items() if coefficient % r}

    baseline = []
    for u in range(81):
        for i, j in combinations(by_endpoint[u], 2):
            baseline.append(difference(bundles[i, u], bundles[j, u]))
    assert len(baseline) == 370 and rank_sparse(baseline, r) == 181

    results = {}
    for policy in ('first', 'prefer_existing_columns'):
        configuration = {}
        for j in eligible:
            keys = []
            for t in range(3):
                u = 3 * j + t
                options = by_endpoint[u]
                if policy == 'first':
                    chosen = options[0]
                else:
                    chosen = min(options, key=lambda i: (len(set(bundles[i, u]) - baseline_columns), i))
                keys.append((chosen, u))
            configuration[j] = keys
        def columns_for(j):
            return tuple(value for key in configuration[j] for value in bundles[key])
        anchor = eligible[0]
        rows = []
        row_records = []
        for j in eligible[1:]:
            left, right = columns_for(j), columns_for(anchor)
            # Σ X_i(u_j) = 9S-6C+k(3q_j+Σq_i), where q_i=[-2a_i]G.
            left_weight = 3 * scalars[j] + sum(scalars[i] for i, _ in configuration[j])
            right_weight = 3 * scalars[anchor] + sum(scalars[i] for i, _ in configuration[anchor])
            scalar = (-4 * h * (left_weight - right_weight)) % r
            group_difference = add(sum_points(left, p), neg(sum_points(right, p), p), p)
            assert group_difference == mul(scalar, G, p)
            row = difference(left, right)
            assert row and sum_points([mul(v, all_columns[index], p) for index, v in row.items()], p) == group_difference
            rows.append(row)
            row_records.append({'triple': j, 'anchor': anchor, 'scalar_mod_r': str(scalar), 'nonzero_columns': len(row)})
        combined = baseline + rows
        active = {all_columns[index] for row in combined for index in row}
        degrees = Counter(value for key in (baseline_bundles | {key for selected in configuration.values() for key in selected}) for value in bundles[key])
        results[policy] = {
            'eligible_query_triples': eligible,
            'configuration': [{'triple': j, 'bundle_query_indices': [i for i, _ in configuration[j]]} for j in eligible],
            'anchor': anchor, 'cross_triple_rows': row_records,
            'cross_triple_row_count': len(rows), 'combined_row_count': len(combined),
            'baseline_rank': 181, 'combined_rank': rank_sparse(combined, r),
            'active_columns': len(active), 'new_columns_vs_baseline': len(active - baseline_columns),
            'selected_bundle_count': len(baseline_bundles | {key for selected in configuration.values() for key in selected}),
            'max_selected_bundle_degree': max(degrees.values()),
            'selected_bundle_degree_histogram': {str(d): count for d, count in sorted(Counter(degrees.values()).items())},
        }
    out = {'status': 'verified_cross_endpoint_triple_bundle_rows', 'candidate_id': None,
           'input_sha256': {name: digest(HERE / name) for name in (
               'type_i_known_scalar_query_cross_fan.json', 'type_i_known_scalar_query_cross_fan_replay.json',
               'type_i_known_scalar_query_transfer.json', 'type_i_known_scalar_query_transfer_replay.json',
               'type_i_quotient_base_probe.json', 'type_i_triple_star_screen.json')},
           'source_sha256': digest(Path(__file__)),
           'field_prime': str(p), 'subgroup_order': str(r),
           'baseline': {'rows': len(baseline), 'rank': 181, 'columns': len(baseline_columns),
                        'selected_bundles': len(baseline_bundles)}, 'policies': results}
    path = HERE / 'type_i_cross_endpoint_query_triples.json'
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + '\n')
    print('CROSS_ENDPOINT_QUERY_TRIPLES_PASS', json.dumps({policy: {
        k: value for k, value in result.items() if k in ('eligible_query_triples', 'cross_triple_row_count',
             'combined_rank', 'active_columns', 'new_columns_vs_baseline', 'max_selected_bundle_degree')}
        for policy, result in results.items()}, sort_keys=True))


if __name__ == '__main__':
    main()
