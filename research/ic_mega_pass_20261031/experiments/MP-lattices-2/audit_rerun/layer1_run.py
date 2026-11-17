import json, time, math, sys, random
from common import *
from layer1 import logvol_rows
def setup(kind, l, p, seed):
    rnd = random.Random(seed); a, b, xR = rnd.randrange(p), rnd.randrange(p), rnd.randrange(p)
    f, d, n = (S3_specialised(a,b,xR,p), 2, 2) if kind == "S3" else (S4_specialised(a,b,xR,p), 4, 3)
    rows = build_rows(f, p, l, d); mons, idx = deg_order(n, d*l + d)
    return rows, idx, len(mons)
def margin(rows, idx, nc, p, l, dl, tau):
    X = int(round(2**(dl*math.log2(p))))
    sel = rows if tau is None else [r for r in rows if dl*sum(r[0]) - r[1] <= tau + 1e-12]
    r = len(sel)
    if r < 2: return None, r, None, None
    lv = logvol_rows(sel, idx, X, nc) / math.log(p)
    diag = sum(dl*sum(al) + (l-k) for al, k, _ in sel)
    return l - 0.5*math.log(r)/math.log(p) - lv/r, r, lv, diag
def bisect(rows, idx, nc, p, l, tau, lo=0.0, hi=0.5, it=12):
    m, *_ = margin(rows, idx, nc, p, l, lo + 1e-4, tau)
    if m is None or m <= 0: return 0.0, None
    for _ in range(it):
        mid = (lo+hi)/2; m, r, lv, dg = margin(rows, idx, nc, p, l, mid, tau)
        if m is not None and m > 0: lo = mid
        else: hi = mid
    m, r, lv, dg = margin(rows, idx, nc, p, l, lo, tau)
    return lo, {"r": r, "logvol_minus_logdiag": None if lv is None else lv - dg}
if __name__ == "__main__":
    kind, lmax, bits, seed = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    p = int(pari.nextprime(2**bits)); taus = [None, -0.5, -0.25, 0.0, 0.25, 0.5, 1.0, 1.5]
    out = {"kind": kind, "bits": bits, "p": str(p), "seed": seed, "rows": []}
    for l in range(1, lmax+1):
        t = time.time(); rows, idx, nc = setup(kind, l, p, seed); rec = {"l": l, "dim_A": len(rows)}
        for tau in taus:
            ds, info = bisect(rows, idx, nc, p, l, tau); rec["tau=" + str(tau)] = {"delta_star": ds, **(info or {})}
        rec["sec"] = time.time() - t; out["rows"].append(rec); print(json.dumps(rec), flush=True)
    json.dump(out, open(f"raw/layer1_{kind}_b{bits}_s{seed}.json", "w"), indent=1)
