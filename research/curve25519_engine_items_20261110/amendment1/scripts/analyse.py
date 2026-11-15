#!/usr/bin/env python3
"""Amendment 1 (PROTOCOL.md Addendum 1): copy of ../../scripts/analyse.py restricted to the arms
rho-neg, twcj, twcj-cv0 -> amendment1/data/pooled_measured.json.
Pooled single-target analysis (PROTOCOL.md sections 3-5).
Measured rounds only (pooled_compare.load skips warm-up records). Two band readings:
'priced' (the current default: table inserts priced at w as well) and 'unpriced'
(--no-price-table-work: cost.unpriced only). Template: research/curve25519_tw_jumps_20261108."""
import json, os, sys, statistics, glob, math
from collections import defaultdict
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'scripts'))
from pooled_compare import pooled, load, unpriced, t975

HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
SIZES = [int(x) for x in (sys.argv[1:] or ['20', '24'])]
ARMS = ['rho-neg', 'twcj', 'twcj-cv0']
PAIRS = [('twcj', 'twcj-cv0'), ('rho-neg', 'twcj'), ('rho-neg', 'twcj-cv0')]
WS = (0, 0.1, 1)


def inserts(x):
    return sum(p.get('work_native', {}).get('table_insert_attempts', 0) for p in x['phases'])


def ratio_ci(Ak, Bk):
    R = sum(Bk) / sum(Ak); K = len(Ak); mA = statistics.fmean(Ak)
    z = [(bk - R * ak) / mA for ak, bk in zip(Ak, Bk)]
    se = statistics.stdev(z) / math.sqrt(K); t = t975(K - 1)
    return dict(ratio=R, ci95=[R * math.exp(-t * se / R), R * math.exp(t * se / R)], workloads=K)


def pooled_fn(A, B, recs, f):
    ia = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == A}
    ib = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == B}
    cl = defaultdict(lambda: [0.0, 0.0])
    for k in sorted(set(ia) & set(ib)):
        cl[k[0]][0] += f(ia[k]); cl[k[0]][1] += f(ib[k])
    return ratio_ci([v[0] for v in cl.values()], [v[1] for v in cl.values()])


def S_of(w, sq, priced):
    return lambda x: (x['cost']['total_gae'] + w * (unpriced(x) + (inserts(x) if priced else 0))) / sq


def phase(x, name):
    for p in x['phases']:
        if p['name'] == name:
            return p
    return None


def phases_of(x, sq):
    if x['arm'].startswith('rho'):
        return {p['name']: p['gae'] / sq for p in x['phases']}
    rel = phase(x, 'relations')['native']
    out = {}
    out['fixed (base, LA, [d]G check)'] = sum(phase(x, n)['gae'] for n in ('factor_base', 'linear_algebra', 'verify')) / sq
    out['setup'] = (rel['tw_setup_adds'] + rel['tw_setup_doubles']) / sq
    out['walk'] = rel['tw_walk_adds'] / sq
    out['collision verify'] = rel['tw_verify_adds'] / sq
    out['probes'] = phase(x, 'relations')['gae'] / sq
    return out


def recovered(x):
    o = x['outcome']
    return o.get('recovered', o.get('log'))


def identity_A(recs):
    """A3: paired twcj / twcj-cv0 runs agree on the walk and differ by tw_verify_adds exactly."""
    ia = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == 'twcj'}
    ib = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == 'twcj-cv0'}
    bad, n = [], 0
    for k in sorted(set(ia) & set(ib)):
        a, b = ia[k], ib[k]
        ra, rb = phase(a, 'relations')['native'], phase(b, 'relations')['native']
        n += 1
        ok = all(ra[c] == rb[c] for c in ('tw_steps', 'tw_rows', 'tw_rank'))
        ok &= recovered(a) == recovered(b)
        ok &= abs(a['cost']['total_gae'] - b['cost']['total_gae'] - ra['tw_verify_adds']) < 1e-9
        ok &= rb.get('tw_verify_adds', 0) == 0
        if not ok:
            bad.append(list(k))
    return dict(pairs=n, mismatches=len(bad), first_mismatches=bad[:5])


def rho_counters(ra):
    names = ['setup_additions', 'setup_doublings', 'walk_operations', 'start_additions',
             'look_ahead_additions', 'cycle_escape_doublings', 'cycles_length_2', 'cycles_length_4',
             'cycles_other_length', 'walks_abandoned_in_a_cycle', 'walks_capped', 'useless_collisions',
             'canonicalisations_uncharged', 'verification_operations']
    out = {}
    for n in names:
        v = [x.get('counters', {}).get(n) for x in ra]
        v = [y for y in v if y is not None]
        if v:
            out[n] = statistics.fmean(v)
    return out


out = {}
for k in SIZES:
    dirs = sorted(glob.glob(os.path.join(HERE, f'sessions/k{k}-s*')))
    if not dirs:
        continue
    recs = load(dirs)
    sq = {round(x['cost']['total_gae'] / x['cost']['s'], 6) for x in recs if x['cost']['s']}.pop()
    failures = {a: sum(1 for x in recs if x['arm'] == a and x['outcome']['status'] != 'verified') for a in ARMS}
    # Warm-up records too (prediction C covers every record).
    warm_fail = 0
    for d in dirs:
        for line in open(os.path.join(d, 'records.jsonl')):
            r = json.loads(line)
            if r.get('warmup') and r['outcome']['status'] != 'verified':
                warm_fail += 1
    means, phases, diag, rhoc = {}, {}, {}, {}
    for a in ARMS:
        ra = [x for x in recs if x['arm'] == a]
        for w in WS:
            means.setdefault(a, {})[str(w)] = dict(
                priced=statistics.fmean(map(S_of(w, sq, True), ra)),
                unpriced=statistics.fmean(map(S_of(w, sq, False), ra)), n=len(ra))
        acc = {}
        for x in ra:
            for n, v in phases_of(x, sq).items():
                acc[n] = acc.get(n, 0.0) + v
        phases[a] = {n: v / len(ra) for n, v in acc.items()}
        if a.startswith('tw'):
            rels = [phase(x, 'relations')['native'] for x in ra]
            steps = [r_['tw_steps'] / sq for r_ in rels]
            diag[a] = dict(steps_mean=statistics.fmean(steps), steps_sd=statistics.stdev(steps),
                           cap_below_rank2_fraction=sum(1 for r_ in rels if r_['tw_rank'] < 2) / len(rels),
                           probe_hits_mean=statistics.fmean(r_['probe_hits'] for r_ in rels),
                           collisions_mean=statistics.fmean(r_['tw_collisions'] for r_ in rels),
                           rows_mean=statistics.fmean(r_['tw_rows'] for r_ in rels),
                           verify_adds_mean=statistics.fmean(r_['tw_verify_adds'] for r_ in rels),
                           n=len(rels))
        else:
            rhoc[a] = rho_counters(ra)
            rhoc[a]['setup_gae_mean'] = statistics.fmean(phase(x, 'setup')['gae'] for x in ra)
    ratios = {}
    for reading, pr in (('priced', True), ('unpriced', False)):
        ratios[reading] = {f'{b}/{a}': {str(w): pooled_fn(a, b, recs, S_of(w, sq, pr)) for w in WS} for a, b in PAIRS}
    # Cross-check of the unpriced reading against the template's pooled() (same formula).
    check = pooled('twcj', 'twcj-cv0', 0, dirs)
    out[f'k{k}'] = dict(sessions=len(dirs), records=len(recs), sqrt_r=sq, failures_measured=failures,
                        failures_warmup=warm_fail, S=means, ratios=ratios, template_check=check,
                        phases=phases, tw_diagnostics=diag,
                        rho_counters=rhoc, identity_A=identity_A(recs))
json.dump(out, open(os.path.join(HERE, 'data/pooled_measured.json'), 'w'), indent=1)
for k, d in out.items():
    print(k, 'sessions', d['sessions'], 'records', d['records'], 'failures', d['failures_measured'], 'warm-up failures', d['failures_warmup'])
    for a in ARMS:
        print('  S', a, 'priced', ' '.join(f"{d['S'][a][w]['priced']:.3f}" for w in ('0', '0.1', '1')),
              '| unpriced', ' '.join(f"{d['S'][a][w]['unpriced']:.3f}" for w in ('0', '0.1', '1')))
    for rd in ('priced', 'unpriced'):
        for p, r in d['ratios'][rd].items():
            print('  R', rd, p, ' '.join(f"{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('0', '0.1', '1')))
    print('  template check', json.dumps(d['template_check']))
    for a, ph in d['phases'].items():
        print('  P', a, ', '.join(f'{n}={v:.3f}' for n, v in ph.items()))
    for a, dg in d['tw_diagnostics'].items():
        print('  D', a, json.dumps({n: round(v, 4) for n, v in dg.items()}))
    for a, c in d['rho_counters'].items():
        print('  C', a, json.dumps({n: round(v, 3) for n, v in c.items()}))
    print('  identity A', json.dumps(d['identity_A']))
