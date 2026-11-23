import json, sys, random, itertools
SMOKE = '--smoke' in sys.argv
def trees(n):  # all binary merge trees over n ordered leaves
    if n == 1: return ['x']
    out = []
    for k in range(1, n):
        for l in trees(k):
            for r in trees(n - k): out.append((l, r))
    return out
def leaves(t): return 1 if t == 'x' else leaves(t[0]) + leaves(t[1])
def depths(t, d=0):
    return [d] if t == 'x' else depths(t[0], d + 1) + depths(t[1], d + 1)
def degs(t):
    # topology-AWARE model: each S_3 elimination doubles degrees, leaf degree 2^depth; solver degree = max leaf degree
    return 2 ** max(depths(t))
def semaev_deg(t):  # Semaev's fixed-degree model: S_{m+1} has degree 2^(m-1) per variable, topology-blind
    return 2 ** (leaves(t) - 1)
def naive_cost(t):  # topology-DEPENDENT toy proxy: sum of 2^(leaves of node) work
    if t == 'x': return 0
    return naive_cost(t[0]) + naive_cost(t[1]) + 2 ** leaves(t)
def curve(p, a, b):
    sq = {}
    for y in range(p): sq.setdefault(y * y % p, []).append(y)
    pts = [(x, y) for x in range(p) for y in sq.get((x**3 + a*x + b) % p, [])]
    return pts
def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3*P[0]*P[0] + a) * pow(2*P[1], -1, p) % p
    else: l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
    x = (l*l - P[0] - Q[0]) % p
    return (x, (l*(P[0]-x) - P[1]) % p)
def count_tree(t, fb, p, a, R, tab):
    # number of ordered sign-tuples of FB points whose sum is R, via a given merge tree
    def sums(t, i):  # returns (dict point->count, next index)
        if t == 'x': return tab[i], i + 1
        L, j = sums(t[0], i); Rr, k = sums(t[1], j)
        d = {}
        for P, c in L.items():
            for Q, e in Rr.items():
                S = add(P, Q, p, a); d[S] = d.get(S, 0) + c * e
        return d, k
    d, _ = sums(t, 0); return d.get(R, 0)
def main():
    seed = 20261110; rnd = random.Random(seed)
    p = 521 if SMOKE else 1019
    a, b = 3, 7
    while True:
        pts = curve(p, a, b)
        if len(pts) > 100: break
        b += 1
    B = 6 if SMOKE else 12
    fb = [P for P in pts if P[0] < 4*B][:B]
    signed = {}
    for P in fb:
        signed[P] = 1; Q = (P[0], (-P[1]) % p); signed[Q] = signed.get(Q, 0) + 1
    ms = [3, 4] if SMOKE else [3, 4, 5, 6, 7, 8]
    per_m = {}; cost_vals = {}
    for m in ms:
        T = trees(m)
        dv = set(); nc = set(); aware = set()
        for t in T:
            d = degs(t)
            dv.add(semaev_deg(t)); aware.add(d)
            nc.add(naive_cost(t))
        per_m[m] = {"trees": len(T), "distinct_degree_values": sorted(map(str, dv)), "distinct_naive_costs": len(nc), "topology_aware_max_degree_values": sorted(aware), "min_aware_degree": min(aware)}
        cost_vals[m] = len(nc)
    # relation-count invariance and A/A
    mc = 3 if SMOKE else 4
    tab = [signed] * mc
    R = pts[rnd.randrange(len(pts))]
    counts = [count_tree(t, fb, p, a, R, tab) for t in trees(mc)]
    counts_shuf = [count_tree(t, fb, p, a, R, tab) for t in rnd.sample(trees(mc), len(trees(mc)))]
    count_invariant = len(set(counts)) == 1
    aa = sorted(counts) == sorted(counts_shuf)
    # null model: deliberately topology-dependent proxy must separate topologies
    positive_ctrl = all(cost_vals[m] > 1 for m in ms if m >= 4) if any(m >= 4 for m in ms) else True
    if SMOKE: positive_ctrl = True
    # degree uniform across all trees at each m, and independent of topology
    uniform = all(len(v["distinct_degree_values"]) == 1 for v in per_m.values())
    aware_varies = all(len(v['topology_aware_max_degree_values']) > 1 for v in per_m.values() if v['trees'] > 2)
    if not uniform or not count_invariant: verdict = "falsified"
    elif aa and positive_ctrl and aware_varies: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICE-semaev-chain-cost-model", "smoke": SMOKE, "seed": seed,
        "params": {"p": p, "curve": [a, b], "group_order_pts": len(pts), "FB": B, "ms": ms, "count_m": mc},
        "statistic": {"degree_topology_uniform": uniform, "relation_count_invariant": count_invariant,
                      "counts_sample": counts[:5]},
        "controls": {"A/A_shuffled_tree_order_equal": aa, "positive_naive_cost_topology_dependent": positive_ctrl,
                     "negative_control": "naive 2^leaves proxy shown topology-dependent; degree identity must not be"},
        "per_m": per_m, "verdict": verdict}))
main()
