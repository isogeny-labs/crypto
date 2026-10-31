# Row 46 | 20261015-genus3-2lp-rebalance: recompute headline fits, paired ratios and rho ratio from committed records.
import json, glob, math, os, re, statistics, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "research/ic_improve_20261014/04b-genus3-2lp-rebalance"
TEST = f"{STUDY}/results/test"
RHO = f"{STUDY}/results/rho"
TD = os.path.join(C.REPO, STUDY, "results/test")

def load_test(k, kap):
    out = {}
    for f in glob.glob(os.path.join(TD, f"te_k{k}_kap{kap}_c*_t*.jsonl")):
        m = re.search(r"_c(\d+)_t(\d+)\.jsonl$", f)
        out[(int(m.group(1)), int(m.group(2)))] = json.loads(open(f).readline())
    return out

def ols(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

KAP = {k: "2" for k in range(10, 21)}
KAP_A1 = {14: "2", 15: "2", 16: "2", 17: "1.4", 18: "1.4", 19: "1.4", 20: "1.4"}
bl = {k: load_test(k, "2") for k in range(10, 21)}
a1 = {k: (load_test(k, KAP_A1[k]) if KAP_A1[k] != "2" else bl[k]) for k in range(14, 21)}
# A1 at 14-16 is the kappa-2 run (kappa_used A1/14..16 = 2.0); instances are the same records.

def series(R_by_k, fn, ks):
    return {k: statistics.mean(fn(r) for r in R_by_k[k].values()) for k in ks}

q_of = {k: next(iter(bl[k].values()))["q"] for k in range(10, 21)}
ks = list(range(14, 21))
BL_tot = series(bl, lambda r: r["la"][0]["total_mults"], ks)      # la[0] total_mults includes relation phase
SG_tot = series(bl, lambda r: r["la"][1]["total_mults"], ks)      # la[1] = sge_lanczos, same relations
REL = series(bl, lambda r: r["rel"]["rel_mults"], ks)
LA_only = series(bl, lambda r: r["la"][0]["total_mults"] - r["rel"]["rel_mults"], ks)
A1_tot = series(a1, lambda r: r["la"][0]["total_mults"], ks)

fit = lambda ser: ols([math.log(q_of[k]) for k in ks], [math.log(ser[k]) for k in ks])
E_BL, E_A2, E_rel, E_LA, E_A1 = fit(BL_tot), fit(SG_tot), fit(REL), fit(LA_only), fit(A1_tot)

def paired_geo(num_R, den_R, k, numfn, denfn):
    keys = sorted(set(num_R[k]) & set(den_R[k]))
    rs = [numfn(num_R[k][x]) / denfn(den_R[k][x]) for x in keys]
    return math.exp(sum(math.log(r) for r in rs) / len(rs)), len(keys)

A1_BL_20, n20 = paired_geo(a1, bl, 20, lambda r: r["la"][0]["total_mults"], lambda r: r["la"][0]["total_mults"])
A2_BL_14, n14 = paired_geo(bl, bl, 14, lambda r: r["la"][1]["total_mults"], lambda r: r["la"][0]["total_mults"])
A2_BL_20, _ = paired_geo(bl, bl, 20, lambda r: r["la"][1]["total_mults"], lambda r: r["la"][0]["total_mults"])

# S per instance and rho reference
def S_mean(k):
    return statistics.mean(r["la"][0]["total_mults"] / math.sqrt(r["ell"]) for r in bl[k].values())
rho14 = [json.loads(open(os.path.join(C.REPO, RHO, f)).readline()) for f in sorted(os.listdir(os.path.join(C.REPO, RHO))) if "_k14_" in f]
S_rho14 = statistics.mean(r["S"] for r in rho14)
w_row = lambda k: statistics.mean(r["la"][0]["w_row"] for r in bl[k].values())

claims = [
    {"claimed": "baseline fit q^1.383 over 2^14-2^20", "computed": f"{E_BL:.4f}",
     "definition": "OLS slope of ln(mean la[0].total_mults) on ln q, k=14..20 (total includes relation phase per record field definitions)",
     "files": [f"{TEST}/te_k14_kap2_c*_t*.jsonl", f"{TEST}/te_k20_kap2_c*_t*.jsonl"],
     "status": "match" if abs(E_BL - 1.383) <= 0.0006 else "mismatch"},
    {"claimed": "99% interval [1.382, 1.384] for the baseline fit", "computed": "not recomputed (bootstrap B=10000, seed 20261016; RNG not stated)",
     "definition": "nonparametric bootstrap over 12 instances per size", "files": [f"{STUDY}/analysis.json"], "status": "cannot_define"},
    {"claimed": "relation phase q^1.332", "computed": f"{E_rel:.4f}",
     "definition": "OLS slope of ln(mean rel.rel_mults) on ln q, k=14..20", "files": [f"{TEST}/te_k14_kap2_c*_t*.jsonl"],
     "status": "match" if abs(E_rel - 1.332) <= 0.0006 else "mismatch"},
    {"claimed": "linear algebra q^1.447", "computed": f"{E_LA:.4f}",
     "definition": "OLS slope of ln(mean (la[0].total_mults - rel.rel_mults)) on ln q, k=14..20 (LA = elimination + Lanczos + verify, per record)",
     "files": [f"{TEST}/te_k14_kap2_c*_t*.jsonl"], "status": "match" if abs(E_LA - 1.447) <= 0.0006 else "mismatch"},
    {"claimed": "row weight 7.8 at 2^14", "computed": f"{w_row(14):.4f} (mean la[0].w_row, k=14)",
     "definition": "mean of la[0].w_row over 12 instances", "files": [f"{TEST}/te_k14_kap2_c*_t*.jsonl"],
     "status": "match" if abs(w_row(14) - 7.8) / 7.8 <= 0.01 else "mismatch"},
    {"claimed": "row weight 14.4 at 2^20", "computed": f"{w_row(20):.4f} (mean la[0].w_row, k=20)",
     "definition": "mean of la[0].w_row over 12 instances", "files": [f"{TEST}/te_k20_kap2_c*_t*.jsonl"],
     "status": "match" if abs(w_row(20) - 14.4) / 14.4 <= 0.01 else "mismatch"},
    {"claimed": "retuned kappa: q^1.373", "computed": f"{E_A1:.4f} (A1: kappa 2 at 14-16, kappa 1.4 at 17-20, la[0] totals)",
     "definition": "OLS slope as for baseline, arm A1 with kappa_used per analysis.json", "files": [f"{TEST}/te_k17_kap1.4_c*_t*.jsonl"],
     "status": "match" if abs(E_A1 - 1.373) <= 0.0006 else "mismatch"},
    {"claimed": "paired ratio 0.961 at 2^20 (A1/BL)", "computed": f"{A1_BL_20:.4f} (geometric mean over {n20} paired instances)",
     "definition": "per instance r = total_A1/total_BL, geometric mean at k=20 (protocol section 6)", "files": [f"{TEST}/te_k20_kap1.4_c*_t*.jsonl", f"{TEST}/te_k20_kap2_c*_t*.jsonl"],
     "status": "match" if abs(A1_BL_20 - 0.961) / 0.961 <= 0.01 else "mismatch"},
    {"claimed": "SGE q^1.419 (A2 fit)", "computed": f"{E_A2:.4f}", "definition": "OLS slope of ln(mean la[1].total_mults) on ln q, k=14..20",
     "files": [f"{TEST}/te_k14_kap2_c*_t*.jsonl"], "status": "match" if abs(E_A2 - 1.419) <= 0.0006 else "mismatch"},
    {"claimed": "SGE paired ratio 0.781 at 2^14 (A2/BL)", "computed": f"{A2_BL_14:.4f} (geometric mean over {n14} instances)",
     "definition": "per instance la[1].total_mults / la[0].total_mults, geometric mean at k=14", "files": [f"{TEST}/te_k14_kap2_c*_t*.jsonl"],
     "status": "match" if abs(A2_BL_14 - 0.781) / 0.781 <= 0.01 else "mismatch"},
    {"claimed": "SGE paired ratio 0.906 at 2^20 (A2/BL)", "computed": f"{A2_BL_20:.4f}",
     "definition": "as above at k=20", "files": [f"{TEST}/te_k20_kap2_c*_t*.jsonl"],
     "status": "match" if abs(A2_BL_20 - 0.906) / 0.906 <= 0.01 else "mismatch"},
    {"claimed": "rebuilt baseline / rho on Jac(C) 0.16 raw at 2^14 (n = 4)",
     "computed": f"S_BL(14) mean {S_mean(14):.2f} / S_rho(14) mean {S_rho14:.2f} (n={len(rho14)}) = {S_mean(14)/S_rho14:.4f}",
     "definition": "S = total_mults / sqrt(ell) per instance, mean over 12; S_rho = mean S field over the 4 rho records at k=14",
     "files": [f"{TEST}/te_k14_kap2_c*_t*.jsonl", f"{RHO}/rho_k14_*.jsonl"],
     "status": "match" if abs(S_mean(14)/S_rho14 - 0.16) <= 0.005 else "mismatch"},
]

offrepo = C.grep_lines([STUDY], C.OFFREPO_PAT)
aa = C.grep_lines([f"{STUDY}/README.md", f"{STUDY}/REPORT.typ"], C.AA_PAT, cap=20)
iso = C.grep_lines([f"{STUDY}/README.md", f"{STUDY}/PROTOCOL.md", f"{STUDY}/REPORT.typ"], C.ISO_PAT, cap=20)
row = {
    "row_no": 46,
    "slug": "20261015-genus3-2lp-rebalance",
    "claims": claims,
    "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [],
                  "status": "not_claimed"},
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": C.git_first_add(f"{STUDY}/PROTOCOL.md"),
    "first_raw_git_date": C.git_first_add(f"{TEST}/te_k14_kap2_c1_t1.jsonl"),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/46.py",
    "notes": ("Definition check: the la[i].total_mults field includes relation multiplications (rel_mults + elimination + "
              "Lanczos + verification); the BL total used by the study equals la[0].total_mults. "
              "A1 arm reuses kap2 records at 14-16 (kappa_used 2.0) and kap1.4 records at 17-20. "
              "Bootstrap CIs are not regenerated (B=10000, seed 20261016 stated; RNG not stated). "
              "Host: shared 4-vCPU container, no isolation (PROTOCOL.md section 5). The analysis.json file "
              "is the study's own output and was not used as the source of the recomputed numbers."),
}
print(json.dumps(row, ensure_ascii=False))
