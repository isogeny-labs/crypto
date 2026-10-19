# SWB-RECOMPUTE-08 row 58: 20261015-binary-mitm-oracle-l8. Reads committed files only.
import json, statistics, subprocess, re, os, glob
REPO = "/home/user/crypto"
STUDY = os.path.join(REPO, "research/binary_mitm_oracle_l8_20261015")
OUTJSONL = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-08.jsonl")
R = json.load(open(os.path.join(STUDY, "results.json")))
pt = R["decomposition"]["per_target"]
med_mitm = statistics.median([t["mitm_ms"] for t in pt])
med_pairs = statistics.median([t["pairs_ms"] for t in pt])
print("n_targets", len(pt))
print("median_mitm_ms(recomputed)", med_mitm, "stored", R["decomposition"]["median_ms_per_target"]["mitm"])
print("median_pairs_ms(recomputed)", med_pairs, "stored", R["decomposition"]["median_ms_per_target"]["pairs_and_solve"])
print("ratio_pairs_over_mitm", med_pairs / med_mitm)
print("max_mitm_ops", max(t["mitm_ops"] for t in pt), "min_mitm_ops", min(t["mitm_ops"] for t in pt))
print("pairs_pairs_values", sorted(set(t["pairs_pairs"] for t in pt)))
print("found", sum(t["mitm_found"] for t in pt), "refuted", sum(not t["mitm_found"] for t in pt))
print("pairs_found", sum(t["pairs_found"] for t in pt))
n_agree = sum(t["agree"] for t in pt); print("agree_true", n_agree, "agree_false", len(pt) - n_agree)
print("mitm_eq_pairs_decision", sum(t["mitm_found"] == t["pairs_found"] for t in pt))
print("witness_verified_among_found", sum(t["witness_group_verified"] for t in pt if t["mitm_found"]), "of", sum(t["mitm_found"] for t in pt))
print("kinds", {k: sum(1 for t in pt if t["kind"] == k) for k in set(t["kind"] for t in pt)})
print("gate_flag", R["decomposition"]["gate_median_le_half_pairs"], "disagreements_field", R["decomposition"]["disagreements"], "all_verified_field", R["decomposition"]["all_witnesses_group_verified"])
print("ratio_pairs_over_mitm_min_max_over_targets", min(t["pairs_ms"]/t["mitm_ms"] for t in pt), max(t["pairs_ms"]/t["mitm_ms"] for t in pt))
# reports directory listing (raw files present for runs 1 and 2?)
print("REPORTS_FILES", sorted(os.path.relpath(p, REPO) for p in glob.glob(os.path.join(STUDY, "reports", "**", "*"), recursive=True) if os.path.isfile(p)))
print("PROVENANCE_RUNS", json.load(open(os.path.join(STUDY, "provenance.json")))["runs"])
pat_off = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|not committed|scratch|Modal volume", re.I)
pat_aa = re.compile(r"A/A|null control|negative control|placebo|null-control", re.I)
pat_iso = re.compile(r"isolat|taskset|pinned|\bL0\b|tier|shared", re.I)
files = [os.path.join(STUDY, f) for f in ("REPORT.typ", "provenance.json")] + [p for p in glob.glob(os.path.join(STUDY, "reports", "**", "*.md"), recursive=True)]
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
print("GITDATE_FIRSTRAW", gitdate("research/binary_mitm_oracle_l8_20261015/provenance.json"))
print("GITDATE_RESULTS", gitdate("research/binary_mitm_oracle_l8_20261015/results.json"))
print("STORE_INDEX_EXISTS", os.path.exists(os.path.join(REPO, "store/index.json")))

# ---- row object ----
def fmt(x, nd): return f"{x:.{nd}f}"
RES = ["research/binary_mitm_oracle_l8_20261015/results.json"]
claims = [
 {"claimed": "median per-target MITM 0.120 ms (run 3)", "computed": fmt(med_mitm, 4) + " ms (median of per_target mitm_ms); stored median_ms_per_target.mitm = " + str(R["decomposition"]["median_ms_per_target"]["mitm"]),
  "definition": "statistics.median over the 64 per_target mitm_ms values in results.json", "files": RES, "status": "mismatch"},
 {"claimed": "median per-target pairs-and-solve 12.117 ms (run 3)", "computed": fmt(med_pairs, 3) + " ms (median of per_target pairs_ms); stored = " + str(R["decomposition"]["median_ms_per_target"]["pairs_and_solve"]),
  "definition": "statistics.median over the 64 per_target pairs_ms values in results.json", "files": RES, "status": "mismatch"},
 {"claimed": "ratio 100.7x (run 3)", "computed": fmt(med_pairs / med_mitm, 1) + "x (median pairs_ms / median mitm_ms); stored-median ratio = " + fmt(R["decomposition"]["median_ms_per_target"]["pairs_and_solve"] / R["decomposition"]["median_ms_per_target"]["mitm"], 1) + "x",
  "definition": "median pairs_ms / median mitm_ms over per_target", "files": RES, "status": "mismatch"},
 {"claimed": "median 0.120-0.155 ms vs 11.7-42.8 ms, 94x-277x across three runs", "computed": "only run 3 in repo (results.json); runs 1-2 absent (provenance.json note)",
  "definition": "three-run medians", "files": ["research/binary_mitm_oracle_l8_20261015/provenance.json", "research/binary_mitm_oracle_l8_20261015/REPORT.typ"], "status": "no_raw_in_repo"},
 {"claimed": "max 1,068 counted point-adds per target", "computed": str(max(t["mitm_ops"] for t in pt)),
  "definition": "max of per_target mitm_ops", "files": RES, "status": "match"},
 {"claimed": "32,896 quartic solves (pairs-and-solve)", "computed": str(max(t["pairs_pairs"] for t in pt)),
  "definition": "max of per_target pairs_pairs", "files": RES, "status": "match"},
 {"claimed": "20 found / 44 refuted of 64", "computed": f"{sum(t['mitm_found'] for t in pt)} found / {sum(not t['mitm_found'] for t in pt)} refuted of {len(pt)}",
  "definition": "count of per_target mitm_found true / false", "files": RES, "status": "match"},
 {"claimed": "0 disagreements vs pairs-and-solve", "computed": f"{sum(not t['agree'] for t in pt)} per_target agree=false; mitm_found==pairs_found in {sum(t['mitm_found']==t['pairs_found'] for t in pt)} of {len(pt)}",
  "definition": "count of per_target agree false", "files": RES, "status": "match"},
 {"claimed": "all witnesses group-verified (20/20)", "computed": f"{sum(t['witness_group_verified'] for t in pt if t['mitm_found'])} of {sum(t['mitm_found'] for t in pt)} found targets have witness_group_verified true",
  "definition": "count of witness_group_verified true among mitm_found true", "files": RES, "status": "match"},
 {"claimed": "table ~21 ms build, ~1 MiB", "computed": "no build-time or memory field in results.json or provenance.json", "definition": "not a field in raw records", "files": RES, "status": "cannot_define"},
 {"claimed": "gate median <= half pairs-and-solve, PASS", "computed": f"gate_median_le_half_pairs field = {R['decomposition']['gate_median_le_half_pairs']}; recomputed ratio {fmt(med_pairs/med_mitm,1)}x",
  "definition": "stored gate flag and recomputed median ratio", "files": RES, "status": "partial"},
]
row = {
 "row_no": 58, "slug": "20261015-binary-mitm-oracle-l8", "claims": claims,
 "identical": {"claimed": True, "n_compared": len(pt), "n_equal": sum(t["agree"] for t in pt), "n_differ": sum(not t["agree"] for t in pt),
               "files": RES, "status": "match"},
 "offrepo_refs": offrepo, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": "",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (provenance.json added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/58.py",
 "notes": "No PROTOCOL file exists in the study directory (files: REPORT.typ, provenance.json, results.json, reports/). Stored median_ms_per_target in results.json does not equal the median of its own per_target values; both are recorded above. results.json is the third run only (provenance.json runs=3, note says results.json is the frozen output of the third run). The 'identical' claim is the 0 disagreements count and n_compared is the 64 per_target agree flags. No A/A or null-control line matched the pattern. No off-repo reference was found in REPORT.typ, provenance.json or reports/*.md. store:<name> not present in the study files; store/index.json exists in the repo."
}
with open(OUTJSONL, "a", encoding="utf-8") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("ROW58_WRITTEN", len(claims), "claims")
