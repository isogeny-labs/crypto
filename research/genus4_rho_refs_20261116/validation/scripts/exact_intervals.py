#!/usr/bin/env python3
"""Reproduce the producers' bootstrap intervals exactly: the validator's own
code, the protocols' resampling design, and the producers' RNG seeds and draw
order (study 1: random.Random(20261119) per cell, curves sorted, per-curve value
lists in run.jsonl order, numerator and denominator drawn independently; study 2:
random.Random(20261120) per cell, targets sorted, keys sorted, paired).  Prints
every interval of tables.md as recomputed, for a digit-by-digit comparison."""
import json, random, sys
ROOT, OUT = sys.argv[1], sys.argv[2]
B = 2000
mean = lambda v: sum(v) / len(v)
out = {}
# ---- study 1
s1 = [json.loads(l) for l in open(f"{ROOT}/research/genus4_positive_control_20261116/run.jsonl")]
A1 = {"c": "C_double_lp+pre", "v": "C_double_lp+pre+coord", "w": "C_double_lp+pre+coord+unitT", "R": "R_rho_neg"}
def grp(arm, p, fn):
    g = {}
    for r in s1:
        if r["kind"] == "run" and r["arm"] == A1[arm] and r["p"] == p:
            g.setdefault(r["curve_seed"], []).append(fn(r))
    return g
def boot1(na, da, p, fn, seed=20261119):
    rnd = random.Random(seed)
    gn, gd = grp(na, p, fn), grp(da, p, fn)
    curves = sorted(gn)
    est = mean(sum(gn.values(), [])) / mean(sum(gd.values(), []))
    bs = []
    for _ in range(B):
        cs = [rnd.choice(curves) for _ in curves]
        n, d = [], []
        for c in cs:
            n += [rnd.choice(gn[c]) for _ in gn[c]]
            d += [rnd.choice(gd[c]) for _ in gd[c]]
        bs.append(mean(n) / mean(d))
    bs.sort()
    return [round(est, 3), round(bs[int(0.025 * B)], 3), round(bs[int(0.975 * B) - 1], 3)]
tot = lambda w: (lambda r: r["group_ops"] + w * (r["rel_mul"] + r["la_mul"]) if "rel_mul" in r else r["group_ops"] + w * r["native"])
fpm = lambda r: (r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + 16 * r["la_mul"]) if "rel_mul" in r else r["cantor_mul"] + 10 * r["cantor_inv"]
for p in (2053, 4099, 8209):
    out[f"s1 {p} LA v/c"] = boot1("v", "c", p, lambda r: r["la_mul"])
    out[f"s1 {p} LA w/c"] = boot1("w", "c", p, lambda r: r["la_mul"])
    out[f"s1 {p} rel w/c"] = boot1("w", "c", p, lambda r: r["rel_mul"])
    for arm in "cvw":
        for w in (0.0, 0.1, 1.0):
            out[f"s1 {p} {arm}/R w={w}"] = boot1(arm, "R", p, tot(w))
        out[f"s1 {p} {arm}/R FPM"] = boot1(arm, "R", p, fpm)
    for arm in "vw":
        for w in (0.1, 1.0):
            out[f"s1 {p} {arm}/c w={w}"] = boot1(arm, "c", p, tot(w))
# ---- study 2
s2 = [json.loads(l) for l in open(f"{ROOT}/research/genus4_rho_refs_20261116/run.jsonl")]
X = {(r["arm"], r["curve_seed"], r["target_seed"], r["seed"]): r for r in s2 if r["kind"] == "run"}
cal = [r for r in s2 if r["kind"] == "calibration"]
wm_p = mean([c["w_cal_mul_p"] for c in cal]); wa_p = mean([c["w_cal_add_p"] for c in cal]); wm_r = mean([c["w_cal_mul_r"] for c in cal])
def ic(r, rd, w):
    G, M, Ad = r["group_ops"], r["rel_mul"] + r["la_mul"], r["rel_add"] + r["la_add"]
    if rd == "Q1": return G + w * M
    if rd == "Qcal": return G + wm_p * r["rel_mul"] + wm_r * r["la_mul"] + wa_p * Ad
    return G + w * (M + Ad)
def rho(r, rd, w):
    G, C, K, H, D = r["group_ops"], r["native"], r["compares_uncharged"], r["hash_uncharged"], r["coef_adds_uncharged"]
    return {"Q1": G + w * C, "Q2": G + w * C, "Q3": G + w * (C + K), "Q4": G + w * (C + K + H + D), "Qcal": G}[rd]
def boot2(rd, w, ks, seed=20261120):
    rnd = random.Random(seed)
    tg = sorted({(k[0], k[1]) for k in ks}); by = {t: [k for k in ks if (k[0], k[1]) == t] for t in tg}
    def st(kk):
        i = mean([ic(X[("C_double_lp+pre+coord+unitT",) + k], rd, w) for k in kk])
        a = mean([rho(X[("R_rho_neg",) + k], rd, w) for k in kk]); b = mean([rho(X[("R_rho_plain",) + k], rd, w) for k in kk])
        return i / min(a, b)
    est = st(ks); bs = []
    for _ in range(B):
        kk = []
        for t in [rnd.choice(tg) for _ in tg]:
            kk += [rnd.choice(by[t]) for _ in by[t]]
        bs.append(st(kk))
    bs.sort()
    return [round(est, 3), round(bs[int(0.025 * B)], 3), round(bs[int(0.975 * B) - 1], 3)]
keys = sorted({k[1:] for k in X if k[0] == "C_double_lp+pre+coord+unitT"})
for rd in ("Q1", "Q2", "Q3", "Q4"):
    for w in (0.0, 0.1, 1.0):
        out[f"s2 {rd} w={w}"] = boot2(rd, w, keys)
    for c in (363, 393, 407):
        out[f"s2 {rd} LOO-{c} w=1"] = boot2(rd, 1.0, [k for k in keys if k[0] != c])
out["s2 Qcal"] = boot2("Qcal", 0.0, keys)
json.dump(out, open(OUT, "w"), indent=1)
for k, v in out.items():
    print(f"{k:28s} {v[0]:.3f} [{v[1]:.3f}, {v[2]:.3f}]")
