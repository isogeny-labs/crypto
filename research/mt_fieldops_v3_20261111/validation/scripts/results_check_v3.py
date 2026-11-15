#!/usr/bin/env python3
"""Validator: check the RESULTS.md main table, the 5a decision column, noverify.txt and the prose numbers of
Study A against the validator's own recompute (out/recompute_v3.json) and rows.jsonl phases.
usage: results_check_v3.py <RESULTS.md> <recompute_v3.json> <rows.jsonl> <noverify.txt> <out.json>"""
import json, sys, re, statistics, collections
RES, REC, ROWS, NOV, OUT = sys.argv[1:6]
rec = json.load(open(REC)); E = rec["entries"]; D = rec["decisions"]
def rng(cand, vs, pb, T):
    ms = [e["median"] for e in E if e["cand"] == cand and e["vs"] == vs and e["bits"] == pb and e["T"] == T]
    return min(ms), max(ms)
txt = open(RES).read()
checks = []; mism = []
for line in txt.splitlines():
    m = re.match(r"\| (\d+) \| (\d+) \| ([\d.]+)-([\d.]+) \| ([\d.]+)-([\d.]+) \| ([\d.]+)-([\d.]+) \| ([\d.]+)-([\d.]+) \| (\w+) / (\w+) / (\w+) \|", line)
    if not m: continue
    pb, T = int(m.group(1)), int(m.group(2))
    claimed = [(float(m.group(3 + 2 * i)), float(m.group(4 + 2 * i))) for i in range(4)]
    for (cand, vs), (clo, chi) in zip([("ic5", "rhobest"), ("ic5", "bsgs2"), ("ic5", "ic4"), ("ic4", "rhobest")], claimed):
        lo, hi = rng(cand, vs, pb, T)
        ok = round(lo, 3) == clo and round(hi, 3) == chi
        checks.append(dict(cell=f"{pb}/{T}", pair=f"{cand}/{vs}", claimed=[clo, chi], recomputed=[lo, hi], ok=ok))
        if not ok: mism.append(checks[-1])
    for ci, word in zip([3, 10, 30], m.group(11, 12, 13)):
        d = D[f"ic4|rhobest|{ci}|{pb}|{T}"]["frozen"]
        checks.append(dict(cell=f"{pb}/{T}", pair=f"ic4 vs rhobest decision cI={ci}", claimed=word, recomputed=d, ok=word == d))
        if word != d: mism.append(checks[-1])
# global ranges quoted in prose
def grange(cand, vs): ms = [e["median"] for e in E if e["cand"] == cand and e["vs"] == vs]; return min(ms), max(ms)
prose = {k: grange(*k.split("/")) for k in ["ic5/rhobest", "ic5/bsgs2", "ic5/ic4", "ic4/rhobest", "ic4/bsgs2"]}
# noverify: cost without the verify phase at cI = 10
rows = [json.loads(l) for l in open(ROWS)]
RHO = ["rho", "rho2", "rho2x", "rhoseq"]
def c_nv(r, ci, h):
    ph = r["phases"]; keys = list(ph); last = ph[keys[-2]]  # snapshot before verify
    I, M, S, H, L = last
    return ((ci * I + M + S) / (ci + 3) + h * (H + L)) / r["targets"]
cfg = collections.defaultdict(lambda: collections.defaultdict(list))
for r in rows: cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]].append(r)
def best(k, a, ci, h):
    rs = [r for x in RHO for r in cfg[k][x]] if a == "rhobest" else cfg[k][a]
    return min(c_nv(r, ci, h) for r in rs)
nv = {}
for pb, T in sorted({k[:2] for k in cfg}):
    ks = [k for k in cfg if k[:2] == (pb, T)]
    for cand, vs in [("ic5", "rhobest"), ("ic5", "bsgs2"), ("ic4", "rhobest")]:
        nv[f"{pb}/{T}/{cand}/{vs}"] = [round(statistics.median([best(k, cand, 10, h) / best(k, vs, 10, h) for k in ks]), 3) for h in [0, 0.1, 1]]
nov_txt = open(NOV).read(); nv_mism = []
for line in nov_txt.splitlines():
    m = re.match(r"\| (\d+) \| (\d+) \| ([\d.]+) / ([\d.]+) / ([\d.]+) \| ([\d.]+) / ([\d.]+) / ([\d.]+) \| ([\d.]+) / ([\d.]+) / ([\d.]+) \|", line)
    if not m: continue
    pb, T = m.group(1), m.group(2)
    for j, (cand, vs) in enumerate([("ic5", "rhobest"), ("ic5", "bsgs2"), ("ic4", "rhobest")]):
        cl = [float(m.group(3 + 3 * j + i)) for i in range(3)]
        if cl != nv[f"{pb}/{T}/{cand}/{vs}"]: nv_mism.append((pb, T, cand, vs, cl, nv[f"{pb}/{T}/{cand}/{vs}"]))
nv_ranges = {k: (min(v for kk, vv in nv.items() if kk.endswith(k) for v in vv), max(v for kk, vv in nv.items() if kk.endswith(k) for v in vv))
             for k in ["ic5/rhobest", "ic5/bsgs2", "ic4/rhobest"]}
# verification phase share (GAE/target at cI = 10)
ver = collections.defaultdict(list)
for r in rows:
    ph = r["phases"]; ks = list(ph); a, b = ph[ks[-2]], ph[ks[-1]]
    ver[r["pb"]].append(((10 * (b[0] - a[0]) + (b[1] - a[1]) + (b[2] - a[2])) / 13) / r["targets"])
ver_range = {pb: (min(v), max(v)) for pb, v in ver.items()}
# ic5 structural numbers quoted in prose (32 b, T = 1024)
ic5_32 = [r for r in rows if r["arm"] == "ic5" and r["pb"] == 32 and r["T"] == 1024]
ic4_32 = [r for r in rows if r["arm"] == "ic4" and r["pb"] == 32 and r["T"] == 1024]
struct = dict(ic5_m=sorted({r["m"] for r in ic5_32}), ic5_relations=[min(r["relations"] for r in ic5_32), statistics.median(r["relations"] for r in ic5_32), max(r["relations"] for r in ic5_32)],
              ic4_m=sorted({r["m"] for r in ic4_32}), ic4_relations=sorted({r["relations"] for r in ic4_32}),
              ic5_trials_per_target=[min(r["trials"] / r["targets"] for r in ic5_32), statistics.median(r["trials"] / r["targets"] for r in ic5_32), max(r["trials"] / r["targets"] for r in ic5_32)],
              n_over_2K=[min(r["n"] / (2 * r["table"]) for r in ic5_32), max(r["n"] / (2 * r["table"]) for r in ic5_32)],
              row0=dict(trials_per_target=ic5_32[0]["trials"] / ic5_32[0]["targets"], n_over_2K=ic5_32[0]["n"] / (2 * ic5_32[0]["table"]), table=ic5_32[0]["table"]))
# ic5 relation+LA phase share of total (cI = 10, h = 0.1) per cell
share = collections.defaultdict(list)
def g(v, ci=10, h=0.1): return (ci * v[0] + v[1] + v[2]) / (ci + 3) + h * (v[3] + v[4])
for r in rows:
    if r["arm"] != "ic5": continue
    ph = r["phases"]; tot = g([r["I"], r["M"], r["S"], r["H"], r["L"]])
    share[(r["pb"], r["T"])].append(dict(rel=(g(ph["relations"]) - g(ph["table"])) / tot, table=(g(ph["table"]) - g(ph["combos"])) / tot,
                                          combos=(g(ph["combos"]) - g(ph["fb"])) / tot, targets=(g(ph["targets"]) - g(ph["relations"])) / tot,
                                          verify=(g(ph["verify"]) - g(ph["targets"])) / tot, fb=g(ph["fb"]) / tot))
share_med = {f"{k[0]}/{k[1]}": {f: statistics.median(x[f] for x in v) for f in v[0]} for k, v in sorted(share.items())}
out = dict(table_checks=len(checks), table_mismatches=mism, prose_global_ranges=prose, noverify_mismatches=nv_mism,
           noverify_ranges=nv_ranges, verify_gae_per_target_cI10=ver_range, ic5_struct_32b_T1024=struct, ic5_phase_share_cI10_h01=share_med)
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps({k: v for k, v in out.items() if k not in ("ic5_phase_share_cI10_h01",)}, indent=1))
for k, v in share_med.items(): print(k, {f: round(x, 4) for f, x in v.items()})
