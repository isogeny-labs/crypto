"""End-to-end toy ECDL via the paper's algorithm (m=2) on GF(2^5), r=11.
Contraction is bucket elimination, so this checks the construction only."""
import random, sys, time, json
from fractions import Fraction
from btn import Field, Curve, contract
from relation import build

seed = int(sys.argv[1]) if len(sys.argv) > 1 else 1
rng = random.Random(seed)
n = 5
F = Field(n, 0b100101); C = Curve(F)
pts = [(x, y) for x in range(1 << n) for y in range(1 << n) if C.on((x, y))]
r = 11
G = next(p for p in pts if C.mulk(4, p) is not None and C.mulk(44, p) is None)
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
    total = contract(net.nv, net.cons)
    if total == 0: return None, 0
    def count(prefix):
        c0 = list(net.cons)
        for v, b in prefix: c0.append(((v,), __import__('numpy').array([1 - b, b], dtype='int64')))
        return contract(net.nv, c0)
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
print(f"{len(rels)} relations verified against reference arithmetic in {time.time()-t0:.0f}s")

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
sys.exit(0 if ok else 1)
