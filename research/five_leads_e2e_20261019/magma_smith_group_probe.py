#!/usr/bin/env python3
"""Preserve one public Magma order/Euler-factor probe, including failures.

An order or Euler factor is necessary group data, not an odd-kernel screen.
The browser calculator has its own compute limits; it is an external oracle.
"""

import argparse
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ENDPOINT = 'https://calculator.magma-maths.org/execute'


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--index', type=int)
    parser.add_argument('--control', action='store_true')
    parser.add_argument('--timeout', type=int, default=90)
    args = parser.parse_args()
    assert args.control != (args.index is not None)
    if args.control:
        p = 257
        coefficients = [int(c) for c in json.loads((HERE / 'fixtures/tian-genus3-f257-t1.json').read_text())['odd_curve']['f']]
        source = {'kind': 'tian-f257-control'}
    else:
        gate = json.loads((HERE / 'correctness/smith-odd-model-gate-v1.json').read_text())
        row = next(r for r in gate['rows'] if r['index'] == args.index)
        assert row['odd_models'], 'no Fp-rational one-infinity model'
        p = gate['p']
        coefficients = row['odd_models'][0]['h_ascending']
        source = {'kind': 'smith-frozen', 'index': args.index, 'root': row['odd_models'][0]['root']}
    terms = [f'{c}*x^{i}' for i, c in enumerate(coefficients) if c]
    code = (f'SetColumns(0); P<x>:=PolynomialRing(GF({p})); f:=' + ' + '.join(terms)
            + '; J:=Jacobian(HyperellipticCurve(f)); print "ORDER"; print Order(J); print "EULER"; print EulerFactor(J);')
    process = subprocess.run(['curl', '-sS', '-m', str(args.timeout), '-H', 'Content-Type: application/json',
                              '--data-binary', '@-', ENDPOINT], input=json.dumps({'code': code}),
                             text=True, capture_output=True, check=False)
    try:
        response = json.loads(process.stdout)
    except json.JSONDecodeError:
        response = {'raw_stdout': process.stdout}
    order = None
    if process.returncode == 0 and response.get('success'):
        lines = response.get('stdout', '').splitlines()
        if 'ORDER' in lines:
            order = int(lines[lines.index('ORDER') + 1])
        if args.control:
            assert order % 690212 == 0
    output = {'schema': 'magma-smith-group-probe/v1', 'source': source, 'p': p,
              'f_ascending': coefficients, 'endpoint': ENDPOINT, 'code': code,
              'timeout_seconds': args.timeout,
              'curl_returncode': process.returncode, 'curl_stderr': process.stderr,
              'response': response, 'order': order,
              'order_divisible_by_3': None if order is None else order % 3 == 0,
              'order_divisible_by_5': None if order is None else order % 5 == 0,
              'odd_kernel_screen_executed': False}
    print(json.dumps(output, sort_keys=True))


if __name__ == '__main__':
    main()
