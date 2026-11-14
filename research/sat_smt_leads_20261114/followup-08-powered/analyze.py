"""Analysis fixed in PROTOCOL.md. usage: python3 analyze.py raw1.jsonl [raw2.jsonl ...] > analysis.txt
Writes results.json."""
import json, sys, math, random
from statistics import median

FRESH = ('FRESH-A', 'FRESH-U', 'SPEC')
ARMS = ('INC', 'INC2', 'FRESH-A', 'FRESH-U', 'SPEC', 'FRESH-A2')


def load(files):
    cells = {}
    for fn in files:
        for L in open(fn):
            r = json.loads(L)
            c = cells.setdefault(r['l'], dict(meta=None, rows={}, files=[]))
            if r['kind'] == 'meta':
                c['files'].append(fn)
                if c['meta'] is None:
                    c['meta'] = r
                else:
                    assert c['meta']['targets'] == r['targets'], 'target lists differ between processes'
                    c['meta']['arms'] = c['meta']['arms'] + r['arms']
            else:
                c['rows'].setdefault(r['idx'], {})[r['arm']] = r
    return cells


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def boot_gm(xs, seed, B=10000):
    rng = random.Random(seed); n = len(xs); lg = [math.log(x) for x in xs]; out = []
    for _ in range(B):
        out.append(sum(lg[rng.randrange(n)] for _ in range(n)) / n)
    out.sort()
    return [math.exp(out[int(0.025 * B)]), math.exp(out[int(0.975 * B) - 1])], out


def ols(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx if sxx else float('nan')


def boot_slope(xs, ys, seed, B=10000):
    rng = random.Random(seed); n = len(xs); out = []
    for _ in range(B):
        ii = [rng.randrange(n) for _ in range(n)]
        s = ols([xs[i] for i in ii], [ys[i] for i in ii])
        if s == s:
            out.append(s)
    out.sort()
    return [out[int(0.025 * len(out))], out[int(0.975 * len(out)) - 1]]


def stat(xs, seed):
    if not xs:
        return None
    ci, _ = boot_gm(xs, seed)
    return dict(n=len(xs), gm=round(gm(xs), 4), ci95=[round(ci[0], 4), round(ci[1], 4)])


def main(files):
    cells = load(files); res = dict(cells={}, raw_rows=[]); boots = {}
    for l in sorted(cells):
        c = cells[l]; m = c['meta']; N = len(m['targets']); half = N // 2
        rows = [c['rows'][i] for i in range(N)]
        for r in rows:
            for a in ARMS:
                res['raw_rows'].append(r[a])
        errs = sum(1 for r in rows for a in ARMS if r[a]['err'])
        cf = lambda r, a: max(r[a]['conflicts'], 1)
        best = lambda r: min(cf(r, a) for a in FRESH)
        sets = dict(
            unsat_second=[r for i, r in enumerate(rows) if i >= half and not r['INC']['decomposable']],
            unsat_first=[r for i, r in enumerate(rows) if 1 <= i < half and not r['INC']['decomposable']],
            unsat_all=[r for i, r in enumerate(rows) if i >= 1 and not r['INC']['decomposable']],
            sat_all=[r for i, r in enumerate(rows) if i >= 1 and r['INC']['decomposable']])
        out = dict(n=m['n'], l=l, targets=N, n_unsat=m['n_unsat'], n_sat=m['n_sat'],
                   decomposable_abscissae_exact=m.get('decomposable_abscissae_exact'),
                   signed_base=m['signed_base'], vars=m['vars'], clauses=m['clauses'],
                   seeds=dict(setup=m['setup_seed'], target=m['target_seed'], shuffle=m['shuffle_seed']),
                   mod=m['mod'], V=m['V'], errors=errs, files=c['files'],
                   reference_counts=dict(linear_2plus1_leaf_solves=2 ** l,
                                         mitm=2 ** (2 * l) // 2 + 2 ** l, enumeration=round(2 ** (3 * l) / 6)),
                   ratios={}, medians={}, sums={})
        P = sets['unsat_second']
        prim = [cf(r, 'INC') / best(r) for r in P]
        ci, bs = boot_gm(prim, 808); boots[l] = (bs, None)
        out['primary'] = dict(n=len(prim), gm=round(gm(prim), 4), ci95=[round(ci[0], 4), round(ci[1], 4)])
        _, bs2 = boot_gm(prim, 810 + l); boots[l] = bs2
        out['null_reference_FRESH-A2_over_BEST'] = stat([cf(r, 'FRESH-A2') / best(r) for r in P], 808)
        aa = [cf(r, 'FRESH-A2') / cf(r, 'FRESH-A') for r in P]
        la = [math.log(x) for x in aa]; ma = sum(la) / len(la)
        out['AA_FRESH-A2_over_FRESH-A'] = dict(stat(aa, 808), sd_ln=round(math.sqrt(sum((x - ma) ** 2 for x in la) / (len(la) - 1)), 4))
        for sname, R in sets.items():
            d = {}
            for num, den in (('INC', 'BEST'), ('INC2', 'BEST'), ('FRESH-A2', 'BEST'), ('INC', 'FRESH-A'),
                             ('INC', 'FRESH-U'), ('INC', 'SPEC'), ('INC2', 'FRESH-A2'), ('FRESH-A2', 'FRESH-A'),
                             ('FRESH-A', 'SPEC')):
                dv = (lambda r: best(r)) if den == 'BEST' else (lambda r, den=den: cf(r, den))
                s = stat([cf(r, num) / dv(r) for r in R], 808)
                if s:
                    s['ratio_of_sums'] = round(sum(cf(r, num) for r in R) / sum(dv(r) for r in R), 4)
                d[num + '/' + den] = s
            out['ratios'][sname] = d
            out['medians'][sname] = {a: median(r[a]['conflicts'] for r in R) for a in ARMS} if R else None
            if R:
                out['medians'][sname]['BEST'] = median(best(r) for r in R)
        out['propagations_INC_over_BESTconf_arm_unsat_second'] = round(
            gm([max(r['INC']['propagations'], 1) / max(min((r[a] for a in FRESH), key=lambda q: q['conflicts'])['propagations'], 1) for r in P]), 4)
        U = sets['unsat_all']; xs = [r['INC']['idx'] for r in U]
        for name, num in (('position_slope_INC_over_BEST', 'INC'), ('position_slope_FRESH-A2_over_BEST', 'FRESH-A2'),
                          ('position_slope_INC2_over_BEST', 'INC2')):
            ys = [math.log(cf(r, num) / best(r)) for r in U]
            s = ols(xs, ys); ci = boot_slope(xs, ys, 809)
            out[name] = dict(n=len(xs), per_target=round(s, 5), ci95=[round(ci[0], 5), round(ci[1], 5)],
                             per_10_targets_factor=round(math.exp(10 * s), 4),
                             per_10_ci95=[round(math.exp(10 * ci[0]), 4), round(math.exp(10 * ci[1]), 4)])
        out['reuse_curve_unsat'] = [[r['INC']['idx'], round(cf(r, 'INC') / best(r), 3)] for r in rows if not r['INC']['decomposable']]
        out['best_arm_counts_unsat_second'] = {a: sum(1 for r in P if cf(r, a) == best(r)) for a in FRESH}
        res['cells'][l] = out
    L = max(res['cells']); p = res['cells'][L]['primary']
    verdict = 'ALIVE' if p['ci95'][1] < 0.8 else ('KILL' if p['ci95'][0] > 0.8 else 'INCONCLUSIVE')
    res['decision'] = dict(l=L, rule='alive if upper<0.8; kill if lower>0.8; else inconclusive', gm=p['gm'], ci95=p['ci95'], n=p['n'], verdict=verdict)
    res['l_trend'] = {}
    for l in sorted(res['cells']):
        if l == L:
            continue
        d = sorted(a - b for a, b in zip(boots[L], boots[l]))
        ci = [d[int(0.025 * len(d))], d[int(0.975 * len(d)) - 1]]
        pt = math.log(res['cells'][L]['primary']['gm'] / res['cells'][l]['primary']['gm'])
        res['l_trend']['ln_gm_l%d_minus_l%d' % (L, l)] = dict(diff=round(pt, 4), ci95=[round(ci[0], 4), round(ci[1], 4)],
            reading='falls' if ci[1] < 0 else ('rises' if ci[0] > 0 else 'holds / not resolved'))
    json.dump(res, open('results.json', 'w'), indent=1)
    # printed tables
    print('DECISION', json.dumps(res['decision']))
    print('L_TREND', json.dumps(res['l_trend']))
    for l, o in res['cells'].items():
        print('\n== l=%d n=%d targets=%d (unsat %d + sat %d) decomposable abscissae=%s errors=%d refs=%s' % (
            l, o['n'], o['targets'], o['n_unsat'], o['n_sat'], o['decomposable_abscissae_exact'], o['errors'], o['reference_counts']))
        print(' primary INC/BEST unsat second half:', o['primary'])
        print(' null ref FRESH-A2/BEST:', o['null_reference_FRESH-A2_over_BEST'])
        print(' A/A FRESH-A2/FRESH-A:', o['AA_FRESH-A2_over_FRESH-A'])
        for k in ('position_slope_INC_over_BEST', 'position_slope_FRESH-A2_over_BEST', 'position_slope_INC2_over_BEST'):
            print(' ', k, o[k])
        print('  best-arm counts (unsat second):', o['best_arm_counts_unsat_second'], ' propagations INC/best-arm:', o['propagations_INC_over_BESTconf_arm_unsat_second'])
        for s, d in o['ratios'].items():
            print('  [%s] medians %s' % (s, o['medians'][s]))
            for k, v in d.items():
                print('     %-18s %s' % (k, v))


if __name__ == '__main__':
    main(sys.argv[1:])
