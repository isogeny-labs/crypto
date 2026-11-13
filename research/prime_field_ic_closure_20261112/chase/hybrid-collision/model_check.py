"""Check the explicit-table cost model C = T + h*n/(2T) (w=0) against measured
group ops, and HYB-U termination cause. [pcc:unverified]"""
import json, sys, math
from collections import defaultdict
rows = [json.loads(l) for f in sys.argv[1:] for l in open(f)]
g = defaultdict(list)
for r in rows:
    if r['arm'] == 'rho': continue
    g[(r['bits'], r['arm'], r['cfg'], r['neg'])].append(r)
print('| bits | family | walk | V | m | T | h(1dep) | model T+h n/(2T) / sqrt n | measured IC-1dep gops / sqrt n | HYB-U ended by dep (of N) |')
print('|---|---|---|---|---|---|---|---|---|---|')
for k in sorted(g):
    rs = g[k]
    if not all('dep' in r['events'] for r in rs): continue
    mod = sum((r['table_size'] + r['events']['dep']['hits'] * r['n'] / (2 * r['table_size'])) / math.sqrt(r['n']) for r in rs) / len(rs)
    mea = sum(r['events']['dep']['gops'] / math.sqrt(r['n']) for r in rs) / len(rs)
    bydep = sum(1 for r in rs if 'dep' in r['events'] and ('rho' not in r['events'] or r['events']['dep']['steps'] <= r['events']['rho']['steps']))
    V = sum(r['V'] for r in rs)/len(rs); m = sum(r['m'] for r in rs)/len(rs); T = sum(r['table_size'] for r in rs)/len(rs)
    h = sum(r['events']['dep']['hits'] for r in rs)/len(rs)
    print(f"| {k[0]} | {k[1]} | {'neg' if k[3] else 'plain'} | {V:.0f} | {m:.0f} | {T:.0f} | {h:.1f} | {mod:.2f} | {mea:.2f} | {bydep}/{len(rs)} |")
