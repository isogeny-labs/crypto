#!/usr/bin/env python3
"""Coset-typed vs untyped m=3 decomposition coverage toy (stdlib only)."""
import sys, json, random, math

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def prime_near(bits):
    n = (1 << bits) + 1
    while not (n % 4 == 3 and is_prime(n)): n += 1
    return n

def build_group(p, rng):
    """Cyclic subgroup <G> of a random curve; returns N, xs[k]=x(kG) for k=1..N-1."""
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        x = rng.randrange(p); r = (x**3 + a*x + b) % p
        y = pow(r, (p+1)//4, p)
        if y*y % p != r or y == 0: continue
        G = (x, y); P = G; xs = [None, x]
        while True:
            x1, y1 = P
            if x1 == x and (y1 + y) % p == 0: break          # P = -G -> next is O
            if P == G: lam = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
            else: lam = (y1 - y) * pow(x1 - x, -1, p) % p
            x3 = (lam*lam - x1 - x) % p
            P = (x3, (lam*(x1 - x3) - y1) % p)
            xs.append(x3)
            if len(xs) > p + 3: break
        N = len(xs)                                           # order of G
        if N >= p // 2: return N, xs

def cover(reps_k, labels, N, typed):
    """reps_k: group elements; returns (T, distinct covered)."""
    seen = bytearray(N); B = len(reps_k)
    if not typed:
        for i in range(B):
            a = reps_k[i]
            for j in range(i+1, B):
                s = a + reps_k[j]
                for k in range(j+1, B): seen[(s + reps_k[k]) % N] = 1
        T = B*(B-1)*(B-2)//6
    else:
        A = [k for k, l in zip(reps_k, labels) if l]; Bb = [k for k, l in zip(reps_k, labels) if not l]
        for i in range(len(A)):
            for j in range(i+1, len(A)):
                s = A[i] + A[j]
                for c in Bb: seen[(s + c) % N] = 1
        T = len(A)*(len(A)-1)//2*len(Bb)
    return T, sum(seen)

def grow(order, labels_of, N, typed):
    """smallest prefix of `order` with T >= N."""
    for B in range(3, len(order)+1):
        pre = order[:B]
        if typed:
            a = sum(1 for k in pre if labels_of(k)); b = B - a
            T = a*(a-1)//2*b
        else: T = B*(B-1)*(B-2)//6
        if T >= N: return pre
    return order

def one_curve(p, seed):
    rng = random.Random(seed); N, xs = build_group(p, rng)
    half = N // 2
    ks = list(range(1, half + 1))                      # one representative of each +-pair
    chi = lambda k: pow(xs[k], (p-1)//2, p) == 1
    byx = sorted(ks, key=lambda k: xs[k])
    out = {"N": N}
    def pred(T): return N * (1 - math.exp(-T / N))
    def run(name, order, typed, lab):
        pre = grow(order, lab, N, typed)
        T, c = cover(pre, [lab(k) for k in pre], N, typed)
        out[name] = {"B": len(pre), "T": T, "R": c / pred(T)}
    run("x_untyped", byx, False, chi)
    run("x_typed", byx, True, chi)
    rnd = ks[:]; rng.shuffle(rnd)
    rl = {k: rng.random() < 0.5 for k in ks}
    run("null_untyped", rnd, False, rl.get)
    rnd2 = ks[:]; rng.shuffle(rnd2)
    run("null_typed", rnd2, True, rl.get)
    rnd3 = ks[:]; rng.shuffle(rnd3)
    run("aa_untyped", rnd3, False, rl.get)             # A/A: independent replicate of null arm
    run("pos_ap", ks, False, rl.get)                   # arithmetic-progression base: planted structure
    return out

def main():
    smoke = "--smoke" in sys.argv
    bits = [10] if smoke else [12, 14, 16, 18]
    reps = 2 if smoke else 10
    arms = ["x_untyped", "x_typed", "null_untyped", "null_typed", "aa_untyped", "pos_ap"]
    res = {}; allr = {a: [] for a in arms}
    for bi in bits:
        p = prime_near(bi)
        rows = [one_curve(p, 1000*bi + r) for r in range(reps)]
        res[str(bi)] = {"p": p, "mean_N": sum(r["N"] for r in rows)/reps,
                        "mean_R": {a: sum(r[a]["R"] for r in rows)/reps for a in arms},
                        "mean_B": {a: sum(r[a]["B"] for r in rows)/reps for a in arms}}
        for r in rows:
            for a in arms: allr[a].append(r[a]["R"])
    def ms(v):
        m = sum(v)/len(v); s = (sum((x-m)**2 for x in v)/max(1, len(v)-1))**.5; return m, s
    nulls = allr["null_untyped"] + allr["null_typed"]
    nm, ns = ms(nulls); band = max(3*ns, 0.02)
    stats = {a: ms(allr[a]) for a in arms}
    inb = lambda a: abs(stats[a][0] - nm) <= band
    pos_seen = nm - stats["pos_ap"][0] > band
    aa_ok = abs(stats["aa_untyped"][0] - stats["null_untyped"][0]) <= band
    if not (pos_seen and aa_ok): verdict = "inconclusive"
    elif inb("x_untyped") and inb("x_typed"): verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-ICE-semaev-yield-model-coset-typing", "smoke": smoke,
        "seeds": "1000*bits+rep", "bits": bits, "reps": reps, "m": 3,
        "statistic": "R = distinct covered targets / (N(1-exp(-T/N)))",
        "null_mean": nm, "null_sd": ns, "band": band,
        "arm_mean_sd": stats, "per_size": res,
        "controls": {"positive_detected": pos_seen, "aa_ok": aa_ok},
        "verdict": verdict}))
main()
