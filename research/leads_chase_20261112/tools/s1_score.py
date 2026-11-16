#!/usr/bin/env python3
"""S1 scorer: per (arm, modulus, instance, k) the first D with 1 in V_D against the law floor((k+14)/4); cell readings."""
import json, glob, sys
from collections import defaultdict
law = lambda k: (k + 14) // 4
cells = defaultdict(dict)   # (arm, p, a, b, t, k) -> {D: one_in_V}
src = defaultdict(set)
for l in open('s1/small.jsonl'):
    r = json.loads(l)
    if 'one_in_V' in r: cells[(r['arm'], r['p'], r['a'], r['b'], r['t'], r['k'])][r['D']] = r['one_in_V']; src[(r['arm'], r['p'])].add('growth')
for f in glob.glob('s1/bigsmall/*.json') + glob.glob('s1/big/*.json'):
    r = json.load(open(f)); arm = f.split('/')[-1].split('-')[0]
    if arm == 'test': continue
    key = (arm if arm != 'b' else 'b', r['p'], r['a'], r['b'], r['t'], r['k'])
    prev = cells[key].get(r['D'])
    if prev is not None and prev != r['one_in_V']: print('ENGINE DISAGREEMENT', key, r['D'], prev, r['one_in_V'], f)
    cells[key][r['D']] = r['one_in_V']; src[(arm, r['p'])].add('big')
def first(Ds):
    ins = [D for D in sorted(Ds) if Ds[D]]
    return min(ins) if ins else None
summary = defaultdict(list)
for key in sorted(cells):
    arm, p, a, b, t, k = key; Ds = cells[key]; fd = first(Ds); L = law(k)
    maxD = max(Ds)
    if fd is not None: reading = 'below law' if fd < L else ('law' if fd == L else 'above law')
    else: reading = 'law-consistent (1 not in V_%d)' % maxD if maxD < L else ('above law (1 not in V_%d >= law %d)' % (maxD, L))
    summary[(arm, p, k)].append((fd, maxD, reading))
print(f"{'arm':4} {'p':>6} {'k':>2} {'law':>3} | first D with 1 in V per instance (None = not within D<=max) | readings")
for (arm, p, k), lst in sorted(summary.items()):
    print(f"{arm:4} {p:>6} {k:>2} {law(k):>3} | {[x[0] for x in lst]} (max D {[x[1] for x in lst]}) | {[x[2] for x in lst]}")
