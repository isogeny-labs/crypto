"""D2: base abscissae in a multiplicative coset g*mu_d (d | p - 1).
(1) exact coverage of E[r] and unknown count, with torsion compatibility (points of all of E allowed);
(2) counted per-point decision (shared-denominator additions, as D1);
(3) counted norm/resultant decision in R_g = F_p[X]/(X^d - c), c = g^d:
    N(X) = Res_Y(q(X, Y), Y^d - c) = e^d - c W_d + c^2 a^d with q = S3(X, Y, xR) = aY^2 + bY + e,
    W_n = a^n (y1^n + y2^n) by a Lucas ladder, then gcd(N mod (X^d - c), X^d - c) by Euclid.
    A nontrivial gcd means some x1 in g*mu_d has a partner x2 in g*mu_d."""
import json, random, sys, pathlib, time
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import fp
from fp import p, A, r, mul, CNT, batch_inv
D = pathlib.Path(__file__).resolve().parents[1]
rng = random.Random(202611072)

def gen_mu(d):
    while True:
        h = pow(rng.randrange(2, p), (p - 1) // d, p)
        if all(pow(h, d // q, p) != 1 for q in {2, 3, 7, 19, 73} if d % q == 0): return h

def coset_base(d, g):
    h = gen_mu(d); x = g % p; B = []
    for _ in range(d):
        P = fp.u_lift(x)
        if P is not None: B.append(P)
        x = x * h % p
    return B

# torsion index via [r]P in a fixed table of E[8]
def e8_table():
    while True:
        P = fp.u_lift(rng.randrange(p))
        if P is None: continue
        T = fp.u_mul(fp.r, P)
        if fp.u_mul(4, T) is not None: break
    tab = {}; Q = None
    for i in range(8):
        tab[Q] = i; Q = fp.u_add(Q, T)
    return tab
TAB = e8_table()

def tors(P):
    return TAB[fp.u_mul(r, P)]

def coverage(B):
    t = [tors(P) for P in B]
    unk = set()
    for P in B:
        Q = fp.u_proj(P)
        if Q is not None: unk.add(Q[0])
    xs = set()
    for i in range(len(B)):
        for j in range(i, len(B)):
            for s in (1, -1):
                if (t[i] + s * t[j]) % 8 == 0:
                    Q = fp.u_add(B[i], B[j] if s == 1 else fp.u_neg(B[j]))
                    if Q is not None: xs.add(Q[0])
                if (-t[i] + s * t[j]) % 8 == 0:  # -P_i + s P_j
                    Q = fp.u_add(fp.u_neg(B[i]), B[j] if s == 1 else fp.u_neg(B[j]))
                    if Q is not None: xs.add(Q[0])
    return 2 * len(xs), len(unk)

def decide_points(R, B, Bx):
    xR, yR = R; invs = batch_inv([(xR - x) % p for x, _ in B]); hit = False
    for (x1, y1), iv in zip(B, invs):
        for sg in (1, -1):
            lam = mul((yR - sg * y1) % p, iv); x3 = (mul(lam, lam) - A - xR - x1) % p
            if x3 in Bx: hit = True
    return hit

# ---- counted polynomial arithmetic --------------------------------------------
def pmul(u, v):  # Karatsuba, schoolbook below 24
    n, m = len(u), len(v)
    if n == 0 or m == 0: return []
    if min(n, m) < 24:
        out = [0] * (n + m - 1)
        for i, a in enumerate(u):
            if a == 0: continue
            for j, b in enumerate(v):
                if b: out[i + j] = (out[i + j] + mul(a, b)) % p
        return out
    k = max(n, m) // 2
    u0, u1 = u[:k], u[k:]; v0, v1 = v[:k], v[k:]
    z0 = pmul(u0, v0); z2 = pmul(u1, v1)
    sv = [(v0[i] if i < len(v0) else 0) + (v1[i] if i < len(v1) else 0) for i in range(max(len(v0), len(v1)))]
    su = [(u0[i] if i < len(u0) else 0) + (u1[i] if i < len(u1) else 0) for i in range(max(len(u0), len(u1)))]
    z1 = pmul([a % p for a in su], [b % p for b in sv])
    out = [0] * (n + m - 1)
    for i, a in enumerate(z0): out[i] += a; z1[i] -= a
    for i, a in enumerate(z2): out[i + 2 * k] += a; z1[i] -= a
    for i, a in enumerate(z1):
        if i + k < len(out): out[i + k] += a
    return [a % p for a in out]

def rmul(u, v, d, c):  # product in F_p[X]/(X^d - c)
    w = pmul(u, v); out = w[:d] + [0] * max(0, d - len(w))
    for i in range(d, len(w)):
        if w[i]: out[i - d] = (out[i - d] + mul(c, w[i])) % p
    return out

def binom_pow(alpha, beta, n, d, c):  # (alpha X + beta)^n mod X^d - c, O(n) counted mults
    # coef_k = C(n,k) alpha^k beta^(n-k); ratio uses precomputed constants (n-k)/(k+1) (free)
    ib = pow(beta, p - 2, p)  # setup constant (beta fixed per trial: counted below as one inversion)
    CNT['M'] += 37
    ratio = mul(alpha, ib); coef = 1
    for _ in range(n): coef = mul(coef, beta)  # beta^n (counted, conservative: square-and-multiply would be cheaper)
    out = [0] * d
    for k in range(n + 1):
        idx = k % d; tw = pow(c, k // d, p)
        out[idx] = (out[idx] + (coef if k < d else mul(coef, tw))) % p
        if k < n:
            coef = mul(mul(coef, ratio), (n - k) * pow(k + 1, p - 2, p) % p)
    return out

def pgcd_deg(u, v):  # Euclid, counted; returns degree of gcd
    def trim(a):
        while a and a[-1] == 0: a.pop()
        return a
    u = trim(u[:]); v = trim(v[:])
    while v:
        iv = fp.inv(v[-1])
        while len(u) >= len(v):
            q = mul(u[-1], iv); sh = len(u) - len(v)
            for i in range(len(v) - 1):
                if v[i]: u[sh + i] = (u[sh + i] - mul(q, v[i])) % p
            u.pop(); u = trim(u)
            if not u: break
        u, v = v, u
    return len(u) - 1

def decide_poly(xR, d, c):
    X = lambda coeffs: (coeffs + [0] * d)[:d]
    # a = (X - xR)^2, b = -2((X + xR)(xR X + 1) + 2A xR X), e = (xR X - 1)^2
    a = X([xR * xR % p, -2 * xR % p, 1]); CNT['M'] += 1
    b = X([(-2 * xR) % p, (-2 * (xR * xR + 1 + 2 * A * xR)) % p, (-2 * xR) % p]); CNT['M'] += 2
    e = X([1, (-2 * xR) % p, xR * xR % p])
    ae = rmul(a, e, d, c)
    W1 = [(-t) % p for t in b]
    Wn, Wn1, Pn = [2] + [0] * (d - 1), W1, [1] + [0] * (d - 1)  # n = 0
    for bit in bin(d)[2:]:
        # from n to 2n or 2n+1
        W2n = [(s - 2 * t) % p for s, t in zip(rmul(Wn, Wn, d, c), Pn)]
        W2n1 = [(s - t) % p for s, t in zip(rmul(Wn, Wn1, d, c), rmul(Pn, W1, d, c))]
        P2n = rmul(Pn, Pn, d, c)
        if bit == '0':
            Wn, Wn1, Pn = W2n, W2n1, P2n
        else:
            P2n1 = rmul(P2n, ae, d, c)
            W2n2 = [(s - 2 * t) % p for s, t in zip(rmul(Wn1, Wn1, d, c), rmul(Pn, ae, d, c))]
            Wn, Wn1, Pn = W2n1, W2n2, P2n1
    # first iteration from n = 0 with bit '1' gives n = 1 (leading bit); ladder ends at n = d
    ad = binom_pow(1, (-xR) % p, 2 * d, d, c)
    ed = binom_pow(xR, p - 1, 2 * d, d, c)
    c2 = c * c % p; CNT['M'] += 1
    Nx = [(ed[i] - mul(c, Wn[i]) + mul(c2, ad[i])) % p for i in range(d)]
    mod = [(-c) % p] + [0] * (d - 1) + [1]
    return pgcd_deg(mod, Nx)

def main():
    out = dict(rows=[], poly_check=[])
    # (1)-(2) coverage and per-point cost
    for d in (1022, 1197, 1596, 2394):
        for g in (1, 5):
            B = coset_base(d, g); Bx = {P[0] for P in B}
            t0 = time.time(); C, Funk = coverage(B)
            hits = 0; cost = 0; T = 200
            for _ in range(T):
                R = fp.u_rand_Er(rng); fp.reset(); hits += decide_points(R, B, Bx); cost += CNT['M']
            row = dict(d=d, g=g, base_points=len(B), unknowns=Funk, C=C, C_over_r=C / r, hit_rate=hits / T,
                       c_points=cost / T, c_points_per_unknown=cost / T / Funk)
            out['rows'].append(row); print(row, round(time.time() - t0, 1), "s")

    # (3) polynomial decision: correctness at small d against the per-point oracle, then cost at d needed
    for d in (126, 252):
        g = 5; B = coset_base(d, g); Bx = {P[0] for P in B}; c = pow(g, d, p); agree = 0; n = 0; yes = 0
        for _ in range(30):
            R = fp.u_rand_Er(rng)
            # plant half of the trials: R = P1 + P2 with compatible torsion
            if _ % 2 == 0:
                for _try in range(500):
                    i, j = rng.randrange(len(B)), rng.randrange(len(B))
                    Q = fp.u_add(B[i], B[j])
                    if Q is not None and fp.u_mul(r, Q) is None: R = Q; break
            fp.reset(); h1 = decide_points(R, B, Bx)
            fp.reset(); gd = decide_poly(R[0], d, c); cpoly = CNT['M']
            h2 = gd > 0
            # x1 off the curve gives a nonsquare discriminant and no F_p root, so the two oracles must agree.
            agree += (h1 == h2); n += 1; yes += h1
        out['poly_check'].append(dict(d=d, trials=n, yes_points=yes, consistent=agree, c_poly_last=cpoly))
        print(out['poly_check'][-1])
    for d in (1022, 2394):
        c = pow(5, d, p); fp.reset(); t0 = time.time(); decide_poly(fp.u_rand_Er(rng)[0], d, c)
        out.setdefault('poly_cost', []).append(dict(d=d, c_poly=CNT['M'])); print(d, CNT['M'], round(time.time() - t0, 1))
    json.dump(out, open(D / "data/d2_coset.json", "w"), indent=1)


if __name__ == "__main__":
    main()
