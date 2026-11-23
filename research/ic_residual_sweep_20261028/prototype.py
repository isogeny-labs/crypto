"""Single-large-prime residual sweep on a toy prime curve: how many additions to recover x?

Targets R = aG + bQ.  For every base point P_v (unknown log) and sign, the residual
Z = R - s*P_v is stored under x(Z).  Two stored residuals with equal x (Z = +-Z') give
  (a - t a') + (b - t b') x = u_v*s - t*s' u_v'   (t = +-1)
over unknowns u_v = log P_v and x.  A weighted union-find over the base carries the
potentials as c + d*x (mod r); a cycle gives c + d*x = 0.
"""
import math, random, sys

def curve(p):
    while True:
        a, b = random.randrange(p), random.randrange(p)
        if (4 * a**3 + 27 * b * b) % p == 0:
            continue
        # count points naively with Legendre symbol
        cnt = 1
        for x in range(p):
            f = (x**3 + a * x + b) % p
            if f == 0: cnt += 1
            elif pow(f, (p - 1) // 2, p) == 1: cnt += 2
        if all(cnt % q for q in range(2, int(cnt**0.5) + 1)):
            return a, b, cnt

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def neg(P, p): return None if P is None else (P[0], (-P[1]) % p)

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def run(p, B, seed):
    random.seed(seed)
    ca, cb, r = curve(p)
    # generator
    while True:
        x = random.randrange(p); f = (x**3 + ca * x + cb) % p
        if pow(f, (p - 1) // 2, p) == 1:
            y = pow(f, (p + 1) // 4, p) if p % 4 == 3 else None
            if y is not None: G = (x, y); break
    secret = random.randrange(1, r)
    Q = mul(secret, G, ca, p)
    # base: B points with unknown logs (distinct x)
    base = []
    seen = set()
    while len(base) < B:
        k = random.randrange(1, r)
        P = mul(k, G, ca, p)
        if P is None or P[0] in seen: continue
        seen.add(P[0]); base.append(P)
    adds = 0
    store = {}               # x(Z) -> (a, b, v, s, ysign)
    parent = list(range(B)); pot = [(0, 0)] * B   # u_v = u_root + pot (c + d x)
    def find(v):
        if parent[v] == v: return v, (0, 0)
        root, (c, d) = find(parent[v])
        c0, d0 = pot[v]
        parent[v] = root; pot[v] = ((c0 + c) % r, (d0 + d) % r)
        return root, pot[v]
    found = None
    rel = 0
    while found is None:
        a_, b_ = random.randrange(r), random.randrange(r)
        R = add(mul(a_, G, ca, p), mul(b_, Q, ca, p), ca, p)   # not counted: fresh randomisation
        for v, P in enumerate(base):
            for s in (1, -1):
                Z = add(R, P if s == -1 else neg(P, p), ca, p)  # Z = R - s P
                adds += 1
                if Z is None: continue
                key = Z[0]
                if key in store:
                    a2, b2, v2, s2, y2 = store[key]
                    t = 1 if y2 == Z[1] else -1
                    # R - sP = t (R' - s'P')  ->  (a - t a') + (b - t b') x = s u_v - t s' u_v'
                    A = (a_ - t * a2) % r; Bc = (b_ - t * b2) % r
                    # s u_v - t s' u_v' = A + Bc x ; handle via potentials on signed nodes
                    # use u_v' sign by writing both over roots: u_v = (A + Bc x + t s' u_v') / s
                    # Only s = s' = +-1 and t = +-1 -> coefficient on u_v' is t s' s (since 1/s = s)
                    coef = (t * s2 * s) % r
                    c_rel = (A * s) % r; d_rel = (Bc * s) % r        # u_v = c_rel + d_rel x + coef * u_v2
                    rv, (cv, dv) = find(v); rw, (cw, dw) = find(v2)
                    # u_v = u_rv + (cv + dv x); u_v2 = u_rw + (cw + dw x)
                    if rv == rw:
                        if coef == 1:
                            # u_rv + cv + dv x = c_rel + d_rel x + u_rv + cw + dw x
                            cc = (cv - c_rel - cw) % r; dd = (dv - d_rel - dw) % r
                        else:
                            # u_rv + cv + dv x = c_rel + d_rel x - u_rv - cw - dw x  -> 2 u_rv = ...; skip (uses a free unknown)
                            continue
                        rel += 1
                        if dd % r:
                            xs = (-cc * pow(dd, -1, r)) % r
                            if mul(xs, G, ca, p) == Q: found = xs
                    else:
                        if coef == 1:
                            # merge rw under rv:  u_rv + cv+dv x = c_rel+d_rel x + u_rw + cw+dw x
                            parent[rw] = rv
                            pot[rw] = ((cv - c_rel - cw) % r, (dv - d_rel - dw) % r)   # u_rw = u_rv + pot
                            rel += 1
                        # coef == -1 relations dropped (conservative)
                else:
                    store[key] = (a_, b_, v, s, Z[1])
                if found is not None: break
            if found is not None: break
    return r, adds, rel, found == secret

if __name__ == "__main__":
    p = int(sys.argv[1]); Bs = [int(x) for x in sys.argv[2].split(",")]; reps = int(sys.argv[3])
    for B in Bs:
        res = [run(p, B, 1000 * B + i) for i in range(reps)]
        r = res[0][0]
        S = [a / math.sqrt(r) for (_, a, _, ok) in res]
        print(f"p={p} B={B} reps={reps} ok={sum(x[3] for x in res)} mean adds={sum(x[1] for x in res)/reps:.0f} "
              f"mean S={sum(S)/reps:.2f} mean rel={sum(x[2] for x in res)/reps:.1f}")
