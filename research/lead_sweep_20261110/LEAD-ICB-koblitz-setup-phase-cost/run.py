#!/usr/bin/env python3
"""Toy Koblitz (a=1, F_2 curve) index calculus: setup-phase share of S. Stdlib only."""
import json, random, sys, statistics

POLY = {9: 0x211, 11: 0x805, 13: 0x201B, 17: 0x20009}
WEIGHTS = (0.0, 0.1, 1.0)


class Field:
    def __init__(s, n):
        s.n = n; s.size = 1 << n
        s.exp = [0] * (2 * s.size); s.log = [0] * s.size
        x = 1
        for i in range(s.size - 1):
            s.exp[i] = x; s.log[x] = i
            x <<= 1
            if x >> n: x ^= POLY[n]
        assert x == 1, "polynomial not primitive"
        for i in range(s.size - 1, 2 * s.size): s.exp[i] = s.exp[i - (s.size - 1)]
        s.sq = {}
        for z in range(s.size):
            s.sq.setdefault(s.mul(z, z) ^ z, z)

    def mul(s, a, b):
        return 0 if a == 0 or b == 0 else s.exp[s.log[a] + s.log[b]]

    def inv(s, a):
        return s.exp[(s.size - 1) - s.log[a]]


class Curve:  # y^2+xy=x^3+x^2+1
    def __init__(s, n):
        s.F = Field(n); s.n = n; s.adds = 0; s.looks = 0

    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        F = s.F; x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            s.adds += 1
            l = x1 ^ F.mul(y1, F.inv(x1)); x3 = F.mul(l, l) ^ l ^ 1
            return (x3, F.mul(x1, x1) ^ F.mul(l ^ 1, x3))
        s.adds += 1
        l = F.mul(y1 ^ y2, F.inv(x1 ^ x2)); x3 = F.mul(l, l) ^ l ^ x1 ^ x2 ^ 1
        return (x3, F.mul(l, x1 ^ x3) ^ x3 ^ y1)

    def neg(s, P):
        return None if P is None else (P[0], P[1] ^ P[0])

    def mul(s, k, P):
        R = None
        for b in bin(k)[2:]:
            R = s.add(R, R)
            if b == '1': R = s.add(R, P)
        return R

    def point(s, x):
        F = s.F
        if x == 0: return (0, 1)
        c = x ^ 1 ^ F.inv(F.mul(x, x))
        z = F.sq.get(c)
        return None if z is None else (x, F.mul(x, z))


def order(n):
    V0, V1 = 2, 1
    for _ in range(n - 1): V0, V1 = V1, V1 - 2 * V0
    return (1 << n) + 1 - V1


def largest_prime_factor(N):
    f, m, p = 1, N, 2
    while p * p <= m:
        while m % p == 0: f = p; m //= p
        p += 1
    return m if m > 1 else f


def setup_phase(C, l, r, h, P, rng):
    """Build FB, sums table, relations, rank finder + elimination. Returns logs dict, counters."""
    adds0 = C.adds; look = 0; mm = 0
    FB = []
    for x in range(1, 1 << l):
        pt = C.point(x)
        if pt is None: continue
        pt = C.mul(h, pt) if h > 1 else pt
        if pt is None: continue
        FB.append(pt)
    # reps must have distinct x
    seen = {}; FB = [p for p in FB if not (p[0] in seen or seen.setdefault(p[0], 1) is None)]
    u = len(FB); idx = {p[0]: i for i, p in enumerate(FB)}
    table = {}
    for i in range(u):
        for j in range(i, u):
            for sj in (1, -1):
                Q = FB[j] if sj == 1 else C.neg(FB[j])
                S = C.add(FB[i], Q)
                if S is None: continue
                table.setdefault(S[0], []).append((S[1], i, j, sj))
    table_adds = C.adds - adds0
    rows = {}  # pivot col -> dense row (len u+1)
    rank = 0; rels = 0; tries = 0
    a = 0; R = None; steps = 0
    while rank < u and tries < 400 * u + 4000:
        if R is None or steps >= 64:
            a = rng.randrange(1, r); R = C.mul(a, P); steps = 0
        else:
            a = (a + 1) % r; R = C.add(R, P); steps += 1
        tries += 1; look += 1
        if R is None or R[0] not in table: continue
        for (y, i, j, sj) in table[R[0]]:
            sgn = 1 if y == R[1] else -1  # R = sgn*(Fi+sj*Fj)
            row = [0] * (u + 1)
            row[i] = (row[i] + sgn) % r; row[j] = (row[j] + sgn * sj) % r
            row[u] = a % r
            rels += 1
            for c in range(u):  # reduce by pivots
                if row[c] and c in rows:
                    f = row[c]; pr = rows[c]
                    for t in range(c, u + 1):
                        if pr[t]: row[t] = (row[t] - f * pr[t]) % r; mm += 1
            c = next((c for c in range(u) if row[c]), None)
            if c is None: continue
            iv = pow(row[c], -1, r); mm += 1
            row = [(v * iv) % r for v in row]; mm += u + 1
            rows[c] = row; rank += 1
            if rank == u: break
    ok = rank == u
    logs = None
    if ok:  # back-substitute
        logs = [0] * u
        for c in sorted(rows, reverse=True):
            pr = rows[c]; v = pr[u]
            for t in range(c + 1, u):
                if pr[t]: v = (v - pr[t] * logs[t]) % r; mm += 1
            logs[c] = v
    return dict(ok=ok, FB=FB, idx=idx, table=table, logs=logs, u=u, adds=C.adds - adds0,
                table_adds=table_adds, lookups=look, mulmods=mm, rank=rank, relations=rels, tries=tries)


def online(C, S, r, P, Q, rng):
    adds0 = C.adds; look = 0
    FB, table, logs = S['FB'], S['table'], S['logs']
    s = a = 0; R = None; steps = 0
    for _ in range(100000):
        if R is None or steps >= 64:
            s = rng.randrange(1, r); a = rng.randrange(r)
            R = C.add(C.mul(s, Q), C.mul(a, P)); steps = 0
        else:
            a = (a + 1) % r; R = C.add(R, P); steps += 1
        look += 1
        if R is None or R[0] not in table: continue
        y, i, j, sj = table[R[0]][0]
        sgn = 1 if y == R[1] else -1
        val = sgn * (logs[i] + sj * logs[j]) - a
        k = val * pow(s, -1, r) % r
        return k, C.adds - adds0, look, 2 + 0
    return None, C.adds - adds0, look, 0


def trial(n, l, seed, targets):
    rng = random.Random(seed)
    N = order(n); r = largest_prime_factor(N); h = N // r
    C = Curve(n)
    while True:
        pt = C.point(rng.randrange(1, 1 << n))
        if pt is None: continue
        P = C.mul(h, pt) if h > 1 else pt
        if P is not None: break
    C.adds = 0
    S = setup_phase(C, l, r, h, P, rng)
    if not S['ok']:
        return dict(n=n, seed=seed, ok=False)
    on_adds = on_look = 0; correct = 0; wrong_ctrl = 0
    for _ in range(targets):
        k0 = rng.randrange(1, r); Q = C.mul(k0, P); C.adds = 0
        k, ad, lk, _ = online(C, S, r, P, Q, rng)
        on_adds += ad; on_look += lk
        correct += (k == k0)
        save = S['logs'][0]; S['logs'][0] = (save + 1) % r  # null control: corrupted logs
        k2, _, _, _ = online(C, S, r, P, Q, rng); S['logs'][0] = save
        wrong_ctrl += (k2 != k0)
    set_adds = S['adds']; set_other = S['lookups'] + S['mulmods']
    on_per_adds = on_adds / targets; on_per_other = on_look / targets
    fr = {}; kstar = {}
    for w in WEIGHTS:
        setc = set_adds + w * set_other; onc = on_per_adds + w * on_per_other
        fr[str(w)] = setc / (setc + onc); kstar[str(w)] = setc / onc
    return dict(n=n, l=l, seed=seed, ok=True, r=r, h=h, u=S['u'], rank=S['rank'], relations=S['relations'],
                setup_adds=set_adds, table_adds=S['table_adds'], setup_lookups=S['lookups'],
                setup_mulmods=S['mulmods'], online_adds_per_target=on_per_adds,
                online_lookups_per_target=on_per_other, correct=correct, targets=targets,
                null_wrong=wrong_ctrl, f_setup=fr, breakeven_targets=kstar)


def main():
    smoke = '--smoke' in sys.argv
    cells = [(9, 4)] if smoke else [(11, 5), (13, 6), (17, 8)]
    seeds_main = [101, 102] if smoke else [101, 102, 103, 104, 105]
    seeds_aa = [201, 202] if smoke else [201, 202, 203, 204, 205]
    T = 3 if smoke else 20
    out = dict(lead="LEAD-ICB-koblitz-setup-phase-cost", smoke=smoke, weights=list(WEIGHTS),
               seeds_main=seeds_main, seeds_aa=seeds_aa, targets_per_seed=T, cells=[])
    falsified = False; incon = False
    for n, l in cells:
        A = [trial(n, l, s, T) for s in seeds_main]; B = [trial(n, l, s, T) for s in seeds_aa]
        cell = dict(n=n, l=l, arms={}, trials=A + B)
        good = all(t['ok'] for t in A + B)
        pos = good and all(t['correct'] == t['targets'] for t in A + B)
        nullc = good and all(t['null_wrong'] >= t['targets'] * 0.5 for t in A + B)
        cell['positive_control_all_targets_recovered'] = pos
        cell['null_control_corrupted_logs_detected'] = nullc
        if not (good and pos and nullc):
            incon = True; cell['status'] = 'controls_failed'; out['cells'].append(cell); continue
        med = {}
        for w in WEIGHTS:
            k = str(w)
            ma = statistics.median(t['f_setup'][k] for t in A)
            mb = statistics.median(t['f_setup'][k] for t in B)
            ks = statistics.median(t['breakeven_targets'][k] for t in A + B)
            med[k] = dict(median_f_main=ma, median_f_aa=mb, aa_abs_diff=abs(ma - mb), median_breakeven=ks)
            if abs(ma - mb) > 0.02: incon = True
            if ma < 0.90: falsified = True
            if ma < 0.95 or ks < 10: incon = True
        cell['by_weight'] = med; out['cells'].append(cell)
    out['verdict'] = 'falsified' if falsified else ('inconclusive' if incon else 'consistent_with_prediction')
    if smoke and out['verdict'] == 'falsified': pass
    print(json.dumps(out))


if __name__ == '__main__':
    main()
