#!/usr/bin/env python3
"""Tabulate out/cells.jsonl (+ out/cells_extra.jsonl) against the frozen L3/L4 formulas (PREREG.md)."""
import json
def rows(f):
    try: return [json.loads(l) for l in open(f)]
    except FileNotFoundError: return []
def verdict(d):
    n, D, W = d['n'], d.get('Dreq', d.get('D')), d.get('W_dims')
    if W is None: return 'ERROR ' + d.get('error', '')
    if D == 3:
        p2, p3 = 3 * n, n * (5 * n + 1) / 2
        ok = (W[2] == p2 and W[3] == p3)
        return f"pred ({p2}, {p3:g}) {'MATCH' if ok else 'DEVIATES'}"
    if D == 4:
        if d['one_in']: return 'fell (1 in W_4): not a stalled instance'
        p2, p3 = 3 * n, 3 * n * (n + 2)
        ok = (W[2] == p2 and W[3] == p3)
        return f"pred ({p2}, {p3}) {'MATCH' if ok else 'DEVIATES'}"
print('| tag | exe | type | n | k | N | seed | sols | D | W_dims (cum. e=0..D) | codim | sec | maxRSS MB | vs L3/L4 formula |')
print('|:--|:--|:--|--:|--:|--:|--:|--:|--:|:--|--:|--:|--:|:--|')
for d in rows('out/cells.jsonl') + rows('out/cells_extra.jsonl'):
    d.setdefault('n', int(d['line'].split()[0])); d.setdefault('k', int(d['line'].split()[1]))
    print(f"| {d.get('tag')} | {d.get('exe','fallclose')} | {d.get('type')} | {d['n']} | {d.get('k','')} | {d.get('N','')} | {d.get('seed')} | {d.get('sols')} | {d.get('Dreq')} | {d.get('W_dims')} | {d.get('codim','')} | {d.get('sec','')} | {round(d['maxrss_kb']/1024) if 'maxrss_kb' in d else ''} | {verdict(d)} |")
