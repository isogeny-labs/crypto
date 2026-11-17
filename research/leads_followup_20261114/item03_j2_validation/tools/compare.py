#!/usr/bin/env python3
"""Validator comparator (own code). Compares solver record files field by field.
usage: compare.py <mode> ...
  pair A.jsonl B.jsonl            -> prints IDENTICAL or the differing fields
Compared: every key except timing/host keys. Ignored: keys ending 'wall' or 'cpu', load1, search_s, s.controls_*,
and, for rec == 'ratios', r_inv and r_lmul (timing ratios)."""
import json, sys
def recs(path):
    out = []
    for l in open(path):
        l = l.strip()
        if l.startswith('{'): out.append(json.loads(l))
    return out
def volatile(rec, k, extra=()):
    if k in extra: return True
    if k.endswith('wall') or k.endswith('cpu') or k in ('load1', 'search_s') or k.startswith('s.controls_'): return True
    if rec.get('rec') == 'ratios' and k in ('r_inv', 'r_lmul'): return True
    return False
def diff(ra, rb, extra=()):
    """returns (n_records_a, n_records_b, n_fields_compared, [differences])"""
    d = []; nf = 0
    if len(ra) != len(rb): d.append(f'record count {len(ra)} vs {len(rb)}')
    for i, (x, y) in enumerate(zip(ra, rb)):
        keys = sorted(set(x) | set(y))
        for k in keys:
            if volatile(x, k, extra) and volatile(y, k, extra): continue
            nf += 1
            if x.get(k, '<absent>') != y.get(k, '<absent>'):
                d.append(f'rec#{i} arm={x.get("arm")} att={x.get("attempt")} key={k}: {x.get(k, "<absent>")} vs {y.get(k, "<absent>")}')
    return len(ra), len(rb), nf, d
if __name__ == '__main__':
    if sys.argv[1] == 'pair':
        na, nb, nf, d = diff(recs(sys.argv[2]), recs(sys.argv[3]))
        print('IDENTICAL' if not d else 'DIFFERS', na, nb, nf)
        for x in d[:20]: print('  ', x)
