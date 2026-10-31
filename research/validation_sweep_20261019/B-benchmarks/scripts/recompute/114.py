#!/usr/bin/env python3
# SWB-RECOMPUTE-15, row 114 (20261011-lit2026-w11-generic-engineering).
# Reads committed raw records only. Prints one JSON row object.
import json, os, re, subprocess, statistics

REPO = "/home/user/crypto"
STUDY_REL = "research/lit2026_w11_generic_engineering_20261011"
STUDY = os.path.join(REPO, STUDY_REL)
RAW = os.path.join(STUDY, "reports/run-20261011T2049Z/raw")
LABELS = ["secp40k1", "secp44k1", "secp48k1", "secp52k1", "secp56k1"]


def load(p):
    return [json.loads(l) for l in open(p) if l.strip()]


def git_date(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel],
                       capture_output=True, text=True)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else ""


# ---- operation counts (giant phase, per visited point) ----
op = load(os.path.join(RAW, "bsgs_opcount.jsonl"))


def pick_op(label, kernel, k):
    hits = [o for o in op if o["label"] == label and o["phase"] == "giant" and o["kernel"] == kernel and o["pair_k"] == k]
    assert len(hits) == 1, (label, kernel, k, len(hits))
    return hits[0]


def recomputed_m_eq(o):
    return o["mul_per_point"] + o["sqr_per_point"] + o["inv_per_point"] * (o["fermat_sqr"] + o["fermat_mul"])


chain48 = pick_op("secp48k1", "chain", 1)
paired48 = pick_op("secp48k1", "paired", 32)
recomp_check = {}
for o in op:
    recomp_check[f"{o['label']}/{o['phase']}/{o['kernel']}/K{o['pair_k']}"] = abs(recomputed_m_eq(o) - o["m_equiv_per_point"]) <= 1e-3 * max(1.0, o["m_equiv_per_point"])
recomp_fail = [k for k, v in recomp_check.items() if not v]

# ---- wall time: stated scope = BsgsFast::build + solve_many (total_s), ratio of medians, [min,max] of per-round ratios ----
tm = load(os.path.join(RAW, "bsgs_time.jsonl"))


def series(label, arm, field):
    rows = sorted([t for t in tm if t["label"] == label and t["arm"] == arm], key=lambda t: t["round"])
    return rows, [t[field] for t in rows]


ratios = {}
ratios_search = {}
for lab in LABELS:
    ro, vo = series(lab, "original", "total_s")
    rp, vp = series(lab, "paired", "total_s")
    assert [t["round"] for t in ro] == [t["round"] for t in rp] == [1, 2, 3, 4, 5]
    med = statistics.median(vo) / statistics.median(vp)
    per = [a / b for a, b in zip(vo, vp)]
    ratios[lab] = {"ratio_of_medians": med, "min": min(per), "max": max(per),
                   "median_original_s": statistics.median(vo), "median_paired_s": statistics.median(vp)}
    _, so = series(lab, "original", "search_s")
    _, sp = series(lab, "paired", "search_s")
    ratios_search[lab] = statistics.median(so) / statistics.median(sp)

rmed = {k: v["ratio_of_medians"] for k, v in ratios.items()}
lo_lab = min(rmed, key=rmed.get)
hi_lab = max(rmed, key=rmed.get)

claims = [
    {
        "claimed": "16.75 (chain, secp48k1, M-equivalents per visited point)",
        "computed": f"{chain48['m_equiv_per_point']:.4f}",
        "definition": "m_equiv_per_point stored field, giant phase, kernel chain, secp48k1 (recomputed mul_per_point+sqr_per_point+inv_per_point*(fermat_sqr+fermat_mul) = %.4f)" % recomputed_m_eq(chain48),
        "files": [f"{STUDY_REL}/reports/run-20261011T2049Z/raw/bsgs_opcount.jsonl"],
        "status": "match" if abs(chain48["m_equiv_per_point"] - 16.75) <= 0.01 * 16.75 else "mismatch",
    },
    {
        "claimed": "3.71 (paired K=32, secp48k1, M-equivalents per visited point)",
        "computed": f"{paired48['m_equiv_per_point']:.4f}",
        "definition": "m_equiv_per_point stored field, giant phase, kernel paired, pair_k 32, secp48k1 (recomputed = %.4f)" % recomputed_m_eq(paired48),
        "files": [f"{STUDY_REL}/reports/run-20261011T2049Z/raw/bsgs_opcount.jsonl"],
        "status": "match" if abs(paired48["m_equiv_per_point"] - 3.71) <= 0.01 * 3.71 else "mismatch",
    },
    {
        "claimed": "1.145 (lower end of original/paired wall ratios 1.145–1.670)",
        "computed": f"{rmed[lo_lab]:.4f} ({lo_lab}; per-round [{ratios[lo_lab]['min']:.3f}, {ratios[lo_lab]['max']:.3f}])",
        "definition": "min over the five sizes of ratio-of-medians original/paired, wall = total_s (BsgsFast::build + solve_many), 5 rounds; all sizes: " + ", ".join(f"{k}={v:.4f}" for k, v in rmed.items()),
        "files": [f"{STUDY_REL}/reports/run-20261011T2049Z/raw/bsgs_time.jsonl"],
        "status": "match" if abs(rmed[lo_lab] - 1.145) <= 0.01 * 1.145 else "mismatch",
    },
    {
        "claimed": "1.670 (upper end of original/paired wall ratios 1.145–1.670)",
        "computed": f"{rmed[hi_lab]:.4f} ({hi_lab}; per-round [{ratios[hi_lab]['min']:.3f}, {ratios[hi_lab]['max']:.3f}])",
        "definition": "max over the five sizes of ratio-of-medians original/paired, wall = total_s, 5 rounds",
        "files": [f"{STUDY_REL}/reports/run-20261011T2049Z/raw/bsgs_time.jsonl"],
        "status": "match" if abs(rmed[hi_lab] - 1.670) <= 0.01 * 1.670 else "mismatch",
    },
]

PAT_OFF = re.compile(r"drive\.google|/Volumes|\bSSD\b|/Users/|s3://|gs://|store:|Modal|not committed|scratch", re.I)
PAT_ISO = re.compile(r"isolation|\bL[0-3]\b|isolab|NUMA|reserved CPU|CPU reservation", re.I)
PAT_AA = re.compile(r"A/A|aa-compare|identical", re.I)
offrepo, isolation, aa = [], [], []
for root, _, files in os.walk(STUDY):
    for fn in files:
        p = os.path.join(root, fn)
        if not fn.endswith((".md", ".log", ".txt", ".json", ".jsonl", ".typ", ".py", ".sh", ".svg")) or os.path.getsize(p) > 3_000_000:
            continue
        rel = os.path.relpath(p, REPO)
        with open(p, encoding="utf-8", errors="ignore") as f:
            for i, line in enumerate(f, 1):
                t = line.rstrip("\n")
                if PAT_OFF.search(t):
                    offrepo.append(f"{rel}:{i}: {t.strip()[:300]}")
                if PAT_ISO.search(t) and fn.endswith(".md"):
                    isolation.append(f"{rel}:{i}: {t.strip()[:300]}")
                if PAT_AA.search(t) and fn.endswith(".md"):
                    aa.append(f"{rel}:{i}: {t.strip()[:300]}")

row = {
    "row_no": 114,
    "slug": "20261011-lit2026-w11-generic-engineering",
    "claims": claims,
    "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [],
                  "status": "not_claimed"},
    "offrepo_refs": sorted(offrepo),
    "aa_lines": aa,
    "isolation_lines": isolation,
    "protocol_git_date": git_date(f"{STUDY_REL}/README.md"),
    "first_raw_git_date": git_date(f"{STUDY_REL}/reports/run-20261011T2049Z/raw/binary_sect163.jsonl"),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/114.py",
    "notes": json.dumps({
        "protocol_substitute": "no PROTOCOL file exists for this study; README.md is the protocol-equivalent path used for the git date",
        "wall_scope_choice": "total_s used: README states scope 'BsgsFast::build + solve_many' and total_s = build_s + search_s in the records",
        "search_s_ratio_of_medians_for_reference": {k: round(v, 6) for k, v in ratios_search.items()},
        "ratio_of_medians": {k: {kk: round(vv, 6) for kk, vv in v.items()} for k, v in ratios.items()},
        "m_equiv_formula_check_failures": recomp_fail,
        "m_equiv_formula_checked_records": len(recomp_check),
        "giant_chain_fields": {k: chain48[k] for k in ["mul_per_point", "sqr_per_point", "inv_per_point", "fermat_sqr", "fermat_mul", "m_equiv_per_point"]},
        "giant_paired_k32_fields": {k: paired48[k] for k in ["mul_per_point", "sqr_per_point", "inv_per_point", "fermat_sqr", "fermat_mul", "m_equiv_per_point"]},
        "headline_no_identical_claim": True,
        "non_numeric_headline_items_not_recomputed": ["end-to-end binary class walk: no-change", "external comparison: not run"],
    }),
}
print(json.dumps(row, ensure_ascii=False))
