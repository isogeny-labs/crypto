#!/usr/bin/env python3
"""Pair-count model of the mean exact coverage (DESK.md section 4). Uses only r, p, n, t and the
2-part structure of each arm; no census data. Frozen with the protocol; it produces the numeric
predictions of PROTOCOL.md.

  python3 model.py            # prints the prediction table for k = 16, 18, 20, t = 4..8

Model. A degree-2 base is N fibres {a, tau a}, N ~ Bin(2^t, q), q = n/(2p); points have uniform
independent 2-part cosets c in G2 (|G2| = 8); tau acts on cosets by c -> t_T - c. Distinct differences
in coset z: cross-fibre N(N-1)/4 plus within-fibre N * w2(z), w2(z) = (#{2c = t_T + z} + #{2c = t_T - z})/8.
A degree-4 (Jacobi-intersection) base is N4 orbits of a Klein group, N4 ~ Bin(2^t, n/(4p)): cross-orbit
N4(N4-1)/2, within-orbit N4 * w4. Saturation: covered = (r-1)(1 - exp(-M/(r-1))).
"""
import json, math
import curves as cv


def g2(k):
    """2-part cosets as (a mod m2, e) and the size m2 of the cyclic factor."""
    return None


class G2:
    def __init__(self, kind):
        self.kind = kind
        self.m2, self.ne = (8, 1) if kind == 'E' else (4, 2)
        self.els = [(a, e) for a in range(self.m2) for e in range(self.ne)]

    def add(self, x, y):
        return ((x[0] + y[0]) % self.m2, (x[1] + y[1]) % 2)

    def neg(self, x):
        return ((-x[0]) % self.m2, x[1])

    def count2(self, z):
        return sum(1 for c in self.els if self.add(c, c) == (z[0] % self.m2, z[1] % 2))


def sat(M, r):
    return (r - 1) * (1 - math.exp(-M / (r - 1)))


def moments(nv, q):
    return nv * q, nv * (nv - 1) * q * q          # E[N], E[N(N-1)]


def M2(G, nv, q, tT, z):
    EN, ENN = moments(nv, q)
    w = (G.count2(G.add(tT, z)) + G.count2(G.add(tT, G.neg(z)))) / 8
    return ENN / 4 + EN * w


def arm_model(arm, k, t, facts):
    """Predicted mean covered_D and covered_full (None where not modelled)."""
    p, n, r = facts['p'], facts['n'], facts['r']
    nv = 1 << t
    q = n / (2 * p)
    GE, GEp = G2('E'), G2('Ep')
    O = (0, 0)
    if arm in ('E.x', 'E.x_noself', 'Ep.x'):
        G = GE if arm.startswith('E.') else GEp
        if arm == 'E.x_noself':
            EN, ENN = moments(nv, q)
            return dict(D=sat(ENN / 4, r), full=sat(ENN / 4, r))
        m = M2(G, nv, q, O, O)
        return dict(D=sat(m, r), full=sat(m, r))
    if arm in ('E.x|shift_odd',) or arm.startswith('Ep.x|shift'):
        G = GE if arm.startswith('E.') else GEp
        ts = classes_t(arm.split('|shift_')[1], G)
        vals = [sat(M2(G, nv, q, O, O) + M2(G, nv, q, O, G.neg(tt)) + M2(G, nv, q, O, tt), r) for tt in ts]
        return dict(D=None, full=sum(vals) / len(vals))
    if arm.startswith('E.iota') or arm.startswith('Ep.iota') or arm.startswith('Ep.huff'):
        G = GE if arm.startswith('E.') else GEp
        if arm == 'E.iota_T0':
            ts, two = [(4, 0)], True
        elif arm.startswith('Ep.huff'):
            ts, two = [(0, 1), (2, 1)], True            # T is a 2-torsion point of class (0,1)
        else:
            cls = {'E.iota_even': 'even', 'E.iota_odd': 'odd', 'Ep.iota_2E': '00',
                   'Ep.iota_c01': '01', 'Ep.iota_c10': '10', 'Ep.iota_c11': '11'}[arm]
            ts, two = classes_t(cls, G), False
        Ds, Fs = [], []
        for tt in ts:
            m0 = M2(G, nv, q, tt, O)
            if two:
                mf = m0 + M2(G, nv, q, tt, tt)
            else:
                mf = m0 + M2(G, nv, q, tt, G.neg(tt)) + M2(G, nv, q, tt, tt)
            Ds.append(sat(m0, r)); Fs.append(sat(mf, r))
        return dict(D=sum(Ds) / len(Ds), full=sum(Fs) / len(Fs))
    if arm.startswith('Ep.ji'):
        q4 = n / (4 * p)
        EN, ENN = moments(nv, q4)
        w = facts['w4'][arm]
        m = ENN / 2 + EN * w
        return dict(D=sat(m, r), full=sat(m, r) if arm in ('Ep.ji_c', 'Ep.ji_d') else None)
    raise KeyError(arm)


def classes_t(cls, G):
    """Coset values t_T for T uniform in a class (uniform over the cosets the class contains)."""
    if G.kind == 'E':
        return [(a, 0) for a in range(8) if (a % 2 == 1) == (cls == 'odd')]
    want = {'00': (0, 0), '01': (0, 1), '10': (1, 0), '11': (1, 1)}[cls]
    return [(a, e) for (a, e) in G.els if (a % 2, e) == want]


def facts_for(k):
    F = cv.FAMILY[k]
    p, A, n = F['p'], F['A'], F['n']
    sq = lambda v: cv.legendre(v, p) == 1
    # w4: s -> 1; c (translation by e_beta = (A-2,0)) -> 1 + [A-2 square]; d (e_alpha = (A+2,0)) -> 1 + [A+2 square]
    return dict(p=p, A=A, n=n, r=n // 8,
                w4={'Ep.ji_s': 1, 'Ep.ji_c': 1 + sq(A - 2), 'Ep.ji_d': 1 + sq(A + 2)})


ARMS = ['E.x', 'E.x_noself', 'E.iota_even', 'E.iota_T0', 'E.iota_odd', 'Ep.x', 'Ep.iota_2E',
        'Ep.iota_c01', 'Ep.iota_c10', 'Ep.iota_c11', 'Ep.huff_x', 'Ep.huff_y', 'Ep.ji_s', 'Ep.ji_c', 'Ep.ji_d',
        'E.x|shift_odd', 'Ep.x|shift_01', 'Ep.x|shift_10', 'Ep.x|shift_11']


def table(ks=(16, 18, 20), ts=(4, 5, 6, 7, 8)):
    out = {}
    for k in ks:
        f = facts_for(k)
        for t in ts:
            for arm in ARMS:
                out[f'{k}|{t}|{arm}'] = arm_model(arm, k, t, f)
    return out


if __name__ == '__main__':
    T = table()
    ks, ts = (16, 18, 20), (4, 5, 6, 7, 8)
    print('Predicted mean covered_D (and ratio to the matched x-null)')
    for arm in ARMS[:15]:
        null = 'E.x' if arm.startswith('E.') else 'Ep.x'
        row = []
        for k in ks:
            for t in ts:
                d = T[f'{k}|{t}|{arm}']['D']; d0 = T[f'{k}|{t}|{null}']['D']
                row.append(f'{d:8.1f}({d / d0:.3f})')
        print(f'{arm:13s}', ' '.join(row))
    print('Predicted mean covered_full; iota arms vs x-null with the same T-shift class')
    for arm, ref in (('E.iota_odd', 'E.x|shift_odd'), ('Ep.iota_c01', 'Ep.x|shift_01'),
                     ('Ep.iota_c10', 'Ep.x|shift_10'), ('Ep.iota_c11', 'Ep.x|shift_11')):
        row = []
        for k in ks:
            for t in ts:
                a = T[f'{k}|{t}|{arm}']['full']; b = T[f'{k}|{t}|{ref}']['full']
                row.append(f'{a:8.1f}({a / b:.3f})')
        print(f'{arm:13s}', ' '.join(row))
    json.dump(T, open('/dev/stdout', 'w') if False else open(__file__.replace('model.py', '../model_predictions.json'), 'w'), indent=0)
