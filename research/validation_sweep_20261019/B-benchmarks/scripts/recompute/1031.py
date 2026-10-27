#!/usr/bin/env python3
# Row 1031 (20261011-eigen-seta-1946z-l3-l5-fix). Reads committed files only; prints one JSON object.
import json, os, re, subprocess

ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 1031
SLUG = '20261011-eigen-seta-1946z-l3-l5-fix'
STUDY = ['research/eigen_seta_rust_20261011',
         'research/eigen_seta_rust_20261011/reports/run-20261011T1946Z-l3-l5-fix/report.pdf']
RUN = 'research/eigen_seta_rust_20261011/reports/run-20261011T1946Z-l3-l5-fix'
RAW = RUN + '/records/run_output.txt'
PROTO = ''  # no PROTOCOL/PREREGISTRATION/spec file in the study tree
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1031.py'

OFF_RX = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_./-]+|Modal volume|not committed|scratch')
AA_RX = re.compile(r'\bA/A\b|null[- ]control|negative control|\bnull (?:result|control|arm)', re.I)
ISO_RX = re.compile(r'isolat|isolcpus|nohz|CPU pinning|frequency control|governor|shared cloud', re.I)


def files_under(paths, exts=('.md', '.typ', '.txt', '.json', '.sh', '.py')):
    out = []
    for p in paths:
        if os.path.isfile(p):
            out.append(p)
            continue
        for dp, _, fn in os.walk(p):
            for f in fn:
                q = os.path.join(dp, f)
                if f.endswith(exts) and os.path.getsize(q) < 2_000_000:
                    out.append(q)
    return sorted(out)


def grep(files, rx, cap=200):
    hits = []
    for q in files:
        with open(q, encoding='utf-8', errors='replace') as h:
            for i, line in enumerate(h, 1):
                if rx.search(line):
                    hits.append(f'{q}:{i}: {line.strip()[:260]}')
                    if len(hits) >= cap:
                        return hits
    return hits


def git_added(path):
    if not path:
        return ''
    r = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                       capture_output=True, text=True, cwd=ROOT)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else ''


def main():
    txt = open(RAW, encoding='utf-8').read()
    trials = re.findall(r'^(L\d) trial (\d+) encaps_s ([\d.]+) decaps_s ([\d.]+) ok (\w+)', txt, re.M)
    results = re.findall(r'^(L\d) RESULT (\d+)/(\d+) decapsulations correct', txt, re.M)
    keygen = dict(re.findall(r'^(L\d) keygen_s ([\d.]+)', txt, re.M))
    decaps = {s: d for s, _, _, d, _ in trials}
    per_size = {}
    for s, *_ in trials:
        per_size[s] = per_size.get(s, 0) + 1

    claims = []
    l3 = decaps.get('L3')
    claims.append({
        'claimed': 'L3 Decaps 47.275 s',
        'computed': f'L3 decaps_s = {l3}',
        'definition': 'decaps_s of the L3 trial line in records/run_output.txt',
        'files': [RAW],
        'status': 'match' if l3 is not None and abs(float(l3) - 47.275) / 47.275 <= 0.01 else 'mismatch'})
    l5 = decaps.get('L5')
    claims.append({
        'claimed': 'L5 Decaps 107.221 s',
        'computed': f'L5 decaps_s = {l5}',
        'definition': 'decaps_s of the L5 trial line in records/run_output.txt',
        'files': [RAW],
        'status': 'match' if l5 is not None and abs(float(l5) - 107.221) / 107.221 <= 0.01 else 'mismatch'})
    claims.append({
        'claimed': 'one sample per size',
        'computed': f'trial lines per size = {per_size}; RESULT lines = {results}',
        'definition': 'count of trial lines per size in records/run_output.txt',
        'files': [RAW],
        'status': 'match' if per_size == {'L3': 1, 'L5': 1} else 'mismatch'})

    files = files_under(STUDY)
    offrepo = grep(files, OFF_RX)
    aa = grep(files, AA_RX)
    iso = grep(files, ISO_RX)
    notes = (f'protocol: no PROTOCOL/PREREGISTRATION/spec file in the study tree, so protocol_git_date is empty; '
             f'keygen_s L3={keygen.get("L3")} L5={keygen.get("L5")}; '
             'HEADLINE "no spread" is not a numeric claim (n=1 per size) and is not recomputed; '
             'the study files and their text were not interpreted; '
             'MEASURED text says "byte-identical browsing copies" (copies vs sources), not an arm comparison.')
    out = {
        'row_no': 1000 + 31,
        'slug': SLUG,
        'claims': claims,
        'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                      'files': [], 'status': 'not_claimed_between_arms'},
        'offrepo_refs': offrepo,
        'aa_lines': aa,
        'isolation_lines': iso,
        'protocol_git_date': git_added(PROTO),
        'first_raw_git_date': git_added(RAW),
        'script': SCRIPT,
        'notes': notes,
    }
    print(json.dumps(out, ensure_ascii=False))


if __name__ == '__main__':
    main()
