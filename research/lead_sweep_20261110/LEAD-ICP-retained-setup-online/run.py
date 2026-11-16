import sys, json, math, random

def nextprime(n):
    def ip(m):
        if m < 2: return False
        i = 2
        while i*i <= m:
            if m % i == 0: return False
            i += 1
        return True
    while not ip(n): n += 1
    return n

R = 32
def mk_steps(rng, N): return [rng.randrange(1, N) for _ in range(R)]
def hsh(x): return ((x * 2654435761) ^ (x >> 7)) % R

def online(N, S, rng, trials, permute=False, cap_mult=40):
    """Retained table: S random points a*g with stored dlog a. Target h=x*g.
    Walk w=h+c*g (c tracked) until w is in table; return steps list, wrong-count."""
    steps = mk_steps(rng, N)
    table = {}
    while len(table) < S:
        a = rng.randrange(N); table[a] = a
    if permute:
        keys = list(table); vals = keys[:]; rng.shuffle(vals)
        table = dict(zip(keys, vals))
    T = []; wrong = 0; cap = int(cap_mult * N / S) + 50
    for _ in range(trials):
        x = rng.randrange(N); h = x
        c = rng.randrange(N); w = (h + c) % N; n = 0; leg = 0; L = int(3 * N / S) + 1
        while w not in table and n < cap:
            if leg >= L:  # restart with fresh offset (cost counted) to escape rho-cycles
                c = rng.randrange(N); w = (h + c) % N; leg = 0
                continue
            m = steps[hsh(w)]; w = (w + m) % N; c = (c + m) % N; n += 1; leg += 1
        T.append(n)
        if w in table:
            if (table[w] - c) % N != x: wrong += 1
        else: wrong += 1
    return T, wrong

def rho(N, rng, trials):
    steps = mk_steps(rng, N); T = []
    for _ in range(trials):
        w = rng.randrange(N); seen = {w}; n = 0
        while True:
            w = (w + steps[hsh(w)]) % N; n += 1
            if w in seen: break
            seen.add(w)
        T.append(n)
    return T

def mean(v): return sum(v) / len(v)

def main():
    smoke = "--smoke" in sys.argv
    sizes = [2**10] if smoke else [2**14, 2**16, 2**18, 2**20]
    trials = 20 if smoke else 300
    rtr = 20 if smoke else 150
    out = {"seed_base": 20261110, "smoke": smoke, "rows": []}
    ratios_ts = []; ratios_rho = []; pred_rho = []
    for k, n0 in enumerate(sizes):
        N = nextprime(n0 + 1)
        S = max(2, int(round(N ** (2/3))))
        rng = random.Random(20261110 + k)
        T1, w1 = online(N, S, random.Random(1000 + k), trials)
        T2, w2 = online(N, S, random.Random(2000 + k), trials)   # A/A arm: fresh seed, same protocol
        tsn = mean(T1) * S / N
        aa = mean(T1) / mean(T2)
        rh = mean(rho(N, random.Random(3000 + k), rtr))
        ratio = rh / mean(T1)
        formula = math.sqrt(math.pi * N / 2) / (N / S)
        ratios_ts.append(tsn); ratios_rho.append(ratio); pred_rho.append(formula)
        # positive control: S = N/4, expect T ~ 4
        Tp, _ = online(N, N // 4, random.Random(4000 + k), trials)
        # negative control: permuted dlog labels -> verification must fail
        Tn, wn = online(N, S, random.Random(5000 + k), trials // 4 + 1, permute=True)
        out["rows"].append({"N": N, "S": S, "mean_T": mean(T1), "T_S_over_N": tsn, "AA_ratio": aa,
            "mean_rho_steps": rh, "rho_over_online": ratio, "formula_ratio": formula,
            "wrong_rate": (w1 + w2) / (2 * trials), "pos_ctrl_meanT_expect4": mean(Tp),
            "neg_ctrl_wrong_rate": wn / (trials // 4 + 1)})
    # sweep S at fixed N: slope of log T vs log S
    N = nextprime(2**16 + 1) if not smoke else nextprime(2**10)
    Ss = [max(2, int(N ** e)) for e in (0.5, 0.6, 0.7, 0.8)]
    ls = []; lt = []
    for i, S in enumerate(Ss):
        T, _ = online(N, S, random.Random(6000 + i), trials)
        ls.append(math.log(S)); lt.append(math.log(mean(T)))
    mx, my = mean(ls), mean(lt)
    slope = sum((a - mx) * (b - my) for a, b in zip(ls, lt)) / sum((a - mx) ** 2 for a in ls)
    out["slope_logT_vs_logS"] = slope
    ok_ts = all(0.6 <= t <= 1.6 for t in ratios_ts)
    ok_slope = -1.15 <= slope <= -0.85
    ok_ratio = all(0.7 <= r / f <= 1.4 for r, f in zip(ratios_rho, pred_rho))
    ok_ctrl = all(r["neg_ctrl_wrong_rate"] > 0.9 and r["wrong_rate"] == 0 and 2.5 <= r["pos_ctrl_meanT_expect4"] <= 6 and 0.7 <= r["AA_ratio"] <= 1.4 for r in out["rows"])
    out["checks"] = {"T_S_over_N_in_[0.6,1.6]": ok_ts, "slope_in_[-1.15,-0.85]": ok_slope,
                     "rho_ratio_within_[0.7,1.4]_of_formula": ok_ratio, "controls_ok": ok_ctrl}
    if not ok_ctrl: out["verdict"] = "inconclusive"
    elif ok_ts and ok_slope and ok_ratio: out["verdict"] = "consistent_with_prediction"
    else: out["verdict"] = "falsified"
    print(json.dumps(out))

main()
