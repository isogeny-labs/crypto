#!/usr/bin/env python3
"""(3) Independent exhaustive census at k = 16: every unordered pair x1 <= x2 in F_p.
Usage: v_exhaustive.py OUT.json [rows]"""
import json, sys, time
import numpy as np
sys.path.insert(0, __import__("os").path.dirname(__file__))
from vcore import CURVES, Tables, classify

k = 16
p, A = CURVES[k]
T = Tables(p, A)
rows = int(sys.argv[2]) if len(sys.argv) > 2 else p
H = np.zeros(9 * 2 * 27, dtype=np.int64)
et_examples = []
t0 = time.process_time()
allx = np.arange(p, dtype=np.int64)
for i in range(rows):
    x2 = allx[i:]
    pc, deg, code = classify(T, i, x2)
    assert int(code.min()) >= 0
    H += np.bincount((pc * 2 + deg) * 27 + code, minlength=9 * 2 * 27)
    bad = (pc == 5) & (code > 0)
    if bad.any() and len(et_examples) < 100:
        et_examples += [[i, int(v)] for v in x2[bad][:100]]
    if i % 5000 == 0:
        print(f"row {i} cpu {time.process_time()-t0:.1f}s", file=sys.stderr, flush=True)
cpu = time.process_time() - t0
NAMES = {0: "ZZ", 1: "ZE", 2: "ZT", 4: "EE", 5: "ET", 8: "TT"}
table = []
for pc, name in NAMES.items():
    for deg in (0, 1):
        sl = H[(pc * 2 + deg) * 27:(pc * 2 + deg + 1) * 27]
        if sl.sum() == 0:
            continue
        codes = np.arange(27)
        nZ, nT, nE = codes % 3, (codes // 3) % 3, codes // 9
        nr = nZ + nT + nE
        table.append(dict(pair_class=name, x1_eq_x2=bool(deg), pairs=int(sl.sum()),
                          roots0=int(sl[nr == 0].sum()), roots1=int(sl[nr == 1].sum()), roots2=int(sl[nr == 2].sum()),
                          roots_on_E=int((sl * nE).sum()), roots_on_twist=int((sl * nT).sum()),
                          roots_on_Z=int((sl * nZ).sum()), pairs_with_root_on_Z=int(sl[nZ > 0].sum())))
res = dict(k=k, p=p, A=A, rows=rows, npairs=int(H.sum()), cpu_s=round(cpu, 1), table=table,
           hist={str(j): int(v) for j, v in enumerate(H) if v}, et_with_root_examples=et_examples)
json.dump(res, open(sys.argv[1], "w"), indent=1)
for t in table:
    print(t)
print("npairs", res["npairs"], "cpu", res["cpu_s"])
