#!/usr/bin/env python3
# SWB-RECOMPUTE-15, row 113 (20261012-conductor-gap-counts).
# Recomputes headline numbers from committed ecbench records using the rule in
# examples/conductor_gap_analyze.rs (splitmix64 Resampler, 20000 resamples, seed 20261012).
# Reads committed files only. Prints one JSON row object.
import json, math, os, re, subprocess

REPO = "/home/user/crypto"
STUDY_REL = "research/ecbench_conductor_gap_20261012"
STUDY = os.path.join(REPO, STUDY_REL)
MASK = (1 << 64) - 1
SAMPLES = 20000
SEED = 20261012


def splitmix64(x):
    x = (x + 0x9E3779B97F4A7C15) & MASK
    z = x
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return z ^ (z >> 31)


class Resampler:
    def __init__(self, seed):
        self.s = seed & MASK

    def below(self, n):
        self.s = splitmix64(self.s)
        return self.s % n


def mean(xs):
    t = 0.0
    for x in xs:
        t += x
    return t / len(xs)


def quantile(xs, q):
    v = sorted(xs)
    pos = min(max(q, 0.0), 1.0) * (len(v) - 1)
    lo, hi = math.floor(pos), math.ceil(pos)
    return v[lo] + (v[hi] - v[lo]) * (pos - lo)


def cross_curve(a, b):
    ratio = mean(b) / mean(a)
    rng = Resampler(SEED)
    boot = []
    for _ in range(SAMPLES):
        sa = 0.0
        for _ in range(len(a)):
            sa += a[rng.below(len(a))]
        sb = 0.0
        for _ in range(len(b)):
            sb += b[rng.below(len(b))]
        boot.append((sb / len(b)) / (sa / len(a)))
    pool = list(a) + list(b)
    observed = abs(math.log(ratio))
    extreme = 0
    for _ in range(SAMPLES):
        for i in range(len(pool) - 1, 0, -1):
            j = rng.below(i + 1)
            pool[i], pool[j] = pool[j], pool[i]
        ra = mean(pool[: len(a)])
        rb = mean(pool[len(a):])
        if abs(math.log(rb / ra)) + 1e-12 >= observed:
            extreme += 1
    return {
        "ratio": ratio,
        "ci95": [quantile(boot, 0.025), quantile(boot, 0.975)],
        "ci99": [quantile(boot, 0.005), quantile(boot, 0.995)],
        "perm_p": (extreme + 1) / (SAMPLES + 1),
    }


def load(path):
    out = []
    with open(path) as f:
        for line in f:
            if line.strip():
                out.append(json.loads(line))
    return out


def analyze_session(name):
    d = os.path.join(STUDY, "sessions", name)
    spec = json.load(open(os.path.join(d, "spec.json")))
    curves = spec["workloads"]["curves"]
    recs = load(os.path.join(d, "records.jsonl"))
    measured = [r for r in recs if not r["warmup"]]
    samples = []
    per_curve = []
    for ci, role in enumerate(["crater", "floor"]):
        sel = [r for r in measured if r["arm"] == "ic-mitm" and r["workload"]["curve_spec"] == curves[ci]]
        wl = {}
        for r in sel:
            if r["outcome"]["status"] == "verified":
                wl.setdefault(r["workload"]["workload_id"], []).append(r["cost"]["s"])
        vals = [mean(wl[k]) for k in sorted(wl)]  # BTreeMap order = sorted workload_id
        samples.append(vals)
        per_curve.append({"role": role, "n_measured": len(sel), "n_workloads": len(vals), "mean_s": mean(vals)})
    cc = cross_curve(samples[0], samples[1])
    # A/A check on rho-neg vs rho-neg-aa, exact cost/counters/recovered equality
    rho = {r["workload"]["workload_id"]: r for r in measured if r["arm"] == "rho-neg"}
    aa = {r["workload"]["workload_id"]: r for r in measured if r["arm"] == "rho-neg-aa"}
    aa_mismatch = [k for k in rho if k not in aa or aa[k]["cost"] != rho[k]["cost"]
                   or aa[k]["counters"] != rho[k]["counters"] or aa[k]["outcome"]["recovered"] != rho[k]["outcome"]["recovered"]]
    verified = sum(1 for r in recs if r["outcome"]["status"] == "verified")
    audit = json.load(open(os.path.join(STUDY, "sessions", name + ".audit.json")))
    replays = audit.get("replays", [])
    exact = sum(1 for x in replays if x.get("reproduced") is True and x.get("detail") == "identical")
    return {
        "name": name, "per_curve": per_curve, "cc": cc, "records": len(recs), "verified": verified,
        "replays_listed": len(replays), "replays_exact": exact, "aa_pairs": len(rho),
        "aa_mismatch": aa_mismatch, "audit_ok": audit.get("ok"),
        "spec_curves_first": curves[0]["a"],
    }


def git_date(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel],
                       capture_output=True, text=True)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else ""  # oldest add commit (log is newest first)


TEXT_EXT = (".md", ".json", ".log", ".txt", ".sh", ".typ", ".mjs", ".py", ".rs", ".svg", ".jsonl")


def study_text_lines():
    out = []
    for root, _, files in os.walk(STUDY):
        for fn in files:
            p = os.path.join(root, fn)
            if not fn.endswith(TEXT_EXT) or os.path.getsize(p) > 3_000_000:
                continue
            with open(p, encoding="utf-8", errors="ignore") as f:
                for i, line in enumerate(f, 1):
                    out.append((os.path.relpath(p, REPO), i, line.rstrip("\n")))
    return out


def within(claimed, computed, digits=None):
    if digits is not None and round(computed, digits) == claimed:
        return True
    return abs(computed - claimed) <= 0.01 * abs(claimed)


s8209 = analyze_session("ell8209-q29")
s131 = analyze_session("ell131101-q37")

claims = []


def ratio_claim(claimed_txt, claimed_val, s, digits=6):
    comp = s["cc"]["ratio"]
    st = "match" if within(claimed_val, comp, digits) else "mismatch"
    return {
        "claimed": claimed_txt,
        "computed": f"{comp:.6f}",
        "definition": "mean over 8 measured verified ic-mitm workloads at floor / same at crater (per-workload mean S, GAE/sqrt(r)); non-warmup records",
        "files": [f"{STUDY_REL}/sessions/{s['name']}/records.jsonl", f"{STUDY_REL}/sessions/{s['name']}/spec.json"],
        "status": st,
    }


def interval_claim(claimed_txt, lo_c, hi_c, s, key):
    lo, hi = s["cc"][key]
    st = "match" if within(lo_c, lo, 6) and within(hi_c, hi, 6) else "mismatch"
    return {
        "claimed": claimed_txt,
        "computed": f"{lo:.6f}–{hi:.6f}",
        "definition": f"{key}: percentile bootstrap of per-curve workload-mean S ratio (independent workloads, 20000 resamples, seed 20261012, splitmix64 Resampler as in examples/conductor_gap_analyze.rs)",
        "files": [f"{STUDY_REL}/sessions/{s['name']}/records.jsonl", "examples/conductor_gap_analyze.rs"],
        "status": st,
    }


claims.append(ratio_claim("0.975242 at ell = 8209", 0.975242, s8209))
claims.append(interval_claim("99% 0.603463–1.450934 at ell = 8209", 0.603463, 1.450934, s8209, "ci99"))
claims.append(ratio_claim("1.127134 at ell = 131101", 1.127134, s131))
claims.append(interval_claim("99% 0.922471–1.341694 at ell = 131101", 0.922471, 1.341694, s131, "ci99"))
total_verified = s8209["verified"] + s131["verified"]
claims.append({
    "claimed": "192 verified records",
    "computed": str(total_verified),
    "definition": "count of records with outcome.status == verified across both session records.jsonl files",
    "files": [f"{STUDY_REL}/sessions/ell8209-q29/records.jsonl", f"{STUDY_REL}/sessions/ell131101-q37/records.jsonl"],
    "status": "match" if total_verified == 192 else "mismatch",
})
total_exact = s8209["replays_exact"] + s131["replays_exact"]
claims.append({
    "claimed": "96 exact measured replays",
    "computed": str(total_exact),
    "definition": "count of replays in audit.json with reproduced==true and detail=='identical', summed over the two sessions",
    "files": [f"{STUDY_REL}/sessions/ell8209-q29.audit.json", f"{STUDY_REL}/sessions/ell131101-q37.audit.json"],
    "status": "match" if total_exact == 96 else "mismatch",
})

# Off-repo references (path:line verbatim)
PAT_OFF = re.compile(r"drive\.google|/Volumes|\bSSD\b|/Users/|s3://|gs://|store:|Modal|not committed|scratch", re.I)
offrepo = []
isolation = []
aa = []
PAT_ISO = re.compile(r"isolation|\bL[0-3]\b|isolab|NUMA|reserved CPU|CPU reservation", re.I)
PAT_AA = re.compile(r"A/A|aa-compare|mismatch", re.I)
for path, ln, text in study_text_lines():
    if PAT_OFF.search(text):
        offrepo.append(f"{path}:{ln}: {text.strip()[:300]}")
    if PAT_ISO.search(text) and path.endswith(".md"):
        isolation.append(f"{path}:{ln}: {text.strip()[:300]}")
    if PAT_AA.search(text) and path.endswith((".md", ".log")):
        aa.append(f"{path}:{ln}: {text.strip()[:300]}")

# store index check (no store: references found for this row, recorded for completeness)
store_checks = []
for m in re.finditer(r"store:([A-Za-z0-9_.-]+)", " ".join(o for o in offrepo)):
    idx = open(os.path.join(REPO, "store/index.json")).read()
    store_checks.append({"name": m.group(1), "in_index": m.group(1) in idx})

first_raw = f"{STUDY_REL}/sessions/ell131101-q37/records.jsonl"
proto = f"{STUDY_REL}/PROTOCOL.md"
row = {
    "row_no": 113,
    "slug": "20261012-conductor-gap-counts",
    "claims": claims,
    "identical": {
        "claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [],
        "status": "not_claimed",
    },
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": isolation,
    "protocol_git_date": git_date(proto),
    "first_raw_git_date": git_date(first_raw),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/113.py",
    "notes": json.dumps({
        "sessions": {s["name"]: {"per_curve": s["per_curve"], "ci95": s["cc"]["ci95"], "ci99": s["cc"]["ci99"],
                                  "perm_p": s["cc"]["perm_p"], "records": s["records"], "verified": s["verified"],
                                  "replays_listed": s["replays_listed"], "replays_exact": s["replays_exact"],
                                  "audit_ok": s["audit_ok"], "aa_pairs": s["aa_pairs"], "aa_mismatch_count": len(s["aa_mismatch"])}
                     for s in (s8209, s131)},
        "first_raw_choice": "first RAW CANDIDATE that is a per-run record file (sessions/ell131101-q37/records.jsonl); the list's analysis.json is derived output",
        "protocol_git_date_source": "git log --diff-filter=A on PROTOCOL.md (oldest add commit)",
        "store_checks": store_checks,
        "note_headline_no_identical_claim": True,
    }),
}
print(json.dumps(row, ensure_ascii=False))
