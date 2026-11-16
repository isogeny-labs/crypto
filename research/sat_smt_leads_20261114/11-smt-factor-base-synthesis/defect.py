import random, json, itertools, time
from fb import *
MASTER = 2026111411
out = {'cells': [], 'exhaustive': [], 'control': []}
def stats(F, vs, xR):
    nm2, r2, nm3, r3 = macaulay_m2(F, vs, xR)
    return dict(w=prod_dim(F, vs), def2=nm2 - r2, rank2=r2, def3=nm3 - r3, rank3=r3)
def hill(F, l, rng, guided, moves=2000):
    vs = rand_subspace(F.n, l, rng); cur = prod_dim(F, vs); start = cur
    for _ in range(moves):
        i = rng.randrange(l)
        if guided:
            a, b, c = (rng.choice(vs) for _ in range(3))
            cand = F.mul(a, b) if rng.random() < 0.5 else F.mul(F.mul(a, b), F.inv(c))
        else: cand = rng.randrange(1, 1 << F.n)
        nv = vs[:i] + [cand] + vs[i + 1:]
        if rank(nv) < l: continue
        d = prod_dim(F, nv)
        if d <= cur: vs, cur = nv, d
    return start, cur
for (l, n) in ((3, 11), (4, 13), (5, 17), (6, 19), (7, 23)):
    rng = random.Random(MASTER * 1000 + 37 * l + n + 7); F = Field(n); xR = F.rand_point(rng)[0]
    cell = dict(l=l, n=n, xR=xR, lower_bound_w=2 * l - 1, generic_w=min(n, l * (l + 1) // 2))
    cell['random'] = [stats(F, rand_subspace(n, l, rng), xR) for _ in range(200)]
    cell['poly'] = [stats(F, poly_span(F, l, rng), xR) for _ in range(8)]
    cell['frob'] = [stats(F, frob_span(F, l, rng), xR) for _ in range(8)]
    cell['trace0'] = [stats(F, trace0_span(F, l, rng), xR) for _ in range(8)]
    cell['hill_blind'] = [hill(F, l, rng, False) for _ in range(5)]
    cell['hill_guided'] = [hill(F, l, rng, True) for _ in range(5)]
    out['cells'].append(cell); print('defect cell', l, n, time.strftime('%H:%M:%S'), flush=True)
# exhaustive: all 3-dim V in F_2^7; all 3-dim V containing 1 in F_2^11
for n, with1 in ((7, False), (11, True)):
    F = Field(n); hist = {}; seen = set()
    if with1:
        for a in range(2, 1 << n):
            for b in range(a + 1, 1 << n):
                key = tuple(span_basis([1, a, b])[0])
                if len(key) < 3 or key in seen: continue
                seen.add(key); d = prod_dim(F, list(key)); hist[d] = hist.get(d, 0) + 1
    else:
        for a, b, c in itertools.combinations(range(1, 1 << n), 3):
            if a ^ b == c: continue
            key = tuple(span_basis([a, b, c])[0])
            if key in seen: continue
            seen.add(key); d = prod_dim(F, list(key)); hist[d] = hist.get(d, 0) + 1
    out['exhaustive'].append(dict(n=n, l=3, contains_one=with1, count=len(seen), w_hist=hist))
    print('exhaustive', n, hist, flush=True)
F = Field(15); rng = random.Random(MASTER)
for k in (3, 5):
    sub = []
    for x in range(1 << 15):
        y = x
        for _ in range(k): y = F.sq(y)
        if y == x: sub.append(x)
    vs, _ = span_basis(sub)
    rnd = sorted(prod_dim(F, rand_subspace(15, k, rng)) for _ in range(200))
    out['control'].append(dict(n=15, l=k, subfield_w=prod_dim(F, vs), random_w_min=rnd[0], random_w_median=rnd[100]))
json.dump(out, open('defect.json', 'w'))
