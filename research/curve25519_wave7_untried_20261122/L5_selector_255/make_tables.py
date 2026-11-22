#!/usr/bin/env python3
"""Render the README gain tables from analysis/gains.csv (read-only). Usage: python3 -I make_tables.py > tables.md"""
import csv, os
here = os.path.dirname(os.path.abspath(__file__))
G = {}
for r in csv.DictReader(open(os.path.join(here, 'analysis', 'gains.csv'))):
    G[(r['arm'], r['curve'], int(r['t']), r['selector'], r['value'], int(r['K']))] = r
def f(r, ci=True):
    if r is None: return '–'
    g = float(r['gain']); s = f'{g:.3f}'
    if ci and r['lo95'] != 'nan': s += f' [{float(r["lo95"]):.3f}, {float(r["hi95"]):.3f}]'
    return s
for cur in ('k20', 'k24f', 'k24r'):
    print(f'\n**{cur}** (N = 4000 frames per t; gain = selected ÷ mean; K = 2000 unless stated; 95% calibrated bootstrap)\n')
    print('| t | mean c | J-sel, J | J-sel, J (K = 100) | c-only, J | S1, J | Φ-sel, Φ | J-sel, covered | iid surrogate | decoy |')
    print('|--:|--:|:--|--:|--:|--:|--:|:--|--:|--:|')
    for t in range(4, 15):
        r = G.get(('F', cur, t, 'J', 'J', 2000))
        if r is None: continue
        print(f"| {t} | {float(r['mean_c']):.0f} | {f(r)} | {f(G.get(('F',cur,t,'J','J',100)),False)} | {f(G.get(('F',cur,t,'c','J',2000)),False)} | "
              f"{f(G.get(('F',cur,t,'S1','J',2000)),False)} | {f(G.get(('F',cur,t,'Phi','Phi',2000)),False)} | {f(G.get(('F',cur,t,'J','covered',2000)))} | "
              f"{f(G.get(('F',cur,t,'surrogate','surrJ',2000)),False)} | {f(G.get(('F',cur,t,'decoy','J',2000)),False)} |")
print('\n**Seed selector (arm I)** (N = 1000 seeds per t; K = 500; J-selector)\n')
print('| t | k20 mean cols | k20 gain on J | k20 gain on covered | k24r mean cols | k24r gain on J | k24r gain on covered |')
print('|--:|--:|:--|:--|--:|:--|:--|')
for t in range(4, 10):
    a = G.get(('I', 'k20', t, 'J', 'J', 500)); b = G.get(('I', 'k24r', t, 'J', 'J', 500))
    print(f"| {t} | {float(a['mean_c']):.1f} | {f(a)} | {f(G.get(('I','k20',t,'J','covered',500)))} | {float(b['mean_c']):.1f} | {f(b)} | {f(G.get(('I','k24r',t,'J','covered',500)))} |")
print('\n**Real Curve25519 (arm R1) vs miniatures** (K = 200; J-selector, value J)\n')
print('| t | frames | Curve25519 | k24f | k20 |')
print('|--:|--:|:--|:--|:--|')
for t, N in ((6, 2000), (8, 1000), (10, 400)):
    print(f"| {t} | {N} | {f(G.get(('R','curve25519',t,'J','J',200)))} | {f(G.get(('F','k24f',t,'J','J',200)))} | {f(G.get(('F','k20',t,'J','J',200)))} |")
