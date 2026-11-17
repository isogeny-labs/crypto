#!/usr/bin/env python3
"""LEAD-EXO-invertible-hint-subset: toy RANSAC measure of low-degree agreement A(d)
between degree-d polynomials over F_p and the EC log graph {(x(kG),k)}.  stdlib only."""
import sys, json, random, math

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def make_curve(pmin, seed):
    rnd = random.Random(seed)
    p = pmin
    while True:
        p += 1
        if not is_prime(p) or p % 3 == 0: continue
        sq = {}
        for y in range(p): sq.setdefault(y * y % p, []).append(y)
        for _ in range(200):
            a, b = rnd.randrange(p), rnd.randrange(1, p)
            if (4 * a**3 + 27 * b * b) % p == 0: continue
            pts = [(x, y) for x in range(p) for y in sq.get((x**3 + a * x + b) % p, [])]
            N = len(pts) + 1
            if is_prime(N) and N != p and N > p // 2:
                return p, a, b, N, pts[rnd.randrange(len(pts))]
        # next prime

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2 and (y1 + y2) % p == 0: return None
    if P == Q: l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else: l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def ec_graph(pmin, seed):
    p, a, b, N, G = make_curve(pmin, seed)
    n = (N - 1) // 2
    X, Y = [], []
    P = None
    for k in range(1, n + 1):
        P = add(P, G, a, p)
        X.append(P[0]); Y.append(k)
    return p, N, X, Y

def interp(xs, ys, p):
    """Lagrange -> coefficient list (low to high) mod p."""
    d = len(xs); coef = [0] * d
    for i in range(d):
        num = [1]; den = 1
        for j in range(d):
            if j == i: continue
            num = [( (num[k - 1] if k else 0) - xs[j] * (num[k] if k < len(num) else 0)) % p for k in range(len(num) + 1)]
            den = den * (xs[i] - xs[j]) % p
        f = ys[i] * pow(den, -1, p) % p
        for k in range(d): coef[k] = (coef[k] + f * num[k]) % p
    return coef

def ransac(X, Y, p, d, T, rnd):
    n = len(X); ex = []; M = 0
    for _ in range(T):
        idx = rnd.sample(range(n), d + 1)
        c = interp([X[i] for i in idx], [Y[i] for i in idx], p)
        cnt = 0
        for x, y in zip(X, Y):
            v = 0
            for co in reversed(c): v = (v * x + co) % p
            if v == y: cnt += 1
        ex.append(cnt - (d + 1)); M = max(M, cnt)
    m = sum(ex) / T
    var = sum((e - m) ** 2 for e in ex) / max(T - 1, 1)
    return m, var, M

def run(pmin, T, degs, nec, seed0):
    out = {"arms": {}, "per_d": {}}
    ecs = [ec_graph(pmin + 300 * i, seed0 + i) for i in range(nec)]
    arms = {}
    for i, (p, N, X, Y) in enumerate(ecs):
        arms["EC%d" % i] = (p, X, Y)
    for i, (p, N, X, Y) in enumerate(ecs):   # null: same x set, random bijection of labels
        r = random.Random(seed0 + 100 + i); Y2 = Y[:]; r.shuffle(Y2)
        arms["NULL%d" % i] = (p, X, Y2)
    p, N, X, Y = ecs[0]                       # A/A: second independent shuffle of curve 0
    r = random.Random(seed0 + 200); Y2 = Y[:]; r.shuffle(Y2); arms["AA_null"] = (p, X, Y2)
    params = {"p": [e[0] for e in ecs], "N": [e[1] for e in ecs], "n": [len(e[2]) for e in ecs]}
    verdict_parts = {"falsify": False, "pos_fail": False, "aa_fail": False}
    for d in degs:
        res = {}
        for name, (pp, XX, YY) in arms.items():
            res[name] = ransac(XX, YY, pp, d, T, random.Random(seed0 * 7 + d * 13 + hash_s(name)))
        # positive control: plant degree-d poly on 25% of curve-0 points
        pr = random.Random(seed0 + 300 + d)
        co = [pr.randrange(p) for _ in range(d + 1)]
        Yp = Y[:]
        for i in pr.sample(range(len(X)), len(X) // 4):
            v = 0
            for c in reversed(co): v = (v * X[i] + c) % p
            Yp[i] = v
        pm, pv, pM = ransac(X, Yp, p, d, T, random.Random(seed0 + 400 + d))
        pos_ok = pM >= 0.15 * len(X)
        def pool(names):
            ms = [res[k] for k in names]
            return sum(m for m, v, M in ms) / len(ms), sum(v for m, v, M in ms) / len(ms), len(ms) * T
        ec_m, ec_v, ec_n = pool([k for k in res if k.startswith("EC")])
        nu_m, nu_v, nu_n = pool([k for k in res if k.startswith("NULL")])
        z = (ec_m - nu_m) / math.sqrt(ec_v / ec_n + nu_v / nu_n + 1e-18)
        a_m, a_v, _ = res["AA_null"]; b_m, b_v, _ = res["NULL0"]
        zaa = (a_m - b_m) / math.sqrt((a_v + b_v) / T + 1e-18)
        out["per_d"][str(d)] = {"z_EC_vs_null": round(z, 3), "z_AA": round(zaa, 3),
            "mean_excess_EC": ec_m, "mean_excess_null": nu_m,
            "maxagree": {k: v[2] for k, v in res.items()},
            "positive_control_maxagree": pM, "positive_control_ok": pos_ok}
        if z > 4: verdict_parts["falsify"] = True
        if not pos_ok: verdict_parts["pos_fail"] = True
        if abs(zaa) > 4: verdict_parts["aa_fail"] = True
    if verdict_parts["pos_fail"] or verdict_parts["aa_fail"]: v = "inconclusive"
    elif verdict_parts["falsify"]: v = "falsified"
    else: v = "consistent_with_prediction"
    out["verdict"] = v; out["flags"] = verdict_parts; out["params"] = params
    return out

def hash_s(s):
    h = 0
    for ch in s: h = (h * 131 + ord(ch)) % 1000003
    return h

if __name__ == "__main__":
    if "--smoke" in sys.argv:
        cfg = dict(pmin=60, T=60, degs=[1, 2], nec=1, seed0=11)
    else:
        cfg = dict(pmin=800, T=2500, degs=[1, 2, 3], nec=3, seed0=20261110)
    o = run(**cfg)
    o.update({"lead": "LEAD-EXO-invertible-hint-subset", "config": cfg, "smoke": "--smoke" in sys.argv,
              "label": "pcc:unverified"})
    print(json.dumps(o))
