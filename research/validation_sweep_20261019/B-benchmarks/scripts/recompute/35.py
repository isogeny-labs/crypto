#!/usr/bin/env python3
# SWB-RECOMPUTE-05, row 35 (20261016-strong-rho-assurance).
# Reads only committed files under research/rho_assurance_20261016. Prints one JSON object.
import glob
import hashlib
import json
import math
import os
import re
import statistics
import subprocess

REPO = "/home/user/crypto"
os.chdir(REPO)
STUDY = "research/rho_assurance_20261016"
ROW_NO = 35
SLUG = "20261016-strong-rho-assurance"
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/35.py"
Q2 = {
    41: f"{STUDY}/runs/q2/icv1-f2m41-tm2308219-7f48b14a.jsonl",
    53: f"{STUDY}/runs/q2/icv1-f2m53-tm56619371-dac20a85.jsonl",
    61: f"{STUDY}/runs/q2/icv1-f2m61-t158598901-ab42b6c5.jsonl",
}
CALLGRIND = {a: f"{STUDY}/runs/q4/callgrind/fixture-n53-4-{a}" for a in ("baseline", "candidate", "candidate2")}
TIME_GLOB = f"{STUDY}/runs/q4/time/*/*.jsonl"
ISO = f"{STUDY}/runs/q4/time/ab/isolation.jsonl"
RAW_CANDIDATES = [
    Q2[41], Q2[53], Q2[61],
    f"{STUDY}/runs/q4/time/ab/isolation.jsonl",
    f"{STUDY}/summary.json",
    f"{STUDY}/tla/out/wit_step_capped.txt",
]
TEXT_EXT = (".md", ".typ", ".txt", ".sh")
OFFREPO_RE = re.compile(
    r"drive\.google\.com|/Volumes|\bSSD\b|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|"
    r"not committed|scratch|Modal volume"
)
AA_RE = re.compile(r"A/A|null control|null-control")
ISO_RE = re.compile(r"isolat|PMU|contend|CPU-seconds|--cpus|CPU 3")


def load_jsonl(path):
    with open(path, encoding="utf-8") as fh:
        return [json.loads(line) for line in fh if line.strip()]


def mean(xs):
    return sum(xs) / len(xs)


def sd(xs):
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1))


def ci_upper(xs):
    return mean(xs) + 1.96 * sd(xs) / math.sqrt(len(xs))


def rel_status(computed, claimed):
    return "match" if abs(computed - claimed) <= 0.01 * abs(claimed) else "mismatch"


def collected(path):
    txt = open(path, encoding="utf-8", errors="replace").read()
    vals = re.findall(r"Collected : (\d+)", txt)
    assert len(vals) == 1, (path, vals)
    return int(vals[0])


claims = []
q2_stats = {}
for n, path in Q2.items():
    recs = load_jsonl(path)
    r = [x["walk_steps"] / x["ideal_steps"] for x in recs]
    q2_stats[n] = {"mean": mean(r), "ci_upper": ci_upper(r), "count": len(r)}

means_claim = {41: 0.994, 53: 0.922, 61: 0.943}
ci_claim = {41: 1.044, 53: 0.971, 61: 0.993}
for n in (41, 53, 61):
    claims.append({
        "claimed": f"steps/ideal mean at n={n}: {means_claim[n]}",
        "computed": f"{q2_stats[n]['mean']:.4f} (count {q2_stats[n]['count']})",
        "definition": "mean over fixtures of walk_steps / ideal_steps, raw q2 jsonl (summarise.sh ratio)",
        "files": [Q2[n]],
        "status": rel_status(q2_stats[n]["mean"], means_claim[n]),
    })
for n in (41, 53, 61):
    claims.append({
        "claimed": f"95% CI upper of steps/ideal at n={n}: {ci_claim[n]}",
        "computed": f"{q2_stats[n]['ci_upper']:.4f}",
        "definition": "mean + 1.96 * sample sd / sqrt(n) of walk_steps / ideal_steps (normal approximation, as summarise.sh)",
        "files": [Q2[n]],
        "status": rel_status(q2_stats[n]["ci_upper"], ci_claim[n]),
    })

# instructions per walk step, callgrind fixture n53-4
ir_per_step = {}
for arm, stem in CALLGRIND.items():
    recs = load_jsonl(stem + ".jsonl")
    steps = sum(x["walk_steps"] for x in recs)
    ir = collected(stem + ".valgrind.log")
    ir_per_step[arm] = {"ir": ir, "steps": steps, "per_step": ir / steps}
base_ps = ir_per_step["baseline"]["per_step"]
cand2_ps = ir_per_step["candidate2"]["per_step"]
drop = (cand2_ps / base_ps - 1) * 100
claims.append({
    "claimed": "1,196 -> 938 instructions per step (-21.5 %)",
    "computed": f"{base_ps:.2f} -> {cand2_ps:.2f} ({drop:.2f} %)",
    "definition": "valgrind 'Collected' instruction count / sum walk_steps, baseline and candidate2 callgrind fixture n53-4",
    "files": [CALLGRIND["baseline"] + ".valgrind.log", CALLGRIND["candidate2"] + ".valgrind.log"],
    "status": "match" if (round(base_ps) == 1196 and round(cand2_ps) == 938 and abs(drop + 21.5) <= 0.215) else "mismatch",
})

# native time cells, contended runs excluded (isolation.jsonl run.contended)
iso_rows = load_jsonl(ISO)
contended = {x["label"] for x in iso_rows if x.get("run", {}).get("contended") is True}
cells = {}
for path in sorted(glob.glob(TIME_GLOB)):
    if os.path.basename(path) == "isolation.jsonl":
        continue
    phase = os.path.basename(os.path.dirname(path))
    label = "q4/" + phase + "/" + os.path.basename(path)[: -len(".jsonl")]
    if label in contended:
        continue
    fname = os.path.basename(path)
    arm = re.search(r"-(baseline|candidate)-r", fname).group(1)
    slug = re.match(r"(icv1-[^-]+-[^-]+-[0-9a-f]{8})", fname).group(1)
    recs = load_jsonl(path)
    ns = sum(x["walk_ms"] for x in recs) * 1e6 / sum(x["walk_steps"] for x in recs)
    cells.setdefault(slug, {"aa": [], "baseline": [], "candidate": []})
    key = "aa" if phase == "aa" else arm
    cells[slug][key].append(ns)

native = {}
for slug, c in cells.items():
    native[slug] = {
        "ratio": statistics.median(c["baseline"]) / statistics.median(c["candidate"]),
        "aa_spread": max(c["aa"]) / min(c["aa"]) - 1,
        "n_base": len(c["baseline"]),
        "n_cand": len(c["candidate"]),
    }
slug53 = "icv1-f2m53-tm56619371-dac20a85"
slug61 = "icv1-f2m61-t158598901-ab42b6c5"
claims.append({
    "claimed": "native 1.023x at n53",
    "computed": f"{native[slug53]['ratio']:.4f} (n baseline {native[slug53]['n_base']}, candidate {native[slug53]['n_cand']})",
    "definition": "median of per-run ns/step (sum walk_ms*1e6/sum walk_steps), non-contended ab runs: baseline median / candidate median",
    "files": [f"{STUDY}/runs/q4/time/ab/{slug53}-{a}-r{i}.jsonl" for a in ("baseline", "candidate") for i in range(1, 6)] + [ISO],
    "status": rel_status(native[slug53]["ratio"], 1.023),
})
claims.append({
    "claimed": "native 1.209x at n61",
    "computed": f"{native[slug61]['ratio']:.4f} (n baseline {native[slug61]['n_base']}, candidate {native[slug61]['n_cand']})",
    "definition": "as above, n=61 cell",
    "files": [f"{STUDY}/runs/q4/time/ab/{slug61}-{a}-r{i}.jsonl" for a in ("baseline", "candidate") for i in range(1, 6)] + [ISO],
    "status": rel_status(native[slug61]["ratio"], 1.209),
})
claims.append({
    "claimed": "A/A 10.7 % (n53) and 18.3 % (n61), not established (A/A spread)",
    "computed": f"n53 {native[slug53]['aa_spread'] * 100:.2f} %; n61 {native[slug61]['aa_spread'] * 100:.2f} %",
    "definition": "max/min - 1 of per-run ns/step over the aa phase files, non-contended",
    "files": sorted(glob.glob(f"{STUDY}/runs/q4/time/aa/*.jsonl")) + [ISO],
    "status": "match" if (abs(native[slug53]["aa_spread"] * 100 - 10.7) <= 0.107
                          and abs(native[slug61]["aa_spread"] * 100 - 18.3) <= 0.183) else "mismatch",
})

claims = claims[:10]

# identical outputs: callgrind fixture records across the three arms (timing and RSS keys removed)
def strip(rec):
    return {k: v for k, v in rec.items() if not k.endswith("_ms") and k != "peak_rss_bytes"}


base_recs = load_jsonl(CALLGRIND["baseline"] + ".jsonl")
n_compared = n_equal = n_differ = 0
files_cmp = [CALLGRIND["baseline"] + ".jsonl"]
for arm in ("candidate", "candidate2"):
    other = load_jsonl(CALLGRIND[arm] + ".jsonl")
    files_cmp.append(CALLGRIND[arm] + ".jsonl")
    assert len(other) == len(base_recs)
    for a, b in zip(base_recs, other):
        n_compared += 1
        if strip(a) == strip(b):
            n_equal += 1
        else:
            n_differ += 1
# output digests as the summarise.sh identity (sha256 of compact JSON, first 16 hex)
digests = {}
for arm, stem in CALLGRIND.items():
    recs = load_jsonl(stem + ".jsonl")
    digests[arm] = hashlib.sha256(
        "\n".join(json.dumps(strip(r), sort_keys=True, separators=(",", ":")) for r in recs).encode()
    ).hexdigest()[:16]

offrepo = []
for root, _d, files in os.walk(STUDY):
    for f in sorted(files):
        if f.endswith(TEXT_EXT):
            p = os.path.join(root, f)
            with open(p, encoding="utf-8", errors="replace") as fh:
                for i, line in enumerate(fh, 1):
                    if OFFREPO_RE.search(line):
                        offrepo.append(f"{p}:{i}: {line.rstrip()}")
for s in sorted(set(re.findall(r"store:([A-Za-z0-9_.\-]+)", " ".join(offrepo)))):
    listed = "no index found"
    for p in (f"{REPO}/store/index.json", f"{STUDY}/store/index.json"):
        if os.path.exists(p):
            listed = "yes" if s in open(p, encoding="utf-8", errors="replace").read() else "no"
            break
    offrepo.append(f"store:{s} listed in store/index.json: {listed}")

aa_lines = []
iso_lines = []
for root, _d, files in os.walk(STUDY):
    for f in sorted(files):
        if f.endswith(TEXT_EXT):
            p = os.path.join(root, f)
            with open(p, encoding="utf-8", errors="replace") as fh:
                for i, line in enumerate(fh, 1):
                    if AA_RE.search(line):
                        aa_lines.append(f"{p}:{i}: {line.rstrip()}")
                    if ISO_RE.search(line):
                        iso_lines.append(f"{p}:{i}: {line.rstrip()}")


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
        "claimed": True,
        "n_compared": n_compared,
        "n_equal": n_equal,
        "n_differ": n_differ,
        "files": files_cmp,
        "status": "match" if n_differ == 0 and n_compared > 0 else "mismatch",
    },
    "offrepo_refs": offrepo,
    "aa_lines": aa_lines,
    "isolation_lines": iso_lines,
    "protocol_git_date": proto,
    "first_raw_git_date": first_raw[0],
    "script": SCRIPT,
    "notes": (
        f"identical compares the {len(base_recs)} callgrind fixture records per arm (baseline vs candidate, "
        f"baseline vs candidate2) with *_ms and peak_rss_bytes removed; sha256 identity prefixes "
        f"{digests}; time-cell identity not recomputed beyond the three q4 records; "
        f"contended excluded: {sorted(contended)}; first raw file by git add date: {first_raw[1]}; "
        f"per-cell native ns/step: " + ", ".join(
            f"{k}: baseline median/candidate median={v['ratio']:.4f}" for k, v in native.items())
        + "; the 1.209x and 1.023x values are the summary cells; A/A values established by these runs only"
    ),
}
print(json.dumps(out, ensure_ascii=False))
