#!/usr/bin/env python3
"""LEAD-RHO-kangaroo-interval: stdlib toy. Group = Z_q (q prime), generator 1; the
algorithms see only element labels and never use x except through the element."""
import json, math, random, sys
M64 = (1 << 64) - 1
GOLD = 0x9E3779B97F4A7C15

def next_prime(n):
    n |= 1
    while True:
        if all(n % d for d in range(3, int(n ** 0.5) + 1, 2)):
            return n
        n += 2

def hsh(e):
    return ((e * GOLD) & M64) >> 32

def dp_mod(q, frac):
    return max(2, int(math.sqrt(q) / frac))

def kangaroo(q, x, W, seed, dpfrac=16):
    """x in [0,W). Returns (ops, ok). Tame at midpoint, wild at x; alternate steps."""
    rng = random.Random(seed)
    target = max(1.0, math.sqrt(W) / 2)
    k = min(range(1, 40), key=lambda k: abs((2 ** k - 1) / k - target))
    jumps = [2 ** i for i in range(k)]
    D = dp_mod(W, dpfrac)
    mid = W // 2
    pos = [mid % q, x % q]; dist = [0, 0]
    table = {}
    cap = 60 * int(math.sqrt(W)) + 1000
    ops = 0
    while ops < cap:
        for who in (0, 1):
            h = hsh(pos[who])
            j = jumps[h % k]
            pos[who] = (pos[who] + j) % q; dist[who] += j; ops += 1
            h = hsh(pos[who])
            if (h // k) % D == 0:
                key = pos[who]
                if key in table and table[key][0] != who:
                    ow, od = table[key]
                    dt, dw = (dist[0], od) if who == 0 else (od, dist[1])
                    xr = (mid + dt - dw) % q
                    return ops, xr == x % q
                table[key] = (who, dist[who])
    return ops, False

def rho(q, x, seed, dpfrac=16):
    rng = random.Random(seed)
    nb = 20
    steps = [(rng.randrange(q), rng.randrange(1, q)) for _ in range(nb)]
    sv = [(a + b * x) % q for a, b in steps]
    D = dp_mod(q, dpfrac)
    table = {}
    cap = 60 * int(math.sqrt(q)) + 1000
    ops = 0
    while ops < cap:
        a, b = rng.randrange(q), rng.randrange(1, q)
        e = (a + b * x) % q
        for _ in range(20 * D):
            h = hsh(e)
            i = h % nb
            a = (a + steps[i][0]) % q; b = (b + steps[i][1]) % q
            e = (e + sv[i]) % q; ops += 1
            if (hsh(e) // nb) % D == 0:
                if e in table and table[e] != (a, b):
                    a2, b2 = table[e]
                    if b != b2:
                        xr = ((a2 - a) * pow(b - b2, -1, q)) % q
                        return ops, xr == x
                table[e] = (a, b)
                break
            if ops >= cap:
                break
    return ops, False

def boot_ratio(sizes_data, rng, B):
    """sizes_data: list of (numerator list, denominator list). Mean over sizes of ratio of means."""
    pt = sum((sum(n) / len(n)) / (sum(d) / len(d)) for n, d in sizes_data) / len(sizes_data)
    vals = []
    for _ in range(B):
        r = 0.0
        for n, d in sizes_data:
            nn = sum(rng.choice(n) for _ in n) / len(n)
            dd = sum(rng.choice(d) for _ in d) / len(d)
            r += nn / dd
        vals.append(r / len(sizes_data))
    vals.sort()
    return pt, vals[int(0.025 * B)], vals[int(0.975 * B) - 1]

def main():
    smoke = "--smoke" in sys.argv
    exps = [12] if smoke else [14, 16, 18, 20]
    T = 8 if smoke else 300
    B = 50 if smoke else 1000
    SEED = 20261110
    kang, rh, rh2, short, fail = [], [], [], [], {"kang": 0, "rho": 0, "rho_aa": 0, "short": 0}
    per = []
    for e in exps:
        q = next_prime(2 ** e)
        kk, r1, r2, ss = [], [], [], []
        for t in range(T):
            xr = random.Random(SEED + 7 * e + t * 1000003)
            x = xr.randrange(q)  # same secrets for all arms
            o, ok = kangaroo(q, x, q, SEED + 1 + t * 31 + e); kk.append(o); fail["kang"] += not ok
            o, ok = rho(q, x, SEED + 2 + t * 31 + e); r1.append(o); fail["rho"] += not ok
            o, ok = rho(q, x, SEED + 3 + t * 31 + e); r2.append(o); fail["rho_aa"] += not ok
            W = max(64, q // 256)
            o, ok = kangaroo(q, x % W, W, SEED + 4 + t * 31 + e); ss.append(o); fail["short"] += not ok
        kang.append(kk); rh.append(r1); rh2.append(r2); short.append(ss)
        per.append({"log2q": e, "q": q, "kang_mean": sum(kk) / T, "rho_mean": sum(r1) / T,
                    "kang_over_sqrtq": sum(kk) / T / math.sqrt(q), "rho_over_sqrtq": sum(r1) / T / math.sqrt(q)})
    br = random.Random(SEED)
    R = boot_ratio(list(zip(kang, rh)), br, B)
    AA = boot_ratio(list(zip(rh, rh2)), br, B)
    POS = boot_ratio(list(zip(short, kang)), br, B)  # short-interval / full-interval cost
    ctrl_aa = AA[1] <= 1.0 <= AA[2]
    ctrl_pos = 0.03 <= POS[0] <= 0.15
    nfail = sum(fail.values())
    if not (ctrl_aa and ctrl_pos) or nfail > 0.02 * 4 * T * len(exps):
        verdict = "inconclusive"
    elif R[1] >= 1.3 and R[2] <= 2.4:
        verdict = "consistent_with_prediction"
    elif R[2] < 1.3 or R[1] > 2.4:
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    print(json.dumps({
        "lead": "LEAD-RHO-kangaroo-interval", "smoke": smoke, "seed": SEED, "trials_per_size": T,
        "log2_sizes": exps, "dp_fraction": "D=sqrt(W)/16", "per_size": per,
        "statistic_kangaroo_over_rho_ops": {"point": R[0], "ci95": [R[1], R[2]]},
        "threshold": "CI within [1.3,2.4] consistent; CI wholly outside falsified",
        "control_AA_rho_vs_rho": {"point": AA[0], "ci95": [AA[1], AA[2]], "pass": ctrl_aa},
        "control_positive_short_over_full": {"point": POS[0], "ci95": [POS[1], POS[2]], "expected": "~0.0625", "pass": ctrl_pos},
        "failures": fail, "verdict": verdict}))

main()
