import json, re, statistics as st, subprocess, os, glob
D = 'research/rho_canon2_20261016/'
CG = D + 'runs/callgrind/'
TIME = D + 'runs/time/'
STUDY_TEXT = [D + 'README.md', D + 'PROTOCOL.md', D + 'report/report.typ', D + 'run_time.sh', D + 'summarise.sh', D + 'report/plots.py']

def jl(p):
    return [json.loads(l) for l in open(p, encoding='utf-8') if l.strip()]

def collected(p):
    return int(re.search(r'Collected : (\d+)', open(p, encoding='utf-8').read()).group(1))

arms = {}
for arm in ['baseline', 'final']:
    ir = collected(f'{CG}fixture-n53-4-{arm}.valgrind.log')
    recs = jl(f'{CG}fixture-n53-4-{arm}.jsonl')
    steps = sum(r['walk_steps'] for r in recs)
    arms[arm] = (ir, steps, ir / steps, recs)
base_ps, final_ps = arms['baseline'][2], arms['final'][2]
reduction = (1 - final_ps / base_ps) * 100
speed = arms['baseline'][0] / arms['final'][0]

def ident(r):
    return json.dumps({k: v for k, v in r.items() if k != 'peak_rss_bytes' and not k.endswith('_ms')}, sort_keys=True)
bl, fl = arms['baseline'][3], arms['final'][3]
n_cmp = min(len(bl), len(fl))
n_eq = sum(1 for a, b in zip(bl, fl) if ident(a) == ident(b))

iso_rows = []
for f in glob.glob(TIME + '*/isolation.jsonl'):
    iso_rows += jl(f)
contended = sorted(r['label'] for r in iso_rows if r.get('run', {}).get('contended') is True)
n_runs = len(iso_rows)
cells = {}
for f in sorted(glob.glob(TIME + '*/*.jsonl')):
    if f.endswith('isolation.jsonl'):
        continue
    phase = os.path.basename(os.path.dirname(f))
    fname = os.path.basename(f)[:-6]
    if f'time/{phase}/{fname}' in contended:
        continue
    recs = jl(f)
    ns = sum(r['walk_ms'] for r in recs) * 1e6 / sum(r['walk_steps'] for r in recs)
    arm = re.search(r'-(baseline|candidate)-r', fname).group(1)
    slug = re.match(r'(icv1-[^-]+-[^-]+-[0-9a-f]{8})', fname).group(1)
    cells.setdefault(slug, []).append((phase, arm, ns))
res = {}
for slug, items in cells.items():
    aa = [x[2] for x in items if x[0] == 'aa']
    b = [x[2] for x in items if x[0] == 'ab' and x[1] == 'baseline']
    c = [x[2] for x in items if x[0] == 'ab' and x[1] == 'candidate']
    res[slug] = dict(aa_spread=max(aa) / min(aa) - 1, aa_n=len(aa), b_n=len(b), c_n=len(c), ratio=st.median(b) / st.median(c))
n53 = [k for k in res if 'f2m53' in k][0]
n61 = [k for k in res if 'f2m61' in k][0]

perf_status = jl(D + 'runs/perf/final-n53.jsonl')[0].get('status')

OFF = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch", re.I)
AA = re.compile(r"A/A|A-A|null|negative control|control", re.I)
ISO = re.compile(r"isolat|PMU|isolated_bench|unqualified|contended|pinned|shared|hardware", re.I)
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

def ok(v, t, dp): return round(v, dp) == t

claims = [
 {"claimed": "938 -> 781 instructions per step (-16.8%, 1.20x)",
  "computed": f"baseline {base_ps:.4f} -> final {final_ps:.4f} instr/step; reduction {reduction:.4f}%; baseline/final {speed:.6f}",
  "definition": "Collected (valgrind.log) / sum(walk_steps), 4 fixtures, n53, arms baseline and final",
  "files": [CG + 'fixture-n53-4-baseline.valgrind.log', CG + 'fixture-n53-4-final.valgrind.log', CG + 'fixture-n53-4-baseline.jsonl', CG + 'fixture-n53-4-final.jsonl'],
  "status": "match" if (ok(base_ps, 938, 0) and ok(final_ps, 781, 0) and ok(reduction, 16.8, 1) and ok(speed, 1.20, 2)) else "mismatch"},
 {"claimed": "native 1.173x at n53", "computed": f"{res[n53]['ratio']:.6f}",
  "definition": f"median(baseline ns/step)/median(candidate ns/step), uncontended ab runs, n={res[n53]['b_n']}/{res[n53]['c_n']} (summarise.sh rule)",
  "files": [TIME + 'ab/*.jsonl'], "status": "match" if ok(res[n53]['ratio'], 1.173, 3) else "mismatch"},
 {"claimed": "A/A 20.6% (n53)", "computed": f"{res[n53]['aa_spread']*100:.4f}% from n={res[n53]['aa_n']}",
  "definition": "max/min - 1 over uncontended aa ns/step", "files": [TIME + 'aa/*.jsonl'],
  "status": "match" if ok(res[n53]['aa_spread']*100, 20.6, 1) else "mismatch"},
 {"claimed": "1.135x at n61", "computed": f"{res[n61]['ratio']:.6f}",
  "definition": f"median(baseline ns/step)/median(candidate ns/step), uncontended ab runs, n={res[n61]['b_n']}/{res[n61]['c_n']}",
  "files": [TIME + 'ab/*.jsonl'], "status": "match" if ok(res[n61]['ratio'], 1.135, 3) else "mismatch"},
 {"claimed": "A/A 12.1% (n61)", "computed": f"{res[n61]['aa_spread']*100:.4f}% from n={res[n61]['aa_n']}",
  "definition": "max/min - 1 over uncontended aa ns/step", "files": [TIME + 'aa/*.jsonl'],
  "status": "match" if ok(res[n61]['aa_spread']*100, 12.1, 1) else "mismatch"},
 {"claimed": "outputs identical at n = 41, 53, 61",
  "computed": f"n53 baseline vs final: compared {n_cmp}, equal {n_eq}; no n41/n61 records of either arm in repo",
  "definition": "non-timing fields (drop *_ms, peak_rss_bytes) compared per fixture record",
  "files": [CG + 'fixture-n53-4-baseline.jsonl', CG + 'fixture-n53-4-final.jsonl'], "status": "partial"},
 {"claimed": "IPC unavailable (no hardware PMU)", "computed": f"perf record status={perf_status}",
  "definition": "runs/perf/final-n53.jsonl status field", "files": [D + 'runs/perf/final-n53.jsonl'],
  "status": "match" if perf_status == 'unsupported' else "mismatch"},
 {"claimed": "0 of 32 runs contended", "computed": f"{len(contended)} contended of {n_runs} isolation rows",
  "definition": "count of run.contended == true over runs/time/*/isolation.jsonl",
  "files": [TIME + 'aa/isolation.jsonl', TIME + 'ab/isolation.jsonl'],
  "status": "match" if (len(contended) == 0 and n_runs == 32) else "mismatch"},
]

out = {
 "row_no": 28, "slug": "20261016-strong-rho-canon2", "claims": claims,
 "identical": {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
               "files": [CG + 'fixture-n53-4-baseline.jsonl', CG + 'fixture-n53-4-final.jsonl'],
               "status": "match" if n_eq == n_cmp and n_cmp > 0 else "mismatch"},
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": git_date(D + 'PROTOCOL.md'),
 "first_raw_git_date": git_date(TIME + 'aa/isolation.jsonl'),
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/28.py",
 "notes": (f"native cells: {json.dumps(res, sort_keys=True)}. contended labels: {contended}. "
           f"report/report.pdf not read (binary; source report/report.typ scanned). "
           f"first RAW CANDIDATE runs/time/aa/isolation.jsonl used as first_raw; callgrind .callgrind.out files are not in repo (only .jsonl and .valgrind.log)."),
}
print(json.dumps(out))
