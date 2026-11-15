#!/usr/bin/env python3
"""LEAD-ICE-sat-xor-pdp-oracle: stdlib toy. SAT (binary-clause DPLL, one-hot) vs table
point-decomposition oracle, priced in group-addition equivalents (GAE). See PROTOCOL.md."""
import sys, json, random, time, statistics

def is_prime(n):
    return n > 1 and all(n % q for q in range(2, int(n ** .5) + 1))

class Curve:
    def __init__(s, p, a=1, b=3):
        s.p, s.a, s.b = p, a, b
        s.adds = 0
    def add(s, P, Q):
        s.adds += 1
        p = s.p
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3 * P[0] * P[0] + s.a) * pow(2 * P[1], -1, p) % p
        else:
            l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x = (l * l - P[0] - Q[0]) % p
        return (x, (l * (P[0] - x) - P[1]) % p)
    def neg(s, P):
        return None if P is None else (P[0], (-P[1]) % s.p)
    def points(s, X):
        sq = {}
        for y in range(s.p): sq.setdefault(y * y % s.p, []).append(y)
        out = []
        for x in range(s.p):
            r = (x ** 3 + s.a * x + s.b) % s.p
            for y in sq.get(r, []): out.append((x, y))
            if len(out) >= X: break
        return out

def table_oracle(C, FB, R):
    """B hash lookups: R - P_i in FB. Returns (i,j) or None; charged adds counted in C.adds."""
    idx = {P: i for i, P in enumerate(FB)}
    for i, P in enumerate(FB):
        Q = C.add(R, C.neg(P))
        if Q in idx: return (i, idx[Q])
    return None

class Sat:
    def __init__(s, nv, clauses):
        s.nv, s.cl = nv, clauses
        s.occ = {}
        for ci, c in enumerate(clauses):
            for l in c: s.occ.setdefault(l, []).append(ci)
        s.props = s.confl = s.dec = 0
    def solve(s):
        val = {}
        trail = []
        def lit_true(l): return val.get(abs(l)) == (l > 0)
        def assign(l, q):
            val[abs(l)] = l > 0; trail.append(abs(l)); q.append(l)
        def propagate(q):
            while q:
                l = q.pop()
                for ci in s.occ.get(-l, []):
                    c = s.cl[ci]; un = None; n = 0; sat = False
                    for m in c:
                        v = val.get(abs(m))
                        if v is None: n += 1; un = m
                        elif v == (m > 0): sat = True; break
                    if sat: continue
                    if n == 0: s.confl += 1; return False
                    if n == 1: s.props += 1; assign(un, q)
            return True
        def rec():
            q = []
            for v in range(1, s.nv + 1):
                if v not in val: break
            else: return True
            for l in (v, -v):
                s.dec += 1; mark = len(trail)
                q = []; assign(l, q)
                if propagate(q) and rec(): return True
                while len(trail) > mark: del val[trail.pop()]
            return False
        # initial units
        q = []
        ok = rec()
        return ok, dict(val)

def sat_oracle(C, FB, R, charge_enc):
    F = len(FB)
    a0, b0 = 0, F
    add0 = C.adds
    cl = [[a0 + i + 1 for i in range(F)], [b0 + j + 1 for j in range(F)]]
    for off in (a0, b0):
        for i in range(F):
            for k in range(i + 1, F): cl.append([-(off + i + 1), -(off + k + 1)])
    for i in range(F):
        Ri = C.add(R, C.neg(FB[i]))  # one add per i: R-P_i, then forbid j with P_j != R-P_i
        for j in range(F):
            if FB[j] != Ri: cl.append([-(a0 + i + 1), -(b0 + j + 1)])
    enc_adds = C.adds - add0
    S = Sat(2 * F, cl)
    ok, val = S.solve()
    if not ok: return None, S, enc_adds
    i = next(k for k in range(F) if val.get(a0 + k + 1))
    j = next(k for k in range(F) if val.get(b0 + k + 1))
    return (i, j), S, enc_adds

def gae_ns(C, FB, reps):
    t = time.perf_counter_ns(); n = 0
    for r in range(reps):
        P, Q = FB[r % len(FB)], FB[(r * 7 + 3) % len(FB)]
        if P != Q: C.add(P, Q); n += 1
    return (time.perf_counter_ns() - t) / max(n, 1)

def prop_ns(S_list, wall):
    t = sum(x[1] for x in S_list)
    w = sum(x[0].props + x[0].confl for x in S_list)
    return wall / max(w, 1) if False else t / max(w, 1)

def main():
    smoke = "--smoke" in sys.argv
    seed = 20261110
    rng = random.Random(seed)
    p = 1009 if smoke else 16381
    assert is_prime(p)
    Bs = [4] if smoke else [8, 16, 24, 32]   # x-values -> |FB| ~ 2*B
    T = 4 if smoke else 24
    C = Curve(p)
    allpts = C.points(10 ** 9)
    res, ratio_free = [], []
    mism = 0
    for B in Bs:
        FBx = sorted(set(P[0] for P in allpts))[:B]
        FB = []
        for P in allpts:
            if P[0] in FBx: FB.append(P)
        FB = FB[:2 * B]
        F = len(FB)
        # table oracle cost in adds (per query) and SAT work
        S_free, S_naive, S_prop, yes = [], [], [], 0
        ns_ga = gae_ns(C, allpts, 2000 if smoke else 20000)
        ns_ga2 = gae_ns(C, allpts, 2000 if smoke else 20000)  # A/A
        tw = 0.0; sw = 0; svals = []
        for t in range(T):
            if t % 2 == 0:  # planted yes
                R = C.add(rng.choice(FB), rng.choice(FB))
            else:  # random curve point (null, may or may not decompose)
                R = rng.choice(allpts)
            c0 = C.adds
            truth = table_oracle(C, FB, R)
            tcost = C.adds - c0
            t0 = time.perf_counter_ns()
            ans, S, enc = sat_oracle(C, FB, R, False)
            el = time.perf_counter_ns() - t0
            if (truth is None) != (ans is None): mism += 1
            if ans is not None and C.add(FB[ans[0]], FB[ans[1]]) != R: mism += 1
            if R is not None and truth is not None: yes += 1
            work = S.props + S.confl
            svals.append((S.props, S.confl, S.dec, enc, tcost, el, work))
        mp = statistics.median(v[0] for v in svals)
        mc = statistics.median(v[1] for v in svals)
        mt = statistics.median(v[4] for v in svals)
        me = statistics.median(v[3] for v in svals)
        # SAT time excl. encoding is not separated: use total wall, plus work-based pricing
        tot_work = sum(v[6] for v in svals); tot_wall = sum(v[5] for v in svals)
        ns_per_work = tot_wall / max(tot_work, 1)
        rw = ns_per_work / ns_ga  # GAE per propagation/conflict (python, wall incl. encoding: upper bound)
        row = {"B": B, "F": F, "yes_targets": yes, "median_props": mp, "median_confl": mc,
               "median_table_adds": mt, "median_encoding_adds": me,
               "ns_per_GA": round(ns_ga, 1), "ns_per_GA_AA": round(ns_ga2, 1),
               "AA_rel_diff": round(abs(ns_ga - ns_ga2) / ns_ga, 3),
               "measured_GAE_per_prop_upper": round(rw, 3)}
        row["S_free_enc"] = {}
        row["S_naive_enc"] = {}
        for w in (0.0, 0.1, 1.0):
            row["S_free_enc"][str(w)] = round((w * (mp + mc)) / max(mt, 1), 3)
            row["S_naive_enc"][str(w)] = round((me + w * (mp + mc)) / max(mt, 1), 3)
        res.append(row)
    # decision: SAT loses (S>1) with free encoding at w=1 and naive encoding at all w, for every B
    loses_w1 = all(r["S_free_enc"]["1.0"] > 1 for r in res)
    loses_naive = all(r["S_naive_enc"][k] > 1 for r in res for k in r["S_naive_enc"])
    wins_w01 = any(r["S_free_enc"]["0.1"] < 1 for r in res)
    aa_ok = all(r["AA_rel_diff"] < 0.25 for r in res)
    pos_ok = mism == 0
    if not (pos_ok and aa_ok):
        verdict = "inconclusive"
    elif loses_w1 and loses_naive:
        verdict = "consistent_with_prediction"
    elif any(r["S_free_enc"]["1.0"] <= 1 for r in res):
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICE-sat-xor-pdp-oracle", "smoke": smoke, "seed": seed,
        "p": p, "curve": "y^2=x^3+x+3", "targets_per_B": T, "rows": res,
        "controls": {"oracle_disagreements_sat_vs_table": mism, "positive_control_ok": pos_ok,
                     "AA_ok": aa_ok, "null": "random curve points, table oracle decides truth"},
        "statistic": "S = SAT_GAE / table_adds, median over targets",
        "free_enc_wins_at_w0.1": wins_w01,
        "verdict": verdict}))
main()
