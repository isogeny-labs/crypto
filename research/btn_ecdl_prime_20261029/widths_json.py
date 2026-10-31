"""Measure min-fill widths for the given primes and append to widths.json."""
import json, os, sys, time
from curves import prime_order_curve
from relation_p import build_p
from width_p import width
H = os.path.dirname(os.path.abspath(__file__)); path = os.path.join(H, "widths.json")
out = json.load(open(path)) if os.path.exists(path) else []
for p in [int(a) for a in sys.argv[1:]]:
    C, n = prime_order_curve(p)
    net, _ = build_p(C, C.points()[0], ([1] * p, [1] * p))
    t = time.time(); w = width(net.cons)
    out = [r for r in out if r["p"] != p] + [dict(p=p, d=C.F.d, order=n, variables=net.nv, constraints=len(net.cons), width=w, seconds=round(time.time() - t, 1))]
    out.sort(key=lambda r: r["d"]); json.dump(out, open(path, "w"), indent=1)
    print(p, C.F.d, w, flush=True)
