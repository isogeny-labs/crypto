# Expected cost (units of sqrt(n)) of: build table of mu*sqrt(n) known-log points
# (x-keyed, covers 2*mu*sqrt(n) group elements), then rho walk that also
# checks each step against the table. [pcc:unverified] analytic model.
import math
def E(mu, neg, du=1e-4):
    # survival S(u) = exp(-(2 mu u + k u^2)) with k=1/2 plain, k=1 neg (classes n/2)
    k = 1.0 if neg else 0.5
    s, u = 0.0, 0.0
    while True:
        v = math.exp(-(2*mu*u + k*u*u))
        if v < 1e-12: break
        s += v*du; u += du
    return mu + s
for neg in (False, True):
    best = min((E(mu/1000, neg), mu/1000) for mu in range(0, 1500, 5))
    print('neg' if neg else 'plain', 'rho alone', round(E(0, neg), 4), 'best union', round(best[0], 4), 'at mu', best[1])
for neg in (False, True):
    print('neg' if neg else 'plain', 'mu=0.7071', round(E(0.7071, neg), 4), 'pure BSGS mu=0.7071 (no rho)', round(0.7071 + 1/(2*0.7071), 4))
