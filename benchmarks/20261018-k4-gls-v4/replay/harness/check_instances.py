"""Independent check of the regenerated E19/E24/E25 instances (validator scratch).

Own F_{p^4} = F_p[t]/(t^4 - c) and affine short-Weierstrass arithmetic, written here and
sharing no code with the repository.  For each instance: G and Q on the curve, [n]G = O,
[d]G = Q (the planted scalar the pipeline's `correct` flag compares against), n prime,
#E = h*n with [#E]P = O on random points, the field of definition of j (j^p = j, j^(p^2) = j),
and on GLS curves psi^2 = -1 and psi(G) = [lambda]G.  Then n, #E, h are compared with the
committed run JSON.
"""
import json, os, random, sys, glob

REPO = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', '..'))

def isprime(n):
    if n < 2: return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True

class F4:
    def __init__(s, p, c): s.p, s.c = p, c
    def add(s, a, b): return tuple((x + y) % s.p for x, y in zip(a, b))
    def sub(s, a, b): return tuple((x - y) % s.p for x, y in zip(a, b))
    def neg(s, a): return tuple((-x) % s.p for x in a)
    def mul(s, a, b):
        d = [0] * 7
        for i in range(4):
            for j in range(4): d[i + j] += a[i] * b[j]
        return tuple((d[i] + s.c * (d[i + 4] if i + 4 < 7 else 0)) % s.p for i in range(4))
    def pow(s, a, e):
        r, b = (1, 0, 0, 0), a
        while e:
            if e & 1: r = s.mul(r, b)
            b = s.mul(b, b); e >>= 1
        return r
    def inv(s, a):
        assert any(a)
        return s.pow(a, s.p ** 4 - 2)
    def sqrt(s, a, rng):
        q = s.p ** 4 - 1
        if not any(a): return a
        if s.pow(a, q // 2) != (1, 0, 0, 0): return None
        m, odd = 0, q
        while odd % 2 == 0: odd //= 2; m += 1
        while True:
            z = tuple(rng.randrange(s.p) for _ in range(4))
            if any(z) and s.pow(z, q // 2) != (1, 0, 0, 0): break
        cc, t, r = s.pow(z, odd), s.pow(a, odd), s.pow(a, (odd + 1) // 2)
        one = (1, 0, 0, 0)
        while t != one:
            i, tt = 0, t
            while tt != one: tt = s.mul(tt, tt); i += 1
            b = cc
            for _ in range(m - i - 1): b = s.mul(b, b)
            m, cc = i, s.mul(b, b)
            t, r = s.mul(t, cc), s.mul(r, b)
        return r

INF = None
class Curve:
    def __init__(s, f, a, b): s.f, s.a, s.b = f, a, b
    def on(s, P):
        if P is INF: return True
        x, y = P; f = s.f
        return f.mul(y, y) == f.add(f.add(f.mul(f.mul(x, x), x), f.mul(s.a, x)), s.b)
    def add(s, P, Q):
        f = s.f
        if P is INF: return Q
        if Q is INF: return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if y1 != y2 or not any(y1): return INF
            num = f.add(f.mul((3, 0, 0, 0), f.mul(x1, x1)), s.a)
            lam = f.mul(num, f.inv(f.add(y1, y1)))
        else:
            lam = f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
        x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2)
        return (x3, f.sub(f.mul(lam, f.sub(x1, x3)), y1))
    def mul(s, P, k):
        R = INF
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R
    def neg(s, P): return INF if P is INF else (P[0], s.f.neg(P[1]))

def pt(d): return INF if d['inf'] else (tuple(d['x']), tuple(d['y']))

def main(path, out):
    rng = random.Random(20261021)
    committed = {}
    for exp, d in (('e19', '27_quartic_pipeline_d4'), ('e24', '29_quartic_pipeline_v4'), ('e25', '30_gls_v4')):
        for f in glob.glob(os.path.join(REPO, 'experiments', d, 'p*s*.json')):
            for r in json.load(open(f)):
                committed.setdefault((exp, r['p'], r['seed']), []).append(r)
    res = []
    for line in open(path):
        I = json.loads(line)
        p = I['p']; f = F4(p, I['fp4_c'])
        # c is the least non-square of F_p (t^4 - c irreducible needs p = 1 mod 4 and c a non-square)
        assert p % 4 == 1 and pow(I['fp4_c'], (p - 1) // 2, p) == p - 1
        E = Curve(f, tuple(I['a']), tuple(I['b']))
        G, Q, n, d = pt(I['g']), pt(I['q']), I['n'], I['d']
        chk = {}
        chk['G_on_curve'] = E.on(G) and G is not INF
        chk['Q_on_curve'] = E.on(Q)
        chk['n_prime'] = isprime(n)
        chk['nG_is_O'] = E.mul(G, n) is INF
        chk['dG_eq_Q'] = E.mul(G, d) == Q
        chk['order_eq_h_n'] = I['group_order'] == I['cofactor'] * n
        lo, hi = p ** 4 + 1 - 2 * p * p, p ** 4 + 1 + 2 * p * p
        chk['order_in_hasse'] = lo <= I['group_order'] <= hi
        ok = True
        for _ in range(3):
            while True:
                x = tuple(rng.randrange(p) for _ in range(4))
                rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(E.a, x)), E.b)
                y = f.sqrt(rhs, rng)
                if y is not None: break
            P = (x, y); assert E.on(P)
            ok &= E.mul(P, I['group_order']) is INF
        chk['order_kills_random_points'] = ok
        a3x4 = f.mul((4, 0, 0, 0), f.mul(f.mul(E.a, E.a), E.a))
        disc = f.add(a3x4, f.mul((27, 0, 0, 0), f.mul(E.b, E.b)))
        j = f.mul(f.mul((1728 % p, 0, 0, 0), a3x4), f.inv(disc))
        j_in_fp = f.pow(j, p) == j
        j_in_fp2 = f.pow(j, p * p) == j
        rec = dict(experiment=I['experiment'], p=p, seed=I['seed'], n=n, group_order=I['group_order'],
                   cofactor=I['cofactor'], planted_d=d, j=list(j), j_in_fp=j_in_fp, j_in_fp2=j_in_fp2,
                   j_is_0_or_1728=j in ((0, 0, 0, 0), (1728 % p, 0, 0, 0)))
        if I['gls']:
            cx, cy = tuple(I['gls']['cx']), tuple(I['gls']['cy'])
            sig = lambda v: f.pow(v, p * p)  # the p^2-power map, computed by exponentiation
            psi = lambda P: INF if P is INF else (f.mul(cx, sig(P[0])), f.mul(cy, sig(P[1])))
            lam = I['lambda']
            chk['psi_G_on_curve'] = E.on(psi(G))
            chk['psi2_G_eq_minus_G'] = psi(psi(G)) == E.neg(G)
            chk['lambda2_eq_minus1_mod_n'] = lam * lam % n == n - 1
            chk['psi_G_eq_lambda_G'] = psi(G) == E.mul(G, lam)
        cm = committed.get((I['experiment'], p, I['seed']), [])
        chk['matches_committed_n_order_h'] = bool(cm) and all(
            r['n'] == n and r['group_order'] == I['group_order'] and r['cofactor'] == I['cofactor'] for r in cm)
        rec['checks'] = chk
        rec['all_checks_pass'] = all(chk.values())
        res.append(rec)
        print(I['experiment'], p, I['seed'], 'd =', d, 'j in Fp', j_in_fp, 'j in Fp2', j_in_fp2, 'all', rec['all_checks_pass'],
              [k for k, v in chk.items() if not v])
    json.dump(res, open(out, 'w'), indent=1)

if __name__ == '__main__':
    # check_instances.py INSTANCES.jsonl OUT.json   (INSTANCES.jsonl: output of instances.rs)
    main(sys.argv[1], sys.argv[2])
