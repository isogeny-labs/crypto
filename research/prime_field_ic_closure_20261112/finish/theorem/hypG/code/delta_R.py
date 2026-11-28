"""Target dependence of Delta_R := weight-degree of S_R as a function on the tower grid W^m (the least degree
of any representative S_R + j, j in J).  [pcc:unverified]
Claim checked: at most 2^{m-1} target abscissas have Delta_R below the generic (maximal) value.
Input: an exhaustive_G output file (curve, g, all target abscissas)."""
import sys, json, collections
import numpy as np
import icmodel as M, fastdelta as F
from check_G_hypothesis import semaev_dict
for f in sys.argv[1:]:
    D = json.load(open(f)); s = D["summary"]
    m, k, p, a, b, g = s["m"], s["k"], s["p"], s["a"], s["b"], s["g"]
    d = 2 ** k
    roots = [w[0] for w in M.tower_base(p, k, g).pts]
    Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
    Wt = F.weight_grid(d, m, "tower")
    cnt = collections.Counter(); low = []
    for r in D["rows"]:
        G = F.semaev_grid(semaev_dict(m, p, a, b, r["xR"]), m, roots, p)
        dl = F.delta_of(F.coeffs(G, Vinv, p), Wt)
        cnt[dl] += 1; low.append((dl, r["xR"], r["dec"], r["degen"]))
    top = max(cnt)
    exc = [x for x in low if x[0] < top]
    print(json.dumps({"file": f.split("/")[-1], "m": m, "k": k, "p": p, "Delta_hist": dict(sorted(cnt.items())),
                      "generic": top, "n_exceptional_abscissas": len(exc), "bound_2^(m-1)": 2 ** (m - 1),
                      "ok": len(exc) <= 2 ** (m - 1), "exceptional": exc[:8]}))
