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
