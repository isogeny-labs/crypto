#!/usr/bin/env python3
"""VALIDATOR recomputation of the v5 analysis from results/holdout/runs.json (validator's own code).
Rules (PROTOCOL.md): cost/target = [(cI I + M + S)/(cI + 3) + h (H + L + N)]/T (primary, '5a');
'v4': h on H + L only.  icbest = min over ic5, ic5ks, ic5w, ic5rw runs; rhobest = min over the 10 rho runs;
single arms = min over their own grid; median of the 22 per-configuration ratios; bootstrap 95% exactly as
PROTOCOL/analyze.py state it (random.Random(1).choices, B = 4000, order statistics 100 and 3899) and an
independent numpy bootstrap (B = 20000, percentile 2.5/97.5, generator seeded from the nonce).
usage: recompute.py <frozen study dir> <out.json> <nonce>"""
import json, sys, os, random, statistics as st, collections, hashlib
import numpy as np
FZ, OUT, NONCE = sys.argv[1:4]
D = json.load(open(os.path.join(FZ, "results/holdout/runs.json")))
F = D["fields"]; R = [dict(zip(F, x)) for x in D["rows"]]
CI = [3, 10, 30]; HP = [0, 0.1, 1]
ICS = ["ic5", "ic5ks", "ic5w", "ic5rw"]; RHOS = ["rho2xks", "rhodpu"]
CANDS = ["icbest", "ic5", "ic5rw"]; REFS = ["rhobest", "rho2xks", "rhodpu", "bsgs2"]
def cost(r, ci, h, rule, nmap=None):
    N = r["N"] if nmap is None else nmap(r)
    u = r["H"] + r["L"] + (N if rule == "5a" else 0)
    return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * u) / r["targets"]
cfg = collections.OrderedDict()
for r in R:
    cfg.setdefault((r["pb"], r["T"], r["idx"], r["seed"]), collections.defaultdict(list))[r["arm"]].append(r)
cells = sorted({k[:2] for k in cfg})
def members(k, a):
    if a == "icbest": return [r for x in ICS for r in cfg[k][x]]
    if a == "rhobest": return [r for x in RHOS for r in cfg[k][x]]
    return cfg[k][a]
def best(k, a, ci, h, rule, nmap=None):
    return min(cost(r, ci, h, rule, nmap) for r in members(k, a))
def argbest(k, a, ci, h, rule, nmap=None):
    return min(members(k, a), key=lambda r: cost(r, ci, h, rule, nmap))
def boot_proto(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    meds = sorted(st.median(rnd.choices(xs, k=n)) for _ in range(B)); return meds[int(.025 * B)], meds[int(.975 * B) - 1]
GEN = np.random.default_rng(int(hashlib.sha256(NONCE.encode()).hexdigest()[:16], 16))
def boot_np(xs, B=20000):
    a = np.asarray(xs); idx = GEN.integers(0, len(a), size=(B, len(a)))
    m = np.median(a[idx], axis=1); return float(np.percentile(m, 2.5)), float(np.percentile(m, 97.5))
def table(rule, nmap=None, cands=CANDS, refs=REFS, npboot=True):
    S = []
    for ci in CI:
        for cand in cands:
            for pb, T in cells:
                ks = [k for k in cfg if k[:2] == (pb, T)]
                for h in HP:
                    c = {a: [best(k, a, ci, h, rule, nmap) for k in ks] for a in [cand] + [x for x in refs if x != cand]}
                    for a in [x for x in refs if x != cand]:
                        rat = [x / y for x, y in zip(c[cand], c[a])]; lo, hi = boot_proto(rat)
                        e = dict(cand=cand, cI=ci, bits=pb, T=T, h=h, vs=a, median=st.median(rat), lo=lo, hi=hi,
                                 wins=sum(x < 1 for x in rat), n=len(rat), ic=st.median(c[cand]), ref=st.median(c[a]))
                        if npboot: e["np_lo"], e["np_hi"] = boot_np(rat)
                        S.append(e)
    return S
def decisions(S, lo="lo", hi="hi"):
    d = {}
    for s in S:
        d.setdefault((s["cand"], s["vs"], s["cI"], s["bits"], s["T"]), []).append(s)
    out = {}
    for k, ss in d.items():
        out[k] = "cheaper" if all(s[hi] < 1 for s in ss) else ("dearer" if all(s[lo] > 1 for s in ss) else "unranked")
    return out
def dcount(dec, cand, vs):
    return dict(collections.Counter(v for k, v in dec.items() if k[0] == cand and k[1] == vs))
def compare(S, path):
    P = json.load(open(path)); assert len(P) == len(S), (len(P), len(S))
    mx = 0.0; nbad = 0
    for a, b in zip(S, P):
        for f in ("cand", "cI", "bits", "T", "h", "vs", "wins", "n"):
            if a[f] != b[f]: nbad += 1
        for f in ("median", "lo", "hi", "ic", "ref"):
            rel = abs(a[f] - b[f]) / abs(b[f]); mx = max(mx, rel)
            if rel > 1e-12: nbad += 1
    return dict(entries=len(P), mismatching_fields=nbad, max_rel_diff=mx)
res = {"runs_json_sha256": hashlib.sha256(open(os.path.join(FZ, "results/holdout/runs.json"), "rb").read()).hexdigest(),
       "rows": len(R), "configurations": len(cfg), "cells": cells,
       "per_config_runs": dict(collections.Counter(sum(len(v) for v in a.values()) for a in cfg.values())),
       "unverified_rows": sum(r["ok"] != r["targets"] for r in R)}
for rule, fn in (("5a", "summary.json"), ("v4", "summary_v4rule.json")):
    S = table(rule)
    res[f"summary_{rule}"] = S
    res[f"compare_{rule}"] = compare(S, os.path.join(FZ, "results/holdout", fn))
    dec = decisions(S); decn = decisions(S, "np_lo", "np_hi")
    res[f"decisions_{rule}"] = {f"{c} vs {v}": dcount(dec, c, v) for c in CANDS for v in REFS if v != c}
    res[f"decisions_{rule}_npboot"] = {f"{c} vs {v}": dcount(decn, c, v) for c in CANDS for v in REFS if v != c}
    res[f"decision_draw_dependence_{rule}"] = sum(dec[k] != decn[k] for k in dec)
    sel = lambda c, v, ci=None, h=None: [s for s in S if s["cand"] == c and s["vs"] == v and (ci is None or s["cI"] == ci) and (h is None or s["h"] == h)]
    rng = lambda ss, f="median": [min(s[f] for s in ss), max(s[f] for s in ss)]
    res[f"ranges_{rule}"] = {
        "icbest/rhobest all": rng(sel("icbest", "rhobest")),
        **{f"icbest/rhobest I=10M h={h}": rng(sel("icbest", "rhobest", 10, h)) for h in HP},
        **{f"icbest/rhobest I=10M h={h} lo>1 cells": sum(s["lo"] > 1 for s in sel("icbest", "rhobest", 10, h)) for h in HP},
        **{f"icbest/rhobest I=10M h={h} hi<1 cells": sum(s["hi"] < 1 for s in sel("icbest", "rhobest", 10, h)) for h in HP},
        **{f"icbest/rhobest h={h} hi<1 (all cI)": sum(s["hi"] < 1 for s in sel("icbest", "rhobest", None, h)) for h in HP},
        **{f"icbest/rhobest h={h} lo>1 (all cI)": sum(s["lo"] > 1 for s in sel("icbest", "rhobest", None, h)) for h in HP},
        "icbest/bsgs2 all": rng(sel("icbest", "bsgs2")),
        "icbest/bsgs2 I=10M h=0": rng(sel("icbest", "bsgs2", 10, 0)),
        "icbest/rhobest I=10M all h": rng(sel("icbest", "rhobest", 10)),
    }
    # RESULTS table rows (I = 10M)
    if rule == "5a":
        tab = []
        for pb, T in cells:
            g = {h: [s for s in sel("icbest", "rhobest", 10, h) if (s["bits"], s["T"]) == (pb, T)][0] for h in HP}
            b0 = [s for s in sel("icbest", "bsgs2", 10, 0) if (s["bits"], s["T"]) == (pb, T)][0]
            tab.append(dict(bits=pb, T=T, **{f"h{h}": [round(g[h]["median"], 3), round(g[h]["lo"], 3), round(g[h]["hi"], 3)] for h in HP},
                            bsgs_h0=round(b0["median"], 3), gae_ic_h01=round(g[0.1]["ic"], 1), gae_rho_h01=round(g[0.1]["ref"], 1)))
        res["results_table"] = tab
# winners at I = 10M
win = {}
for h in HP:
    rw = collections.Counter(); kw = collections.Counter(); iw = collections.Counter()
    for k in cfg:
        r = argbest(k, "rhobest", 10, h, "5a"); rw[r["arm"]] += 1
        if r["arm"] == "rho2xks": kw[r["param"]] += 1
        iw[argbest(k, "icbest", 10, h, "5a")["arm"]] += 1
    win[str(h)] = dict(rho_arm=dict(rw), rho2xks_kappa=dict(kw), ic_arm=dict(iw))
res["winners_I10M_5a"] = win
# noverify from rows.jsonl phases (validator's own code)
rows = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/rows.jsonl"))]
cfg2 = collections.defaultdict(lambda: collections.defaultdict(list))
for o in rows: cfg2[(o["pb"], o["T"], o["idx"], o["seed"])][o["arm"]].append(o)
def cnv(o, ci, h):
    I, M, S_, H, L = o["phases"]["targets"]
    return ((ci * I + M + S_) / (ci + 3) + h * (H + L + o["N"])) / o["targets"]
nv = {}
for pb, T in cells:
    ks = [k for k in cfg2 if k[:2] == (pb, T)]
    nv[f"{pb}/{T}"] = [st.median([min(cnv(o, 10, h) for a in ICS for o in cfg2[k][a]) / min(cnv(o, 10, h) for a in RHOS for o in cfg2[k][a]) for k in ks]) for h in HP]
res["noverify_I10M"] = nv
res["noverify_ranges"] = {str(h): [min(v[i] for v in nv.values()), max(v[i] for v in nv.values())] for i, h in enumerate(HP)}
# verification constant per target (GAE at I = 10M) by arm
vc = collections.defaultdict(list)
for o in rows:
    a, b = o["phases"]["targets"], o["phases"]["verify"]
    d = [y - x for x, y in zip(a, b)]
    vc[o["arm"]].append(((10 * d[0] + d[1] + d[2]) / 13) / o["targets"])
res["verify_gae_per_target_I10M"] = {a: [min(v), max(v)] for a, v in vc.items()}
# phase sanity: monotone and last phase == totals
bad = 0
for o in rows:
    ph = list(o["phases"].values())
    for x, y in zip(ph, ph[1:]):
        if any(b < a for a, b in zip(x, y)): bad += 1
    if ph[-1] != [o["I"], o["M"], o["S"], o["H"], o["L"]]: bad += 1
res["phase_violations"] = bad
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps({k: v for k, v in res.items() if not k.startswith("summary_")}, indent=1)[:9000])
