# Row 41 | 20261015-pair46-factor-bases: recompute headline claims from committed raw records.
import json, math, re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "research/pair46_factor_bases_20261015"
BASE = os.path.join(C.REPO, STUDY)


def mean(x):
    return sum(x) / len(x)


def ols_slope(xs, ys):
    mx, my = mean(xs), mean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


r01 = json.load(open(f"{BASE}/01-endo-chain/results.json"))
toy22 = [t for t in r01["toys"] if t["bits"] == 22][0]
arms22 = {a["arm"]: a for a in toy22["arms"]}
S_known = arms22["endo-chain-known"]["S"]
S_bsgs = arms22["bsgs-pm"]["S"]
pair = r01["pair46"]
derived = json.load(open(f"{BASE}/01-endo-chain/derived_ratios.json"))["rows"]
der22 = [r for r in derived if r["bits"] == 22 and r["num"] == "endo-chain-known" and r["den"] == "bsgs-pm"][0]

r06 = json.load(open(f"{BASE}/06-pkm-rational-map/results.json"))
s06 = json.load(open(f"{BASE}/06-pkm-rational-map/summary.json"))
levels = r06["levels"]

xs, ys = [], []
for L in levels:
    a = L["arms"]["A1_pkm_mult_chain"]
    if "gb" in a and len(a["gb"]) == 12 and L["n1"] <= 5:
        xs.append(math.log2(L["p"])); ys.append(math.log2(mean([g["time"] for g in a["gb"]])))
slopeA1 = ols_slope(xs, ys)

rx, ry = [], []
for L in levels:
    rx.append(math.log2(L["p"])); ry.append(math.log2(mean([r["ops"] for r in L["rho"]])))
slope_rho = ols_slope(rx, ry)

L5 = [L for L in levels if L["n1"] == 5][0]
rho_t = [r["time"] for r in L5["rho"]]
a4 = L5["arms"]["A4_dickson_chain"]
ic_rho_a4 = ((a4["F"] + 10) / mean(a4["valid"])) * mean([g["time"] for g in a4["gb"]]) / mean(rho_t)
stored_ic_a4 = [e for e in s06["levels"] if e["n1"] == 5][0]["arms"]["A4_dickson_chain"]["ic_over_rho"]

F01 = ["README.md", "01-endo-chain", "06-pkm-rational-map"]
F01_FILES = [f"{STUDY}/01-endo-chain/results.json", f"{STUDY}/01-endo-chain/derived_ratios.json",
             f"{STUDY}/06-pkm-rational-map/results.json", f"{STUDY}/06-pkm-rational-map/summary.json"]

claims = [
    {"claimed": "S = 1.47", "computed": f"{mean(S_known):.4f} (mean of 40 per-target S, endo-chain-known, 22-bit)",
     "definition": "arithmetic mean of recorded per-target S over the 40 targets of arm endo-chain-known in toy 22 bit",
     "files": [f"{STUDY}/01-endo-chain/results.json"], "status": "match"},
    {"claimed": "[1.28, 1.69] (S endo-chain-known CI)", "computed": "not recomputed",
     "definition": "percentile bootstrap 95% CI; derived_ratios.json method says 2000 resamples seed 19 but names no RNG; "
                   "the S CI seed is 7 in PROTOCOL.md and its RNG is not stated",
     "files": [f"{STUDY}/01-endo-chain/PROTOCOL.md"], "status": "cannot_define"},
    {"claimed": "0.955", "computed": f"{mean(S_bsgs):.4f} (mean of 40 per-target S, bsgs-pm, 22-bit)",
     "definition": "arithmetic mean of recorded per-target S, arm bsgs-pm, toy 22 bit",
     "files": [f"{STUDY}/01-endo-chain/results.json"], "status": "match"},
    {"claimed": "[0.86, 1.05] (bsgs-pm CI)", "computed": "not recomputed",
     "definition": "percentile bootstrap CI; RNG not stated (see row claim 2)",
     "files": [f"{STUDY}/01-endo-chain/PROTOCOL.md"], "status": "cannot_define"},
    {"claimed": "ratio 1.54", "computed": f"{mean(S_known)/mean(S_bsgs):.4f} (ratio of means endo-chain-known / bsgs-pm, 22-bit)",
     "definition": "ratio of arm means of per-target S (derived_ratios.json method: ratio of means); stored derived value "
                   f"{der22['ratio'][0]}",
     "files": [f"{STUDY}/01-endo-chain/results.json", f"{STUDY}/01-endo-chain/derived_ratios.json"], "status": "match"},
    {"claimed": "[1.30, 1.82] (ratio CI)", "computed": f"not recomputed (stored in derived_ratios.json: {der22['ratio'][1]}, {der22['ratio'][2]})",
     "definition": "bootstrap CI of ratio of means, seed 19; RNG not stated",
     "files": [f"{STUDY}/01-endo-chain/derived_ratios.json"], "status": "cannot_define"},
    {"claimed": "psi x-evaluation 454 ns vs addition 462 ns (c_E 0.98)",
     "computed": f"ns_psi_x={pair['ns_psi_x']} ns_add={pair['ns_add']} c_E={pair['ns_psi_x']/pair['ns_add']:.4f}",
     "definition": "ns_psi_x and ns_add as recorded in pair46 block; c_E = ns_psi_x / ns_add",
     "files": [f"{STUDY}/01-endo-chain/results.json"], "status": "match"},
    {"claimed": "per-system cost slope p^2.02 [2.00, 2.04] (A1)",
     "computed": f"slope={slopeA1:.4f} (OLS log2 mean T on log2 p, levels 2-5, 12 targets); stored={s06['slopes']['A1_pkm_mult_chain']['slope_all']:.4f} ci={[round(v,4) for v in s06['slopes']['A1_pkm_mult_chain']['ci']]}",
     "definition": "OLS of log2(mean gb time) on log2(p), n1 = 2..5, arm A1 (results.json); CI is a stored bootstrap not regenerated",
     "files": [f"{STUDY}/06-pkm-rational-map/results.json", f"{STUDY}/06-pkm-rational-map/summary.json"],
     "status": "partial"},
    {"claimed": "rho p^0.52", "computed": f"slope={slope_rho:.4f} (OLS log2 mean rho ops on log2 p, n1 = 2..7)",
     "definition": "OLS of log2(mean rho ops per level) on log2(p), all six levels",
     "files": [f"{STUDY}/06-pkm-rational-map/results.json"], "status": "match"},
    {"claimed": "projected relation stage about 4e6x rho at n1 = 5 (Python constants)",
     "computed": f"{ic_rho_a4:.4e} (A4 Dickson chain; stored {stored_ic_a4:.4e}); A1 at n1=5 = "
                 f"{((L5['arms']['A1_pkm_mult_chain']['F']+10)/mean(L5['arms']['A1_pkm_mult_chain']['valid']))*mean([g['time'] for g in L5['arms']['A1_pkm_mult_chain']['gb']])/mean(rho_t):.4e}",
     "definition": "((F+10)/mean valid relations) x mean system time / mean rho time, n1 = 5, best chained arm A4 (analyze.py ic_over_rho point estimate)",
     "files": [f"{STUDY}/06-pkm-rational-map/results.json", f"{STUDY}/06-pkm-rational-map/analyze.py"], "status": "match"},
]

offrepo = C.grep_lines([f"{STUDY}/README.md", f"{STUDY}/01-endo-chain", f"{STUDY}/06-pkm-rational-map",
                        f"{STUDY}/02-deuring-lift", f"{STUDY}/03-floor-pullback", f"{STUDY}/04-family-sweep",
                        f"{STUDY}/05-qr-window", f"{STUDY}/07-torsion-division", f"{STUDY}/08-learned",
                        f"{STUDY}/10-bounded-degree", f"{STUDY}/12-model-small-x", f"{STUDY}/13-xy-windows",
                        f"{STUDY}/14-dor-verdict"], C.OFFREPO_PAT)
aa = C.grep_lines([f"{STUDY}/README.md", f"{STUDY}/01-endo-chain/RESULTS.md", f"{STUDY}/06-pkm-rational-map/RESULTS.md"],
                  C.AA_PAT)
iso = C.grep_lines([f"{STUDY}/README.md", f"{STUDY}/01-endo-chain/RESULTS.md", f"{STUDY}/06-pkm-rational-map/RESULTS.md"],
                   C.ISO_PAT)

row = {
    "row_no": 41,
    "slug": "20261015-pair46-factor-bases",
    "claims": claims,
    "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [],
                  "status": "not_claimed"},
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": " | ".join([
        "01 PROTOCOL: " + C.git_first_add(f"{STUDY}/01-endo-chain/PROTOCOL.md"),
        "06 PROTOCOL: " + C.git_first_add(f"{STUDY}/06-pkm-rational-map/PROTOCOL.md")]),
    "first_raw_git_date": " | ".join([
        "01 results.json: " + C.git_first_add(f"{STUDY}/01-endo-chain/results.json"),
        "06 results.json: " + C.git_first_add(f"{STUDY}/06-pkm-rational-map/results.json")]),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/41.py",
    "notes": ("Row HEADLINE has no identical-output claim; identical field not applicable. Items 01 and 06 are "
              "computed from the per-target S (01) and per-system times (06) as recorded in results.json. "
              "The bootstrap CIs of the headline are not regenerated: the RNG is not stated (derived_ratios.json "
              "says seed 19 with percentile bootstrap, PROTOCOL.md says seed 7 for S CIs). The 06 projection "
              "uses analyze.py's ic_over_rho formula. Headline 'upheld 12/13' and 'item 14 refuted' counts are "
              "not recomputed; they are red-team verdicts in REDTEAM.md files, not raw records. "
              "protocol_git_date and first_raw_git_date give the git commit dates of the first add commit."),
}
print(json.dumps(row, ensure_ascii=False))
