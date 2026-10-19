#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 37 (20261016-isogeny-vs-flint-pari).
# Reads only committed files under isogeny_algos/results/p15-external and docs/external-20261016.
# Prints one JSON object.
import glob
import json
import os
import re
import statistics
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
RES = "isogeny_algos/results/p15-external"
DOC = "isogeny_algos/docs/external-20261016"
ROW_NO = 37
SLUG = "20261016-isogeny-vs-flint-pari"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/37.py"
RAW_CANDIDATES = [
    f"{RES}/isolation.jsonl",
    f"{RES}/round1/ours.poly.p511.jsonl",
    f"{RES}/round1/pari.poly.p511.jsonl",
    f"{RES}/round2/flint.poly.p511.jsonl",
    f"{RES}/round2/ours.poly.p511.jsonl",
    f"{RES}/round2/pari.poly.p511.jsonl",
]
TEXT_FILES = [f"{DOC}/REPORT.md", f"{RES}/SUMMARY.md"]
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3")


def role_of(fname):
    # ours.* -> isogeny_algos, flint.* -> FLINT, pari.* / pari-nosea.* -> PARI
    return fname.split(".")[0]


records = []  # (round, role, file, record)
for rd in ("round1", "round2"):
    for path in sorted(glob.glob(f"{RES}/{rd}/*.jsonl")):
        fname = os.path.basename(path)
        for line in open(path, encoding="utf-8"):
            if line.strip():
                records.append((rd, role_of(fname), f"{RES}/{rd}/{fname}", json.loads(line)))


def best_by_key(role_filter, op_filter, keyfn, valfn):
    """min over rounds of the per-record value, keyed by keyfn"""
    out = {}
    for rd, role, path, r in records:
        if role not in role_filter or r.get("op") not in op_filter:
            continue
        k = keyfn(r)
        v = valfn(r)
        if k not in out or v < out[k][0]:
            out[k] = (v, rd, path)
    return out


# multiplication and squaring, best_us = min over reps, min over rounds
mul_ours = best_by_key({"ours"}, {"mul", "sqr"}, lambda r: (r["op"], r["p"], r["n"]), lambda r: r["best_us"])
mul_flint = best_by_key({"flint"}, {"mul", "sqr"}, lambda r: (r["op"], r["p"], r["n"]), lambda r: r["best_us"])
mul_pari = best_by_key({"pari"}, {"mul", "sqr"}, lambda r: (r["op"], r["p"], r["n"]), lambda r: r["best_us"])
powx_ours = best_by_key({"ours"}, {"powx"}, lambda r: (r["p"], r["n"]), lambda r: r["best_us"])
powx_flint = best_by_key({"flint"}, {"powx"}, lambda r: (r["p"], r["n"]), lambda r: r["best_us"])
powx_pari = best_by_key({"pari"}, {"powx"}, lambda r: (r["p"], r["n"]), lambda r: r["best_us"])


def ratio_ours_over_fastest(ours, fl, pa, op, p, n):
    fastest = min(fl[(op, p, n)][0], pa[(op, p, n)][0])
    return ours[(op, p, n)][0] / fastest


def ours_over_flint(ours, fl, key):
    return ours[key][0] / fl[key][0]


def rng(vals):
    return min(vals), max(vals)


claims = []
files_mul = [f"{RES}/round1/ours.poly.p127.jsonl", f"{RES}/round2/ours.poly.p127.jsonl",
             f"{RES}/round1/flint.poly.p127.jsonl", f"{RES}/round2/flint.poly.p127.jsonl",
             f"{RES}/round1/pari.poly.p127.jsonl", f"{RES}/round2/pari.poly.p127.jsonl"]


def mul_range(p):
    keys = [k for k in mul_ours if k[0] == "mul" and k[1] == p and ("mul", p, k[2]) in mul_flint]
    vals = [ratio_ours_over_fastest(mul_ours, mul_flint, mul_pari, "mul", p, k[2]) for k in keys]
    return rng(vals), len(vals)


(lo127, hi127), n127 = mul_range("p127")
claims.append({
    "claimed": "products 127-bit ours / fastest external 0.42-0.95",
    "computed": f"{lo127:.4f} - {hi127:.4f} over {n127} mul cells (best over rounds, min of best_us)",
    "definition": "ours mul best_us / min(FLINT, PARI) mul best_us, per n, min and max",
    "files": files_mul,
    "status": "match" if round(lo127, 2) == 0.42 and round(hi127, 2) == 0.95 else "mismatch",
})
(lo255, hi255), n255 = mul_range("p255")
claims.append({
    "claimed": "products 255-bit ours / fastest external 0.59-1.07",
    "computed": f"{lo255:.4f} - {hi255:.4f} over {n255} mul cells",
    "definition": "as above for p255 mul",
    "files": [f"{RES}/round{r}/{t}.poly.p255.jsonl" for r in (1, 2) for t in ("ours", "flint", "pari")],
    "status": "match" if round(lo255, 2) == 0.59 and round(hi255, 2) == 1.07 else "mismatch",
})

def flint_faster_max(p, nlo, nhi):
    keys = [k for k in mul_ours if k[0] == "mul" and k[1] == p and nlo <= k[2] <= nhi and ("mul", p, k[2]) in mul_flint]
    vals = [ours_over_flint(mul_ours, mul_flint, ("mul", p, k[2])) for k in keys]
    return max(vals), sorted(k[2] for k in keys)


m61, ns61 = flint_faster_max("p61", 32, 2048)
claims.append({
    "claimed": "61-bit FLINT nmod_poly faster for n = 32-2048, to 2.5x",
    "computed": f"max ours/FLINT mul = {m61:.4f} over n {ns61[0]}..{ns61[-1]} ({len(ns61)} cells)",
    "definition": "max over n in [32,2048] of ours mul best_us / FLINT mul best_us",
    "files": [f"{RES}/round{r}/{t}.poly.p61.jsonl" for r in (1, 2) for t in ("ours", "flint")],
    "status": "match" if round(m61, 1) == 2.5 else "mismatch",
})
m511, ns511 = flint_faster_max("p511", 64, 2048)
claims.append({
    "claimed": "511-bit FLINT faster for n = 64-2048, to 2.3x",
    "computed": f"max ours/FLINT mul = {m511:.4f} over n {ns511[0]}..{ns511[-1]} ({len(ns511)} cells)",
    "definition": "max over n in [64,2048] of ours mul best_us / FLINT mul best_us",
    "files": [f"{RES}/round{r}/{t}.poly.p511.jsonl" for r in (1, 2) for t in ("ours", "flint")],
    "status": "match" if round(m511, 1) == 2.3 else "mismatch",
})


def powx_range(p):
    keys = [k for k in powx_ours if k[0] == p and (p, k[1]) in powx_flint]
    vals = [ours_over_flint(powx_ours, powx_flint, (p, k[1])) for k in keys]
    return rng(vals), len(vals)


(p61lo, p61hi), np61 = powx_range("p61")
claims.append({
    "claimed": "x^p mod g at 61 bits FLINT faster 1.65-2.1x",
    "computed": f"ours/FLINT {p61lo:.4f} - {p61hi:.4f} over {np61} powx cells",
    "definition": "ours powx best_us / FLINT powx best_us, min and max over n",
    "files": [f"{RES}/round{r}/{t}.powx.p61.jsonl" for r in (1, 2) for t in ("ours", "flint")],
    "status": "match" if round(p61lo, 2) == 1.65 and round(p61hi, 1) == 2.1 else "mismatch",
})
(p511lo, p511hi), np511 = powx_range("p511")
claims.append({
    "claimed": "x^p mod g at 511 bits FLINT faster 1.3-3.0x",
    "computed": f"ours/FLINT {p511lo:.4f} - {p511hi:.4f} over {np511} powx cells",
    "definition": "as above for p511",
    "files": [f"{RES}/round{r}/{t}.powx.p511.jsonl" for r in (1, 2) for t in ("ours", "flint")],
    "status": "match" if round(p511lo, 1) == 1.3 and round(p511hi, 1) == 3.0 else "mismatch",
})

# Phi_l totals: ours phi_all, PARI phi_spec_all (single j) and phi_full_all
phi_ours = min((r["total_us"], rd) for rd, role, _p, r in records if role == "ours" and r.get("op") == "phi_all")
phi_spec = min((r["total_us"], rd) for rd, role, _p, r in records if role == "pari" and r.get("op") == "phi_spec_all")
phi_full = min((r["total_us"], rd) for rd, role, _p, r in records if role == "pari" and r.get("op") == "phi_full_all")
claims.append({
    "claimed": "Phi_l l<=89 over 2^127-1: ours 2.24 s vs PARI 11.4 s (5.1x)",
    "computed": (f"ours {phi_ours[0] / 1000:.0f} ms; PARI phi_spec_all {phi_spec[0] / 1000:.0f} ms; "
                 f"PARI phi_full_all {phi_full[0] / 1000:.0f} ms; ratio spec/ours {phi_spec[0] / phi_ours[0]:.3f}"),
    "definition": "min over rounds of total_us: ours phi_all, PARI phi_spec_all (the 11.4 s row)",
    "files": [f"{RES}/round1/ours.phi.p127.jsonl", f"{RES}/round2/ours.phi.p127.jsonl",
              f"{RES}/round1/pari.phi.p127.jsonl", f"{RES}/round2/pari.phi.p127.jsonl"],
    "status": "match" if (round(phi_ours[0] / 1e6, 2) == 2.24 and round(phi_spec[0] / 1e3, 1) == 11.4) else "mismatch",
})


# SEA: per (p, k) min over rounds of 'us', summed over k
def sea_sum(role, p, rd_filter=None):
    per_k = {}
    for rd, rl, path, r in records:
        if rl == role and r.get("op") == "sea" and r["p"] == p:
            v = r["us"]
            if r["k"] not in per_k or v < per_k[r["k"]]:
                per_k[r["k"]] = v
    return sum(per_k.values()) / 1000.0, len(per_k)


ours61, n61 = sea_sum("ours", "p61")
ours127, n127s = sea_sum("ours", "p127")
pari61, _ = sea_sum("pari", "p61")
pari127, _ = sea_sum("pari", "p127")
claims.append({
    "claimed": "SEA p61: ours 41 ms vs PARI with seadata 91 ms",
    "computed": f"ours {ours61:.1f} ms vs PARI(seadata) {pari61:.1f} ms over {n61} curves",
    "definition": "per curve k, min over rounds of 'us', summed over k (ours and pari.sea.p61)",
    "files": [f"{RES}/round{r}/{t}.sea.p61.jsonl" for r in (1, 2) for t in ("ours", "pari")],
    "status": "match" if round(ours61) == 41 and round(pari61) == 91 else "mismatch",
})
claims.append({
    "claimed": "SEA p127: ours 1254 ms vs PARI with seadata 1062 ms (PARI 1.18x faster)",
    "computed": f"ours {ours127:.1f} ms vs PARI(seadata) {pari127:.1f} ms over {n127s} curves; ours/PARI {ours127 / pari127:.4f}",
    "definition": "as above for p127; ratio ours/PARI",
    "files": [f"{RES}/round{r}/{t}.sea.p127.jsonl" for r in (1, 2) for t in ("ours", "pari")],
    "status": "match" if round(ours127) == 1254 and round(pari127) == 1062 else "mismatch",
})

# identical outputs: digests (mul, sqr, powx), phi digest, SEA order; compare every record to ours
n_cmp = n_eq = n_diff = 0
diff_examples = []
ours_dig = {}
for rd, role, path, r in records:
    if role == "ours" and r.get("op") in ("mul", "sqr", "powx"):
        ours_dig.setdefault((r["op"], r["p"], r["n"]), set()).add(r["digest"])
for rd, role, path, r in records:
    if role in ("flint", "pari") and r.get("op") in ("mul", "sqr", "powx"):
        key = (r["op"], r["p"], r["n"])
        if key in ours_dig:
            n_cmp += 1
            if r["digest"] in ours_dig[key] and len(ours_dig[key]) == 1:
                n_eq += 1
            else:
                n_diff += 1
                diff_examples.append(path)
ours_phi = {}
for rd, role, path, r in records:
    if role == "ours" and r.get("op") == "phi_digest":
        ours_phi.setdefault((r["p"], r["ell"]), set()).add(r["digest"])
for rd, role, path, r in records:
    if role == "pari" and r.get("op") == "phi_digest":
        key = (r["p"], r["ell"])
        if key in ours_phi:
            n_cmp += 1
            if r["digest"] in ours_phi[key]:
                n_eq += 1
            else:
                n_diff += 1
                diff_examples.append(path)
ours_sea = {}
for rd, role, path, r in records:
    if role == "ours" and r.get("op") == "sea":
        ours_sea.setdefault((r["p"], r["k"]), set()).add(r["order"])
for rd, role, path, r in records:
    if role == "pari" and r.get("op") == "sea":
        key = (r["p"], r["k"])
        if key in ours_sea:
            n_cmp += 1
            if r["order"] in ours_sea[key]:
                n_eq += 1
            else:
                n_diff += 1
                diff_examples.append(path)
identical_files = sorted({os.path.relpath(p, REPO) if os.path.isabs(p) else p for _rd, _rl, p, _r in records})

offrepo = []
aa_lines = []
iso_lines = []
for f in TEXT_FILES + [f"{RES}/host.txt"]:
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
    for p in (f"{REPO}/store/index.json", f"{REPO}/{DOC}/store/index.json"):
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
proto = ""
out = {
    "row_no": ROW_NO,
    "slug": SLUG,
    "claims": claims,
    "identical": {
        "claimed": True,
        "n_compared": n_cmp,
        "n_equal": n_eq,
        "n_differ": n_diff,
        "files": identical_files,
        "status": "match" if n_diff == 0 and n_cmp > 0 else "mismatch",
    },
    "offrepo_refs": offrepo,
    "aa_lines": aa_lines,
    "isolation_lines": iso_lines,
    "protocol_git_date": proto,
    "first_raw_git_date": first_raw[0],
    "script": SCRIPT,
    "notes": (
        "no PROTOCOL file in the study directories checked, so protocol_git_date is empty; "
        "products claim uses mul cells only (sqr not in claim); "
        f"SEA per-curve rule = min over rounds of 'us', then sum over curves; "
        f"Phi uses min over rounds of total_us; differing digests: {len(diff_examples)}; "
        f"first raw file by git add date: {first_raw[1]}"
    ),
}
print(json.dumps(out, ensure_ascii=False))
