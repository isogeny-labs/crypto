#!/usr/bin/env python3
"""Track 1 local check [pcc:unverified]: m=2 Kummer closure V_c; multipliers h = v/f of a basis of V_c.
Reports, per c < d_F: R(c) = max popcount over the union of multiplier supports (basis-independent),
and whether the top monomial z1^{K-1} z2^{K-1} (popcount 2t) is reached. Reuses the wave-2 closure engine."""
import sys, os, json, time, numpy as np
HERE = "/home/user/crypto/research/kummer_wave2_20261115/KM3-LQR-CLOSURE"
sys.path.insert(0, HERE); os.chdir(HERE)
from hky_closure import *            # Space, closure, nsd, mod
from run_cells import fwd, inv_mod

def reach(p, t, idx, cmax=8):
    C, meta, f = nsd.make_instance(p, t, idx, 2)
    K = C.K
    coef = C.invdft(f).reshape(-1)
    finv = np.array([inv_mod(v, p) for v in f], dtype=np.int64)
    pck = C.popk
    out = []
    for c in range(2, cmax + 1):
        S = Space(C, c, False)
        g = S.vec_from_coefs([coef])
        E, piv, one = closure(S, [g], p)
        if one:
            out.append(dict(c=c, contains_one=True)); break
        union = np.zeros(K * K, dtype=bool)
        for row in E:                                  # each basis vector of V_c
            full = np.zeros(K * K, dtype=np.int64)
            full[S.flat] = np.mod(np.rint(row).astype(np.int64), p)
            h = C.invdft(fwd(C, full) * finv % p).reshape(-1)   # multiplier coefficients
            union |= (h % p) != 0
        ks = np.nonzero(union)[0]
        pcs = pck[ks // K] + pck[ks % K]
        out.append(dict(c=c, dim=len(piv), R=int(pcs.max()), top=bool(union[(K-1)*K + (K-1)]),
                        hist={int(v): int((pcs == v).sum()) for v in np.unique(pcs)}))
    return dict(p=p, t=t, idx=idx, cells=out)

if __name__ == "__main__":
    p, idx = int(sys.argv[1]), int(sys.argv[2])
    cmax = int(os.environ.get("CMAX", "8"))
    for t in map(int, sys.argv[3:]):
        t0 = time.time(); r = reach(p, t, idx, cmax); r["secs"] = round(time.time() - t0, 1)
        print(json.dumps(r), flush=True)
