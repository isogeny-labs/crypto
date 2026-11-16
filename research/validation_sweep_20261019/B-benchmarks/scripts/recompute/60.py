# SWB-RECOMPUTE-08 row 60: 20261015-d4-weighted-s5. Reads committed files only.
import json, statistics, subprocess, re, os, glob, math, collections
REPO = "/home/user/crypto"
STUDY = os.path.join(REPO, "research/d4_symmetrised_s5_20261015")
RAW = os.path.join(STUDY, "raw")
OUTJSONL = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-08.jsonl")
def recs(name):
    out = []
    for l in open(os.path.join(RAW, name)):
        r = json.loads(l)
        if "target" in r and "w_solve" in r: out.append(r)
    return out
FILES = ["measured.jsonl", "v2_measured.jsonl", "tiny.jsonl", "v2_tiny.jsonl", "w_only.jsonl", "v2_w_only.jsonl"]
allkeys = set(); per_file = {}
for fn in FILES:
    rs = recs(fn)
    keys = [(r["p"], r["seed"], r["target"]) for r in rs]
    allkeys |= set(keys)
    per_file[fn] = rs
    print("FILE", fn, "records", len(rs), "distinct_targets", len(set(keys)))
print("UNION_DISTINCT_TARGETS (p,seed,target) across the 6 per-target files", len(allkeys))
def summarize(fn):
    rs = per_file[fn]
    groups = collections.OrderedDict()
    for r in rs: groups.setdefault((r["p"], r["seed"]), []).append(r)
    for (p, s), g in sorted(groups.items()):
        cw = [r["w_solve"]["total_muls"] for r in g if r.get("w_solved")]
        c8 = [r["c8_solve"]["total_muls"] for r in g if r.get("c8_solved") and r.get("c8_solve")]
        ratio = [r["c8_solve"]["total_muls"] / r["w_solve"]["total_muls"] for r in g if r.get("c8_solved") and r.get("c8_solve") and r.get("w_solved")]
        lift = [r["w_lift"]["muls"] / r["w_solve"]["total_muls"] for r in g if r.get("w_solved") and r.get("w_lift")]
        eq_b = collections.Counter(str(r.get("w_equals_brute")) for r in g)
        eq_m = collections.Counter(str(r.get("w_equals_mitm")) for r in g)
        eq_c = collections.Counter(str(r.get("c8_equals_w")) for r in g)
        print(f"  {fn} p={p} seed={s} n={len(g)} w_solved={len(cw)} c8_solved={len(c8)} "
              f"C_w_median={statistics.median(cw) if cw else None} C_w_min_max={(min(cw), max(cw)) if cw else None} "
              f"C_c8_median={statistics.median(c8) if c8 else None} paired_c8_over_w_median={statistics.median(ratio) if ratio else None} paired_min={min(ratio) if ratio else None} "
              f"lift_over_w_max={max(lift) if lift else None} equals_brute={dict(eq_b)} equals_mitm={dict(eq_m)} c8_equals_w={dict(eq_c)}")
for fn in ["measured.jsonl", "v2_measured.jsonl"]:
    print("SUMMARY", fn); summarize(fn)
# claims on the measured file (p = 269, 1033, 4093; seed 1; 24 targets)
m = per_file["measured.jsonl"]
cw_all = [r["w_solve"]["total_muls"] for r in m if r.get("w_solved")]
c8_all = [r["c8_solve"]["total_muls"] for r in m if r.get("c8_solved") and r.get("c8_solve")]
ratio_all = [r["c8_solve"]["total_muls"] / r["w_solve"]["total_muls"] for r in m if r.get("c8_solved") and r.get("c8_solve") and r.get("w_solved")]
plain = 1.213e12
print("MEASURED_C_w_median_all", statistics.median(cw_all), "range", min(cw_all), max(cw_all))
print("MEASURED_C_c8_median_all", statistics.median(c8_all), "range", min(c8_all), max(c8_all))
print("MEASURED_paired_c8_over_w_median", statistics.median(ratio_all), "min", min(ratio_all), "max", max(ratio_all))
print("MEASURED_plain_over_Cw_median", plain / statistics.median(cw_all))
lift_all = [r["w_lift"]["muls"] / r["w_solve"]["total_muls"] for r in m if r.get("w_solved") and r.get("w_lift")]
print("MEASURED_lift_over_w_max", max(lift_all))
n_star = 157.9 - 4 * math.log2(plain / statistics.median(cw_all))
print("HANDOVER_n_star_bits_from_median_Cw_measured", round(n_star, 3))
print("HANDOVER_n_star_bits_from_1.420e9", round(157.9 - 4 * math.log2(plain / 1.420e9), 3))
print("HANDOVER_n_star_bits_from_1.213e12_over_C8_25.55e9", round(157.9 - 4 * math.log2(plain / 2.555e10), 3))
pat_off = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|not committed|scratch|Modal volume", re.I)
pat_aa = re.compile(r"A/A|null control|negative control|placebo|null-control", re.I)
pat_iso = re.compile(r"isolat|taskset|pinned|\bL0\b|tier|shared", re.I)
files = [os.path.join(STUDY, f) for f in ("README.md", "PROTOCOL.md", "REPORT.typ")] + [os.path.join(RAW, f) for f in ("binary_commit.txt", "started_utc.txt", "finished_utc.txt", "run_all.sh", "run_v2.sh")]
offrepo, aa, iso = [], [], []
for fp in files:
    if not os.path.exists(fp): continue
    rel = os.path.relpath(fp, REPO)
    for i, line in enumerate(open(fp, encoding="utf-8", errors="replace"), 1):
        if pat_off.search(line): offrepo.append(f"{rel}:{i}: {line.strip()}")
        if pat_aa.search(line): aa.append(f"{rel}:{i}: {line.strip()}")
        if pat_iso.search(line): iso.append(f"{rel}:{i}: {line.strip()}")
print("OFFREPO", *offrepo, sep="\n"); print("AA", *aa, sep="\n"); print("ISO", *iso, sep="\n")
def gitdate(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel], capture_output=True, text=True)
    return r.stdout.strip()
print("GITDATE_PROTOCOL", gitdate("research/d4_symmetrised_s5_20261015/PROTOCOL.md") or "EMPTY")
print("GITDATE_FIRSTRAW", gitdate("research/d4_symmetrised_s5_20261015/raw/measured.jsonl") or "EMPTY")
print("STORE_INDEX_EXISTS", os.path.exists(os.path.join(REPO, "store/index.json")))

# ---- union over all six per-target files, dedup by (p, seed, target); first file wins ----
U = {}
for fn in FILES:
    for r in per_file[fn]:
        U.setdefault((r["p"], r["seed"], r["target"]), (fn, r))
print("UNION_N", len(U), "w_unsolved", sum(1 for k, (fn, r) in U.items() if not r.get("w_solved")),
      "c8_unsolved_among_c8_run", sum(1 for k, (fn, r) in U.items() if r.get("c8_run") and not r.get("c8_solved")),
      "c8_run", sum(1 for k, (fn, r) in U.items() if r.get("c8_run")))
byp = collections.defaultdict(list)
for k, (fn, r) in U.items(): byp[k[0]].append(r)
for p in sorted(byp):
    g = byp[p]
    cw = [r["w_solve"]["total_muls"] for r in g if r.get("w_solved")]
    c8 = [r["c8_solve"]["total_muls"] for r in g if r.get("c8_solved") and r.get("c8_solve")]
    rat = [r["c8_solve"]["total_muls"] / r["w_solve"]["total_muls"] for r in g if r.get("c8_solved") and r.get("c8_solve") and r.get("w_solved")]
    print("UNION_P", p, "n", len(g), "Cw_median", statistics.median(cw) if cw else None, "c8_median", statistics.median(c8) if c8 else None,
          "paired_median", round(statistics.median(rat), 3) if rat else None, "paired_min", round(min(rat), 3) if rat else None,
          "plain_over_Cw_median", round(plain / statistics.median(cw), 1) if cw else None)
allcw = [r["w_solve"]["total_muls"] for k, (fn, r) in U.items() if r.get("w_solved")]
allc8 = [r["c8_solve"]["total_muls"] for k, (fn, r) in U.items() if r.get("c8_solved") and r.get("c8_solve")]
allrat = [r["c8_solve"]["total_muls"] / r["w_solve"]["total_muls"] for k, (fn, r) in U.items() if r.get("c8_solved") and r.get("c8_solve") and r.get("w_solved")]
print("UNION_CW_RANGE_MEDIAN", min(allcw), max(allcw), statistics.median(allcw))
print("UNION_CW_p_MEDIANS_RANGE", [round(statistics.median([r["w_solve"]["total_muls"] for r in byp[p] if r.get("w_solved")])/1e9, 4) for p in sorted(byp)])
print("UNION_C8_MEDIAN_RANGE", [round(statistics.median([r["c8_solve"]["total_muls"] for r in byp[p] if r.get("c8_solved") and r.get("c8_solve")])/1e10, 4) for p in sorted(byp) if any(r.get("c8_solved") for r in byp[p])])
print("UNION_PAIRED_MIN_MAX", round(min(allrat), 3), round(max(allrat), 3))
print("UNION_LIFT_MAX", max(r["w_lift"]["muls"] / r["w_solve"]["total_muls"] for k, (fn, r) in U.items() if r.get("w_solved") and r.get("w_lift")))
print("UNION_EQ_BRUTE", collections.Counter(str(r.get("w_equals_brute")) for k, (fn, r) in U.items()))
print("UNION_EQ_MITM", collections.Counter(str(r.get("w_equals_mitm")) for k, (fn, r) in U.items()))
print("UNION_C8_EQ_W", collections.Counter(str(r.get("c8_equals_w")) for k, (fn, r) in U.items()))
print("UNION_DEGREE27_RERUN", sum(1 for k, (fn, r) in U.items() if r.get("w_degree_used") == 27))

# ---- row object ----
def fmt(x, nd): return f"{x:.{nd}f}"
seed1 = {p: [r for r in byp[p] if r["seed"] == 1] for p in (269, 1033, 4093)}
cw_s1 = {p: statistics.median([r["w_solve"]["total_muls"] for r in g if r.get("w_solved")]) for p, g in seed1.items()}
c8_s1 = {p: statistics.median([r["c8_solve"]["total_muls"] for r in g if r.get("c8_solved") and r.get("c8_solve")]) for p, g in seed1.items()}
ratio_s1 = [r["c8_solve"]["total_muls"] / r["w_solve"]["total_muls"] for p, g in seed1.items() for r in g if r.get("c8_solved") and r.get("c8_solve") and r.get("w_solved")]
plain_s1 = {p: plain / cw_s1[p] for p in cw_s1}
unsolved_w = sum(1 for k, (fn, r) in U.items() if not r.get("w_solved"))
F = ["research/d4_symmetrised_s5_20261015/raw/measured.jsonl", "research/d4_symmetrised_s5_20261015/raw/v2_measured.jsonl",
     "research/d4_symmetrised_s5_20261015/raw/tiny.jsonl", "research/d4_symmetrised_s5_20261015/raw/v2_tiny.jsonl",
     "research/d4_symmetrised_s5_20261015/raw/w_only.jsonl", "research/d4_symmetrised_s5_20261015/raw/v2_w_only.jsonl"]
claims_out = [
 {"claimed": "C_w = 1.41-1.44e9 F_p mults", "computed": "seed-1 cells p=269,1033,4093 median C_w = " + ", ".join(f"{cw_s1[p]:.4e}" for p in cw_s1),
  "definition": "median of w_solve.total_muls over solved targets, seed 1, per p", "files": F[:2], "status": "match"},
 {"claimed": "C8 = 2.54-2.56e10", "computed": "seed-1 cells median C_c8 = " + ", ".join(f"{c8_s1[p]:.4e}" for p in c8_s1),
  "definition": "median of c8_solve.total_muls over c8-solved targets, seed 1, per p", "files": F[:2], "status": "match"},
 {"claimed": "paired x17.8-18.5", "computed": f"per-p paired median C_c8/C_w over all six files union: {min(r for r in [statistics.median([r['c8_solve']['total_muls']/r['w_solve']['total_muls'] for r in byp[p] if r.get('c8_solved') and r.get('c8_solve') and r.get('w_solved')]) for p in byp]):.2f}-{max(statistics.median([r['c8_solve']['total_muls']/r['w_solve']['total_muls'] for r in byp[p] if r.get('c8_solved') and r.get('c8_solve') and r.get('w_solved')]) for p in byp):.2f}",
  "definition": "median over paired targets of C_c8/C_w, per p, then range over p", "files": F, "status": "match"},
 {"claimed": "paired minimum 7.7", "computed": f"min paired C_c8/C_w over union = {min(allrat):.3f}", "definition": "min over all paired targets", "files": F, "status": "match"},
 {"claimed": "x844-863 below 1.213e12", "computed": "seed-1 plain/C_w per p = " + ", ".join(f"{plain_s1[p]:.1f}" for p in plain_s1) + " (1.213e12 quoted, not in raw)",
  "definition": "1.213e12 divided by median C_w, seed 1, per p", "files": F[:2], "status": "match"},
 {"claimed": "projection 630x", "computed": "no projection field in any raw file", "definition": "not a raw record field", "files": F, "status": "cannot_define"},
 {"claimed": "lift-back <= 2e-4 of a solve", "computed": f"max w_lift.muls / w_solve.total_muls over union = {max(r['w_lift']['muls']/r['w_solve']['total_muls'] for k,(fn,r) in U.items() if r.get('w_solved') and r.get('w_lift')):.4e}",
  "definition": "max of w_lift.muls / w_solve.total_muls", "files": F, "status": "match"},
 {"claimed": "1 of 102 targets degenerate (unsolved by both arms)", "computed": f"{len(U)} distinct targets in committed per-target files; w unsolved = {unsolved_w}; c8 unsolved among c8-run = {sum(1 for k,(fn,r) in U.items() if r.get('c8_run') and not r.get('c8_solved'))}",
  "definition": "count of distinct (p,seed,target) and count of w_solved false", "files": F, "status": "mismatch"},
 {"claimed": "handover n* 157.9 -> 118.9 bits", "computed": f"157.9 - 4*log2(1.213e12 / 1.420e9) = {157.9 - 4*math.log2(plain/1.420e9):.3f} (measured-file median C_w {statistics.median(cw_all):.4e} gives {157.9 - 4*math.log2(plain/statistics.median(cw_all)):.3f})",
  "definition": "n* = 157.9 - 4 log2(1.213e12 / C), C = median C_w", "files": F[:1], "status": "match"},
 {"claimed": "C_w flat in p (seed 1)", "computed": "seed-1 median C_w range " + f"{min(cw_s1.values())/1e9:.4f}-{max(cw_s1.values())/1e9:.4f} e9", "definition": "range of per-p medians, seed 1", "files": F[:1], "status": "partial"},
]
row = {
 "row_no": 60, "slug": "20261015-d4-weighted-s5", "claims": claims_out,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": offrepo, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (PROTOCOL.md added in commit ca00c151764f7c3221b22a43a138887f171e051d; README cites 87fe78b63)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (raw/measured.jsonl added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/60.py",
 "notes": "Committed per-target records: 84 distinct (p,seed,target) across measured, v2_measured, tiny, v2_tiny, w_only, v2_w_only; the study text says 102 targets (REPORT.typ line 62, README line 143). measured.jsonl and v2_measured.jsonl give identical C_w and paired values per target. Identity counts (not in HEADLINE; recorded for reference): w_equals_brute True 23, False 1, None 60; w_equals_mitm True 83, False 1; c8_equals_w True 47, None 37. README line 289 says 'Every C8 value is identical' (not a raw field). Off-repo grep matched no path or store reference. The isolation lines are README/REPORT host statements. No A/A or null-control line matched. README cites protocol commit 87fe78b63; git log reports PROTOCOL.md add commit ca00c151764f. store/index.json exists."
}
with open(OUTJSONL, "a", encoding="utf-8") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("ROW60_WRITTEN", len(claims_out), "claims")
