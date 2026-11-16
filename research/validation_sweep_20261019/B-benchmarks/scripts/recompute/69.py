# Row 69 recompute: 20261015-pkm-isogeny-tower. Reads committed files only. Run from repo root with python3 -I.
import json, glob, os, re, statistics
from collections import defaultdict
R = "research/pkm_isogeny_tower_ecbench_20261015"
sess = sorted(os.path.basename(p) for p in glob.glob(R + "/sessions/*") if os.path.isdir(p))
print("sessions:", sess)
def load(s):
    return [json.loads(x) for x in open(os.path.join(R, "sessions", s, "records.jsonl")) if x.strip()]
def paired(recs, a, b):
    by = defaultdict(dict)
    for r in recs:
        if r["warmup"]:
            continue
        by[(r["workload"]["workload_id"], r["round"])][r["arm"]] = r["cost"]["s"]
    pr = [(v[a], v[b]) for v in by.values() if a in v and b in v]
    return pr
table = {}
allS = {}
for s in sess:
    recs = load(s)
    arms = defaultdict(list)
    for r in recs:
        if not r["warmup"]:
            arms[r["arm"]].append(r["cost"]["s"])
    allS[s] = {a: statistics.fmean(v) for a, v in arms.items()}
    rho = allS[s]["rho-neg"]
    iso_plain = paired(recs, "ic-plain", "ic-iso-rootfind")
    iso_kum = paired(recs, "ic-kummer-rootfind", "ic-iso-rootfind")
    aa = paired(recs, "rho-neg", "rho-neg-aa")
    print("   arms:", sorted(arms.keys()))
    ratio = lambda pr: (sum(b for a, b in pr) / sum(a for a, b in pr)) if pr else float("nan")
    r1 = ratio(iso_plain); r2 = ratio(iso_kum); raa = ratio(aa)
    ic_over_rho = {a: allS[s][a] / rho for a in allS[s] if a.startswith("ic-")}
    table[s] = (r1, r2, raa, ic_over_rho, len(iso_plain), len(iso_kum), len(aa))
    print("%s n_pairs plain=%d kummer=%d aa=%d | iso/plain=%.4f iso/kummer=%.4f rho-neg-aa/rho-neg=%.4f" % (s, len(iso_plain), len(iso_kum), len(aa), r1, r2, raa))
    print("   IC/rho:", {k: round(v, 2) for k, v in ic_over_rho.items()})
vals1 = [t[0] for t in table.values() if t[0] == t[0]]; vals2 = [t[1] for t in table.values() if t[1] == t[1]]
print("iso/plain range over sessions: min %.4f max %.4f" % (min(vals1), max(vals1)))
print("iso/kummer range over sessions: min %.4f max %.4f" % (min(vals2), max(vals2)))
icr = [v for t in table.values() for v in t[3].values()]
print("IC/rho range over all IC arms and sessions: min %.2f max %.2f" % (min(icr), max(icr)))
# audits
ver = 0; tot = 0; rep_ok = 0; rep_tot = 0
for f in sorted(glob.glob(R + "/audits/*.audit.json")):
    a = json.load(open(f))
    ver += a["verified_records"]; tot += a["records"]
    rep_tot += len(a["replays"]); rep_ok += sum(1 for x in a["replays"] if x.get("reproduced") and x.get("detail") == "identical")
print("audits: verified %d/%d ; replays identical %d/%d" % (ver, tot, rep_ok, rep_tot))
# curves tried per factor base
tried = defaultdict(list)
for s in sess:
    for r in load(s):
        fb = r.get("factor_base") or {}
        d = fb.get("description") or ""
        m = re.search(r"(\d+) auxiliary curves tried", d)
        t = (fb.get("params") or {}).get("t")
        if m and t is not None and not r["warmup"]:
            tried[(fb["fb_id"], int(t))].append(int(m.group(1)))
builds = {}
for (fb_id, t), v in tried.items():
    builds[fb_id] = (t, v[0])
print("distinct factor-base builds with tried counts:", len(builds))
bt = defaultdict(list)
for fb_id, (t, n) in builds.items():
    bt[t].append(n)
for t in sorted(bt):
    print("t=%d builds=%d mean tried=%.2f 1.05*2^t=%.2f ratio=%.3f" % (t, len(bt[t]), statistics.fmean(bt[t]), 1.05 * 2 ** t, statistics.fmean(bt[t]) / (1.05 * 2 ** t)))
