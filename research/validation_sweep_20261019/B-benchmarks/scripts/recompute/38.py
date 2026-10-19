#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 38 (20261016-isogeny-round5).
# The card lists no raw candidates; the study names results/p14-bench/*.jsonl (records) and
# results/p14-round5.txt (profile_ntt tables). Reads only committed files. Prints one JSON object.
import glob
import json
import math
import os
import re
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
ISO = "isogeny_algos"
BENCH = f"{ISO}/results/p14-bench"
ROUND5_TXT = f"{ISO}/results/p14-round5.txt"
DOC = f"{ISO}/docs/round5-20261016/REPORT.md"
ROW_NO = 38
SLUG = "20261016-isogeny-round5"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/38.py"
RAW_CANDIDATES = sorted(glob.glob(f"{BENCH}/*.jsonl")) + [ROUND5_TXT]
TEXT_FILES = [DOC, ROUND5_TXT]
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3|shared")


def load(arm_glob):
    out = []
    for path in sorted(glob.glob(f"{BENCH}/{arm_glob}")):
        out.append((path, [json.loads(line) for line in open(path, encoding="utf-8") if line.strip()]))
    return out


def rel(ok):
    return "match" if ok else "mismatch"


claims = []

# (1) NTT vs Karatsuba at len 8192, profile_ntt tables in results/p14-round5.txt
ntt = {}
cur = None
for line in open(ROUND5_TXT, encoding="utf-8"):
    m = re.match(r"#\s*(FpM\d)\b", line)
    if m:
        cur = m.group(1)
        continue
    m = re.match(r"\s*(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s*$", line)
    if m and cur:
        ntt[(cur, int(m.group(1)))] = (float(m.group(2)), float(m.group(3)), float(m.group(4)))
r2 = ntt[("FpM2", 8192)]
r4 = ntt[("FpM4", 8192)]
claims.append({
    "claimed": "NTT 10-11x Karatsuba at n = 8192 (2 and 4 limbs)",
    "computed": (f"FpM2 karatsuba/ntt = {r2[0] / r2[1]:.3f} (table ratio col {r2[2]:.2f}); "
                 f"FpM4 karatsuba/ntt = {r4[0] / r4[1]:.3f} (table ratio col {r4[2]:.2f})"),
    "definition": "karatsuba_us / ntt_us at len 8192 from the profile_ntt table, FpM2 and FpM4",
    "files": [ROUND5_TXT],
    "status": rel(round(r2[0] / r2[1]) in (10, 11) and round(r4[0] / r4[1]) in (10, 11)),
})


# (2) Phi_l for all l <= 89 over 2^127-1 (field FpM2_127), main vs round5-pre-bmss
def phi_pre(arm_glob):
    vals = []
    for path, recs in load(arm_glob):
        for r in recs:
            if r.get("algo") == "sea_phi_precompute" and r.get("field") == "FpM2_127" and str(r.get("max_ell")) == "89":
                vals.append((path, r["ns"]))
    return vals


pm = phi_pre("sea.main.*.jsonl")
pb = phi_pre("sea.round5-pre-bmss.*.jsonl")
main_min = min(v for _p, v in pm)
br_min = min(v for _p, v in pb)
claims.append({
    "claimed": "Phi_l l<=89 over 2^127-1: main 9.36-9.57 s -> 2.26-2.28 s (4.1-4.2x)",
    "computed": (f"main {min(v for _p, v in pm) / 1e9:.3f}-{max(v for _p, v in pm) / 1e9:.3f} s ({len(pm)} runs); "
                 f"branch {min(v for _p, v in pb) / 1e9:.3f}-{max(v for _p, v in pb) / 1e9:.3f} s ({len(pb)} runs); "
                 f"min/min ratio {main_min / br_min:.3f}"),
    "definition": "sea_phi_precompute ns, field FpM2_127, max_ell 89; min over runs per arm; ratio main/branch",
    "files": [p for p, _v in pm] + [p for p, _v in pb],
    "status": rel(round(main_min / 1e9, 2) == 9.36 and round(max(v for _p, v in pm) / 1e9, 2) == 9.57
                  and round(min(v for _p, v in pb) / 1e9, 2) == 2.26 and round(max(v for _p, v in pb) / 1e9, 2) == 2.28),
})


# (3) SEA point count over 2^127-1 (field FpM2_127): sum over instances, and geomean of per-instance ratios
def sea_by(arm_glob):
    per = {}
    for path, recs in load(arm_glob):
        for r in recs:
            if r.get("algo") == "sea_point_count" and r.get("field") == "FpM2_127":
                per.setdefault((r["instance"], r["cycle_degree"]), []).append((r["ns"], path))
    return per


sm = sea_by("sea.main.*.jsonl")
sb = sea_by("sea.round5-pre-bmss.*.jsonl")
keys = sorted(set(sm) & set(sb), key=lambda x: (int(x[0]), int(x[1])))
main_sum = sum(min(v for v, _ in sm[k]) for k in keys)
br_sum = sum(min(v for v, _ in sb[k]) for k in keys)
ratios = [min(v for v, _ in sm[k]) / min(v for v, _ in sb[k]) for k in keys]
geo = math.exp(sum(math.log(x) for x in ratios) / len(ratios))
sea_files = sorted({p for k in keys for _v, p in sm[k]} | {p for k in keys for _v, p in sb[k]})
n_recs_main = sum(len(sm[k]) for k in keys)
claims.append({
    "claimed": "SEA point count over 2^127-1 (24 records, sum): 2.12 s -> 1.81 s",
    "computed": (f"per instance min over runs, summed over {len(keys)} instances: main {main_sum / 1e9:.3f} s, "
                 f"branch {br_sum / 1e9:.3f} s; total main records {n_recs_main}"),
    "definition": "sum over instances of min-over-runs sea_point_count ns, field FpM2_127",
    "files": sea_files,
    "status": rel(round(main_sum / 1e9, 2) == 2.12 and round(br_sum / 1e9, 2) == 1.81),
})
claims.append({
    "claimed": "SEA point count geomean speedup 1.14 (range 0.82-1.42)",
    "computed": (f"geomean main/branch {geo:.4f}; range {min(ratios):.4f}-{max(ratios):.4f} "
                 f"over {len(ratios)} instances"),
    "definition": "geometric mean and min/max of per-instance min-over-runs ratio main/branch",
    "files": sea_files,
    "status": rel(round(geo, 2) == 1.14 and round(min(ratios), 2) == 0.82 and round(max(ratios), 2) == 1.42),
})


# (4) BMSS linear algebra ell = 101: 40-bit in bmss.* group; 256/511-bit in big.* group (linear_algebra)
def bmss_arm(arm_glob, pbits, group_algo_filter):
    vals = []
    for path, recs in load(arm_glob):
        for r in recs:
            if group_algo_filter(r) and str(r.get("ell")) == "101" and str(r.get("p_bits")) == pbits:
                vals.append((r["median_ns"], path))
    return vals


bmss_files = []
for pbits, claimed, main_glob, branch_glob, filt in (
        ("40", 40, "bmss.main.*.jsonl", "bmss.round5.*.jsonl", lambda r: r.get("algo") == "linear_algebra"),
        ("256", 64, "big.main.*.jsonl", "big.round5.*.jsonl", lambda r: r.get("algo") == "linear_algebra"),
        ("511", 52, "big.main.*.jsonl", "big.round5.*.jsonl", lambda r: r.get("algo") == "linear_algebra")):
    bm = bmss_arm(main_glob, pbits, filt)
    bb = bmss_arm(branch_glob, pbits, filt)
    bmss_files += sorted({p for _v, p in bm} | {p for _v, p in bb})
    mn = min(v for v, _ in bm)
    bn = min(v for v, _ in bb)
    claims.append({
        "claimed": f"BMSS linear algebra ell=101, {pbits}-bit p: {claimed}x",
        "computed": (f"main min {mn} ns over {len(bm)} runs; branch min {bn} ns over {len(bb)} runs; "
                     f"ratio {mn / bn:.2f}"),
        "definition": "linear_algebra median_ns, ell 101, p_bits; min over runs per arm; ratio main/branch",
        "files": sorted({p for _v, p in bm} | {p for _v, p in bb}),
        "status": rel(round(mn / bn) == claimed),
    })

# (5) Kohel ell = 1009 (big group), 256 and 511-bit, main vs round5 (after change 6)
kohel = {}
for arm_glob, arm in (("big.main.*.jsonl", "main"), ("big.round5.*.jsonl", "branch")):
    for path, recs in load(arm_glob):
        for r in recs:
            if r.get("algo") == "kohel_given_h" and str(r.get("ell")) == "1009" and r.get("task") == "codomain":
                kohel.setdefault((str(r.get("p_bits")), r.get("task"), arm), []).append((r["median_ns"], path))
tasks = sorted({k[1] for k in kohel})
kohel_text = []
for pbits in ("256", "511"):
    for t in tasks:
        m = kohel.get((pbits, t, "main"))
        b = kohel.get((pbits, t, "branch"))
        if m and b:
            kohel_text.append(f"{pbits}-bit task {t}: main min {min(v for v, _ in m)} ns, branch min {min(v for v, _ in b)} ns, "
                              f"ratio {min(v for v, _ in m) / min(v for v, _ in b):.2f}")
kohel_files = sorted({p for lst in kohel.values() for _v, p in lst})
claims.append({
    "claimed": "Kohel from kernel polynomial ell=1009, 256/511-bit: 2.4x / 2.0x (10.2 / 24.9 ms -> 4.2 / 12.4 ms)",
    "computed": "; ".join(kohel_text) if kohel_text else "no matched kohel task present in main and branch",
    "definition": "kohel_given_h median_ns, ell 1009, task codomain, p_bits 256/511; min over runs per arm; ratio main/branch",
    "files": kohel_files,
    "status": ("match" if kohel_text and all(
        round(float(x.split("ratio ")[1]), 1) in (2.4, 2.0) for x in kohel_text) else "mismatch") if kohel_text else "cannot_define",
})

claims = claims[:10]

# identical: records carry 'verified' flags only, no output digests
verified = {}
for path in sorted(glob.glob(f"{BENCH}/*.jsonl")):
    name = os.path.basename(path)
    arm = name.split(".")[1] if name.count(".") >= 2 else "?"
    recs = [json.loads(line) for line in open(path, encoding="utf-8") if line.strip()]
    slot = verified.setdefault(arm, [0, 0])
    slot[0] += sum(1 for r in recs if r.get("verified") is True)
    slot[1] += sum(1 for r in recs if r.get("verified") is False)

offrepo = []
aa_lines = []
iso_lines = []
for f in TEXT_FILES:
    with open(f, encoding="utf-8", errors="replace") as fh:
        for i, line in enumerate(fh, 1):
            if OFFREPO_RE.search(line):
                offrepo.append(f"{f}:{i}: {line.rstrip()}")
            if AA_RE.search(line):
                aa_lines.append(f"{f}:{i}: {line.rstrip()}")
            if ISO_RE.search(line):
                iso_lines.append(f"{f}:{i}: {line.rstrip()}")
for s in sorted(set(re.findall(r"store:([A-Za-z0-9_.\-]+)", " ".join(offrepo)))):
    listed = "no index found"
    for p in (f"{REPO}/store/index.json", f"{ISO}/store/index.json"):
        if os.path.exists(p):
            listed = "yes" if s in open(p, encoding="utf-8", errors="replace").read() else "no"
            break
    offrepo.append(f"store:{s} listed in store/index.json: {listed}")


def git_add(path):
    out = subprocess.run(["git", "log", "--diff-filter=A", "--format=%H %cI", "--", path],
                         capture_output=True, text=True, cwd=REPO).stdout.strip()
    return out.split("\n")[-1].split(" ")[1] if out else ""


raw_dates = sorted((git_add(p), p) for p in RAW_CANDIDATES if git_add(p))
first_raw = raw_dates[0] if raw_dates else ("", "")

out = {
    "row_no": ROW_NO,
    "slug": SLUG,
    "claims": claims,
    "identical": {
        "claimed": True,
        "n_compared": None,
        "n_equal": None,
        "n_differ": None,
        "files": [f"{BENCH}/*.jsonl"],
        "status": "no_raw_in_repo",
    },
    "offrepo_refs": offrepo,
    "aa_lines": aa_lines,
    "isolation_lines": iso_lines,
    "protocol_git_date": "",
    "first_raw_git_date": first_raw[0],
    "script": SCRIPT,
    "notes": (
        "card RAW CANDIDATES list is empty; records used: results/p14-bench/*.jsonl and results/p14-round5.txt, "
        "named by docs/round5-20261016/REPORT.md. No PROTOCOL file in the study. "
        "identical: HEADLINE says 'bit-identical output' for the NTT; records carry only a 'verified' flag, "
        f"counted (verified true, verified false) per arm: {verified}. "
        "148/148 tests: no test log in the study paths checked, not computed. "
        f"first raw file by git add date: {first_raw[1]}"
    ),
}
print(json.dumps(out, ensure_ascii=False))
