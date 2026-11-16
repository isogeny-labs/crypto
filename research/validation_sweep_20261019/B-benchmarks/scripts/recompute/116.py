#!/usr/bin/env python3
# SWB-RECOMPUTE-15, row 116 (20261011-inherited-f4-eager-rewrite).
# Reads committed raw probe records only. Prints one JSON row object.
import json, os, re, statistics, subprocess

REPO = "/home/user/crypto"
STUDY_REL = "research/inherited_f4_eager_rewrite_20261011"
STUDY = os.path.join(REPO, STUDY_REL)
RAW = os.path.join(STUDY, "reports/run-20261011T1657Z/raw")
RAW_REL = f"{STUDY_REL}/reports/run-20261011T1657Z/raw"
ROUNDS = [1, 2, 3]
CELLS = {"n23": "K0/2^23 dim 8", "n31": "K0/2^31 dim 9"}
# fields the report names as identical between modes (word_xors excluded: report says it rises)
CMP = ["hit", "exhausted", "reductions", "splits", "propagations", "infeasible", "geo_refute",
       "geo_witness", "partial_refute", "geo_adds", "pair_builds", "pair_lookups", "hits"]


def git_date(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel],
                       capture_output=True, text=True)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else ""


def records(cell, mode, rnd):
    out = []
    with open(os.path.join(RAW, f"{cell}_dim{'8' if cell == 'n23' else '9'}_{mode}.round{rnd}.jsonl")) as f:
        for line in f:
            s = line.strip()
            if s.startswith("{"):
                out.append(json.loads(s))
    return out


def fname(cell, mode, rnd):
    return f"{cell}_dim{'8' if cell == 'n23' else '9'}_{mode}.round{rnd}.jsonl"


arms_seen = set()
total = {}  # (cell, arm, mode) -> list of total_ms per round
cmp_n = cmp_eq = cmp_diff = 0
cmp_fields_seen = set()
diff_detail = []
for cell in CELLS:
    for rnd in ROUNDS:
        e0 = records(cell, "eager0", rnd)
        e1 = records(cell, "eager1", rnd)
        for mode, recs in (("eager0", e0), ("eager1", e1)):
            for r in recs:
                if "total_ms" in r:
                    arm = r["arm"]
                    arms_seen.add(arm)
                    total.setdefault((cell, arm, mode), []).append((rnd, r["total_ms"]))
        # pair records by position within the file (same target/total order in both modes)
        cmp0 = [r for r in e0 if ("target" in r or "total_ms" in r)]
        cmp1 = [r for r in e1 if ("target" in r or "total_ms" in r)]
        assert len(cmp0) == len(cmp1), (cell, rnd, len(cmp0), len(cmp1))
        for a, b in zip(cmp0, cmp1):
            assert a["arm"] == b["arm"] and a.get("target") == b.get("target")
            fields = [k for k in CMP if k in a or k in b]
            cmp_fields_seen.update(fields)
            cmp_n += 1
            diffs = [k for k in fields if a.get(k) != b.get(k)]
            if diffs:
                cmp_diff += 1
                diff_detail.append((cell, rnd, a["arm"], a.get("target"), diffs))
            else:
                cmp_eq += 1

inherit_arm = "f4"
fc_arm = sorted(a for a in arms_seen if a != "f4")
assert len(fc_arm) == 1, arms_seen
fc_arm = fc_arm[0]


def ratio(cell, arm):
    lazy = [t for _, t in sorted(total[(cell, arm, "eager0")])]
    eager = [t for _, t in sorted(total[(cell, arm, "eager1")])]
    assert len(lazy) == 3 and len(eager) == 3
    return statistics.median(lazy) / statistics.median(eager), statistics.median(lazy), statistics.median(eager), [l / e for l, e in zip(lazy, eager)]


R = {}
for cell in CELLS:
    for arm in (inherit_arm, fc_arm):
        R[(cell, arm)] = ratio(cell, arm)


def claim(claimed, cell, arm, target_val):
    rr, lm, em, per = R[(cell, arm)]
    return {
        "claimed": claimed,
        "computed": f"{rr:.4f} (median lazy {lm:.1f} ms / median eager {em:.1f} ms; per-round {', '.join(f'{x:.3f}' for x in per)})",
        "definition": f"median over 3 rounds of total_ms, lazy (KIC_F4_EAGER=0) / median over 3 rounds of total_ms, eager (default); cell {CELLS[cell]}, arm {'inherited F4' if arm == inherit_arm else 'F6-IC + index (raw arm ' + fc_arm + ')'}",
        "files": [f"{RAW_REL}/{fname(cell, 'eager0', r)}" for r in ROUNDS] + [f"{RAW_REL}/{fname(cell, 'eager1', r)}" for r in ROUNDS],
        "status": "match" if round(rr, 2) == target_val else "mismatch",
    }


claims = [
    claim("K0/2^23 dim 8 inherited F4 lazy/eager 1.40", "n23", inherit_arm, 1.40),
    claim("K0/2^23 dim 8 F6-IC + index lazy/eager 1.62", "n23", fc_arm, 1.62),
    claim("K0/2^31 dim 9 inherited F4 lazy/eager 1.48", "n31", inherit_arm, 1.48),
    claim("K0/2^31 dim 9 F6-IC + index lazy/eager 1.53", "n31", fc_arm, 1.53),
]

PAT_OFF = re.compile(r"drive\.google|/Volumes|\bSSD\b|/Users/|s3://|gs://|store:|Modal|not committed|scratch", re.I)
PAT_ISO = re.compile(r"isolation|\bL[0-3]\b|isolab|NUMA|reserved CPU|CPU reservation", re.I)
PAT_AA = re.compile(r"A/A|aa-compare", re.I)
offrepo, isolation, aa = [], [], []
for root, _, files in os.walk(STUDY):
    for fn in files:
        p = os.path.join(root, fn)
        if fn.endswith((".pdf", ".svg", ".png")) or os.path.getsize(p) > 3_000_000:
            continue
        rel = os.path.relpath(p, REPO)
        with open(p, encoding="utf-8", errors="ignore") as f:
            for i, line in enumerate(f, 1):
                t = line.rstrip("\n")
                if PAT_OFF.search(t):
                    offrepo.append(f"{rel}:{i}: {t.strip()[:300]}")
                if PAT_ISO.search(t) and fn.endswith((".md", ".typ")):
                    isolation.append(f"{rel}:{i}: {t.strip()[:300]}")
                if PAT_AA.search(t) and fn.endswith((".md", ".typ")):
                    aa.append(f"{rel}:{i}: {t.strip()[:300]}")

row = {
    "row_no": 116,
    "slug": "20261011-inherited-f4-eager-rewrite",
    "claims": claims,
    "identical": {
        "claimed": True,
        "n_compared": cmp_n,
        "n_equal": cmp_eq,
        "n_differ": cmp_diff,
        "files": [f"{RAW_REL}/{fname(c, m, r)}" for c in CELLS for m in ("eager0", "eager1") for r in ROUNDS],
        "status": "match" if cmp_diff == 0 else "mismatch",
    },
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": isolation,
    "protocol_git_date": git_date(f"{STUDY_REL}/README.md"),
    "first_raw_git_date": git_date(f"{RAW_REL}/n23_dim8_eager0.round1.jsonl"),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/116.py",
    "notes": json.dumps({
        "protocol_substitute": "no PROTOCOL file; README.md used for git date",
        "identical_fields_compared": sorted(cmp_fields_seen),
        "word_xors_excluded_from_identical_check": "report states word_xors rises by design",
        "differing_records": diff_detail,
        "arms_seen": sorted(arms_seen),
        "f6_arm_raw_name": fc_arm,
        "total_ms_by_cell_arm_mode": {f"{k[0]}|{k[1]}|{k[2]}": [round(t, 3) for _, t in sorted(v)] for k, v in total.items()},
    }, default=str),
}
print(json.dumps(row, ensure_ascii=False))
