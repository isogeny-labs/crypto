#!/usr/bin/env python3
# Row 103 recompute: committed ecbench records and replay audit only. Prints one JSON object.
import json, os, re, subprocess
ROOT = '/home/user/crypto'; os.chdir(ROOT)
ROW = 103; SLUG = '20261013-rootfinding-pair-size'
STUDY = 'research/ecbench_rootfinding_tuning_20261013'
REC = STUDY + '/sessions/prime20-b32-vs-b64/records.jsonl'
REPLAY = STUDY + '/replay-all.json'
PROTO = STUDY + '/PROTOCOL.md'
FIRST_RAW = STUDY + '/cargo-test-jv-cover.log'
B32, B64, RHO = 'pair-b32', 'pair-b64', 'rho-neg'

def jl(p):
    with open(p) as f:
        return [json.loads(x) for x in f if x.strip()]
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
def meas(arm): return [r for r in recs if r['arm'] == arm and not r['warmup']]
X, Y, R = meas(B32), meas(B64), meas(RHO)
claims = []; notes = []
for arm, lst in ((B32, X), (B64, Y)):
    v = sum(1 for r in lst if r['outcome']['status'] == 'verified')
    claims.append(dict(claimed=f"24/24 verified measured per arm ({arm})", computed=f"{v}/{len(lst)}",
                       definition='outcome.status verified among warmup false records', files=[REC], status='match' if v == len(lst) == 24 else 'mismatch'))
mx = sum(r['cost']['total_gae'] for r in X) / len(X)
my = sum(r['cost']['total_gae'] for r in Y) / len(Y)
mr = sum(r['cost']['total_gae'] for r in R) / len(R)
claims.append(dict(claimed='B64/B32 counted-GAE ratio 0.830225', computed=f"{my/mx:.6f}",
                   definition='mean cost.total_gae pair-b64 / pair-b32 over measured runs (interval not recomputed)', files=[REC],
                   status='match' if abs(my/mx - 0.830225) / 0.830225 <= 0.01 else 'mismatch'))
claims.append(dict(claimed='interval 0.697026–0.961546 (B64/B32)', computed='not computed',
                   definition='95% target-cluster interval: resampling seed and count are not stated in the committed records', files=[REC], status='cannot_define'))
claims.append(dict(claimed='B64/rho counted ratio 18.873', computed=f"{my/mr:.6f}",
                   definition='mean cost.total_gae pair-b64 / rho-neg over measured runs', files=[REC],
                   status='match' if abs(my/mr - 18.873) / 18.873 <= 0.01 else 'mismatch'))
rep = json.load(open(REPLAY))['replays']
n_id = sum(1 for x in rep if x.get('reproduced') and x.get('detail') == 'identical')
claims.append(dict(claimed='96/96 identical measured replays', computed=f"{n_id}/{len(rep)}", definition='replays[] reproduced and detail identical in replay-all.json', files=[REPLAY],
                   status='match' if n_id == len(rep) == 96 else 'mismatch'))
off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control|rho-neg-aa')
iso, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned|shared|14 logical')
notes.append(f"measured n: pair-b32={len(X)} pair-b64={len(Y)} rho-neg={len(R)}; off-repo hits {off_n}; AA hits {aa_n}; isolation hits {iso_n}")
print(json.dumps({'row_no': ROW, 'slug': SLUG, 'claims': claims,
    'identical': {'claimed': True, 'n_compared': len(rep), 'n_equal': n_id, 'n_differ': len(rep) - n_id, 'files': [REPLAY],
                  'status': 'match' if n_id == len(rep) else 'mismatch'},
    'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso,
    'protocol_git_date': gitdate(PROTO), 'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/103.py', 'notes': ' | '.join(notes)}, ensure_ascii=False))
