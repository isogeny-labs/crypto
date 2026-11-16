# SWB-RECOMPUTE-08 row 62: 20261015-factor-base-evaluation. Reads committed files only.
import csv, json, statistics, subprocess, re, os, collections
REPO = "/home/user/crypto"
STUDY = os.path.join(REPO, "research/factor_base_evaluation_20261015")
RES = os.path.join(STUDY, "results")
OUTJSONL = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-08.jsonl")
cases = list(csv.DictReader(open(os.path.join(RES, "cases.csv"))))
gen = list(csv.DictReader(open(os.path.join(RES, "generic_summary.csv"))))
S = json.load(open(os.path.join(RES, "summary.json")))
agg = collections.defaultdict(lambda: [0, 0.0])
for r in cases:
    if r["bits"] == "24":
        agg[r["kind"]][0] += int(r["rank"]); agg[r["kind"]][1] += float(r["total_cpu"])
rpc = {k: v[0] / v[1] for k, v in agg.items()}
print("CASES_24_RANK_PER_CPU", {k: round(v, 3) for k, v in rpc.items()}, "rank_sums", {k: v[0] for k, v in agg.items()})
print("CASES_24_ROWS_BY_KIND", collections.Counter(r["kind"] for r in cases if r["bits"] == "24"))
print("GENERIC_SUMMARY_24", [(g["kind"], g["rank"], round(float(g["rank_per_cpu"]), 3), round(float(g["total_cpu"]), 6)) for g in gen if g["bits"] == "24"])
print("RATIO_COSET_OVER_RANDOM_CASES", rpc["coset"] / rpc["random"])
gen24 = {g["kind"]: g for g in gen if g["bits"] == "24"}
print("RATIO_COSET_OVER_RANDOM_GENERIC", float(gen24["coset"]["rank_per_cpu"]) / float(gen24["random"]["rank_per_cpu"]))
print("CONTRASTS_24", [c for c in S["contrasts"] if c["bits"] == 24])
alg24 = [a for a in S["algebraic"] if a["bits"] == 24][0]
print("ALG_24", {k: alg24[k] for k in ("attempts", "completed", "found", "rank", "total_cpu", "generic_rank", "generic_cpu", "timeouts", "oracle_positive")})
print("GATES_24", [g for g in S["gates"] if g["bits"] == 24])
pat_off = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|not committed|scratch|Modal volume", re.I)
pat_aa = re.compile(r"A/A|null control|negative control|placebo|null-control", re.I)
pat_iso = re.compile(r"isolat|taskset|pinned|\bL0\b|tier|shared", re.I)
files = [os.path.join(STUDY, "README.md"), os.path.join(STUDY, "protocol.json"), os.path.join(STUDY, "timing_wrapper_addendum.json"), os.path.join(STUDY, "gate_sampling_addendum.json"), os.path.join(RES, "environment.json")]
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
print("GITDATE_PROTOCOL", gitdate("research/factor_base_evaluation_20261015/protocol.json") or "EMPTY")
print("GITDATE_FIRSTRAW", gitdate("research/factor_base_evaluation_20261015/results/cases.csv") or "EMPTY")
print("STORE_INDEX_EXISTS", os.path.exists(os.path.join(REPO, "store/index.json")))
print("ENVIRONMENT_JSON_EXISTS", os.path.exists(os.path.join(RES, "environment.json")))

# ---- row object ----
def mk(claimed, computed, definition, files, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files, "status": status}
CASES = ["research/factor_base_evaluation_20261015/results/cases.csv"]
GS = ["research/factor_base_evaluation_20261015/results/generic_summary.csv"]
SUM = ["research/factor_base_evaluation_20261015/results/summary.json"]
RPC = "sum(rank)/sum(total_cpu) over the 32 cases at bits=24 for the kind, cases.csv"
rand_contrast = [c for c in S["contrasts"] if c["bits"] == 24 and c["control"] == "random"][0]
claims_out = [
 mk("24-bit rows/CPU s low-x 1,352", f"{rpc['low_x']:.3f}", RPC, CASES + GS, "match"),
 mk("24-bit rows/CPU s coset 1,167", f"{rpc['coset']:.3f}", RPC, CASES + GS, "match"),
 mk("24-bit rows/CPU s random 1,144", f"{rpc['random']:.3f}", RPC, CASES + GS, "match"),
 mk("24-bit rows/CPU s low digit weight 859", f"{rpc['low_digit_weight']:.3f}", RPC, CASES + GS, "match"),
 mk("coset/random throughput 1.020", f"{rpc['coset']/rpc['random']:.4f}", "rpc coset / rpc random at bits=24", CASES, "match"),
 mk("95% field-bootstrap interval 0.959-1.083",
    f"stored contrast interval [{rand_contrast['throughput']['lower']:.4f}, {rand_contrast['throughput']['upper']:.4f}] (summary.json); bootstrap seed and estimator not stated in committed files, not re-derived",
    "whole-field bootstrap, 10000 draws per summary.json; procedure details not stated", SUM, "cannot_define"),
 mk("norm solver 90 rows", str(alg24["rank"]), "algebraic entry bits=24 rank field, summary.json", SUM, "match"),
 mk("norm solver 229.902 CPU s", f"{alg24['total_cpu']:.3f}", "algebraic entry bits=24 total_cpu field, summary.json", SUM, "match"),
 mk("exact join 115 rows", str(alg24["generic_rank"]), "algebraic entry bits=24 generic_rank field, summary.json", SUM, "match"),
 mk("exact join 0.268 CPU s", f"{alg24['generic_cpu']:.3f}", "algebraic entry bits=24 generic_cpu field, summary.json", SUM, "match"),
]
row = {
 "row_no": 62, "slug": "20261015-factor-base-evaluation", "claims": claims_out,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": offrepo, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (protocol.json added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (results/cases.csv added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/62.py",
 "notes": "The HEADLINE rates and ratios reproduce from cases.csv and generic_summary.csv (identical values for generic_summary rank_per_cpu). The 24-bit algebraic entry in summary.json has completed 0 of 96 attempts, timeouts 96, found 76, oracle_positive 84; the 24-bit gate in summary.json has planted successes 14 of 16 and known-positive successes 16 of 16, known-negative successes 0 of 16. README.md line 54-56 states no A/A receipt and host Intel Xeon Platinum 8573C pinned CPU 0. README.md line 90 states '.conductor/WORKFLOW.md returned 404 at the pinned base commit'. The bootstrap row uses the stored summary.json contrast; the committed files do not give the resampling seed or the estimator used. Off-repo grep matched no lines. store/index.json exists. The raw artifact is a zip under artifacts/ (not opened: contents not needed for the claims listed)."
}
with open(OUTJSONL, "a", encoding="utf-8") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("ROW62_WRITTEN", len(claims_out), "claims")
