"""Ratmap route, part A: equation count and m=2 relation yield for
image bases B_phi = {P in E(F_p): x(P) in phi(T)}.
(1) relaxed count: #{(t1,t2) in F_p^2 : S3(phi(t1),phi(t2),xR)=0}  ~ c*p  => solution set has dim m-1=1
    over the base field, i.e. the parametrisation x=phi(t) adds 0 net equations.
(2) m=2 yield: #distinct x(R) reachable as x(+-P1+-P2), P_i in B, vs a random base of equal size.
Toy only. Output: JSON lines on stdout."""
import random, json, sys, math
from sympy import nextprime
from sympy.ntheory import sqrt_mod

def run(bits, L, seed):
    rnd = random.Random(seed)
    p = nextprime(rnd.randrange(2**(bits-1), 2**bits))
    # E': y^2 = (x-x0)(x^2+x0 x+c) has rational 2-torsion (x0,0); expand to x^3+a'x+b'
    x0 = rnd.randrange(p); c = rnd.randrange(p)
    a1 = (c - x0*x0) % p; b1 = (-x0*c) % p          # (x-x0)(x^2+x0x+c)=x^3+(c-x0^2)x - x0 c
    tv = (3*x0*x0 + a1) % p
    a = (a1 - 5*tv) % p; b = (b1 - 7*x0*tv) % p      # Velu 2-isogeny image E: y^2=x^3+ax+b
    inv = lambda z: pow(z, p-2, p)
    f = lambda x: (x*x*x + a*x + b) % p
    def oncurve(x): v = f(x); return v == 0 or pow(v, (p-1)//2, p) == 1
    def lattes(t):  # x([2]Q) for x(Q)=t on E, as a rational function of t (degree 4)
        d = 4*f(t) % p
        return None if d == 0 else ((t*t - a)**2 - 8*b*t) * inv(d) % p
    def velu(t):    # x-map of the 2-isogeny E'->E (degree 2)
        return None if (t - x0) % p == 0 else (t + tv*inv(t - x0)) % p
    d_pow = None
    for n in range(int(round(p**0.5)), 1, -1):
        if (p-1) % n == 0: d_pow = (p-1)//n; break
    maps = {
        'id': lambda t: t % p,
        'sq': lambda t: t*t % p,
        'cube': lambda t: pow(t, 3, p),
        'mobius': lambda t: None if (7*t+11) % p == 0 else (3*t+5)*inv(7*t+11) % p,
        'lattes2': lattes,
        'velu2': velu,
    }
    out = []
    # (1) relaxed count, small p only (O(p) loop)
    def relaxed(phi, xR):
        pre = {}
        for t in range(p):
            v = phi(t)
            if v is not None: pre[v] = pre.get(v, 0) + 1
        cnt = 0
        for x1, c1 in pre.items():
            # S3(x1,x2,xR) as quadratic in x2: A x2^2 + B x2 + C
            A = (x1 - xR)**2 % p
            B = (-2*((x1 + xR)*(x1*xR + a) + 2*b)) % p
            C = ((x1*xR - a)**2 - 4*b*(x1 + xR)) % p
            if A == 0: continue
            disc = (B*B - 4*A*C) % p
            if disc == 0: roots = [(-B)*inv(2*A) % p]
            else:
                s = sqrt_mod(disc, p)
                if s is None: continue
                roots = [(-B + s)*inv(2*A) % p, (-B - s)*inv(2*A) % p]
            for r in roots: cnt += c1 * pre.get(r, 0)
        return cnt
    xR = next(x for x in (rnd.randrange(p) for _ in range(1000)) if oncurve(x))
    # point ops for yield
    def ysqrt(x):
        v = f(x); return 0 if v == 0 else sqrt_mod(v, p)
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            lam = (3*P[0]*P[0] + a) * inv(2*P[1]) % p
        else: lam = (Q[1]-P[1]) * inv(Q[0]-P[0]) % p
        x3 = (lam*lam - P[0] - Q[0]) % p
        return (x3, (lam*(P[0]-x3) - P[1]) % p)
    def reach(xs):
        pts = [(x, ysqrt(x)) for x in xs]
        R = set()
        for i in range(len(pts)):
            for j in range(i, len(pts)):
                P, Q = pts[i], pts[j]
                for S in (add(P, Q), add(P, (Q[0], (-Q[1]) % p))):
                    if S is not None: R.add(S[0])
        return len(R)
    ncurve = sum(1 for x in range(p) if oncurve(x))
    Tset = range(1, L+1)
    for name, phi in list(maps.items()) + [('pow_mu', None)]:
        if name == 'pow_mu':
            n = (p-1)//d_pow
            g = next(g for g in range(2, p) if all(pow(g, (p-1)//q, p) != 1 for q in __import__('sympy').primefactors(p-1)))
            img = {pow(g, d_pow*i, p) for i in range(n)}
            rel = None
        else:
            img = {v for v in (phi(t) for t in Tset) if v is not None}
            rel = relaxed(phi, xR) if bits <= 14 else None
        xs = sorted(x for x in img if oncurve(x))
        allc = [x for x in range(p) if oncurve(x)]
        ctrl = [reach(rnd.sample(allc, len(xs))) for _ in range(3)]
        y = reach(xs)
        out.append(dict(bits=bits, p=p, map=name, T=(L if name != 'pow_mu' else 'all F_p*'),
                        base_x=len(xs), base_exp=round(math.log(max(len(xs),2))/math.log(p), 3),
                        relaxed_solutions=rel, relaxed_over_p=(round(rel/p, 3) if rel is not None else None),
                        reach=y, reach_ctrl_mean=round(sum(ctrl)/3, 1),
                        yield_ratio=round(y/(sum(ctrl)/3), 3), curve_x=ncurve))
    return out

if __name__ == '__main__':
    for bits, L, seed in [(14, 180, 1), (14, 180, 2), (16, 360, 3), (16, 360, 4)]:
        for r in run(bits, L, seed): print(json.dumps(r), flush=True)
