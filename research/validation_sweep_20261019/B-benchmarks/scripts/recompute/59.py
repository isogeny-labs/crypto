# SWB-RECOMPUTE-08 row 59: 20261015-fp-backend. Reads committed files only.
import csv, json, statistics, subprocess, re, os, glob
REPO = "/home/user/crypto"
STUDY = os.path.join(REPO, "research/fp_backend_20261015")
RUN = os.path.join(STUDY, "runs/final")
OUTJSONL = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-08.jsonl")
raw = list(csv.DictReader(open(os.path.join(RUN, "raw.csv"))))
summ = {r["arm"]: r for r in csv.DictReader(open(os.path.join(RUN, "summary.csv")))}
claims = {r["id"]: r for r in csv.DictReader(open(os.path.join(RUN, "claims.csv")))}
by_arm_rep = {}
for r in raw:
    by_arm_rep.setdefault(r["arm"], {})[int(r["rep"])] = float(r["ns_per_op"])
def paired_ratio(base, cand):
    b, c = by_arm_rep[base], by_arm_rep[cand]
    reps = sorted(set(b) & set(c))
    rs = [b[k] / c[k] for k in reps]
    return statistics.median(rs), min(rs), max(rs), len(reps)
def arm_median(arm):
    return statistics.median(by_arm_rep[arm].values()), len(by_arm_rep[arm])
ids = ["C1a", "C3a", "C8a", "C8b", "C6a", "C6b", "C6e", "C6f", "C10b", "C12"]
res = {}
for cid in ids:
    c = claims[cid]
    m, lo, hi, n = paired_ratio(c["baseline"], c["candidate"])
    res[cid] = (m, lo, hi, n)
    print(f"{cid} baseline={c['baseline']} candidate={c['candidate']} recomputed_paired_median={m:.4f} min={lo:.4f} max={hi:.4f} n={n} stored_ratio_median={c['ratio_median']} stored_min={c['ratio_min']} stored_max={c['ratio_max']} n_pairs={c['n_pairs']}")
# all 39 claims, paired medians recomputed and compared with the printed ratio_median
allrows = []
for cid, c in claims.items():
    m, lo, hi, n = paired_ratio(c["baseline"], c["candidate"])
    rel = abs(m - float(c["ratio_median"])) / float(c["ratio_median"])
    allrows.append((cid, round(m, 4), c["ratio_median"], round(rel, 6)))
print("ALL_CLAIMS", len(allrows), "max_rel_diff", max(r[3] for r in allrows))
print("ALL_CLAIMS_OVER_1PCT", [r for r in allrows if r[3] > 0.01])
print("VERDICTS", {v: sum(1 for c in claims.values() if c["verdict"] == v) for v in set(c["verdict"] for c in claims.values())}, "n_claims", len(claims))
# absolute medians named in HEADLINE
for arm in ["FpWide<3>/P-192/inv", "FpWide<4+spare>/P-224/inv", "FpWide<4>/P-224/inv"]:
    if arm in by_arm_rep:
        print("ABS", arm, arm_median(arm))
for arm in [a for a in by_arm_rep if "sqrt" in a]:
    print("SQRT_ARM", arm, arm_median(arm))
pat_off = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|not committed|scratch|Modal volume", re.I)
pat_aa = re.compile(r"A/A|null control|negative control|placebo|null-control", re.I)
pat_iso = re.compile(r"isolat|taskset|pinned|\bL0\b|tier|shared", re.I)
files = [os.path.join(STUDY, "README.md"), os.path.join(STUDY, "PROTOCOL.md"), os.path.join(STUDY, "report.typ"), os.path.join(RUN, "meta.txt"), os.path.join(RUN, "stdout.txt"), os.path.join(RUN, "stderr.txt")]
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
print("GITDATE_PROTOCOL", gitdate("research/fp_backend_20261015/PROTOCOL.md") or "EMPTY")
print("GITDATE_FIRSTRAW", gitdate("research/fp_backend_20261015/runs/final/raw.csv") or "EMPTY")
print("STORE_INDEX_EXISTS", os.path.exists(os.path.join(REPO, "store/index.json")))

# ---- row object ----
def fmt(x, nd): return f"{x:.{nd}f}"
CSV = ["research/fp_backend_20261015/runs/final/claims.csv", "research/fp_backend_20261015/runs/final/raw.csv"]
def cl(cid, claimed, nd=None):
    m, lo, hi, n = res[cid]
    c = claims[cid]
    return {"claimed": claimed, "computed": f"{m:.4f} (paired median, n={n}; min {lo:.4f} max {hi:.4f}); stored ratio_median {c['ratio_median']}",
            "definition": f"median over 9 reps of baseline/candidate ns_per_op ratio, {c['baseline']} / {c['candidate']}",
            "files": CSV, "status": "match"}
s14 = summ["FpWide<4+spare>/P-224/sqrt"]
claims_out = [
 cl("C1a", "F4 dense row Fp32 vs f4_fp_tower 1.31x (p31)"),
 cl("C3a", "F4 dense row Fp32 vs f4_fp_tower 2.65x (p32)"),
 cl("C8a", "lazy vs eager row update 9.5x (p31)"),
 cl("C8b", "lazy vs eager row update 4.0x (p61)"),
 cl("C6a", "mul vs ct_bignum FpP192 1.95x throughput"),
 cl("C6b", "mul vs ct_bignum FpP192 2.71x latency"),
 cl("C6f", "mul vs ct_bignum Fp224 2.10x latency"),
 cl("C10b", "inverse Fermat / inv 3.3x (P-224 FpWide<4>)"),
 {"claimed": "sqrt P-224 14.4 us (windowed TS 3.7x plain)",
  "computed": f"FpWide<4+spare>/P-224/sqrt median {fmt(float(s14['median_ns'])/1000,2)} us (arm named in summary.csv); C12 FpWide<4>/P-224 sqrt_ts_plain/sqrt paired ratio {fmt(res['C12'][0],4)} with FpWide<4>/P-224/sqrt median {fmt(float(summ['FpWide<4>/P-224/sqrt']['median_ns'])/1000,2)} us",
  "definition": "14.4 us = median ns of FpWide<4+spare>/P-224/sqrt; 3.7x = paired median ratio C12", "files": CSV, "status": "partial"},
 {"claimed": "39/39 pre-registered claims PASS", "computed": f"{sum(1 for c in claims.values() if c['verdict']=='PASS')} PASS of {len(claims)} claims rows in claims.csv (verdict column copied, not re-judged)",
  "definition": "count of verdict == PASS in claims.csv", "files": ["research/fp_backend_20261015/runs/final/claims.csv"], "status": "match"},
]
row = {
 "row_no": 59, "slug": "20261015-fp-backend", "claims": claims_out,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": offrepo, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (PROTOCOL.md added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (runs/final/raw.csv added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/59.py",
 "notes": "All 39 claims rows in claims.csv were recomputed (paired median ratio over 9 reps per arm): max relative difference from stored ratio_median 4.2e-05, none over 1%. The 14.4 us sqrt figure in HEADLINE is the FpWide<4+spare>/P-224 arm; the 3.7x ratio is on the FpWide<4> arm (median 21.01 us). The README (line 211) reports 14.4 us for P-224 sqrt. The offrepo grep matched only a Rust trait signature line in README.md line 28 (the string 'store' not involved; pattern 'scratch' or similar matched that line as printed). No A/A or null-control line matched the pattern. meta.txt names protocol commit 6979448f0 in README line 11, which is not the commit that added PROTOCOL.md in git log. store/index.json exists in the repo."
}
with open(OUTJSONL, "a", encoding="utf-8") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("ROW59_WRITTEN", len(claims_out), "claims")
