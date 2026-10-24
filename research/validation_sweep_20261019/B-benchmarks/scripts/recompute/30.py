import re, statistics as st, subprocess, os, csv
D = 'research/f4_fp_blocked_pairs_20261016/'
R = D + 'reports/run-20261016T1720Z/'
AB = R + 'raw/ab_full.tsv'
TAB = R + 'table.tsv'
RUN = R + 'raw/run_ab.sh'
STUDY_TEXT = [D + 'README.md', R + 'report.typ', D + 'plot.py', RUN]

rows = []
for line in open(AB, encoding='utf-8'):
    if line.strip():
        rd, case, arm, ms, dig = line.rstrip('\n').split('\t')
        rows.append((int(rd), case, arm, float(ms), dig))
cases = sorted({r[1] for r in rows})
rounds = sorted({r[0] for r in rows})
def vals(case, arm):
    return [r[3] for r in sorted(rows) if r[1] == case and r[2] == arm]
def digs(case, arm):
    return [r[4] for r in sorted(rows) if r[1] == case and r[2] == arm]

stats = {}
for c in cases:
    b, k, m = vals(c, 'ours_B1'), vals(c, 'ours_B0'), vals(c, 'msolve')
    stats[c] = dict(bm=st.median(b), km=st.median(k), mm=st.median(m),
                    bmin=min(b), bmax=max(b), kmin=min(k), kmax=max(k),
                    ratio_bc=st.median(b) / st.median(k), ratio_bm=st.median(b) / st.median(m))

# identity: blocked vs control basis digests, every round and case
n_cmp = n_eq = 0
for c in cases:
    bd, kd = digs(c, 'ours_B1'), digs(c, 'ours_B0')
    for x, y in zip(bd, kd):
        n_cmp += 1
        n_eq += (x == y)

# table.tsv cross-check
tab = {r['case']: r for r in csv.DictReader(open(TAB, encoding='utf-8'), delimiter='\t')}
tab_mismatch = 0
for c in cases:
    if c in tab:
        if abs(float(tab[c]['blocked/control']) - round(stats[c]['ratio_bc'], 3)) > 0.0015:
            tab_mismatch += 1

def ok(v, t): return round(v, 2) == t
q10, q11, q9 = stats['quad_n10_p65521'], stats['quad_n11_p65521'], stats['quad_n9_p65521']
disjoint10 = q10['bmax'] < q10['kmin']
disjoint11 = q11['bmax'] < q11['kmin']
overlap9 = q9['bmin'] <= q9['kmax'] and q9['kmin'] <= q9['bmax']
band = [stats[c]['ratio_bm'] for c in ['quad_n8_p65521', 'quad_n9_p65521', 'cubic_n5_p31', 'cubic_n6_p65521']]
files_ab = [AB]

claims = [
 {"claimed": "blocked/control 0.78x quad_n10 (1.41 s vs 1.80 s)",
  "computed": f"ratio {q10['ratio_bc']:.6f}; blocked median {q10['bm']/1000:.4f} s; control median {q10['km']/1000:.4f} s",
  "definition": "median over 3 rounds of ours_B1 / median of ours_B0 (ab_full.tsv), ms converted to s",
  "files": files_ab, "status": "match" if (ok(q10['ratio_bc'], 0.78) and round(q10['bm']/1000, 2) == 1.41 and round(q10['km']/1000, 2) == 1.80) else "mismatch"},
 {"claimed": "0.89x quad_n11 (9.37 s vs 10.55 s)",
  "computed": f"ratio {q11['ratio_bc']:.6f}; blocked median {q11['bm']/1000:.4f} s; control median {q11['km']/1000:.4f} s",
  "definition": "as above", "files": files_ab,
  "status": "match" if (ok(q11['ratio_bc'], 0.89) and round(q11['bm']/1000, 2) == 9.37 and round(q11['km']/1000, 2) == 10.55) else "mismatch"},
 {"claimed": "0.88x quad_n9 (ranges overlap)",
  "computed": f"ratio {q9['ratio_bc']:.6f}; blocked [{q9['bmin']},{q9['bmax']}] ms; control [{q9['kmin']},{q9['kmax']}] ms; overlap {overlap9}",
  "definition": "as above; ranges = min..max over 3 rounds", "files": files_ab,
  "status": "match" if (ok(q9['ratio_bc'], 0.88) and overlap9) else "mismatch"},
 {"claimed": "per-round ranges disjoint for quad_n10 and quad_n11",
  "computed": f"quad_n10 blocked max {q10['bmax']} < control min {q10['kmin']}: {disjoint10}; quad_n11 blocked max {q11['bmax']} < control min {q11['kmin']}: {disjoint11}",
  "definition": "blocked max below control min", "files": files_ab,
  "status": "match" if (disjoint10 and disjoint11) else "mismatch"},
 {"claimed": "vs msolve 0.78x / 0.75x on quad_n10 / quad_n11",
  "computed": f"blocked/msolve quad_n10 {q10['ratio_bm']:.6f}; quad_n11 {q11['ratio_bm']:.6f}",
  "definition": "median blocked / median msolve (process wall ms), 3 rounds", "files": files_ab,
  "status": "match" if (ok(q10['ratio_bm'], 0.78) and ok(q11['ratio_bm'], 0.75)) else "mismatch"},
 {"claimed": "1.11-1.49x vs msolve on quad_n8, quad_n9, cubic_n5, cubic_n6",
  "computed": f"blocked/msolve quad_n8 {stats['quad_n8_p65521']['ratio_bm']:.6f}, quad_n9 {q9['ratio_bm']:.6f}, cubic_n5 {stats['cubic_n5_p31']['ratio_bm']:.6f}, cubic_n6 {stats['cubic_n6_p65521']['ratio_bm']:.6f}; min {min(band):.6f} max {max(band):.6f}",
  "definition": "median blocked / median msolve", "files": files_ab,
  "status": "match" if (round(min(band), 2) == 1.11 and round(max(band), 2) == 1.49) else "mismatch"},
 {"claimed": "18 F_p systems (13 standard + 5 large), 1 thread, 3 interleaved rounds",
  "computed": f"distinct cases {len(cases)}; rounds {rounds}; rows {len(rows)}",
  "definition": "count of distinct case names and rounds in ab_full.tsv", "files": files_ab,
  "status": "match" if (len(cases) == 18 and len(rounds) == 3) else "mismatch"},
 {"claimed": "no effect seen at 4 threads (exploratory)",
  "computed": "not computed", "definition": "no 4-thread records in the study directory",
  "files": [AB, TAB], "status": "no_raw_in_repo"},
]

ident = {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
         "files": [AB], "status": "match" if (n_cmp == n_eq and n_cmp > 0) else "mismatch"}

OFF = re.compile(r"drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch", re.I)
AA = re.compile(r"A/A|A-A|null|negative control|control", re.I)
ISO = re.compile(r"isolat|PMU|isolated_bench|unqualified|contended|pinned|shared|hardware|L0|threads", re.I)
off, aa, iso = [], [], []
for p in STUDY_TEXT:
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
 "row_no": 30, "slug": "20261016-f4fp-blocked-pairs", "claims": claims, "identical": ident,
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": git_date(RUN),
 "first_raw_git_date": git_date(AB),
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/30.py",
 "notes": (f"table.tsv blocked/control vs recomputed: mismatches beyond 0.0015 = {tab_mismatch} of {len(cases)} cases. "
           f"msolve values in ab_full.tsv are wall ms (printf of ns/1000 then e-3). no PROTOCOL file; run_ab.sh used as protocol_git_date path; README is the only prose spec. "
           f"basis digests: ours_B1 vs ours_B0 per round and case (ab_full.tsv column 5)."),
}
print(json.dumps(out) if False else __import__('json').dumps(out))
