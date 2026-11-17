import json, random, sys, math
SMOKE = '--smoke' in sys.argv
def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True
def add(P, Q, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        m = 3 * x1 * x1 // 1
        m = (m + A_[0]) * pow(2 * y1, -1, p) % p
    else:
        m = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (m * m - x1 - x2) % p
    return (x3, (m * (x1 - x3) - y1) % p)
A_ = [0]
def mul(k, P, p):
    R = None
    while k:
        if k & 1: R = add(R, P, p)
        P = add(P, P, p); k >>= 1
    return R
def make_curve(rng, plo, phi):
    # y^2=(x-e1)(x-e2)(x-e3) with #E = 4N, N prime; full 2-torsion over Fp
    while True:
        p = rng.randrange(plo, phi)
        if not isprime(p) or p % 4 == 0: continue
        e = rng.sample(range(p), 3)
        e1, e2, e3 = e
        s = (e1 + e2 + e3) % p
        # expand: x^3 - s x^2 + c x - d ; shift to short form needs x -> x + s/3 (p>3)
        c = (e1 * e2 + e1 * e3 + e2 * e3) % p
        sq = [0] * p
        for y in range(p): sq[y * y % p] = y + 1  # root+1 marker
        chi = lambda a: 0 if a % p == 0 else (1 if sq[a % p] else -1)
        n = p + 1
        for x in range(p):
            n += chi((x - e1) * (x - e2) * (x - e3))
        if n % 4 or not isprime(n // 4) or n // 4 < 50: continue
        # work in Weierstrass y^2 = x^3 + a2 x^2 + a4 x + a6 -> shift x=u+s/3
        inv3 = pow(3, -1, p)
        sh = s * inv3 % p
        es = [(ei - sh) % p for ei in e]
        a = (es[0] * es[1] + es[0] * es[2] + es[1] * es[2]) % p
        b = (-es[0] * es[1] * es[2]) % p
        return p, es, a, b, n // 4, sq
def run(nc, plo, phi, kmax, seed):
    rng = random.Random(seed)
    nontriv_G = 0; tot_G = 0; pos_ok = 0; pos_tot = 0; leg_ok = 0; leg_tot = 0; aa_ok = 0
    for ci in range(nc):
        p, es, a, b, N, sq = make_curve(rng, plo, phi)
        A_[0] = a
        chi = lambda v: 0 if v % p == 0 else (1 if sq[v % p] else -1)
        def pts():
            while True:
                x = rng.randrange(p); r = (x ** 3 + a * x + b) % p
                if r and sq[r]: return (x, sq[r] - 1)
        def desc(Q): return tuple(chi(Q[0] - ei) for ei in es)
        # positive control: R of order 2N (descent nontrivial), kR leaks k mod 2
        while True:
            R = pts()
            if desc(R) != (1, 1, 1) and mul(4, R, p) is not None: break
        G = mul(4, R, p)  # prime order N
        assert mul(N, G, p) is None
        Q = None; K = min(N - 1, kmax); labs = [rng.randrange(2) for _ in range(K + 1)]
        Qp = None
        for k in range(1, K + 1):
            Q = add(Q, G, p); Qp = add(Qp, R, p)
            tot_G += 1
            if desc(Q) != (1, 1, 1): nontriv_G += 1
            leg_tot += 1
            if (chi(Q[0]) == 1) == (k % 2 == 0): leg_ok += 1
            if (chi(Q[0]) == 1) == (labs[k] == 0): aa_ok += 1
            if Qp is not None:
                d = desc(Qp); pos_tot += 1
                if (d != (1, 1, 1)) == (k % 2 == 1): pos_ok += 1
    sd = math.sqrt(leg_tot) / 2
    legz = (leg_ok - leg_tot / 2) / sd
    aaz = (aa_ok - leg_tot / 2) / sd
    pos_acc = pos_ok / pos_tot
    ok = nontriv_G == 0 and pos_acc == 1.0 and abs(legz) < 4 and abs(aaz) < 4
    if pos_acc < 1.0: v = "inconclusive"
    elif nontriv_G > 0: v = "falsified"
    elif abs(legz) < 4 and abs(aaz) < 4: v = "consistent_with_prediction"
    else: v = "inconclusive"
    return {"lead": "LEAD-STR-torsion-module-leak", "smoke": SMOKE, "seed": seed,
            "params": {"curves": nc, "p_range": [plo, phi], "kmax": kmax, "l": 2},
            "statistic": {"G_nontrivial_2descent_count": nontriv_G, "G_samples": tot_G},
            "positive_control_acc": pos_acc, "positive_samples": pos_tot,
            "negative_control_legendre_x_parity_z": legz,
            "AA_random_label_z": aaz, "verdict": v}
if SMOKE: r = run(2, 200, 400, 60, 1)
else: r = run(40, 3000, 9000, 1500, 20261110)
print(json.dumps(r))
