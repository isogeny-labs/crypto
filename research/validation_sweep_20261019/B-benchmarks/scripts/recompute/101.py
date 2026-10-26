#!/usr/bin/env python3
# Row 101 recompute: committed ecbench records and replay audit only. Prints one JSON object.
import json, os, re, subprocess
ROOT = '/home/user/crypto'; os.chdir(ROOT)
ROW = 101; SLUG = '20261013-prime-rootfinding-batchtable'
STUDY = 'research/ecbench_rootfinding_batchtable_20261013'
REC = STUDY + '/sessions/prime20-b64-batchtable/records.jsonl'
REPLAY = STUDY + '/replay-all.json'
PROTO = STUDY + '/PROTOCOL.md'
FIRST_RAW = STUDY + '/followup/cargo-test-release-lib-parallel.log'
A_ARM, B_ARM, RHO = 'pair-batch', 'pair-scalar', 'rho-neg'

def jl(p):
    with open(p) as f:
        return [json.loads(x) for x in f if x.strip()]
def sig_eq(claimed, sig, computed):
    rounded = float(f"{computed:.{sig}g}"); rel = abs(computed - claimed) / abs(claimed)
    return 'match' if (rounded == claimed or rel <= 0.01) else 'mismatch'
def gitdate(path):
    out = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True, cwd=ROOT).stdout.strip()
    return out.splitlines()[-1] if out else ''
def scan(pat, cap=30):
    hits = []; n = 0
    for dp, dn, fn in os.walk(STUDY):
        for f in sorted(fn):
            if f.endswith(('.md', '.typ', '.txt', '.tex')):
                p = os.path.join(dp, f)
                with open(p, encoding='utf-8', errors='replace') as fh:
                    for i, line in enumerate(fh, 1):
                        if re.search(pat, line, re.I):
                            n += 1
                            if len(hits) < cap: hits.append(f"{p}:{i}: {line.strip()}")
    return hits, n

recs = jl(REC)
def sel(arm, measured=True):
    return [r for r in recs if r['arm'] == arm and (not r['warmup'] if measured else True)]
A = sel(A_ARM); B = sel(B_ARM); RH = sel(RHO)
claims = []; notes = []
for arm, lst in ((A_ARM, A), (B_ARM, B)):
    v = sum(1 for r in lst if r['outcome']['status'] == 'verified')
    claims.append(dict(claimed=f"24/24 verified measured per arm ({arm})", computed=f"{v}/{len(lst)}",
                       definition='outcome.status verified among warmup false records', files=[REC],
                       status='match' if v == len(lst) == 24 else 'mismatch'))
# exact paired outputs including warmups
pa = {(r['workload']['workload_id'], r['round'], r['warmup']): r for r in recs if r['arm'] == A_ARM}
pb = {(r['workload']['workload_id'], r['round'], r['warmup']): r for r in recs if r['arm'] == B_ARM}
keys = sorted(set(pa) & set(pb), key=str)
eq = sum(1 for k in keys if pa[k]['outcome']['recovered'] == pb[k]['outcome']['recovered']
         and [p['gae'] for p in pa[k]['phases']] == [p['gae'] for p in pb[k]['phases']])
claims.append(dict(claimed='32/32 exact paired outputs and counted phases including warmups', computed=f"{eq}/{len(keys)}",
                   definition='pairs keyed on (workload_id, round, warmup) with equal recovered value and equal per-phase gae', files=[REC],
                   status='match' if eq == len(keys) == 32 else 'mismatch'))
ma = sum(r['cost']['total_gae'] for r in A) / len(A); mb = sum(r['cost']['total_gae'] for r in B) / len(B)
mr = sum(r['cost']['total_gae'] for r in RH) / len(RH)
claims.append(dict(claimed='18,633.042 mean cold GAE (batch arm)', computed=f"{ma:.3f}", definition='mean cost.total_gae over measured pair-batch runs', files=[REC],
                   status=sig_eq(18633.042, 8, ma)))
claims.append(dict(claimed='batch/scalar 1.000 mean cold GAE', computed=f"{ma/mb:.6f}", definition='mean cost.total_gae pair-batch / pair-scalar over measured runs', files=[REC],
                   status=sig_eq(1.000, 4, ma / mb)))
claims.append(dict(claimed='batch/rho counted ratio 16.229', computed=f"{ma/mr:.6f}", definition='mean cost.total_gae pair-batch / rho-neg over measured runs', files=[REC],
                   status=sig_eq(16.229, 5, ma / mr)))
# inversion counts and setup wall: report the keys present, do not invent
inv = {}
for r in A[:1]:
    for sect in ('counters', 'field_ops', 'solver', 'detail'):
        d = r.get(sect) or {}
        for k, v in d.items():
            if 'invers' in k.lower(): inv[f'{sect}.{k}'] = v
notes.append(f"inversion-like keys in first pair-batch record: {json.dumps(inv)}")
for arm in (A_ARM, B_ARM):
    pass
setup = {}
for arm, lst in ((A_ARM, A), (B_ARM, B)):
    vals = [v for r in lst for k, v in ((r.get('online') or {}).get('phases_ns') or {}).items() if 'setup' in k.lower()]
    setup[arm] = (sum(vals) / len(vals) / 1000.0, len(vals)) if vals else None
claims.append(dict(claimed='mean setup wall 243.226 µs (batch) vs 128.380 µs (scalar)',
                   computed=json.dumps(setup), definition='mean of online.phases_ns entries whose name contains setup, divided by 1000, measured runs',
                   files=[REC], status='cannot_define' if not all(setup.values()) else ('match' if abs(setup[A_ARM][0]-243.226) < 0.01 and abs(setup[B_ARM][0]-128.380) < 0.01 else 'mismatch')))
claims.append(dict(claimed='source-derived table inversions 4,096 versus 64', computed='see notes', definition='no inversion counter with a matching name was found in the committed records unless listed in notes',
                   files=[REC], status='cannot_define' if not inv else 'partial'))
rep = json.load(open(REPLAY))['replays']
n_id = sum(1 for x in rep if x.get('reproduced') and x.get('detail') == 'identical')
claims.append(dict(claimed='96/96 identical measured replays', computed=f"{n_id}/{len(rep)}", definition='replays[] reproduced and detail identical in replay-all.json', files=[REPLAY],
                   status='match' if n_id == len(rep) == 96 else 'mismatch'))
off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control|rho-neg-aa')
iso, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned|shared')
notes.append(f"off-repo hits {off_n}; AA hits {aa_n}; isolation hits {iso_n}; replay records {len(rep)}")
print(json.dumps({'row_no': ROW, 'slug': SLUG, 'claims': claims,
    'identical': {'claimed': True, 'n_compared': len(rep), 'n_equal': n_id, 'n_differ': len(rep) - n_id, 'files': [REPLAY],
                  'status': 'match' if n_id == len(rep) else 'mismatch'},
    'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso,
    'protocol_git_date': gitdate(PROTO), 'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/101.py', 'notes': ' | '.join(notes)}, ensure_ascii=False))
