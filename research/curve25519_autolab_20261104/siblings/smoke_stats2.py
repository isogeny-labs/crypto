#!/usr/bin/env python3
"""Second-smoke statistics: smoke_stats2.py <session dir> <band json> <out json>."""
import json, math, random, statistics as st, sys
sess, bandf, outf = sys.argv[1:4]
recs = [json.loads(l) for l in open(sess + '/records.jsonl')]
recs = [r for r in recs if not r['warmup'] and r['outcome']['status'] == 'verified']
band = json.load(open(bandf))
r = int(recs[0]['workload']['curve']['r']); sq = math.sqrt(r)
arms = sorted({x['arm'] for x in recs}, key=lambda a: (a != 'rho-neg', a != 'tw2cj-lazy', a))
def pn(x, n): return sum(p.get('native', {}).get(n, 0) for p in x['phases'])
def wn(x, n): return sum(p.get('work_native', {}).get(n, 0) for p in x['phases'])
inserts = lambda x: wn(x, 'table_insert_attempts')
key = lambda x: (x['round'], x['workload']['target_index'])
by = {a: {key(x): x for x in recs if x['arm'] == a} for a in arms}
b = recs[0]['work']['widths']['field']['bits']
add = 13 * b; mul = 2 * (b * b + 5 * b * (b - 1)) + 13 * b; inv = 26 * b * b
G = inv + 2 * mul + mul + 6 * add  # group_add (sqr = mul)
# group_double price from the records' own prims
def phase(x, n): return [p for p in x['work']['phases'] if p['name'] == n][0]
x0 = by[arms[1]][next(iter(by[arms[1]]))]; pr = phase(x0, 'oracle_setup')['prims']
D = (int(phase(x0, 'oracle_setup')['bitops']) - pr.get('field_add', 0) * add - pr.get('field_mul', 0) * mul
     - pr.get('field_sqr', 0) * mul - pr.get('hash_probe', 0) * 4 * b - pr['group_add'] * G) / pr['group_double']
out = {'r': r, 'sqrt_r': sq, 'field_bits': b, 'prices': {'group_add': G, 'group_double': D}, 'arms': {}}
for a in arms:
    xs = list(by[a].values()); m = lambda f: st.mean(f(x) for x in xs)
    d = {'n': len(xs), 'bitops_per_sqrt_r': m(lambda x: x['work']['bitops_per_sqrt_r']),
         'gae_per_sqrt_r': m(lambda x: x['cost']['total_gae'] / sq)}
    if a == 'rho-neg':
        d['steps_per_sqrt_r'] = m(lambda x: x['counters']['walk_operations'] / sq)
    else:
        d['stored_points_per_sqrt_r'] = m(lambda x: inserts(x) / sq)
        d['stored_se'] = st.stdev(inserts(x) / sq for x in xs) / math.sqrt(len(xs))
        d['setup_adds'] = m(lambda x: pn(x, 'tw_setup_adds')); d['setup_doubles'] = m(lambda x: pn(x, 'tw_setup_doubles'))
        d['walk_adds'] = m(lambda x: pn(x, 'tw_walk_adds'))
        sec = lambda x: wn(x, 'sibling_second_points'); lift = lambda x: wn(x, 'sibling_lift_products')
        SEC = mul + 2 * add; LIFT = mul + 2 * add
        # second point = sqr + 2 add; sqr priced as mul
        SEC = mul + 2 * add
        parts = {'setup': m(lambda x: (pn(x, 'tw_setup_adds') * G + pn(x, 'tw_setup_doubles') * D) / sq),
                 'walk_additions': m(lambda x: pn(x, 'tw_walk_adds') * G / sq),
                 'second_points': m(lambda x: sec(x) * SEC / sq),
                 'lift_products': m(lambda x: lift(x) * LIFT / sq),
                 'sign_resolutions': m(lambda x: pn(x, 'tw_sib_resolve_adds') * G / sq),
                 'table_inserts': m(lambda x: inserts(x) * 4 * b / sq)}
        parts['walk_total'] = sum(v for k, v in parts.items() if k != 'setup')
        parts['setup_share_of_oracle_setup'] = parts['setup'] / (parts['setup'] + parts['walk_total'])
        d['post_hoc_bitops_per_sqrt_r'] = parts
    d['S'] = dict(zip(map(str, band['weights']), next(rw['mean_s'] for rw in band['rows'] if rw['arm'] == a)))
    out['arms'][a] = d
random.seed(20261110)
def ratio(num, den, f):
    ks = sorted(set(by[num]) & set(by[den]))
    est = lambda s: st.mean(f(by[num][k]) for k in s) / st.mean(f(by[den][k]) for k in s)
    bs = sorted(est([random.choice(ks) for _ in ks]) for _ in range(4000))
    return [est(ks), bs[100], bs[3899]]
bo = lambda x: x['work']['bitops_per_sqrt_r']
walk = lambda x: (pn(x, 'tw_walk_adds') * G + wn(x, 'sibling_second_points') * (mul + 2 * add) + wn(x, 'sibling_lift_products') * (mul + 2 * add) + pn(x, 'tw_sib_resolve_adds') * G + inserts(x) * 4 * b)
out['ratios'] = {}
for a in arms:
    if a.startswith('tw2cj-lazy-sib'):
        out['ratios'][f'bitops {a}/tw2cj-lazy'] = ratio(a, 'tw2cj-lazy', bo)
        out['ratios'][f'bitops {a}/rho-neg'] = ratio(a, 'rho-neg', bo)
        out['ratios'][f'stored {a}/tw2cj-lazy'] = ratio(a, 'tw2cj-lazy', inserts)
        out['ratios'][f'walk-only bitops {a}/tw2cj-lazy'] = ratio(a, 'tw2cj-lazy', walk)
out['ratios']['bitops tw2cj-lazy/rho-neg'] = ratio('tw2cj-lazy', 'rho-neg', bo)
json.dump(out, open(outf, 'w'), indent=1); print(json.dumps(out, indent=1))
