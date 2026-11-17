#!/usr/bin/env python3
"""Pooled single-target analysis (PROTOCOL.md sections 3-5) -> data/pooled_measured.json.

Measured rounds only (pooled_compare.load skips warm-up records)."""
import json, os, sys, statistics, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pooled_compare import pooled, load, unpriced

HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
ARMS = ['rho-neg', 'f1-x07', 'tw-x15', 'twcj-x15']
PAIRS = [('f1-x07', 'twcj-x15'), ('f1-x07', 'tw-x15'), ('tw-x15', 'twcj-x15'),
         ('rho-neg', 'twcj-x15'), ('rho-neg', 'tw-x15'), ('rho-neg', 'f1-x07')]


def phase_breakdown(x, sq):
    """Per-record cost split in units of sqrt(r) (PROTOCOL.md section 3)."""
    ph = {p['name']: p for p in x['phases']}
    out = {}
    if x['arm'] == 'rho-neg':
        for n, p in ph.items():
            out[n] = p['gae'] / sq
        return out
    rel = ph['relations']['native']
    fixed = ph['factor_base']['gae'] + ph['linear_algebra']['gae'] + ph['verify']['gae']
    out['fixed (base, LA, [d]G check)'] = fixed / sq
    if x['arm'].startswith('tw'):
        out['setup'] = (rel['tw_setup_adds'] + rel['tw_setup_doubles']) / sq
        out['walk'] = rel['tw_walk_adds'] / sq
        out['collision verify'] = rel['tw_verify_adds'] / sq
        assert abs(out['setup'] + out['walk'] + out['collision verify']
                   - ph['oracle_setup']['gae'] / sq) < 1e-9, x['record_id']
        out['probes'] = ph['relations']['gae'] / sq
    else:
        walk = ph['oracle_setup']['work_native']['table_insert_attempts']
        out['setup + collision verify'] = (ph['oracle_setup']['gae'] - walk) / sq
        out['walk'] = walk / sq
        out['probes'] = ph['relations']['gae'] / sq
    return out


out = {}
for k in (20, 24):
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
            means.setdefault(a, {})[str(w)] = dict(mean=statistics.fmean(v), n=len(v))
        acc = {}
        for x in ra:
            for n, v in phase_breakdown(x, sq).items():
                acc[n] = acc.get(n, 0.0) + v
        phases[a] = {n: v / len(ra) for n, v in acc.items()}
        if a.startswith('tw'):
            rels = [{p['name']: p for p in x['phases']}['relations']['native'] for x in ra]
            steps = [r_['tw_steps'] / sq for r_ in rels]
            rank2 = [r_['tw_rank'] == 2 and r_['probe_hits'] == 0 and r_['trials'] == 1 for r_ in rels]
            diag[a] = dict(
                steps_mean=statistics.fmean(steps), steps_sd=statistics.stdev(steps),
                steps_first_row_mean=statistics.fmean(r_['tw_steps_first_row'] / sq for r_ in rels),
                rank2_first_trial_fraction=sum(rank2) / len(rank2), n=len(rels),
                collisions_mean=statistics.fmean(r_['tw_collisions'] for r_ in rels),
                rows_mean=statistics.fmean(r_['tw_rows'] for r_ in rels))
    ratios = {f'{b}/{a}': {str(w): pooled(a, b, w, dirs) for w in (0, 0.1, 1)} for a, b in PAIRS}
    out[f'k{k}'] = dict(sessions=len(dirs), records=len(recs), sqrt_r=sq, S=means, ratios=ratios,
                        phases=phases, tw_diagnostics=diag)
json.dump(out, open(os.path.join(HERE, 'data/pooled_measured.json'), 'w'), indent=1)
for k, d in out.items():
    print(k, 'sessions', d['sessions'], 'records', d['records'])
    for a in ARMS:
        print('  S', a, ' '.join(f"{d['S'][a][w]['mean']:.3f}" for w in ('0', '0.1', '1')))
    for p, r in d['ratios'].items():
        print('  R', p, ' '.join(f"{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('0', '0.1', '1')))
    for a, ph in d['phases'].items():
        print('  P', a, ', '.join(f'{n}={v:.3f}' for n, v in ph.items()))
    for a, dg in d['tw_diagnostics'].items():
        print('  D', a, json.dumps({n: round(v, 4) for n, v in dg.items()}))
