#!/usr/bin/env python3
"""Toy relation-diversity protocol (stdlib only). See PROTOCOL.md."""
import json, random, sys, itertools

Q = 4093           # prime group order (~2^12) of the toy cyclic group (Z_Q, +)
K = 15             # factor-base size (matches the recorded 13/15, 9/15 ranks)

def vectors(maxl1):
    out = []
    def rec(i, left, cur):
        if i == K:
            out.append(tuple(cur)); return
        for c in range(-left, left + 1):
            cur.append(c); rec(i + 1, left - abs(c), cur); cur.pop()
    rec(0, maxl1, [])
    return out

def rank_mod(rows, q=Q):
    rows = [[x % q for x in r] for r in rows]; rk = 0
    for c in range(K):
        p = next((i for i in range(rk, len(rows)) if rows[i][c]), None)
        if p is None: continue
        rows[rk], rows[p] = rows[p], rows[rk]
        inv = pow(rows[rk][c], q - 2, q)
        rows[rk] = [x * inv % q for x in rows[rk]]
        for i in range(len(rows)):
            if i != rk and rows[i][c]:
                f = rows[i][c]; rows[i] = [(x - f * y) % q for x, y in zip(rows[i], rows[rk])]
        rk += 1
    return rk

def canon(v):
    for x in v:
        if x: return tuple(v) if x > 0 else tuple(-y for y in v)
    return tuple(v)

def generic_pool(rng, vecs):
    fb = [rng.randrange(1, Q) for _ in range(K)]
    seen = {}; pool = []
    for v in vecs:
        s = sum(a * b for a, b in zip(v, fb)) % Q
        if s in seen:
            d = tuple(a - b for a, b in zip(v, seen[s]))
            if any(d): pool.append(canon(d))
        else: seen[s] = v
    return pool

def draw(rng, pool, A):
    acc = [rng.choice(pool) for _ in range(A)]
    return len(set(acc)), rank_mod(list(set(acc)))

def capped(rng, pool, S, A):
    sub = rng.sample(sorted(set(pool)), S)      # witness generator with support S
    return draw(rng, sub, A)

def main():
    smoke = "--smoke" in sys.argv
    l1 = 3 if smoke else 4
    seeds = 3 if smoke else 100
    vecs = vectors(l1)
    recs = [(97, 13), (67, 9)]
    gen = []; aa = []; ctrl = {13: [], 9: []}; dbl = []
    for s in range(seeds):
        rng = random.Random(1000 + s); pool = sorted(set(generic_pool(rng, vecs)))
        if len(set(pool)) < 30: continue
        gen.append(draw(rng, pool, 97) + draw(rng, pool, 67))
        dbl.append(draw(rng, pool, 194)[0])
        rng2 = random.Random(5000 + s); pool2 = sorted(set(generic_pool(rng2, vecs)))   # A/A: independent seed stream
        if len(set(pool2)) >= 30: aa.append(draw(rng2, pool2, 97)[0])
        for S in (13, 9):
            ctrl[S].append(capped(rng, pool, S, 97))
    n = max(len(gen), 1)
    frac_div = sum(1 for g in gen if g[0] >= 0.9 * 97) / n
    frac_rank = sum(1 for g in gen if g[1] == K) / n
    mean = lambda x: sum(x) / max(len(x), 1)
    pos_ok = all(all(c[0] == S and c[1] <= S for c in ctrl[S]) for S in (13, 9)) and bool(ctrl[13])
    aa_gap = abs(mean(aa) - mean([g[0] for g in gen])) if aa else None
    gain_dbl = mean(dbl) - mean([g[0] for g in gen])
    if smoke or not pos_ok: verdict = "inconclusive"
    elif frac_div >= 0.95 and frac_rank >= 0.95 and gain_dbl >= 50 and aa_gap < 3: verdict = "consistent_with_prediction"
    elif frac_div < 0.8 or frac_rank < 0.8: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICP-prym-witness-relation-diversity", "smoke": smoke,
        "params": {"Q": Q, "K": K, "l1_max": l1, "seeds": seeds, "seed_base": 1000, "aa_seed_base": 5000, "recorded": recs},
        "n_valid_seeds": len(gen), "frac_generic_distinct_ge_0.9A": frac_div, "frac_generic_rank_eq_K": frac_rank,
        "mean_distinct_A97": mean([g[0] for g in gen]), "mean_rank_A97": mean([g[1] for g in gen]),
        "mean_gain_distinct_doubling_A": gain_dbl, "aa_mean_gap": aa_gap,
        "positive_control_capped_support_ok": pos_ok,
        "capped_rank_max": {str(S): max((c[1] for c in ctrl[S]), default=None) for S in ctrl},
        "verdict": verdict}))
main()
