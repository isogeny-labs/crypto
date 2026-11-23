#!/usr/bin/env python3
"""Summarise out/c/emin_n*.jsonl: one line per instance."""
import glob, json, re
rows = []
for f in sorted(glob.glob('out/c/emin_n*.jsonl'), key=lambda p: int(re.findall(r'\d+', p)[-1])):
    for l in open(f):
        d = json.loads(l); r = d['result']
        last = r['tests'][-1] if r['tests'] else {}
        rows.append((r['n'], r['k'], d['seed'], r.get('e_min'), r.get('e_min_gt'), last.get('cols'), d['wall_s']))
print(f"{'n':>3} {'k':>3} {'seed':>4} {'e_min':>6} {'e_min>':>6} {'last cols':>9} {'wall s':>8}")
for n, k, s, em, gt, c, w in rows:
    print(f"{n:>3} {k:>3} {s:>4} {str(em):>6} {str(gt):>6} {str(c):>9} {w:>8.1f}")
