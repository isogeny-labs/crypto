import json, math, collections, glob, os
D = os.path.dirname(os.path.abspath(__file__))
sat = []
for f in sorted(glob.glob(D + '/sat_l*.json')):
    sat += json.load(open(f))
alg = json.load(open(D + '/algebra_rows.json'))
sym = json.load(open(D + '/symbolic_e.json'))
gm = lambda v: math.exp(sum(math.log(max(x, 1)) for x in v) / len(v))


def fit(pts):
    xs, ys = zip(*pts)
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in pts) / sum((x - mx) ** 2 for x in xs)


summary = {'sat_cells': [], 'slopes': {}, 'ratios': [], 'algebra': []}
g = collections.defaultdict(list)
for r in sat:
    g[(r['solver'], r['fresh'], r['l'], r['truth_sat'], r['arm'], r['k'])].append(r)
print('solver fresh l truth arm k | N | gm conflicts | gm decisions | gm props | gm time s')
for key in sorted(g, key=str):
    v = g[key]
    row = dict(solver=key[0], fresh=key[1], l=key[2], truth_sat=key[3], arm=key[4], k=key[5], N=len(v),
               gm_time=gm([x['time'] * 1e6 for x in v]) / 1e6)
    if key[0] == 'cadical153':
        row.update(gm_conflicts=gm([x['conflicts'] for x in v]), gm_decisions=gm([x['decisions'] for x in v]),
                   gm_propagations=gm([x['propagations'] for x in v]))
    summary['sat_cells'].append(row)
    print(*key, '|', len(v), '|', round(row.get('gm_conflicts', 0)), '|', round(row.get('gm_decisions', 0)), '|',
          round(row.get('gm_propagations', 0)), '|', round(row['gm_time'], 3))

# paired ratios xor/lit per instance
print('\npaired ratio xor-k / lit-k total conflicts (cadical, incremental unless fresh)')
by = {(r['solver'], r['fresh'], r['seed'], r['arm'], r['k']): r for r in sat}
rat = collections.defaultdict(list)
for (solver, fresh, seed, arm, k), r in by.items():
    if arm == 'xor':
        o = by[(solver, fresh, seed, 'lit', k)]
        m = 'conflicts' if solver == 'cadical153' else 'time'
        rat[(solver, fresh, r['l'], r['truth_sat'], k, m)].append(r[m] / max(o[m], 1e-9))
for key in sorted(rat, key=str):
    v = rat[key]
    row = dict(solver=key[0], fresh=key[1], l=key[2], truth_sat=key[3], k=key[4], metric=key[5], N=len(v), gm_ratio_xor_over_lit=gm_r if (gm_r := math.exp(sum(map(math.log, v)) / len(v))) else 0,
               min=min(v), max=max(v))
    summary['ratios'].append(row)
    print(*key, '| N', len(v), '| gm', round(row['gm_ratio_xor_over_lit'], 3), 'min', round(min(v), 3), 'max', round(max(v), 3))

# slopes, UNSAT targets, incremental cadical; arms: plain, lit-l, xor-l, lit-2, xor-2
print('\nslopes (bits per l), truth-UNSAT, cadical incremental, log2 gm conflicts')
cells = {(c['l'], c['arm'], c['k']): c for c in summary['sat_cells'] if c['solver'] == 'cadical153' and not c['fresh'] and not c['truth_sat']}
ls = sorted({k[0] for k in cells})
for name, sel in [('plain', lambda l: ('plain', 0)), ('lit-k=l', lambda l: ('lit', l)), ('xor-k=l', lambda l: ('xor', l)),
                  ('lit-k=2', lambda l: ('lit', 2)), ('xor-k=2', lambda l: ('xor', 2))]:
    pts = [(l, math.log2(cells[(l,) + sel(l)]['gm_conflicts'])) for l in ls if (l,) + sel(l) in cells]
    if len(pts) >= 2:
        summary['slopes'][name] = dict(points=pts, bits_per_l=fit(pts))
        print(name, [(l, round(y, 2)) for l, y in pts], 'slope', round(fit(pts), 3))
for name, f in [('enumeration 2^{3l}/6', lambda l: 3 * l - math.log2(6)), ('MITM 2^{2l}/2', lambda l: 2 * l - 1), ('linear 2+1 scan 2^l', lambda l: l)]:
    summary['slopes'][name] = dict(points=[(l, f(l)) for l in ls], bits_per_l=fit([(l, f(l)) for l in ls]))
    print(name, [(l, round(f(l), 2)) for l in ls], 'slope', round(fit([(l, f(l)) for l in ls]), 3))

a = collections.defaultdict(list)
for r in alg:
    a[(r['l'], r['n'], r['kind'], r['k'])].append(r)
for key in sorted(a):
    v = a[key]
    summary['algebra'].append(dict(l=key[0], n=key[1], kind=key[2], k=key[3], N=len(v), nvars=v[0]['nvars'],
                                   rank_nonlin_min=min(x['rank_nonlin'] for x in v), rank_nonlin_max=max(x['rank_nonlin'] for x in v),
                                   rank_deg3_min=min(x['rank_deg3'] for x in v), rank_deg3_max=max(x['rank_deg3'] for x in v),
                                   nonlin_monos_mean=sum(x['n_nonlin_monos'] for x in v) / len(v), max_deg=max(x['max_deg'] for x in v)))
print('\nmin rank_nonlin over all xor rows with k<l:', min(r['rank_nonlin'] for r in alg if r['kind'] == 'xor' and r['k'] < r['l']),
      '; with k=l:', min(r['rank_nonlin'] for r in alg if r['kind'] == 'xor' and r['k'] == r['l']))
json.dump(dict(note='pilot, every number [pcc:unverified]', summary=summary, symbolic_e=sym, sat_rows=sat, algebra_rows=alg),
          open(D + '/results.json', 'w'))
