import json, re, statistics as st, subprocess, os, glob
D = 'research/prime_order_search_20261017/'
RC = D + 'replan-check.txt'
MAIN, NEW = D + 'timing-main.txt', D + 'timing-new.txt'
STUDY = [D + 'README.md', D + 'report.html', D + 'make_figure.py', RC, MAIN, NEW]

def load(p):
    d = {}
    for line in open(p, encoding='utf-8'):
        if line.strip():
            _, b, _, s = line.split()
            d.setdefault(int(b), []).append(float(s))
    return d

main, new = load(MAIN), load(NEW)
mmed = {b: st.median(v) for b, v in main.items()}
nmed = {b: st.median(v) for b, v in new.items()}

def cl(claimed, computed, definition, files, ok):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files,
            "status": "match" if ok else "mismatch"}

claims = []
claims.append(cl("plan time 0.21 -> 0.040 s (16 bits)",
                 f"main {mmed[16]:.6f} s -> new {nmed[16]:.6f} s",
                 "median of 3 runs each (timing-main.txt, timing-new.txt)",
                 [MAIN, NEW], round(mmed[16], 2) == 0.21 and round(nmed[16], 3) == 0.040))
claims.append(cl("6.90 -> 0.064 s (22)",
                 f"main {mmed[22]:.6f} s -> new {nmed[22]:.6f} s",
                 "median of 3 runs each", [MAIN, NEW],
                 round(mmed[22], 2) == 6.90 and round(nmed[22], 3) == 0.064))
claims.append(cl("10.96 -> 0.046 s (24)",
                 f"main {mmed[24]:.6f} s -> new {nmed[24]:.6f} s",
                 "median of 3 runs each", [MAIN, NEW],
                 round(mmed[24], 2) == 10.96 and round(nmed[24], 3) == 0.046))
claims.append(cl("217.6 -> 0.137 s (26)",
                 f"main {mmed[26]:.6f} s (1 run) -> new {nmed[26]:.6f} s (3 runs, median)",
                 "main median of 1 run; new median of 3 runs", [MAIN, NEW],
                 round(mmed[26], 1) == 217.6 and round(nmed[26], 3) == 0.137))
ratio26 = mmed[26] / nmed[26]
claims.append(cl("1,592x (26 bits)", f"{ratio26:.4f}",
                 "median main / median new at 26 bits, rounded to integer",
                 [MAIN, NEW], round(ratio26) == 1592))

# replan record: 21 committed sessions, 288 workload IDs
rc = [l.split() for l in open(RC, encoding='utf-8') if l.strip()]
n_sessions = len(rc)
n_wl_rc = sum(int(r[1]) for r in rc)
claims.append(cl("all 21 committed prime_search sessions re-plan",
                 str(n_sessions), "count of lines in replan-check.txt", [RC], n_sessions == 21))
claims.append(cl("288 recorded workload IDs", str(n_wl_rc),
                 "sum of the workload-count column in replan-check.txt", [RC], n_wl_rc == 288))

# recorded plan.json workload counts for the same sessions (raw records)
plan_counts, plan_missing, eq_count = [], [], 0
for r in rc:
    pj = r[2] + '/plan.json'
    if not os.path.exists(pj):
        plan_missing.append(pj); continue
    d = json.load(open(pj, encoding='utf-8'))
    n = len(d.get('workloads', []))
    plan_counts.append((pj, n))
    if n == int(r[1]): eq_count += 1
n_plan = sum(n for _, n in plan_counts)

# unit-test claim: source exists, no recorded output in repo
src = 'src/cryptanalysis/ic_boundary.rs'
test_src = os.path.exists(src) and 'fn prime_order_matches_the_naive_count' in open(src, encoding='utf-8').read()

claims.append({"claimed": "unit test agrees with the naive count on 400 curves and the naive search at 8-16 bits",
               "computed": "not computed (cargo test forbidden; no recorded test output in repo)",
               "definition": f"test source present: {test_src} ({src}:8476)",
               "files": [src], "status": "no_raw_in_repo"})

OFF = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch", re.I)
AA = re.compile(r"A/A|A-A|null|negative control|control", re.I)
ISO = re.compile(r"isolat|PMU|isolated_bench|unqualified|contended|pinned|shared|L0", re.I)
off, aa, iso = [], [], []
for p in STUDY:
    if p.endswith('.txt') and p != RC: pass
    for i, line in enumerate(open(p, encoding='utf-8'), 1):
        s = line.rstrip('\n')
        if OFF.search(s): off.append(f"{p}:{i}: {s.strip()}")
        if AA.search(s): aa.append(f"{p}:{i}: {s.strip()}")
        if ISO.search(s): iso.append(f"{p}:{i}: {s.strip()}")

def git_date(path):
    out = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY (no add commit for ' + path + ')'

out = {
    "row_no": 26,
    "slug": "20261017-prime-order-search",
    "claims": claims,
    "identical": {"claimed": True, "n_compared": n_plan, "n_equal": eq_count and n_plan if eq_count == len(plan_counts) else eq_count,
                  "n_differ": len(plan_counts) - eq_count,
                  "files": [RC] + [p for p, _ in plan_counts],
                  "status": "partial"},
    "offrepo_refs": off,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": git_date(D + 'README.md'),
    "first_raw_git_date": git_date(D + 'bits16.json'),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/26.py",
    "notes": (f"no PROTOCOL file in study dir; protocol_git_date uses README.md; first_raw uses first listed RAW CANDIDATE bits16.json. "
              f"recorded plan.json workload counts: sessions with plan.json={len(plan_counts)}, missing={plan_missing}, sum={n_plan}, "
              f"count equal to replan-check column={eq_count}. replan itself not re-run (ecbench binaries forbidden); "
              f"comparison field is the committed replan-check.txt status 'identical' only. "
              f"timing 26-bit main has 1 run (timing-main.txt)."),
}
print(json.dumps(out))
