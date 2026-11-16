#!/usr/bin/env python3
# Row 97 recompute: reads committed raw records only. Prints one JSON object.
import json, os, re, subprocess, glob
ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 97
SLUG = '20261014-gaudry-k4-d4-cost'
STUDY = 'research/gaudry_quartic_d4_cost_20261014'
RAW = STUDY + '/raw/'
PROTO = STUDY + '/PROTOCOL.md'
FIRST_RAW = STUDY + '/pilot/p29_new.jsonl'

def jl(p):
    with open(p) as f:
        return [json.loads(x) for x in f if x.strip()]

def solved(recs):
    out = {}
    dup = 0
    for r in recs:
        s = r.get('solve', {})
        if s.get('outcome') == 'solved' and 'total_muls' in s:
            if r['target'] in out:
                dup += 1
            out[r['target']] = s['total_muls']
    return out, dup

def steady(recs):
    rep = any(r.get('solve', {}).get('replay') == 'Replayed' for r in recs)
    out = {}
    for r in recs:
        s = r.get('solve', {})
        if s.get('outcome') != 'solved' or 'total_muls' not in s:
            continue
        if rep and s.get('replay') != 'Replayed':
            continue
        out[r['target']] = s['total_muls']
    return out

def mean(xs):
    return sum(xs) / len(xs)

def sig_eq(claimed, sig, computed):
    rounded = float(f"{computed:.{sig}g}")
    rel = abs(computed - claimed) / abs(claimed)
    return 'match' if (rounded == claimed or rel <= 0.01) else 'mismatch'

def gitdate(path):
    out = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True, cwd=ROOT).stdout.strip()
    return out.splitlines()[-1] if out else ''

def scan(pat, cap=30):
    hits = []
    n = 0
    for dp, dn, fn in os.walk(STUDY):
        for f in sorted(fn):
            if f.endswith(('.md', '.typ', '.txt', '.tex')):
                p = os.path.join(dp, f)
                with open(p, encoding='utf-8', errors='replace') as fh:
                    for i, line in enumerate(fh, 1):
                        if re.search(pat, line, re.I):
                            n += 1
                            if len(hits) < cap:
                                hits.append(f"{p}:{i}: {line.strip()}")
    return hits, n

claims = []
notes = []

# Committed and new ablation arms at p = 269 (raw/abl_*.jsonl)
com = steady(jl(RAW + 'abl_committed.jsonl'))
new = steady(jl(RAW + 'abl_wcsr.jsonl'))
sp = steady(jl(RAW + 'abl_s.jsonl'))
scr = steady(jl(RAW + 'abl_scr.jsonl'))
wcs_new = new
notes.append(f"abl_committed solved records with total_muls: {len(com)}; abl_wcsr: {len(new)}; abl_s: {len(sp)}; abl_scr: {len(scr)}")

def ratio_shared(a, b):
    keys = sorted(set(a) & set(b))
    return keys, mean([a[k] for k in keys]) / mean([b[k] for k in keys])

c_mean = mean(list(com.values()))
n_mean = mean(list(new.values()))
claims.append(dict(claimed='2.55·10¹⁰ (committed C4, p=269)', computed=f"{c_mean:.6e}",
                   definition='mean total_muls over solved abl_committed records (p=269)',
                   files=[RAW + 'abl_committed.jsonl'],
                   status=sig_eq(2.55e10, 3, c_mean)))
claims.append(dict(claimed='6.89·10⁷ (new C4, p=269)', computed=f"{n_mean:.6e}",
                   definition='mean total_muls over solved abl_wcsr records (p=269)',
                   files=[RAW + 'abl_wcsr.jsonl'],
                   status=sig_eq(6.89e7, 3, n_mean)))
k, r = ratio_shared(com, new)
claims.append(dict(claimed='369.8× committed/new paired', computed=f"{r:.4f} (n_pairs={len(k)})",
                   definition='ratio of means of total_muls over shared targets, committed/abl_wcsr',
                   files=[RAW + 'abl_committed.jsonl', RAW + 'abl_wcsr.jsonl'],
                   status=sig_eq(369.8, 4, r)))
k, r = ratio_shared(com, sp)
claims.append(dict(claimed='26.4× sparsest pivots alone', computed=f"{r:.4f} (n_pairs={len(k)})",
                   definition='ratio of means over shared targets, abl_committed/abl_s',
                   files=[RAW + 'abl_committed.jsonl', RAW + 'abl_s.jsonl'],
                   status=sig_eq(26.4, 3, r)))
k, r = ratio_shared(scr, new)
claims.append(dict(claimed='10.95× weighted form at equal linear algebra', computed=f"{r:.4f} (n_pairs={len(k)})",
                   definition='ratio of means over shared targets, abl_scr/abl_wcsr',
                   files=[RAW + 'abl_scr.jsonl', RAW + 'abl_wcsr.jsonl'],
                   status=sig_eq(10.95, 4, r)))

# ecbench session k4-p29-d4cost
SES = STUDY + '/sessions/k4-p29-d4cost/records.jsonl'
ses = jl(SES)
S = {}
for rec in ses:
    S[(rec['arm'], rec['workload']['target_index'])] = rec['cost']['s']
rho = {t: S[('rho-neg', t)] for t in (0, 1)}
com_s = {t: S[('ic-d4-committed', t)] for t in (0, 1)}
ch_s = {t: S[('ic-d4-chains', t)] for t in (0, 1)}
claims.append(dict(claimed='6.57·10⁸ ecbench S committed (target 0)', computed=f"{com_s[0]:.6e}",
                   definition='cost.s of ic-d4-committed, target_index 0', files=[SES],
                   status=sig_eq(6.57e8, 3, com_s[0])))
claims.append(dict(claimed='7.23·10⁸ ecbench S committed (target 1)', computed=f"{com_s[1]:.6e}",
                   definition='cost.s of ic-d4-committed, target_index 1', files=[SES],
                   status=sig_eq(7.23e8, 3, com_s[1])))
claims.append(dict(claimed='398× ecbench committed/new (target 0)', computed=f"{com_s[0]/ch_s[0]:.4f}",
                   definition='cost.s ratio committed/chains, target 0', files=[SES],
                   status=sig_eq(398, 3, com_s[0] / ch_s[0])))
claims.append(dict(claimed='406× ecbench committed/new (target 1)', computed=f"{com_s[1]/ch_s[1]:.4f}",
                   definition='cost.s ratio committed/chains, target 1', files=[SES],
                   status=sig_eq(406, 3, com_s[1] / ch_s[1])))
ic_mean = mean([ch_s[0], ch_s[1]])
rho_mean = mean([rho[0], rho[1]])
claims.append(dict(claimed='1.39·10⁶× rho (new IC vs rho)', computed=f"{ic_mean/rho_mean:.6e}",
                   definition='arithmetic mean cost.s of ic-d4-chains over 2 targets / arithmetic mean cost.s of rho-neg over 2 targets (the ecbench table aggregation)',
                   files=[SES], status=sig_eq(1.39e6, 3, ic_mean / rho_mean)))

# Identical / exact claim: per shared target, rational solution counts and decomposition agreement
ident_n = 0; ident_eq = 0
A = {r['target']: r for r in jl(RAW + 'abl_committed.jsonl') if r.get('solve', {}).get('outcome') == 'solved'}
B = {r['target']: r for r in jl(RAW + 'abl_wcsr.jsonl') if r.get('solve', {}).get('outcome') == 'solved'}
for t in sorted(set(com) & set(new)):
    ident_n += 1
    a, b = A[t], B[t]
    if a['rational_solutions'] == b['rational_solutions'] and a['decompositions_agree'] and b['decompositions_agree']:
        ident_eq += 1
# Count of closed solves that agree with MITM across all raw and pilot files
closed = 0; agree = 0
for p in glob.glob(STUDY + '/raw/*.jsonl') + glob.glob(STUDY + '/pilot/*.jsonl'):
    for r in jl(p):
        if r.get('solve', {}).get('outcome') == 'solved' and 'decompositions_agree' in r:
            closed += 1
            agree += 1 if r['decompositions_agree'] else 0
notes.append(f"closed solves (solve.outcome=solved) with decompositions_agree field across raw/*.jsonl and pilot/*.jsonl: {closed}; of which decompositions_agree true: {agree}")
notes.append(f"ecbench session k4-p29-d4cost: recovered values per arm and target: " + "; ".join(f"{r['arm']}/t{r['workload']['target_index']} recovered={r['outcome']['recovered']} matches_target={r['outcome']['matches_target']}" for r in ses))

# Off-repo, A/A, isolation, git
off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control|rho-neg-aa')
iso, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned')
store_names = sorted(set(re.findall(r'store:([A-Za-z0-9_\-]+)', ' '.join(off))))
idx = ROOT + '/store/index.json'
store_status = {}
if os.path.exists(idx):
    txt = open(idx, encoding='utf-8', errors='replace').read()
    for s in store_names:
        store_status[s] = 'yes' if s in txt else 'no'
else:
    for s in store_names:
        store_status[s] = 'no (store/index.json absent)'
notes.append(f"off-repo line hits (total {off_n}, shown {len(off)}); AA line hits total {aa_n}; isolation line hits total {iso_n} (shown up to 30 each); store index: {store_status}")

out = {
    'row_no': ROW,
    'slug': SLUG,
    'claims': claims,
    'identical': {
        'claimed': True,
        'n_compared': ident_n,
        'n_equal': ident_eq,
        'n_differ': ident_n - ident_eq,
        'files': [RAW + 'abl_committed.jsonl', RAW + 'abl_wcsr.jsonl'],
        'status': 'match' if ident_n and ident_eq == ident_n else 'mismatch'
    },
    'offrepo_refs': off,
    'aa_lines': aa,
    'isolation_lines': iso,
    'protocol_git_date': gitdate(PROTO),
    'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/97.py',
    'notes': ' | '.join(notes),
}
print(json.dumps(out, ensure_ascii=False))
