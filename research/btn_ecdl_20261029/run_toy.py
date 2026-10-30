"""End-to-end toy ECDL via the paper's algorithm (m=2) on y^2+xy=x^3+1.
n=5: GF(2^5), r=11; n=7: GF(2^7), r=29 (group orders 4r).  Contraction is
bucket elimination (CPU) or gpu_contract (CuPy, batched cutset), so this
checks the construction, not the claimed subexponential speed.

  python3 run_toy.py 1                                  # n=5, CPU (original run)
  python3 run_toy.py 1 --n 7 --backend gpu --cut 7      # n=7 on a GPU
"""
import argparse, random, sys, time, json
from btn import Field, Curve, contract
from relation import build

ap = argparse.ArgumentParser()
ap.add_argument("seed", type=int, nargs="?", default=1)
ap.add_argument("--n", type=int, default=5, choices=[5, 7])
ap.add_argument("--backend", default="cpu", choices=["cpu", "gpu", "numpy"],
                help="cpu: btn.contract; gpu: gpu_contract on CuPy; numpy: gpu_contract on NumPy")
ap.add_argument("--cut", type=int, default=0, help="batch the first CUT bits of x1 (gpu/numpy backends)")
ap.add_argument("--mem-gb", type=float, default=16.0)
ap.add_argument("--json", default="", help="write a result record here")
args = ap.parse_args()
seed = args.seed
rng = random.Random(seed)
n = args.n
F = Field(n, {5: 0b100101, 7: (1 << 7) | 3}[n]); C = Curve(F)
pts = [(x, y) for x in range(1 << n) for y in range(1 << n) if C.on((x, y))]
r = {5: 11, 7: 29}[n]
order = 4 * r
assert len(pts) + 1 == order
if args.backend == "cpu":
    count_fn = lambda nv, cons, iface: contract(nv, cons)
else:
    import gpu_contract
    xp, bname = gpu_contract.backend("cupy" if args.backend == "gpu" else "numpy")
    print("contraction backend:", bname, flush=True)
    count_fn = lambda nv, cons, iface: gpu_contract.contract_gpu(nv, cons, iface[:args.cut], xp, args.mem_gb)
G = next(p for p in pts if C.mulk(4, p) is not None and C.mulk(order, p) is None)
P = C.mulk(4, G)
assert C.mulk(r, P) is None
sub = [None]; Z = P
for _ in range(r - 1): sub.append(Z); Z = C.add(Z, P)
xi = rng.randrange(1, r)
Q = C.mulk(xi, P)
subpts = [p for p in sub if p is not None]
xs = sorted({p[0] for p in subpts})
tabs = []
for _ in range(2):
    keep = {x for x in xs if rng.random() < 0.8} or {xs[0]}
    tabs.append([1 if x in keep else 0 for x in range(1 << n)])
FB = [[p for p in subpts if tabs[i][p[0]]] for i in range(2)]
cols = {}
for i in range(2):
    for p in FB[i]: cols[(i, p)] = len(cols)
N = len(cols)
print(f"seed {seed} xi(secret)={xi} |F1|={len(FB[0])} |F2|={len(FB[1])} N={N}")

def sample(R):
    net, iface, _, _ = build(F, R, tabs)
    total = count_fn(net.nv, net.cons, iface)
    if total == 0: return None, 0
    def count(prefix):
        c0 = list(net.cons)
        for v, b in prefix: c0.append(((v,), __import__('numpy').array([1 - b, b], dtype='int64')))
        return count_fn(net.nv, c0, iface)
    prefix, cur = [], total
    for v in iface:
        n0 = count(prefix + [(v, 0)])
        b = 0 if rng.random() < n0 / cur else 1
        cur = n0 if b == 0 else cur - n0
        prefix.append((v, b))
    val = {v: b for v, b in prefix}
    w = lambda ws: sum(val[v] << i for i, v in enumerate(ws))
    A1 = (w(iface[0:n]), w(iface[n:2*n])); A2 = (w(iface[2*n:3*n]), w(iface[3*n:4*n]))
    return (A1, A2), total

rels, t0 = [], time.time()
while len(rels) < N + 1:
    u, v = rng.randrange(r), rng.randrange(r)
    R = C.add(C.mulk(u, P), C.mulk(v, Q)) if (u or v) else None
    if R is None: continue
    res, total = sample(R)
    if res is None: continue
    A1, A2 = res
    assert C.add(A1, A2) == R and tabs[0][A1[0]] and tabs[1][A2[0]]
    rels.append((u, v, cols[(0, A1)], cols[(1, A2)]))
t_rel = time.time() - t0
print(f"{len(rels)} relations verified against reference arithmetic in {t_rel:.0f}s", flush=True)

# kernel of the (N+1) x N matrix over Z_r: find c != 0 with sum c_i e_i = 0
def kernel(rels):
    m = len(rels)
    rows = []
    for i, (u, v, a, b) in enumerate(rels):
        row = [0] * N + [1 if j == i else 0 for j in range(m)]
        row[a] = (row[a] + 1) % r; row[b] = (row[b] + 1) % r
        rows.append(row)
    piv = 0
    for c in range(N):
        pr = next((i for i in range(piv, m) if rows[i][c]), None)
        if pr is None: continue
        rows[piv], rows[pr] = rows[pr], rows[piv]
        inv = pow(rows[piv][c], -1, r)
        rows[piv] = [x * inv % r for x in rows[piv]]
        for i in range(m):
            if i != piv and rows[i][c]:
                f = rows[i][c]; rows[i] = [(x - f * y) % r for x, y in zip(rows[i], rows[piv])]
        piv += 1
    return [row[N:] for row in rows[piv:]]
ok = False
for c in kernel(rels):
    U = sum(ci * rl[0] for ci, rl in zip(c, rels)) % r
    V = sum(ci * rl[1] for ci, rl in zip(c, rels)) % r
    if V:
        got = (-U * pow(V, -1, r)) % r
        print("recovered xi =", got, "check Q == [xi]P:", C.mulk(got, P) == Q)
        ok = C.mulk(got, P) == Q; break
else:
    print("all kernel vectors had V = 0 (re-run with another seed)")
if args.json:
    json.dump(dict(n=n, r=r, seed=seed, backend=args.backend, cut=args.cut, N=N, relations=len(rels),
                   relation_seconds=round(t_rel, 1), secret=xi, recovered=ok), open(args.json, "w"), indent=1)
sys.exit(0 if ok else 1)
