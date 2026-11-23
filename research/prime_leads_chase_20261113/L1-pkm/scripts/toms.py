# raw msolve presentation from a gen.py JSON (recomputes S3 without tower reduction)
import json, sys
sys.path.insert(0, sys.path[0]); import gen
d = json.load(open(sys.argv[1])); p, m, t = d['p'], d['m'], d['t']; k = max(m-2, 0)
R = gen.Ring(p, m, 30); R.mask = (1 << 60) - 1   # no reduction
a, b, g, xR = d['a'], d['b'], d['g'], d['xR']
xs = [R.x(i, g) for i in range(m)]
if m == 2: polys = [gen.S3(R, xs[0], xs[1], R.const(xR), a, b)]
else:
    us = [R.u(j) for j in range(m-2)]
    polys = [gen.S3(R, xs[0], xs[1], us[0], a, b)]
    for kk in range(1, m-2): polys.append(gen.S3(R, us[kk-1], xs[kk+1], us[kk], a, b))
    polys.append(gen.S3(R, us[m-3], xs[m-1], R.const(xR), a, b))
def term(key, c):
    be, ue = key; f = [str(c)]
    for i, e in enumerate(be):
        if e: f.append(f"y{i}_0" + (f"^{e}" if e > 1 else ""))
    for j, e in enumerate(ue):
        if e: f.append(f"u{j}" + (f"^{e}" if e > 1 else ""))
    return '*'.join(f)
vars_ = [f"y{i}_{j}" for i in range(m) for j in range(t)] + [f"u{j}" for j in range(k)]
eqs = ['+'.join(term(kk, v) for kk, v in sorted(P.items(), reverse=True)) for P in polys]
for i in range(m):
    for j in range(t-1): eqs.append(f"1*y{i}_{j+1}+{p-1}*y{i}_{j}^2")
    eqs.append(f"1+{p-1}*y{i}_{t-1}^2")
open(sys.argv[2], 'w').write(','.join(vars_) + '\n' + str(p) + '\n' + ',\n'.join(eqs) + '\n')
