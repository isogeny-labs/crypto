#!/usr/bin/env python3
# Row 102 recompute: committed ecbench records and replay audit only. Prints one JSON object.
import json, os, re, subprocess
ROOT = '/home/user/crypto'; os.chdir(ROOT)
ROW = 102; SLUG = '20261013-prime-rootfinding-arity'
STUDY = 'research/ecbench_rootfinding_arity_20261013'
REC = STUDY + '/sessions/prime20-b64-m2-vs-m3/records.jsonl'
REPLAY = STUDY + '/replay-all.json'
PROTO = STUDY + '/PROTOCOL.md'
FIRST_RAW = STUDY + '/compare-rho-m2.log'
M2, M3, RHO, AUX = 'pair-m2', 'pair-m3', 'rho-neg', 'rootfind-m3'

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
def meas(arm): return [r for r in recs if r['arm'] == arm and not r['warmup']]
M2r, M3r, RHr, AUr = meas(M2), meas(M3), meas(RHO), meas(AUX)
claims = []; notes = []
for arm, lst in ((M2, M2r), (M3, M3r)):
    v = sum(1 for r in lst if r['outcome']['status'] == 'verified')
    claims.append(dict(claimed=f"24/24 verified measured per pair-table arm ({arm})", computed=f"{v}/{len(lst)}",
                       definition='outcome.status verified among warmup false records', files=[REC], status='match' if v == len(lst) == 24 else 'mismatch'))
mm2 = sum(r['cost']['total_gae'] for r in M2r) / len(M2r)
mm3 = sum(r['cost']['total_gae'] for r in M3r) / len(M3r)
mrho = sum(r['cost']['total_gae'] for r in RHr) / len(RHr)
claims.append(dict(claimed='m3/m2 counted-GAE ratio 1.431345', computed=f"{mm3/mm2:.6f}",
                   definition='mean cost.total_gae pair-m3 / pair-m2 over measured runs (95% cluster interval not recomputed)', files=[REC],
                   status='match' if abs(mm3/mm2 - 1.431345) / 1.431345 <= 0.01 else 'mismatch'))
claims.append(dict(claimed='interval 1.224624–1.716117 (m3/m2)', computed='not computed',
                   definition='95% target-cluster interval: resampling seed and count are not stated in the committed records', files=[REC], status='cannot_define'))
claims.append(dict(claimed='m2/rho counted ratio 20.029', computed=f"{mm2/mrho:.6f}",
                   definition='mean cost.total_gae pair-m2 / rho-neg over measured runs', files=[REC],
                   status=sig_eq(20.029, 5, mm2 / mrho)))
err = sum(1 for r in AUr if r['outcome']['status'] == 'error')
claims.append(dict(claimed='rootfind auxiliary arm: 32 setup errors', computed=f"{err} error outcomes among {len(AUr)} measured rootfind-m3 records; {sum(1 for r in recs if r['arm']==AUX and r['warmup'])} warmup records",
                   definition='count of outcome.status == error for arm rootfind-m3 (warmup and measured)', files=[REC],
                   status='match' if sum(1 for r in recs if r['arm']==AUX and r['outcome']['status']=='error') == 32 else 'mismatch'))
rep = json.load(open(REPLAY))['replays']
n_id = sum(1 for x in rep if x.get('reproduced') and x.get('detail') == 'identical')
claims.append(dict(claimed='96/96 identical measured replays', computed=f"{n_id}/{len(rep)}", definition='replays[] reproduced and detail identical in replay-all.json', files=[REPLAY],
                   status='match' if n_id == len(rep) == 96 else 'mismatch'))
claims.append(dict(claimed='24/24 verified measured per pair-table arm (pair-m3)', computed=f"{sum(1 for r in M3r if r['outcome']['status']=='verified')}/{len(M3r)}",
                   definition='duplicate of the per-arm check, pair-m3 only', files=[REC], status='match' if len(M3r) == 24 and all(r['outcome']['status']=='verified' for r in M3r) else 'mismatch'))
off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control|rho-neg-aa')
iso, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned|shared')
notes.append(f"measured n: pair-m2={len(M2r)} pair-m3={len(M3r)} rho-neg={len(RHr)} rootfind-m3={len(AUr)}; off-repo hits {off_n}; AA hits {aa_n}; isolation hits {iso_n}")
print(json.dumps({'row_no': ROW, 'slug': SLUG, 'claims': claims,
    'identical': {'claimed': True, 'n_compared': len(rep), 'n_equal': n_id, 'n_differ': len(rep) - n_id, 'files': [REPLAY],
                  'status': 'match' if n_id == len(rep) else 'mismatch'},
    'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso,
    'protocol_git_date': gitdate(PROTO), 'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/102.py', 'notes': ' | '.join(notes)}, ensure_ascii=False))
