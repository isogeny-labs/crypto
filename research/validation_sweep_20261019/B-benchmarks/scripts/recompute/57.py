# SWB-RECOMPUTE-08 row 57: 20261015-isogeny-p192-search. Reads committed files only.
import json, statistics, subprocess, re, os, glob
REPO = "/home/user/crypto"
STUDY = os.path.join(REPO, "research/pkm_isogeny_tower_p192_20261015")
def load(rel): return json.load(open(os.path.join(STUDY, rel)))
def reps(fam, arm):
    vals = []
    for f in sorted(glob.glob(os.path.join(STUDY, "data/c1", f"{fam}-{arm}-rep*.json"))):
        d = json.load(open(f)); vals.append(d["us_per_draw"])
    return vals
out = {}
b_z8 = reps("z2xz8", "baseline"); f_z8 = reps("z2xz8", "fast"); c_z8 = reps("z2xz8", "fast-chi")
b_z2 = reps("z2xz2", "baseline"); f_z2 = reps("z2xz2", "fast"); c_z2 = reps("z2xz2", "fast-chi")
med = statistics.median
out["n_reps"] = {"z2xz8": [len(b_z8), len(c_z8), len(f_z8)], "z2xz2": [len(b_z2), len(c_z2), len(f_z2)]}
out["z2xz8_fast_median_us"] = med(f_z8); out["z2xz8_fast_minmax_us"] = [min(f_z8), max(f_z8)]
out["z2xz8_baseline_median_us"] = med(b_z8)
out["z2xz8_fast_over_baseline"] = med(f_z8) / med(b_z8)
out["z2xz2_baseline_median_us"] = med(b_z2); out["z2xz2_fast_median_us"] = med(f_z2)
out["z2xz2_fast_over_baseline"] = med(f_z2) / med(b_z2)
out["z2xz8_fast_chi_median_us"] = med(c_z8); out["z2xz2_fast_chi_median_us"] = med(c_z2)
out["z2xz2_fast_chi_median_us"] = med(c_z2)
out["z2xz8_fast_over_fastchi"] = med(f_z8) / med(c_z8)
# depth counts from c4 (z2xz8, 2^28 draws) and c3 (z2xz2, 2^25 draws)
c4 = load("data/c4/summary.json"); c3 = load("data/c3/summary.json")
def count_ge(hist, t): return sum(hist[t:])
out["c4_draws"] = c4["draws"]; out["c3_draws"] = c3["draws"]
out["c4_count_ge_16"] = count_ge(c4["hist_depth"], 16)
out["c4_count_ge_24"] = count_ge(c4["hist_depth"], 24)
out["c3_count_b_eq_1"] = c3["hist_depth"][1]
out["c4_count_b_eq_1"] = c4["hist_depth"][1]
out["c4_model_p_ge_16_draws_per_success"] = c4["draws"] / count_ge(c4["hist_depth"], 16)
# derived t=48 core-hours: 2^45 draws x measured z2xz8 fast us/draw
us = med(f_z8)
out["t48_core_hours_from_median"] = (2**45) * us * 1e-6 / 3600
out["t48_core_hours_from_min_max"] = [(2**45) * min(f_z8) * 1e-6 / 3600, (2**45) * max(f_z8) * 1e-6 / 3600]
out["derived_json_claims"] = load("derived.json")["claims"]
out["derived_json_c1_median_z2xz8_fast"] = load("derived.json")["c1"]["z2xz8/fast"]["median_us"]
print(json.dumps(out, indent=1, sort_keys=True))
# raw helper outputs for verbatim copy
pat_off = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|not committed|scratch|Modal volume", re.I)
pat_aa = re.compile(r"A/A|null control|negative control|placebo|null-control", re.I)
pat_iso = re.compile(r"isolat|taskset|pinned|\bL0\b|tier|shared", re.I)
files = [os.path.join(STUDY, f) for f in ("README.md", "PROTOCOL.md", "report.typ") if os.path.exists(os.path.join(STUDY, f))]
offrepo, aa, iso = [], [], []
for fp in files:
    for i, line in enumerate(open(fp, encoding="utf-8", errors="replace"), 1):
        rel = os.path.relpath(fp, REPO)
        if pat_off.search(line): offrepo.append(f"{rel}:{i}: {line.strip()}")
        if pat_aa.search(line): aa.append(f"{rel}:{i}: {line.strip()}")
        if pat_iso.search(line): iso.append(f"{rel}:{i}: {line.strip()}")
print("OFFREPO", *offrepo, sep="\n")
print("AA", *aa, sep="\n")
print("ISO", *iso, sep="\n")
def gitdate(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel], capture_output=True, text=True)
    return r.stdout.strip()
print("GITDATE_PROTOCOL", gitdate("research/pkm_isogeny_tower_p192_20261015/PROTOCOL.md"))
print("GITDATE_FIRSTRAW", gitdate("research/pkm_isogeny_tower_p192_20261015/data/c1/z2xz2-baseline-rep2.json"))
print("STORE_INDEX_EXISTS", os.path.exists(os.path.join(REPO, "store/index.json")))

# ---- row object (values from the printed output above) ----
OUTJSONL = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-08.jsonl")
Z8 = "research/pkm_isogeny_tower_p192_20261015/data/c1/z2xz8-*-rep*.json"
Z2 = "research/pkm_isogeny_tower_p192_20261015/data/c1/z2xz2-*-rep*.json"
def fmt(x, nd): return f"{x:.{nd}f}"
claims = [
 {"claimed": "z2xz8 fast/baseline 0.326x", "computed": fmt(out["z2xz8_fast_over_baseline"], 3),
  "definition": "median us_per_draw (7 reps) fast / baseline, z2xz8", "files": [Z8], "status": "match"},
 {"claimed": "z2xz8 fast 10.06 us/draw", "computed": fmt(out["z2xz8_fast_median_us"], 2),
  "definition": "median of 7 rep us_per_draw, z2xz8 fast", "files": [Z8], "status": "match"},
 {"claimed": "z2xz8 fast min-max 9.78-10.19", "computed": f"{fmt(out['z2xz8_fast_minmax_us'][0],2)}-{fmt(out['z2xz8_fast_minmax_us'][1],2)}",
  "definition": "min and max of 7 rep us_per_draw, z2xz8 fast", "files": [Z8], "status": "match"},
 {"claimed": "z2xz8 baseline 30.83 us/draw", "computed": fmt(out["z2xz8_baseline_median_us"], 2),
  "definition": "median of 7 rep us_per_draw, z2xz8 baseline", "files": [Z8], "status": "match"},
 {"claimed": "z2xz8 fast-chi 12.48 us/draw", "computed": fmt(out["z2xz8_fast_chi_median_us"], 2),
  "definition": "median of 7 rep us_per_draw, z2xz8 fast-chi", "files": [Z8], "status": "match"},
 {"claimed": "z2xz2 fast/baseline 0.561x", "computed": fmt(out["z2xz2_fast_over_baseline"], 3),
  "definition": "median us_per_draw fast / baseline, z2xz2", "files": [Z2], "status": "match"},
 {"claimed": "z2xz2 fast 13.89 us/draw", "computed": fmt(out["z2xz2_fast_median_us"], 2),
  "definition": "median of 7 rep us_per_draw, z2xz2 fast", "files": [Z2], "status": "match"},
 {"claimed": "z2xz2 baseline 24.74 us/draw", "computed": fmt(out["z2xz2_baseline_median_us"], 2),
  "definition": "median of 7 rep us_per_draw, z2xz2 baseline", "files": [Z2], "status": "match"},
 {"claimed": "t=48 derived 98,352 core-hours", "computed": fmt(out["t48_core_hours_from_median"], 0),
  "definition": "2^45 draws x z2xz8 fast median us/draw / 3600 / 1e6", "files": [Z8, "research/pkm_isogeny_tower_p192_20261015/derived.json"], "status": "match"},
 {"claimed": "P(b>=t)=2^(3-t) consistent at every tested t (t=24 point)",
  "computed": f"count_ge(24)={out['c4_count_ge_24']} of {out['c4_draws']} draws (frac {out['c4_count_ge_24']/out['c4_draws']:.4e}); model 2^(3-24)={2**(3-24):.4e}; ratio {out['c4_count_ge_24']/(out['c4_draws']*2**(3-24)):.4f}",
  "definition": "sum of c4 hist_depth[24:] divided by c4 draws, compared with 2^(3-t)", "files": ["research/pkm_isogeny_tower_p192_20261015/data/c4/summary.json"], "status": "mismatch"},
]
row = {
 "row_no": 57, "slug": "20261015-isogeny-p192-search", "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": offrepo,
 "aa_lines": aa,
 "isolation_lines": iso,
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (PROTOCOL.md added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (data/c1/z2xz2-baseline-rep2.json added in commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/57.py",
 "notes": "Ten claims recomputed; of the HEADLINE claims, only 7 of 9 numeric claims were recomputed; the 2^38 curve (45.6 core-min) claim was not recomputed. The 2^28 z2xz8 draws and 45.6 min elapsed (c4 elapsed_s 2738.57) are in data/c4/summary.json. README states PROTOCOL committed as fc6317e51; git log reports the PROTOCOL.md add commit as ca00c151764f. The raw files and PROTOCOL.md share the add commit ca00c151764f. The offrepo grep over README.md, PROTOCOL.md, report.typ matched no lines. The A/A grep matched no lines; the README positive control (y^2=x^3-x, b=63) is not an A/A or null control. First raw file = first listed RAW CANDIDATE that exists in git. store/index.json exists in the repo; no store:<name> reference was found in the study files."
}
with open(OUTJSONL, "a", encoding="utf-8") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("ROW57_WRITTEN", len(claims), "claims")
