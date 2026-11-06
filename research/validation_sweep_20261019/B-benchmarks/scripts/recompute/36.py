#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 36 (20261016-hwcost-round2).
# Reads only committed files under research/hwcost_round2_20261016. Prints one JSON object.
import csv
import json
import os
import re
import statistics
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
STUDY = "research/hwcost_round2_20261016"
RUNS = f"{STUDY}/runs"
ROW_NO = 36
SLUG = "20261016-hwcost-round2"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/36.py"
RAW_CANDIDATES = [
    f"{RUNS}/cm-1/measurements.json",
    f"{RUNS}/cm-2/measurements.json",
    f"{RUNS}/isa-base-1/measurements.json",
    f"{RUNS}/isa-base-2/measurements.json",
    f"{RUNS}/isa-native-1/measurements.json",
    f"{RUNS}/isa-native-2/measurements.json",
]
TEXT_EXT = (".md", ".typ", ".txt", ".sh", ".py")
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3|no PMU")


def arm_medians(path):
    """median over reps of raw samples, per arm"""
    d = json.load(open(path))
    by = {}
    for s in d["samples"]:
        by.setdefault(s["arm"], []).append(s["value"])
    return {a: statistics.median(v) for a, v in by.items()}, len(d["samples"])


def med_csv(path):
    out = {}
    with open(path) as fh:
        for row in csv.DictReader(fh):
            out[row["arm"]] = float(row["median"])
    return out


def batch_of(arm, prefix):
    m = re.fullmatch(prefix + r"(\d+)", arm)
    return int(m.group(1)) if m else None


def rel_status(computed, claimed, nd=None):
    if abs(computed - claimed) <= 0.01 * abs(claimed):
        return "match"
    if nd is not None and round(computed, nd) == claimed:
        return "match"
    return "mismatch"


claims = []

# (a) native fp64 kernel: candidate / baseline per batch, both run pairs
native = []
for r in (1, 2):
    base, _ = arm_medians(f"{RUNS}/isa-base-{r}/measurements.json")
    nat, _ = arm_medians(f"{RUNS}/isa-native-{r}/measurements.json")
    chk = med_csv(f"{RUNS}/isa-base-{r}/medians.csv")
    for arm in base:
        if arm.startswith("kernel.fp64.") and not arm.startswith("kernel.fp64bf"):
            native.append((r, arm, nat[arm] / base[arm], abs(chk[arm] - base[arm]) / base[arm]))
ratios = [x[2] for x in native]
lo, hi = min(ratios), max(ratios)
lo_cell = min(native, key=lambda x: x[2])
hi_cell = max(native, key=lambda x: x[2])
native_files = [f"{RUNS}/isa-base-1/measurements.json", f"{RUNS}/isa-native-1/measurements.json",
                f"{RUNS}/isa-base-2/measurements.json", f"{RUNS}/isa-native-2/measurements.json"]
claims.append({
    "claimed": "native fp64 kernel ratio, lower end 0.38 (speed 1.6-2.6x faster)",
    "computed": f"{lo:.4f} (run {lo_cell[0]}, {lo_cell[1]})",
    "definition": "native/base median over 7 reps per arm, all kernel.fp64.<B> arms (not fp64bf), both run pairs",
    "files": native_files,
    "status": rel_status(lo, 0.38, 2),
})
claims.append({
    "claimed": "native fp64 kernel ratio, upper end 0.63",
    "computed": f"{hi:.4f} (run {hi_cell[0]}, {hi_cell[1]})",
    "definition": "as above, maximum",
    "files": native_files,
    "status": rel_status(hi, 0.63, 2),
})

# (b) Fp64Cm kernel / Fp64 kernel at B >= 4096, runs cm-1 and cm-2 (12 cells)
cm_cells = []
for run in ("cm-1", "cm-2"):
    med, _ = arm_medians(f"{RUNS}/{run}/measurements.json")
    for arm in med:
        b = batch_of(arm, "kernel.fp64cm.")
        if b is not None and b >= 4096:
            base_arm = f"kernel.fp64.{b}"
            if base_arm in med:
                cm_cells.append((run, b, med[arm] / med[base_arm]))
cm_vals = [x[2] for x in cm_cells]
cm_lo = min(cm_vals)
cm_hi = max(cm_vals)
cm_files = [f"{RUNS}/cm-1/measurements.json", f"{RUNS}/cm-2/measurements.json"]
claims.append({
    "claimed": "Fp64Cm kernel / Fp64 kernel at B >= 4096, lower end 0.40",
    "computed": f"{cm_lo:.4f} over {len(cm_vals)} cells",
    "definition": "fp64cm kernel median / fp64 kernel median over 7 reps, cells B >= 4096 in cm-1 and cm-2",
    "files": cm_files,
    "status": rel_status(cm_lo, 0.40, 2),
})
claims.append({
    "claimed": "Fp64Cm kernel / Fp64 kernel at B >= 4096, upper end 0.50 (F1' worst cell)",
    "computed": f"{cm_hi:.4f} (cells {len(cm_vals)}; worst = max)",
    "definition": "as above, maximum; committed runs/cm-pair/claims.json states F1prime value 0.4998",
    "files": cm_files,
    "status": rel_status(cm_hi, 0.50, 2),
})

# (c) GPU a priori and calibrated errors from the committed gpu_check.csv table
gpu = list(csv.DictReader(open(f"{RUNS}/gpu/gpu_check.csv")))
sep_same = [r for r in gpu if r["model"] == "separate" and r["set"] in ("same-gpu", "fit")]
sep_same_only = [r for r in gpu if r["model"] == "separate" and r["set"] == "same-gpu"]
sep_cross = [r for r in gpu if r["model"] == "separate" and r["set"] == "cross-gpu"]
g1 = statistics.median(abs(float(r["err_a_priori"])) for r in sep_same)
g2 = statistics.median(abs(float(r["err_calibrated"])) for r in sep_same_only)
g3 = statistics.median(abs(float(r["err_calibrated"])) for r in sep_cross)
gpu_file = [f"{RUNS}/gpu/gpu_check.csv"]
claims.append({
    "claimed": "GPU a priori median error 17.9 % over 46 builds",
    "computed": f"{g1 * 100:.2f} % (n={len(sep_same)} separate rows, same-gpu + fit)",
    "definition": "median |err_a_priori| over separate-model rows in set same-gpu or fit (committed derived table)",
    "files": gpu_file,
    "status": rel_status(g1 * 100, 17.9),
})
claims.append({
    "claimed": "GPU one eta per GPU median error 1.45 %",
    "computed": f"{g2 * 100:.2f} % (n={len(sep_same_only)} separate same-gpu rows)",
    "definition": "median |err_calibrated| over separate-model same-gpu rows (committed derived table)",
    "files": gpu_file,
    "status": rel_status(g2 * 100, 1.45),
})
claims.append({
    "claimed": "GPU eta carried across GPUs median error 19.5 %",
    "computed": f"{g3 * 100:.2f} % (n={len(sep_cross)} separate cross-gpu rows)",
    "definition": "median |err_calibrated| over separate-model cross-gpu rows (committed derived table)",
    "files": gpu_file,
    "status": rel_status(g3 * 100, 19.5),
})

# (d) IC vs rho ranking counts, from the committed pairs tables
def rank_counts(path):
    rows = list(csv.DictReader(open(path)))
    ranked = [r for r in rows if r["rank"] == "IC > rho"]
    return len(ranked), len(rows)


k_ic, k_n = rank_counts(f"{STUDY}/runs/ic-koblitz/pairs.csv")
p_ic, p_n = rank_counts(f"{STUDY}/runs/ic-prime/pairs.csv")
claims.append({
    "claimed": "IC > rho ranked on Koblitz curves: 4 of 8",
    "computed": f"{k_ic} of {k_n}",
    "definition": "count of rows with rank == 'IC > rho' in ic-koblitz/pairs.csv",
    "files": [f"{STUDY}/runs/ic-koblitz/pairs.csv"],
    "status": "match" if (k_ic, k_n) == (4, 8) else "mismatch",
})
claims.append({
    "claimed": "IC > rho ranked on prime curves: 5 of 6",
    "computed": f"{p_ic} of {p_n}",
    "definition": "count of rows with rank == 'IC > rho' in ic-prime/pairs.csv",
    "files": [f"{STUDY}/runs/ic-prime/pairs.csv"],
    "status": "match" if (p_ic, p_n) == (5, 6) else "mismatch",
})

# the claim about a priori 4-8 % for Fp64Cm (F2') is not derivable from committed records here
claims.append({
    "claimed": "Fp64Cm a priori model within 4-8 % (F2')",
    "computed": "not computed",
    "definition": "no committed a priori model prediction per cell for the Fp64Cm kernel was located in the runs "
                  "directories checked (cm-pair/claims.json holds only F1prime)",
    "files": [f"{RUNS}/cm-pair/claims.json"],
    "status": "cannot_define",
})

claims = claims[:10]

# IDENTICAL check: MEASURED (B) names "bit-identical branch-free variants" (Fp64Bf, Fp64Cm). The committed
# measurements.json and validation.json carry timing samples and fit rows, no output fields to compare.
identical = {
    "claimed": True,
    "n_compared": None,
    "n_equal": None,
    "n_differ": None,
    "files": [f"{RUNS}/cm-1/validation.json", f"{RUNS}/cm-1/measurements.json", f"{RUNS}/cm-2/measurements.json"],
    "status": "no_raw_in_repo",
}

offrepo = []
aa_lines = []
iso_lines = []
for root, _d, files in os.walk(STUDY):
    if "/runs/" in root + "/" and root.endswith("runs"):
        pass
    for f in sorted(files):
        if f.endswith(TEXT_EXT) or f.endswith(".txt"):
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


raw_dates = sorted((git_add(p), p) for p in RAW_CANDIDATES if git_add(p))
first_raw = raw_dates[0] if raw_dates else ("", "")
proto_path = f"{STUDY}/PROTOCOL.md"
proto = git_add(proto_path) if os.path.exists(proto_path) else ""

out = {
    "row_no": ROW_NO,
    "slug": SLUG,
    "claims": claims,
    "identical": identical,
    "offrepo_refs": offrepo,
    "aa_lines": aa_lines,
    "isolation_lines": iso_lines,
    "protocol_git_date": proto,
    "first_raw_git_date": first_raw[0],
    "script": SCRIPT,
    "notes": (
        f"PROTOCOL.md at {proto_path} exists={os.path.exists(proto_path)}; "
        f"native cells: {len(native)} (2 run pairs); Fp64Cm cells B>=4096: {len(cm_vals)}; "
        f"GPU: {len(gpu)} rows (model x set), derived committed table, not recomputed from profile JSON; "
        f"Fp64Cm F1prime worst from cm-pair/claims.json = 0.4998; "
        f"first raw file by git add date: {first_raw[1]}; "
        f"median vs medians.csv max relative gap across isa-base-1/2 arms: "
        f"{max(x[3] for x in native):.2e}"
    ),
}
print(json.dumps(out, ensure_ascii=False))
