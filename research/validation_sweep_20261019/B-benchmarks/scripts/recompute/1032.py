#!/usr/bin/env python3
# Row 1032 (20261011-eigen-seta-1944z-l1-fix). Reads committed files only; prints one JSON object.
import json, os, re, subprocess

ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 1032
SLUG = '20261011-eigen-seta-1944z-l1-fix'
STUDY = ['research/eigen_seta_rust_20261011',
         'research/eigen_seta_rust_20261011/reports/run-20261011T1944Z-l1-fix/report.pdf']
RUN = 'research/eigen_seta_rust_20261011/reports/run-20261011T1944Z-l1-fix'
RAW = RUN + '/records/run_output.txt'
PROTO = ''  # no PROTOCOL/PREREGISTRATION/spec file in the study tree
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1032.py'

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


def close(claimed, computed):
    return computed is not None and abs(float(computed) - claimed) / claimed <= 0.01


def main():
    txt = open(RAW, encoding='utf-8').read()
    keygen = re.findall(r'^L1 keygen_s ([\d.]+)', txt, re.M)
    encaps = re.findall(r'^L1 trial \d+ encaps_s ([\d.]+)', txt, re.M)
    decaps = re.findall(r'^L1 trial \d+ encaps_s [\d.]+ decaps_s ([\d.]+)', txt, re.M)
    results = re.findall(r'^L1 RESULT (\d+)/(\d+) decapsulations correct', txt, re.M)

    kg = keygen[0] if keygen else None
    en = encaps[0] if encaps else None
    de = decaps[0] if decaps else None
    claims = [
        {'claimed': 'KeyGen 25.788 s', 'computed': f'L1 keygen_s = {kg}',
         'definition': 'keygen_s of the L1 line in records/run_output.txt',
         'files': [RAW], 'status': 'match' if close(25.788, kg) else 'mismatch'},
        {'claimed': 'Encaps 0.2242 s', 'computed': f'L1 trial 0 encaps_s = {en}',
         'definition': 'encaps_s of the L1 trial 0 line in records/run_output.txt',
         'files': [RAW], 'status': 'match' if close(0.2242, en) else 'mismatch'},
        {'claimed': 'Decaps 11.406 s', 'computed': f'L1 trial 0 decaps_s = {de}',
         'definition': 'decaps_s of the L1 trial 0 line in records/run_output.txt',
         'files': [RAW], 'status': 'match' if close(11.406, de) else 'mismatch'},
        {'claimed': 'one correct round trip', 'computed': f'L1 RESULT lines = {results}',
         'definition': 'L1 RESULT line: decapsulations correct, n/N',
         'files': [RAW],
         'status': 'match' if results == [('1', '1')] else 'mismatch'},
    ]

    files = files_under(STUDY)
    notes = ('protocol: no PROTOCOL/PREREGISTRATION/spec file in the study tree, so protocol_git_date is empty; '
             'MEASURED text says "byte-identical browsing copies" (copies vs sources), not an arm comparison.')
    out = {
        'row_no': 1000 + 32,
        'slug': SLUG,
        'claims': claims,
        'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                      'files': [], 'status': 'not_claimed_between_arms'},
        'offrepo_refs': grep(files, OFF_RX),
        'aa_lines': grep(files, AA_RX),
        'isolation_lines': grep(files, ISO_RX),
        'protocol_git_date': git_added(PROTO),
        'first_raw_git_date': git_added(RAW),
        'script': SCRIPT,
        'notes': notes,
    }
    print(json.dumps(out, ensure_ascii=False))


if __name__ == '__main__':
    main()
