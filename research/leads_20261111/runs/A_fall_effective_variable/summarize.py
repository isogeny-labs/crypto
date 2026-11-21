#!/usr/bin/env python3
"""Tables for RESULT.md from out/cells.jsonl and out/<lowdeg file> (prints Markdown).
Usage: summarize.py [lowdeg.jsonl | lowdeg_followup.jsonl]"""
import json, collections, os
H = os.path.dirname(os.path.abspath(__file__))
print('### D = 4 cells and controls\n')
print('| tag | type | n | k | N | v | n_eq (nonconstant) | seed | sols | 1 in W_4 | W_dims (at stop) | ncols | codim | s | RSS MB |')
print('|:--|:--|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|--:|--:|--:|')
for l in open(os.path.join(H, 'out', 'cells.jsonl')):
    d = json.loads(l)
    if 'error' in d:
        print(f"| {d['tag']} | {d['type']} | {d['n']} | {d['k']} | | | | {d['seed']} | | {d['error']} | | | | | |"); continue
    v = d['N'] if d['type'] == 'T' else d['N'] - 1
    print(f"| {d['tag']} | {d['type']} | {d['n']} | {d['k']} | {d['N']} | {v} | {d['gen_rank']} | {d['seed']} | {d['sols']} | "
          f"{'yes (fall)' if d['one_in'] else 'no (stall)'} | {tuple(d['W_dims'])} | {d['ncols']} | {d['codim']} | {d['sec']} | {d['maxrss_kb']//1024} |")
print('\n### Lower degrees: d_F by (n, family, k), non-degenerate unsatisfiable instances\n')
import sys
LOW = sys.argv[1] if len(sys.argv) > 1 else 'lowdeg.jsonl'
t = collections.defaultdict(list); deg = collections.Counter()
for l in open(os.path.join(H, 'out', LOW)):
    d = json.loads(l)
    if d['degenerate']: deg[(d['n'], d['type'], d['k'])] += 1; continue
    t[(d['n'], d['type'])].append((d['k'], d['v'], d['dF']))
print('| n | family | (k, v, d_F) per instance | monotone in v |')
print('|--:|:--|:--|:--|')
for key in sorted(t):
    rows = sorted(t[key]); mono = all(a[2] <= b[2] for a in rows for b in rows if a[1] < b[1])
    print(f"| {key[0]} | {key[1]} | {' '.join(f'({k},{v},{f})' for k, v, f in rows)} | {mono} |")
print('\nPooled R+T at fixed n (v -> d_F values):\n')
for n in sorted({k[0] for k in t}):
    pool = collections.defaultdict(set)
    for fam in 'RT':
        for k, v, f in t.get((n, fam), []): pool[v].add(f)
    vs = sorted(pool); mono = all(max(pool[a]) <= min(pool[b]) for a in vs for b in vs if a < b)
    print(f"- n={n}: " + ', '.join(f"v={v}:{sorted(pool[v])}" for v in vs) + f"  -> pooled monotone: {mono}")
print('\nDegenerate unsatisfiable instances (1 in span F), excluded by the pre-run rule:', dict(deg))
