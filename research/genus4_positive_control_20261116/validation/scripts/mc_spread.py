#!/usr/bin/env python3
"""Monte-Carlo spread of the bootstrap endpoints the claims check flagged: the
validator's bootstrap (same resampling design as the protocols) at the
producers' B = 2000 with 30 RNG seeds; a claimed endpoint inside the seeds'
[min, max] is consistent with Monte-Carlo variation."""
import json, random, sys
ROOT, OUT = sys.argv[1], sys.argv[2]
s1 = [json.loads(l) for l in open(f"{ROOT}/research/genus4_positive_control_20261116/run.jsonl")]
s2 = [json.loads(l) for l in open(f"{ROOT}/research/genus4_rho_refs_20261116/run.jsonl")]
mean = lambda v: sum(v) / len(v)
B = 2000
def q(bs):
    bs = sorted(bs); return bs[int(0.025 * B)], bs[int(0.975 * B) - 1]
# study 1: c/R p=4099 w=1 (curves then seeds, unpaired as the producer's analyze.py)
X1 = {(r["arm"], r["curve_seed"], r["seed"]): r for r in s1 if r["kind"] == "run" and r["p"] == 4099}
cs1 = sorted({k[1] for k in X1}); sd1 = [1, 2, 3, 4]
def s1stat(seed):
    rnd = random.Random(seed); bs = []
    for _ in range(B):
        n, d = [], []
        for c in [rnd.choice(cs1) for _ in cs1]:
            n += [X1[("C_double_lp+pre", c, rnd.choice(sd1))] for _ in sd1]
            d += [X1[("R_rho_neg", c, rnd.choice(sd1))] for _ in sd1]
        bs.append(mean([r["group_ops"] + r["rel_mul"] + r["la_mul"] for r in n]) / mean([r["group_ops"] + r["native"] for r in d]))
    return q(bs)
# study 2: readings, targets then seeds, paired
X2 = {(r["arm"], r["curve_seed"], r["target_seed"], r["seed"]): r for r in s2 if r["kind"] == "run"}
keys = sorted({k[1:] for k in X2})
def ic(r, rd, w):
    M, A = r["rel_mul"] + r["la_mul"], r["rel_add"] + r["la_add"]
    return r["group_ops"] + w * (M if rd == "Q1" else M + A)
def rho(r, rd, w):
    C, K = r["native"], r["compares_uncharged"]
    return r["group_ops"] + w * (C if rd in ("Q1", "Q2") else C + K)
def s2stat(seed, rd, w, ks):
    rnd = random.Random(seed); tg = sorted({k[:2] for k in ks}); by = {t: [k for k in ks if k[:2] == t] for t in tg}; bs = []
    for _ in range(B):
        kk = []
        for t in [rnd.choice(tg) for _ in tg]:
            kk += [rnd.choice(by[t]) for _ in by[t]]
        i = mean([ic(X2[("C_double_lp+pre+coord+unitT",) + k], rd, w) for k in kk])
        rr = mean([rho(X2[("R_rho_neg",) + k], rd, w) for k in kk]); pp = mean([rho(X2[("R_rho_plain",) + k], rd, w) for k in kk])
        bs.append(i / min(rr, pp))
    return q(bs)
cells = {
  "s1 c/R p=4099 w=1 [0.675, 1.838]": ((0.675, 1.838), lambda sd: s1stat(sd)),
  "s2 Q2 w=0.1 [0.183, 0.380]": ((0.183, 0.380), lambda sd: s2stat(sd, "Q2", 0.1, keys)),
  "s2 Q2 w=1 [0.995, 2.068]": ((0.995, 2.068), lambda sd: s2stat(sd, "Q2", 1.0, keys)),
  "s2 Q1 w=1 LOO without 363 [0.293, 0.786]": ((0.293, 0.786), lambda sd: s2stat(sd, "Q1", 1.0, [k for k in keys if k[0] != 363])),
  "s2 Q1 w=1 LOO without 393 [0.512, 0.974]": ((0.512, 0.974), lambda sd: s2stat(sd, "Q1", 1.0, [k for k in keys if k[0] != 393])),
  "s2 Q1 w=1 LOO without 407 [0.291, 0.677]": ((0.291, 0.677), lambda sd: s2stat(sd, "Q1", 1.0, [k for k in keys if k[0] != 407])),
  "s2 Q3 w=1 LOO without 363 [0.088, 0.238]": ((0.088, 0.238), lambda sd: s2stat(sd, "Q3", 1.0, [k for k in keys if k[0] != 363])),
  "s2 Q3 w=1 LOO without 393 [0.154, 0.296]": ((0.154, 0.296), lambda sd: s2stat(sd, "Q3", 1.0, [k for k in keys if k[0] != 393])),
  "s2 Q3 w=1 LOO without 407 [0.087, 0.203]": ((0.087, 0.203), lambda sd: s2stat(sd, "Q3", 1.0, [k for k in keys if k[0] != 407])),
}
out = {}
for name, (claimed, fn) in cells.items():
    los, his = zip(*[fn(1000 + i) for i in range(30)])
    inside = min(los) <= claimed[0] <= max(los) and min(his) <= claimed[1] <= max(his)
    out[name] = {"claimed": claimed, "lo_range": [min(los), max(los)], "hi_range": [min(his), max(his)], "inside": inside}
    print(f"{name:45s} lo in [{min(los):.3f}, {max(los):.3f}]  hi in [{min(his):.3f}, {max(his):.3f}]  claimed inside: {inside}")
json.dump(out, open(OUT, "w"), indent=1)
