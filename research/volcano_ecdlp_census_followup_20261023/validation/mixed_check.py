#!/usr/bin/env python3
"""s3 secondary model spot check (validator-written): log S ~ C(level) +
C(family) + (1 | census vertex), statsmodels MixedLM, REML, for the ic and
rho-neg arms on V5 and V6 (isogenous families). Reports ICC and level ratios.
Usage: python3 -I mixed_check.py STUDY_DIR ROWS_DIR OUT_JSON"""
import json, os, sys, warnings
import numpy as np
import statsmodels.formula.api as smf
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vfu_data
STUDY, ROWS, OUT = sys.argv[1:4]
df, verts, tcr, checks = vfu_data.load(STUDY, ROWS)
ISO = {"V5": ["D", "Ua", "Ub", "N"], "V6": ["D", "Ua", "Ub", "S", "N"]}
out = {}
for V in ("V5", "V6"):
    for arm in ("ic", "rho-neg"):
        s = df[(df.V == V) & df.fam.isin(ISO[V]) & (df.arm == arm)].copy()
        s["logS"] = np.log(s["s"].astype(float))
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            fit = smf.mixedlm("logS ~ C(level) + C(fam)", s, groups=s["vertex"]).fit(reml=True)
        vc = float(fit.cov_re.iloc[0, 0]); vr = float(fit.scale)
        out[f"{V}|{arm}"] = {"converged": bool(fit.converged), "icc": vc / (vc + vr), "var_curve": vc, "var_resid": vr,
                             "level_ratio": {k.split("T.")[1].rstrip("]"): float(np.exp(v)) for k, v in fit.params.items() if k.startswith("C(level)")}}
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps(out, indent=1))
