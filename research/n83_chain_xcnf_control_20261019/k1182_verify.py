#!/usr/bin/env python3
"""Independently replay an ordinary N83 K1182/866 SAT model through the group."""
import itertools
import json
import sys
from pathlib import Path

from verify import (N, assignment, block, check_constraints, inverse, mul,
                    on_curve, point_add, s3, sha256)

HERE = Path(__file__).resolve().parent
PROTOCOL = HERE / 'k1182_protocol.json'
DOMAINS = (HERE.parent / 'koblitz_n83_factor_base_sweep_20261008/verification/'
           'known-witness-ordered-sat-20261014/domains.json')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def square(value):
    return mul(value, value)


def orbit_xs(representatives):
    codes = set()
    for raw in representatives:
        value = int(raw)
        for _ in range(N):
            require(value not in codes, 'signed-Frobenius x-domain overlaps')
            codes.add(value)
            value = square(value)
    require(len(codes) == len(representatives) * N,
            'signed-Frobenius x-domain cardinality differs')
    return codes


def lift_x(x):
    if x == 0:
        point = (0, 1)
        require(on_curve(point), 'zero-x point fails curve equation')
        return [point]
    inverse_x = inverse(x)
    target = x ^ square(inverse_x)
    half_trace = 0
    power = target
    for _ in range((N + 1) // 2):
        half_trace ^= power
        power = square(square(power))
    require(square(half_trace) ^ half_trace == target,
            'x-coordinate does not lift to the curve')
    y = mul(x, half_trace)
    points = [(x, y), (x, y ^ x)]
    require(all(map(on_curve, points)), 'lifted point fails curve equation')
    return points


def verify(xcnf, solver_output, export):
    protocol = json.loads(PROTOCOL.read_text())
    domains = json.loads(DOMAINS.read_text())
    receipt = json.loads(Path(export).read_text())
    require(sha256(DOMAINS) == protocol['domain_input_sha256'] and
            sha256(xcnf) == protocol['xcnf_sha256'] and
            receipt['query_point'] == protocol['query_point'] and
            receipt['variables'] == protocol['variables'] and
            receipt['clauses'] == protocol['cnf_clauses'] and
            receipt['xors'] == protocol['xor_rows'],
            'frozen input or export receipt differs')
    status, model = assignment(solver_output, protocol['variables'])
    if status != 'SATISFIABLE':
        print(json.dumps({'status': 'UNKNOWN_solver_cap' if status in ('INDETERMINATE', 'UNKNOWN')
                          else 'UNVERIFIED_solver_status',
                          'solver_status': status,
                          'candidate_id': None,
                          'selected_best_total_runtime': None}, sort_keys=True))
        return 0
    clauses, xors = check_constraints(
        xcnf, model, protocol['variables'],
        protocol['cnf_clauses'] + protocol['xor_rows'])
    require((clauses, xors) == (protocol['cnf_clauses'], protocol['xor_rows']),
            'XCNF row counts differ')
    values = [block(model, i) for i in range(9)]
    a, b, c, d, e, ab, abc, abcd, target = values
    query = tuple(map(int, protocol['query_point']))
    require(target == query[0] and
            all(s3(x, y, z) == 0 for x, y, z in
                ((a, b, ab), (ab, c, abc), (abc, d, abcd), (abcd, e, target))),
            'original-field chained S3 replay differs')
    small = orbit_xs(domains['small_reps'])
    residual = orbit_xs(domains['residual_reps'])
    require(all(x in small for x in (a, b, c)) and
            all(x in residual for x in (d, e)),
            'model summand leaves declared signed-orbit domains')
    choices = [lift_x(x) for x in (a, b, c, d, e)]
    for points in itertools.product(*choices):
        current = None
        for point in points:
            current = point_add(current, point)
        if current == query:
            print(json.dumps({
                'status': 'PASS_VERIFIED_NATURAL_PARTIAL_RELATION',
                'query_point': list(map(str, query)),
                'summands': [[str(x), str(y)] for x, y in points],
                'xcnf_clauses_checked': clauses,
                'xor_rows_checked': xors,
                'candidate_id': None,
                'selected_best_total_runtime': None,
            }, sort_keys=True))
            return 0
    raise ValueError('SAT model has no curve-group lift to the query point')


if __name__ == '__main__':
    if len(sys.argv) != 4:
        raise SystemExit('usage: k1182_verify.py XCNF SOLVER_STDOUT EXPORT_JSON')
    raise SystemExit(verify(*sys.argv[1:]))
