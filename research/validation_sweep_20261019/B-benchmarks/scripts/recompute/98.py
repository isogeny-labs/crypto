#!/usr/bin/env python3
# Row 98 recompute: reads committed raw records and audit files only. Prints one JSON object.
import json, os, re, subprocess
ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 98
SLUG = '20261013-gaudry-k4-ecbench'
STUDY = 'research/gaudry_quartic_d4_ecbench_20261013'
REC = STUDY + '/sessions/k4-p29/records.jsonl'
AUD = STUDY + '/audits/k4-p29.audit.json'
PROTO = STUDY + '/PROTOCOL.md'
FIRST_RAW = STUDY + '/sessions/k4-p29-zero/host.json'

def jl(p):
    with open(p) as f:
        return [json.loads(x) for x in f if x.strip()]

def sig_eq(claimed, sig, computed):
    rounded = float(f"{computed:.{sig}g}")
    rel = abs(computed - claimed) / abs(claimed)
    return 'match' if (rounded == claimed or rel <= 0.01) else 'mismatch'

def gitdate(path):
    out = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True, cwd=ROOT).stdout.strip()
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
                            if len(hits) < cap:
                                hits.append(f"{p}:{i}: {line.strip()}")
    return hits, n

recs = jl(REC)
by = {r['arm']: r for r in recs}
rho = by['rho-neg']; ic = by['ic-k4-d4']
claims = []
rel = [p for p in ic['phases'] if p['name'] == 'relations'][0]
trials = rel['native']['trials']; relations = rel['native']['relations']
claims.append(dict(claimed='7.17·10⁸ IC S', computed=f"{ic['cost']['s']:.6e}",
                   definition='cost.s of arm ic-k4-d4 (single record)', files=[REC],
                   status=sig_eq(7.17e8, 3, ic['cost']['s'])))
claims.append(dict(claimed='rho 1.065', computed=f"{rho['cost']['s']:.6f}",
                   definition='cost.s of arm rho-neg (single record)', files=[REC],
                   status=sig_eq(1.065, 4, rho['cost']['s'])))
claims.append(dict(claimed='6.7·10⁸× rho', computed=f"{ic['cost']['s']/rho['cost']['s']:.6e}",
                   definition='cost.s ratio ic-k4-d4 / rho-neg (single record each)', files=[REC],
                   status=sig_eq(6.7e8, 2, ic['cost']['s'] / rho['cost']['s'])))
claims.append(dict(claimed='633 trials', computed=str(trials),
                   definition='phases[relations].native.trials', files=[REC],
                   status='match' if trials == 633 else 'mismatch'))
claims.append(dict(claimed='8 relations', computed=str(relations),
                   definition='phases[relations].native.relations', files=[REC],
                   status='match' if relations == 8 else 'mismatch'))
rel_share = rel['gae'] / ic['cost']['total_gae']
claims.append(dict(claimed='relations ≈ 100 % of S', computed=f"{rel_share*100:.7f} % (relations gae / total gae)",
                   definition='phases[relations].gae / cost.total_gae', files=[REC],
                   status='match' if rel_share >= 0.99 else 'mismatch'))
claims.append(dict(claimed='all runs verified', computed=f"{sum(1 for r in recs if r['outcome']['status']=='verified')}/{len(recs)} verified",
                   definition='count of outcome.status == verified', files=[REC],
                   status='match' if all(r['outcome']['status'] == 'verified' for r in recs) else 'mismatch'))
aud = json.load(open(AUD))
rep = aud['replays']
n_id = sum(1 for x in rep if x['reproduced'] and x['detail'] == 'identical')
claims.append(dict(claimed='3/3 replays identical', computed=f"{n_id}/{len(rep)} replays identical",
                   definition='audits/k4-p29.audit.json replays[].reproduced and detail == identical', files=[AUD],
                   status='match' if n_id == len(rep) == 3 else 'mismatch'))
claims.append(dict(claimed='within 2× of §11.16 model', computed='', definition='theoretical S from §11.16 is not in the committed raw records of this study',
                   files=[REC], status='cannot_define'))
claims.append(dict(claimed='633 trials (headline count, repeat)', computed='', definition='duplicate of the above, not computed', files=[REC], status='cannot_define'))

off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control|rho-neg-aa')
iso, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned|cpus')
notes = [f"off-repo hits total {off_n}; AA hits total {aa_n}; isolation hits total {iso_n} (each shown up to 30)",
         f"relations trials={trials} relations={relations}; rho S={rho['cost']['s']}; ic S={ic['cost']['s']}",
         f"record status: {[r['outcome']['status'] for r in recs]}; records audited {aud['records']}"]
out = {
    'row_no': ROW, 'slug': SLUG, 'claims': claims,
    'identical': {'claimed': True, 'n_compared': len(rep), 'n_equal': n_id, 'n_differ': len(rep) - n_id,
                  'files': [AUD], 'status': 'match' if n_id == len(rep) else 'mismatch'},
    'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso,
    'protocol_git_date': gitdate(PROTO), 'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/98.py',
    'notes': ' | '.join(notes),
}
print(json.dumps(out, ensure_ascii=False))
