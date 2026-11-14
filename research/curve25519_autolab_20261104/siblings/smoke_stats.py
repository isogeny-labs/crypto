#!/usr/bin/env python3
"""Smoke statistics for SMOKE.md: reads smoke-k20/records.jsonl and band-smoke-k20.json."""
import json, math, random, statistics as st
recs = [json.loads(l) for l in open('smoke-k20/records.jsonl')]
recs = [r for r in recs if not r['warmup'] and r['outcome']['status'] == 'verified']
band = json.load(open('band-smoke-k20.json'))
r = int(recs[0]['workload']['curve']['r']) if 'r' in recs[0]['workload']['curve'] else None
sq = math.sqrt(r)
arms = ['rho-neg', 'tw2cj-lazy', 'tw2cj-lazy-sib8']
def inserts(x):
    return sum(p.get('work_native', {}).get('table_insert_attempts', 0) for p in x['phases'])
def pn(x, name):
    return sum(p.get('native', {}).get(name, 0) for p in x['phases'])
def setup_adds(x):
    return pn(x, 'tw_setup_adds')
def key(x):
    return (x['round'], x['workload']['target_index'])
by = {a: {key(x): x for x in recs if x['arm'] == a} for a in arms}
out = {'r': r, 'sqrt_r': sq, 'arms': {}}
for a in arms:
    xs = list(by[a].values())
    d = {'n': len(xs),
         'bitops_per_sqrt_r': st.mean(x['work']['bitops_per_sqrt_r'] for x in xs),
         'gae_per_sqrt_r': st.mean(x['cost']['total_gae'] / sq for x in xs)}
    if a == 'rho-neg':
        d['steps_per_sqrt_r'] = st.mean(x['counters']['walk_operations'] / sq for x in xs)
    else:
        d['stored_points_per_sqrt_r'] = st.mean(inserts(x) / sq for x in xs)
        d['stored_se'] = st.stdev(inserts(x) / sq for x in xs) / math.sqrt(len(xs))
        d['setup_adds'] = st.mean(setup_adds(x) for x in xs)
        d['tw_walk_adds'] = st.mean(pn(x, 'tw_walk_adds') for x in xs)
        if a.endswith('sib8'):
            for k in ('tw_sib_parents', 'tw_sib_pairs', 'tw_walk_invs', 'tw_walk_muls', 'tw_walk_sqrs', 'tw_sib_resolutions', 'tw_sib_degenerate'):
                d[k] = st.mean(pn(x, k) for x in xs)
        d['sib_second_points'] = st.mean(
            sum(p.get('work_native', {}).get('sibling_second_points', 0) for p in x['phases']) for x in xs)
        # bit-ops by phase
        ph = {}
        for x in xs:
            for p in x['work']['phases']:
                ph[p['name']] = ph.get(p['name'], 0) + int(p['bitops'])
        d['bitops_phase_per_sqrt_r'] = {k: v / len(xs) / sq for k, v in ph.items()}
    out['arms'][a] = d
for row in band['rows']:
    out['arms'][row['arm']]['S'] = {str(w): s for w, s in zip(band['weights'], row['mean_s'])}
random.seed(20261110)
def ratio(num, den, f):
    ks = sorted(set(by[num]) & set(by[den]))
    def est(sample):
        return st.mean(f(by[num][k]) for k in sample) / st.mean(f(by[den][k]) for k in sample)
    pt = est(ks)
    bs = sorted(est([random.choice(ks) for _ in ks]) for _ in range(4000))
    return pt, bs[100], bs[3899]
bo = lambda x: x['work']['bitops_per_sqrt_r']
out['ratios'] = {
    'bitops sib8/tw2cj-lazy': ratio('tw2cj-lazy-sib8', 'tw2cj-lazy', bo),
    'bitops sib8/rho-neg': ratio('tw2cj-lazy-sib8', 'rho-neg', bo),
    'bitops tw2cj-lazy/rho-neg': ratio('tw2cj-lazy', 'rho-neg', bo),
    'stored sib8/tw2cj-lazy': ratio('tw2cj-lazy-sib8', 'tw2cj-lazy', inserts),
}
# post hoc decomposition of the oracle_setup phase with the bitop/v1 prices at b = 20
# (add 260, mul = sqr 4860, group_add 26540, group_double 31920, hash_probe 80; derived from the records' own prims)
G, D, SEC, H = 26540, 31920, 4860 * 2 + 3 * 260, 80
dec = {}
for a in ('tw2cj-lazy', 'tw2cj-lazy-sib8'):
    xs = list(by[a].values())
    m = lambda f: st.mean(f(x) for x in xs) / sq
    dec[a] = {
        'pool_or_jump_setup': m(lambda x: pn(x, 'tw_setup_adds') * G + pn(x, 'tw_setup_doubles') * D),
        'walk_additions_and_pairs': m(lambda x: pn(x, 'tw_walk_adds') * G),
        'second_points': m(lambda x: sum(p.get('work_native', {}).get('sibling_second_points', 0) for p in x['phases']) * SEC),
        'sign_resolutions': m(lambda x: pn(x, 'tw_sib_resolve_adds') * G),
        'table_inserts': m(lambda x: inserts(x) * H),
        'oracle_setup_total': m(lambda x: sum(int(p['bitops']) for p in x['work']['phases'] if p['name'] == 'oracle_setup')),
    }
    dec[a]['walk_phase_total'] = sum(dec[a][k] for k in ('walk_additions_and_pairs', 'second_points', 'sign_resolutions', 'table_inserts'))
out['post_hoc_decomposition_bitops_per_sqrt_r'] = dec
out['post_hoc_walk_only_ratio_sib8_over_tw2cj_lazy'] = dec['tw2cj-lazy-sib8']['walk_phase_total'] / dec['tw2cj-lazy']['walk_phase_total']
# post hoc: walk-only bit-ops (oracle_setup minus pool set-up) -- phases other than oracle_setup are common
json.dump(out, open('smoke-stats.json', 'w'), indent=1)
print(json.dumps(out, indent=1))
