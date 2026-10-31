import json, re, glob, statistics as st, subprocess, os, collections
D = 'research/inherited_f4_eager_runs_20261016/'
R = D + 'reports/run-20261016T2335Z/'
RAW = R + 'raw/'
ARMS = ['bmi2', 'runs', 'lazy', 'scatter']
NAME = re.compile(r'^r(\d+)-(K[01]_\d+(?:d\d)?)-(bmi2|runs|lazy|scatter)\.jsonl$')
files = sorted(f for f in os.listdir(RAW) if f.endswith('.jsonl'))
dec = collections.defaultdict(dict)   # (cell,engine,target) -> {(arm,round): tuple}
summ = {}                              # (cell,engine,arm,round) -> dict
for f in files:
    m = NAME.match(f)
    if not m:
        continue
    rd, cell, arm = int(m.group(1)), m.group(2), m.group(3)
    for line in open(RAW + f, encoding='utf-8'):
        if not line.strip():
            continue
        r = json.loads(line)
        if 'target' in r:
            key = (cell, r['arm'], r['target'])
            dec[key][(arm, rd)] = (r['hit'], tuple(r['indices']) if r['indices'] is not None else None, r['exhausted'])
        elif 'cpu_ms' in r:
            summ[(cell, r['arm'], arm, rd)] = dict(cpu=r['cpu_ms'], units=r['word_xors'])

cells = sorted({k[0] for k in dec})
engines = sorted({k[1] for k in dec})
n_targets = len(dec)
n_cmp = n_eq = 0
for key, rec in dec.items():
    vals = list(rec.values())
    ref = vals[0]
    for v in vals:
        n_cmp += 1
        n_eq += (v == ref)
n_keys_identical = sum(1 for rec in dec.values() if len({v for v in rec.values()}) == 1)

# units: per (cell,engine,arm) must be the same in all rounds; runs==bmi2, lazy==scatter
unit_var = 0
units = {}
for (cell, eng, arm, rd), s in summ.items():
    units.setdefault((cell, eng, arm), set()).add(s['units'])
for k, v in units.items():
    if len(v) != 1:
        unit_var += 1
eq_runs_bmi2 = all(units[(c, e, 'runs')] == units[(c, e, 'bmi2')] for (c, e, a) in units if a == 'runs')
eq_lazy_scatter = all(units[(c, e, 'lazy')] == units[(c, e, 'scatter')] for (c, e, a) in units if a == 'lazy')
n_units_pairs = sum(1 for k in units if k[2] == 'runs')

def cpu_med(cell, eng, arm):
    return st.median([summ[(cell, eng, arm, rd)]['cpu'] for rd in range(1, 6)])
def ratio(cell, eng, num, den):
    return cpu_med(cell, eng, num) / cpu_med(cell, eng, den)

# bracket rules (both readings, reported as facts)
def brackets(cell, eng):
    cross = [summ[(cell, eng, 'scatter', i)]['cpu'] / summ[(cell, eng, 'runs', j)]['cpu'] for i in range(1, 6) for j in range(1, 6)]
    same = [summ[(cell, eng, 'scatter', i)]['cpu'] / summ[(cell, eng, 'runs', i)]['cpu'] for i in range(1, 6)]
    return min(cross), max(cross), min(same), max(same)

six = ['K1_23', 'K0_23', 'K0_31', 'K0_19', 'K1_29', 'K0_37']
six_vals = {(c, e): ratio(c, e, 'scatter', 'runs') for c in six for e in engines}
six_lo, six_hi = min(six_vals.values()), max(six_vals.values())
k19 = {e: ratio('K1_19', e, 'scatter', 'runs') for e in engines}
lvl = [ratio(c, e, 'scatter', 'runs') for c in ('K1_17', 'K1_17d7') for e in engines]
bmi = [ratio(c, e, 'bmi2', 'runs') for c in cells for e in engines]
br = brackets('K1_17', 'f4')
br_cells = {f"{c}/{e}": [round(x, 3) for x in brackets(c, e)] for c in cells for e in engines}

OFF = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch", re.I)
AA = re.compile(r"A/A|A-A|null|negative control|control", re.I)
ISO = re.compile(r"isolat|PMU|isolated_bench|unqualified|contended|pinned|shared|hardware|L0|aarch64|x86", re.I)
STUDY = [D + 'README.md', R + 'report.typ', R + 'summary.txt', R + 'host.txt', D + 'scripts/run_arms.sh']
off, aa, iso = [], [], []
for p in STUDY:
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

claims = [
 {"claimed": "80/80 targets decided identically in every arm and round",
  "computed": f"distinct (cell,engine,target) keys {n_targets}; keys with one decision tuple across all arms and rounds {n_keys_identical}; compared {n_cmp}, equal {n_eq}",
  "definition": "decision tuple (hit, indices, exhausted) per target record, all arms x 5 rounds",
  "files": [RAW + f for f in files[:1]] + [D + 'reports/run-20261016T2335Z/summary.txt'],
  "status": "match" if (n_targets == 80 and n_keys_identical == 80) else "mismatch"},
 {"claimed": "runs charge exactly the BMI2 path's units; stage units identical across rounds",
  "computed": f"word_xors (summary record) keys {n_units_pairs} runs/bmi2 pairs; runs==bmi2 {eq_runs_bmi2}; lazy==scatter {eq_lazy_scatter}; (cell,engine,arm) with >1 distinct value across rounds {unit_var}",
  "definition": "word_xors per (cell,engine,arm,round) from summary records", "files": [RAW + 'r1-K1_17d7-runs.jsonl', RAW + 'r1-K1_17d7-bmi2.jsonl'],
  "status": "match" if (eq_runs_bmi2 and eq_lazy_scatter and unit_var == 0) else "mismatch"},
 {"claimed": "median CPU 1.22-1.70x lower (scatter/runs) on six cells on both engines",
  "computed": f"min {six_lo:.6f} max {six_hi:.6f} over K1_23,K0_23,K0_31,K0_19,K1_29,K0_37 x f4,f6p (median of 5 rounds cpu_ms)",
  "definition": "median cpu_ms scatter / median cpu_ms runs", "files": [RAW + 'r1-K1_17d7-runs.jsonl'],
  "status": "match" if (round(six_lo, 2) == 1.22 and round(six_hi, 2) == 1.70) else "mismatch"},
 {"claimed": "1.27x (f4) / 1.07x (f6p) on K1/2^19",
  "computed": f"f4 {k19['f4']:.6f}, f6p {k19['f6p']:.6f}",
  "definition": "median scatter / median runs", "files": [RAW + 'r1-K1_19-scatter.jsonl'],
  "status": "match" if (round(k19['f4'], 2) == 1.27 and round(k19['f6p'], 2) == 1.07) else "mismatch"},
 {"claimed": "level on K1/2^17 d6 and d7 (0.99-1.06x)",
  "computed": f"min {min(lvl):.6f} max {max(lvl):.6f} over K1_17 and K1_17d7 x f4,f6p",
  "definition": "median scatter / median runs", "files": [RAW + 'r1-K1_17-scatter.jsonl', RAW + 'r1-K1_17d7-scatter.jsonl'],
  "status": "match" if (round(min(lvl), 2) == 0.99 and round(max(lvl), 2) == 1.06) else "mismatch"},
 {"claimed": "against pdep∘pext: 2-28% slower (bmi2/runs 0.78-0.98)",
  "computed": f"bmi2/runs median CPU min {min(bmi):.6f} max {max(bmi):.6f} over 9 cells x 2 engines",
  "definition": "median cpu bmi2 / median cpu runs",
  "files": [RAW + 'r1-K1_17d7-bmi2.jsonl', RAW + 'r1-K1_17d7-runs.jsonl'],
  "status": "match" if (round(min(bmi), 2) == 0.78 and round(max(bmi), 2) == 0.98) else "mismatch"},
 {"claimed": "aarch64 host not measured", "computed": f"host.txt architecture line: {open(R+'host.txt',encoding='utf-8').read().split(chr(10))[1]!r}",
  "definition": "host record contains only x86 CPU; no aarch64 run file", "files": [R + 'host.txt'],
  "status": "match" if all('aarch64' not in open(R + 'host.txt', encoding='utf-8').read().lower() for _ in [0]) else "mismatch"},
]

ident = {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
         "files": [RAW], "status": "match" if (n_eq == n_cmp and n_cmp > 0) else "mismatch"}

out = {
 "row_no": 31, "slug": "20261016-inherited-f4-eager-runs", "claims": claims, "identical": ident,
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": git_date(D + 'scripts/run_arms.sh'),
 "first_raw_git_date": git_date(RAW + 'r1-K1_17d7-runs.jsonl'),
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/31.py",
 "notes": (f"raw files {len(files)}; cells {cells}; engines {engines}; bracket readings for K1_17/f4 (cross 5x5 min,max; same-round min,max): {[round(x,4) for x in br]}, table.tsv C_scatter/runs_lo=0.737 hi=1.225. "
           f"all-cell bracket readings: {json.dumps(br_cells, sort_keys=True)}. "
           f"raw records: host line in host.txt; x86 only. "
           f"raw record content includes absolute host path strings (build binary path) in summary files; not reproduced here."),
}
print(json.dumps(out))
