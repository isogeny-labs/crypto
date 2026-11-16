import gzip, json, re, statistics as st, subprocess, os
R = 'research/generalized_rootfinding_20261016/'
H = R + 'historical/'
ATT = H + 'lab/ledger/attempts/batch-separator-manyfield-exact-coldpairs-20261016-v1/0001/'
SCI = ATT + 'scientific_result.json.gz'
AU = H + 'research/autolab_cycle_20261016_v1/REPORT.json.gz'
STUDY = [R + 'README.md', R + 'reports/run-20261016T0125Z/REPORT.md']
PROTO = ATT + 'preregistration.json'
FIRST_RAW = ATT + 'process.json.gz'

def load(p):
    with gzip.open(p, 'rt') as f:
        return json.load(f)

sci = load(SCI)
ev = sci['evidence']
co, cn = ev['cold_old_seconds'], ev['cold_new_seconds']
claim1 = st.median(co) / st.median(cn)
per_pair = st.median([a / b for a, b in zip(co, cn)])

au = load(AU)
exp = {e['id']: e for e in au['experiments']}
ids = list(exp.keys())
a83 = [k for k in ids if 'actual83' in k][0]
row83 = exp[a83]['evidence']['rows'][0]
claim2 = st.median(row83['old_power_seconds']) / st.median(row83['gcd_shortcut_seconds'])
claim2_perrun = st.median([a / b for a, b in zip(row83['old_power_seconds'], row83['gcd_shortcut_seconds'])])
claim2_stored = row83['median_old_over_shortcut']

man = [k for k in ids if 'support-squarefree-cost-manyfields' in k][0]
nist = ['P192', 'P224', 'P256', 'P384', 'P521']
nrows = {r['case']: r for r in exp[man]['evidence']['rows'] if r['case'] in nist}
nvals = {c: st.median(r['old_power_seconds']) / st.median(r['gcd_shortcut_seconds']) for c, r in nrows.items()}
nstored = {c: r['median_old_over_shortcut'] for c, r in nrows.items()}
claim3_lo, claim3_hi = min(nvals.values()), max(nvals.values())

def git_date(path, flt):
    out = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY (no git add commit for ' + path + ')'

# study-file scans (README + REPORT only)
OFF = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch", re.I)
AA = re.compile(r"A/A|A-A|null|negative control|control", re.I)
ISO = re.compile(r"isolat|PMU|isolated_bench|unqualified|contended|pinned|shared", re.I)
off, aa, iso = [], [], []
for p in STUDY:
    for i, line in enumerate(open(p, encoding='utf-8'), 1):
        s = line.rstrip('\n')
        if OFF.search(s): off.append(f"{p}:{i}: {s.strip()}")
        if AA.search(s): aa.append(f"{p}:{i}: {s.strip()}")
        if ISO.search(s): iso.append(f"{p}:{i}: {s.strip()}")
stores = sorted(set(re.findall(r"store:([A-Za-z0-9_\-]+)", ' '.join(off))))
store_idx = os.path.exists('store/index.json')
store_info = {s: ('store/index.json not found' if not store_idx else
                  ('yes' if s in open('store/index.json', encoding='utf-8').read() else 'no')) for s in stores}

out = {
    "row_no": 25,
    "slug": "20261016-generalized-rootfinding",
    "claims": [
        {"claimed": "prior cached separator / batch whole-cold median 1.3830x",
         "computed": f"{claim1:.10f}",
         "definition": "median(cold_old_seconds)/median(cold_new_seconds), 3 cold pairs (ratio of medians; the study's stored cold_panel_median_old_over_new uses this rule). Per-pair median of ratios computed as %.10f" % per_pair,
         "files": [ATT + 'scientific_result.json.gz'],
         "status": "match" if abs(claim1 / 1.3830 - 1) <= 0.01 else "mismatch"},
        {"claimed": "actual GF83 support-only 1.1409x",
         "computed": f"{claim2:.10f}",
         "definition": f"median(old_power_seconds)/median(gcd_shortcut_seconds) over 3 rounds, experiment {a83}, case actual83-fixedh-k-relaxed16. Stored median_old_over_shortcut={claim2_stored:.10f}; per-run median of ratios={claim2_perrun:.10f}",
         "files": [AU],
         "status": "match" if abs(claim2 / 1.1409 - 1) <= 0.01 else "mismatch"},
        {"claimed": "repeated-root NIST support cases regress (0.5948-0.7034)",
         "computed": f"min {claim3_lo:.10f} (P-case with min) max {claim3_hi:.10f}; per case " + ", ".join(f"{c}={nvals[c]:.10f}" for c in nist),
         "definition": "per NIST case (P192,P224,P256,P384,P521) median(old_power)/median(gcd_shortcut) in experiment " + man + "; min and max",
         "files": [AU],
         "status": "match" if abs(claim3_lo - 0.5948) < 0.00005 and abs(claim3_hi - 0.7034) < 0.00005 else "mismatch"},
    ],
    "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
    "offrepo_refs": off,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": git_date(PROTO, None),
    "first_raw_git_date": git_date(FIRST_RAW, None),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/25.py",
    "notes": f"store refs {stores} in store/index.json: {store_info}; claim 2 stored value equals ratio of medians to 1e-9: {abs(claim2-claim2_stored)<1e-9}; NIST stored values {nstored}",
}
print(json.dumps(out))
