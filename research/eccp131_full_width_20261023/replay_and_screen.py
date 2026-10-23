#!/usr/bin/env python3
"""Independently replay the GP tower and screen the exact ECCp-131 curve."""
import argparse
import ast
import hashlib
import json
import re
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'eccp131_structured_ic_20261022'))
from screen import add, lift, mul, parameters, screen_base  # noqa: E402


def residues(text, p):
    def replace(m):
        assert int(m[2]) == p
        return m[1]
    return re.sub(r'Mod\((\d+), (\d+)\)', replace, text)


def point(text, p):
    q = ast.literal_eval(residues(text, p))
    assert isinstance(q, list) and len(q) == 2
    return tuple(q)


def on_curve(P, p, a, b):
    return P is None or (P[1]*P[1] - P[0]**3 - a*P[0] - b) % p == 0


def velu_map(P, xi, tau, p):
    if P is None or P[0] == xi:
        return None
    d = pow((P[0] - xi) % p, -1, p)
    return ((P[0] + tau*d) % p, P[1]*(1 - tau*d*d) % p)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('gp_output', type=Path)
    args = ap.parse_args()
    start = time.perf_counter()
    lines = args.gp_output.read_text().splitlines()
    assert lines[-1] == 'FOUND 1'
    certs = [line for line in lines if line.startswith('CERT ')]
    assert len(certs) == 1
    m = re.fullmatch(r'CERT (\d+) (\d+) (\[[^]]+\]) (\[[^]]+\]) (\[[^]]+\])', certs[0])
    assert m is not None
    b, n = int(m[1]), int(m[2])
    vals = parameters()
    p = vals['p']
    P, Q, R = (point(m[i], p) for i in (3, 4, 5))
    orders = [(int(z[1]), int(z[2])) for line in lines
              if (z := re.fullmatch(r'ORDER (\d+) (\d+)', line))]
    assert len(orders) <= 512 and orders == list(enumerate((v for _, v in orders), start=1))
    assert orders[-1] == (b, n) and n % 256 == 0 and (4 + 27*b*b) % p != 0
    assert all(on_curve(pt, p, 1, b) for pt in (P, Q, R))
    assert mul(n, P, p, 1) is None and mul(n//256, P, p, 1) == Q
    assert mul(128, Q, p, 1) is not None and mul(256, Q, p, 1) is None
    assert mul(256, R, p, 1) is not None
    xs = [int(residues(line.split(' ', 2)[2], p)) for line in lines if line.startswith('X ')]
    assert len(xs) == 256 and len(set(xs)) == 256
    assert [int(line.split()[1]) for line in lines if line.startswith('X ')] == list(range(256))
    T = R
    for x in xs:
        assert T is not None and T[0] == x
        T = add(T, Q, p, 1)
    assert T == R

    aj, bj, qj, rj = 1, b, Q, R
    steps = []
    for j in range(8):
        kernel = mul(1 << (7-j), qj, p, aj)
        assert kernel is not None and kernel[1] == 0
        xi = kernel[0]
        tau = (3*xi*xi + aj) % p
        steps.append({'xi': hex(xi), 'tau': hex(tau), 'a': hex(aj), 'b': hex(bj)})
        qj, rj = velu_map(qj, xi, tau, p), velu_map(rj, xi, tau, p)
        aj, bj = (aj - 5*tau) % p, (bj - 7*xi*tau) % p
        assert on_curve(qj, p, aj, bj) and on_curve(rj, p, aj, bj)
    assert qj is None and rj is not None
    c = rj[0]
    for x in xs:
        y = x
        for s in steps:
            xi, tau = int(s['xi'], 16), int(s['tau'], 16)
            assert y != xi
            y = (y + tau*pow((y-xi) % p, -1, p)) % p
        assert y == c

    base = [pt for x in xs if (pt := lift(x, vals)) is not None]
    assert len(base) >= 2
    for pt in ((vals['gx'], vals['gy']), (vals['qx'], vals['qy'])):
        assert on_curve(pt, p, vals['a'], vals['b'])
        assert mul(vals['n'], pt, p, vals['a']) is None
    screen = screen_base(base, vals, 131, 256, 256)
    assert len(screen['planted_controls']) == 8
    out = {'schema':'eccp131-pkm-full-width/v1',
           'input_sha256':hashlib.sha256(args.gp_output.read_bytes()).hexdigest(),
           'parameters':{k:hex(v) for k,v in vals.items()},
           'auxiliary':{'a':hex(1),'b':hex(b),'order':hex(n),'P':[hex(z) for z in P],
                        'Q':[hex(z) for z in Q],'R':[hex(z) for z in R]},
           'tower':{'t':8,'x':[hex(x) for x in xs],'steps':steps,'image_x':hex(c),
                    'lifted_columns':len(base)},
           'screen':screen,'elapsed_seconds':time.perf_counter()-start}
    print(json.dumps(out,separators=(',',':')))


if __name__ == '__main__':
    main()
