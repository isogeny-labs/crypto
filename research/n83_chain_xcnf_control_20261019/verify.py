#!/usr/bin/env python3
"""Independently check a CryptoMiniSat assignment for the pinned N83 XCNF."""
import hashlib
import json
import sys
from pathlib import Path

N = 83
MASK = (1 << N) - 1
LOW = (1 << 45) | (1 << 2) | (1 << 1) | 1
FIXTURE_SHA256 = 'fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538'
DOMAINS_SHA256 = '63accc5b9dc9de6327c5b07e93b3e62717f8376a96c3f91f7bb70b77095f23bb'


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def mul(a, b):
    out = 0
    while b:
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & (1 << N):
            a ^= (1 << N) | LOW
    return out & MASK


def s3(x, y, z):
    xy = mul(x, y)
    xz = mul(x, z)
    yz = mul(y, z)
    return mul(xy, z) ^ mul(xy ^ xz ^ yz, xy ^ xz ^ yz) ^ 1


def inverse(value):
    if value == 0:
        raise ValueError('zero has no field inverse')
    result = 1
    power = value
    exponent = (1 << N) - 2
    while exponent:
        if exponent & 1:
            result = mul(result, power)
        power = mul(power, power)
        exponent >>= 1
    if mul(result, value) != 1:
        raise ValueError('field inverse replay differs')
    return result


def on_curve(point):
    x, y = point
    return mul(y, y) ^ mul(x, y) == mul(mul(x, x), x) ^ 1


def point_add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2:
        if y1 ^ y2 == x1:
            return None
        if x1 == 0 or y1 != y2:
            raise ValueError('invalid equal-x point pair')
        slope = x1 ^ mul(y1, inverse(x1))
        x3 = mul(slope, slope) ^ slope
        y3 = mul(x1, x1) ^ mul(slope ^ 1, x3)
    else:
        slope = mul(y1 ^ y2, inverse(x1 ^ x2))
        x3 = mul(slope, slope) ^ slope ^ x1 ^ x2
        y3 = mul(slope, x1 ^ x3) ^ x3 ^ y1
    result = x3, y3
    if not on_curve(result):
        raise ValueError('sum fails curve equation')
    return result


def assignment(path, variables):
    lines = Path(path).read_text().splitlines()
    statuses = [line for line in lines if line.startswith('s ')]
    if len(statuses) != 1:
        raise ValueError('expected one solver status')
    status = statuses[0].split(maxsplit=1)[1]
    if status != 'SATISFIABLE':
        return status, None
    model = [None] * (variables + 1)
    for line in lines:
        if not line.startswith('v '):
            continue
        for raw in line.split()[1:]:
            lit = int(raw)
            if lit == 0:
                continue
            idx = abs(lit)
            if idx < 1 or idx > variables or model[idx] is not None:
                raise ValueError('duplicate or out-of-range model literal')
            model[idx] = lit > 0
    if any(value is None for value in model[1:]):
        raise ValueError('incomplete solver assignment')
    return status, model


def check_constraints(path, model, variables, declared):
    clauses = xors = 0
    with Path(path).open() as stream:
        header = stream.readline().split()
        if header != ['p', 'cnf', str(variables), str(declared)]:
            raise ValueError('XCNF header differs')
        for line in stream:
            words = line.split()
            if not words:
                continue
            is_xor = words[0] == 'x'
            lits = [int(word) for word in words[1 if is_xor else 0:]]
            if not lits or lits[-1] != 0:
                raise ValueError('unterminated XCNF row')
            lits.pop()
            if any(abs(lit) < 1 or abs(lit) > variables for lit in lits):
                raise ValueError('XCNF literal out of range')
            satisfied = [model[abs(lit)] == (lit > 0) for lit in lits]
            if is_xor:
                xors += 1
                if sum(satisfied) % 2 != 1:
                    raise ValueError(f'XOR row {xors} is false')
            else:
                clauses += 1
                if not any(satisfied):
                    raise ValueError(f'CNF clause {clauses} is false')
    if clauses + xors != declared:
        raise ValueError('XCNF row count differs')
    return clauses, xors


def block(model, number):
    return sum(int(model[number * N + bit + 1]) << bit for bit in range(N))


def main(xcnf, solver_output, domains, export):
    if sha256(domains) != DOMAINS_SHA256:
        raise ValueError('retained domains bytes differ')
    data = json.loads(Path(domains).read_text())
    receipt = json.loads(Path(export).read_text())
    if (receipt.get('status') != 'EXPORTED_XCNF' or
            receipt.get('fixture_sha256') != FIXTURE_SHA256 or
            receipt.get('input_blake3') is None or
            receipt.get('fixed_x_blocks') != 8 or
            receipt.get('identity_mask') != 0 or
            receipt.get('xcnf_bytes') != Path(xcnf).stat().st_size):
        raise ValueError('export receipt differs')
    if receipt.get('xcnf_sha256') and receipt['xcnf_sha256'] != sha256(xcnf):
        raise ValueError('XCNF bytes differ from export receipt')
    variables = receipt['variables']
    declared = receipt['clauses'] + receipt['xors']
    status, model = assignment(solver_output, variables)
    if status != 'SATISFIABLE':
        print(json.dumps({'status': 'UNSAT_fixed_witness_encoding' if status == 'UNSATISFIABLE'
                          else 'UNKNOWN_external_solver',
                          'solver_status': status, 'xcnf_sha256': sha256(xcnf)}, sort_keys=True))
        return 2 if status == 'UNSATISFIABLE' else 0
    clauses, xors = check_constraints(xcnf, model, variables, declared)
    if (clauses, xors) != (receipt['clauses'], receipt['xors']):
        raise ValueError('exported constraint counts differ')
    actual = [block(model, i) for i in range(9)]
    expected = [int(value) for value in data['ordered_xs'] + data['partial_xs'] +
                [data['target_x']]]
    if actual != expected:
        raise ValueError('external model differs from pinned witness blocks')
    a, b, c, d, e, ab, abc, abcd, target = actual
    if any((s3(x, y, z) != 0 for x, y, z in
            ((a, b, ab), (ab, c, abc), (abc, d, abcd), (abcd, e, target)))):
        raise ValueError('external model fails original-field S3 replay')
    fixture = json.loads((Path(__file__).resolve().parent.parent /
                          'koblitz_n83_factor_base_sweep_20261008/verification/'
                          'mixed-orbit-known-witness-source-20261012/fixture.json').read_text())
    if sha256(Path(__file__).resolve().parent.parent /
              'koblitz_n83_factor_base_sweep_20261008/verification/'
              'mixed-orbit-known-witness-source-20261012/fixture.json') != FIXTURE_SHA256:
        raise ValueError('curve witness fixture bytes differ')
    points = [tuple(map(int, fixture['terms'][i]['point'])) for i in (1, 0, 2, 3, 4)]
    if not all(map(on_curve, points)):
        raise ValueError('witness point fails curve equation')
    partials = []
    current = None
    for point in points:
        current = point_add(current, point)
        if current is None:
            raise ValueError('intermediate sum is identity')
        partials.append(current)
    if ([point[0] for point in partials[1:]] != [ab, abc, abcd, target] or
            current != tuple(map(int, fixture['equation']['target']))):
        raise ValueError('point-sum witness differs from model or target')
    print(json.dumps({'status': 'PASS_EXTERNAL_XCNF_AND_ORIGINAL_FIELD',
                      'variables': variables, 'clauses': clauses, 'xors': xors,
                      'xcnf_sha256': sha256(xcnf), 'fixed_blocks': 8,
                      'curve_point_sum_verified': True}, sort_keys=True))
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 5:
        raise SystemExit('usage: verify.py XCNF SOLVER_STDOUT DOMAINS_JSON EXPORT_JSON')
    raise SystemExit(main(*sys.argv[1:]))
