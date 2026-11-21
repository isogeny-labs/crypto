#!/usr/bin/env python3
"""P1 stage 1 scorer: q/2^k = (|M_<=D(k)| - dim V_D(k)) / 2^k per cell from the growth-ideal JSON records."""
import json, glob, sys
from math import comb
cols = lambda k, D: sum(comb(2 * k, j) for j in range(D + 1))
committed = {(3, 2): 2.500, (3, 3): 4.375, (3, 4): 5.250, (3, 5): 5.156, (4, 6): 2.953, (4, 7): 5.016, (4, 8): 5.688, (4, 9): 5.365}
for f in sorted(glob.glob('p1/ideal-D*.json')):
    rows = json.load(open(f)); rows = rows if isinstance(rows, list) else rows.get('rows', rows)
    for r in rows:
        D, k = r['D'], r['k']; q = cols(k, D) - r['dim_V']; ratio = q / 2 ** k
        ref = committed.get((D, k)); dev = f"{100 * (ratio / ref - 1):+.1f}%" if ref else ''
        print(f"{f.split('/')[-1]:34} D={D} k={k} cols={cols(k, D):6} dimV={r['dim_V']:6} dimS={r.get('dim_S','?'):>6} 1inV={str(r['one_in_V']):5} q/2^k={ratio:6.3f} {('ref ' + str(ref) + ' ' + dev) if ref else ''}")
