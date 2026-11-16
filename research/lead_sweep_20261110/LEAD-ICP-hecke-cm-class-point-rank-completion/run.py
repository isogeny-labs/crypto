#!/usr/bin/env python3
"""Toy: marginal rank of Hecke/eigenvalue relations over a pair-collision control. stdlib only."""
import json, random, sys

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def prime_1mod4(lo):
    n = lo | 1
    while not (is_prime(n) and n % 4 == 1): n += 2
    return n

def sqrt_m1(n, rng):
    while True:
        a = rng.randrange(2, n - 1)
        t = pow(a, (n - 1) // 4, n)
        if t * t % n == n - 1: return t

def rank(rows, m, n):
    """rank mod prime n of integer rows (dict col->coef)."""
    piv = {}
    r = 0
    for row in rows:
        v = [0] * m
        for c, a in row.items(): v[c] = a % n
        for c in range(m):
            if v[c] == 0: continue
            if c in piv:
                p = piv[c]; f = v[c]
                v = [(x - f * y) % n for x, y in zip(v, p)]
            else:
                inv = pow(v[c], -1, n)
                v = [x * inv % n for x in v]
                piv[c] = v; r += 1
                break
    return r

def build(n, m, lam, rng):
    k = m // 4
    seeds = [rng.randrange(1, n) for _ in range(k)]
    logs = []
    for s in seeds:
        x = s
        for _ in range(4):
            logs.append(x); x = x * lam % n
    return logs  # index 4a+t holds lam^t * seed_a

def pair_rows(logs, m, n):
    buckets = {}
    for i in range(m):
        for j in range(i + 1, m):
            buckets.setdefault((logs[i] + logs[j]) % n, []).append((i, j))
    rows = []
    for ps in buckets.values():
        for (i, j), (k, l) in zip(ps, ps[1:]):
            row = {}
            for c, a in ((i, 1), (j, 1), (k, -1), (l, -1)): row[c] = row.get(c, 0) + a
            row = {c: a for c, a in row.items() if a}
            if row: rows.append(row)
    return rows

def hecke_rows(m, lam):
    rows = []
    for a in range(m // 4):
        for t in range(3):  # x_{t+1} = lam * x_t
            rows.append({4 * a + t + 1: 1, 4 * a + t: -lam})
    return rows

def valid(rows, logs, n):
    return all(sum(a * logs[c] for c, a in r.items()) % n == 0 for r in rows)

def cell(n, m, seed):
    rng = random.Random(seed)
    lam = sqrt_m1(n, rng)
    logs = build(n, m, lam, rng)
    # A/A: second independent pair collector = same collector on a disjoint sub-order of pairs is
    # deterministic, so A/A uses an independently seeded instance of the same cell.
    logs2 = build(n, m, lam, random.Random(seed + 7919))
    pr = pair_rows(logs, m, n); pr2 = pair_rows(logs2, m, n)
    hr = hecke_rows(m, lam)
    wrong = hecke_rows(m, (lam * 3 + 1) % n)  # non-eigenvalue negative control
    rp = rank(pr, m, n); rp2 = rank(pr2, m, n)
    ru = rank(pr + hr, m, n)
    return dict(n=n, m=m, pair_rank=rp, pair_rank_AA=rp2, union_rank=ru, marginal=ru - rp,
                hecke_alone=rank(hr, m, n), collisions=len(pr), pair_free=m - rp,
                hecke_valid=valid(hr, logs, n), wrong_valid=valid(wrong, logs, n))

def main():
    smoke = "--smoke" in sys.argv
    if smoke: cells = [(prime_1mod4(1 << 8), 16)]; reps = 2
    else:
        cells = [(prime_1mod4(1 << b), m) for b in (12, 14, 16, 18, 20) for m in (16, 32, 64)]; reps = 20
    out = []
    for ci, (n, m) in enumerate(cells):
        rs = [cell(n, m, 1000003 * ci + r) for r in range(reps)]
        out.append(dict(n=n, m=m, mean_marginal=sum(r["marginal"] for r in rs) / reps,
                        max_marginal=max(r["marginal"] for r in rs),
                        mean_pair_free=sum(r["pair_free"] for r in rs) / reps,
                        mean_collisions=sum(r["collisions"] for r in rs) / reps,
                        AA_mean_abs_diff=sum(abs(r["pair_rank"] - r["pair_rank_AA"]) for r in rs) / reps,
                        hecke_valid_all=all(r["hecke_valid"] for r in rs),
                        wrong_rejected_all=not any(r["wrong_valid"] for r in rs)))
    dense = [c for c in out if c["mean_collisions"] >= 3 * c["m"]]
    sparse = [c for c in out if c["mean_collisions"] < c["m"] / 2]
    stat = dict(dense_cells=len(dense), dense_max_marginal=max([c["max_marginal"] for c in dense], default=None),
                sparse_cells=len(sparse), sparse_max_mean_marginal=max([c["mean_marginal"] for c in sparse], default=None))
    controls_ok = all(c["hecke_valid_all"] and c["wrong_rejected_all"] for c in out)
    if smoke or not dense or not sparse or not controls_ok: verdict = "inconclusive"
    elif stat["dense_max_marginal"] == 0 and stat["sparse_max_mean_marginal"] >= 1: verdict = "consistent_with_prediction"
    elif stat["dense_max_marginal"] > 0 or stat["sparse_max_mean_marginal"] < 1: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps(dict(lead="LEAD-ICP-hecke-cm-class-point-rank-completion", smoke=smoke, statistic=stat,
        controls=dict(negative="wrong-eigenvalue relations rejected by validity check", positive="sparse cells must show marginal>=1",
                      AA="independent re-instantiation of pair collector", controls_ok=controls_ok),
        seeds="cell index ci, rep r: seed 1000003*ci+r; A/A seed+7919", parameters=dict(orbit_size=4, reps=reps, cells=len(cells)),
        cells=out, verdict=verdict)))
main()
