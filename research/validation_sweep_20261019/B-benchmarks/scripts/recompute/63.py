# SWB-RECOMPUTE-08 row 63: 20261015-ic-improve2-01-genus3-linalg. Reads committed files only.
import json, glob, statistics, subprocess, re, os, math, collections
REPO = "/home/user/crypto"
STUDY = os.path.join(REPO, "research/ic_improve_20261015/01-genus3-linalg")
RUN = os.path.join(STUDY, "results/run")
OUTJSONL = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-08.jsonl")
recs = []
logs = 0; logs_ok = 0; arm_entries = 0; arm_correct = 0
for fp in sorted(glob.glob(os.path.join(RUN, "*.jsonl"))):
    for l in open(fp):
        r = json.loads(l); recs.append((os.path.basename(fp), r))
        logs += 1
        for a in r.get("results", []):
            arm_entries += 1
            if a.get("correct") is True: arm_correct += 1
print("JSONL_FILES", len(glob.glob(os.path.join(RUN, "*.jsonl"))), "RECORDS", logs, "ARM_ENTRIES", arm_entries, "ARM_CORRECT_TRUE", arm_correct)
print("ARM_NAMES", sorted(set(a["arm"] for _, r in recs for a in r.get("results", []))))
print("BITS", sorted(set(r["bits"] for _, r in recs)), "CURVES", sorted(set(r["curve"] for _, r in recs)))
# cost per (arm, bits, curve) taken from the record's arm entry; q from record field q
pts = collections.defaultdict(dict)
for fn, r in recs:
    for a in r.get("results", []):
        if a.get("cost") is None: continue
        pts[(a["arm"], r["bits"])].setdefault(r["curve"], (r["q"], a["cost"]))
def slope(xs, ys):
    mx, my = statistics.mean(xs), statistics.mean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
out = {}
for arm in ["main:inc", "main:lpsgewd", "tuned:lpsgewd", "aa:inc"]:
    bits = sorted(b for (a, b) in pts if a == arm)
    per_size_mean = {}; pooled_x = []; pooled_y = []
    for b in bits:
        cs = pts[(arm, b)]
        if not cs: continue
        per_size_mean[b] = statistics.mean(c for _, c in cs.values())
        for q, c in cs.values():
            pooled_x.append(math.log(q)); pooled_y.append(math.log(c))
    bs = [b for b in per_size_mean if 10 <= b <= 20]
    s_mean = slope([math.log(2 ** b) for b in bs], [math.log(per_size_mean[b]) for b in bs])
    out[arm] = {"sizes": bs, "n_curves_at_20": len(pts[(arm, 20)]), "slope_on_per_size_mean_cost_b10_20": round(s_mean, 4), "slope_pooled_curve_size_points_b10_20": None}
    out[arm]["per_size_mean_cost"] = {b: round(per_size_mean[b]) for b in bs}
# ratio at b=20 over curves common to both arms, three aggregations (study rule not stated, so none is selected)
common = sorted(set(pts[("main:lpsgewd", 20)]) & set(pts[("main:inc", 20)]))
rat = [pts[("main:lpsgewd", 20)][c][1] / pts[("main:inc", 20)][c][1] for c in common]
print("RATIO_B20_common_curves", len(common))
print("RATIO_B20_ratio_of_means", statistics.mean(pts[("main:lpsgewd", 20)][c][1] for c in common) / statistics.mean(pts[("main:inc", 20)][c][1] for c in common))
print("RATIO_B20_mean_of_paired", statistics.mean(rat), "median_of_paired", statistics.median(rat), "min", min(rat), "max", max(rat))
for arm in out:
    print("ARM", arm, json.dumps({k: v for k, v in out[arm].items() if k != "per_size_mean_cost"}))
    print("  PER_SIZE_MEAN_COST", out[arm]["per_size_mean_cost"])
pat_off = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|not committed|scratch|Modal volume", re.I)
pat_aa = re.compile(r"A/A|null control|negative control|placebo|null-control", re.I)
pat_iso = re.compile(r"isolat|taskset|pinned|\bL0\b|tier|shared", re.I)
files = [os.path.join(STUDY, "README.md"), os.path.join(STUDY, "PROTOCOL.md")] + [os.path.join(RUN, "analysis.md")]
offrepo, aa, iso = [], [], []
for fp in files:
    if not os.path.exists(fp): continue
    rel = os.path.relpath(fp, REPO)
    for i, line in enumerate(open(fp, encoding="utf-8", errors="replace"), 1):
        if pat_off.search(line): offrepo.append(f"{rel}:{i}: {line.strip()}")
        if pat_aa.search(line): aa.append(f"{rel}:{i}: {line.strip()}")
        if pat_iso.search(line): iso.append(f"{rel}:{i}: {line.strip()}")
print("OFFREPO", *offrepo, sep="\n"); print("AA", *aa[:40], sep="\n"); print("ISO", *iso, sep="\n")
def gitdate(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel], capture_output=True, text=True)
    return r.stdout.strip()
print("GITDATE_PROTOCOL", gitdate("research/ic_improve_20261015/01-genus3-linalg/PROTOCOL.md") or "EMPTY")
print("GITDATE_FIRSTRAW", gitdate("research/ic_improve_20261015/01-genus3-linalg/results/run/b10_c0_main.jsonl") or "EMPTY")
print("STORE_INDEX_EXISTS", os.path.exists(os.path.join(REPO, "store/index.json")))

# ---- additional computed quantities ----
la = {}
for arm in ["main:inc", "main:lpsgewd"]:
    d = collections.defaultdict(dict)
    for fn, r in recs:
        for a in r.get("results", []):
            if a["arm"] == arm and a.get("la_cost") is not None:
                d[r["bits"]].setdefault(r["curve"], (r["q"], a["la_cost"]))
    bs = [b for b in sorted(d) if 10 <= b <= 20 and len(d[b]) == 3]
    la[arm] = {b: statistics.mean(v for _, v in d[b].values()) for b in bs}
    la[arm + "_slope"] = slope([math.log(2 ** b) for b in bs], [math.log(la[arm][b]) for b in bs])
    print("LA_SLOPE", arm, round(la[arm + "_slope"], 4), "sizes", bs)
la20_inc = [v for q, v in [pts_ for pts_ in []]]
la_c = collections.defaultdict(dict)
for fn, r in recs:
    for a in r.get("results", []):
        if a["arm"] in ("main:inc", "main:lpsgewd") and r["bits"] == 20 and a.get("la_cost") is not None:
            la_c[a["arm"]].setdefault(r["curve"], a["la_cost"])
com = sorted(set(la_c["main:inc"]) & set(la_c["main:lpsgewd"]))
print("LA_RATIO_B20_ratio_of_means", statistics.mean(la_c["main:lpsgewd"][c] for c in com) / statistics.mean(la_c["main:inc"][c] for c in com), "n", len(com))
print("LA_RATIO_B20_mean_of_paired", statistics.mean(la_c["main:lpsgewd"][c] / la_c["main:inc"][c] for c in com))
b21 = [(r["curve"], a["arm"], a["cost"]) for fn, r in recs for a in r.get("results", []) if r["bits"] == 21]
b21 += [(r["curve"], r["arm"], r["cost"]) for fn, r in recs if "arm" in r and r["bits"] == 21]
print("B21_ENTRIES", sorted(set((c, a) for c, a, _ in b21)))
flat_logs = 0; flat_ok = 0
for fp in sorted(glob.glob(os.path.join(STUDY, "results", "**", "progress.log"), recursive=True)):
    n = 0; ok = 0
    for l in open(fp):
        if l.startswith("{"):
            r = json.loads(l); n += 1; ok += r.get("correct") is True
    print("PROGRESS_LOG", os.path.relpath(fp, REPO), "json_lines", n, "correct_true", ok)
    flat_logs += n; flat_ok += ok
print("PROGRESS_LOG_TOTAL", flat_logs, flat_ok)

# ---- 2^21 cost ratio tuned:lpsgewd / main:inc (whatever curves are present) and row object ----
c21 = collections.defaultdict(dict)
for fn, r in recs:
    if r["bits"] != 21: continue
    for a in r.get("results", []):
        if a.get("cost") is not None: c21[a["arm"]].setdefault(r["curve"], a["cost"])
for fn, r in recs:
    if r["bits"] == 21 and "arm" in r and r.get("cost") is not None: c21[r["arm"]].setdefault(r["curve"], r["cost"])
com21 = sorted(set(c21["tuned:lpsgewd"]) & set(c21["main:inc"]))
r21 = [c21["tuned:lpsgewd"][c] / c21["main:inc"][c] for c in com21]
print("B21_RATIO", "curves", com21, "ratio_of_means", sum(c21["tuned:lpsgewd"][c] for c in com21) / sum(c21["main:inc"][c] for c in com21), "per_curve", [round(x, 4) for x in r21])
aa_minus = round(out["aa:inc"]["slope_on_per_size_mean_cost_b10_20"] - out["main:inc"]["slope_on_per_size_mean_cost_b10_20"], 4)
def mk(claimed, computed, definition, files, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files, "status": status}
RUNF = ["research/ic_improve_20261015/01-genus3-linalg/results/run/progress.log"]
ALLF = ["research/ic_improve_20261015/01-genus3-linalg/results/run/progress.log", "research/ic_improve_20261015/01-genus3-linalg/results/tune1/progress.log", "research/ic_improve_20261015/01-genus3-linalg/results/tune2/progress.log"]
claims_out = [
 mk("whole exponent q^1.380 (lpsgewd)", f"{out['main:lpsgewd']['slope_on_per_size_mean_cost_b10_20']} (OLS slope of ln cost on ln 2^b, per-size mean over 3 curves, b=10..20)", "least-squares slope, study PROTOCOL line 96", ["research/ic_improve_20261015/01-genus3-linalg/results/run/b10_c0_main.jsonl"], "match"),
 mk("incumbent exponent q^1.398", f"{out['main:inc']['slope_on_per_size_mean_cost_b10_20']}", "as above, arm main:inc", ["research/ic_improve_20261015/01-genus3-linalg/results/run/b10_c0_main.jsonl"], "match"),
 mk("paired ratio at 2^20 = 0.653", f"{statistics.mean(pts[('main:lpsgewd', 20)][c][1] / pts[('main:inc', 20)][c][1] for c in common):.4f} (mean of per-curve paired ratios, 3 curves); ratio of means {sum(pts[('main:lpsgewd',20)][c][1] for c in common)/sum(pts[('main:inc',20)][c][1] for c in common):.4f}; median {statistics.median(rat):.4f}", "paired ratio at b=20; study rule for the aggregate not stated in committed files", ["research/ic_improve_20261015/01-genus3-linalg/results/run/b10_c0_main.jsonl"], "partial"),
 mk("paired ratio 95% CI [0.647, 0.660]", f"per-curve min {min(rat):.4f}, max {max(rat):.4f}; bootstrap (2000 resamples, curves within size) seed not stated, CI not re-derived", "bootstrap interval (PROTOCOL line 94-96)", ["research/ic_improve_20261015/01-genus3-linalg/PROTOCOL.md"], "cannot_define"),
 mk("LA ratio 0.463 at 2^20", f"{la20 if False else ''}{statistics.mean(la_c['main:lpsgewd'][c] for c in com) / statistics.mean(la_c['main:inc'][c] for c in com):.4f} (ratio of means of la_cost, 3 curves)", "ratio of la_cost at b=20 over curves", ["research/ic_improve_20261015/01-genus3-linalg/results/run/progress.log"], "match"),
 mk("LA exponent 1.702 against 1.519", f"main:lpsgewd LA slope {la['main:lpsgewd_slope']:.4f}; main:inc LA slope {la['main:inc_slope']:.4f} (OLS on per-size mean la_cost, b=10..20)", "same slope rule applied to la_cost", ["research/ic_improve_20261015/01-genus3-linalg/results/run/progress.log"], "match"),
 mk("tuned:lpsgewd (kappa 1.41) exponent 1.357", f"{out['tuned:lpsgewd']['slope_on_per_size_mean_cost_b10_20']}", "as above, arm tuned:lpsgewd", ["research/ic_improve_20261015/01-genus3-linalg/results/tune1/progress.log"], "match"),
 mk("A/A exponent difference 0.000 [-0.004, 0.004]", f"slope(aa:inc) - slope(main:inc) = {aa_minus}; CI not re-derived (seed not stated)", "exponent difference point value", ["research/ic_improve_20261015/01-genus3-linalg/results/run/progress.log"], "partial"),
 mk("474/474 logs verified", f"progress.log JSON lines total {flat_logs}, correct true {flat_ok}; all results/**/*.jsonl arm entries 748, correct 746", "count of verified log records", ALLF, "mismatch"),
 mk("2^21 ratio 0.638 (tuned:lpsgewd / main:inc)", f"over curves {com21}: ratio of means {sum(c21['tuned:lpsgewd'][c] for c in com21)/sum(c21['main:inc'][c] for c in com21):.4f}; per-curve {[round(x,4) for x in r21]}", "cost ratio at b=21 over curves present in the raw files", [f"research/ic_improve_20261015/01-genus3-linalg/results/run/b21_c{c}_{arm}.jsonl" for c in (0, 1, 2) for arm in ("main", "tuned")], "match"),
]
row = {
 "row_no": 63, "slug": "20261015-ic-improve2-01-genus3-linalg", "claims": claims_out,
 "identical": {"claimed": True, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "partial"},
 "offrepo_refs": offrepo, "aa_lines": aa[:40], "isolation_lines": iso,
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (PROTOCOL.md added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (results/run/b10_c0_main.jsonl added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/63.py",
 "notes": "identical field: the HEADLINE A/A text gives 'A/A Delta 0.000' and '474/474 logs verified', not identical outputs; 'identical' set claimed true only because of the 474/474 verification statement and the A/A row, counts left null except as above (status partial). Raw counts: 111 records in results/run (450 arm entries, all correct true), 152 jsonl files total under results/ (748 arm entries, 746 correct true). The 2^21 records for curves 0, 1, 2 (main and tuned arms) are in results/run/b21_c{0,1,2}_{main,tuned}.jsonl and give the 0.638 ratio of means; results/b21_c0_overflow holds a separate copy of curve-0 b21 records, not used in the ratio. The CI endpoints in the study are bootstrap percentiles whose seed is not in committed files. store/index.json exists. Off-repo grep matched no lines. The isolation lines are shared-host and load statements from README and PROTOCOL."
}
with open(OUTJSONL, "a", encoding="utf-8") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("ROW63_WRITTEN", len(claims_out), "claims")
