#!/usr/bin/env python3
"""Convert msolve input (vars line, prime line, comma-terminated polys) to M4GB default format.
Variables x1..xn -> X0..X{n-1}; drop field equations x^2+x (M4GB adds them for p=2);
over GF(2) reduce exponents to 1; over GF(p) keep x^k as repeated factors."""
import re, sys
src, dst = sys.argv[1], sys.argv[2]
lines = [l.strip() for l in open(src) if l.strip()]
vars_ = [v.strip() for v in lines[0].split(',')]
p = int(lines[1])
idx = {v: i for i, v in enumerate(vars_)}
out = [f"$fieldsize {p}", f"$vars {len(vars_)} X"]
for l in lines[2:]:
    l = l.rstrip(',')
    terms = []
    for t in l.split('+'):
        t = t.strip()
        if not t: continue
        coef = 1; fac = []
        for f in t.split('*'):
            m = re.fullmatch(r'(x\d+)(?:\^(\d+))?', f)
            if m:
                e = int(m.group(2) or 1)
                if p == 2: e = 1
                fac += [f"X{idx[m.group(1)]}"] * e
            else:
                coef = (coef * int(f)) % p
        if coef == 0: continue
        if p == 2:
            fac = sorted(set(fac), key=lambda s: int(s[1:]))
        s = '*'.join(fac) if fac else ''
        if coef != 1 or not fac:
            s = (str(coef) + ('*' + s if s else ''))
        terms.append(s)
    # field equation x^2+x over GF(2): after exponent reduction it is X+X -> cancel pairs
    if p == 2:
        from collections import Counter
        c = Counter(terms); terms = [t for t, k in c.items() if k % 2]
        if not terms: continue
    out.append('+'.join(terms))
open(dst, 'w').write('\n'.join(out) + '\n')
print(dst, len(vars_), p, len(out) - 2)
