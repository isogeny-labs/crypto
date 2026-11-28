"""Route A9 (BKK presentation) msolve comparison: system builders.  [pcc:unverified]"""
import random, itertools, math, os, pickle
import flint
from sympy import isprime, nextprime
from sympy.ntheory import sqrt_mod

# ------------------------------------------------------------------ curve
def rand_prime(bits, rng):
    while True:
        p = nextprime(rng.randrange(2 ** (bits - 1), 2 ** bits))
        if p < 2 ** bits:
            return p

def rand_curve(p, rng):
    while True:
        a = rng.randrange(p); b = rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b ** 2) % p:
            return a, b

def is_x(p, a, b, x):
    r = (x ** 3 + a * x + b) % p
    return r == 0 or pow(r, (p - 1) // 2, p) == 1

def lift_x(p, a, b, x, rng):
    r = (x ** 3 + a * x + b) % p
    y = sqrt_mod(r, p)
    if y is None:
        return None
    return (x, y if rng.random() < 0.5 else (-y) % p)

def rand_point(p, a, b, rng):
    while True:
        P = lift_x(p, a, b, rng.randrange(p), rng)
        if P is not None and P[1] != 0:
            return P

def ec_add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

# ------------------------------------------------------------------ Semaev over Z[a,b,...]
_ZV = ('a', 'b', 'x1', 'x2', 'x3', 'x4', 'u')
_zctx = flint.fmpz_mpoly_ctx.get(_ZV, 'lex')
_A, _B, _X1, _X2, _X3, _X4, _U = _zctx.gens()

def _S3z(p_, q_, r_):
    return (p_ - q_) ** 2 * r_ ** 2 - 2 * ((p_ + q_) * (p_ * q_ + _A) + 2 * _B) * r_ \
        + (p_ * q_ - _A) ** 2 - 4 * _B * (p_ + q_)

S3_GEN = {tuple(int(t) for t in e[:5]): int(c) for e, c in _S3z(_X1, _X2, _X3).to_dict().items()}
_S4 = _S3z(_X1, _X2, _U).resultant(_S3z(_X3, _X4, _U), 'u')
S4_GEN = {}
for e, c in _S4.to_dict().items():
    assert int(e[6]) == 0
    S4_GEN[tuple(int(t) for t in e[:6])] = int(c)

def special(gen, p, a, b, names):
    """Specialise a generic Semaev dict {(ea,eb,e1..ek):c} at (a,b) mod p.
    names: list of k entries, each a variable name (str) or an int constant.
    Returns dict {monomial-tuple over the distinct variable names: coeff mod p}, var list."""
    vars_ = []
    for n in names:
        if isinstance(n, str) and n not in vars_:
            vars_.append(n)
    out = {}
    for e, c in gen.items():
        coef = c * pow(a, e[0], p) * pow(b, e[1], p) % p
        mono = [0] * len(vars_)
        for k, n in enumerate(names):
            ek = e[2 + k]
            if isinstance(n, str):
                mono[vars_.index(n)] += ek
            else:
                coef = coef * pow(n, ek, p) % p
        if coef:
            t = tuple(mono)
            out[t] = (out.get(t, 0) + coef) % p
    return {k: v for k, v in out.items() if v}, vars_

def S5_special(p, a, b, xR):
    """S_5(x1,x2,x3,x4,xR) = Res_u(S_4(x1,x2,x3,u), S_3(x4,xR,u)) over F_p."""
    ctx = flint.nmod_mpoly_ctx.get(('x1', 'x2', 'x3', 'x4', 'u'), modulus=p, ordering='lex')
    g = ctx.gens()
    def todict_poly(d, vs):
        pol = ctx.from_dict({tuple(dict(zip(vs, m)).get(n, 0) for n in ('x1', 'x2', 'x3', 'x4', 'u')): c
                             for m, c in d.items()})
        return pol
    s4, v4 = special(S4_GEN, p, a, b, ['x1', 'x2', 'x3', 'u'])
    s3, v3 = special(S3_GEN, p, a, b, ['x4', xR, 'u'])
    P = todict_poly(s4, v4); Q = todict_poly(s3, v3)
    R = P.resultant(Q, 'u')
    out = {}
    for e, c in R.to_dict().items():
        assert int(e[4]) == 0
        out[tuple(int(t) for t in e[:4])] = int(c) % p
    return {k: v for k, v in out.items() if v}, ['x1', 'x2', 'x3', 'x4']

def eval_poly(d, vs, assign, p):
    s = 0
    for m, c in d.items():
        t = c
        for v, e in zip(vs, m):
            t = t * pow(assign[v], e, p) % p
        s += t
    return s % p

def poly_str(d, vs):
    terms = []
    for m, c in sorted(d.items(), reverse=True):
        mon = '*'.join(f'{v}^{e}' if e > 1 else v for v, e in zip(vs, m) if e)
        terms.append(f'{c}*{mon}' if mon else f'{c}')
    return '+'.join(terms) if terms else '0'

def base_poly(roots, p, var):
    """dense f(var) = prod (var - r) as dict over [var]."""
    c = [1]
    for r in roots:
        nc = [0] * (len(c) + 1)
        for i, ci in enumerate(c):
            nc[i + 1] = (nc[i + 1] + ci) % p
            nc[i] = (nc[i] - r * ci) % p
        c = nc
    return {(i,): ci for i, ci in enumerate(c) if ci}, [var]

# ------------------------------------------------------------------ presentations
ARMS = {3: ['D', 'C3', 'C3u'], 4: ['D', 'C4', 'B4', 'H4a', 'H4b']}

def semaev_part(arm, m, p, a, b, xR):
    """List of (dict, vars) Semaev-part equations, plus variable order (DRL, first = largest)."""
    xs = [f'x{i}' for i in range(1, m + 1)]
    if arm == 'D':
        if m == 3:
            eqs = [special(S4_GEN, p, a, b, ['x1', 'x2', 'x3', xR])]
        else:
            eqs = [S5_special(p, a, b, xR)]
        return eqs, xs
    if arm == 'C3':
        return [special(S3_GEN, p, a, b, ['x1', 'x2', 'u']), special(S3_GEN, p, a, b, ['u', 'x3', xR])], xs + ['u']
    if arm == 'C3u':
        return [special(S3_GEN, p, a, b, ['x1', 'x2', 'u']), special(S3_GEN, p, a, b, ['u', 'x3', xR])], ['u'] + xs
    if arm == 'C4':
        return [special(S3_GEN, p, a, b, ['x1', 'x2', 'u1']), special(S3_GEN, p, a, b, ['u1', 'x3', 'u2']),
                special(S3_GEN, p, a, b, ['u2', 'x4', xR])], xs + ['u1', 'u2']
    if arm == 'B4':
        return [special(S3_GEN, p, a, b, ['x1', 'x2', 'u']), special(S3_GEN, p, a, b, ['x3', 'x4', 'v']),
                special(S3_GEN, p, a, b, ['u', 'v', xR])], xs + ['u', 'v']
    if arm == 'H4a':
        return [special(S4_GEN, p, a, b, ['x1', 'x2', 'x3', 'u']), special(S3_GEN, p, a, b, ['u', 'x4', xR])], xs + ['u']
    if arm == 'H4b':
        return [special(S3_GEN, p, a, b, ['x1', 'x2', 'u']), special(S4_GEN, p, a, b, ['u', 'x3', 'x4', xR])], xs + ['u']
    raise ValueError(arm)

def msolve_text(eqs, base_roots, m, order, p):
    lines = [','.join(order), str(p)]
    polys = [poly_str(d, vs) for d, vs in eqs]
    for i in range(1, m + 1):
        d, vs = base_poly(base_roots, p, f'x{i}')
        polys.append(poly_str(d, vs))
    lines.append(',\n'.join(polys))
    return '\n'.join(lines) + '\n'

# ------------------------------------------------------------------ instances
def L_of(b, m):
    return int(round(2 ** (b / m)))

def make_instance(m, bits, idx, planted=False):
    rng = random.Random(f'bkk-A9-{m}-{bits}-{idx}-{int(planted)}')
    p = rand_prime(bits, rng)
    a, b = rand_curve(p, rng)
    L = L_of(bits, m)
    roots = set()
    if planted:
        Ps = [rand_point(p, a, b, rng) for _ in range(m)]
        R = None
        for P in Ps:
            R = ec_add(R, P, p, a)
        if R is None or len({P[0] for P in Ps}) < m:
            return make_instance(m, bits, idx + 1000, planted)
        roots |= {P[0] for P in Ps}
        xR = R[0]
        planted_pts = Ps
    else:
        xR = rand_point(p, a, b, rng)[0]
        planted_pts = None
    while len(roots) < L:
        x = rng.randrange(p)
        if is_x(p, a, b, x):
            roots.add(x)
    roots = sorted(roots)
    rng.shuffle(roots)
    return dict(m=m, bits=bits, idx=idx, planted=planted, p=p, a=a, b=b, L=L, xR=xR, roots=roots, planted_pts=planted_pts)
