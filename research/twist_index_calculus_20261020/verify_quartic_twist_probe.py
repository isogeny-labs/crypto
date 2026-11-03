"""Independent Fp4/group replay of gls_quartic_twist_probe_v1.ndjson."""
from __future__ import annotations

import collections
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent
Z = (0, 0, 0, 0)
ONE = (1, 0, 0, 0)


class Field:
    def __init__(self, p: int, d: int):
        self.p, self.d, self.q = p, d, p**4

    def add(self, a, b):
        return tuple((x + y) % self.p for x, y in zip(a, b))

    def neg(self, a):
        return tuple(-x % self.p for x in a)

    def sub(self, a, b):
        return self.add(a, self.neg(b))

    def mul(self, a, b):
        z = [0] * 7
        for i in range(4):
            for j in range(4):
                z[i + j] += a[i] * b[j]
        return tuple((z[i] + self.d * z[i + 4] if i < 3 else z[3]) % self.p
                     for i in range(4))

    def power(self, a, n):
        z = ONE
        while n:
            if n & 1:
                z = self.mul(z, a)
            n //= 2
            if n:
                a = self.mul(a, a)
        return z

    def inverse(self, a):
        assert a != Z
        return self.power(a, self.q - 2)


class Curve:
    def __init__(self, f, b):
        self.f, self.b = f, b

    def rhs(self, x):
        return self.f.add(self.f.mul(self.f.mul(x, x), x), self.b)

    def on(self, p):
        return p is None or self.f.mul(p[1], p[1]) == self.rhs(p[0])

    def neg(self, p):
        return None if p is None else (p[0], self.f.neg(p[1]))

    def add(self, a, b):
        if a is None:
            return b
        if b is None:
            return a
        f = self.f
        x, y = a
        u, v = b
        if x == u and f.add(y, v) == Z:
            return None
        if a == b:
            num = f.add(f.add(f.mul(x, x), f.mul(x, x)), f.mul(x, x))
            den = f.add(y, y)
        else:
            num, den = f.sub(v, y), f.sub(u, x)
        slope = f.mul(num, f.inverse(den))
        xx = f.sub(f.sub(f.mul(slope, slope), x), u)
        yy = f.sub(f.mul(slope, f.sub(x, xx)), y)
        return xx, yy

    def scale(self, pt, n):
        z = None
        while n:
            if n & 1:
                z = self.add(z, pt)
            n //= 2
            if n:
                pt = self.add(pt, pt)
        return z


def miller_rabin(n):
    if n < 2:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % p == 0:
            return n == p
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 325, 9375, 28178, 450775, 9780504, 1795265022):
        if a % n == 0:
            continue
        x = pow(a % n, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def unpack(record):
    return tuple(tuple(x) for x in (record['x'], record['y']))


def orbit_hist(points, generators):
    all_points = set(points)
    seen = set()
    hist = collections.Counter()
    for p in points:
        if p in seen:
            continue
        seen.add(p)
        queue = [p]
        for x in queue:
            for action in generators:
                y = action(x)
                assert y in all_points
                if y not in seen:
                    seen.add(y)
                    queue.append(y)
        hist[len(queue)] += 1
    assert seen == all_points
    return {str(k): v for k, v in sorted(hist.items())}


def direct_p13_order(c):
    p, f = c.f.p, c.f
    assert p == 13
    squares = collections.Counter()
    for a in range(p):
        for b in range(p):
            for cc in range(p):
                for d in range(p):
                    y = (a, b, cc, d)
                    squares[f.mul(y, y)] += 1
    n = 1
    for a in range(p):
        for b in range(p):
            for cc in range(p):
                for d in range(p):
                    n += squares[c.rhs((a, b, cc, d))]
    return n


def verify(path):
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    assert [x['p'] for x in rows] == [13, 37, 73, 97, 157, 229, 241, 337]
    base_total = 0
    for row in rows:
        p, r = row['p'], row['r']
        assert p % 12 == 1 and miller_rabin(p) and miller_rabin(r)
        assert r > 4 * p * p
        d = row['d']
        f = Field(p, d)
        t = (0, 1, 0, 0)
        assert pow(d, (p - 1) // 2, p) == p - 1
        assert f.power(t, (f.q - 1) // 2) != ONE
        t3 = f.mul(f.mul(t, t), t)
        expected_b = f.mul(t3, (row['b0'], 0, 0, 0))
        assert expected_b == tuple(row['b_curve'])
        c = Curve(f, expected_b)
        trace = p + 1 - (1 + sum(
            1 if (v := (x*x*x + row['b0']) % p) == 0 else
            (2 if pow(v, (p-1)//2, p) == 1 else 0) for x in range(p)))
        t2 = trace * trace - 2 * p
        t4 = t2 * t2 - 2 * p * p
        assert trace == row['trace_fp'] and r == p**4 + 1 + t4
        cert = unpack(row['order_certificate'])
        assert c.on(cert) and cert is not None and c.scale(cert, r) is None
        if p == 13:
            assert direct_p13_order(c) == r == row['direct_order']
        else:
            assert row['direct_order'] is None
        line = [x for x in range(p) if c.rhs((x,0,0,0)) != Z
                and f.power(c.rhs((x,0,0,0)), (f.q-1)//2) == ONE]
        assert line == row['line_x']
        if row['status'] == 'empty_base':
            assert p == 13 and line == [] and row['base_size'] == 0
            continue
        assert row['status'] == 'pass'
        pts = [(tuple(a), tuple(b)) for a, b in row['base_points']]
        assert len(pts) == row['base_size'] == 2 * len(line)
        assert len(set(pts)) == len(pts)
        assert all(c.on(pt) and pt[0][1:] == (0,0,0) for pt in pts)
        for x in line:
            pair = [pt for pt in pts if pt[0][0] == x]
            assert len(pair) == 2 and pair[1] == c.neg(pair[0])
        base_total += len(pts)
        g = unpack(row['generator'])
        assert g == pts[0] and c.scale(g,r) is None
        omega = row['omega']
        assert omega != 1 and pow(omega,3,p) == 1
        h = tuple(row['h'])
        h2, h3 = f.mul(h,h), f.mul(f.mul(h,h),h)
        z = f.power(t,p-1)
        assert z == (row['z'],0,0,0) and pow(row['z'],2,p) == p-1
        for x in line:
            assert f.mul(h2,(x,0,0,0)) == (x*pow(row['z'],-1,p)%p,0,0,0)
        def psi(pt):
            if pt is None:
                return None
            return f.mul(h2,f.power(pt[0],p)), f.mul(h3,f.power(pt[1],p))
        def cubic(pt):
            if pt is None:
                return None
            return f.mul(pt[0],(omega,0,0,0)), pt[1]
        lp, lc = row['lambda_psi'], row['lambda_cubic']
        assert pow(lp,4,r) == r-1 and pow(lp,8,r)==1
        assert pow(lc,3,r)==1 and lc != 1
        assert c.scale(g,lp)==psi(g) and c.scale(g,lc)==cubic(g)
        assert row['psi_evals_checked'] == 4*len(pts)
        for pt in pts:
            assert psi(psi(psi(psi(pt)))) == c.neg(pt)
            assert cubic(cubic(cubic(pt))) == pt
            assert psi(cubic(pt)) == cubic(psi(pt))
            assert psi(pt)==c.scale(pt,lp) and cubic(pt)==c.scale(pt,lc)
            assert psi(pt)[0] == (pt[0][0]*pow(row['z'],-1,p)%p,0,0,0)
        control = orbit_hist(pts,(c.neg,cubic))
        combined = orbit_hist(pts,(psi,cubic))
        assert control == row['control_orbits'] == {'6':len(pts)//6}
        assert combined == row['combined_orbits'] == {'24':len(pts)//24}
        assert row['control_columns']==len(pts)//6
        assert row['combined_columns']==len(pts)//24
    return {'status':'pass','curves':len(rows),'empty_lines':1,
            'positive_lines':7,'base_points_checked':base_total,
            'group_order_certificates':len(rows),'independent_p13_order':True,
            'all_base_point_actions':True,'orbit_histograms':True}


if __name__ == '__main__':
    path = pathlib.Path(sys.argv[1]) if len(sys.argv)>1 else ROOT/'gls_quartic_twist_probe_v1.ndjson'
    print(json.dumps(verify(path),sort_keys=True))
