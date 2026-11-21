#!/usr/bin/env python3
"""Validator sensitivity analysis of the v4 section-5a decisions, from
  results/holdout/runs.json (producer counters; every row replayed exactly by the validator),
  the validator's full instrumented replay (replay_all.json: per-run counts of the uncounted rho native
  work: canonicalisations, step-index hashes, DP tests; and HashMap calls),
  the validator sensitivity panels (sens.json: ic5rw / ic5w / ic5ks at other mu; sens_rho.json: rho2xks
  at kappa 0.05/0.1/0.25).
Decision rule and bootstrap exactly as PROTOCOL.md / analyze.py (median of the 22 per-configuration
ratios; random.Random(1), B = 4000; cheaper / dearer / unranked across h = 0, 0.1, 1).
  pricing    P0 = the protocol (h on H + L only)
             P1 = P0 + h per rhodp canonicalisation (ecbench prices prime-field rho's
                  canonicalisations_uncharged in the section-5a band; AGENTS.md 5a names them)
             P2 = P1 + h per step-index hash and DP test of the rho arms (maximal reading)
  candidates C0  = {ic5, ic5ks}                          (the producer's icbest)
             C0mu= C0 + ic5ks at mu 0.85, 0.7            (diagnostic: tuning mu alone)
             C1  = C0 + ic5rw at mu 1.0/0.85/0.7/0.6/0.5 (validator's non-degenerate IC retention)
  references R0  = rho2xks x3 + rhodp x12                (the producer's rhobest, 15 runs)
             R1  = R0 + rho2xks at kappa 0.05/0.1/0.25    (reference tuned downward too, 18 runs)
             R2  = R1 + rhodpu x12 (sens_dpu.json: rhodp with distinct step values, no uncaught fruitless
                   2-cycles; its canonicalisations = its walk-phase inversions, I at the 'targets'
                   snapshot minus the setup inversions of rhodp on the same curve and seed, exact to +-2;
                   P2 is not evaluated with R2)
  x suffix   = cross-fitted: one parameter set per (cell, cI, h) chosen on the OTHER seed's 11
               configurations (removes the min-over-many-runs selection noise from both sides)
usage: sens_analysis.py <frozen study dir> <replay_all.json> <sens.json> <sens_rho.json> <sens_dpu.json> <out.json>"""
import json, sys, os, random, statistics as st, collections
study, replay, sensf, sensrf, sensdf, out = sys.argv[1:7]
R = json.load(open(os.path.join(study, "results/holdout/runs.json")))
runs = [dict(zip(R["fields"], r), src="producer") for r in R["rows"]]
def rowsof(d):
    return [dict(zip(d["fields"], r)) for r in d["rows"]] if "fields" in d else d.get("rows", d.get("runs"))
RP = rowsof(json.load(open(replay)))
rows_ = [json.loads(l) for l in open(os.path.join(study, "results/holdout/rows.jsonl"))]
assert len(runs) == len(RP) == len(rows_)
for rp in RP:
    r = runs[rp["i"]]
    assert (r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"]) == (rp["arm"], rp["pb"], rp["T"], rp["idx"], rp["seed"], rp["param"])
    assert rp["H"] == r["H"] and rp["match"]
    r.update(canon=rp["canon"], pidx=rp["pidx"], dpt=rp["dpt"], hop=rp["hop"], I_targets=rows_[rp["i"]]["phases"]["targets"][0])
val = []
for f, ren in ((sensf, {"ic5rw": "ic5rw", "ic5w": "ic5w", "ic5ks": "ic5ks_mu"}), (sensrf, {"rho2xks": "rho2xks_lo"})):
    SX = json.load(open(f)); assert SX["meta"]["errors"] == 0 and SX["meta"]["unverified"] == 0, f
    for s in rowsof(SX):
        s = dict(s, arm=ren[s["arm"]], doff=None, src="validator", canon=0, pidx=0, dpt=0); val.append(s)
setup = {}
for r in runs:
    if r["arm"] == "rhodp":
        k = (r["pb"], r["idx"], r["seed"]); v = r["I_targets"] - r["canon"]
        setup[k] = min(setup.get(k, v), v)
SD = json.load(open(sensdf)); assert SD["meta"]["errors"] == 0 and SD["meta"]["unverified"] == 0, sensdf
for s in rowsof(SD):
    c = s["I_targets"] - setup[(s["pb"], s["idx"], s["seed"])]
    val.append(dict(s, src="validator", canon=c, pidx=None, dpt=None))
allruns = runs + val
cfg = collections.defaultdict(list)
for r in allruns: cfg[(r["pb"], r["T"], r["idx"], r["seed"])].append(r)
assert all(len(v) == 20 + 8 + 3 + 12 for v in cfg.values()), collections.Counter(len(v) for v in cfg.values())
cells = sorted({k[:2] for k in cfg}); CI = [3, 10, 30]; HP = [0, 0.1, 1]
def extra(r, P):
    if P == "P0": return 0
    if P == "P1": return r["canon"] if r["arm"] in ("rhodp", "rhodpu") else 0
    if P == "P2":
        assert r["arm"] != "rhodpu"
        return r["canon"] + r["pidx"] + r["dpt"] if r["arm"] in ("rhodp", "rho2xks") else 0
def cost(r, ci, h, P):
    return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * (r["H"] + r["L"] + extra(r, P))) / r["targets"]
SETS = {"C0": {"ic5", "ic5ks"}, "C0mu": {"ic5", "ic5ks", "ic5ks_mu"}, "C1": {"ic5", "ic5ks", "ic5rw"},
        "ic5rw": {"ic5rw"}, "ic5w": {"ic5w"}, "ic5": {"ic5"},
        "R0": {"rho2xks", "rhodp"}, "R1": {"rho2xks", "rhodp", "rho2xks_lo"}, "rho2xks": {"rho2xks"},
        "R2": {"rho2xks", "rhodp", "rho2xks_lo", "rhodpu"}, "rhodpu": {"rhodpu"},
        "rhodp": {"rhodp"}, "bsgs2": {"bsgs2"}}
def pkey(r): return (r["arm"], r["param"], r.get("doff"))
def choose(ks, arms, ci, h, P):
    ch = {}
    for s in (31, 32):
        acc = collections.defaultdict(list)
        for k in ks:
            if k[3] == s: continue
            for r in cfg[k]:
                if r["arm"] in arms: acc[pkey(r)].append(cost(r, ci, h, P))
        ch[s] = min(acc, key=lambda q: st.median(acc[q]))
    return ch
def val_(k, setname, ci, h, P, cross, ks):
    arms = SETS[setname]
    if not cross:
        return min(cost(r, ci, h, P) for r in cfg[k] if r["arm"] in arms)
    if setname.startswith("C"):   # ic5 and ic5ks kept as is (fixed parameters); ic5rw mu cross-fitted
        ch = choose(ks, {"ic5rw"} & arms, ci, h, P) if "ic5rw" in arms else None
        return min(cost(r, ci, h, P) for r in cfg[k] if r["arm"] in arms and (r["arm"] != "ic5rw" or pkey(r) == ch[k[3]]))
    ch = choose(ks, arms, ci, h, P)
    return min(cost(r, ci, h, P) for r in cfg[k] if pkey(r) == ch[k[3]])
def boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    m = sorted(st.median(rnd.choices(xs, k=n)) for _ in range(B)); return m[int(.025 * B)], m[int(.975 * B) - 1]
def run(cand, ref, P, cross):
    S = {}
    for ci in CI:
        for pb, T in cells:
            ks = [k for k in cfg if k[:2] == (pb, T)]; trip = []
            for h in HP:
                c = [val_(k, cand, ci, h, P, cross, ks) for k in ks]; y = [val_(k, ref, ci, h, P, cross, ks) for k in ks]
                rat = [a / b for a, b in zip(c, y)]; lo, hi = boot(rat)
                trip.append(dict(h=h, median=round(st.median(rat), 4), lo=round(lo, 4), hi=round(hi, 4), wins=sum(x < 1 for x in rat),
                                 cand_gae=round(st.median(c), 2), ref_gae=round(st.median(y), 2)))
            dec = "cheaper" if all(t["hi"] < 1 for t in trip) else "dearer" if all(t["lo"] > 1 for t in trip) else "unranked"
            S[f"{pb}/{T}/cI{ci}"] = dict(decision=dec, by_h=trip)
    return S
VARIANTS = [("C0", "R0", "P0", False), ("C0", "R0", "P1", False), ("C0", "R0", "P2", False),
            ("C0", "R0", "P0", True), ("C0", "R1", "P0", False),
            ("C0mu", "R0", "P0", False), ("ic5w", "R0", "P0", False),
            ("C1", "R0", "P0", False), ("C1", "R1", "P0", False), ("C1", "R1", "P1", False), ("C1", "R1", "P2", False),
            ("C1", "R1", "P0", True), ("C1", "R1", "P1", True),
            ("C0", "rho2xks", "P0", False), ("C0", "rhodp", "P0", False), ("C0", "rhodp", "P1", False),
            ("C0", "R2", "P0", False), ("C0", "R2", "P1", False), ("C1", "R2", "P0", False), ("C1", "R2", "P1", False),
            ("C1", "R2", "P0", True), ("C1", "R2", "P1", True), ("C0", "rhodpu", "P1", False)]
res = {}
for cand, ref, P, cross in VARIANTS:
    name = f"{cand}{'x' if cross else ''} vs {ref}{'x' if cross else ''}, {P}"
    S = run(cand, ref, P, cross)
    cnt = dict(collections.Counter(v["decision"] for v in S.values()))
    rng = lambda filt: {h: [min(v["by_h"][j]["median"] for c, v in S.items() if filt(c)), max(v["by_h"][j]["median"] for c, v in S.items() if filt(c))] for j, h in enumerate(HP)}
    res[name] = dict(decision_counts=cnt, median_range_all=rng(lambda c: True), median_range_cI10=rng(lambda c: c.endswith("cI10")), cells=S)
    m = res[name]["median_range_cI10"]
    print(f"{name:26s} {str(cnt):45s} cI10 h0 {m[0]} h0.1 {m[0.1]} h1 {m[1]}", flush=True)
# diagnostics
d = {}
w = collections.Counter(); g = []; g2 = []
for k in cfg:
    rw = [r for r in cfg[k] if r["arm"] == "ic5rw"]; b = min(rw, key=lambda r: cost(r, 10, 0, "P0")); w[b["param"]] += 1
    c0 = min(cost(r, 10, 0, "P0") for r in cfg[k] if r["arm"] in SETS["C0"]); g.append(cost(b, 10, 0, "P0") / c0)
    i5 = [r for r in cfg[k] if r["arm"] == "ic5"][0]; ks5 = [r for r in cfg[k] if r["arm"] == "ic5ks"][0]; i5w = [r for r in cfg[k] if r["arm"] == "ic5w"][0]
    g2.append((cost(ks5, 10, 0, "P0") / cost(i5, 10, 0, "P0"), cost(i5w, 10, 0, "P0") / cost(i5, 10, 0, "P0")))
d["ic5rw_best_mu_count_cI10_h0"] = dict(w)
d["ic5rw_best_over_C0_cI10_h0"] = [min(g), st.median(g), max(g)]
d["ic5ks_over_ic5_cI10_h0"] = [min(x for x, _ in g2), st.median(x for x, _ in g2), max(x for x, _ in g2)]
d["ic5w_over_ic5_cI10_h0"] = [min(y for _, y in g2), st.median(y for _, y in g2), max(y for _, y in g2)]
rk = collections.Counter(); rk1 = collections.Counter()
for k in cfg:
    b = min((r for r in cfg[k] if r["arm"] in SETS["R1"]), key=lambda r: cost(r, 10, 0, "P0")); rk[f'{b["arm"]} {b["param"]}'] += 1
    b = min((r for r in cfg[k] if r["arm"] in SETS["R1"]), key=lambda r: cost(r, 10, 1, "P1")); rk1[f'{b["arm"]} {b["param"]} {b.get("doff")}'] += 1
d["R1_winner_cI10_h0_P0"] = dict(rk); d["R1_winner_cI10_h1_P1"] = dict(rk1)
rk2 = collections.Counter(); rk3 = collections.Counter(); du = []
for k in cfg:
    b = min((r for r in cfg[k] if r["arm"] in SETS["R2"]), key=lambda r: cost(r, 10, 1, "P1")); rk2[f'{b["arm"]} {b["param"]} {b.get("doff")}'] += 1
    b = min((r for r in cfg[k] if r["arm"] in SETS["R2"]), key=lambda r: cost(r, 10, 1, "P0")); rk3[f'{b["arm"]} {b["param"]} {b.get("doff")}'] += 1
    a = min(cost(r, 10, 1, "P0") for r in cfg[k] if r["arm"] == "rhodpu"); b_ = min(cost(r, 10, 1, "P0") for r in cfg[k] if r["arm"] == "rhodp"); du.append(a / b_)
d["R2_winner_cI10_h1_P1"] = dict(rk2); d["R2_winner_cI10_h1_P0"] = dict(rk3)
d["rhodpu_over_rhodp_best_cI10_h1_P0"] = [min(du), st.median(du), max(du)]
ab = collections.defaultdict(lambda: [0, 0])
for r in val:
    if r["arm"] == "rhodpu": ab[r["pb"]][0] += r["abandoned"]; ab[r["pb"]][1] += r["walks"]
d["rhodpu_abandoned_per_walk"] = {k: v[0] / v[1] for k, v in ab.items()}
tot = collections.defaultdict(lambda: collections.Counter())
for r in runs:
    t = tot[r["arm"]]; t["runs"] += 1; t["H"] += r["H"]; t["hashmap_calls"] += r["hop"]; t["canonicalisations"] += r["canon"]
    t["step_index_hashes"] += r["pidx"]; t["dp_tests"] += r["dpt"]; t["targets"] += r["targets"]
d["native_work_totals"] = {k: dict(v) for k, v in tot.items()}
d["hashmap_calls_equal_H"] = {a: all(r["hop"] == r["H"] for r in runs if r["arm"] == a) for a in tot}
cp = [r["canon"] / r["targets"] for r in runs if r["arm"] == "rhodp"]
d["rhodp_canon_per_target"] = [min(cp), st.median(cp), max(cp)]
res["diagnostics"] = d
json.dump(res, open(out, "w"), indent=1)
print(json.dumps(d, indent=1))
