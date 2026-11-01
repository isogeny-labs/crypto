import json, re, statistics as st, subprocess, os, csv
D = 'research/fp64_branchfree_20261016/'
RUNS = {'base-1': D + 'runs/base-1/', 'cand-1': D + 'runs/cand-1/', 'base-2': D + 'runs/base-2/', 'cand-2': D + 'runs/cand-2/'}
STUDY_TEXT = [D + 'README.md', D + 'PROTOCOL.md', D + 'report.typ', D + 'reports/run-20261016T2319Z/run_report.typ', D + 'plot.py', D + 'disasm.sh', D + 'disasm_rho_worker.sh']

def med_csv(run):
    out = {}
    with open(RUNS[run] + 'medians.csv', encoding='utf-8') as f:
        for r in csv.DictReader(f):
            out[r['arm']] = float(r['median'])
    return out

M = {k: med_csv(k) for k in RUNS}

# verify medians.csv against raw samples (median over reps per arm)
mismatch, checked = 0, 0
for run in RUNS:
    d = json.load(open(RUNS[run] + 'measurements.json', encoding='utf-8'))
    by = {}
    for s in d['samples']:
        by.setdefault(s['arm'], []).append(s['value'])
    for arm, vals in by.items():
        if arm in M[run]:
            checked += 1
            if abs(st.median(vals) - M[run][arm]) > 1e-9 * max(1, abs(M[run][arm])):
                mismatch += 1

def r(arm, k):  # k=1: cand-1/base-1 ; k=2: cand-2/base-2
    return M[f'cand-{k}'][arm] / M[f'base-{k}'][arm]

def arms_with(prefix, lo=4096):
    out = []
    for a in M['base-1']:
        m = re.fullmatch(prefix + r'\.(\d+)', a)
        if m and int(m.group(1)) >= lo:
            out.append(a)
    return sorted(out, key=lambda x: int(x.rsplit('.', 1)[1]))

fp = arms_with(r'kernel\.fp64')
pf = arms_with(r'kernel\.pf64')
cm = arms_with(r'kernel\.fp64cm')
w1 = arms_with(r'kernel\.w1')
fp_lo = min(min(r(a, 1), r(a, 2)) for a in fp); fp_hi = max(max(r(a, 1), r(a, 2)) for a in fp)
pf_lo = min(min(r(a, 1), r(a, 2)) for a in pf); pf_hi = max(max(r(a, 1), r(a, 2)) for a in pf)
p1 = max(max(r(a, 1), r(a, 2)) for a in fp)
p1pf = max(max(r(a, 1), r(a, 2)) for a in pf)
p4 = max(r('pfrho.44', 1), r('pfrho.44', 2))
p2 = {a: max(r(a, 1), r(a, 2)) for a in ['fp64.mul.latency', 'fp64.sqr.latency', 'pf64.mul.latency', 'pf64.sqr.latency']}
c1a = max(abs(x - 1) for a in cm for x in (r(a, 1), r(a, 2)))
c1b = max(abs(x - 1) for a in w1 for x in (r(a, 1), r(a, 2)))
p3_cells = [M[f'cand-{k}'][a] / M[f'cand-{k}'][b] for k in (1, 2) for a, b in zip(fp, cm)]
p3 = max(p3_cells)
verdicts = {
 'P1': p1 <= 0.85, 'P1pf': p1pf <= 0.85, 'P4': p4 <= 0.95,
 'P2a': p2['fp64.mul.latency'] <= 1.10, 'P2b': p2['fp64.sqr.latency'] <= 1.10,
 'P2c': p2['pf64.mul.latency'] <= 1.10, 'P2d': p2['pf64.sqr.latency'] <= 1.10,
 'C1a': c1a <= 0.10, 'C1b': c1b <= 0.10}
n_pass = sum(verdicts.values())

def pct(x): return f"{x*100:.4f}%"
pf_files = [RUNS[k] + 'measurements.json' for k in RUNS]

claims = [
 {"claimed": "rho kernel 0.40-0.44x (fp_backend::Fp64) of baseline time at every B >= 4096",
  "computed": f"min {fp_lo:.6f} max {fp_hi:.6f} over arms {fp} and both runs",
  "definition": "r_k = median(cand-k arm)/median(base-k arm) from medians.csv, k=1,2; range of min(r1,r2) and max(r1,r2)",
  "files": [RUNS['base-1'] + 'medians.csv', RUNS['cand-1'] + 'medians.csv', RUNS['base-2'] + 'medians.csv', RUNS['cand-2'] + 'medians.csv'],
  "status": "match" if (round(fp_lo, 2) == 0.40 and round(fp_hi, 2) == 0.44) else "mismatch"},
 {"claimed": "0.45-0.47x (prime_fast::Fp64) of baseline time at every B >= 4096",
  "computed": f"min {pf_lo:.6f} max {pf_hi:.6f} over arms {pf} and both runs",
  "definition": "as above for kernel.pf64.B", "files": pf_files[:1] + pf_files[2:3],
  "status": "match" if (round(pf_lo, 2) == 0.45 and round(pf_hi, 2) == 0.47) else "mismatch"},
 {"claimed": "rho_parallel 98.9/98.6 -> 68.2/71.1 ns per step (0.69, 0.72)",
  "computed": f"base-1 {M['base-1']['pfrho.44']:.4f}, base-2 {M['base-2']['pfrho.44']:.4f}, cand-1 {M['cand-1']['pfrho.44']:.4f}, cand-2 {M['cand-2']['pfrho.44']:.4f} ns; r1 {r('pfrho.44',1):.6f}, r2 {r('pfrho.44',2):.6f}",
  "definition": "pfrho.44 medians per run; r_k = cand-k/base-k",
  "files": [RUNS[k] + 'medians.csv' for k in RUNS],
  "status": "match" if (f"{M['base-1']['pfrho.44']:.1f}" == '98.9' and f"{M['base-2']['pfrho.44']:.1f}" == '98.6' and f"{M['cand-1']['pfrho.44']:.1f}" == '68.2' and f"{M['cand-2']['pfrho.44']:.1f}" == '71.1' and round(r('pfrho.44',1), 2) == 0.69 and round(r('pfrho.44',2), 2) == 0.72) else "mismatch"},
 {"claimed": "dependent mul/sqr chains within +1.8%",
  "computed": f"max over fp64/pf64 mul,sqr latency arms of max(r1,r2) = {max(p2.values()):.6f} (+{(max(p2.values())-1)*100:.4f}%)",
  "definition": "max(r1,r2) per latency arm", "files": [RUNS[k] + 'medians.csv' for k in RUNS],
  "status": "match" if round((max(p2.values()) - 1) * 100, 1) == 1.8 else "mismatch"},
 {"claimed": "unchanged controls within 3%",
  "computed": f"max |r_k - 1| kernel.fp64cm.B {c1a*100:.4f}%, kernel.w1.B {c1b*100:.4f}% (B >= 4096)",
  "definition": "C1a/C1b as in PROTOCOL.md", "files": [RUNS[k] + 'medians.csv' for k in RUNS],
  "status": "match" if max(c1a, c1b) * 100 <= 3.0 else "mismatch"},
 {"claimed": "all 9 frozen claims PASS",
  "computed": f"recomputed PASS count {n_pass} of {len(verdicts)} (P1,P1pf,P4,P2a,P2b,P2c,P2d,C1a,C1b); P3 recomputed {p3:.6f} (worst fp64/fp64cm, 12 cells, cand-1 and cand-2)",
  "definition": "thresholds from PROTOCOL.md table; P3 = worst cand fp64/fp64cm kernel ratio at B>=4096",
  "files": [D + 'PROTOCOL.md'] + [RUNS[k] + 'medians.csv' for k in RUNS],
  "status": "match" if (n_pass == 9 and p3 <= 1.10) else "mismatch"},
 {"claimed": "dependent sub chains with a predictable wrap 18-33% slower on fp64",
  "computed": "not computed", "definition": "no sub-chain arm in measurements.json or medians.csv of the four runs",
  "files": [RUNS[k] + 'measurements.json' for k in RUNS], "status": "no_raw_in_repo"},
 {"claimed": "the same arm moved -27% and -14% for the unchanged controls",
  "computed": "not computed", "definition": "no sub-chain control arm in the four runs' records",
  "files": [RUNS[k] + 'measurements.json' for k in RUNS], "status": "no_raw_in_repo"},
]

OFF = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch", re.I)
AA = re.compile(r"A/A|A-A|null|negative control|control", re.I)
ISO = re.compile(r"isolat|PMU|isolated_bench|unqualified|contended|pinned|shared|hardware|L0", re.I)
off, aa, iso = [], [], []
for p in STUDY_TEXT:
    if not os.path.exists(p):
        continue
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
 "row_no": 29, "slug": "20261016-fp64-branchfree", "claims": claims,
 "identical": {"claimed": True, "n_compared": None, "n_equal": None, "n_differ": None,
               "files": [RUNS['cand-1'] + 'stderr.txt', RUNS['cand-2'] + 'stderr.txt'],
               "status": "no_raw_in_repo"},
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": git_date(D + 'PROTOCOL.md'),
 "first_raw_git_date": git_date(RUNS['base-1'] + 'measurements.json'),
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/29.py",
 "notes": (f"medians.csv vs median of measurements.json samples: checked {checked} arm-run pairs, mismatches {mismatch}. "
           f"identity claim: no base-vs-candidate output comparison field found; stderr lines in cand-1/cand-2 record per-binary checks only ('batched walk == unbatched reference' etc.). "
           f"P3 is computed in runs/cm-pair/claims.json, not compared to recompute beyond cand worst ratio. "
           f"the measurements host.build.binary_path references /tmp/claude-0/... (off-repo path inside raw record)."),
}
print(json.dumps(out))
