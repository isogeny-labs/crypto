"""Aggregate raw/arms_*.jsonl -> results table + slope of log2(work) vs l with bootstrap CI.
Classification: an instance is UNSAT/SAT by the WDSat -b verdict (reference); every arm that finished must agree.
Work metric per cell = sum of conflicts over the instances (relation collection pays the total); also CPU seconds.
Slope: OLS of log2(total work over finished instances / #instances) on l; CI: bootstrap over instances within each l (2000 draws)."""
import json, glob, math, random, sys
from collections import defaultdict
rows = [json.loads(l) for f in sorted(glob.glob('raw/arms_*.jsonl')) for l in open(f)]
by = defaultdict(dict)
for r in rows:
    by[r['name']][r['arm']] = r
cls = {}
for nm, d in by.items():
    ref = d.get('wd_b')
    if ref and ref['status'] in ('SAT', 'UNSAT'):
        cls[nm] = ref['status']
# agreement check
dis = []
for nm, d in by.items():
    for a, r in d.items():
        if r['status'] in ('SAT', 'UNSAT') and nm in cls and r['status'] != cls[nm]:
            dis.append((nm, a, r['status'], cls[nm]))
print('status disagreements vs wd_b:', dis)
arms = ['wd_b', 'wd_bx', 'cms_std', 'cms_gad', 'cms_std_gj', 'cms_gad_gj']
ls = sorted({r['l'] for r in rows})
def slope(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
out = {}
for metric in ('conflicts', 'cpu_s'):
    for cl in ('UNSAT', 'SAT'):
        print(f'\n=== {cl} cells, metric={metric}: mean per instance (n finished / n total; TO=timeouts) ===')
        for arm in arms:
            line = []; pts = {}
            for l in ls:
                vals = []; tot = 0; to = 0
                for nm, d in by.items():
                    if cls.get(nm) != cl or d.get(arm) is None or d[arm]['l'] != l:
                        continue
                    tot += 1
                    r = d[arm]
                    if r['status'] in ('SAT', 'UNSAT') and r.get(metric) is not None:
                        vals.append(r[metric])
                    else:
                        to += 1
                if vals:
                    pts[l] = vals
                line.append(f"l={l}: {sum(vals)/len(vals):10.1f} ({len(vals)}/{tot}{', TO='+str(to) if to else ''})" if vals else f"l={l}: --")
            print(f'{arm:8s}', ' | '.join(line))
            for lset, nm_ in (([5, 6, 7], 'l5-7'), (sorted(pts), 'all')):
                if all(l in pts for l in lset) and len(lset) >= 3:
                    rng = random.Random(1)
                    def est(samp):
                        return slope(lset, [math.log2(sum(samp[l]) / len(samp[l])) for l in lset])
                    s0 = est(pts)
                    bs = sorted(est({l: [rng.choice(pts[l]) for _ in pts[l]] for l in lset}) for _ in range(2000))
                    print(f'   slope log2(mean {metric}) vs l [{nm_}] = {s0:.3f}  95% CI [{bs[50]:.3f}, {bs[1949]:.3f}]')
                    out[f'{cl}|{metric}|{arm}|{nm_}'] = dict(slope=s0, lo=bs[50], hi=bs[1949], ls=lset, n={l: len(pts[l]) for l in lset})
json.dump(out, open('raw/slopes.json', 'w'), indent=1)

# paired ratios gad/std on instances where both finished
for a, b in (('cms_gad', 'cms_std'), ('cms_gad_gj', 'cms_std_gj')):
    print(f'\n=== paired ratio {a}/{b} (sum over matched finished instances) ===')
    for cl in ('UNSAT', 'SAT'):
        for l in ls:
            m = [(d[a], d[b]) for nm, d in by.items() if cls.get(nm) == cl and a in d and b in d and d[a]['l'] == l
                 and d[a]['status'] in ('SAT', 'UNSAT') and d[b]['status'] in ('SAT', 'UNSAT')]
            if m:
                rc = sum(x['conflicts'] for x, y in m) / max(1, sum(y['conflicts'] for x, y in m))
                rt = sum(x['cpu_s'] for x, y in m) / max(1e-9, sum(y['cpu_s'] for x, y in m))
                print(f'{cl:5s} l={l} n={len(m)}  conflicts ratio {rc:7.2f}  cpu ratio {rt:8.2f}')
