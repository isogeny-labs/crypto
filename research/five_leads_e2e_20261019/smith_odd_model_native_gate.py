#!/usr/bin/env python3
"""Replay all convertible Smith models through checked native Jacobian code."""

import argparse
import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
P = 2147483647


def evaluate(f, x):
    result = 0
    for c in reversed(f):
        result = (result * x + c) % P
    return result


def native(binary, curve, operation, **kwargs):
    request = {'schema': 'hyperelliptic-jacobian-operation/v1', 'odd_curve': curve, 'operation': operation, **kwargs}
    process = subprocess.run([binary, 'arithmetic', '--input', '-'], input=json.dumps(request), text=True, capture_output=True, check=False)
    assert process.returncode == 0, (request, process.stderr, process.stdout)
    response = json.loads(process.stdout)
    assert response['status'] == 'ok' and response['result']['operation'] == operation
    return response['result']['result']


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', default='target/release/hyperelliptic-cover')
    args = parser.parse_args()
    models = json.loads((HERE / 'correctness/smith-odd-model-gate-v1.json').read_text())
    source = {r['index']: r for r in map(json.loads, (HERE / 'correctness/smith-negative-materialized-v1.jsonl').read_text().splitlines())}
    output = []
    for row in models['rows']:
        if not row['odd_models']:
            continue
        model = row['odd_models'][0]
        r, derivative, h = model['root'], model['derivative'], model['h_ascending']
        curve = {'family': 'odd_prime_one_infinity_v1', 'p': str(P), 'genus': 3, 'f': list(map(str, h)), 'infinity': 'one_rational'}
        assert native(args.binary, curve, 'identity')['divisor'] == {'u': ['0x1'], 'v': []}
        f = source[row['index']]['f_ascending']
        for x in range(1000):
            if x == r:
                continue
            square = evaluate(f, x)
            if square and pow(square, (P - 1) // 2, P) == 1:
                break
        else:
            raise AssertionError('no sampled affine source point')
        y = pow(square, (P + 1) // 4, P)
        assert y * y % P == square
        z = pow(derivative * (x - r) % P, -1, P)
        w = pow(z, 4, P) * y % P
        assert w * w % P == evaluate(h, z)
        point = {'u': [str((-z) % P), '1'], 'v': [str(w)]}
        native(args.binary, curve, 'validate', left=point)
        for infinity_w in model['old_infinity_images_w_at_z0']:
            native(args.binary, curve, 'validate', left={'u': ['0', '1'], 'v': [str(infinity_w)]})
        bad_point = {'u': point['u'], 'v': [str((w + 1) % P)]}
        bad_request = {'schema': 'hyperelliptic-jacobian-operation/v1', 'odd_curve': curve, 'operation': 'validate', 'left': bad_point}
        bad = subprocess.run([args.binary, 'arithmetic', '--input', '-'], input=json.dumps(bad_request), text=True, capture_output=True, check=False)
        assert bad.returncode == 3 and json.loads(bad.stdout)['status'] == 'invalid_input'
        d3 = native(args.binary, curve, 'scalar_multiply', left=point, scalar='3')['divisor']
        d5 = native(args.binary, curve, 'scalar_multiply', left=point, scalar='5')['divisor']
        d8 = native(args.binary, curve, 'scalar_multiply', left=point, scalar='8')['divisor']
        summed = native(args.binary, curve, 'add', left=d3, right=d5)['divisor']
        assert summed == d8, (row['index'], summed, d8)
        dp = native(args.binary, curve, 'scalar_multiply', left=point, scalar=str(P))['divisor']
        dp1 = native(args.binary, curve, 'scalar_multiply', left=point, scalar=str(P + 1))['divisor']
        assert native(args.binary, curve, 'add', left=dp, right=point)['divisor'] == dp1
        output.append({'index': row['index'], 'root': r, 'source_point': [x, y], 'odd_point': [z, w], 'old_infinity_images_checked': len(model['old_infinity_images_w_at_z0']), 'corrupted_point_rejected': True, 'three_plus_five_equals_eight': True, 'long_scalar_additive': True, 'd8': d8})
    binary_digest = hashlib.sha256(Path(args.binary).read_bytes()).hexdigest()
    print(json.dumps({'schema': 'smith-odd-model-native-gate/v1', 'p': P, 'binary': args.binary, 'binary_sha256': binary_digest, 'tested_count': len(output), 'odd_kernel_screen_executed': False, 'rows': output}, sort_keys=True))


if __name__ == '__main__':
    main()
