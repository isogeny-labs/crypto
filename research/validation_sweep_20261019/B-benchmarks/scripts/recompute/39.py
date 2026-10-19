#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 39 (20261016-hwcost-rho-model).
# Reads only committed files under research/hwcost_model_20261016. Prints one JSON object.
import csv
import json
import os
import re
import statistics
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
STUDY = "research/hwcost_model_20261016"
RUN = f"{STUDY}/runs/final"
ROW_NO = 39
SLUG = "20261016-hwcost-rho-model"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/39.py"
RAW_CANDIDATES = [
    f"{RUN}/measurements.json",
    f"{RUN}/profile.json",
    f"{RUN}/rho.csv",
    f"{RUN}/rho_negation.csv",
    f"{RUN}/samples.csv",
    f"{RUN}/validation.json",
]
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3|L0")


def rel(ok):
    return "match" if ok else "mismatch"


val = json.load(open(f"{RUN}/validation.json"))
rows = val["rows"]
claims = []

# V1: calibrated model, test batches, median |err_calibrated| per field
v1 = {}
for field in ("fp64", "w1", "w2", "w3", "w4"):
    e = [abs(r["err_calibrated"]) for r in rows if r["field"] == field and r["set"] == "test"]
    v1[field] = statistics.median(e)
v1_lo, v1_hi = min(v1.values()), max(v1.values())
claims.append({
    "claimed": "calibrated model, B>=4096 test batches: median |error| 1.8-5.5 %, 5/5 PASS",
    "computed": f"{v1_lo * 100:.3f} % - {v1_hi * 100:.2f} % (fp64 {v1['fp64'] * 100:.2f}, w1 {v1['w1'] * 100:.2f}, "
                f"w2 {v1['w2'] * 100:.2f}, w3 {v1['w3'] * 100:.2f}, w4 {v1['w4'] * 100:.2f})",
    "definition": "median |err_calibrated| over the 6 test rows per field (batches >= 4096)",
    "files": [f"{RUN}/validation.json"],
    "status": rel(round(v1_lo * 100, 1) == 1.8 and round(v1_hi * 100, 1) == 5.5),
})

# V2: a priori model, all batches, fp64
v2_fp64 = statistics.median(abs(r["err_a_priori"]) for r in rows if r["field"] == "fp64")
claims.append({
    "claimed": "a priori model, fp64: median |error| 54.2 % (FAIL)",
    "computed": f"{v2_fp64 * 100:.2f} %",
    "definition": "median |err_a_priori| over all rows (test and fit) for fp64",
    "files": [f"{RUN}/validation.json"],
    "status": rel(round(v2_fp64 * 100, 1) == 54.2),
})

# a priori verdicts from the committed claims list
claims_list = val["claims"]
v2_pass = [c for c in claims_list if c["id"].startswith("V2.")]
n_pass = sum(1 for c in v2_pass if c["verdict"] == "PASS")
v1_pass = [c for c in claims_list if c["id"].startswith("V1.")]
n_v1_pass = sum(1 for c in v1_pass if c["verdict"] == "PASS")
claims.append({
    "claimed": "a priori model: 4/5 PASS (fp64 FAIL)",
    "computed": f"{n_pass}/{len(v2_pass)} PASS in committed claims list (V2.*)",
    "definition": "count of verdict PASS among V2.* entries of validation.json claims",
    "files": [f"{RUN}/validation.json"],
    "status": rel(n_pass == 4 and len(v2_pass) == 5),
})
claims.append({
    "claimed": "calibrated model: 5/5 PASS",
    "computed": f"{n_v1_pass}/{len(v1_pass)} PASS in committed claims list (V1.*)",
    "definition": "count of verdict PASS among V1.* entries",
    "files": [f"{RUN}/validation.json"],
    "status": rel(n_v1_pass == 5 and len(v1_pass) == 5),
})

# V3: size-extrapolated w4, all batches
v3_w4 = statistics.median(abs(r["err_size_extrapolated"]) for r in rows if r["field"] == "w4")
claims.append({
    "claimed": "size-extrapolated model to P-256 from 1-3 limbs: 7.4 %",
    "computed": f"{v3_w4 * 100:.2f} % (w4, all rows)",
    "definition": "median |err_size_extrapolated| over all w4 rows",
    "files": [f"{RUN}/validation.json"],
    "status": rel(round(v3_w4 * 100, 1) == 7.4),
})

# C0 clock agreement and clocks from raw clock samples
meas = json.load(open(f"{RUN}/measurements.json"))
clk = {}
for s in meas["samples"]:
    if s["group"] == "clock":
        clk.setdefault(s["arm"], []).append(s["value"])
add_med = statistics.median(clk["clock.add_chain"])
imul_med = statistics.median(clk["clock.imul_chain"])
c0 = abs(add_med - imul_med) / add_med
claims.append({
    "claimed": "measured clock 2.878 GHz (add chain) vs 2.797 GHz (imul chain)",
    "computed": f"add {add_med:.4f} GHz, imul {imul_med:.4f} GHz (median of raw clock samples, 7 reps); "
                f"validation.json clock_ghz {val['clock_ghz']:.4f}, clock_imul_ghz {val['clock_imul_ghz']:.4f}",
    "definition": "median of measurements.json clock.add_chain and clock.imul_chain samples",
    "files": [f"{RUN}/measurements.json", f"{RUN}/validation.json"],
    "status": rel(round(add_med, 3) == 2.878 and round(imul_med, 3) == 2.797),
})
claims.append({
    "claimed": "add-chain and imul-chain clocks agree: 2.8 %",
    "computed": f"{c0 * 100:.2f} % (|add - imul| / add, medians)",
    "definition": "relative difference of the two clock medians",
    "files": [f"{RUN}/measurements.json"],
    "status": rel(round(c0 * 100, 1) == 2.8),
})

# rho kernel ns/iter at B = 1024: measured kernel.w1.1024 and kernel.w4.1024 medians (7 reps, raw samples);
# the rho.csv column is the model's compute estimate and is reported separately in notes
kern = {}
for smp in meas["samples"]:
    if smp["group"] == "kernel":
        kern.setdefault(smp["arm"], []).append(smp["value"])
k_w1 = statistics.median(kern["kernel.w1.1024"])
k_w4 = statistics.median(kern["kernel.w4.1024"])
rho = list(csv.DictReader(open(f"{RUN}/rho.csv")))
model_w1 = {r["field"]: float(r["ns_per_iter_compute"]) for r in rho if r["batch"] == "1024"}
claims.append({
    "claimed": "rho kernel ns/iter at B=1024: 28.4 (w1) to 196 (w4)",
    "computed": (f"measured kernel.w1.1024 median {k_w1:.3f} ns, kernel.w4.1024 median {k_w4:.3f} ns "
                 f"(7 reps each); rho.csv model compute w1 {model_w1['w1']:.4f}, w4 {model_w1['w4']:.4f}"),
    "definition": "median over the 7 raw samples of kernel.w1.1024 and kernel.w4.1024 in measurements.json",
    "files": [f"{RUN}/measurements.json", f"{RUN}/rho.csv"],
    "status": rel(round(k_w1, 1) == 28.4 and round(k_w4) == 196),
})

# 4-thread efficiency range from validation.json scaling
eff = {f: val["scaling"][f]["4"] for f in val["scaling"]}
claims.append({
    "claimed": "4-thread efficiency 0.83-0.94",
    "computed": f"{min(eff.values()):.4f} - {max(eff.values()):.4f} ({', '.join(f'{k} {v:.4f}' for k, v in eff.items())})",
    "definition": "validation.json scaling[field]['4'] min and max",
    "files": [f"{RUN}/validation.json"],
    "status": rel(round(min(eff.values()), 2) == 0.83 and round(max(eff.values()), 2) == 0.94),
})

claims = claims[:10]

# identical: no arm-to-arm output comparison in this row's claims
offrepo = []
aa_lines = []
iso_lines = []
for root, _d, files in os.walk(STUDY):
    if os.sep + "runs" in root:
        continue
    for f in sorted(files):
        if f.endswith((".md", ".typ", ".txt")):
            p = os.path.join(root, f)
            with open(p, encoding="utf-8", errors="replace") as fh:
                for i, line in enumerate(fh, 1):
                    if OFFREPO_RE.search(line):
                        offrepo.append(f"{p}:{i}: {line.rstrip()}")
                    if AA_RE.search(line):
                        aa_lines.append(f"{p}:{i}: {line.rstrip()}")
                    if ISO_RE.search(line):
                        iso_lines.append(f"{p}:{i}: {line.rstrip()}")
for s in sorted(set(re.findall(r"store:([A-Za-z0-9_.\-]+)", " ".join(offrepo)))):
    listed = "no index found"
    for p in (f"{REPO}/store/index.json", f"{STUDY}/store/index.json"):
        if os.path.exists(p):
            listed = "yes" if s in open(p, encoding="utf-8", errors="replace").read() else "no"
            break
    offrepo.append(f"store:{s} listed in store/index.json: {listed}")


def git_add(path):
    out = subprocess.run(["git", "log", "--diff-filter=A", "--format=%H %cI", "--", path],
                         capture_output=True, text=True, cwd=REPO).stdout.strip()
    return out.split("\n")[-1].split(" ")[1] if out else ""


proto = git_add(f"{STUDY}/PROTOCOL.md")
raw_dates = sorted((git_add(p), p) for p in RAW_CANDIDATES if git_add(p))
first_raw = raw_dates[0] if raw_dates else ("", "")

out = {
    "row_no": ROW_NO,
    "slug": SLUG,
    "claims": claims,
    "identical": {
        "claimed": False,
        "n_compared": None,
        "n_equal": None,
        "n_differ": None,
        "files": [],
        "status": "not_claimed",
    },
    "offrepo_refs": offrepo,
    "aa_lines": aa_lines,
    "isolation_lines": iso_lines,
    "protocol_git_date": proto,
    "first_raw_git_date": first_raw[0],
    "script": SCRIPT,
    "notes": (
        "HEADLINE has no identical/equal-output claim; "
        f"PROTOCOL.md git add {proto}; first raw file by git add: {first_raw[1]}; "
        "claims 1-10 cover the V1, V2, V3, C0 values, clock and rho.csv rows"
    ),
}
print(json.dumps(out, ensure_ascii=False))
