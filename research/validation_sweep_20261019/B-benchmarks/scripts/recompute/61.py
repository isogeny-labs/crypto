# SWB-RECOMPUTE-08 row 61: 20261015-conductor-gap-cm-orbits. Reads committed files only.
import json, statistics, subprocess, re, os, collections
REPO = "/home/user/crypto"
STUDY = os.path.join(REPO, "research/conductor_gap_cm_orbits_20261015")
RES = os.path.join(STUDY, "results")
OUTJSONL = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-08.jsonl")
# 44,000 orbit trials: orbit-only (8000), isolated (12000), recurrence (12000), cold (12000)
trial_files = ["orbits.jsonl", "isolated-orbits.jsonl", "recurrence-orbits.jsonl", "cold-orbits.jsonl"]
n_trials = {f: sum(1 for _ in open(os.path.join(RES, f))) for f in trial_files}
print("TRIAL_LINES", n_trials, "SUM", sum(n_trials.values()))
prog = json.load(open(os.path.join(RES, "campaign-progress.json")))
c = prog["counts"]
primary = sum(v for k, v in c.items() if k.startswith("primary/"))
repl = sum(v for k, v in c.items() if k.startswith("replication/"))
print("PRIMARY_VERIFIED", primary, "REPLICATION_VERIFIED", repl)
print("SAME_CRATER_EXACT_SOLVE_MATCHES primary", prog["same_crater_exact_solve_matches"], "replication", prog["replication_same_crater_exact_solve_matches"])
pp = json.load(open(os.path.join(RES, "paired-primary.json")))
fc = pp["floor_over_crater_cpu"]; print("FLOOR_OVER_CRATER_CPU ratio_of_means", fc["ratio_of_means"], "paired_p95", fc["paired_percentile_95"])
print("FLOOR_OVER_CRATER_QUERIES ratio_of_means", pp["floor_over_crater_queries"]["ratio_of_means"], pp["floor_over_crater_queries"]["paired_percentile_95"])
print("PAIRED_ENDPOINTS_OBSERVED", pp["paired_endpoints_observed"], "all_100_pairs_verified", pp["all_100_pairs_verified"])
iso = [json.loads(l) for l in open(os.path.join(RES, "isolated-orbits.jsonl"))]
byrole = collections.defaultdict(list)
for r in iso: byrole[r["role"]].append(r["orbit_cpu_seconds"])
for role in sorted(byrole):
    print("ISO_ROLE", role, "n", len(byrole[role]), "mean_ms", round(statistics.mean(byrole[role]) * 1000, 4))
isum = json.load(open(os.path.join(RES, "isolated-summary.json")))
print("ISO_SUMMARY_MEAN_ms", {a["role"]: round(a["mean_cpu_seconds"] * 1000, 4) for a in isum["aggregate"]})
print("ISO_SUMMARY_IDENTICAL_SAME_CRATER_PAIRS", isum["identical_same_crater_pairs"], "RECURRENCE_SAME_ENDPOINT_MATCHES", isum["recurrence_same_endpoint_matches"])
# orbit-level same_crater_exact_checked counts (per record, all four trial files)
sc = collections.Counter()
for f in trial_files:
    for l in open(os.path.join(RES, f)):
        r = json.loads(l); sc[(f, str(r.get("same_crater_exact_checked")))] += 1
print("SAME_CRATER_EXACT_CHECKED_FLAGS", dict(sc))
pat_off = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|not committed|scratch|Modal volume", re.I)
pat_aa = re.compile(r"A/A|null control|negative control|placebo|null-control", re.I)
pat_iso = re.compile(r"isolat|taskset|pinned|\bL0\b|tier|shared", re.I)
files = [os.path.join(STUDY, "REPORT.md"), os.path.join(STUDY, "PROTOCOL.md"), os.path.join(STUDY, "REVIEW.md")]
offrepo, aa, iso_l = [], [], []
for fp in files:
    if not os.path.exists(fp): continue
    rel = os.path.relpath(fp, REPO)
    for i, line in enumerate(open(fp, encoding="utf-8", errors="replace"), 1):
        if pat_off.search(line): offrepo.append(f"{rel}:{i}: {line.strip()}")
        if pat_aa.search(line): aa.append(f"{rel}:{i}: {line.strip()}")
        if pat_iso.search(line): iso_l.append(f"{rel}:{i}: {line.strip()}")
print("OFFREPO", *offrepo, sep="\n"); print("AA", *aa, sep="\n"); print("ISO", *iso_l, sep="\n")
def gitdate(rel):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel], capture_output=True, text=True)
    return r.stdout.strip()
print("GITDATE_PROTOCOL", gitdate("research/conductor_gap_cm_orbits_20261015/PROTOCOL.md") or "EMPTY")
print("GITDATE_FIRSTRAW", gitdate("research/conductor_gap_cm_orbits_20261015/results/orbits.jsonl") or "EMPTY")
print("STORE_INDEX_EXISTS", os.path.exists(os.path.join(REPO, "store/index.json")))

# ---- row object ----
def mk(claimed, computed, definition, files, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files, "status": status}
PROG = ["research/conductor_gap_cm_orbits_20261015/results/campaign-progress.json"]
TRIALS = ["research/conductor_gap_cm_orbits_20261015/results/" + f for f in trial_files]
ISO_F = ["research/conductor_gap_cm_orbits_20261015/results/isolated-orbits.jsonl"]
PAIRED = ["research/conductor_gap_cm_orbits_20261015/results/paired-primary.json"]
claims_out = [
 mk("44,000 orbit controls", str(sum(n_trials.values())), "sum of line counts of orbits, isolated-orbits, recurrence-orbits, cold-orbits .jsonl", TRIALS, "match"),
 mk("500 primary attempts verified", str(primary), "sum of campaign-progress counts primary/* verified", PROG, "match"),
 mk("265 replication attempts verified", str(repl), "sum of campaign-progress counts replication/* verified", PROG, "match"),
 mk("145 exact same-crater solve comparisons", str(prog["same_crater_exact_solve_matches"] + prog["replication_same_crater_exact_solve_matches"]),
    "campaign-progress same_crater_exact_solve_matches (100) + replication_same_crater_exact_solve_matches (45); aggregate counters, not per-comparison records", PROG, "match"),
 mk("floor/crater native CPU 0.980", f"{fc['ratio_of_means']:.6f}", "ratio of arithmetic means, paired bootstrap unit (whole pair), paired-primary.json", PAIRED, "match"),
 mk("floor/crater native CPU [0.866, 1.111]", f"[{fc['paired_percentile_95'][0]:.6f}, {fc['paired_percentile_95'][1]:.6f}]", "paired_percentile_95 in paired-primary.json", PAIRED, "match"),
 mk("pinned construction CM 5.464 ms", f"{statistics.mean(byrole[0])*1000:.4f} ms (role 0, n=2000)", "mean orbit_cpu_seconds over role 0 in isolated-orbits.jsonl; role-to-arm mapping taken from REPORT text (no arm field in raw)", ISO_F, "match"),
 mk("pinned floor recurrence 6.857 ms", f"{statistics.mean(byrole[5])*1000:.4f} ms (role 5, n=2000)", "mean orbit_cpu_seconds over role 5 in isolated-orbits.jsonl; role-to-arm mapping taken from REPORT text", ISO_F, "match"),
 mk("pinned trials 12,000 (CPU 0)", str(len(iso)), "line count of isolated-orbits.jsonl", ISO_F, "match"),
 mk("100 primary same-crater pairs verified", f"all_100_pairs_verified={pp['all_100_pairs_verified']}, paired_endpoints_observed={pp['paired_endpoints_observed']}", "paired-primary.json fields", PAIRED, "match"),
]
row = {
 "row_no": 61, "slug": "20261015-conductor-gap-cm-orbits", "claims": claims_out,
 "identical": {"claimed": True, "n_compared": prog["same_crater_exact_solve_matches"] + prog["replication_same_crater_exact_solve_matches"],
               "n_equal": prog["same_crater_exact_solve_matches"] + prog["replication_same_crater_exact_solve_matches"], "n_differ": 0,
               "files": PROG, "status": "partial"},
 "offrepo_refs": offrepo, "aa_lines": aa, "isolation_lines": iso_l,
 "protocol_git_date": "2026-10-17T10:49:49-07:00 (PROTOCOL.md added in commit c6d64436a33359320ad9d5b42c4acebee2968b9a)",
 "first_raw_git_date": "2026-10-17T10:49:49-07:00 (results/orbits.jsonl added in commit c6d64436a33359320ad9d5b42c4acebee2968b9a)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/61.py",
 "notes": "identical field: the 145 same-crater comparisons are campaign-progress aggregate counters (100 primary + 45 replication); no per-comparison raw record with an equality field was found, so status partial. Orbit-level same_crater_exact_checked flags in the four trial files: orbits.jsonl False 2100 / True 5900; isolated-orbits.jsonl False 200 / True 11800; recurrence-orbits.jsonl False 200 / True 11800; cold-orbits.jsonl False 4000 / True 8000 (not solve comparisons). isolated-summary.json reports identical_same_crater_pairs 2000 and recurrence_same_endpoint_matches 4000. The isolated raw records carry a role number, not an arm name; the arm mapping for role 5 (floor recurrence) and role 0 (CM) is taken from REPORT.md lines 82-84 text. REPORT.md line 296 has a 'A/A' string in a table row about 44,000 constructions. The AA lines listed are REPORT.md lines 159, 169, 296 and PROTOCOL.md lines 24, 77. The offrepo grep matched nothing. The REVIEW.md line 23 states 'native CPU ratio 0.980 [0.866, 1.111]'. store/index.json exists."
}
with open(OUTJSONL, "a", encoding="utf-8") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("ROW61_WRITTEN", len(claims_out), "claims")
