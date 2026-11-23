#!/usr/bin/env python3
"""Pooled single-target analysis (PROTOCOL.md sections 3-5) -> data/pooled_measured.json."""
import json, os, sys, statistics, glob, math
from collections import defaultdict
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pooled_compare import pooled, load, unpriced, t975
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
SIZES = [int(x) for x in (sys.argv[1:] or ['28', '32'])]
ARMS = ['rho-neg', 'f1-x07', 'twcj-j16', 'twcj-j32', 'twcj-j64']
PAIRS = [('rho-neg', a) for a in ARMS[1:]] + [('f1-x07', 'twcj-j16'), ('twcj-j16', 'twcj-j32'), ('twcj-j16', 'twcj-j64')]


def inserts(x):
    return sum(p.get('work_native', {}).get('table_insert_attempts', 0) for p in x['phases'])


def pooled_tw(A, B, w, recs, sq):
    def S(x):
        return (x['cost']['total_gae'] + w * (unpriced(x) + inserts(x))) / sq
    ia = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == A}
    ib = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == B}
    cl = defaultdict(lambda: [0.0, 0.0])
    for k in sorted(set(ia) & set(ib)):
        cl[k[0]][0] += S(ia[k]); cl[k[0]][1] += S(ib[k])
    Ak = [v[0] for v in cl.values()]; Bk = [v[1] for v in cl.values()]
    R = sum(Bk) / sum(Ak); K = len(Ak); mA = statistics.fmean(Ak)
    z = [(bk - R * ak) / mA for ak, bk in zip(Ak, Bk)]
    se = statistics.stdev(z) / math.sqrt(K); t = t975(K - 1)
    return dict(ratio=R, ci95=[R * math.exp(-t * se / R), R * math.exp(t * se / R)], workloads=K)


def phases_of(x, sq):
    ph = {p['name']: p for p in x['phases']}
    out = {}
    if x['arm'] == 'rho-neg':
        return {n: p['gae'] / sq for n, p in ph.items()}
    out['fixed (base, LA, [d]G check)'] = (ph['factor_base']['gae'] + ph['linear_algebra']['gae'] + ph['verify']['gae']) / sq
    if x['arm'].startswith('tw'):
        rel = ph['relations']['native']
        out['setup'] = (rel['tw_setup_adds'] + rel['tw_setup_doubles']) / sq
        out['walk'] = rel['tw_walk_adds'] / sq
        out['collision verify'] = rel['tw_verify_adds'] / sq
    else:
        walk = ph['oracle_setup']['work_native']['table_insert_attempts']
        out['setup + collision verify'] = (ph['oracle_setup']['gae'] - walk) / sq
        out['walk'] = walk / sq
    out['probes'] = ph['relations']['gae'] / sq
    return out


out = {}
for k in SIZES:
    dirs = sorted(glob.glob(os.path.join(HERE, f'sessions/k{k}-s*')))
    if not dirs:
        continue
    recs = load(dirs)
    sq = {round(x['cost']['total_gae'] / x['cost']['s'], 6) for x in recs if x['cost']['s']}.pop()
    assert all(x['outcome']['status'] == 'verified' for x in recs)
    means, phases, diag = {}, {}, {}
    for a in ARMS:
        ra = [x for x in recs if x['arm'] == a]
        for w in (0, 0.1, 1):
            v = [(x['cost']['total_gae'] + w * unpriced(x)) / sq for x in ra]
            vt = [(x['cost']['total_gae'] + w * (unpriced(x) + inserts(x))) / sq for x in ra]
            means.setdefault(a, {})[str(w)] = dict(mean=statistics.fmean(v), mean_table_work=statistics.fmean(vt), n=len(v))
        acc = {}
        for x in ra:
            for n, v in phases_of(x, sq).items():
                acc[n] = acc.get(n, 0.0) + v
        phases[a] = {n: v / len(ra) for n, v in acc.items()}
        if a.startswith('tw'):
            rels = [{p['name']: p for p in x['phases']}['relations']['native'] for x in ra]
            steps = [r_['tw_steps'] / sq for r_ in rels]
            ok = [r_['tw_rank'] == 2 and r_['probe_hits'] == 0 and r_['trials'] == 1 for r_ in rels]
            diag[a] = dict(steps_mean=statistics.fmean(steps), steps_sd=statistics.stdev(steps),
                           rank2_first_trial_fraction=sum(ok) / len(ok),
                           collisions_mean=statistics.fmean(r_['tw_collisions'] for r_ in rels),
                           setup_gae_mean=statistics.fmean(r_['tw_setup_adds'] + r_['tw_setup_doubles'] for r_ in rels), n=len(rels))
    ratios = {f'{b}/{a}': {str(w): pooled(a, b, w, dirs) for w in (0, 0.1, 1)} for a, b in PAIRS}
    ratios_tw = {f'{b}/{a}': {str(w): pooled_tw(a, b, w, recs, sq) for w in (0, 0.1, 1)} for a, b in PAIRS}
    out[f'k{k}'] = dict(sessions=len(dirs), records=len(recs), sqrt_r=sq, S=means, ratios=ratios,
                        ratios_table_work_priced=ratios_tw, phases=phases, tw_diagnostics=diag)
json.dump(out, open(os.path.join(HERE, 'data/pooled_measured.json'), 'w'), indent=1)
for k, d in out.items():
    print(k, 'sessions', d['sessions'], 'records', d['records'])
    for a in ARMS:
        print('  S', a, ' '.join(f"{d['S'][a][w]['mean']:.3f}" for w in ('0', '0.1', '1')),
              '| inserts priced:', ' '.join(f"{d['S'][a][w]['mean_table_work']:.3f}" for w in ('0', '0.1', '1')))
    for p, r in d['ratios'].items():
        print('  R', p, ' '.join(f"{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('0', '0.1', '1')))
    for p, r in d['ratios_table_work_priced'].items():
        print('  Rtw', p, ' '.join(f"{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('0', '0.1', '1')))
    for a, ph in d['phases'].items():
        print('  P', a, ', '.join(f'{n}={v:.3f}' for n, v in ph.items()))
    for a, dg in d['tw_diagnostics'].items():
        print('  D', a, json.dumps({n: round(v, 4) for n, v in dg.items()}))
