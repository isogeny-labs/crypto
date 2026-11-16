#!/usr/bin/env python3
"""S4(x1,x2,x3,x4) = Res_X(S3(x1,x2,X), S3(x3,x4,X)), symmetrized in x1,x2,x3 -> e1,e2,e3.
Writes s4_sym.json: list of [i, j, k, coeff-poly terms [[ex4, ea, eb, int], ...]]."""
import json, time
from sympy import symbols, resultant, expand, Poly
from sympy.polys.polyfuncs import symmetrize
t0 = time.time()
x1, x2, x3, x4, X, a, b, e1, e2, e3 = symbols('x1 x2 x3 x4 X a b e1 e2 e3')
def S3(u, v, w):
    return (u - v)**2 * w**2 - 2*((u + v)*(u*v + a) + 2*b)*w + (u*v - a)**2 - 4*b*(u + v)
S4 = expand(resultant(S3(x1, x2, X), S3(x3, x4, X), X))
sym, rem = symmetrize(S4, [x1, x2, x3], formal=True)[0:2] if False else (None, None)
res = symmetrize(S4, x1, x2, x3, formal=True)
symexpr, remainder, defs = res
assert remainder == 0, remainder
sub = {s: e for (s, _), e in zip(defs, (e1, e2, e3))}
symexpr = expand(symexpr.subs(sub))
P = Poly(symexpr, e1, e2, e3)
out = []
for (i, j, k), c in P.terms():
    cp = Poly(c, x4, a, b)
    out.append([i, j, k, [[m[0], m[1], m[2], int(v)] for m, v in cp.terms()]])
# check: substituting e's back reproduces S4 at a random point
import random
vals = {x1: 3, x2: 5, x3: 7, x4: 11, a: 13, b: 17}
E = {e1: 15, e2: 3*5 + 3*7 + 5*7, e3: 105}
assert expand(symexpr.subs(E).subs(vals) - S4.subs(vals)) == 0
json.dump(dict(terms=out, total_degree=P.total_degree(), degs=P.degree_list() if hasattr(P,'degree_list') else None,
               nterms=len(out), sec=time.time() - t0), open('s4_sym.json', 'w'))
print('S4 sym: terms', len(out), 'total degree', P.total_degree(), 'sec', time.time() - t0)
