#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 89 (20261014-secg-small-x-s3-cost).
# Reads committed raw records only. Prints computed values, then appends one JSON line
# to recompute/SWB-RECOMPUTE-12.jsonl (static evidence fields are copied from the study files by grep).
import glob
import json
import os

REPO = "/home/user/crypto"
DATA = os.path.join(REPO, "research/secg_small_x_20261014/data")
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/89.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/89.py"


def load(path):
    with open(path) as fh:
        return json.load(fh)


def s3_rep(path):
    return load(path)["report"]


def rel_ok(claimed, computed, decimals):
    if round(computed, decimals) == claimed:
        return True
    return abs(computed - claimed) <= 0.01 * abs(claimed)


def make_claim(claimed_str, pairs, definition, files):
    # pairs: list of (claimed_number, computed_number, decimals)
    ok = all(rel_ok(c, v, d) for c, v, d in pairs)
    computed = ", ".join((f"{v:.{d}f}" if d >= 0 else f"{v:.6g}") for _, v, d in pairs)
    return {
        "claimed": claimed_str,
        "computed": computed,
        "definition": definition,
        "files": files,
        "status": "match" if ok else "mismatch",
    }


# 33 SECG curves at B = 4096 (one S3 run per curve and size).
files_4096 = sorted(glob.glob(os.path.join(DATA, "s3_*_4096.json")))
files_1024 = sorted(glob.glob(os.path.join(DATA, "s3_*_1024.json")))
rows_4096 = {}
for p in files_4096:
    name = os.path.basename(p)[len("s3_"):-len("_4096.json")]
    rows_4096[name] = s3_rep(p)

print("count s3 files B=4096:", len(files_4096))
print("count s3 files B=1024:", len(files_1024))
print("count s3 runs total:", len(files_4096) + len(files_1024))

def rel(name):
    return "research/secg_small_x_20261014/data/s3_%s_4096.json" % name

def s3_ms(name):
    return rows_4096[name]["s3"]["s3_us_per_trial"] / 1000.0

def table_us(name):
    return rows_4096[name]["s3"]["table_ns_per_trial"] / 1000.0

def ratio(name):
    return rows_4096[name]["s3"]["s3_us_per_trial"] / (rows_4096[name]["s3"]["table_ns_per_trial"] / 1000.0)

def bits(name):
    return rows_4096[name]["s3"]["bits_slower_than_rho"]

names = sorted(rows_4096)
print("curves at 4096:", len(names))

claims = []
claims.append(make_claim(
    "S3 per target at B = 4096: 5.4 ms sect113r1",
    [(5.4, s3_ms("sect113r1"), 1)],
    "s3.s3_us_per_trial / 1000 (ms per target), B=4096",
    [rel("sect113r1")],
))
claims.append(make_claim(
    "9.6 ms sect163r2",
    [(9.6, s3_ms("sect163r2"), 1)],
    "s3.s3_us_per_trial / 1000 (ms per target), B=4096",
    [rel("sect163r2")],
))
claims.append(make_claim(
    "42 ms secp160r1",
    [(42.0, s3_ms("secp160r1"), 0)],
    "s3.s3_us_per_trial / 1000 (ms per target), B=4096",
    [rel("secp160r1")],
))
tab = [table_us(n) for n in names]
claims.append(make_claim(
    "table lookup 2 to 5 us",
    [(2.0, min(tab), 0), (5.0, max(tab), 0)],
    "min and max over the 33 curves at B=4096 of s3.table_ns_per_trial / 1000 (us per target)",
    [rel(n) for n in names],
))
rat = [ratio(n) for n in names]
claims.append(make_claim(
    "S3 is 2600 to 313 000 times dearer than the table lookup",
    [(2600.0, min(rat), -2), (313000.0, max(rat), -3)],
    "min and max over the 33 curves at B=4096 of s3.s3_us_per_trial / s3.table_ns_per_trial * 1000 (ratio per target)",
    [rel(n) for n in names],
))
claims.append(make_claim(
    "B + 1 relations by S3 solves are 52 bits slower than rho (sect113r1)",
    [(52.0, bits("sect113r1"), 0)],
    "s3.bits_slower_than_rho, B=4096",
    [rel("sect113r1")],
))
claims.append(make_claim(
    "77 bits slower (sect163r2)",
    [(77.0, bits("sect163r2"), 0)],
    "s3.bits_slower_than_rho, B=4096",
    [rel("sect163r2")],
))
claims.append(make_claim(
    "79 bits slower (secp160r1)",
    [(79.0, bits("secp160r1"), 0)],
    "s3.bits_slower_than_rho, B=4096",
    [rel("secp160r1")],
))
bl = [bits(n) for n in names]
claims.append(make_claim(
    "52 to 280 bits over the 33 curves",
    [(52.0, min(bl), 0), (280.0, max(bl), 0)],
    "min and max over the 33 curves at B=4096 of s3.bits_slower_than_rho",
    [rel(n) for n in names],
))
claims.append(make_claim(
    "66 runs (count of S3 run records)",
    [(66.0, float(len(files_4096) + len(files_1024)), 0)],
    "count of data/s3_*_4096.json plus data/s3_*_1024.json files",
    [os.path.relpath(p, REPO) for p in files_4096 + files_1024],
))

# Identical-output claim: decision of S3 equals the pair-sum table.
n_compared = 0
n_differ = 0
for p in files_4096 + files_1024:
    s3 = load(p)["report"]["s3"]
    n_compared += int(s3["trials"])
    n_differ += int(s3["disagreements"])
n_equal = n_compared - n_differ

# Toy-field check: look for any S3 decision field in toy records.
toy_files = sorted(glob.glob(os.path.join(DATA, "toy*.json")))
toy_with_s3 = 0
toy_disagree = 0
for p in toy_files:
    rep = load(p).get("report", {})
    if "s3" in rep:
        toy_with_s3 += 1
        toy_disagree += int(rep["s3"].get("disagreements", 0))

print("identical: n_compared", n_compared, "n_equal", n_equal, "n_differ", n_differ)
print("toy files:", len(toy_files), "with s3 field:", toy_with_s3, "toy disagreements:", toy_disagree)
for c in claims:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

identical_files = [os.path.relpath(p, REPO) for p in files_4096 + files_1024]
identical_status = "match" if (n_differ == 0 and n_compared > 0) else "mismatch"
if toy_with_s3 == 0:
    # the toy-field part of the identical claim has no raw S3 field in repo
    identical_status = "partial" if identical_status == "match" else identical_status
    toy_note = "toy-field decision check: no toy record in data/ carries an s3 field (toy files: %d); not verifiable from these records" % len(toy_files)
else:
    toy_note = "toy-field decision check: %d toy records with s3 field, %d disagreements" % (toy_with_s3, toy_disagree)

row = {
    "row_no": 89,
    "slug": "20261014-secg-small-x-s3-cost",
    "claims": claims,
    "identical": {
        "claimed": True,
        "n_compared": n_compared,
        "n_equal": n_equal,
        "n_differ": n_differ,
        "files": identical_files,
        "status": identical_status,
    },
    "offrepo_refs": [],
    "aa_lines": [
        "research/secg_small_x_20261014/README.md:25: \"Monte Carlo finds 0 hits in 91.6 million trials, as the yields\"",
        "research/secg_small_x_20261014/REPORT.typ:88: \"Monte Carlo: 0 hits in every run. Planted controls detected and recovered in every run.\"",
    ],
    "isolation_lines": [
        "research/secg_small_x_20261014/README.md:66: \"S₃ times: 20 targets per run, one thread, single runs on a shared machine; rho charged √n affine additions.\"",
        "research/secg_small_x_20261014/REPORT.typ:353: \"The S₃ times are 20 targets per run, one thread, single runs on a shared 4-vCPU machine, at B = 1024 and 4096\"",
    ],
    "protocol_git_date": "2026-10-17T10:41:58-07:00 (commit 62c2e7d5b6d38c59d3693276d5c46ccecbf429c2, research/secg_small_x_20261014/README.md; the study has no separate PROTOCOL file)",
    "first_raw_git_date": "2026-10-17T10:41:58-07:00 (same commit 62c2e7d5b6d38c59d3693276d5c46ccecbf429c2; earliest-added file under data/ in that commit)",
    "script": SCRIPT,
    "notes": (
        "Study date in the row is 2026-10-14; the study README and data/ were first added in commit 62c2e7d5 dated 2026-10-17. "
        "The 4096 S3 records carry one run per curve (trials=20 targets each). "
        "Claimed table-lookup range and ratio range are computed over the 33 curves at B=4096. "
        "Off-repo grep (drive.google, /Volumes, SSD, /Users, s3://, gs://, store:, Modal volume, not committed, scratch) matched nothing in README.md, REPORT.typ, data/COMMANDS.txt. "
        + toy_note + "."
    ),
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("appended row 89 to", OUT)
