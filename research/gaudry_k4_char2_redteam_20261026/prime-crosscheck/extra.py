import json, math, collections
import numpy as np
exec(open('fit.py').read().split("prime=D['prime']")[0])  # reuse ols/fmt helpers
prime=D['prime']; binary=D['binary']
ic_all=[r for r in prime if r['arm']=='ic-d4-w' and r['status']=='verified']
bic=[r for r in binary if r['arm']=='ic-wied' and r['status']=='verified']
def rung(L, field):
    g=collections.defaultdict(list)
    for r in L: g[r['q']].append(r[field])
    qs=sorted(g); return ols([math.log(q) for q in qs],[math.log(np.mean(g[q])) for q in qs])
for lab,L in [("prime all-clean",ic_all),("prime chain1",[r for r in ic_all if r['session'].startswith('main-')])]:
    print(lab, "trials rung:", fmt(rung(L,'trials')), "| relations found rung:", fmt(rung(L,'rel')), "| GAE per trial rung:", fmt(ols([math.log(r['q']) for r in L],[math.log(r['GAE_rel']/r['trials']) for r in L])))
print("binary ic-wied", "trials rung:", fmt(rung(bic,'trials')), "| rel rung:", fmt(rung(bic,'rel')), "| GAE per trial:", fmt(ols([math.log(r['q']) for r in bic],[math.log(r['GAE_rel']/r['trials']) for r in bic])))
# S-vs-GAE normalisation check: S/GAE constant within a p across arms?
print("prime S/GAE by p (rho, ic) mean:")
