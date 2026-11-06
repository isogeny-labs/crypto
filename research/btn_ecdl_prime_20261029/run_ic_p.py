"""End-to-end index calculus (m=2, hashed factor bases) on a prime-order curve
y^2=x^3+ax+b over F_p, with relations found by BTN contraction + Lemma-1
sampling.  Contraction: gpu_contract (NumPy or CuPy), optional x1 cutset.

  python3 run_ic_p.py --p 11 --seed 1 --json out.json
"""
import argparse, json, random, sys, time
import numpy as np
import gpu_contract
from curves import prime_order_curve
from relation_p import build_p

ap = argparse.ArgumentParser()
ap.add_argument("--p", type=int, required=True)
ap.add_argument("--seed", type=int, default=1)
ap.add_argument("--cut", type=int, default=0)
ap.add_argument("--keep", type=float, default=0.7, help="probability an x-value is in each factor base")
ap.add_argument("--backend", default="numpy", choices=["numpy", "cupy"])
ap.add_argument("--mem-gb", type=float, default=6.0)
ap.add_argument("--json", default="")
a = ap.parse_args()
xp, bname = gpu_contract.backend(a.backend)
rng = random.Random(a.seed)
C, r = prime_order_curve(a.p)
F = C.F; d = F.d
pts = C.points()
P = pts[0]
assert C.mulk(r, P) is None
xi = rng.randrange(1, r)
Q = C.mulk(xi, P)
tabs = []
for _ in range(2):
    keep = {x for x in range(a.p) if rng.random() < a.keep} or {pts[0][0]}
    tabs.append([1 if x in keep else 0 for x in range(a.p)])
FB = [[q for q in pts if tabs[i][q[0]]] for i in range(2)]
cols = {}
for i in range(2):
    for q in FB[i]: cols[(i, q)] = len(cols)
N = len(cols)
stats = dict(counts=0, count_seconds=0.0, width=0)

def count(nv, cons, iface):
    t = time.time()
    plan = gpu_contract.Plan(nv, cons, iface[:a.cut])
    if not plan.empty: stats["width"] = max(stats["width"], plan.width)
    c = gpu_contract.contract_gpu(nv, cons, iface[:a.cut], xp, a.mem_gb, plan=plan)
    stats["counts"] += 1; stats["count_seconds"] += time.time() - t
    return c

def sample(R):
    net, iface = build_p(C, R, tabs)
    total = count(net.nv, net.cons, iface)
    if total == 0: return None
    prefix, cur = [], total
    for v in iface:
        c0 = list(net.cons) + [((u,), np.array([1 - b, b], dtype=np.int64)) for u, b in prefix + [(v, 0)]]
        n0 = count(net.nv, c0, iface)
        b = 0 if rng.random() < n0 / cur else 1
        cur = n0 if b == 0 else cur - n0
        prefix.append((v, b))
    val = dict(prefix)
    w = lambda ws: sum(val[u] << i for i, u in enumerate(ws))
    k = len(iface) // 4
    return (w(iface[0:k]), w(iface[k:2*k])), (w(iface[2*k:3*k]), w(iface[3*k:4*k]))

print(f"p={a.p} d={d} curve a={C.a} b={C.b} order r={r} |F1|={len(FB[0])} |F2|={len(FB[1])} N={N} backend={bname}", flush=True)
rels, tries, t0 = [], 0, time.time()
while len(rels) < N + 1:
    u, v = rng.randrange(r), rng.randrange(r)
    R = C.add(C.mulk(u, P), C.mulk(v, Q))
    if R is None: continue
    tries += 1
    res = sample(R)
    if res is None: continue
    A1, A2 = res
    assert C.add(A1, A2) == R and tabs[0][A1[0]] and tabs[1][A2[0]], "relation failed reference check"
    rels.append((u, v, cols[(0, A1)], cols[(1, A2)]))
t_rel = time.time() - t0

def kernel(rels):
    m = len(rels); rows = []
    for i, (u, v, x, y) in enumerate(rels):
        row = [0] * N + [1 if j == i else 0 for j in range(m)]
        row[x] = (row[x] + 1) % r; row[y] = (row[y] + 1) % r
        rows.append(row)
    piv = 0
    for c in range(N):
        pr = next((i for i in range(piv, m) if rows[i][c]), None)
        if pr is None: continue
        rows[piv], rows[pr] = rows[pr], rows[piv]
        inv = pow(rows[piv][c], -1, r); rows[piv] = [x * inv % r for x in rows[piv]]
        for i in range(m):
            if i != piv and rows[i][c]:
                f = rows[i][c]; rows[i] = [(x - f * y) % r for x, y in zip(rows[i], rows[piv])]
        piv += 1
    return [row[N:] for row in rows[piv:]]

got = None
for c in kernel(rels):
    U = sum(ci * rl[0] for ci, rl in zip(c, rels)) % r
    V = sum(ci * rl[1] for ci, rl in zip(c, rels)) % r
    if V:
        got = (-U * pow(V, -1, r)) % r; break
ok = got is not None and C.mulk(got, P) == Q
rec = dict(p=a.p, d=d, a=C.a, b=C.b, r=r, seed=a.seed, cut=a.cut, backend=bname, N=N, relations=len(rels),
           targets_tried=tries, counts=stats["counts"], count_seconds=round(stats["count_seconds"], 2),
           seconds_per_count=round(stats["count_seconds"] / max(1, stats["counts"]), 4),
           width=stats["width"], relation_seconds=round(t_rel, 1), secret=xi, recovered=got, ok=ok)
print(json.dumps(rec), flush=True)
if a.json: json.dump(rec, open(a.json, "w"), indent=1)
sys.exit(0 if ok else 1)
