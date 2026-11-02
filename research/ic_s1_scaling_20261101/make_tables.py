#!/usr/bin/env python3
"""markdown tables from s1_analyze.py json.  usage: make_tables.py <analysis.json>"""
import json, sys
d = json.load(open(sys.argv[1])); C = d['cells']
order = ['K0 n=9', 'K1 n=9', 'K1 n=11', 'K0 n=13', 'K0 n=13 s2', 'K0 n=15', 'K1 n=15', 'K1 n=17', 'K0 n=19', 'K1 n=19']
def g(e, k, fmt='%.3g'): return fmt % e[k] if k in e else '-'
print('| cell | r | best lab l, Ir | A: whole l, Ir | ratio | A: solver-only l, Ir | ratio | D: whole l, Ir | ratio | D: solver-only ratio |')
print('|:--|--:|:--|:--|--:|:--|--:|:--|--:|--:|')
for c in order:
    r = C[c]; A = r['cfg']['A']; D = r['cfg']['D']
    print(f"| {c} | {r['r']} | l{r.get('best_lab','-')}, {r.get('lab_ir',0):.3g} | " +
          (f"l{A['best_whole_l']}, {A['wd_whole']:.3g} | {A['ratio_whole']:.2f} | l{A['best_lower_l']}, {A['wd_lower']:.3g} | {A['ratio_lower']:.2f} | " if 'best_whole_l' in A else '- | - | - | - | ') +
          (f"l{D['best_whole_l']}, {D['wd_whole']:.3g} | {D['ratio_whole']:.2f} | {D['ratio_lower']:.2f} |" if 'best_whole_l' in D else '- | - | - |'))
print()
for c in order:
    r = C[c]
    print(f"- {c}: lab " + ', '.join(f'l{l} {v:.3g}' for l, v in r.get('lab_curve', {}).items()))
    for cfg in 'ABCD':
        e = r['cfg'][cfg]
        if e.get('curve'): print(f"  - {cfg}: " + ', '.join(f"l{l} {('%.3g' % v) if v else 'failed'}" for l, v in sorted(e['curve'].items(), key=lambda kv: int(kv[0]))))
