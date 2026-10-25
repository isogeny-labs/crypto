#!/usr/bin/env python3
# SWB-RECOMPUTE-15, row 115 (20261011-groebner-vs-msolve-singular-polybori).
# Reads committed raw records only. Prints one JSON row object.
import gzip, json, os, re, statistics, subprocess

REPO = "/home/user/crypto"
STUDY_REL = "research/gb_cas_comparison_20261011"
STUDY = os.path.join(REPO, STUDY_REL)
RUN = os.path.join(STUDY, "reports/run-20261011T1806Z")
RAW = os.path.join(RUN, "raw")
RAW_REL = f"{STUDY_REL}/reports/run-20261011T1806Z/raw"


def git_date(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel],
                       capture_output=True, text=True)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else ""


def jl(p):
    return [json.loads(l) for l in open(p) if l.strip()]


ROUNDS = [1, 2, 3]
# ---- F_p: ours (ms median per round) and external (wall ms per round) ----
ours_t1 = {}
for r in ROUNDS:
    for rec in jl(os.path.join(RAW, f"fp_ours_t1.round{r}.jsonl")):
        ours_t1.setdefault(rec["case"], []).append(rec["ours_f4_ms_median"])
ext = {}  # (case, tool) -> list of wall_ms per round
sing = {}  # case -> list of dict per round
for r in ROUNDS:
    for line in open(os.path.join(RAW, f"fp_external.round{r}.tsv")):
        f = line.rstrip("\n").split("\t")
        if len(f) < 3:
            continue
        case, tool = f[0], f[1]
        if tool in ("msolve_t1", "msolve_t4"):
            ext.setdefault((case, tool), []).append(float(f[3]))
        elif tool == "singular_std":
            kv = dict(zip(f[2].split()[0::2], f[2].split()[1::2]))
            sing.setdefault(case, []).append(kv)

cases = sorted(ours_t1)
ratio_t1 = {c: statistics.median(ours_t1[c]) / statistics.median(ext[(c, "msolve_t1")]) for c in cases}
ratio_t4 = {c: statistics.median(ours_t1[c]) / statistics.median(ext[(c, "msolve_t4")]) for c in cases if (c, "msolve_t4") in ext}
square = [c for c in cases if c.startswith(("quad_", "cubic_", "quart_"))]
sq_ratio = {c: ratio_t1[c] for c in square}
sq_min = min(sq_ratio, key=sq_ratio.get)
sq_max = max(sq_ratio, key=sq_ratio.get)

# Singular same-basis fields (values printed by the Singular check in the run)
ours_len_fp = {}
for r in ROUNDS:
    for rec in jl(os.path.join(RAW, f"fp_ours_t1.round{r}.jsonl")):
        ours_len_fp.setdefault(rec["case"], []).append((rec["basis_len"], rec["basis_fp"]))
n_cmp = n_eq = n_diff = 0
per_round_detail = []
for c in cases:
    for i, kv in enumerate(sing.get(c, [])):
        n_cmp += 1
        eq = kv.get("same_ideal") == "1" and kv.get("same_size") == "1" and int(kv["std_size"]) == int(kv["ours_size"]) == ours_len_fp[c][i][0]
        n_eq += eq
        n_diff += (not eq)
        per_round_detail.append((c, i + 1, kv.get("std_size"), kv.get("ours_size"), kv.get("same_ideal"), kv.get("same_size")))
basis_fp_stable = all(len({x[1] for x in ours_len_fp[c]}) == 1 for c in cases)

# ---- Boolean n = 17 ----
bool_ours = jl(os.path.join(RAW, "bool_n17_ours.jsonl"))
tg = [r for r in bool_ours if "target" in r]
ours_ms = [r["ms"] for r in tg]
ours_ms_f4 = [r["ms"] for r in tg if r["arm"] == "f4"]
ours_ms_f6 = [r["ms"] for r in tg if r["arm"] == "f6p"]

ext_b = {}
for line in open(os.path.join(RAW, "bool_n17_external.tsv")):
    f = line.rstrip("\n").split("\t")
    ext_b.setdefault(f[0], {})[f[1]] = f
ms_msolve = [float(ext_b[k]["msolve_t1"][3]) for k in sorted(ext_b)]
pb_ms = [float(re.search(r"polybori_ms ([0-9.]+)", " ".join(ext_b[k]["polybori"])).group(1)) for k in sorted(ext_b) if "polybori" in ext_b[k]]
pb_unit = {k: re.search(r"unit_ideal (\d)", " ".join(ext_b[k]["polybori"])).group(1) for k in sorted(ext_b) if "polybori" in ext_b[k]}
sing_rows = [(k, int(ext_b[k]["singular_std"][3]), ext_b[k]["singular_std"][5]) for k in sorted(ext_b) if "singular_std" in ext_b[k]]

# external verdict from msolve header: basis length 1 with constant 1 means unit ideal
msolve_unit = {}
for k in sorted(ext_b):
    with gzip.open(os.path.join(RAW, f"{k}.msolve_t1.out.gz"), "rt") as g:
        head = g.read(4000)
    L = int(re.search(r"length of basis:\s+(\d+)", head).group(1))
    msolve_unit[k] = (L == 1)
agree_pb = agree_ms = n_verdicts = 0
verdict_rows = []
for t in range(1, 9):
    key = f"root{t}"
    hit_f4 = [r["hit"] for r in tg if r["arm"] == "f4" and r["target"] == t]
    hit_f6 = [r["hit"] for r in tg if r["arm"] == "f6p" and r["target"] == t]
    ext_has_root_pb = pb_unit[key] == "0"
    ext_has_root_ms = not msolve_unit[key]
    for hit in hit_f4 + hit_f6:
        n_verdicts += 1
        agree_pb += (hit == ext_has_root_pb)
        agree_ms += (hit == ext_has_root_ms)
    verdict_rows.append((key, hit_f4, hit_f6, ext_has_root_pb, ext_has_root_ms))

claims = [
    {
        "claimed": "same reduced basis as Singular on 13/13 systems",
        "computed": f"{n_eq}/{n_cmp} Singular comparisons equal (same_ideal==1, same_size==1, std_size==ours_size==ours basis_len); cases {len(cases)}, rounds {len(ROUNDS)}",
        "definition": "fields same_ideal, same_size, std_size, ours_size from the Singular check lines in fp_external.roundN.tsv, plus ours basis_len from fp_ours_t1.roundN.jsonl; bases themselves not stored in repo (ours is basis_fp hash only)",
        "files": [f"{RAW_REL}/fp_external.round{r}.tsv" for r in ROUNDS] + [f"{RAW_REL}/fp_ours_t1.round{r}.jsonl" for r in ROUNDS],
        "status": "partial",
    },
    {
        "claimed": "msolve 1-thread 12–24× faster than ours on square generic systems",
        "computed": f"ours/msolve_t1 ratio of medians over square generic cases: min {sq_ratio[sq_min]:.2f} ({sq_min}), max {sq_ratio[sq_max]:.2f} ({sq_max}); REPORT table: quad_n8 12.2, quad_n9 23.1, cubic_n5 13.4 (computed quad_n8 {ratio_t1['quad_n8_p65521']:.2f}, quad_n9 {ratio_t1['quad_n9_p65521']:.2f}, cubic_n5 {ratio_t1['cubic_n5_p31']:.2f})",
        "definition": "median over 3 rounds of per-round ours_f4_ms_median (t1) divided by median over 3 rounds of msolve_t1 wall_ms; square generic = cases named quad_/cubic_/quart_",
        "files": [f"{RAW_REL}/fp_ours_t1.round{r}.jsonl" for r in ROUNDS] + [f"{RAW_REL}/fp_external.round{r}.tsv" for r in ROUNDS],
        "status": "match" if (12 <= sq_ratio[sq_min] <= 12.5 and 23.5 <= sq_ratio[sq_max] <= 24.5) else "mismatch",
    },
    {
        "claimed": "quad_n9: msolve 246 ms",
        "computed": f"{statistics.median(ext[('quad_n9_p65521', 'msolve_t1')]):.1f} ms",
        "definition": "median over 3 rounds of msolve_t1 wall_ms, quad_n9_p65521",
        "files": [f"{RAW_REL}/fp_external.round{r}.tsv" for r in ROUNDS],
        "status": "match" if abs(statistics.median(ext[('quad_n9_p65521', 'msolve_t1')]) - 246) <= 2.46 else "mismatch",
    },
    {
        "claimed": "quad_n9: ours 5 693 ms",
        "computed": f"{statistics.median(ours_t1['quad_n9_p65521']):.1f} ms",
        "definition": "median over 3 rounds of per-round ours_f4_ms_median (1 thread), quad_n9_p65521",
        "files": [f"{RAW_REL}/fp_ours_t1.round{r}.jsonl" for r in ROUNDS],
        "status": "match" if abs(statistics.median(ours_t1['quad_n9_p65521']) - 5693) <= 56.93 else "mismatch",
    },
    {
        "claimed": "ours decides in 7–24 ms per target (Boolean n = 17, 8 targets)",
        "computed": f"min {min(ours_ms):.3f} ms, max {max(ours_ms):.3f} ms over {len(ours_ms)} records (inherited F4 {min(ours_ms_f4):.3f}–{max(ours_ms_f4):.3f}; F6-IC {min(ours_ms_f6):.3f}–{max(ours_ms_f6):.3f})",
        "definition": "min and max of field ms over bool_n17_ours.jsonl records that carry a target field (one measured value per arm and target)",
        "files": [f"{RAW_REL}/bool_n17_ours.jsonl"],
        "status": "match" if round(min(ours_ms)) == 7 and round(max(ours_ms)) == 24 else "mismatch",
    },
    {
        "claimed": "msolve 42.7–46.5 s for the full reduced basis (Boolean)",
        "computed": f"min {min(ms_msolve)/1000:.3f} s, max {max(ms_msolve)/1000:.3f} s over {len(ms_msolve)} roots (msolve_t1 wall_ms)",
        "definition": "min and max of wall_ms with tool msolve_t1 in bool_n17_external.tsv, one run per root",
        "files": [f"{RAW_REL}/bool_n17_external.tsv"],
        "status": "match" if abs(min(ms_msolve)/1000 - 42.7) <= 0.427 and abs(max(ms_msolve)/1000 - 46.5) <= 0.465 else "mismatch",
    },
    {
        "claimed": "PolyBoRi 40.4–52.3 s for the full reduced basis (Boolean)",
        "computed": f"min {min(pb_ms)/1000:.3f} s, max {max(pb_ms)/1000:.3f} s over {len(pb_ms)} roots (polybori_ms)",
        "definition": "min and max of polybori_ms in bool_n17_external.tsv",
        "files": [f"{RAW_REL}/bool_n17_external.tsv"],
        "status": "match" if abs(min(pb_ms)/1000 - 40.4) <= 0.404 and abs(max(pb_ms)/1000 - 52.3) <= 0.523 else "mismatch",
    },
    {
        "claimed": "Singular does not finish in 120 s (Boolean)",
        "computed": f"{sum(1 for _, ms, st in sing_rows if st.startswith('exit124') and ms >= 120000)}/{len(sing_rows)} roots with status exit124 and wall_ms >= 120000; min wall_ms {min(ms for _, ms, _ in sing_rows)}",
        "definition": "count of singular_std rows with status exit124 (timeout) and wall_ms >= 120000 in bool_n17_external.tsv",
        "files": [f"{RAW_REL}/bool_n17_external.tsv"],
        "status": "match" if all(st.startswith("exit124") and ms >= 120000 for _, ms, st in sing_rows) and len(sing_rows) == 8 else "mismatch",
    },
    {
        "claimed": "All verdicts agree (Boolean n = 17, 8 targets)",
        "computed": f"ours hit vs external has-root: agree vs polybori {agree_pb}/{n_verdicts}, vs msolve {agree_ms}/{n_verdicts} (ours records per arm and target)",
        "definition": "ours verdict = hit field; external has-root = polybori unit_ideal==0 or msolve basis length != 1; target t mapped to root t (index mapping not stated in raw files)",
        "files": [f"{RAW_REL}/bool_n17_ours.jsonl", f"{RAW_REL}/bool_n17_external.tsv"] + [f"{RAW_REL}/{k}.msolve_t1.out.gz" for k in sorted(ext_b)],
        "status": "partial" if agree_pb == n_verdicts and agree_ms == n_verdicts else "mismatch",
    },
]

PAT_OFF = re.compile(r"drive\.google|/Volumes|\bSSD\b|/Users/|s3://|gs://|store:|Modal|not committed|scratch", re.I)
PAT_ISO = re.compile(r"isolation|\bL[0-3]\b|isolab|NUMA|reserved CPU|CPU reservation", re.I)
PAT_AA = re.compile(r"A/A|aa-compare", re.I)
offrepo, isolation, aa = [], [], []
for root, _, files in os.walk(STUDY):
    for fn in files:
        p = os.path.join(root, fn)
        if fn.endswith(".gz") or os.path.getsize(p) > 3_000_000:
            continue
        rel = os.path.relpath(p, REPO)
        try:
            with open(p, encoding="utf-8", errors="ignore") as f:
                for i, line in enumerate(f, 1):
                    t = line.rstrip("\n")
                    if PAT_OFF.search(t):
                        offrepo.append(f"{rel}:{i}: {t.strip()[:300]}")
                    if PAT_ISO.search(t) and fn.endswith((".md", ".typ")):
                        isolation.append(f"{rel}:{i}: {t.strip()[:300]}")
                    if PAT_AA.search(t) and fn.endswith((".md", ".typ")):
                        aa.append(f"{rel}:{i}: {t.strip()[:300]}")
        except OSError:
            pass

row = {
    "row_no": 115,
    "slug": "20261011-groebner-vs-msolve-singular-polybori",
    "claims": claims,
    "identical": {
        "claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_diff,
        "files": [f"{RAW_REL}/fp_external.round{r}.tsv" for r in ROUNDS] + [f"{RAW_REL}/fp_ours_t1.round{r}.jsonl" for r in ROUNDS],
        "status": "partial",
    },
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": isolation,
    "protocol_git_date": git_date(f"{STUDY_REL}/README.md"),
    "first_raw_git_date": git_date(f"{RAW_REL}/cubic_n5_p31.msolve_t1.out.gz"),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/115.py",
    "notes": json.dumps({
        "protocol_substitute": "no PROTOCOL file; README.md used for git date",
        "ratio_t4_ours_over_msolve_t4": {k: round(v, 3) for k, v in ratio_t4.items()},
        "ratio_t1_all_cases": {k: round(v, 3) for k, v in ratio_t1.items()},
        "singular_per_case_round": per_round_detail,
        "ours_basis_fp_stable_across_rounds": basis_fp_stable,
        "boolean_verdict_rows": verdict_rows,
        "msolve_unit_ideal": msolve_unit,
        "polybori_unit_ideal": pb_unit,
        "singular_rows": sing_rows,
        "magma": "not run (README and REPORT.typ state Magma not installed)",
    }, default=str),
}
print(json.dumps(row, ensure_ascii=False))
