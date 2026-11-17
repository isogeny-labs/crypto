#!/usr/bin/env python3
"""Validator: Study B (rho restart correction) checks against own recomputes.
 - merged.jsonl = v2 ic4/bsgs2 rows (byte-identical lines of the v2 rows.jsonl) + the corrected rho rows (rows.jsonl);
 - the corrected rho rows use the same argv as the v2 rho rows (same jobs);
 - RESULTS.md table: v2-as-published and corrected IC4/rhobest ranges and decisions per (cell, cI);
 - prose: rho reference 1.4%-4.8% too expensive; T = 16384 margin 1.6%-4.9% (median 0.951-0.991);
   25 b, cI = 30, h = 1 margin 0.9%; three (cell, cI) at T = 4096 move from unranked to dearer.
usage: results_check_b.py <frozen research dir> <recompute_b.json> <recompute_v2orig.json> <out.json>"""
import json, sys, re, collections
FR, RB, RV2, OUT = sys.argv[1:5]
B = f"{FR}/mt_rho_start_fix_20261111"; V2 = f"{FR}/mt_fieldops_v2_20261111"
v2lines = open(f"{V2}/results/holdout/rows.jsonl").read().splitlines()
mlines = open(f"{B}/results/merged.jsonl").read().splitlines()
nlines = open(f"{B}/results/rows.jsonl").read().splitlines()
RHO = {"rho", "rho2", "rho2x", "rhoseq"}
keep = [l for l in v2lines if json.loads(l)["arm"] not in RHO]
merge_ok = mlines == keep + nlines
v2rho = [json.loads(l) for l in v2lines if json.loads(l)["arm"] in RHO]
nrho = [json.loads(l) for l in nlines]
def jobkey(r): return (r["arm"], r["pb"], r["T"], r["idx"], r["seed"], float(r["param"]), tuple(r["argv"]))
same_jobs = sorted(map(jobkey, v2rho)) == sorted(map(jobkey, nrho))
rb = json.load(open(RB)); rv = json.load(open(RV2))
def ent(rec, vs="rhobest"):
    return {(e["cI"], e["bits"], e["T"], e["h"]): e for e in rec["entries"] if e["vs"] == vs}
eb, ev = ent(rb), ent(rv)
cells = sorted({(k[1], k[2]) for k in eb})
tab = []
txt = open(f"{B}/RESULTS.md").read(); mism = []
for line in txt.splitlines():
    m = re.match(r"\| (\d+) \| (\d+) \| ([\d.]+)-([\d.]+) \| ([\d.]+)-([\d.]+) \| \*{0,2}(\w+)\*{0,2} \| \*{0,2}(\w+) / (\w+) / (\w+)\*{0,2} \|", line)
    if not m: continue
    pb, T = int(m.group(1)), int(m.group(2))
    v2r = [ev[(ci, pb, T, h)]["median"] for ci in [3, 10, 30] for h in [0, 0.1, 1]]
    nr = [eb[(ci, pb, T, h)]["median"] for ci in [3, 10, 30] for h in [0, 0.1, 1]]
    c = dict(cell=f"{pb}/{T}", v2_claimed=[float(m.group(3)), float(m.group(4))], v2_recomputed=[min(v2r), max(v2r)],
             corr_claimed=[float(m.group(5)), float(m.group(6))], corr_recomputed=[min(nr), max(nr)],
             v2_dec_claimed=m.group(7), corr_dec_claimed=list(m.group(8, 9, 10)),
             v2_dec_recomputed=[rv["decisions"][f"ic4|rhobest|{ci}|{pb}|{T}"]["frozen"] for ci in [3, 10, 30]],
             corr_dec_recomputed=[rb["decisions"][f"ic4|rhobest|{ci}|{pb}|{T}"]["frozen"] for ci in [3, 10, 30]])
    c["ok"] = (round(c["v2_recomputed"][0], 3), round(c["v2_recomputed"][1], 3)) == tuple(c["v2_claimed"]) and \
              (round(c["corr_recomputed"][0], 3), round(c["corr_recomputed"][1], 3)) == tuple(c["corr_claimed"]) and \
              c["corr_dec_claimed"] == c["corr_dec_recomputed"] and all(c["v2_dec_claimed"] == d for d in c["v2_dec_recomputed"])
    tab.append(c)
    if not c["ok"]: mism.append(c)
# rhobest cost change per (cell, cI, h): new ref / old ref (medians of per-config rhobest) and per-config
ch = []
for k in eb: ch.append(1 - eb[k]["ref"] / ev[k]["ref"])
t16 = [eb[k]["median"] for k in eb if k[2] == 16384]
m25 = eb[(30, 25, 16384, 1)]
moved = [f"{pb}/{T}/cI{ci}" for pb, T in cells for ci in [3, 10, 30]
         if rv["decisions"][f"ic4|rhobest|{ci}|{pb}|{T}"]["frozen"] != rb["decisions"][f"ic4|rhobest|{ci}|{pb}|{T}"]["frozen"]]
# ic4/bsgs2 unchanged?
bs_b, bs_v = ent(rb, "bsgs2"), ent(rv, "bsgs2")
bs_same = all(abs(bs_b[k]["median"] - bs_v[k]["median"]) == 0 for k in bs_b)
# per-arm rho cost change per config (cI = 10, h = 0.1)
def cost(r, ci=10, h=0.1): return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * (r["H"] + r["L"])) / r["targets"]
old = {jobkey(r): r for r in v2rho}
per_arm = collections.defaultdict(list)
for r in nrho: per_arm[r["arm"]].append(cost(r) / cost(old[jobkey(r)]))
arm_change = {a: dict(min=min(v), median=sorted(v)[len(v) // 2], max=max(v), frac_cheaper=sum(x < 1 for x in v) / len(v)) for a, v in per_arm.items()}
# phase-1 steps per database point, old vs new (rho, kappa 1.0, 25 b, T 16384 as quoted)
spp = collections.defaultdict(list)
for tag, rs in (("v2", v2rho), ("fixed", nrho)):
    for r in rs: spp[(tag, r["arm"], r["pb"], r["T"], r["param"])].append(r["db_steps"] / r["db"])
spp_q = {f"{k[0]}/{k[1]}/{k[2]}/{k[3]}/{k[4]}": max(v) for k, v in spp.items() if k[1] in ("rho", "rho2") and k[3] == 16384}
out = dict(merge_exact=merge_ok, merged_lines=len(mlines), kept_v2_lines=len(keep), new_rho_lines=len(nlines),
           new_rho_same_jobs_as_v2=same_jobs, new_rho_unverified=sum(r["ok"] != r["targets"] for r in nrho),
           table=tab, table_mismatches=mism,
           rhobest_ref_reduction_range=[min(ch), max(ch)], corrected_T16384_median_range=[min(t16), max(t16)],
           m25_cI30_h1=dict(median=m25["median"], lo=m25["lo"], hi=m25["hi"]), decisions_moved=moved,
           bsgs2_ratios_unchanged=bs_same, per_arm_cost_ratio_new_over_v2_cI10_h01=arm_change,
           phase1_steps_per_db_point_max_T16384=spp_q,
           decision_counts_v2=rv["decision_counts"].get("ic4 vs rhobest"), decision_counts_corrected=rb["decision_counts"].get("ic4 vs rhobest"))
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps({k: v for k, v in out.items() if k not in ("table",)}, indent=1))
