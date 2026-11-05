# Layer 1: exact log-volume (in log_p units) of arm A (full) vs arm B (helpful rows only) vs B' (threshold tau).
import json, time, math, sys
from common import *
pari.default("realprecision", 3000)
def logvol_rows(rows, idx, X, ncols):
    # build integer matrix (rows x cols) with column scaling X^{|beta|}; volume via matqr of transpose
    M = [[0]*ncols for _ in rows]
    for r, (_, _, g) in enumerate(rows):
        for e, c in g.items(): M[r][idx[e]] = c * X**sum(e)
    # drop zero columns
    nz = [j for j in range(ncols) if any(M[r][j] for r in range(len(rows)))]
    A = pari.matrix(len(nz), len(rows), [M[r][j] for j in nz for r in range(len(rows))])  # cols = rows
    G = pari.mattranspose(A) * A
    D = pari.matdet(G)
    return 0.5 * float(pari.log(D * pari('1.0')))
def run(kind, l, p, seed, deltas, taus):
    rnd = random.Random(seed); a, b, xR = rnd.randrange(p), rnd.randrange(p), rnd.randrange(p)
    f, d, n = (S3_specialised(a,b,xR,p), 2, 2) if kind == "S3" else (S4_specialised(a,b,xR,p), 4, 3)
    rows = build_rows(f, p, l, d)
    mons, idx = deg_order(n, d*l + d)  # superset
    lp = math.log(p); out = []
    for dl in deltas:
        e = round(dl * math.log2(p)); X = 2**e; dlx = e / math.log2(p)
        res = {"delta": dlx}
        for tau in taus:
            sel = [r for r in rows if dlx*sum(r[0]) - r[1] <= tau + 1e-12] if tau is not None else rows
            r = len(sel)
            if r < 2: res[str(tau)] = None; continue
            lv = logvol_rows(sel, idx, X, len(mons)) / lp  # log_p vol
            diag = sum(dlx*sum(al) + (l-k) for al, k, _ in sel)  # log_p prod diag
            # HG surrogate margin: l - log_p(sqrt r) - lv/r  (>0 => vol^{1/r} < p^l/sqrt r)
            res[str(tau)] = {"r": r, "logvol": lv, "logdiag": diag, "margin": l - 0.5*math.log(r)/lp - lv/r}
        out.append(res)
    return out
if __name__ == "__main__":
    t = time.time(); p = int(pari.nextprime(2**64 + 12345))
    print(json.dumps(run("S3", 3, p, 1, [0.1, 0.15], [None, 0.0])), time.time()-t)
