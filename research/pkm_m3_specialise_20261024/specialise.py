#!/usr/bin/env python3
"""specialise.py IN.ms OUT.ms SPEC...   SPEC = VAR=L:P  ->  VAR := (31^((p-1)/2^L))^P  mod p.
Substitutes the constants into every polynomial, drops the variable, prints a summary.
31 is a primitive root of p = 2013265921; L:P with P odd gives a primitive 2^L-th root of unity."""
import sys, re
src, dst, specs = sys.argv[1], sys.argv[2], sys.argv[3:]
lines = open(src).read().strip().split('\n')
vars_ = lines[0].split(','); p = int(lines[1])
body = ','.join(lines[2:]).split(',')
body = [b.strip() for b in body if b.strip()]
fix = {}
for s in specs:
    v, lp = s.split('='); L, P = map(int, lp.split(':'))
    assert v in vars_ and (p - 1) % (1 << L) == 0
    fix[v] = pow(pow(31, (p - 1) >> L, p), P, p)
def parse(t):
    c = 1; mon = {}
    for f in t.split('*'):
        if re.fullmatch(r'\d+', f): c = c * int(f) % p
        else:
            m = re.fullmatch(r'(\w+)(?:\^(\d+))?', f); mon[m.group(1)] = mon.get(m.group(1), 0) + int(m.group(2) or 1)
    return c, mon
polys = []
for b in body:
    acc = {}
    for t in b.split('+'):
        c, mon = parse(t)
        for v in list(mon):
            if v in fix: c = c * pow(fix[v], mon.pop(v), p) % p
        k = tuple(sorted(mon.items())); acc[k] = (acc.get(k, 0) + c) % p
    acc = {k: c for k, c in acc.items() if c}
    polys.append(acc)
nv = [v for v in vars_ if v not in fix]
bad = [a for a in polys if list(a) == [()]]
out = [a for a in polys if a and list(a) != [()]]
def fmt(a):
    return '+'.join('*'.join([str(c)] + [v if e == 1 else f'{v}^{e}' for v, e in k]) for k, c in sorted(a.items()))
open(dst, 'w').write(','.join(nv) + '\n' + str(p) + '\n' + ',\n'.join(fmt(a) for a in out) + '\n')
print(f'{dst}: vars {len(vars_)}->{len(nv)}, polys {len(body)}->{len(out)}, zero-dropped {len(polys)-len(out)-len(bad)}, inconsistent-constants {len(bad)}, fixed {fix}')
