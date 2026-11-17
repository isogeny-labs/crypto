#!/usr/bin/env python3
"""LEAD-RHO-prime-gpu-rho: DP-lag overhead of W-parallel (GPU-style) rho, op-count toy. stdlib only."""
import sys, json, math, random, time

def is_prime(n):
    if n < 2: return False
    if n % 2 == 0: return n == 2
    i = 3
    while i * i <= n:
        if n % i == 0: return False
        i += 2
    return True

def add(P, Q, p, a):
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

def smul(k, P, p, a):
    R = None
    while k:
        if k & 1: R = add(R, P, p, a)
        P = add(P, P, p, a); k >>= 1
    return R

def make_curve(bits):
    p = (1 << bits) + 3
    while not (p % 4 == 3 and is_prime(p)): p += 1
    sq = bytearray(p)
    for y in range(p): sq[y * y % p] = 1
    a = 1
    for b in range(1, 10000):
        cnt = 1
        for x in range(p):
            r = (x * x * x + a * x + b) % p
            if r == 0: cnt += 1
            elif sq[r]: cnt += 2
        if is_prime(cnt):
            n = cnt
            for x in range(1, p):
                r = (x * x * x + a * x + b) % p
                if r and sq[r]:
                    P = (x, pow(r, (p + 1) // 4, p))
                    return dict(p=p, a=a, b=b, n=n, P=P)
    raise RuntimeError("no prime-order curve")

def solve(C, k, W, kdp, rng, cap):
    """W round-robin walkers, DP prob 2^-kdp. Returns (steps, solved, key_ok)."""
    p, a, n, P = C["p"], C["a"], C["n"], C["P"]
    Q = smul(k, P, p, a)
    mask = (1 << max(kdp, 0)) - 1
    M, U, V = [], [], []
    for _ in range(16):
        u, v = rng.randrange(1, n), rng.randrange(1, n)
        M.append(add(smul(u, P, p, a), smul(v, Q, p, a), p, a)); U.append(u); V.append(v)
    trail_cap = 20 * (1 << max(kdp, 0))
    def start():
        while True:
            s, t = rng.randrange(n), rng.randrange(n)
            pt = add(smul(s, P, p, a), smul(t, Q, p, a), p, a)
            if pt is not None: return [pt, s, t, 0]
    ws = [start() for _ in range(W)]
    table = {}
    steps = 0
    while steps < cap:
        for i in range(W):
            w = ws[i]
            pt = add(w[0], M[w[0][0] & 15], p, a)
            steps += 1
            if pt is None: ws[i] = start(); continue
            j = w[0][0] & 15
            w[0] = pt; w[1] = (w[1] + U[j]) % n; w[2] = (w[2] + V[j]) % n; w[3] += 1
            if kdp >= 0 and ((pt[0] >> 4) & mask) == 0:
                prev = table.get(pt)
                if prev is None:
                    table[pt] = (w[1], w[2])
                elif prev != (w[1], w[2]) and prev[1] != w[2]:
                    kk = (prev[0] - w[1]) * pow(w[2] - prev[1], -1, n) % n
                    return steps, True, smul(kk, P, p, a) == Q
                ws[i] = start()
            elif w[3] > trail_cap:
                ws[i] = start()
    return steps, False, True

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    plan = [(10, 6)] if smoke else [(16, 1000), (20, 600)]
    W = 4 if smoke else 16
    ks = (1, 2) if smoke else (2, 4)  # placeholder, overridden per size below
    out = dict(lead="LEAD-RHO-prime-gpu-rho", smoke=smoke, W=W, sizes=[], seeds="int seeds 20261110*1000+size*100000+trial*10+arm")
    checks = dict(all_keys_verified=True, neg_control_solved=0, aa_ok=True, powered=True, monotone=True)
    cs, fals, incon = [], False, False
    for bits, trials in plan:
        C = make_curve(bits); n = C["n"]; T0 = math.sqrt(math.pi * n / 2)
        kA, kB = (1, 3) if smoke else ((2, 4) if bits == 16 else (3, 5))
        arms = {"W1_A": (1, kA), "W1_Aprime": (1, kA), "WW_kA": (W, kA), "WW_kB": (W, kB)}
        res = {a: [] for a in arms}; unsolved = 0
        for t in range(trials):
            key = random.Random(20261110 * 1000 + bits * 100000 + t).randrange(1, n)
            for ai, (name, (w, kd)) in enumerate(arms.items()):  # interleaved
                rng = random.Random(20261110 * 1000 + bits * 100000 + t * 10 + ai + 1)
                s, ok, kok = solve(C, key, w, kd, rng, int(50 * T0) + 200)
                if not kok: checks["all_keys_verified"] = False
                if ok: res[name].append(s)
                else: unsolved += 1
        def ms(x):
            m = sum(x) / len(x); v = sum((y - m) ** 2 for y in x) / max(1, len(x) - 1)
            return m, math.sqrt(v / len(x))
        st = {a: ms(v) for a, v in res.items()}
        aa = abs(st["W1_A"][0] - st["W1_Aprime"][0]); aase = math.hypot(st["W1_A"][1], st["W1_Aprime"][1])
        base = (st["W1_A"][0] + st["W1_Aprime"][0]) / 2
        c = {}
        for nm, kd in (("WW_kA", kA), ("WW_kB", kB)):
            # base mean at same theta needed: W=1 arm is at kA only; use model for kB from W1 slope-free approx
            c[nm] = (st[nm][0] - base) / ((W - 1) * (1 << kd)) if nm == "WW_kA" else None
        # dose-response: excess difference between kB and kA arms over predicted lag difference
        c_dose = (st["WW_kB"][0] - st["WW_kA"][0]) / ((W - 1) * ((1 << kB) - (1 << kA)))
        # Note: W1 base differs by its own 1.5/theta lag; W=1 lag at kA included in base. c_dose is lag-slope per walker.
        if aa > 3 * aase: checks["aa_ok"] = False
        if (W - 1) * (1 << kA) * 0.5 < 3 * aase: checks["powered"] = False
        if st["WW_kB"][0] <= st["WW_kA"][0]: checks["monotone"] = False
        if unsolved > 0.01 * trials * 4: incon = True
        cs.append((c["WW_kA"], c_dose))
        out["sizes"].append(dict(bits=bits, n=n, curve=dict(p=C["p"], a=C["a"], b=C["b"]), T0=T0, trials=trials, kA=kA, kB=kB,
            mean_steps={a: [round(v[0], 2), round(v[1], 2)] for a, v in st.items()}, aa_diff=aa, aa_se=aase,
            c_vs_W1=c["WW_kA"], c_dose_slope=c_dose, unsolved=unsolved))
    # negative control: no DPs reachable -> must not solve within cap
    C = make_curve(plan[0][0]); n = C["n"]; T0 = math.sqrt(math.pi * n / 2)
    for t in range(5 if smoke else 20):
        s, ok, _ = solve(C, random.Random(7 + t).randrange(1, n), 4, -1, random.Random(99 + t), int(30 * T0))
        checks["neg_control_solved"] += int(ok)
    out["controls"] = checks
    out["prediction"] = "lag slope c (extra steps per extra walker per 1/theta) in [0.75,2.25] at both sizes and both estimators"
    allc = [x for pair in cs for x in pair]
    valid = checks["all_keys_verified"] and checks["neg_control_solved"] == 0 and checks["aa_ok"] and checks["powered"] and checks["monotone"] and not incon
    if not valid: v = "inconclusive"
    elif all(0.75 <= x <= 2.25 for x in allc): v = "consistent_with_prediction"
    else: v = "falsified"
    out["c_values"] = allc; out["verdict"] = v; out["wall_s"] = round(time.time() - t0, 2)
    print(json.dumps(out)); return 0

if __name__ == "__main__":
    sys.exit(main())
