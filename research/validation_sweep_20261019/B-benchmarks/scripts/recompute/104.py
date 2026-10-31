#!/usr/bin/env python3
# Row 104 recompute: committed autolab evidence (workloads, isolation, verify) only. Prints one JSON object.
import json, os, re, statistics, subprocess
ROOT = '/home/user/crypto'; os.chdir(ROOT)
ROW = 104; SLUG = '20261014-koblitz-n71-n83-hosted-single-target'
STUDY = 'research/sat_factor_base_review_20260908/autolab/evidence/20261014-koblitz-n71-n83-hosted-single-target'
RUN71 = STUDY + '/autolab_runs/gha-38585913645-1'
RUN83 = STUDY + '/autolab_runs/gha-38597929540-1'
FIRST_RAW = RUN71 + '/artifacts/workloads.jsonl'

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

claims = []; notes = []
def run_stats(run):
    W = jl(run + '/artifacts/workloads.jsonl')
    on_ratio = []; ic_on = []; rho_on = []; cold = []
    for w in W:
        ic_ms = float(w['ic_record']['online_ms']); rho_ms = float(w['rho_record']['online_ms'])
        on_ratio.append(rho_ms / ic_ms); ic_on.append(ic_ms); rho_on.append(rho_ms)
        cold.append(float(w['ic']['wall_s']) / float(w['rho']['wall_s']))
    return dict(n=len(W), med_online=statistics.median(on_ratio), mean_ratio=(sum(rho_on) / len(rho_on)) / (sum(ic_on) / len(ic_on)),
                mean_rho=sum(rho_on) / len(rho_on), mean_ic=sum(ic_on) / len(ic_on),
                ic_slower=sum(1 for x in on_ratio if x < 1), med_cold=statistics.median(cold))
s71 = run_stats(RUN71); s83 = run_stats(RUN83)
notes.append(f"n71 stats: {json.dumps(s71)}"); notes.append(f"n83 stats: {json.dumps(s83)}")
W71 = RUN71 + '/artifacts/workloads.jsonl'; W83 = RUN83 + '/artifacts/workloads.jsonl'
claims.append(dict(claimed='online speedup rho/IC median 7.29x (n=71)', computed=f"{s71['med_online']:.6f}",
                   definition='median over 16 workloads of rho_record.online_ms / ic_record.online_ms', files=[W71], status=sig_eq(7.29, 3, s71['med_online'])))
claims.append(dict(claimed='online speedup rho/IC median 12.29x (n=83)', computed=f"{s83['med_online']:.6f}",
                   definition='median over 16 workloads of rho_record.online_ms / ic_record.online_ms', files=[W83], status=sig_eq(12.29, 4, s83['med_online'])))
claims.append(dict(claimed='95% CI 4.64–13.38 (n=71) and 3.26–32.21 (n=83)', computed='not computed',
                   definition='bootstrap of the median: resample count, seed and method are not stated in the committed evidence', files=[W71, W83], status='cannot_define'))
claims.append(dict(claimed='ratio of mean online times 2.80x (n=83 headline)', computed=f"{s83['mean_ratio']:.6f} (rho mean {s83['mean_rho']:.1f} ms / IC mean {s83['mean_ic']:.1f} ms)",
                   definition='mean rho_record.online_ms over 16 workloads / mean ic_record.online_ms over 16 workloads', files=[W83], status=sig_eq(2.80, 3, s83['mean_ratio'])))
claims.append(dict(claimed='IC slower online on 4 of 16 (n=83)', computed=f"{s83['ic_slower']} of {s83['n']}",
                   definition='count of workloads with rho/IC online ratio below 1', files=[W83], status='match' if s83['ic_slower'] == 4 else 'mismatch'))
claims.append(dict(claimed='cold IC/rho 133.7 (n=83, median)', computed=f"{s83['med_cold']:.6f}",
                   definition='median over 16 workloads of ic.wall_s / rho.wall_s (per-process wall, setup paid)', files=[W83], status=sig_eq(133.7, 4, s83['med_cold'])))
claims.append(dict(claimed='cold IC/rho 154.4 (n=71, median)', computed=f"{s71['med_cold']:.6f}",
                   definition='median over 16 workloads of ic.wall_s / rho.wall_s (per-process wall, setup paid)', files=[W71], status=sig_eq(154.4, 4, s71['med_cold'])))
v71 = json.load(open(RUN71 + '/../../verify.json')) if os.path.exists(RUN71 + '/../../verify.json') else json.load(open(STUDY + '/verify.json'))
v83 = json.load(open(STUDY + '/verify-n83.json'))
claims.append(dict(claimed='verify PASS 256/256 files per run', computed=f"n71 {v71['status']} {v71['files']} files, n83 {v83['status']} {v83['files']} files, mismatches {len(v71['mismatches'])}/{len(v83['mismatches'])}",
                   definition='verify.json and verify-n83.json status and files count', files=[STUDY + '/verify.json', STUDY + '/verify-n83.json'],
                   status='match' if v71['status'] == v83['status'] == 'PASS' and v71['files'] == v83['files'] == 256 else 'mismatch'))
iso = jl(STUDY + '/isolation.jsonl') + jl(STUDY + '/isolation-n83.jsonl')
cont = sum(int(x.get('contended_samples', 0)) for x in iso)
claims.append(dict(claimed='0 contended samples', computed=f"{cont} contended samples over {len(iso)} isolation records",
                   definition='sum of contended_samples in isolation.jsonl and isolation-n83.jsonl', files=[STUDY + '/isolation.jsonl', STUDY + '/isolation-n83.jsonl'],
                   status='match' if cont == 0 else 'mismatch'))
claims.append(dict(claimed='local contended M4 comparison 5.6x / 9.46x', computed='not in committed autolab evidence of this study',
                   definition='local comparison runs are cited from another run id; no raw record for them in this study directory', files=[STUDY], status='no_raw_in_repo'))
off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control|rho-neg-aa')
iso_lines, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned|shared|producers')
notes.append(f"off-repo hits {off_n}; AA hits {aa_n}; isolation hits {iso_n}")
print(json.dumps({'row_no': ROW, 'slug': SLUG, 'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'},
    'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso_lines,
    'protocol_git_date': '', 'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/104.py', 'notes': 'no PROTOCOL or spec file in the study directory (protocol_git_date empty) | ' + ' | '.join(notes)}, ensure_ascii=False))
