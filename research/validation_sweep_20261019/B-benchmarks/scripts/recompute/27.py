import json, re, statistics as st, subprocess, os, glob
D = 'research/rho_step3_20261016/'
CG = D + 'runs/callgrind/'
TIME = D + 'runs/time/'
STUDY_TEXT = [D + 'README.md', D + 'PROTOCOL.md', D + 'report/report.typ', D + 'run_time.sh', D + 'summarise.sh', D + 'report/plots.py']

def jl(p):
    return [json.loads(l) for l in open(p, encoding='utf-8') if l.strip()]

def collected(p):
    m = re.search(r'Collected : (\d+)', open(p, encoding='utf-8').read())
    return int(m.group(1))

# Callgrind instructions per walk step (summarise.sh rule)
arms = {}
for arm in ['baseline', 'change1', 'final']:
    log = f'{CG}fixture-n53-4-{arm}.valgrind.log'
    recs = jl(f'{CG}fixture-n53-4-{arm}.jsonl')
    steps = sum(r['walk_steps'] for r in recs)
    ir = collected(log)
    arms[arm] = (ir, steps, ir / steps, recs)
base_ps, final_ps = arms['baseline'][2], arms['final'][2]
reduction = (1 - final_ps / base_ps) * 100
speed = arms['baseline'][0] / arms['final'][0]

# identity at n53: baseline vs final, all non-timing fields (summarise.sh rule)
def ident(r):
    return json.dumps({k: v for k, v in r.items() if k != 'peak_rss_bytes' and not k.endswith('_ms')}, sort_keys=True)
bl, fl = arms['baseline'][3], arms['final'][3]
n_cmp = min(len(bl), len(fl))
n_eq = sum(1 for a, b in zip(bl, fl) if ident(a) == ident(b))
n53_digests = {ident(r) for r in bl + fl}
# merged n41/n61 files contain a single arm; no baseline record in repo for them
merged = {n: jl(f'{D}runs/merge/fixture-n{n}-4-merged.jsonl') for n in (41, 61)}

# native time (summarise.sh rule)
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
    label = f'time/{phase}/{fname}'
    if label in contended:
        continue
    recs = jl(f)
    ns = sum(r['walk_ms'] for r in recs) * 1e6 / sum(r['walk_steps'] for r in recs)
    arm = re.search(r'-(baseline|candidate)-r', fname).group(1)
    slug = re.match(r'(icv1-[^-]+-[^-]+-[0-9a-f]{8})', fname).group(1)
    cells.setdefault(slug, []).append((phase, arm, ns, fname))
res = {}
for slug, items in cells.items():
    aa = [x[2] for x in items if x[0] == 'aa']
    b = [x[2] for x in items if x[0] == 'ab' and x[1] == 'baseline']
    c = [x[2] for x in items if x[0] == 'ab' and x[1] == 'candidate']
    res[slug] = dict(aa_spread=max(aa) / min(aa) - 1, aa_n=len(aa), b_med=st.median(b), b_n=len(b), c_med=st.median(c), c_n=len(c), ratio=st.median(b) / st.median(c))

n53 = [k for k in res if 'f2m53' in k][0]
n61 = [k for k in res if 'f2m61' in k][0]

# IPC record
perf = jl(D + 'runs/perf/final-n53.jsonl')[0]
perf_status = perf.get('status')

# study text scans
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

def chk(v, target, dp):
    return round(v, dp) == target

claims = [
 {"claimed": "781 -> 718 instructions per step (-8.1%, 1.09x)",
  "computed": f"baseline {base_ps:.4f} -> final {final_ps:.4f} instr/step; reduction {reduction:.4f}%; baseline/final {speed:.6f}",
  "definition": "Collected (valgrind.log) / sum(walk_steps) over 4 fixtures, n53, arms baseline and final (summarise.sh)",
  "files": [CG + 'fixture-n53-4-baseline.valgrind.log', CG + 'fixture-n53-4-final.valgrind.log', CG + 'fixture-n53-4-baseline.jsonl', CG + 'fixture-n53-4-final.jsonl'],
  "status": "match" if (chk(base_ps, 781, 0) and chk(final_ps, 718, 0) and chk(reduction, 8.1, 1) and chk(speed, 1.09, 2)) else "mismatch"},
 {"claimed": "native 1.008x at n53", "computed": f"{res[n53]['ratio']:.6f}",
  "definition": f"median(baseline ns/step)/median(candidate ns/step), uncontended ab runs, n={res[n53]['b_n']}/{res[n53]['c_n']} (summarise.sh)",
  "files": [TIME + 'ab/icv1-f2m53-tm56619371-dac20a85-baseline-r1.jsonl'], "status": "match" if chk(res[n53]['ratio'], 1.008, 3) else "mismatch"},
 {"claimed": "A/A 5.6% (n53)", "computed": f"{res[n53]['aa_spread']*100:.4f}%",
  "definition": f"max/min - 1 over uncontended aa ns/step, n={res[n53]['aa_n']}", "files": [TIME + 'aa/*.jsonl'],
  "status": "match" if chk(res[n53]['aa_spread']*100, 5.6, 1) else "mismatch"},
 {"claimed": "1.025x at n61", "computed": f"{res[n61]['ratio']:.6f}",
  "definition": f"median(baseline ns/step)/median(candidate ns/step), uncontended ab runs, n={res[n61]['b_n']}/{res[n61]['c_n']}",
  "files": [TIME + 'ab/icv1-f2m61-t158598901-ab42b6c5-baseline-r1.jsonl'], "status": "match" if chk(res[n61]['ratio'], 1.025, 3) else "mismatch"},
 {"claimed": "A/A 2.4% from 4 runs (n61)", "computed": f"{res[n61]['aa_spread']*100:.4f}% from n={res[n61]['aa_n']} uncontended aa runs",
  "definition": "max/min - 1 over uncontended aa ns/step", "files": [TIME + 'aa/*.jsonl'],
  "status": "match" if (chk(res[n61]['aa_spread']*100, 2.4, 1) and res[n61]['aa_n'] == 4) else "mismatch"},
 {"claimed": "outputs identical at n = 41, 53, 61",
  "computed": f"n53 baseline vs final: compared {n_cmp}, equal {n_eq}; n41/n61 merged files hold one arm only (no baseline record)",
  "definition": "non-timing fields (drop *_ms, peak_rss_bytes) compared per fixture record", "files": [CG + 'fixture-n53-4-baseline.jsonl', CG + 'fixture-n53-4-final.jsonl', D + 'runs/merge/fixture-n41-4-merged.jsonl', D + 'runs/merge/fixture-n61-4-merged.jsonl'],
  "status": "partial"},
 {"claimed": "IPC unavailable (no hardware PMU)", "computed": f"perf record status={perf_status}",
  "definition": "runs/perf/final-n53.jsonl status field", "files": [D + 'runs/perf/final-n53.jsonl'],
  "status": "match" if perf_status == 'unsupported' else "mismatch"},
 {"claimed": "4 of 32 runs contended (excluded)", "computed": f"{len(contended)} of {n_runs} isolation rows contended: {contended}",
  "definition": "count of run.contended == true over runs/time/*/isolation.jsonl", "files": [TIME + 'aa/isolation.jsonl', TIME + 'ab/isolation.jsonl'],
  "status": "match" if (len(contended) == 4 and n_runs == 32) else "mismatch"},
]

ident_claim = {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
               "files": [CG + 'fixture-n53-4-baseline.jsonl', CG + 'fixture-n53-4-final.jsonl'],
               "status": "match" if n_eq == n_cmp and n_cmp > 0 else "mismatch"}

out = {
 "row_no": 27, "slug": "20261016-strong-rho-step3", "claims": claims,
 "identical": ident_claim,
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": git_date(D + 'PROTOCOL.md'),
 "first_raw_git_date": git_date(CG + 'fixture-n53-4-baseline.callgrind.out'),
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/27.py",
 "notes": (f"n41/n61 merged files: {[(n, len(m), sorted({r['n'] for r in m})) for n, m in merged.items()]} each one arm, no baseline record for those n. "
           f"native cells: {json.dumps(res, sort_keys=True)}. "
           f"raw perf record command references /tmp/claude-0/arms6/ binary path (off-repo path string inside raw record, runs/perf/final-n53.jsonl); "
           f"raw time records reference /tmp/claude-0/arms6 binaries (runs/time/*). "
           f"report/report.pdf not read (binary; source report/report.typ scanned). "
           f"A/A uncontended counts n53={res[n53]['aa_n']}, n61={res[n61]['aa_n']}."),
}
print(json.dumps(out))
