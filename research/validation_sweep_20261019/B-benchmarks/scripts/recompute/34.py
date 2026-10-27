#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 34 (20261016-j0-parallel-relations).
# The study directory holds only README.md; the per-rep timings exist only as a transcribed table
# in that README, so every claim is computed from that table and marked no_raw_in_repo.
# Reads only committed files. Prints one JSON object.
import json
import os
import re
import statistics
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
STUDY = "research/j0_parallel_relations_20261016"
README = f"{STUDY}/README.md"
ROW_NO = 34
SLUG = "20261016-j0-parallel-relations"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/34.py"
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3|shared")

text = open(README, encoding="utf-8").read()
table = {}
for line in text.splitlines():
    m = re.match(r"\|\s*(\d+)-bit, (\d+) threads?\s*\|\s*(baseline|candidate)\s*\|\s*([\d.]+) / ([\d.]+)\s*\|", line)
    if m:
        table[(m.group(1), m.group(2), m.group(3))] = [float(m.group(4)), float(m.group(5))]
    m = re.match(r"\|\s*(\d+)-bit, (\d+) threads?\s*\|\s*baseline / candidate\s*\|\s*([\d.]+) / ([\d.]+)\s*\|", line)
    if m:
        table[(m.group(1), m.group(2), "1t")] = [float(m.group(3)), float(m.group(4))]

b16 = statistics.median(table[("16", "14", "baseline")])
c16 = statistics.median(table[("16", "14", "candidate")])
b20 = statistics.median(table[("20", "14", "baseline")])
c20 = statistics.median(table[("20", "14", "candidate")])
t1_16 = table[("16", "1", "1t")]
t1_20 = table[("20", "1", "1t")]

files = [README]
claims = [
    {"claimed": "16-bit, 14 threads, relations stage baseline median ~1025 ms",
     "computed": f"{b16:.2f} ms (median of reps {table[('16', '14', 'baseline')]})",
     "definition": "median of the two rep values in the README table", "files": files,
     "status": "no_raw_in_repo"},
    {"claimed": "16-bit, 14 threads, candidate median ~234 ms",
     "computed": f"{c16:.2f} ms (median of reps {table[('16', '14', 'candidate')]})",
     "definition": "median of the two rep values in the README table", "files": files,
     "status": "no_raw_in_repo"},
    {"claimed": "16-bit relations stage speedup 4.4x",
     "computed": f"{b16 / c16:.3f}x", "definition": "median baseline / median candidate",
     "files": files, "status": "no_raw_in_repo"},
    {"claimed": "20-bit, 14 threads, baseline median ~39.6 s",
     "computed": f"{b20 / 1000:.3f} s", "definition": "median of rep values in the README table",
     "files": files, "status": "no_raw_in_repo"},
    {"claimed": "20-bit, 14 threads, candidate median ~13.2 s",
     "computed": f"{c20 / 1000:.3f} s", "definition": "median of rep values in the README table",
     "files": files, "status": "no_raw_in_repo"},
    {"claimed": "20-bit relations stage speedup 3.0x",
     "computed": f"{b20 / c20:.3f}x", "definition": "median baseline / median candidate",
     "files": files, "status": "no_raw_in_repo"},
    {"claimed": "1-thread 16-bit: +11% (within trial noise)",
     "computed": f"{(t1_16[1] / t1_16[0] - 1) * 100:+.1f} % (candidate/baseline, single rep each)",
     "definition": "README table 1-thread row, candidate over baseline", "files": files,
     "status": "no_raw_in_repo"},
    {"claimed": "1-thread 20-bit: -14% (within trial noise)",
     "computed": f"{(t1_20[1] / t1_20[0] - 1) * 100:+.1f} % (candidate/baseline, single rep each)",
     "definition": "README table 1-thread row, candidate over baseline", "files": files,
     "status": "no_raw_in_repo"},
]

offrepo = []
aa_lines = []
iso_lines = []
for i, line in enumerate(text.splitlines(), 1):
    if OFFREPO_RE.search(line):
        offrepo.append(f"{README}:{i}: {line.rstrip()}")
    if AA_RE.search(line):
        aa_lines.append(f"{README}:{i}: {line.rstrip()}")
    if ISO_RE.search(line):
        iso_lines.append(f"{README}:{i}: {line.rstrip()}")


def git_add(path):
    out = subprocess.run(["git", "log", "--diff-filter=A", "--format=%H %cI", "--", path],
                         capture_output=True, text=True, cwd=REPO).stdout.strip()
    return out.split("\n")[-1].split(" ")[1] if out else ""


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
    "protocol_git_date": "",
    "first_raw_git_date": "",
    "script": SCRIPT,
    "notes": (
        "study directory holds README.md only: no PROTOCOL file and no raw candidates (card list empty); "
        "README.md git add date " + git_add(README) + "; "
        "the per-rep values are transcribed in the README table, not raw records; "
        "HEADLINE claims 'every run recovered the planted scalar, rho agreement, 48 relations': "
        "no raw record in repo, not recomputed; no identical-output claim in HEADLINE or MEASURED"
    ),
}
print(json.dumps(out, ensure_ascii=False))
