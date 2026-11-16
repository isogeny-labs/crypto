#!/usr/bin/env python3
"""Validator recompute (independent of analyze.py): per-configuration ratios IC4/X from runs.json, medians,
bootstrap CIs (own numpy bootstrap; plus a re-implementation of the CI procedure PROTOCOL/analyze declares,
to check the reported bounds), section-5a verdicts, and comparison with summary.json and RESULTS.md.
usage: recompute.py <study dir> <out json>"""
import json, sys, os, re, random, collections
import numpy as np

D, OUT = sys.argv[1], sys.argv[2]
H = os.path.join(D, "results", "holdout")
rj = json.load(open(os.path.join(H, "runs.json")))
F = rj["fields"]
runs = [dict(zip(F, r)) for r in rj["rows"]]
summary = json.load(open(os.path.join(H, "summary.json")))

CIS = [3, 10, 30]; HS = [0, 0.1, 1]
RHO = ["rho", "rho2", "rho2x", "rhoseq"]
REFS = ["rhobest", "rho", "rho2", "rho2x", "rhoseq", "bsgs2"]

def gae_per_target(r, ci, h):
    # PROTOCOL: [(c_I I + M + S)/(c_I + 3) + h (H + L)] / T ; 1 GAE = I + 2M + S at S = M, I = c_I M
    one_gae_in_M = ci + 2 + 1
    return ((ci * r["I"] + r["M"] + r["S"]) / one_gae_in_M + h * (r["H"] + r["L"])) / r["targets"]

assert all(r["ok"] == r["targets"] for r in runs)
conf = collections.defaultdict(list)
for r in runs:
    conf[(r["pb"], r["T"], r["idx"], r["seed"])].append(r)
cells = sorted({k[:2] for k in conf})
keys = {c: sorted(k for k in conf if k[:2] == c) for c in cells}

def ref_cost(rs, ref, ci, h):
    if ref == "rhobest":
        cand = [r for r in rs if r["arm"] in RHO]
        assert len(cand) == 12
    else:
        cand = [r for r in rs if r["arm"] == ref]
        assert len(cand) == 3
    costs = [(gae_per_target(r, ci, h), r["arm"], r["param"]) for r in cand]
    return min(costs)

def my_boot(x, B=20000, seed=0):
    g = np.random.default_rng(seed)
    x = np.asarray(x)
    idx = g.integers(0, len(x), size=(B, len(x)))
    med = np.median(x[idx], axis=1)
    return float(np.quantile(med, 0.025)), float(np.quantile(med, 0.975))

def declared_boot(x, B=4000, seed=1):
    # the CI procedure analyze.py declares: Python random.Random(seed), B resamples with replacement,
    # sorted medians, indices floor(.025 B) and floor(.975 B) - 1
    rnd = random.Random(seed); n = len(x)
    meds = sorted(_median(rnd.choices(x, k=n)) for _ in range(B))
    return meds[int(.025 * B)], meds[int(.975 * B) - 1]

def _median(v):
    s = sorted(v); m = len(s)
    return s[m // 2] if m % 2 else (s[m // 2 - 1] + s[m // 2]) / 2

rows = []
winners = collections.Counter(); winner_cells = collections.defaultdict(collections.Counter)
for ci in CIS:
    for c in cells:
        for h in HS:
            ic = []
            for k in keys[c]:
                rs = conf[k]; icr = [r for r in rs if r["arm"] == "ic4"]; assert len(icr) == 1
                ic.append(gae_per_target(icr[0], ci, h))
            for ref in REFS:
                refc = [ref_cost(conf[k], ref, ci, h) for k in keys[c]]
                if ref == "rhobest":
                    for rc in refc:
                        winners[rc[1]] += 1; winner_cells[(c, ci, h)][rc[1]] += 1
                rat = [a / b[0] for a, b in zip(ic, refc)]
                lo, hi = my_boot(rat)
                lo2, hi2 = my_boot(rat, seed=12345)
                dlo, dhi = declared_boot(rat)
                rows.append(dict(cI=ci, bits=c[0], T=c[1], h=h, vs=ref, median=float(np.median(rat)), n=len(rat),
                                 lo=lo, hi=hi, lo_seed2=lo2, hi_seed2=hi2, dlo=dlo, dhi=dhi,
                                 wins=sum(x < 1 for x in rat), ic=float(np.median(ic)),
                                 ref=float(np.median([b[0] for b in refc]))))

def find(lst, **kw):
    out = [s for s in lst if all(s[k] == v for k, v in kw.items())]
    assert len(out) == 1, (kw, len(out)); return out[0]

# compare with summary.json
cmp = []; worst = 0.0; worst_ci = 0.0; worst_myci = 0.0; n_fields = 0
assert len(summary) == len(rows), (len(summary), len(rows))
for s in summary:
    m = find(rows, cI=s["cI"], bits=s["bits"], T=s["T"], h=s["h"], vs=s["vs"])
    rel = abs(m["median"] - s["median"]) / abs(s["median"])
    worst = max(worst, rel)
    worst_ci = max(worst_ci, abs(m["dlo"] - s["lo"]), abs(m["dhi"] - s["hi"]))
    worst_myci = max(worst_myci, abs(m["lo"] - s["lo"]), abs(m["hi"] - s["hi"]))
    for f in ("wins", "n"):
        assert m[f] == s[f], (s, m)
    for f in ("ic", "ref"):
        assert abs(m[f] - s[f]) <= 1e-9 * abs(s[f]), (f, s, m)
    n_fields += 1

def verdict(lst, ci, c, ref, lo="lo", hi="hi"):
    ss = [find(lst, cI=ci, bits=c[0], T=c[1], h=h, vs=ref) for h in HS]
    if all(s[hi] < 1 for s in ss): return "cheaper"
    if all(s[lo] > 1 for s in ss): return "dearer"
    return "unranked"

verd = {}
for ref in REFS:
    for ci in CIS:
        for c in cells:
            verd[(ref, ci, c)] = dict(
                claimed=verdict(summary, ci, c, ref),
                mine=verdict(rows, ci, c, ref),
                mine_seed2=verdict(rows, ci, c, ref, "lo_seed2", "hi_seed2"),
                declared_repro=verdict(rows, ci, c, ref, "dlo", "dhi"))
vd = {f"{ref}|cI{ci}|{c[0]}b|T{c[1]}": v for (ref, ci, c), v in verd.items()}
disagree = {k: v for k, v in vd.items() if len(set(v.values())) > 1}
counts = {}
for ref in REFS:
    for who in ("claimed", "mine", "mine_seed2", "declared_repro"):
        counts[f"{ref}:{who}"] = dict(collections.Counter(v[who] for (r, ci, c), v in verd.items() if r == ref))

# ranges quoted in RESULTS.md / the proposed row
def rng(lst, **kw):
    v = [s["median"] for s in lst if all((s[k] in val) if isinstance(val, (list, tuple, set)) else s[k] == val for k, val in kw.items())]
    return [min(v), max(v), len(v)]
ranges = {
    "rhobest_T16384_all_cI_h": rng(rows, vs="rhobest", T=16384),
    "rhobest_T4096_all": rng(rows, vs="rhobest", T=4096),
    "rhobest_T4096_h1": rng(rows, vs="rhobest", T=4096, h=1),
    "rhobest_T1024_all": rng(rows, vs="rhobest", T=1024),
    "bsgs2_all": rng(rows, vs="bsgs2"),
    "bsgs2_T16384": rng(rows, vs="bsgs2", T=16384),
    "rho_all": rng(rows, vs="rho"),
}
for ref in REFS:
    for c in cells:
        ranges[f"{ref}_{c[0]}b_T{c[1]}"] = rng(rows, vs=ref, bits=c[0], T=c[1])

# RESULTS.md headline table parse and check (3-decimal / 2-decimal rounding)
md = open(os.path.join(D, "RESULTS.md")).read()
tab = [l for l in md.splitlines() if re.match(r"\| \d\d b, T = \d+ \|", l)]
res_checks = []
num = r"(\d+\.\d+)"
for l in tab:
    cols = [x.strip() for x in l.strip("|").split("|")]
    bits, T = map(int, re.match(r"(\d+) b, T = (\d+)", cols[0]).groups())
    c1 = [float(x) for x in re.findall(num, cols[1])]  # I=3: h0 [lo,hi] / h0.1 / h1 [lo,hi]
    c2 = [float(x) for x in re.findall(num, cols[2])]  # I=30: h0 / h1
    c3 = [x.strip() for x in cols[3].split("/")]
    c4 = [float(x) for x in re.findall(num, cols[4])]  # bsgs2 I=3: h0 / h1
    exp = [
        ("I3_h0_median", c1[0], find(rows, cI=3, bits=bits, T=T, h=0, vs="rhobest")["median"], 3),
        ("I3_h0_lo", c1[1], find(rows, cI=3, bits=bits, T=T, h=0, vs="rhobest")["dlo"], 3),
        ("I3_h0_hi", c1[2], find(rows, cI=3, bits=bits, T=T, h=0, vs="rhobest")["dhi"], 3),
        ("I3_h0.1_median", c1[3], find(rows, cI=3, bits=bits, T=T, h=0.1, vs="rhobest")["median"], 3),
        ("I3_h1_median", c1[4], find(rows, cI=3, bits=bits, T=T, h=1, vs="rhobest")["median"], 3),
        ("I3_h1_lo", c1[5], find(rows, cI=3, bits=bits, T=T, h=1, vs="rhobest")["dlo"], 3),
        ("I3_h1_hi", c1[6], find(rows, cI=3, bits=bits, T=T, h=1, vs="rhobest")["dhi"], 3),
        ("I30_h0_median", c2[0], find(rows, cI=30, bits=bits, T=T, h=0, vs="rhobest")["median"], 3),
        ("I30_h1_median", c2[1], find(rows, cI=30, bits=bits, T=T, h=1, vs="rhobest")["median"], 3),
        ("bsgs2_I3_h0", c4[0], find(rows, cI=3, bits=bits, T=T, h=0, vs="bsgs2")["median"], 2),
        ("bsgs2_I3_h1", c4[1], find(rows, cI=3, bits=bits, T=T, h=1, vs="bsgs2")["median"], 2),
    ]
    for name, cl, mine, nd in exp:
        res_checks.append(dict(cell=f"{bits}b T{T}", name=name, claimed=cl, recomputed=round(mine, 6),
                               ok=abs(round(mine, nd) - cl) < 1e-9))
    for ci, v in zip(CIS, c3):
        mv = verd[("rhobest", ci, (bits, T))]["mine"]
        res_checks.append(dict(cell=f"{bits}b T{T}", name=f"verdict_cI{ci}", claimed=v, recomputed=mv, ok=v == mv))

out = dict(rows=rows, summary_compare=dict(n=n_fields, max_rel_median_diff=worst,
                                            max_abs_ci_diff_declared_method=worst_ci,
                                            max_abs_ci_diff_my_bootstrap=worst_myci),
           verdicts=vd, verdict_disagreements=disagree, verdict_counts=counts, ranges=ranges,
           rhobest_winner_arm_total=dict(winners),
           rhobest_winner_by_setting={f"{c[0]}b_T{c[1]}_cI{ci}_h{h}": dict(v) for (c, ci, h), v in winner_cells.items()},
           results_md_checks=res_checks)
json.dump(out, open(OUT, "w"), indent=1)
print("summary entries", n_fields, "max rel median diff", worst, "max |ci diff| declared", worst_ci, "mine", worst_myci)
print("verdict disagreements", json.dumps(disagree, indent=1))
print("counts", json.dumps({k: v for k, v in counts.items() if k.startswith(("rhobest", "bsgs2"))}, indent=1))
print("ranges", json.dumps({k: v for k, v in ranges.items() if "_T" not in k or k.startswith(("rhobest_T", "bsgs2_T"))}, indent=1))
print("RESULTS.md checks failing:", [x for x in res_checks if not x["ok"]], "of", len(res_checks))
print("winners", dict(winners))
