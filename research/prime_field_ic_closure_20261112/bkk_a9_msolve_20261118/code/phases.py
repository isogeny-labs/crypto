"""Exploratory (not in the preset): msolve phase split per run from the -v 2 log TIMINGS block."""
import re, sys, json, glob, os, collections
rows = collections.defaultdict(list)
for f in sorted(glob.glob('/dev/shm/bkk/runs/log/m*_R_*.log')):
    t = open(f).read()
    tag = os.path.basename(f)[:-4]
    m, b, i, _, arm = tag.split('_')
    def g(k):
        x = re.findall(re.escape(k) + r'\s+([\d.]+) sec', t)
        return float(x[-1]) if x else None
    rows[(m, b, arm)].append(dict(overall=g('overall(elapsed)'), symb=g('symbolic prep.'), la=g('linear algebra'),
                                  update=g('update'), terms=re.findall(r'#terms in basis\s+(\d+)', t)[-1:] or None))
for k, v in sorted(rows.items(), key=lambda kv: (kv[0][0], int(kv[0][1][1:]), kv[0][2])):
    ok = [r for r in v if r['overall']]
    if not ok: print(k, 'no TIMINGS (unfinished)'); continue
    s = lambda key: round(sum(r[key] or 0 for r in ok) / len(ok), 2)
    print(*k, 'n', len(ok), 'overall', s('overall'), 'symbolic', s('symb'), 'linalg', s('la'), 'update', s('update'))
