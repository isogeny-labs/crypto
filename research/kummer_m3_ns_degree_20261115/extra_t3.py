import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run_all import *
rows = []
for p in (40961, 12289):
    C, meta, F = make_instance(p, 3, 0)
    r = dict(p=p, t=3, idx=0, d_by_E={})
    for E in (3, 4):
        t1 = time.time(); d, info, _ = d_ns_desc(C, F, E, 13)
        r["d_by_E"][str(E)] = dict(d=d, steps=info, runtime_s=time.time() - t1); log(f"p={p} E={E} d={d}")
    rows.append(r)
dump("extra_t3_E34", rows)
