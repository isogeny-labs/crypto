"""Wall time of one unpinned contraction (the dominant Lemma-1 count) per prime."""
import json, os, sys, time
import gpu_contract
from curves import prime_order_curve
from relation_p import build_p
H = os.path.dirname(os.path.abspath(__file__)); path = os.path.join(H, "first_count.json")
xp, name = gpu_contract.backend("numpy")
out = []
for p in [int(a) for a in sys.argv[1:]]:
    C, n = prime_order_curve(p); pts = C.points(); R = pts[0]
    net, iface = build_p(C, R, ([1] * p, [1] * p))
    plan = gpu_contract.Plan(net.nv, net.cons, [])
    t = time.time(); c = gpu_contract.contract_gpu(net.nv, net.cons, [], xp, 6.0, plan=plan); dt = time.time() - t
    brute = sum(1 for A in pts for B in pts if A[0] != B[0] and C.add(A, B) == R)
    out.append(dict(p=p, d=C.F.d, width=plan.width, seconds=round(dt, 2), count=c, brute=brute, backend=name))
    print(out[-1], flush=True)
json.dump(out, open(path, "w"), indent=1)
