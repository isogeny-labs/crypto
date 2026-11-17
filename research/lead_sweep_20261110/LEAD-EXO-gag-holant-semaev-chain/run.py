#!/usr/bin/env python3
"""Toy falsifier C for LEAD-EXO-gag-holant-semaev-chain. stdlib only, deterministic."""
import json, random, sys, statistics

RP = 1000003  # rank modulus


def rank_mod(rows):
    rows = [r[:] for r in rows]
    rk, ncol = 0, len(rows[0]) if rows else 0
    for c in range(ncol):
        piv = None
        for i in range(rk, len(rows)):
            if rows[i][c] % RP:
                piv = i
                break
        if piv is None:
            continue
        rows[rk], rows[piv] = rows[piv], rows[rk]
        inv = pow(rows[rk][c], -1, RP)
        pr = [(v * inv) % RP for v in rows[rk]]
        rows[rk] = pr
        for i in range(rk + 1, len(rows)):
            f = rows[i][c]
            if f:
                rows[i] = [(a - f * b) % RP for a, b in zip(rows[i], pr)]
        rk += 1
    return rk


def is_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def make_curve(p, rng):
    """Random curve over F_p with prime group order; returns (a, pts, N, G)."""
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0:
            continue
        pts = [(x, y) for x in range(p) for y in range(p) if (y * y - x ** 3 - a * x - b) % p == 0]
        N = len(pts) + 1
        if is_prime(N) and N > 20:
            return a, pts, N, pts[rng.randrange(len(pts))]


def add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0:
            return None
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)


def one_curve(p, k, seed):
    rng = random.Random(seed)
    a, pts, N, G = make_curve(p, rng)
    # DL class c = min(k, N-k); c=0 is infinity
    mult = [None]
    cur = None
    for i in range(1, N):
        cur = add(cur, G, a, p)
        mult.append(cur)
    cls_of = {None: 0}
    rep = {0: None}
    for c in range(1, (N - 1) // 2 + 1):
        cls_of[mult[c][0]] = c
        rep[c] = mult[c]
    ncls = len(rep)
    B = max(3, p // 8)
    fb = [P for P in pts if P[0] < B]
    M = {}
    for ca, P in rep.items():
        for R in fb:
            Q = add(P, R, a, p)
            cb = cls_of[None if Q is None else Q[0]]
            M[(ca, cb)] = M.get((ca, cb), 0) + 1
    xlab = {0: p}  # infinity label p; class c -> x coordinate
    for c in range(1, ncls):
        xlab[c] = rep[c][0]
    side = 1 << k
    assert p < side
    h = k // 2
    lo = (1 << (k - h))  # low part size
    hi = side // lo

    def stat(lab):
        mat = [[0] * (lo * lo) for _ in range(hi * hi)]
        for (ca, cb), v in M.items():
            i, j = lab[ca], lab[cb]
            mat[(i // lo) * hi + (j // lo)][(i % lo) * lo + (j % lo)] = v
        return rank_mod(mat)

    def rnd(s):
        r = random.Random(s)
        sl = r.sample(range(side), ncls)
        return {c: sl[c] for c in range(ncls)}

    rx = stat(xlab)
    rd = stat({c: c for c in range(ncls)})
    r1, r2 = stat(rnd(seed + 1)), stat(rnd(seed + 2))
    return dict(p=p, N=N, ncls=ncls, B=B, fb=len(fb), rank_x=rx, rank_dl=rd, rank_r1=r1, rank_r2=r2,
                ceiling=min(hi * hi, lo * lo))


def main():
    smoke = "--smoke" in sys.argv
    if smoke:
        plan, k = [(61, 6, 1)], 6
    else:
        plan, k = [(p, 8, 1000 + i) for i, p in enumerate([151, 163, 181, 199, 211, 223, 241, 251])], 8
    rows = [one_curve(p, kk, s) for p, kk, s in plan]
    null = [(r["rank_r1"] + r["rank_r2"]) / 2 for r in rows]
    stat_x = statistics.median(r["rank_x"] / n for r, n in zip(rows, null))
    pc = statistics.median(r["rank_dl"] / n for r, n in zip(rows, null))
    aa = statistics.median(r["rank_r1"] / r["rank_r2"] for r in rows)
    controls_ok = pc <= 0.7 and abs(aa - 1) <= 0.03
    if not controls_ok:
        verdict = "inconclusive"
    elif stat_x >= 0.95:
        verdict = "consistent_with_prediction"
    elif stat_x < 0.9:
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    print(json.dumps(dict(lead="LEAD-EXO-gag-holant-semaev-chain", smoke=smoke, bits=k,
                          seeds=[s for _, _, s in plan], rank_modulus=RP,
                          statistic_x_over_null_median=stat_x, positive_control_dl_over_null=pc,
                          aa_ratio_median=aa, controls_ok=controls_ok, per_curve=rows,
                          thresholds=dict(consistent=">=0.95", falsified="<0.9", pc="<=0.7", aa="|r-1|<=0.03"),
                          verdict=verdict, label="[pcc:unverified]")))


if __name__ == "__main__":
    main()
