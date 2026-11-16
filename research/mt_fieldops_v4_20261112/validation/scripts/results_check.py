#!/usr/bin/env python3
"""Check every number of RESULTS.md's I = 10 M table and noverify.txt against the validator recompute.
noverify is recomputed from rows.jsonl phases: counters at the 'targets' snapshot (the phase before
'verify', for every arm), icbest/rhobest re-minimised per configuration on those costs.
usage: results_check.py <frozen study dir> <recompute.json> <out.json>"""
import json, sys, os, re, statistics as st, collections
study, rec, out = sys.argv[1:4]
RC = json.load(open(rec)); T10 = RC["table_cI10"]
def get(ref, pb, T, h, f="median"):
    s = [x for x in T10 if x["vs"] == ref and x["bits"] == pb and x["T"] == T and x["h"] == h]
    assert len(s) == 1; return s[0][f] if f != "ci" else (s[0]["lo"], s[0]["hi"])
md = open(os.path.join(study, "RESULTS.md")).read()
checks = []; bad = []
def chk(name, claimed, val, nd):
    ok = claimed == round(val, nd) or f"{claimed:.{nd}f}" == f"{val:.{nd}f}"
    checks.append(dict(name=name, claimed=claimed, recomputed=val, ok=ok))
    if not ok: bad.append(checks[-1])
for line in md.splitlines():
    m = re.match(r"\| (24|28|32) \| (\d+) \| (.*)\|$", line)
    if not m: continue
    pb, T = int(m.group(1)), int(m.group(2)); cols = [c.strip() for c in m.group(3).split("|")]
    for h, c in zip([0, 0.1, 1], cols[:3]):
        mm = re.match(r"([\d.]+) \[([\d.]+), ([\d.]+)\]", c)
        chk(f"{pb}/{T} icbest/rhobest h{h} median", float(mm.group(1)), get("rhobest", pb, T, h), 3)
        chk(f"{pb}/{T} icbest/rhobest h{h} lo", float(mm.group(2)), get("rhobest", pb, T, h, "lo"), 3)
        chk(f"{pb}/{T} icbest/rhobest h{h} hi", float(mm.group(3)), get("rhobest", pb, T, h, "hi"), 3)
    chk(f"{pb}/{T} icbest/rho2xks h0", float(cols[3]), get("rho2xks", pb, T, 0), 3)
    chk(f"{pb}/{T} icbest/rhodp h1", float(cols[4]), get("rhodp", pb, T, 1), 3)
    chk(f"{pb}/{T} icbest/bsgs2 h0", float(cols[5]), get("bsgs2", pb, T, 0), 3)
    a, b = [float(x) for x in cols[6].split(",")]
    chk(f"{pb}/{T} GAE/t icbest h0", a, get("rhobest", pb, T, 0, "ic"), 1)
    chk(f"{pb}/{T} GAE/t rhobest h0", b, get("rhobest", pb, T, 0, "ref"), 1)
# noverify
rows = [json.loads(l) for l in open(os.path.join(study, "results/holdout/rows.jsonl"))]
def nv(r, ci, h):
    ph = list(r["phases"].items()); assert ph[-1][0] == "verify" and ph[-2][0] == "targets"
    I, M, S, H, L = ph[-2][1]
    return ((ci * I + M + S) / (ci + 3) + h * (H + L)) / r["targets"]
cfg = collections.defaultdict(list)
for r in rows: cfg[(r["pb"], r["T"], r["idx"], r["seed"])].append(r)
nvt = {}
for (pb, T) in sorted({k[:2] for k in cfg}):
    ks = [k for k in cfg if k[:2] == (pb, T)]; vals = []
    for h in [0, 0.1, 1]:
        rat = [min(nv(r, 10, h) for r in cfg[k] if r["arm"] in ("ic5", "ic5ks")) / min(nv(r, 10, h) for r in cfg[k] if r["arm"] in ("rho2xks", "rhodp")) for k in ks]
        vals.append(st.median(rat))
    nvt[(pb, T)] = vals
nvtxt = open(os.path.join(study, "results/holdout/noverify.txt")).read()
for line in nvtxt.splitlines():
    m = re.match(r"\| (24|28|32) \| (\d+) \| ([\d.]+) / ([\d.]+) / ([\d.]+) \|", line)
    if m:
        pb, T = int(m.group(1)), int(m.group(2))
        for j, h in enumerate([0, 0.1, 1]):
            chk(f"noverify {pb}/{T} h{h}", float(m.group(3 + j)), nvt[(pb, T)][j], 3)
nvr = {h: [min(v[j] for v in nvt.values()), max(v[j] for v in nvt.values())] for j, h in enumerate([0, 0.1, 1])}
# verification GAE per target (cI = 10) per bits, all arms
ver = collections.defaultdict(list)
for r in rows:
    ph = list(r["phases"].values()); d = [a - b for a, b in zip(ph[-1], ph[-2])]
    ver[r["pb"]].append(((10 * d[0] + d[1] + d[2]) / 13) / r["targets"])
res = dict(checks=len(checks), mismatches=len(bad), bad=bad, noverify_ranges_cI10=nvr,
           verify_gae_per_target_cI10={k: [min(v), max(v)] for k, v in ver.items()}, all_checks=checks)
json.dump(res, open(out, "w"), indent=1)
print(json.dumps({k: res[k] for k in ("checks", "mismatches", "bad", "noverify_ranges_cI10", "verify_gae_per_target_cI10")}, indent=1))
