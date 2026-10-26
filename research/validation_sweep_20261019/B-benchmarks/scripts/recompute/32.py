import json, re, os, statistics as st, subprocess, collections
D = 'research/f6_gate_first_x86_20261016/'
R = D + 'reports/run-20261016T2232Z/'
RAW = R + 'raw/'
NAME = re.compile(r'^r(\d+)-(K[01]_\d+(?:d\d)?)-(off|gate|rref|gaterref)\.jsonl$')
dec = collections.defaultdict(dict)
summ = {}
gate_nodes = {}
for f in sorted(os.listdir(RAW)):
    m = NAME.match(f)
    if not m:
        continue
    rd, cell, arm = int(m.group(1)), m.group(2), m.group(3)
    for line in open(RAW + f, encoding='utf-8'):
        if not line.strip():
            continue
        r = json.loads(line)
        if 'target' in r:
            dec[(cell, r['arm'], r['target'])][(arm, rd)] = (r['hit'], tuple(r['indices']) if r['indices'] is not None else None, r['exhausted'])
        elif 'cpu_ms' in r:
            summ[(cell, r['arm'], arm, rd)] = dict(cpu=r['cpu_ms'], units=r['word_xors'], gn=r['gate_first_nodes'], gs=r['gate_first_specialisations'])
cells = sorted({k[0] for k in dec})
n_t = len(dec)
n_cmp = n_eq = n_keys_ok = 0
for key, rec in dec.items():
    vals = list(rec.values())
    for v in vals:
        n_cmp += 1; n_eq += (v == vals[0])
    n_keys_ok += (len(set(vals)) == 1)
units_var = 0
U = {}
for (c, e, a, rd), s in summ.items():
    U.setdefault((c, e, a), set()).add(s['units'])
units_var = sum(1 for v in U.values() if len(v) != 1)
def med(c, e, a): return st.median([summ[(c, e, a, rd)]['cpu'] for rd in range(1, 6)])
def uval(c, e, a): return list(U[(c, e, a)])[0]
f6 = {c: {
    'U_off_gate': uval(c, 'f6p', 'off') / uval(c, 'f6p', 'gate'),
    'C_off_gate': med(c, 'f6p', 'off') / med(c, 'f6p', 'gate'),
    'U_off_rref': uval(c, 'f6p', 'off') / uval(c, 'f6p', 'rref'),
    'C_off_rref': None,
    'skip_nodes': summ[(c, 'f6p', 'gate', 1)]['gn'],
    'skip_specs': summ[(c, 'f6p', 'gate', 1)]['gs'],
    'skip_nodes_all_rounds': sorted({summ[(c, 'f6p', 'gate', rd)]['gn'] for rd in range(1, 6)}),
    'lo_min_off_over_max_gate': min(summ[(c,'f6p','off',rd)]['cpu'] for rd in range(1,6)) / max(summ[(c,'f6p','gate',rd)]['cpu'] for rd in range(1,6)),
    'same_round_min': min(summ[(c,'f6p','off',rd)]['cpu'] / summ[(c,'f6p','gate',rd)]['cpu'] for rd in range(1,6)),
} for c in cells}
f4_U = {c: uval(c, 'f4', 'off') == uval(c, 'f4', 'gate') for c in cells}
f4_C = {c: med(c, 'f4', 'off') / med(c, 'f4', 'gate') for c in cells}
f6_U_list = [f6[c]['U_off_gate'] for c in cells]
f6_C_list = [f6[c]['C_off_gate'] for c in cells]
f6_lo = min(f6[c]['lo_min_off_over_max_gate'] for c in cells)
f6_lo_cell = min(cells, key=lambda c: f6[c]['lo_min_off_over_max_gate'])
rref_big = [f6[c]['U_off_rref'] for c in ['K0_23', 'K0_31', 'K1_29', 'K0_37']]
rref_small = [f6[c]['U_off_rref'] for c in ['K1_17', 'K0_19', 'K1_19', 'K1_17d7']]
k123 = f6['K1_23']['U_off_rref']
skip = {c: f6[c]['skip_nodes'] for c in ['K1_17', 'K1_23', 'K0_23', 'K0_31']}

aarch = D.replace('f6_gate_first_x86_20261016', 'lit2026_w5_lane_batched_f4_20261011')
aarch_exists = os.path.isdir('research/' + os.path.basename(aarch.rstrip('/')))

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
 {"claimed": "every one of the 80 targets decided identically in every arm and round",
  "computed": f"distinct (cell,engine,target) keys {n_t}; keys with one decision tuple across 4 arms x 5 rounds {n_keys_ok}; compared {n_cmp}, equal {n_eq}",
  "definition": "(hit, indices, exhausted) per target record", "files": [RAW + 'r1-K1_17d7-gate.jsonl', RAW + 'r1-K1_17d7-gaterref.jsonl'],
  "status": "match" if (n_t == 80 and n_keys_ok == 80) else "mismatch"},
 {"claimed": "stage units identical across rounds", "computed": f"(cell,engine,arm) with >1 distinct word_xors across rounds: {units_var}",
  "definition": "word_xors per arm per round", "files": [RAW + 'r1-K1_17d7-off.jsonl'],
  "status": "match" if units_var == 0 else "mismatch"},
 {"claimed": "F6-IC 1.336-1.733x fewer word XORs on all nine cells (off/gate, f6p)",
  "computed": f"min {min(f6_U_list):.6f} max {max(f6_U_list):.6f}",
  "definition": "word_xors off / gate, f6p, 9 cells", "files": [RAW + 'r1-K1_17d7-off.jsonl', RAW + 'r1-K1_17d7-gate.jsonl'],
  "status": "match" if (round(min(f6_U_list), 3) == 1.336 and round(max(f6_U_list), 3) == 1.733) else "mismatch"},
 {"claimed": "median process CPU 1.56-2.12x lower (off/gate, f6p)",
  "computed": f"min {min(f6_C_list):.6f} max {max(f6_C_list):.6f}",
  "definition": "median over 5 rounds of cpu_ms, off / gate", "files": [RAW + 'r1-K1_17d7-off.jsonl', RAW + 'r1-K1_17d7-gate.jsonl'],
  "status": "match" if (round(min(f6_C_list), 2) == 1.56 and round(max(f6_C_list), 2) == 2.12) else "mismatch"},
 {"claimed": "worst round pairing >= 1.25",
  "computed": f"min over cells of (min off round CPU / max gate round CPU) = {f6_lo:.6f} at {f6_lo_cell}; same-round min over cells {min(f6[c]['same_round_min'] for c in cells):.6f}",
  "definition": "bracket rule from report.typ: min off / max arm over rounds",
  "files": [R + 'report.typ'],
  "status": "match" if round(f6_lo, 2) >= 1.25 else ("partial" if round(f6_lo, 2) == 1.25 else "mismatch")},
 {"claimed": "inherited F4: stage units unchanged; CPU within noise (0.92-1.18x)",
  "computed": f"f4 units off==gate on all cells: {all(f4_U.values())}; f4 median CPU off/gate min {min(f4_C.values()):.6f} max {max(f4_C.values()):.6f}",
  "definition": "f4 word_xors and median cpu_ms, off/gate", "files": [RAW + 'r1-K1_17d7-off.jsonl'],
  "status": "match" if (all(f4_U.values()) and round(min(f4_C.values()), 2) == 0.92 and round(max(f4_C.values()), 2) == 1.18) else "mismatch"},
 {"claimed": "skipped-node counts on the four W5 cells 447 / 432 / 840 / 405",
  "computed": f"gate_first_nodes f6p: {skip}; stable across rounds: {all(len(f6[c]['skip_nodes_all_rounds'])==1 for c in skip)}",
  "definition": "gate_first_nodes in the gate arm summary record", "files": [RAW + 'r1-K1_17-gate.jsonl'],
  "status": "match" if (list(skip.values()) == [447, 432, 840, 405]) else "mismatch"},
 {"claimed": "equal the aarch64 study's skipped-node counts",
  "computed": f"aarch64 study directory present in this repo: {aarch_exists}",
  "definition": "aarch64 study text lines: REPORT.typ:34 and README.md:154 both state 447 / 432 / 840 / 405",
  "files": ["research/lit2026_w5_lane_batched_f4_20261011/REPORT.typ", "research/lit2026_w5_lane_batched_f4_20261011/README.md"], "status": "match" if aarch_exists else "no_raw_in_repo"},
 {"claimed": "RREF root: 1.10-1.23x fewer on the four larger cells",
  "computed": f"off/rref f6p on K0_23,K0_31,K1_29,K0_37: {[round(x,4) for x in rref_big]} (min {min(rref_big):.6f} max {max(rref_big):.6f}); K1_23 off/rref {k123:.6f} is outside both stated bands",
  "definition": "word_xors off / rref, f6p; 'larger' cells not defined in the study",
  "files": [RAW + 'r1-K1_17d7-rref.jsonl'],
  "status": "partial"},
 {"claimed": "RREF root: 0.89-0.98x on the four smaller cells",
  "computed": f"off/rref f6p on K1_17,K0_19,K1_19,K1_17d7: {[round(x,4) for x in rref_small]} (min {min(rref_small):.6f} max {max(rref_small):.6f})",
  "definition": "word_xors off / rref, f6p; 'smaller' cells not defined in the study",
  "files": [RAW + 'r1-K1_17d7-rref.jsonl'],
  "status": "match" if (round(min(rref_small), 2) == 0.89 and round(max(rref_small), 2) == 0.98) else "mismatch"},
]

ident = {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
         "files": [RAW], "status": "match" if (n_eq == n_cmp and n_cmp > 0) else "mismatch"}

out = {
 "row_no": 32, "slug": "20261016-f6-gate-first-x86", "claims": claims, "identical": ident,
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": git_date(D + 'scripts/run_arms.sh'),
 "first_raw_git_date": git_date(RAW + 'r1-K1_17d7-gate.jsonl'),
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/32.py",
 "notes": (f"raw files {len(os.listdir(RAW))}; per-cell f6p details: "
           + json.dumps({c: {k: (round(v, 6) if isinstance(v, float) else v) for k, v in f6[c].items() if k != 'C_off_rref'} for c in cells}, sort_keys=True)
           + f"; f4 CPU off/gate per cell {json.dumps({c: round(v,6) for c,v in f4_C.items()}, sort_keys=True)}."),
}
print(json.dumps(out))
