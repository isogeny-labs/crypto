#!/usr/bin/env python3
"""Validator recompute of the v4 holdout analysis from results/holdout/runs.json (raw counters), written
independently of analyze.py.  cost/target(cI, h) = [(cI I + M + S)/(cI + 3) + h (H + L)] / T.
Per configuration (bits, T, curve idx, seed): icbest = min(ic5, ic5ks); rhobest = min over the 15 rho
runs (rho2xks x3, rhodp x12); bsgs2 = min over its 3 runs; ic5, ic5ks, rho2xks, rhodp = min over their runs.
Median of the 22 per-configuration ratios; 95% bootstrap interval two ways: (a) the procedure analyze.py
states (random.Random(1), B = 4000, percentile indices 100 and 3899; configuration order = first
appearance in the data) to compare with summary.json bit for bit, and (b) an independent numpy bootstrap
(B = 20000, seed from the validation nonce) to check that no decision depends on the bootstrap draw.
usage: recompute.py <frozen study dir> <out.json> <nonce>"""
import json, sys, os, random, statistics as st, collections, hashlib
import numpy as np
study, out, nonce = sys.argv[1:4]
R = json.load(open(os.path.join(study, "results/holdout/runs.json")))
F = R["fields"]; runs = [dict(zip(F, r)) for r in R["rows"]]
CI = [3, 10, 30]; HP = [0, 0.1, 1]
def cost(r, ci, h, extra=0.0):
    return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * (r["H"] + r["L"] + extra)) / r["targets"]
cfg = collections.OrderedDict()
for r in runs:
    cfg.setdefault((r["pb"], r["T"], r["idx"], r["seed"]), []).append(r)
GROUPS = {"icbest": ("ic5", "ic5ks"), "rhobest": ("rho2xks", "rhodp"), "ic5": ("ic5",), "ic5ks": ("ic5ks",),
          "rho2xks": ("rho2xks",), "rhodp": ("rhodp",), "bsgs2": ("bsgs2",)}
def best(k, g, ci, h, costf=cost):
    return min(costf(r, ci, h) for r in cfg[k] if r["arm"] in GROUPS[g])
def argbest(k, g, ci, h, costf=cost):
    return min((costf(r, ci, h), r["arm"], r["param"], r.get("doff")) for r in cfg[k] if r["arm"] in GROUPS[g])
cells = []
for k in cfg:
    if k[:2] not in cells: cells.append(k[:2])
cells.sort()
def boot_prod(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    m = sorted(st.median(rnd.choices(xs, k=n)) for _ in range(B)); return m[int(.025 * B)], m[int(.975 * B) - 1]
rng = np.random.default_rng(int(nonce[:16], 16))
def boot_np(xs, B=20000):
    a = np.asarray(xs); idx = rng.integers(0, len(a), size=(B, len(a)))
    m = np.median(a[idx], axis=1); return float(np.quantile(m, 0.025)), float(np.quantile(m, 0.975))
def compare(cand, ref, costf=cost, boot=True):
    S = []
    for ci in CI:
        for pb, T in cells:
            ks = [k for k in cfg if k[:2] == (pb, T)]
            for h in HP:
                c = [best(k, cand, ci, h, costf) for k in ks]; y = [best(k, ref, ci, h, costf) for k in ks]
                rat = [a / b for a, b in zip(c, y)]
                e = dict(cand=cand, vs=ref, cI=ci, bits=pb, T=T, h=h, median=st.median(rat), n=len(rat),
                         wins=sum(x < 1 for x in rat), ic=st.median(c), ref=st.median(y))
                if boot:
                    e["lo"], e["hi"] = boot_prod(rat); e["lo_np"], e["hi_np"] = boot_np(rat)
                S.append(e)
    return S
def decide(S, cand, ref, lo="lo", hi="hi"):
    D = {}
    for ci in CI:
        for pb, T in cells:
            ss = [s for s in S if s["cand"] == cand and s["vs"] == ref and s["cI"] == ci and (s["bits"], s["T"]) == (pb, T)]
            assert len(ss) == 3
            D[f"{pb}/{T}/cI{ci}"] = "cheaper" if all(s[hi] < 1 for s in ss) else "dearer" if all(s[lo] > 1 for s in ss) else "unranked"
    return D
if __name__ == "__main__":
    PAIRS = [(c, r) for c in ("icbest", "ic5", "ic5ks") for r in ("rhobest", "rho2xks", "rhodp", "bsgs2") if c != r]
    ALL = []
    for c, r in PAIRS: ALL += compare(c, r)
    # compare with summary.json
    SJ = json.load(open(os.path.join(study, "results/holdout/summary.json")))
    idx = {(s["cand"], s["vs"], s["cI"], s["bits"], s["T"], s["h"]): s for s in SJ}
    mism = []; maxrel = 0.0
    for s in ALL:
        p = idx.get((s["cand"], s["vs"], s["cI"], s["bits"], s["T"], s["h"]))
        if p is None: mism.append(("missing", s)); continue
        for f in ("median", "lo", "hi", "ic", "ref"):
            rel = abs(p[f] - s[f]) / max(abs(s[f]), 1e-300); maxrel = max(maxrel, rel)
            if rel > 1e-12: mism.append((f, p[f], s[f]))
        if p["wins"] != s["wins"] or p["n"] != s["n"]: mism.append(("wins", p["wins"], s["wins"]))
    dec = {f"{c} vs {r}": decide(ALL, c, r) for c, r in PAIRS}
    dec_np = {f"{c} vs {r}": decide(ALL, c, r, "lo_np", "hi_np") for c, r in PAIRS}
    counts = {k: dict(collections.Counter(v.values())) for k, v in dec.items()}
    np_disagree = sum(dec[k][c] != dec_np[k][c] for k in dec for c in dec[k])
    # headline ranges
    def rng_(cand, ref, ci=None, h=None):
        xs = [s["median"] for s in ALL if s["cand"] == cand and s["vs"] == ref and (ci is None or s["cI"] == ci) and (h is None or s["h"] == h)]
        return [min(xs), max(xs)]
    ranges = {"icbest/rhobest all": rng_("icbest", "rhobest"),
              **{f"icbest/rhobest cI10 h{h}": rng_("icbest", "rhobest", 10, h) for h in HP},
              "icbest/bsgs2 all": rng_("icbest", "bsgs2"),
              **{f"icbest/bsgs2 cI10 h{h}": rng_("icbest", "bsgs2", 10, h) for h in HP},
              **{f"icbest/bsgs2 cI{ci}": rng_("icbest", "bsgs2", ci) for ci in CI},
              "icbest/rho2xks all": rng_("icbest", "rho2xks"), "icbest/rhodp all": rng_("icbest", "rhodp")}
    # which rho wins per configuration (cI = 10)
    winners = {}
    for h in HP:
        w = collections.Counter(); wd = collections.Counter()
        for k in cfg:
            c, arm, param, doff = argbest(k, "rhobest", 10, h); w[f"{arm} {param}"] += 1
            if arm == "rhodp": wd[f"kappa={param} off={doff}"] += 1
        winners[f"h{h}"] = dict(arms=dict(w), rhodp_params=dict(wd))
    icw = {}
    for ci in CI:
        for h in HP:
            icw[f"cI{ci} h{h}"] = dict(collections.Counter(argbest(k, "icbest", ci, h)[1] for k in cfg))
    res = dict(n_runs=len(runs), configs=len(cfg), cells=cells, summary_entries=len(ALL), summary_json_entries=len(SJ),
               summary_mismatches=len(mism), summary_mismatch_examples=mism[:5], max_rel_diff=maxrel,
               decision_counts=counts, decisions=dec, numpy_bootstrap_decision_disagreements=np_disagree,
               ranges=ranges, rho_winner_cI10=winners, icbest_winner=icw,
               table_cI10=[s for s in ALL if s["cI"] == 10 and s["cand"] == "icbest"])
    json.dump(res, open(out, "w"), indent=1)
    print(json.dumps({k: res[k] for k in ("n_runs", "configs", "summary_entries", "summary_json_entries", "summary_mismatches", "max_rel_diff", "decision_counts", "numpy_bootstrap_decision_disagreements", "ranges", "rho_winner_cI10")}, indent=1))
