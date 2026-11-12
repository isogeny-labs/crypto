#!/usr/bin/env python3
"""LEAD-ICE-rootfind-pairtable-oracle toy: priced pair-table m=2 IC vs negation-fold rho. stdlib only."""
import sys, json, math, random, time, statistics

C = {"add": 0, "nat": 0}


def isprime(n):
    if n < 2: return False
    if n % 2 == 0: return n == 2
    f = 3
    while f * f <= n:
        if n % f == 0: return False
        f += 2
    return True


class Curve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b

    def add(s, P, Q):
        C["add"] += 1
        if P is None: return Q
        if Q is None: return P
        p = s.p
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            l = (3 * x1 * x1 + s.a) * pow(2 * y1, -1, p) % p
        else:
            l = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (l * l - x1 - x2) % p
        return (x3, (l * (x1 - x3) - y1) % p)

    def neg(s, P): return None if P is None else (P[0], (-P[1]) % s.p)

    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            k >>= 1
            if k: P = s.add(P, P)
        return R


def make_instance(bits, rng):
    while True:
        p = rng.randrange(1 << (bits - 1), 1 << bits) | 3
        if p % 4 == 3 and isprime(p): break
        # loop until prime
    while True:
        p = rng.randrange(1 << (bits - 1), 1 << bits) | 3
        if p % 4 == 3 and isprime(p): break
    sq = bytearray(p)
    for y in range(p): sq[y * y % p] = 1
    for _ in range(200):
        a, b = rng.randrange(p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        cnt = 1
        for x in range(p):
            v = (x * x % p * x + a * x + b) % p
            cnt += 0 if False else (1 if v == 0 else (2 if sq[v] else 0))
        for h in (1, 2, 3, 4, 6, 8):
            if cnt % h == 0 and isprime(cnt // h) and cnt // h >= p // 12:
                n = cnt // h
                E = Curve(p, a, b)
                while True:
                    x = rng.randrange(p); v = (x * x % p * x + a * x + b) % p
                    if v and sq[v]:
                        P = E.mul(h, (x, pow(v, (p + 1) // 4, p)))
                        if P is not None: break
                return E, P, n
    raise RuntimeError("no curve")


def walk_table(E, P, Q, n, rng, k=16):
    W = []
    for _ in range(k):
        m, l = rng.randrange(1, n), rng.randrange(1, n)
        W.append((E.add(E.mul(m, P), E.mul(l, Q)), m, l))
    return W


def rho(E, P, Q, n, bits, rng, W):
    p = E.p
    t = max(1, bits // 2 - 4); mask = (1 << t) - 1; cap = 20 << t
    dps = {}
    while True:
        a, b = rng.randrange(1, n), rng.randrange(1, n)
        X = E.add(E.mul(a, P), E.mul(b, Q)); steps = 0
        while X is not None and steps < cap:
            Wk, m, l = W[X[0] % 16]; C["nat"] += 3
            X = E.add(X, Wk); a = (a + m) % n; b = (b + l) % n; steps += 1
            if X is None: break
            if X[1] > p - X[1]: X = (X[0], p - X[1]); a, b = (-a) % n, (-b) % n
            if X[0] & mask == 0:
                C["nat"] += 1
                if X in dps:
                    a2, b2 = dps[X]
                    if b2 != b:
                        d = (a - a2) * pow(b2 - b, -1, n) % n
                        if E.mul(d, P) == Q: return d
                    break
                dps[X] = (a, b); break


def ic(E, P, Q, n, bits, rng, W, free_table=False):
    p = E.p
    B = math.ceil(2 * n ** (1 / 3))
    FB = []; x = 0
    while len(FB) < B:
        v = (x * x % p * x + E.a * x + E.b) % p
        if v == 0 or pow(v, (p - 1) // 2, p) == 1:
            FB.append((x, pow(v, (p + 1) // 4, p))); C["nat"] += bits
        x += 1
    adds0, nat0 = C["add"], C["nat"]
    T = {}
    for i in range(B):
        for j in range(i, B):
            for ei in (1, -1):
                for ej in (1, -1):
                    if i == j and ej == -ei: continue
                    S = E.add(FB[i] if ei == 1 else E.neg(FB[i]), FB[j] if ej == 1 else E.neg(FB[j]))
                    C["nat"] += 1
                    if S is not None and S not in T: T[S] = (i, ei, j, ej)
    if free_table: C["add"], C["nat"] = adds0, nat0
    piv = {}; trials = hits = 0; ncols = B  # col B is d
    a = rng.randrange(1, n); b = rng.randrange(1, n)
    R = E.add(E.mul(a, P), E.mul(b, Q))
    while B not in piv:
        Wk, m, l = W[rng.randrange(16)]
        R = E.add(R, Wk); a = (a + m) % n; b = (b + l) % n; trials += 1
        if R is None: continue
        C["nat"] += 2
        s = 1; key = R
        if R not in T:
            key = E.neg(R); s = -1
            if key not in T: continue
        i, ei, j, ej = T[key]; hits += 1
        row = {}
        for c, v in ((i, -s * ei), (j, -s * ej)): row[c] = (row.get(c, 0) + v) % n
        row[B] = (row.get(B, 0) + b) % n
        row = {c: v for c, v in row.items() if v}; rhs = (-a) % n
        while row:
            c = min(row)
            if c in piv:
                pr, pr_rhs = piv[c]; f = row[c]
                for cc, vv in pr.items():
                    nv = (row.get(cc, 0) - f * vv) % n; C["nat"] += 1
                    if nv: row[cc] = nv
                    else: row.pop(cc, None)
                rhs = (rhs - f * pr_rhs) % n; C["nat"] += 1
            else:
                inv = pow(row[c], -1, n); C["nat"] += 1
                piv[c] = ({cc: vv * inv % n for cc, vv in row.items()}, rhs * inv % n)
                break
    d = piv[B][1]
    return (d if E.mul(d, P) == Q else None), trials, hits, len(T)


def cost(adds, nat, w): return adds + w * nat


def run_arm(fn, E, P, Q, n, bits, seed, W, **kw):
    C["add"] = C["nat"] = 0
    r = fn(E, P, Q, n, bits, random.Random(seed), W, **kw)
    return r, C["add"], C["nat"]


def main():
    smoke = "--smoke" in sys.argv
    sizes = [10, 12] if smoke else [12, 14, 16, 18, 20]
    reps = 2 if smoke else 5
    deadline = time.time() + (8 if smoke else 540)
    WS = [0.0, 0.1, 1.0]
    rec = {}; ok_all = True; yo = []; ic_wr = None
    for bits in sizes:
        rec[bits] = {"rhoA": [], "rhoB": [], "ic": [], "icfree": []}
        for i in range(reps):
            if time.time() > deadline: break
            rng = random.Random("inst-%d-%d" % (bits, i))
            E, P, n = make_instance(bits, rng)
            d = rng.randrange(1, n); Q = E.mul(d, P)
            C["add"] = C["nat"] = 0
            W = walk_table(E, P, Q, n, random.Random("walk-%d-%d" % (bits, i)))
            wc = C["add"]
            order = [("rhoA", 1), ("ic", 2), ("rhoB", 3), ("icfree", 4)]
            random.Random("ord-%d-%d" % (bits, i)).shuffle(order)
            for name, sd in order:
                s = "arm-%d-%d-%d" % (bits, i, sd)
                if name.startswith("rho"):
                    r, ad, nt = run_arm(rho, E, P, Q, n, bits, s, W)
                    ok_all &= (r == d)
                    rec[bits][name].append((ad + wc, nt))
                else:
                    (r, tr, h, T), ad, nt = run_arm(ic, E, P, Q, n, bits, s, W, free_table=(name == "icfree"))
                    ok_all &= (r == d)
                    rec[bits][name].append((ad + wc, nt))
                    if name == "ic": yo.append((h / tr, T / (n * max(1, round((E.p + 1) / n)))))
    done = [b for b in sizes if len(rec[b]["ic"]) == reps]
    out = {"lead": "LEAD-ICE-rootfind-pairtable-oracle", "smoke": smoke, "sizes_completed": done, "reps": reps,
           "seeds": "random.Random(str) keys inst-/walk-/arm-/ord-<bits>-<rep>", "w_band_GAE_per_native_op": WS,
           "ic_params": "B=ceil(2 n^(1/3)) x-sorted FB, full +-pair table, m=2 negfold", "rho_params": "16-adding walk, negation fold, DP bits=max(1,bits//2-4)"}
    if len(done) < 2:
        out.update(verdict="inconclusive", note="too few sizes completed"); print(json.dumps(out)); return
    med = statistics.median
    ratios = {}; aa = []
    for b in done:
        for w in WS:
            ca = [cost(*x, w) for x in rec[b]["rhoA"]]; cb = [cost(*x, w) for x in rec[b]["rhoB"]]
            ci = [cost(*x, w) for x in rec[b]["ic"]]
            ratios.setdefault(w, {})[b] = med(ci) / med(ca + cb)
            if w == 0.1: aa.append(math.log2(med(ca) / med(cb)))
    def slope(xs, ys):
        mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
        return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    brng = random.Random("boot"); sl = {}
    for w in WS:
        pt = slope(done, [math.log2(ratios[w][b]) for b in done]); bs = []
        for _ in range(200):
            ys = []
            for b in done:
                ix = [brng.randrange(reps) for _ in range(reps)]
                ci = [cost(*rec[b]["ic"][k], w) for k in ix]
                cr = [cost(*rec[b][nm][k], w) for nm in ("rhoA", "rhoB") for k in ix]
                ys.append(math.log2(med(ci) / med(cr)))
            bs.append(slope(done, ys))
        bs.sort(); sl[w] = (pt, bs[5], bs[194])
    yobs = sum(h for h, _ in yo) / max(1, len(yo))
    yexp = sum(min(1.0, 2 * t) for _, t in yo) / max(1, len(yo))
    yield_ratio = yobs / yexp if yexp else float("nan")
    big = done[-1]
    mi0 = med([cost(*x, 0) for x in rec[big]["ic"]]); mi1 = med([cost(*x, 1) for x in rec[big]["ic"]])
    fr = med([cost(*x, 0.1) for x in rec[big]["icfree"]]) / med([cost(*x, 0.1) for x in rec[big]["ic"]])
    ctrl = {"correct_all_runs": bool(ok_all),
            "AA_rho_vs_rho_median_log2": med(aa), "AA_pass": abs(med(aa)) <= 0.3 or smoke,
            "null_yield_obs_over_model": yield_ratio, "null_pass": 0.3 <= yield_ratio <= 1.6 if yo else False,
            "pos_pricing_visible_w1_over_w0": mi1 / mi0, "pos_pass": mi1 / mi0 > 1.05,
            "pos_free_table_cost_fraction": fr}
    cok = ctrl["correct_all_runs"] and ctrl["AA_pass"] and ctrl["null_pass"] and ctrl["pos_pass"]
    last2 = done[-2:]
    if not cok: v = "inconclusive"
    elif any(all(ratios[w][b] < 1 for b in last2) for w in WS): v = "falsified"
    elif all(ratios[w][b] > 1 for w in WS for b in done) and all(sl[w][1] > 0 for w in WS): v = "consistent_with_prediction"
    else: v = "inconclusive"
    out.update(ratios_ic_over_rho={str(w): {str(b): round(r, 3) for b, r in ratios[w].items()} for w in WS},
               slope_log2ratio_vs_bits={str(w): [round(x, 4) for x in sl[w]] for w in WS},
               controls=ctrl, verdict=v, elapsed_note="deadline-guarded")
    print(json.dumps(out))


main()
