import sys, json
from tower_engine import *
p, a, b, tg = 12289, 4021, 9917, 4408
res = {}
for D, ks in ((3, range(4, 10)), (4, range(4, 10))):
    for k in ks:
        r = run_cell(Rules('kummer', k, p), p, a, b, tg, D, early=False)
        print(D, k, r, flush=True); res[f'{D},{k}'] = r
json.dump(res, open('kummer_gate.json', 'w'), indent=1)
