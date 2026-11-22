#!/usr/bin/env python3
"""Analyse ss_coverage.jsonl and tabulate the rank requirement for Silverman-Suzuki lifts. [pcc:unverified]"""
import sys, json, math
import numpy as np

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from bounds_hermite import gamma

recs = [json.loads(l) for l in open(sys.argv[1])]
out = {"curves": [], "rank_req": []}
curves = {r["name"]: r for r in recs if r["kind"] == "curve"}
lines = []
tot_pairs = 0.0
tot_coll = 0
for nm, c in curves.items():
    cov = [r for r in recs if r["kind"] == "cov" and r["name"] == nm and r["status"] == "ok"]
    if not cov:
        continue
    lq = np.array([math.log(r["q"]) for r in cov])
    lx = np.array([math.log(r["Xstar"]) for r in cov])
    slope, icpt = np.polyfit(lq, lx, 1)
    s = c["s"]
    Vs = math.pi ** (s / 2) / math.gamma(s / 2 + 1)
    # Gaussian count N(X) = Vs X^(s/2)/sqrt(det): X at which N = 2^128 (256-bit q)
    X256 = (2.0 ** 128 * math.sqrt(c["detG"]) / Vs) ** (2.0 / s)
    # packing (rigorous) X at which T(1+2 sqrt(X/lam))^s = 2^128
    Xpack = c["lam"] * ((2.0 ** (128.0 / s) / c["T"] ** (1.0 / s) - 1) / 2) ** 2
    ratio = [r["N_at_Xstar"] / r["gauss"] for r in cov]
    for r in cov:
        n = r["N_at_Xstar"]
        tot_pairs += n * (n - 1) / 2 / r["q"]
        tot_coll += r["collisions"]
    row = dict(name=nm, s=s, lam=c["lam"], detG=c["detG"], T=c["T"], n_cells=len(cov),
               slope_logX_logq=slope, expected_slope=1.0 / s,
               N_over_gauss_min=min(ratio), N_over_gauss_max=max(ratio),
               Xstar_over_lam_256_gauss=X256 / c["lam"], log2_Xstar_256_gauss_nats=math.log2(X256),
               Xpack_over_lam_256=Xpack / c["lam"],
               max_bits=max(r["bits"] for r in cov))
    out["curves"].append(row)
out["collisions_observed"] = tot_coll
out["collisions_expected_random"] = tot_pairs
# rank needed at 256 bits (q ~ 2^256): N(X) >= sqrt(q) = 2^128
for rho in (2, 10, 100, 1e3, 1e4, 1e6):
    need_pack = None
    need_gauss = None
    for s in range(1, 400):
        if need_pack is None and s * math.log2(1 + 2 * math.sqrt(rho)) - math.log2(16) >= 128:
            need_pack = s
        log2Vs = ((s / 2) * math.log(math.pi) - math.lgamma(s / 2 + 1)) / math.log(2)
        if need_gauss is None and log2Vs + (s / 2) * math.log2(gamma(s) * rho) >= 128:
            need_gauss = s
    out["rank_req"].append(dict(X_over_lambda=rho, s_min_packing_T16=need_pack,
                                s_min_gauss_hermite=need_gauss))
json.dump(out, open(sys.argv[2], "w"), indent=1)
for r in out["curves"]:
    print({k: (round(v, 4) if isinstance(v, float) else v) for k, v in r.items()})
print("collisions observed", tot_coll, "expected if random", round(tot_pairs, 1))
for r in out["rank_req"]:
    print(r)
