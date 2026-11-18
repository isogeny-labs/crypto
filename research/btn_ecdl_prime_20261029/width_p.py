"""Min-fill elimination width of the prime-field relation network, per bit size d."""
import sys, time, random
from btn import simplify
from curves import prime_order_curve
from relation_p import build_p

def width(cons, seed=0):
    cons, _ = simplify(cons)
    adj = {}
    for sc, _ in cons:
        for v in sc: adj.setdefault(v, set()).update(set(sc) - {v})
    rnd = random.Random(seed); w = 0
    while adj:
        def fill(u):
            nb = list(adj[u]); return sum(1 for i in range(len(nb)) for j in range(i) if nb[j] not in adj[nb[i]])
        cand = sorted(adj, key=lambda u: len(adj[u]))[:40]
        v = min(cand, key=lambda u: (fill(u), rnd.random()))
        nb = adj.pop(v); w = max(w, len(nb) + 1)
        for u in nb: adj[u] |= nb - {u}; adj[u].discard(v)
    return w

if __name__ == "__main__":
  for p in [int(a) for a in sys.argv[1:]]:
      C, n = prime_order_curve(p)
      pts = C.points(); R = pts[0]
      full = [1] * p
      net, iface = build_p(C, R, (full, full))
      t = time.time(); w = width(net.cons); 
      print(f"p={p} d={C.F.d} order={n} vars={net.nv} cons={len(net.cons)} width={w} ({time.time()-t:.0f}s)", flush=True)
