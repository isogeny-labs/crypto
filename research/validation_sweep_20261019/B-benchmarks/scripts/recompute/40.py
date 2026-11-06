#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 40 (20261016-fixed-degree-isogenies).
# Reads only committed files under research/fixed_degree_isogenies_20261016. Prints one JSON object.
import json
import os
import re
import statistics
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
STUDY = "research/fixed_degree_isogenies_20261016"
REC = f"{STUDY}/data/records.jsonl"
ROW_NO = 40
SLUG = "20261016-fixed-degree-isogenies"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/40.py"
RAW_CANDIDATES = [f"{STUDY}/data/host.txt", f"{STUDY}/data/records.jsonl", f"{STUDY}/data/run_id.txt",
                  f"{STUDY}/data/stderr.log"]
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3|shared|L0")

rows = [json.loads(line) for line in open(REC, encoding="utf-8") if line.strip()]


def rel(ok):
    return "match" if ok else "mismatch"


claims = []
pipe = [r for r in rows if r["kind"] == "pipeline"]
planted = [r for r in pipe if r.get("planted") is True and r["solver"] == "Cornacchia"]
planted_ok = [r for r in planted if r["ok"]]
lp = [r["log_p_d"] for r in planted_ok]
claims.append({
    "claimed": "6/6 planted prime d found, log_p d 0.51-0.57 (Cornacchia, Kani-certified)",
    "computed": f"{len(planted_ok)}/{len(planted)} ok; log_p d {min(lp):.4f}-{max(lp):.4f}",
    "definition": "pipeline records, planted True, solver Cornacchia: count ok; min and max log_p_d",
    "files": [REC],
    "status": rel(len(planted_ok) == 6 and len(planted) == 6 and round(min(lp), 2) == 0.51 and round(max(lp), 2) == 0.57),
})

rand = [r for r in pipe if r.get("planted") is False]
rand_ok = [r for r in rand if r["ok"]]
ratios_dp = [r["d"] / r["p"] for r in rand]
claims.append({
    "claimed": "3/3 random targets with d = 8-12 p found (pairing-checked)",
    "computed": (f"{len(rand_ok)}/{len(rand)} ok; d/p {min(ratios_dp):.3f}-{max(ratios_dp):.3f}; "
                 f"pairing_ok {all(r.get('pairing_ok') for r in rand_ok)}"),
    "definition": "pipeline records, planted False: count ok; d/p min and max",
    "files": [REC],
    "status": rel(len(rand_ok) == 3 and len(rand) == 3 and min(ratios_dp) >= 8.0 and max(ratios_dp) <= 12.0),
})

cop = [r for r in pipe if r["solver"] == "CoppersmithBivariate" and r.get("planted") is True]
cop_ok = [r for r in cop if r["ok"]]
claims.append({
    "claimed": "Coron 0/2 at this small p",
    "computed": f"{len(cop_ok)}/{len(cop)} ok (errors: {[r.get('error') for r in cop]})",
    "definition": "pipeline records, solver CoppersmithBivariate: count ok",
    "files": [REC],
    "status": rel(len(cop_ok) == 0 and len(cop) == 2),
})

ok_tot = [r["total_ms"] for r in pipe if r["ok"]]
claims.append({
    "claimed": "1.9-3.9 s per instance",
    "computed": f"total_ms of ok pipeline instances {min(ok_tot) / 1000:.3f}-{max(ok_tot) / 1000:.3f} s ({len(ok_tot)} instances)",
    "definition": "total_ms over ok pipeline records",
    "files": [REC],
    "status": rel(round(min(ok_tot) / 1000, 1) == 1.9 and round(max(ok_tot) / 1000, 1) == 3.9),
})

share = [r["end_ring_ms"] / r["total_ms"] for r in pipe if r["ok"] and r.get("planted") is True and "end_ring_ms" in r]
claims.append({
    "claimed": "94-99 % of the pipeline time is step 1 (End(E)) (planted instances)",
    "computed": f"end_ring_ms / total_ms {min(share) * 100:.2f}-{max(share) * 100:.2f} % over {len(share)} instances",
    "definition": "end_ring_ms / total_ms for ok planted instances (random-target shares 82.6-86 % are not in this claim)",
    "files": [REC],
    "status": rel(round(min(share) * 100) == 94 and round(max(share) * 100) == 99),
})

s25 = [r for r in rows if r["kind"] == "steps25"]
def solver_stats(p_bits, solver):
    xs = [r for r in s25 if r["p_bits"] == p_bits and r["solver"] == solver]
    return xs, [r for r in xs if r["ok"]]


c61, c61_ok = solver_stats(61, "Cornacchia")
b61, b61_ok = solver_stats(61, "CoppersmithBivariate")
claims.append({
    "claimed": "steps 2-5 at 61 bits: Cornacchia and Coron 8/8 each, to log_p d 0.64",
    "computed": (f"Cornacchia {len(c61_ok)}/{len(c61)}, max log_p d {max(r['log_p_d'] for r in c61):.4f}; "
                 f"Coron {len(b61_ok)}/{len(b61)}, max log_p d {max(r['log_p_d'] for r in b61):.4f}"),
    "definition": "steps25 records, p_bits 61, solver Cornacchia and CoppersmithBivariate: count ok; max log_p_d",
    "files": [REC],
    "status": rel(len(c61_ok) == 8 and len(c61) == 8 and len(b61_ok) == 8 and len(b61) == 8
                  and round(max(r["log_p_d"] for r in c61), 2) == 0.64 and round(max(r["log_p_d"] for r in b61), 2) == 0.64),
})

t61, t61_ok = solver_stats(61, "CoppersmithTrivariate")
claims.append({
    "claimed": "trivariate 2/6 at 61 bits",
    "computed": f"{len(t61_ok)}/{len(t61)} ok",
    "definition": "steps25 records, p_bits 61, solver CoppersmithTrivariate: count ok",
    "files": [REC],
    "status": rel(len(t61_ok) == 2 and len(t61) == 6),
})

b127, b127_ok = solver_stats(127, "CoppersmithBivariate")
ms127 = [r["ms"] for r in b127_ok]
claims.append({
    "claimed": "127 bits Coron 6/6 in 0.04-4.3 s",
    "computed": f"{len(b127_ok)}/{len(b127)} ok; ms {min(ms127) / 1000:.4f}-{max(ms127) / 1000:.3f} s",
    "definition": "steps25 records, p_bits 127, solver CoppersmithBivariate: count ok; min and max ms",
    "files": [REC],
    "status": rel(len(b127_ok) == 6 and len(b127) == 6 and round(min(ms127) / 1000, 2) == 0.04
                  and round(max(ms127) / 1000, 1) == 4.3),
})

strass = {r["l"]: r["strassen_ms"] / r["naive_ms"] for r in rows if r["kind"] == "strassen"}
claims.append({
    "claimed": "Strassen 0.42x / 0.26x / 0.24x naive at l = 10^5 / 10^6 / 4*10^6",
    "computed": f"l=1e5 {strass[100000]:.4f}, l=1e6 {strass[1000000]:.4f}, l=4e6 {strass[4000000]:.4f}",
    "definition": "strassen_ms / naive_ms per l",
    "files": [REC],
    "status": rel(round(strass[100000], 2) == 0.42 and round(strass[1000000], 2) == 0.26
                  and round(strass[4000000], 2) == 0.24),
})

csurf = [r["ms"] for r in rows if r["kind"] == "csurf"]
claims.append({
    "claimed": "CSURF-512 action 25.8 ms",
    "computed": f"{csurf} ms",
    "definition": "csurf record ms",
    "files": [REC],
    "status": rel(len(csurf) == 1 and round(csurf[0], 1) == 25.8),
})

claims = claims[:10]

offrepo = []
aa_lines = []
iso_lines = []
for f in [f"{STUDY}/README.md", f"{STUDY}/data/host.txt", f"{STUDY}/data/run_id.txt"]:
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
    "first_raw_git_date": first_raw[0],
    "script": SCRIPT,
    "notes": (
        "no PROTOCOL file in the study directory; protocol_git_date empty. "
        "HEADLINE has no identical claim. Claim 'Coron faster than Cornacchia in 7 of 8 pairs' and "
        "'a priori' and 'Coron 6.5 s' values are not in the claim list (10-claim cap). "
        f"first raw file by git add date: {first_raw[1]}"
    ),
}
print(json.dumps(out, ensure_ascii=False))
