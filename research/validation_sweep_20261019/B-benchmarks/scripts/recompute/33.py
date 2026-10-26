#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 33 (20261016-endo-native-bench).
# Reads only committed files. Prints one JSON object (one JSONL row) to stdout.
import glob
import json
import os
import re
import statistics
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
STUDY = "research/endo_native_bench_20261016"
ROW_NO = 33
SLUG = "20261016-endo-native-bench"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/33.py"

# (display name, timing stem, declared GLV arm key, headline counted ratio, headline median ratio,
#  headline A/A median ratio)
CURVES = [
    ("Bandersnatch", "bandersnatch", "glv/0+1w/2/optimised/jacobian/w5", "1.525", "1.504", "0.997"),
    ("Tom-384", "tom384", "glv/4+1w/7.5.5/optimised/jacobian/w5", "1.521", "1.504", "1.001"),
    ("CP6-782", "cp6_782", "glv/4+1w/7.3.5/optimised/jacobian/w5", "1.492", "1.570", "1.038"),
]
RAW_CANDIDATES = [
    f"{STUDY}/bandersnatch.timing.json",
    f"{STUDY}/cp6_782.timing.json",
    f"{STUDY}/frozen_inputs/cp6_782.chains.constants.json",
    f"{STUDY}/frozen_inputs/cp6_782.model.json",
    f"{STUDY}/frozen_inputs/tom384.chains.constants.json",
    f"{STUDY}/tom384.timing.json",
]
TEXT_EXT = (".md", ".typ", ".txt")
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3")


def text_files():
    out = []
    for root, _dirs, files in os.walk(STUDY):
        for f in files:
            if f.endswith(TEXT_EXT):
                out.append(os.path.join(root, f))
    return sorted(out)


def grep_lines(rx):
    hits = []
    for path in text_files():
        with open(path, encoding="utf-8", errors="replace") as fh:
            for i, line in enumerate(fh, 1):
                if rx.search(line):
                    hits.append(f"{path}:{i}: {line.rstrip()}")
    return hits


def git_add_date(path):
    out = subprocess.run(
        ["git", "log", "--diff-filter=A", "--format=%H %cI", "--", path],
        capture_output=True, text=True, cwd=REPO,
    ).stdout.strip()
    if not out:
        return ""
    return out.split("\n")[-1]  # earliest add commit


def rel_status(computed, claimed):
    c = float(computed)
    k = float(claimed)
    if abs(c - k) <= 0.01 * abs(k):
        return "match"
    return "mismatch"


def load_arms(stem):
    d = json.load(open(f"{STUDY}/{stem}.timing.json"))
    arms = {}
    for a in d["arms"]:
        r = a["ns_per_op_per_round"]
        arms[a["key"]] = {
            "cls": a["class"],
            "med": statistics.median(r),
            "mn": min(r),
            "stored_med": a["median_ns_per_op"],
            "m_eq": a["counted"]["m_eq"],
            "model": a["model_m_eq"],
            "rounds": len(r),
        }
    return arms


claims = []
notes = []
for name, stem, declared, c_counted, c_median, c_aa in CURVES:
    arms = load_arms(stem)
    files = [f"{STUDY}/{stem}.timing.json"]
    base = {k: v for k, v in arms.items() if v["cls"] in ("baseline variant", "reference")}
    glv = {k: v for k, v in arms.items() if v["cls"] == "engineering" and k.startswith("glv/")}
    ref = [k for k, v in arms.items() if v["cls"] == "reference"]
    aa = [k for k, v in arms.items() if v["cls"] == "A/A control"]
    assert declared in arms, declared
    bb_c = min(base, key=lambda k: base[k]["m_eq"])
    bg_c = min(glv, key=lambda k: glv[k]["m_eq"])
    counted_ratio = base[bb_c]["m_eq"] / glv[bg_c]["m_eq"]
    fb = min(base, key=lambda k: base[k]["med"])
    med_ratio = base[fb]["med"] / arms[declared]["med"]
    aa_ratio = arms[ref[0]]["med"] / arms[aa[0]]["med"]
    # per-arm counted vs model
    notes.append(
        f"{name}: best baseline counted={bb_c} ({base[bb_c]['m_eq']:.2f}); "
        f"best GLV counted={bg_c} ({glv[bg_c]['m_eq']:.2f}); "
        f"fastest baseline by median={fb} ({base[fb]['med']:.0f} ns); "
        f"declared={declared} median={arms[declared]['med']:.0f} ns; "
        f"reference={ref[0]}; A/A={aa[0]}"
    )
    claims.append({
        "claimed": f"{name} counted ratio best baseline / best GLV-2: {c_counted}",
        "computed": f"{counted_ratio:.4f}",
        "definition": "min over baseline arms (class baseline variant or reference) of counted m_eq, "
                      "divided by min over engineering glv/* arms of counted m_eq",
        "files": files,
        "status": rel_status(f"{counted_ratio:.4f}", c_counted),
    })
    claims.append({
        "claimed": f"{name} declared GLV vs fastest baseline, median ratio: {c_median}",
        "computed": f"{med_ratio:.4f}",
        "definition": "median recomputed from ns_per_op_per_round (statistics.median); "
                      "fastest baseline = baseline arm with min recomputed median; "
                      "ratio = fastest baseline median / declared arm median",
        "files": files,
        "status": rel_status(f"{med_ratio:.4f}", c_median),
    })
    claims.append({
        "claimed": f"{name} A/A median ratio: {c_aa}",
        "computed": f"{aa_ratio:.4f}",
        "definition": "reference arm median / A/A control arm median (recomputed from per-round times)",
        "files": files,
        "status": rel_status(f"{aa_ratio:.4f}", c_aa),
    })

# worst total arm deviation from the model, all curves, counted m_eq vs model_m_eq
worst = (0.0, None)
for name, stem, *_rest in CURVES:
    arms = load_arms(stem)
    for k, v in arms.items():
        if v["cls"] in ("baseline variant", "reference", "engineering"):
            dev = v["m_eq"] / v["model"] - 1.0
            if abs(dev) > abs(worst[0]):
                worst = (dev, f"{stem}:{k}")
claims.append({
    "claimed": "every total arm within 0.31 % of the model",
    "computed": f"max |counted M_eq / model - 1| = {abs(worst[0]) * 100:.2f} % at {worst[1]}",
    "definition": "over all baseline and GLV total arms of the three timing files, counted m_eq vs model_m_eq",
    "files": [f"{STUDY}/{s}.timing.json" for _n, s, *_r in CURVES],
    "status": "match" if abs(abs(worst[0]) * 100 - 0.31) <= 0.01 * 0.31 or round(abs(worst[0]) * 100, 2) <= 0.31 else "mismatch",
})

# the first ten claims are the cap; the list above is 3 x 3 + 1 = 10
claims = claims[:10]

offrepo = []
for hit in grep_lines(OFFREPO_RE):
    offrepo.append(hit)
store_names = sorted(set(re.findall(r"store:([A-Za-z0-9_.\-]+)", " ".join(offrepo))))
for s in store_names:
    idx_paths = [f"{REPO}/store/index.json", f"{STUDY}/store/index.json"]
    listed = "no index found"
    for p in idx_paths:
        if os.path.exists(p):
            listed = "yes" if s in open(p, encoding="utf-8", errors="replace").read() else "no"
            break
    offrepo.append(f"store:{s} listed in store/index.json: {listed}")

aa_lines = grep_lines(AA_RE)
iso_lines = grep_lines(ISO_RE)

raw_dates = []
for p in RAW_CANDIDATES:
    d = git_add_date(p)
    if d:
        raw_dates.append((d.split(" ")[1], p, d))
raw_dates.sort()
proto_date = git_add_date(f"{STUDY}/PROTOCOL.md").split(" ")[-1]
first_raw = raw_dates[0] if raw_dates else ("", "", "")

notes.append(
    "identical: HEADLINE contains no identical/bit-identical claim between arms; "
    "'equal to the model' statements are model comparisons, not arm-to-arm, so not checked"
)
notes.append(
    f"claims cap: first 10 of 3 curves x (counted ratio, median ratio, A/A median ratio) plus worst-deviation; "
    "min ratios (1.528/1.530/1.520) and model ratios (1.523/1.524/1.494) are in HEADLINE but not in the 10-claim cap"
)
notes.append(f"first raw file by git add date: {first_raw[1]}")

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
    "protocol_git_date": proto_date if proto_date else "",
    "first_raw_git_date": first_raw[0],
    "script": SCRIPT,
    "notes": " | ".join(notes),
}
print(json.dumps(out, ensure_ascii=False))
