"""Spot-check: rank-7 Mestre configs with the largest stored rho_G; recompute with nd=6 and nd=8 doublings.
Also check rho >= max_i hhat(P_i)/hhat(P_i) sanity, parallelogram law on two pairs. Imports the lab's heights/mestre (read-only)."""
import sys, json
sys.path.insert(0, sys.argv[1])
from fractions import Fraction as Fr
import numpy as np
from heights import gram, svp_gram
from mestre import mestre8, integral_model, evalp
rows = [json.loads(l) for l in open(sys.argv[2])]
cands = sorted([(c['rho_G'], d) for d in rows for c in d['cases'] if c['s'] == 7 and 'rho_G' in c], key=lambda t: -t[0])[:2]
for rho_stored, d in cands:
    q, r = mestre8(d['a']); E, mp_ = integral_model(r)
    pts = [mp_(Fr(a), evalp(q, Fr(a))) for a in d['a']]
    out = []
    for nd in (6, 8):
        M, err = gram(E, pts[:7], nd)
        lam, v = svp_gram(M)
        H = max(E.hhat(P, nd)[0] for P in pts)
        out.append((nd, H / lam, H, lam, float(err.max())))
    P, Q = pts[0], pts[1]
    h = lambda X: E.hhat(X, 8)[0]
    par = abs(h(E.add(P, Q)) + h(E.add(P, E.neg(Q))) - 2 * h(P) - 2 * h(Q)) / (2 * h(P) + 2 * h(Q))
    print('a', d['a'], 'stored rho %.4f' % rho_stored, ['nd=%d rho=%.4f H=%.4f lam=%.4f maxerr=%.1e' % o for o in out], 'par_rel(nd8) %.1e' % par)
