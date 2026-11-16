#!/usr/bin/env python3
"""Store-all schedule constants in Z/l (stdlib only). Preregistered in PROTOCOL.md."""
import json, math, random, sys

def single_list(l, x, rng):
    """rho-like store-all: every point (a,b) random, value a+b*x; any collision with b!=b' solves."""
    seen = {}; n = 0
    while True:
        a = rng.randrange(l); b = rng.randrange(l); n += 1
        v = (a + b * x) % l
        if v in seen and seen[v][1] != b:
            return n
        seen[v] = (a, b)

def two_list(l, x, rng):
    """alternating random tame (b=0) / wild (b=1) lists; only cross collisions solve."""
    T = set(); W = set(); n = 0
    while True:
        a = rng.randrange(l); n += 1
        if n & 1:
            if a in W: return n
            T.add(a)
        else:
            v = (a + x) % l  # wild point value a+x
            if v in T: return n
            W.add(v)

def bsgs(l, x, rng):
    n = max(1, math.isqrt(l // 2))
    return n + x // n + 1

def grumpy(l, x, rng):
    """deterministic stored babies 0,1,2,.. interleaved with random wild points (store both)."""
    B = set(); W = set(); k = 0; n = 0
    while True:
        B.add(k); n += 1
        if k in W: return n
        k += 1
        v = (rng.randrange(l) + x) % l; n += 1
        if v in B: return n
        W.add(v)

SCHED = {"single_list": single_list, "two_list": two_list, "bsgs_opt": bsgs, "grumpy_det_baby": grumpy}
THEORY = {"single_list": math.sqrt(math.pi / 2), "two_list": math.sqrt(math.pi),
          "bsgs_opt": math.sqrt(2), "grumpy_det_baby": None}

def measure(l, trials, seed, name):
    rng = random.Random(seed)
    f = SCHED[name]; tot = []; 
    for _ in range(trials):
        x = rng.randrange(1, l)
        tot.append(f(l, x, rng) / math.sqrt(l))
    m = sum(tot) / len(tot)
    sd = math.sqrt(sum((t - m) ** 2 for t in tot) / (len(tot) - 1))
    return m, sd / math.sqrt(len(tot))

def main():
    smoke = "--smoke" in sys.argv
    sizes = [251] if smoke else [4093, 65521, 1048573, 4194301]
    trials = {251: 60, 4093: 1500, 65521: 800, 1048573: 300, 4194301: 120}
    out = {"seeds": "1000+i (arm A), 2000+i (A/A arm B)", "sizes": sizes, "rows": []}
    ok_pos = True; ok_order = True; aa_ok = True
    for i, l in enumerate(sizes):
        t = trials[l]; row = {"l": l, "trials": t}
        for j, name in enumerate(SCHED):
            m, se = measure(l, t, 1000 + 10 * i + j, name)
            m2, se2 = measure(l, t, 2000 + 10 * i + j, name)
            row[name] = {"mean": round(m, 4), "se": round(se, 4), "aa_mean": round(m2, 4), "aa_se": round(se2, 4)}
            if abs(m - m2) > 4 * math.hypot(se, se2): aa_ok = False
            th = THEORY[name]
            if name == "single_list" and abs(m - th) > 4 * se + 0.03 * th + (0.1 if smoke else 0.0):
                ok_pos = False
        # prediction: single_list < bsgs_opt < two_list ; grumpy_det_baby ~ two_list within 5%
        s, b, tw, g = (row[k]["mean"] for k in ("single_list", "bsgs_opt", "two_list", "grumpy_det_baby"))
        if not (s < b < tw) or abs(g - tw) > 0.05 * tw + (0.3 if smoke else 0): ok_order = False
        out["rows"].append(row)
    out["positive_control_single_list_matches_sqrt(pi/2)"] = ok_pos
    out["aa_consistent"] = aa_ok
    out["ordering_prediction_holds"] = ok_order
    out["verdict"] = ("inconclusive" if not (ok_pos and aa_ok) else
                      "consistent_with_prediction" if ok_order else "falsified")
    out["smoke"] = smoke
    print(json.dumps(out))

main()
