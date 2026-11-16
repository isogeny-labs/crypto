#!/usr/bin/env python3
"""Lower-degree part of LEAD-A-fall-effective-variable-law: d_F (least D with 1 in W_D) for
R-type and T-type unsatisfiable instances at small n, k-scan.  Rule fixed BEFORE running:
per (type, n, k) take seeds 1.. until 2 non-degenerate unsatisfiable instances (max 80 seeds);
'degenerate' = 1 already in span(F) (span_dims[0] = 1, d_F <= 1), reported but kept out of the
monotonicity test because the lead's mechanism assumes span(F) contains no constant.
D tested 2..DMAX (DMAX = 6 for N <= 14, else 5).  Output: out/lowdeg.jsonl"""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
L = os.path.join(HERE, '..', '..', '..', 'lfd_lower_bound_20261111')
FC = os.path.join(HERE, 'fallclose'); EMIN = os.path.join(L, 'emin')
def sh(cmd, inp=None):
    return subprocess.run(cmd, input=inp, capture_output=True, text=True, check=True).stdout.strip()
def line(typ, n, s, k):
    return sh([sys.executable, os.path.join(L, 'emit_instance.py' if typ == 'R' else 'emit_instance_T.py'), str(n), str(s), str(k)])
out = open(os.path.join(HERE, 'out', 'lowdeg.jsonl'), 'a')
for n in (13, 15, 17):
    for typ in ('R', 'T'):
        for k in range(3, 10):
            N = 2 * k
            if N > 18 or k >= n: continue
            dmax = 6 if N <= 14 else 5
            got = 0
            for s in range(1, 81):
                if got >= 2: break
                ln = line(typ, n, s, k)
                if int(sh([EMIN, 'count'], ln)) != 0: continue
                sp = json.loads(sh([FC, '2', '64', '2'], ln))['span_dims']
                rec = dict(type=typ, n=n, k=k, N=N, v=(N if typ == 'T' else N - 1), seed=s, degenerate=sp[0] == 1)
                if sp[0] == 1:
                    rec['dF'] = '<=1'
                else:
                    dF = None
                    for D in range(2, dmax + 1):
                        c = json.loads(sh([FC, str(D), '8192', '0'], ln))
                        if c['one_in']: dF = D; break
                    rec['dF'] = dF if dF is not None else '>%d' % dmax
                    got += 1
                print(json.dumps(rec), file=out, flush=True); print(json.dumps(rec), flush=True)
