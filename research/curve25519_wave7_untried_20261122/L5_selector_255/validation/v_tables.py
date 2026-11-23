#!/usr/bin/env python3
"""Validator: compare every number in the README/tables.md gain tables with the validator's recomputation."""
import sys, json, re, os
BASE = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L5_selector_255'
res = json.load(open(sys.argv[1]))
g = res['gains']; meta = res['meta']; Imeta = res['Imeta']

def get(arm, cur, t, s, v, K):
    return g.get('|'.join(map(str, [arm, cur, t, s, v, K])))

def num(cell):
    m = re.match(r'\s*([\d.]+)(?:\s*\[([\d.]+),\s*([\d.]+)\])?\s*$', cell)
    if not m: return None
    return [float(x) if x else None for x in m.groups()]

def cmp3(stated, val, nd=3):
    return abs(round(val, nd) - stated) <= 0.5 * 10 ** (-nd) + 1e-12

def check_file(path):
    txt = open(path).read()
    issues = []; ncheck = 0
    blocks = re.split(r'\n(?=\*\*)', txt)
    for blk in blocks:
        head = blk.split('\n', 1)[0]
        lines = [l for l in blk.split('\n') if l.startswith('|') and not l.startswith('|--') and not l.startswith('| t ') and not l.startswith('|---')]
        if head.startswith('**k20**') or head.startswith('**k24f**') or head.startswith('**k24r**'):
            cur = head[2:].split('**')[0]
            for l in lines:
                c = [x.strip() for x in l.strip('|').split('|')]
                if not c[0].isdigit(): continue
                t = int(c[0])
                spec = [('meanc', None), (('J', 'J', 2000), True), (('J', 'J', 100), False), (('c', 'J', 2000), False), (('S1', 'J', 2000), False),
                        (('Phi', 'Phi', 2000), False), (('J', 'covered', 2000), True), (('surrogate', 'surrJ', 2000), False), (('decoy', 'J', 2000), False)]
                for j, (what, has_int) in enumerate(spec, start=1):
                    cell = c[j]
                    if what == 'meanc':
                        ncheck += 1
                        if int(cell) != round(meta[f'{cur}_t{t}']['mean_c']):
                            issues.append((path, cur, t, 'mean c', cell, meta[f'{cur}_t{t}']['mean_c']))
                        continue
                    if cell == '–':
                        if get('F', cur, t, *what) is not None: issues.append((path, cur, t, what, 'dash but value exists'))
                        continue
                    v = num(cell); rec = get('F', cur, t, *what)
                    ncheck += 1
                    if rec is None: issues.append((path, cur, t, what, 'no recomputed value')); continue
                    if not cmp3(v[0], rec[0]): issues.append((path, cur, t, what, cell, rec))
                    if has_int:
                        if v[1] is None or not (cmp3(v[1], rec[1]) and cmp3(v[2], rec[2])): issues.append((path, cur, t, what, 'interval', cell, rec))
        if head.startswith('**Seed selector'):
            for l in lines:
                c = [x.strip() for x in l.strip('|').split('|')]
                if not c[0].isdigit(): continue
                t = int(c[0])
                for off, cur in ((1, 'k20'), (4, 'k24r')):
                    ncheck += 1
                    if abs(float(c[off]) - round(Imeta[f'{cur}_t{t}']['mean_cols'], 1)) > 1e-9:
                        issues.append((path, 'seed', cur, t, 'mean cols', c[off], Imeta[f'{cur}_t{t}']['mean_cols']))
                    for j, v_ in ((off + 1, 'J'), (off + 2, 'covered')):
                        v = num(c[j]); rec = get('I', cur, t, 'J', v_, 500); ncheck += 1
                        if not (cmp3(v[0], rec[0]) and cmp3(v[1], rec[1]) and cmp3(v[2], rec[2])):
                            issues.append((path, 'seed', cur, t, v_, c[j], rec))
        if head.startswith('**Real Curve25519'):
            for l in lines:
                c = [x.strip() for x in l.strip('|').split('|')]
                if not c[0].isdigit(): continue
                t = int(c[0])
                for j, key in ((2, ('R', 'curve25519')), (3, ('F', 'k24f')), (4, ('F', 'k20'))):
                    v = num(c[j]); rec = get(*key, t, 'J', 'J', 200); ncheck += 1
                    if not (cmp3(v[0], rec[0]) and cmp3(v[1], rec[1]) and cmp3(v[2], rec[2])):
                        issues.append((path, 'real', t, key, c[j], rec))
                ncheck += 1
                if int(c[1]) != res['R1'][f'r1_t{t}']['N']: issues.append((path, 'real frames', t, c[1]))
    return ncheck, issues

for p in ('README.md', 'tables.md'):
    n, iss = check_file(os.path.join(BASE, p))
    print(p, 'cells checked', n, 'issues', len(iss))
    for x in iss: print('  ', x)
