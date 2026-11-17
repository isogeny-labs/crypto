#!/usr/bin/env python3
"""LEAD-EXO-ml-learned-models: learnability control on toy prime-order E(F_p). stdlib only."""
import sys, json, math, random

M_LIST = [2,3,4,5,7,8,9,11,13,16,17,19,23,32,64]

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def order(p, a, b):
    sq = bytearray(p)
    for y in range(p): sq[y * y % p] = 1
    cnt = 1
    for x in range(p):
        r = (x * x * x + a * x + b) % p
        if r == 0: cnt += 1
        elif sq[r]: cnt += 2
    return cnt

def find_curve(bits, rng):
    p = next_prime((1 << bits) + rng.randrange(1 << (bits - 1)))
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a**3 + 27 * b * b) % p == 0:
            continue
        N = order(p, a, b)
        if is_prime(N) and N > 1000:
            return p, a, b, N
        if rng.random() < 0.02:
            p = next_prime(p + 1)

def find_point(p, a, b, rng):
    while True:
        x = rng.randrange(p)
        r = (x * x * x + a * x + b) % p
        if pow(r, (p - 1) // 2, p) == 1 and r:
            if p % 4 == 3: y = pow(r, (p + 1) // 4, p)
            else:
                y = next((t for t in range(1, p) if t * t % p == r), None)
                if y is None: continue
            return (x, y)

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

def all_points(p, a, b, N, rng):
    G = find_point(p, a, b, rng)
    pts = [None] * N
    Q = G
    for k in range(1, N):
        pts[k] = Q
        Q = add(Q, G, p, a)
    assert Q is None  # order N verified
    return pts

def feats(pt, p):
    x, y = pt
    f = []
    off = 0
    for m in M_LIST:
        f.append(off + x % m); off += m
    for m in (2, 3, 4, 8):
        f.append(off + y % m); off += m
    f.append(off + (1 if y > p // 2 else 0)); off += 2
    for s in range(8):
        f.append(off + ((x * 256 // p) >> s & 1) * 8 + s if False else off + (((x << 8) // p) >> s & 1) + 2 * s); 
    off += 16
    return f, off

def train_eval(Xtr, ytr, Xte, yte, dim, seed, epochs):
    rng = random.Random(seed)
    w = [0.0] * dim; bias = 0.0
    idx = list(range(len(Xtr)))
    lr = 0.05
    for _ in range(epochs):
        rng.shuffle(idx)
        for i in idx:
            f = Xtr[i]
            z = bias + sum(w[j] for j in f)
            z = max(-30.0, min(30.0, z))
            g = 1.0 / (1.0 + math.exp(-z)) - ytr[i]
            for j in f: w[j] -= lr * g
            bias -= lr * g
    ok = 0
    for f, y in zip(Xte, yte):
        ok += ((bias + sum(w[j] for j in f)) > 0) == (y == 1)
    return ok / len(yte)

def zstat(acc, n):
    return (acc - 0.5) / math.sqrt(0.25 / n)

def labels(k, N, which):
    return {"lsb": k & 1, "bit1": (k >> 1) & 1, "msb": 1 if 2 * k >= N else 0}[which]

def run(cfg):
    seeds = cfg["seeds"]
    out = {"curves": [], "null_ctrl": [], "pos_ctrl": [], "aa": []}
    wlist = ["lsb", "bit1", "msb"]
    for ci, bits in enumerate(cfg["bits"]):
        crng = random.Random(1000 + ci)
        p, a, b, N = find_curve(bits, crng)
        pts = all_points(p, a, b, N, crng)
        ntr = min(cfg["ntrain"], (N - 1) // 2); nte = min(cfg["ntest"], (N - 1) // 2)
        rec = {"p": p, "a": a, "b": b, "N": N, "ntrain": ntr, "ntest": nte, "tests": {}}
        for rep, sd in enumerate(seeds):
            srng = random.Random(sd * 7919 + ci)
            ks = srng.sample(range(1, N), ntr + nte)
            F = []
            for k in ks:
                f, dim = feats(pts[k], p); F.append(f)
            for w in wlist:
                y = [labels(k, N, w) for k in ks]
                acc = train_eval(F[:ntr], y[:ntr], F[ntr:], y[ntr:], dim, sd, cfg["epochs"])
                rec["tests"].setdefault(w, []).append({"seed": sd, "acc": acc, "z": zstat(acc, nte)})
                # A/A: same data, different SGD order seed
                acc2 = train_eval(F[:ntr], y[:ntr], F[ntr:], y[ntr:], dim, sd + 555, cfg["epochs"])
                out["aa"].append(abs(acc - acc2))
                # negative control: shuffled labels
                ys = y[:]; srng.shuffle(ys)
                accn = train_eval(F[:ntr], ys[:ntr], F[ntr:], ys[ntr:], dim, sd, cfg["epochs"])
                out["null_ctrl"].append(zstat(accn, nte))
                # positive control: planted leak feature agreeing with label w.p. 0.65
                lk = [dim + (v if srng.random() < 0.65 else 1 - v) for v in y]
                Fp = [f + [l] for f, l in zip(F, lk)]
                accp = train_eval(Fp[:ntr], y[:ntr], Fp[ntr:], y[ntr:], dim + 2, sd, cfg["epochs"])
                out["pos_ctrl"].append(zstat(accp, nte))
        out["curves"].append(rec)
    return out

def main():
    smoke = "--smoke" in sys.argv
    cfg = {"bits": [10], "ntrain": 300, "ntest": 300, "epochs": 2, "seeds": [1, 2]} if smoke else \
          {"bits": [12, 14, 16, 18], "ntrain": 8000, "ntest": 8000, "epochs": 4, "seeds": [11, 22]}
    o = run(cfg)
    thr = 4.0
    # real tests: z per (curve,label,seed)
    flagged = 0; maxz = 0.0
    for rec in o["curves"]:
        for w, lst in rec["tests"].items():
            zs = [t["z"] for t in lst]
            maxz = max(maxz, max(abs(z) for z in zs))
            if all(abs(z) >= thr for z in zs) and len(zs) > 1: flagged += 1
            elif any(abs(z) >= thr for z in zs): flagged += 0.5
    null_max = max(abs(z) for z in o["null_ctrl"])
    pos_min = min(o["pos_ctrl"])
    if smoke:
        verdict = "inconclusive"
    elif pos_min < thr or null_max >= thr:
        verdict = "inconclusive"
    elif flagged >= 1:
        verdict = "falsified"
    elif flagged == 0:
        verdict = "consistent_with_prediction"
    else:
        verdict = "inconclusive"
    if not smoke and 0 < flagged < 1: verdict = "inconclusive"
    res = {"lead": "LEAD-EXO-ml-learned-models", "smoke": smoke, "params": cfg, "threshold_z": thr,
           "statistic": "held-out accuracy z vs 0.5, logistic regression on x/y residue+bit features",
           "max_abs_z_real": maxz, "flag_score": flagged, "null_ctrl_max_abs_z": null_max,
           "pos_ctrl_min_z": pos_min, "aa_max_abs_acc_diff": max(o["aa"]),
           "curves": o["curves"], "verdict": verdict}
    print(json.dumps(res))

if __name__ == "__main__":
    main()
