"""POST HOC descriptives (not in PROTOCOL.md): propagations and wall time, UNSAT second half."""
import json, math
R = json.load(open('results.json'))['raw_rows']
out = {}
for l in (3, 4, 5):
    rows = {}
    for r in R:
        if r['l'] == l and not r['decomposable'] and r['idx'] >= 30:
            rows.setdefault(r['idx'], {})[r['arm']] = r
    gm = lambda f: round(math.exp(sum(math.log(f(v)) for v in rows.values()) / len(rows)), 3)
    o = {}
    for den in ('FRESH-A', 'FRESH-U', 'SPEC'):
        o['propagations INC/' + den] = gm(lambda v: v['INC']['propagations'] / v[den]['propagations'])
        o['decisions INC/' + den] = gm(lambda v: v['INC']['decisions'] / v[den]['decisions'])
        o['secs_sum INC/' + den] = round(sum(v['INC']['secs'] for v in rows.values()) / sum(v[den]['secs'] for v in rows.values()), 3)
    o['secs_sum'] = {a: round(sum(v[a]['secs'] for v in rows.values()), 2) for a in ('INC', 'INC2', 'FRESH-A', 'FRESH-U', 'SPEC', 'FRESH-A2')}
    o['thirds INC/BEST gm (unsat, by position 1-19,20-39,40-59)'] = []
    allr = {}
    for r in R:
        if r['l'] == l and not r['decomposable']:
            allr.setdefault(r['idx'], {})[r['arm']] = r
    for lo, hi in ((1, 20), (20, 40), (40, 60)):
        xs = [v['INC']['conflicts'] / min(v[a]['conflicts'] for a in ('FRESH-A', 'FRESH-U', 'SPEC')) for i, v in allr.items() if lo <= i < hi]
        o['thirds INC/BEST gm (unsat, by position 1-19,20-39,40-59)'].append([len(xs), round(math.exp(sum(map(math.log, xs)) / len(xs)), 3)])
    out[l] = o
json.dump(out, open('posthoc.json', 'w'), indent=1); print(json.dumps(out, indent=1))
