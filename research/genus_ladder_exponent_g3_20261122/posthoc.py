#!/usr/bin/env python3
# POST HOC (not in the frozen protocol): rho reference with the random walk length replaced by its expectation sqrt(pi r / 4)
import json, math, random
A = json.load(open('analysis.json')); PR = ['65537','131101','262147','524309','1048583']
R9 = '../genus_ladder_wallclock_vs_rho_20261122/raw/'
def slope(x, y):
    lx = [math.log(a) for a in x]; ly = [math.log(a) for a in y]; mx = sum(lx)/len(lx); my = sum(ly)/len(ly)
    return sum((a-mx)*(b-my) for a, b in zip(lx, ly))/sum((a-mx)**2 for a in lx)
inst = {}; hs = {}
for p in PR:
    f = (R9 if int(p) < 500000 else 'raw/') + f'run_{p}.jsonl'
    rows = [json.loads(l) for l in open(f)]
    ii = [r for r in rows if r['kind'] == 'instance']
    inst[p] = [math.sqrt(math.pi*r['r']/4) for r in ii]; hs[p] = [r['h'] for r in ii]
nss = {p: A['primes'][p]['mean_wall_rho']*1e9/A['primes'][p]['rho_steps_mean'] for p in PR}
ew = lambda p, s: sum(inst[p][i] for i in s)/len(s)*nss[p]/1e9
xs = [int(p) for p in PR]
pt = slope(xs, [ew(p, range(len(inst[p]))) for p in PR])
rng = random.Random(7); bs = []
for _ in range(4000):
    bs.append(slope(xs, [ew(p, [rng.randrange(len(inst[p])) for _ in inst[p]]) for p in PR]))
bs.sort()
out = dict(exp_rho_wall_exponent=pt, ci=[bs[100], bs[3899]], h_values=hs,
           ratio_ic_over_expected_rho={p: A['primes'][p]['mean_wall_ic']/ew(p, range(len(inst[p]))) for p in PR},
           mean_expected_steps={p: sum(inst[p])/len(inst[p]) for p in PR}, ns_per_step=nss)
json.dump(out, open('posthoc.json', 'w'), indent=1); print(json.dumps(out, indent=1))
