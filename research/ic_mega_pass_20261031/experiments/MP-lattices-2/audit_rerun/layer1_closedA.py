import json, math, itertools
def dstar(n, d, l, bits=128):
    lp = bits*math.log(2); al = list(itertools.product(range(d*l+1), repeat=n)); r = len(al)
    ks = [min(min(a//d for a in x), l) for x in al]; S = sum(sum(x) for x in al); K = sum(l-k for k in ks)
    # margin(delta) = l - ln(r)/(2 lp) - (delta*S + K)/r > 0
    return max(0.0, (l - math.log(r)/(2*lp) - K/r) * r / S), r
out = {"S3": [], "S4": []}
for l in range(1, 17): out["S3"].append((l,) + dstar(2, 2, l))
for l in range(1, 6): out["S4"].append((l,) + dstar(3, 4, l))
for key in ["S3","S4"]:
    import numpy as np
    L = np.array([x[0] for x in out[key] if x[0] >= 2]); D = np.array([x[1] for x in out[key] if x[0] >= 2])
    A = np.vstack([np.ones_like(L, dtype=float), -1.0/L]).T; c, res, *_ = np.linalg.lstsq(A, D, rcond=None)
    out[key + "_fit_delta_inf_c"] = list(c)
    # p->infinity limit (drop sqrt r term)
    out[key + "_lim"] = [(l, dstar(2 if key=="S3" else 3, 2 if key=="S3" else 4, l, bits=10**9)[0]) for l in (4, 16, 64)] if key=="S3" else [(l, dstar(3,4,l,bits=10**9)[0]) for l in (2,5,12)]
print(json.dumps(out, indent=0)); json.dump(out, open("raw/layer1_closedA.json", "w"), indent=1)
