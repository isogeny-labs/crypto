#!/usr/bin/env python3
# Row 100 recompute: paper figures. No new measurement; reads the study's raw candidates and the paper text it cites. Prints one JSON object.
import json, os, re, subprocess
ROOT = '/home/user/crypto'; os.chdir(ROOT)
ROW = 100; SLUG = '20261014-paper-figures'
STUDY = 'paper/ecc2k130'
ABS = STUDY + '/abstract.txt'
EABS = STUDY + '/eprint-abstract.txt'
STATUS = STUDY + '/data/status_20261010T154157Z.json'
EMAP = STUDY + '/evidence-map.json'
GPU_TEX = STUDY + '/sec-gpu.tex'
FIRST_RAW = ABS

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

abs_txt = open(ABS, encoding='utf-8').read(); eabs = open(EABS, encoding='utf-8').read()
status = json.load(open(STATUS))
emap = json.load(open(EMAP))
gpu = open(GPU_TEX, encoding='utf-8').read()
claims = []
claims.append(dict(claimed='14.64 billion iterations/s, RTX PRO 6000, audited campaign build', computed='value appears in abstract.txt and eprint-abstract.txt; no per-run record in the committed raw set',
                   definition='search of raw candidates for the walk rate; status feed walk_rate is 0.0 (idle)', files=[ABS, EABS, STATUS, EMAP], status='no_raw_in_repo'))
claims.append(dict(claimed='15.88 billion iterations/s, best later build', computed='value appears in abstract.txt; the measured record cited in sec-gpu.tex:306 is not in the raw candidates',
                   definition='search of raw candidates for the best-build rate', files=[ABS, GPU_TEX], status='no_raw_in_repo'))
r229 = 14.64 / 0.064
claims.append(dict(claimed='about 229× the 2010 GTX 295 card', computed=f"{r229:.4f}",
                   definition='14.64 / 0.064 (both numbers stated in eprint-abstract.txt)', files=[EABS], status=sig_eq(229, 3, r229)))
r171 = 14.64 / 0.852294
claims.append(dict(claimed='ledger 17.2× total', computed=f"{r171:.4f}",
                   definition='14.64 / 0.852294, where 0.852294 is the bitsliced control rate stated in sec-gpu.tex:222 (not one of the raw candidates)', files=[GPU_TEX],
                   status=sig_eq(17.2, 3, r171)))
prod = 6.3 * 1.5 * 1.3 * 1.2 * 1.16
claims.append(dict(claimed='ledger factors split 6.3×, geometry 1.5×, layout 1.3×, clmad 1.2×, unattributed 1.16× (product vs 17.2)', computed=f"{prod:.4f}",
                   definition='arithmetic product of the listed factors compared with 17.2 (arithmetic consistency only; the factor measurements are not in the raw candidates)', files=[ABS, EABS],
                   status=sig_eq(17.2, 3, prod)))
claims.append(dict(claimed='derived ceiling about 24–26 billion iterations/s per card', computed='value stated in eprint-abstract.txt; the measured pipe rates it is derived from are not in the raw candidates',
                   definition='search of raw candidates for pipe rates', files=[EABS, EMAP], status='partial'))
# expected solve: 2^60.91 iterations at 15.88e9 iterations/s per card
iters = 2 ** 60.91
years = iters / 15.88e9 / (365.25 * 86400)
claims.append(dict(claimed='expected solve 4.3 RTX PRO 6000-years', computed=f"{years:.4f}",
                   definition='2^60.91 expected iterations (eprint-abstract.txt walk constant, lower end) / 15.88e9 iterations per second / seconds per year; the 15.88e9 rate is from abstract.txt',
                   files=[EABS, ABS], status=sig_eq(4.3, 2, years)))
claims.append(dict(claimed='five cards: about a year', computed=f"{years/5:.4f} years",
                   definition='expected solve in RTX-years divided by 5', files=[EABS], status='partial'))
naive_days = years / 1000 * 365.25
claims.append(dict(claimed='1,000 cards about 3.2 days (with cutoff-32 delay)', computed=f"{naive_days:.4f} days without delay term; the cutoff-32 delay model is not in the raw candidates",
                   definition='expected years / 1,000 in days; the delay term is not computed', files=[EABS], status='cannot_define'))
claims.append(dict(claimed='P-256 about 6e21 card-years', computed='not computed', definition='no P-256 iteration count is among the raw candidates',
                   files=[EABS, EMAP], status='cannot_define'))
off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control')
iso, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned|HOST')
print(json.dumps({'row_no': ROW, 'slug': SLUG, 'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'},
    'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso,
    'protocol_git_date': '', 'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/100.py',
    'notes': f"no PROTOCOL or spec in paper/ecc2k130 (protocol_git_date empty); status feed generated_at {status['generated_at']}, walk_rate.iterations_per_second {status['walk_rate']['iterations_per_second']}; evidence-map has {len(emap)} entries; off-repo hits {off_n}; AA hits {aa_n}; isolation-or-HOST hits {iso_n}"}, ensure_ascii=False))
