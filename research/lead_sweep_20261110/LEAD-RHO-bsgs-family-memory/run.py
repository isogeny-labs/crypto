import json, math, random, sys
SMOKE = '--smoke' in sys.argv

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True
def prime_near(e):
    n = 2 ** e + 1
    while not isprime(n): n += 1
    return n

def bsgs(n, h, m):
    """generic additive group Z_n, generator 1; ops = group additions; memory = m."""
    ops = 0; tab = {}; x = 0
    for j in range(m):
        tab.setdefault(x, j); x = (x + 1) % n; ops += 1
    step = (-m) % n; y = h
    for i in range(0, n // m + 2):
        if y in tab:
            return (i * m + tab[y]) % n, ops, len(tab)
        y = (y + step) % n; ops += 1
    raise RuntimeError

def rho(n, h, rng):
    """16-adding walks, distinguished points; ops = steps; memory = stored DPs."""
    M = [(rng.randrange(n), rng.randrange(n)) for _ in range(16)]
    Mv = [(a + b * h) % n for a, b in M]
    mask = max(1, int(math.sqrt(n) / 32)); store = {}; ops = 0
    cap = 40 * int(math.sqrt(n)) + 1000
    walks = [None] * 8
    while ops < cap:
        for w in range(8):
            if walks[w] is None:
                a, b = rng.randrange(n), rng.randrange(n)
                walks[w] = [a, b, (a + b * h) % n, 0]
            s = walks[w]; i = s[2] % 16
            s[0] = (s[0] + M[i][0]) % n; s[1] = (s[1] + M[i][1]) % n
            s[2] = (s[2] + Mv[i]) % n; s[3] += 1; ops += 1
            if s[2] % mask == 0:
                if s[2] in store:
                    a2, b2 = store[s[2]]
                    db = (s[1] - b2) % n
                    if db:
                        x = ((a2 - s[0]) * pow(db, -1, n)) % n
                        if x == h: return x, ops, len(store)
                else: store[s[2]] = (s[0], s[1])
                walks[w] = None
            elif s[3] > 20 * mask: walks[w] = None
    return None, ops, len(store)

def slope(xs, ys):
    mx = sum(xs) / len(xs); my = sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def run_arm(name, exps, T, seed, solver):
    rng = random.Random(seed); perN = []; mem = []
    for e in exps:
        n = prime_near(e); vals = []; ms = []
        for _ in range(T):
            h = rng.randrange(1, n)
            if name == 'rho':
                x, o, mm = rho(n, h, rng)
                if x is None: o = rng.random() * 0 + o
            else:
                x, o, mm = bsgs(n, h, solver(n)); assert x == h
            vals.append(o); ms.append(mm)
        perN.append((n, vals)); mem.append(sum(ms) / len(ms))
    xs = [math.log2(n) for n, _ in perN]
    s = slope(xs, [math.log2(sum(v) / len(v)) for _, v in perN])
    brng = random.Random(seed + 1); bs = []
    for _ in range(200):
        ys = []
        for n, v in perN:
            ys.append(math.log2(sum(brng.choice(v) for _ in v) / len(v)))
        bs.append(slope(xs, ys))
    bs.sort()
    return {"slope": s, "ci95": [bs[5], bs[194]], "mean_ops": [sum(v) / len(v) for _, v in perN],
            "mean_memory": mem, "n": [n for n, _ in perN]}

def main():
    exps = [10, 12, 14] if SMOKE else [12, 14, 16, 18, 20, 22]
    T = 6 if SMOKE else 40
    s0 = 20261110
    arms = {
        "bsgs_m_sqrt_positive_control": run_arm('b', exps, T, s0 + 1, lambda n: int(n ** 0.5) + 1),
        "bsgs_m_sqrt_AA_rerun": run_arm('b', exps, T, s0 + 2, lambda n: int(n ** 0.5) + 1),
        "bsgs_m_n^(1/3)_test": run_arm('b', exps, T, s0 + 3, lambda n: max(2, round(n ** (1 / 3)))),
        "bsgs_m_n^(1/4)_test": run_arm('b', exps, T, s0 + 4, lambda n: max(2, round(n ** 0.25))),
        "rho_dp_null_lowmem": run_arm('rho', exps, T, s0 + 5, None),
    }
    pc = arms["bsgs_m_sqrt_positive_control"]["slope"]
    aa = arms["bsgs_m_sqrt_AA_rerun"]["slope"]
    t3 = arms["bsgs_m_n^(1/3)_test"]; rh = arms["rho_dp_null_lowmem"]
    if SMOKE:
        verdict = "inconclusive"
    elif abs(pc - 0.5) > 0.06 or abs(pc - aa) > 0.03:
        verdict = "inconclusive"
    elif t3["ci95"][1] < 0.58:
        verdict = "falsified"
    elif t3["ci95"][0] > 0.58 and t3["slope"] > rh["slope"] + 0.08 and abs(t3["slope"] - 2 / 3) < 0.07:
        verdict = "consistent_with_prediction"
    else:
        verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-RHO-bsgs-family-memory", "smoke": SMOKE, "sizes_log2": exps,
        "targets_per_size": T, "seed_base": s0, "model": "generic additive Z_n, ops = group additions",
        "threshold": "t3 slope CI lower > 0.58, ~2/3 +-0.07, > rho slope + 0.08; pos ctrl 0.5+-0.06, A/A diff<0.03",
        "arms": arms, "verdict": verdict}))
main()
