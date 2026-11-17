# Row 47 | 20261016-isogeny-round5: recompute headline ratios from committed profiles and benchmark records.
import json, glob, math, os, re, statistics, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "isogeny_algos/docs/round5-20261016"
PROF = "isogeny_algos/results/p14-round5.txt"
BENCH = "isogeny_algos/results/p14-bench"
BD = os.path.join(C.REPO, BENCH)

def recs(group, arm):
    out = []
    for f in sorted(glob.glob(os.path.join(BD, f"{group}.{arm}.jsonl"))):
        for line in open(f):
            if line.strip():
                out.append(json.loads(line))
    return out

# ---- profile_ntt table (p14-round5.txt) ----
prof = {}
cur = None
in_ntt = False
for line in open(os.path.join(C.REPO, PROF)):
    if line.startswith("## "):
        in_ntt = "profile_ntt" in line
        cur = None
        continue
    if not in_ntt:
        continue
    m = re.match(r"# (FpM\d) \(", line)
    if m:
        cur = m.group(1); prof.setdefault(cur, {}); continue
    m = re.match(r"^\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s*$", line)
    if m and cur:
        prof[cur][int(m.group(1))] = (float(m.group(2)), float(m.group(3)), float(m.group(4)))
ntt2 = prof["FpM2"][8192]; ntt4 = prof["FpM4"][8192]
ntt_ratio2 = ntt2[0] / ntt2[1]; ntt_ratio4 = ntt4[0] / ntt4[1]

# ---- phi (sea_phi_precompute over FpM2_127), main vs branch, min of runs ----
def min_run(rs):
    return min(rs)
sea_main = recs("sea", "main.1") + recs("sea", "main.2")
sea_br = recs("sea", "round5-pre-bmss.1") + recs("sea", "round5-pre-bmss.2")
def by_key(rs, algo, field):
    return [r for r in rs if r.get("algo") == algo and r.get("field") == field]
phi_m = [int(r["ns"]) for r in by_key(sea_main, "sea_phi_precompute", "FpM2_127")]
phi_b = [int(r["ns"]) for r in by_key(sea_br, "sea_phi_precompute", "FpM2_127")]
phi_ratio_min = min(phi_m) / min(phi_b)
phi_ratio_all = [round(x / 1e9, 3) for x in phi_m], [round(x / 1e9, 3) for x in phi_b]

# ---- SEA point count over FpM2_127: records paired by run index (main.k with round5-pre-bmss.k), 24 per arm ----
def pc(arm_file):
    return [r for r in recs_file(arm_file) if r.get("algo") == "sea_point_count" and r.get("field") == "FpM2_127"]
def recs_file(name):
    return [json.loads(l) for l in open(os.path.join(BD, name)) if l.strip()]
pm = pc("sea.main.1.jsonl") + pc("sea.main.2.jsonl")
pb = pc("sea.round5-pre-bmss.1.jsonl") + pc("sea.round5-pre-bmss.2.jsonl")
pc_ratios = [int(a["ns"]) / int(b["ns"]) for a, b in zip(pm, pb)]
pc_geo = math.exp(sum(math.log(x) for x in pc_ratios) / len(pc_ratios))
pc_sum_main = sum(int(r["ns"]) for r in pm); pc_sum_br = sum(int(r["ns"]) for r in pb)
pc_sum_ratio = pc_sum_main / pc_sum_br

# ---- BMSS linear algebra ell = 101 ----
bm_main = recs("bmss", "main.1") + recs("bmss", "main.2") + recs("bmss", "main.3") + recs("bmss", "main.4")
bm_br = recs("bmss", "round5.1") + recs("bmss", "round5.2")
def bmss_ns(rs, bits):
    rows = [r for r in rs if r.get("algo") == "linear_algebra" and str(r.get("ell")) == "101" and str(r.get("p_bits")) == str(bits)]
    return [int(r["median_ns"]) for r in rows]
bm_ratio = {}
big_all_main = recs("big", "main.1") + recs("big", "main.2")
big_all_br = recs("big", "round5.1")
for bits in (40, 256, 511):
    m = bmss_ns(bm_main, bits); b = bmss_ns(bm_br, bits)
    if not m or not b:
        m = [int(r["median_ns"]) for r in big_all_main if r.get("algo") == "linear_algebra" and str(r.get("ell")) == "101" and str(r.get("p_bits")) == str(bits)]
        b = [int(r["median_ns"]) for r in big_all_br if r.get("algo") == "linear_algebra" and str(r.get("ell")) == "101" and str(r.get("p_bits")) == str(bits)]
    bm_ratio[bits] = (m, b, (min(m) / min(b)) if m and b else None)

# ---- Kohel from kernel polynomial ell = 1009 (big group) ----
big_main = recs("big", "main.1") + recs("big", "main.2")
big_br = recs("big", "round5.1")
def kohel_ns(rs, bits):
    rows = [r for r in rs if r.get("algo") == "kohel_given_h" and str(r.get("ell")) == "1009" and str(r.get("p_bits")) == str(bits) and r.get("task") == "codomain"]
    return [int(r["median_ns"]) for r in rows]
koh = {}
for bits in (256, 511):
    m = kohel_ns(big_main, bits); b = kohel_ns(big_br, bits)
    koh[bits] = (m, b, (min(m) / min(b)) if m and b else None)

claims = [
    {"claimed": "NTT 10-11x Karatsuba at n = 8192 (2 limbs)", "computed": f"{ntt_ratio2:.2f}x (karatsuba {ntt2[0]} us / ntt {ntt2[1]} us)",
     "definition": "profile_ntt FpM2 (2^127-1) row n=8192, karatsuba_us / ntt_us", "files": [PROF], "status": "match" if 10 <= ntt_ratio2 <= 11 else "mismatch"},
    {"claimed": "NTT 10-11x Karatsuba at n = 8192 (4 limbs)", "computed": f"{ntt_ratio4:.2f}x (karatsuba {ntt4[0]} us / ntt {ntt4[1]} us)",
     "definition": "profile_ntt FpM4 (2^255-19) row n=8192", "files": [PROF], "status": "match" if 10 <= ntt_ratio4 <= 11 else "mismatch"},
    {"claimed": "phi_l for all l <= 89 over 2^127-1: 9.4-9.6 s -> 2.3 s (4.1x)",
     "computed": f"main ns (s) {phi_ratio_all[0]} ; branch ns (s) {phi_ratio_all[1]} ; min-of-runs ratio {phi_ratio_min:.3f}",
     "definition": "sea_phi_precompute over FpM2_127, min over the two runs per arm of ns; study table gives 4.1-4.2x per run pair",
     "files": [f"{BENCH}/sea.main.1.jsonl", f"{BENCH}/sea.main.2.jsonl", f"{BENCH}/sea.round5-pre-bmss.1.jsonl", f"{BENCH}/sea.round5-pre-bmss.2.jsonl"],
     "status": "match" if abs(phi_ratio_min - 4.1) / 4.1 <= 0.03 else "mismatch"},
    {"claimed": "SEA point count over 2^127-1, 1.14x geomean (0.82-1.42)",
     "computed": f"geomean of per-record ratio (main.k record i over round5.k record i, 24 pairs) {pc_geo:.4f}; range {min(pc_ratios):.3f} to {max(pc_ratios):.3f}; sum main {pc_sum_main/1e9:.3f} s, sum branch {pc_sum_br/1e9:.3f} s over 24 records each (sum ratio {pc_sum_ratio:.3f}); REPORT sums 2.12 s / 1.81 s",
     "definition": "sea_point_count FpM2_127, run-index paired records (24 per arm); geomean and range as the REPORT labels them",
     "files": [f"{BENCH}/sea.main.1.jsonl", f"{BENCH}/sea.main.2.jsonl", f"{BENCH}/sea.round5-pre-bmss.1.jsonl", f"{BENCH}/sea.round5-pre-bmss.2.jsonl"],
     "status": "mismatch" if (abs(pc_geo - 1.14) / 1.14 > 0.01 or abs(min(pc_ratios) - 0.82) > 0.02 or abs(max(pc_ratios) - 1.42) > 0.02) else "match"},
    {"claimed": "BMSS linear algebra ell=101 at 40-bit: 40x", "computed": f"min main {min(bm_ratio[40][0])/1e9:.4f} s / min branch {min(bm_ratio[40][1])/1e6:.3f} ms = {bm_ratio[40][2]:.2f}" if bm_ratio[40][2] else "no records",
     "definition": "bmss linear_algebra ell 101 p_bits 40, min over runs of median_ns", "files": [f"{BENCH}/bmss.main.1.jsonl", f"{BENCH}/bmss.round5.1.jsonl"],
     "status": "match" if bm_ratio[40][2] and abs(bm_ratio[40][2] - 40) / 40 <= 0.05 else "mismatch"},
    {"claimed": "BMSS linear algebra ell=101 at 256-bit: 64x", "computed": f"{bm_ratio[256][2]:.2f}" if bm_ratio[256][2] else "no records in bmss records for 256-bit",
     "definition": "as above, p_bits 256", "files": [f"{BENCH}/bmss.round5.1.jsonl"],
     "status": "no_raw_in_repo" if not bm_ratio[256][2] else ("match" if abs(bm_ratio[256][2] - 64) / 64 <= 0.05 else "mismatch")},
    {"claimed": "BMSS linear algebra ell=101 at 511-bit: 52x", "computed": f"{bm_ratio[511][2]:.2f}" if bm_ratio[511][2] else "no records in bmss records for 511-bit",
     "definition": "as above, p_bits 511", "files": [f"{BENCH}/bmss.round5.1.jsonl"],
     "status": "no_raw_in_repo" if not bm_ratio[511][2] else ("match" if abs(bm_ratio[511][2] - 52) / 52 <= 0.05 else "mismatch")},
    {"claimed": "Kohel l=1009 2.0-2.4x (256 / 511-bit)",
     "computed": f"256-bit {koh[256][2]:.3f} (main {min(koh[256][0])/1e6:.2f} ms / branch {min(koh[256][1])/1e6:.2f} ms)" if koh[256][2] and koh[256][1] else "no records",
     "definition": "big kohel_given_h codomain ell 1009 p_bits 256, min over runs of median_ns",
     "files": [f"{BENCH}/big.main.1.jsonl", f"{BENCH}/big.round5.1.jsonl"],
     "status": "match" if koh[256][2] and abs(koh[256][2] - 2.4) / 2.4 <= 0.05 else "mismatch"},
    {"claimed": "Kohel l=1009 2.0x (511-bit)",
     "computed": f"511-bit {koh[511][2]:.3f} (main {min(koh[511][0])/1e6:.2f} ms / branch {min(koh[511][1])/1e6:.2f} ms)" if koh[511][2] and koh[511][1] else "no records",
     "definition": "as above, p_bits 511", "files": [f"{BENCH}/big.main.1.jsonl", f"{BENCH}/big.round5.1.jsonl"],
     "status": "match" if koh[511][2] and abs(koh[511][2] - 2.0) / 2.0 <= 0.05 else "mismatch"},
    {"claimed": "bit-identical NTT output (identical claim)", "computed": "no equality field for NTT vs Karatsuba in p14-round5.txt; tests/ntt.rs not run here",
     "definition": "NTT output equals Karatsuba output", "files": [PROF], "status": "no_raw_in_repo"},
    {"claimed": "148/148 tests", "computed": "no test output committed in the repo (cargo test not run; forbidden)",
     "definition": "cargo test count", "files": [f"{STUDY}/REPORT.md"], "status": "no_raw_in_repo"},
]

offrepo = C.grep_lines([STUDY], C.OFFREPO_PAT)
aa = C.grep_lines([STUDY], C.AA_PAT, cap=20)
iso = C.grep_lines([STUDY], C.ISO_PAT, cap=20)
row = {
    "row_no": 47,
    "slug": "20261016-isogeny-round5",
    "claims": claims,
    "identical": {"claimed": True, "n_compared": None, "n_equal": None, "n_differ": None, "files": [PROF],
                  "status": "no_raw_in_repo"},
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": "no PROTOCOL file in " + STUDY,
    "first_raw_git_date": C.git_first_add(PROF) + " | " + C.git_first_add(f"{BENCH}/sea.main.1.jsonl"),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/47.py",
    "notes": ("Card RAW CANDIDATES was empty; the REPORT names committed raw records results/p14-bench/*.jsonl and "
              "results/p14-round5.txt, which are used here. Profile table read from the profile_ntt section only "
              "(later sections of p14-round5.txt reuse the FpM headers). Benchmark ratios use the minimum over the "
              "runs of median_ns per arm (REPORT: min of two runs; bmss main has four). The SEA claim is checked "
              "two ways: run-index paired geomean and the REPORT's stated sums; neither reproduces the REPORT's "
              "2.12 s / 1.81 s sums or its 0.82-1.42 range. The identical claim (bit-identical NTT output) has no "
              "equality field in the committed profiles. Host: 4 vCPU shared VM, one run per profile (REPORT)."
              ),
}
print(json.dumps(row, ensure_ascii=False))
