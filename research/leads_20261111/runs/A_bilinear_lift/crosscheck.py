#!/usr/bin/env python3
"""Cross-check polyclose (native) against lfd_small.closure (legacy Python) on an emitted system.
Usage: emit_lift.py ... | crosscheck.py D   -> one JSON line with both answers and 'agree'."""
import json, os, subprocess, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'lfd_lower_bound_20261111'))
from lfd_small import closure  # noqa: E402

D = int(sys.argv[1])
txt = sys.stdin.read()
tok = txt.split()
NV, NP = int(tok[0]), int(tok[1])
p = 2
gens = []
for _ in range(NP):
    T = int(tok[p]); p += 1
    acc = {}
    for _ in range(T):
        d = int(tok[p]); p += 1
        m = 0
        for j in range(d):
            m |= 1 << int(tok[p + j])
        p += d
        acc[m] = acc.get(m, 0) ^ 1
    gens.append([m for m, c in acc.items() if c])
one, dim, cum = closure(gens, NV, D)
here = os.path.dirname(os.path.abspath(__file__))
nat = subprocess.run([os.path.join(here, 'polyclose'), str(D), '8192', '1'], input=txt, capture_output=True, text=True, check=True)
nj = json.loads(nat.stdout)
print(json.dumps(dict(NV=NV, D=D, py_one=bool(one), py_dims=cum, nat_one=bool(nj['one_in']), nat_dims=nj['W_dims'],
                      agree=(bool(one) == bool(nj['one_in']) and (bool(one) or cum == nj['W_dims'])))), flush=True)
# lfd_small stops early on a fall, so dims are compared on stalls only
