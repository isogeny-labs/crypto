#!/usr/bin/env python3
"""POST-HOC EXPLORATORY (not in PROTOCOL.md, not gate-scored).

Question raised by the panel: program B's lattice box has volume ~81*M^8 and
its determinant is p^2, so its enumeration should grow like M^8/p^2 once
M ~ p^(1/4). F4-visit grows roughly linearly in M. Where do they cross, and
does the program-B arm stay complete under its own 100000-node cap?
Arms B and F4-visit; charts seeds 1, 2 (planted) and 11 (unplanted); one
interleaved B, F4-visit, B, F4-visit sequence per chart; M ladder below.
"""
import json, sys, hashlib
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import panel

out = open(sys.argv[1], 'a')
for M in [128, 192, 256, 320, 384, 448, 512, 640, 768, 1024, 1365]:
    for seed in (1, 2, 11):
        c = panel.chart(seed, M)
        for k in range(2):
            for arm in ('B', 'F4-visit'):
                r = panel.run_b(c, timeout=200) if arm == 'B' else panel.run_f4(c, 'visit', budget=60)
                r.update(M=M, id=c['id'], k=k)
                roots = r.pop('roots', None)
                r['n_roots'] = None if roots is None else len(roots)
                r['roots_sha'] = None if roots is None else hashlib.sha256(json.dumps(sorted(roots)).encode()).hexdigest()[:16]
                out.write(json.dumps(r) + '\n'); out.flush()
                print(M, seed, arm, round(r['wall'], 3), r.get('complete'), r.get('nodes'), r['n_roots'], flush=True)
