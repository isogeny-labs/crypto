"""Part 2 analysis. Reads defect_min.json, raw_l21.jsonl, raw_sat_l*.jsonl.
Per cell/arm/unit: geometric mean (overall and per-base), S/A and A/B ratios, permutation test on
per-base geometric means, split decomposable/non-decomposable. Applies the decision rule.
Writes results.json."""
import json, glob, math, os
import numpy as np
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))


def gm(xs):
    xs = [x for x in xs if x > 0]
    return math.exp(sum(math.log(x) for x in xs) / len(xs)) if xs else float('nan')


def perbase_gm(rows, metric):
    pb = defaultdict(list)
    for r in rows:
        v = metric(r)
        if v is not None and v > 0 and not r.get('censored', False):
            pb[r['base']].append(v)
    return [gm(v) for v in pb.values() if v]


def perm_test(a, b, n=20000, seed=0):
    """2-sided permutation p on difference of mean(log) between groups a,b (per-base gm lists)."""
    a = np.log(np.array([x for x in a if x > 0], dtype=float))
    b = np.log(np.array([x for x in b if x > 0], dtype=float))
    if len(a) == 0 or len(b) == 0:
        return float('nan')
    obs = abs(a.mean() - b.mean())
    pool = np.concatenate([a, b])
    na = len(a)
    rng = np.random.default_rng(seed)
    cnt = 0
    for _ in range(n):
        rng.shuffle(pool)
        if abs(pool[:na].mean() - pool[na:].mean()) >= obs - 1e-12:
            cnt += 1
    return cnt / n


def boot_ratio_ci(numer, denom, n=10000, seed=0):
    """ratio of geometric means with bootstrap CI over per-base values (independent resample)."""
    a = np.log(np.array([x for x in numer if x > 0], dtype=float))
    b = np.log(np.array([x for x in denom if x > 0], dtype=float))
    if len(a) == 0 or len(b) == 0:
        return float('nan'), [float('nan'), float('nan')]
    rng = np.random.default_rng(seed)
    ra = a[rng.integers(0, len(a), (n, len(a)))].mean(1)
    rb = b[rng.integers(0, len(b), (n, len(b)))].mean(1)
    d = ra - rb
    lo, hi = np.percentile(d, [2.5, 97.5])
    return math.exp(a.mean() - b.mean()), [math.exp(lo), math.exp(hi)]


def analyse_unit(rows, metricname, metric):
    cells = sorted({(r['l'], r['n']) for r in rows})
    out = {}
    for (l, n) in cells:
        for dec in (True, False):
            key = f"l{l}n{n}|{'dec' if dec else 'nondec'}"
            sel = [r for r in rows if r['l'] == l and r['n'] == n and r['dec'] == dec]
            arms = {}
            pbs = {}
            for arm in 'SAB':
                a = [r for r in sel if r['arm'] == arm]
                vals = [metric(r) for r in a if metric(r) is not None and not r.get('censored', False)]
                arms[arm] = dict(n=len(vals), censored=sum(r.get('censored', False) for r in a),
                                 gm=gm(vals), w=sorted({r['w'] for r in a}))
                pbs[arm] = perbase_gm(a, metric)
            sa, sa_ci = boot_ratio_ci(pbs['S'], pbs['A'])
            ab, ab_ci = boot_ratio_ci(pbs['A'], pbs['B'])
            out[key] = dict(
                l=l, n=n, dec=dec, metric=metricname, arms=arms,
                nbases=dict((k, len(v)) for k, v in pbs.items()),
                S_over_A=sa, S_over_A_ci=sa_ci, A_over_B=ab, A_over_B_ci=ab_ci,
                perm_S_vs_A=perm_test(pbs['S'], pbs['A']),
                perm_A_vs_B=perm_test(pbs['A'], pbs['B']),
            )
    return out


def main():
    res = dict(tag='[pcc:unverified] pilot; see PROTOCOL.md / README.md', part_a={}, part_b={}, part_c={})
    # part a
    if os.path.exists(os.path.join(HERE, 'defect_min.json')):
        D = json.load(open(os.path.join(HERE, 'defect_min.json')))
        a_hold = all(c['min_equals_2lm1'] for c in D['exhaustive']) and \
            not any(c['any_below_2lm1'] for c in D['sampled'])
        res['part_a'] = dict(exhaustive=D['exhaustive'], sampled=D['sampled'], control=D['control'],
                             holds_all_cells=a_hold)
    # part c (L21)
    if os.path.exists(os.path.join(HERE, 'raw_l21.jsonl')):
        rows = [json.loads(x) for x in open(os.path.join(HERE, 'raw_l21.jsonl'))]
        res['part_c'] = dict(
            leaves=analyse_unit(rows, 'leaves', lambda r: r['leaves']),
            cands=analyse_unit(rows, 'cands', lambda r: r['cands']),
        )
    # part b (SAT)
    satrows = []
    for f in glob.glob(os.path.join(HERE, 'raw_sat_l*.jsonl')):
        satrows += [json.loads(x) for x in open(f)]
    if satrows:
        res['part_b'] = dict(
            conflicts=analyse_unit(satrows, 'conflicts', lambda r: r['conflicts']),
        )

    # decision rule
    a_hold = res['part_a'].get('holds_all_cells', None)
    # part b power: need >=4 complete bases per arm for a usable comparison/A-A floor
    b_minbases = min((min(d['nbases'].values()) for d in res.get('part_b', {}).get('conflicts', {}).values()),
                     default=0)
    b_powered = b_minbases >= 4
    # part b: S lowers conflicts by >=2x outside A/A? i.e. S/A <= 0.5 and its CI upper < A/B range lower?
    b_sep = False
    b_inside_aa = True
    for key, d in res.get('part_b', {}).get('conflicts', {}).items():
        sa = d['S_over_A']
        sa_ci = d['S_over_A_ci']
        ab_ci = d['A_over_B_ci']
        if sa == sa and sa_ci[1] == sa_ci[1]:
            # "outside A/A range": S/A CI does not overlap [min,max] of A/B CI
            outside = sa_ci[1] < ab_ci[0] or sa_ci[0] > ab_ci[1]
            if outside:
                b_inside_aa = False
            if sa <= 0.5 and outside:
                b_sep = True
    a_below = any(c['any_below_2lm1'] for c in res['part_a'].get('sampled', [])) or \
        any(c.get('any_below') for c in res['part_a'].get('exhaustive', []))
    if a_below or (b_sep and b_powered):
        verdict = 'alive'
    elif a_hold and b_powered and b_inside_aa and not b_sep:
        verdict = 'nothing to synthesise'
    elif a_hold and not b_powered:
        # (a) decisive (no V below 2l-1 anywhere); (b) SAT could not be powered here (box contention)
        verdict = 'nothing to synthesise for the structural/defect claim (a); SAT clause (b) underpowered'
    else:
        verdict = 'inconclusive'
    res['decision'] = dict(
        rule='nothing-to-synthesise if (a) holds everywhere AND (b) S-vs-random inside A/A; '
             'alive if a V below 2l-1 exists or structured V lowers SAT conflicts >=2x outside A/A; '
             'else inconclusive',
        part_a_holds=a_hold, part_a_any_V_below_2lm1=a_below,
        part_b_powered=b_powered, part_b_min_bases_per_arm=b_minbases,
        part_b_separates=b_sep, part_b_inside_aa=b_inside_aa, verdict=verdict)
    json.dump(res, open(os.path.join(HERE, 'results.json'), 'w'), indent=1)
    print(json.dumps(res['decision'], indent=1))
    for grp in ('part_b', 'part_c'):
        for unit, cells in res.get(grp, {}).items():
            for key, d in cells.items():
                print(f"{grp} {unit} {key}: S/A={d['S_over_A']:.3f} {d['S_over_A_ci']} "
                      f"A/B={d['A_over_B']:.3f} {d['A_over_B_ci']} "
                      f"permSA={d['perm_S_vs_A']:.3f} permAB={d['perm_A_vs_B']:.3f} "
                      f"nS/A/B={d['nbases']}")


if __name__ == '__main__':
    main()
