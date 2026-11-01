"""Treewidth estimate (min-fill / min-degree elimination) of the relation network."""
import sys, random
from btn import Field, simplify
from relation import build

polys = {5: 0b100101, 7: (1 << 7) | 3, 9: (1 << 9) | (1 << 4) | 1}
n = int(sys.argv[1]); mode = sys.argv[2] if len(sys.argv) > 2 else "fill"
F = Field(n, polys[n])
net, iface, _, _ = build(F, (3, 5), ([1] * (1 << n), [1] * (1 << n)))
import numpy as np
pins = int(sys.argv[3]) if len(sys.argv) > 3 else 0
for v in iface[:pins]: net.cons.append(((v,), np.array([1, 0])))
cons, _ = simplify(net.cons)
adj = {}
for sc, _ in cons:
    for v in sc: adj.setdefault(v, set()).update(set(sc) - {v})
def width(order_key, seed=0):
    rnd = random.Random(seed)
    A = {v: set(s) for v, s in adj.items()}
    w = 0
    while A:
        if order_key == "deg": v = min(A, key=lambda u: (len(A[u]), rnd.random()))
        else:
            def fill(u):
                nb = list(A[u]); return sum(1 for i in range(len(nb)) for j in range(i) if nb[j] not in A[nb[i]])
            cand = sorted(A, key=lambda u: len(A[u]))[:60]
            v = min(cand, key=lambda u: (fill(u), rnd.random()))
        nb = A.pop(v); w = max(w, len(nb) + 1)
        for u in nb: A[u] |= nb - {u}; A[u].discard(v)
    return w
print(n, mode, [width(mode, s) for s in range(3)])
