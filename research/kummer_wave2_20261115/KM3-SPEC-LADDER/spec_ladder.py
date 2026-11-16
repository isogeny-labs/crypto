#!/usr/bin/env python3
"""KM3-SPEC-LADDER (Python route). usage: spec_ladder.py P T IDX [CMAX3] [CMAX2]
d_F of the m=3 instance and, for every F_p-root rho of S3(u,x3,x_R) over grid x3, of the m=2
system S3(x1,x2,rho) on the same tower/curve (same a,b,g). Uses lab A's hky_closure.py (HKY closure
on the group-algebra basis) and lab N1's instance generator."""
import sys, os, json, time, numpy as np
sys.path.insert(0, "/home/user/crypto/research/kummer_wave2_20261115/KM3-LQR-CLOSURE")
sys.path.insert(0, "/home/user/crypto/research/kummer_m3_ns_degree_20261115")
from hky_closure import *
import ns_degree as nsd
from run_cells import gens_m3

def dF_m3(C, F, p, cmax, log):
    for c in range(2, cmax + 1):
        S = Space(C, c, True)
        gens = [g for g in gens_m3(C, F, S) if g is not None]
        E, piv, one = closure(S, gens, p) if gens else (None, [], False)
        log(f"   m3 c={c} cols={S.n} one={one}")
        if one: return c
    return None

def dF_m2_f(C, f, p, cmax, spaces):
    coef = C.invdft(f).reshape(-1)
    for c in range(2, cmax + 1):
        if c not in spaces: spaces[c] = Space(C, c, False)
        S = spaces[c]
        g = S.vec_from_coefs([coef])
        gens = [g] if g is not None else []
        E, piv, one = closure(S, gens, p) if gens else (None, [], False)
        if one: return c
    return None

def m2_f(C, p, a, b, g, rho):
    x = (g * C.Z) % p
    c2, c1, c0 = nsd.S3coef(p, a, b, x[0], x[1])
    return (c2 * rho * rho + c1 * rho + c0) % p

if __name__ == "__main__":
    p, t, idx = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
    cmax3 = int(sys.argv[4]) if len(sys.argv) > 4 else 8
    cmax2 = int(sys.argv[5]) if len(sys.argv) > 5 else 7
    out = os.environ.get("SL_OUT", ".")
    os.makedirs(out, exist_ok=True)
    t0 = time.time()
    C3, meta, F = nsd.make_instance(p, t, idx, 3)
    a, b, g, xR = meta["a"], meta["b"], meta["g"], meta["x_R"]
    log = lambda s: print(s, flush=True)
    rec = dict(p=p, t=t, idx=idx, meta=meta, cmax3=cmax3, cmax2=cmax2)
    # spec arm first (cheap), then m3
    C2 = nsd.Ctx(p, t, 2)
    allv = np.arange(p, dtype=np.int64)
    rho_x3 = {}
    nroot0 = 0
    for ia in range(C3.K):
        x3 = int((g * C3.pw[ia]) % p)
        c2, c1, c0 = [int(v) for v in nsd.S3coef(p, a, b, np.int64(x3), np.int64(xR))]  # poly in 3rd arg u: S3(x3,xR,u)
        vals = (c2 * allv * allv + c1 * allv + c0) % p
        rs = [int(r) for r in np.nonzero(vals == 0)[0]]
        if not rs: nroot0 += 1
        for r in rs: rho_x3.setdefault(r, []).append(x3)
    rec["x3_without_Fp_root"] = nroot0
    spaces = {}
    spec = []
    for rho, x3s in sorted(rho_x3.items()):
        f = m2_f(C2, p, a, b, g, rho)
        cons = bool((f == 0).any())
        d = None if cons else dF_m2_f(C2, f, p, cmax2, spaces)
        # out-of-hypothesis arm: rho+1 (not a root)
        f1 = m2_f(C2, p, a, b, g, (rho + 1) % p)
        d1 = None if (f1 == 0).any() else dF_m2_f(C2, f1, p, cmax2, spaces)
        spec.append(dict(rho=rho, x3=x3s, grid_zero=cons, dF=d, dF_rho_plus_1=d1))
    rec["spec"] = spec
    log(f"p={p} t={t} idx={idx}: {len(spec)} rho; spec dF values {sorted(set(s['dF'] for s in spec if s['dF'] is not None))}, None={sum(s['dF'] is None for s in spec)}; {time.time()-t0:.1f}s")
    rec["dF3"] = dF_m3(C3, F, p, cmax3, log)
    rec["secs"] = round(time.time() - t0, 1)
    json.dump(rec, open(os.path.join(out, f"sl_p{p}_t{t}_i{idx}.json"), "w"), indent=1)
    log(f"DONE p={p} t={t} idx={idx} dF3={rec['dF3']} secs={rec['secs']}")
