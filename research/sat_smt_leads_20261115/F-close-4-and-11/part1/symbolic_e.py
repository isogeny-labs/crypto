"""View E: S4 in elementary symmetric functions of (x1,x2,x3), exact over F2.
Polynomials are sets of exponent tuples (coefficients in F2)."""
import json, random, sys
import numpy as np
from pdplib import Field, S4, B_CURVE


def padd(a, b):
    return a ^ b


def pmul(a, b):
    r = set()
    for m1 in a:
        for m2 in b:
            m = tuple(i + j for i, j in zip(m1, m2))
            r ^= {m}
    return r


def var(i, nv):
    return {tuple(1 if j == i else 0 for j in range(nv))}


def ppow(a, e, nv):
    r = {tuple([0] * nv)}
    for _ in range(e):
        r = pmul(r, a)
    return r


# variables: x1,x2,x3,x4,b
NV = 5
x1, x2, x3, x4, b = (var(i, NV) for i in range(NV))
s, p, t, q = x1 ^ x2, pmul(x1, x2), x3 ^ x4, pmul(x3, x4)
s2, t2, p2, q2 = pmul(s, s), pmul(t, t), pmul(p, p), pmul(q, q)
u = pmul(s2, q2 ^ b) ^ pmul(t2, p2 ^ b)
v = pmul(pmul(s2, q) ^ pmul(t2, p), pmul(p, q2 ^ b) ^ pmul(q, p2 ^ b))
S4poly = pmul(u, u) ^ v

# elementary symmetric in x1..x4
import itertools
sig = []
for d in range(1, 5):
    e = set()
    for c in itertools.combinations(range(4), d):
        e ^= {tuple(1 if j in c else 0 for j in range(NV))}
    sig.append(e)

# symmetric reduction: result as set of (a1,a2,a3,a4,bexp)
res = set()
f = set(S4poly)
while f:
    lead = max(f)  # lex on (x1,x2,x3,x4,b)
    d1, d2, d3, d4, db = lead
    assert d1 >= d2 >= d3 >= d4, lead
    a = (d1 - d2, d2 - d3, d3 - d4, d4)
    term = {tuple([0, 0, 0, 0, db])}
    for i in range(4):
        term = pmul(term, ppow(sig[i], a[i], NV))
    f ^= term
    res ^= {a + (db,)}

# substitute sigma_i = e_i + xR*e_{i-1} (e0=1,e4=0); variables e1,e2,e3,xR,b
e1, e2, e3, xR, bb = (var(i, NV) for i in range(NV))
one = {tuple([0] * NV)}
sub = [e1 ^ xR, e2 ^ pmul(xR, e1), e3 ^ pmul(xR, e2), pmul(xR, e3)]
E = set()
for (a1, a2, a3, a4, db) in res:
    term = ppow(bb, db, NV)
    for i, a in enumerate((a1, a2, a3, a4)):
        term = pmul(term, ppow(sub[i], a, NV))
    E ^= term

# numeric check
F = Field(13)
rng = random.Random(7)
ok = True
for _ in range(200):
    xs = [rng.randrange(1 << 13) for _ in range(4)]
    lhs = int(S4(F, *[np.int64(x) for x in xs]))
    a, bq, c, r = xs
    ev = [a ^ bq ^ c, F.smul(a, bq) ^ F.smul(a, c) ^ F.smul(bq, c), F.smul(F.smul(a, bq), c), r, B_CURVE]
    tot = 0
    for mono in E:
        val = 1
        for base, ex in zip(ev, mono):
            for _ in range(ex):
                val = F.smul(val, base)
        tot ^= val
    ok &= (tot == lhs)

wt = lambda z: bin(z).count('1')
# group by (e1,e2,e3) exponents
groups = {}
for (a1, a2, a3, ar, ab) in E:
    groups.setdefault((a1, a2, a3), []).append((ar, ab))
rows = []
for (a1, a2, a3), cs in sorted(groups.items()):
    rows.append(dict(e1=a1, e2=a2, e3=a3, bitdeg_e2e3=wt(a2) + wt(a3), bitdeg_all=wt(a1) + wt(a2) + wt(a3),
                     coeff_terms_xR_b=sorted(cs)))
out = dict(numeric_check_200_points=ok, n_monomials_e=len(groups),
           max_bitdeg_e2e3_with_e1_fixed=max(r['bitdeg_e2e3'] for r in rows),
           max_bitdeg_all=max(r['bitdeg_all'] for r in rows),
           nonaffine_e2e3_monomials=[(r['e1'], r['e2'], r['e3']) for r in rows if r['bitdeg_e2e3'] >= 2],
           monomials=rows)
json.dump(out, open(sys.argv[1], 'w'), indent=1)
print('numeric check', ok, 'monomials', len(groups))
print('max bit-degree in (e2,e3) with e1 fixed:', out['max_bitdeg_e2e3_with_e1_fixed'])
print('non-affine (e2,e3) monomials [e1,e2,e3 exps]:', out['nonaffine_e2e3_monomials'])
print('all monomials:', [(r['e1'], r['e2'], r['e3']) for r in rows])
