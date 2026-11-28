#!/usr/bin/env python3
import json, math, random, collections
R2 = '../genus_ladder_g3_highp_20261121/'
PR = [65537, 131101, 262147]
IC, RH, NS = 'C_double_lp+pre+sge(8,3)', 'R_rho_neg', 'C_double_lp+pre'
LA_NS = {65537: 16.6, 131101: 20.3, 262147: 19.0}   # R8 ns per counted la_mul (after), median of 15
def load(path):
    return [json.loads(l) for l in open(path)]
def runs(rows, seeds=None):
    d = {}
    for r in rows:
        if r['kind'] == 'run' and (seeds is None or r['curve_seed'] in seeds):
            d[(r['curve_seed'], r['seed'], r['arm'])] = r
    return d
def boot(new, arms, nboot=4000, rng=None):
    """new: {(cs,seed,arm): wall}. ratio of means; hierarchical bootstrap."""
    cs = sorted({k[0] for k in new}); sd = sorted({k[1] for k in new})
    def est(sample):  # list of (cs, [seeds])
        a = [sum(new[(c, s, arms[0])] for s in ss) / len(ss) for c, ss in sample]
        b = [sum(new[(c, s, arms[1])] for s in ss) / len(ss) for c, ss in sample]
        return sum(a) / sum(b), sum(a) / len(a), sum(b) / len(b)
    pt = est([(c, sd) for c in cs])
    rng = random.Random(20261122); out = []
    for _ in range(nboot):
        samp = []
        for _ in cs:
            c = rng.choice(cs); samp.append((c, [rng.choice(sd) for _ in sd]))
        out.append(est(samp)[0])
    out.sort()
    return pt, out[int(.025 * nboot)], out[int(.975 * nboot) - 1]
def slope(xs, ys):
    lx = [math.log(x) for x in xs]; ly = [math.log(y) for y in ys]
    mx = sum(lx) / len(lx); my = sum(ly) / len(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)
res = {'v1': {}, 'primes': {}}
seeds_of = {}
for p in PR:
    inst = [r for r in load(R2 + f'run_{p}.jsonl') if r['kind'] == 'instance'][:6]
    seeds_of[p] = [r['curve_seed'] for r in inst]
walls = {}; oldw = {}
for p in PR:
    new = load(f'raw/run_{p}.jsonl'); old = load(R2 + f'run_{p}.jsonl')
    ninst = [r for r in new if r['kind'] == 'instance']
    rn = runs(new); ro = runs(old, set(seeds_of[p]))
    # V1
    cert = all(r['certified'] for r in rn.values())
    diffs = []
    for k, r in rn.items():
        o = ro.get(k)
        if o is None: diffs.append((k, 'missing')); continue
        for f in set(r) | set(o):
            if f != 'wall_ns' and r.get(f) != o.get(f): diffs.append((k, f))
    inst_ok = [ (x['curve_seed'], x['order'], x['r']) for x in ninst ] == \
        [ (x['curve_seed'], x['order'], x['r']) for x in load(R2 + f'run_{p}.jsonl') if x['kind'] == 'instance'][:6]
    res['v1'][p] = dict(runs=len(rn), expected=6 * 2 * 3, all_certified=cert, n_diffs=len(diffs), diffs=diffs[:10], instances_equal=inst_ok)
    wn = {k: v['wall_ns'] for k, v in rn.items()}; wo = {k: v['wall_ns'] for k, v in ro.items()}
    pn = boot(wn, (IC, RH)); po = boot(wo, (IC, RH))
    pn_ns = boot(wn, (NS, RH))
    P = dict(new_ratio=pn, old_ratio=po, new_noSGE_ratio=pn_ns[0])
    for arm in (IC, NS, RH):
        a = [v for k, v in wn.items() if k[2] == arm]; b = [v for k, v in wo.items() if k[2] == arm]
        P['mean_wall_s_new_' + arm] = sum(a) / len(a) / 1e9
        P['mean_wall_s_old_' + arm] = sum(b) / len(b) / 1e9
    # ns per counted op
    rho = [r for r in rn.values() if r['arm'] == RH]
    P['rho_ns_per_fpm'] = sum(r['wall_ns'] for r in rho) / sum(r['cantor_mul'] + 10 * r['cantor_inv'] for r in rho)
    P['rho_ns_per_step'] = sum(r['wall_ns'] for r in rho) / sum(r['steps'] if 'steps' in r else r['walk_ops'] for r in rho)
    P['rho_fields'] = sorted(rho[0].keys())
    ic = [r for r in rn.values() if r['arm'] == IC]
    tot = sum(r['wall_ns'] for r in ic)
    la = sum(r['la_mul'] * LA_NS[p] for r in ic)
    relops = sum(r['cantor_mul'] + 10 * r['cantor_inv'] + r['rel_mul'] for r in ic)
    P['ic_ns_per_fpm_total'] = tot / sum(r['cantor_mul'] + 10 * r['cantor_inv'] + r['rel_mul'] + 9 * r['la_mul'] for r in ic)
    P['ic_la_wall_share_est'] = la / tot
    P['ic_rel_ns_per_op_est'] = (tot - la) / relops
    P['ic_la_ns_per_la_mul_total'] = tot / sum(r['la_mul'] for r in ic)
    P['ic_la_mul_mean'] = sum(r['la_mul'] for r in ic) / len(ic)
    P['ic_rel_ops_mean'] = relops / len(ic)
    P['rho_count_ratio_fpm'] = (sum(r['cantor_mul'] + 10 * r['cantor_inv'] + r['rel_mul'] + 9 * r['la_mul'] for r in ic) /
                                sum(r['cantor_mul'] + 10 * r['cantor_inv'] for r in rho))
    P['per_instance'] = {}
    for c in seeds_of[p]:
        a = [wn[(c, s, IC)] for s in (1, 2) if (c, s, IC) in wn]; b = [wn[(c, s, RH)] for s in (1, 2) if (c, s, RH) in wn]
        P['per_instance'][c] = dict(ic_s=sum(a) / len(a) / 1e9, rho_s=sum(b) / len(b) / 1e9, ratio=sum(a) / sum(b))
    res['primes'][p] = P
exp = {}
for key in ('new_', 'old_'):
    for arm in (IC, RH):
        exp[key + arm] = slope(PR, [res['primes'][p]['mean_wall_s_' + key + arm] for p in PR])
res['exponents_wall_vs_p'] = exp
json.dump(res, open('analysis.json', 'w'), indent=1, default=str)
