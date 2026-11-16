#!/usr/bin/env python3
# Row 1034 (20261011-eigen-seta-1813z-bench). Reads committed files only; prints one JSON object.
import json, os, re, subprocess

ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 1034
SLUG = '20261011-eigen-seta-1813z-bench'
STUDY = ['research/eigen_seta_rust_20261011',
         'research/eigen_seta_rust_20261011/reports/run-20261011T1813Z-bench/report.pdf']
RUN = 'research/eigen_seta_rust_20261011/reports/run-20261011T1813Z-bench'
RAW = RUN + '/records/bench_output.txt'
PROTO = ''  # no PROTOCOL/PREREGISTRATION/spec file in the study tree
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1034.py'

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
    trials = re.findall(r'^(\w+) trial (\d+) encaps_s [\d.]+ decaps_s ([\d.]+) ok (\w+)', txt, re.M)
    dec = {}
    for size, _, d, _ in trials:
        dec.setdefault(size, []).append(float(d))
    n_trials = len(trials)
    n_ok = sum(1 for *_, ok in trials if ok == 'true')

    def rng(size):
        v = dec.get(size, [])
        return (min(v), max(v)) if v else (None, None)

    l1, l3, l5 = rng('L1'), rng('L3'), rng('L5')
    claims = [
        {'claimed': 'L1 decapsulation 13.424-14.997 s',
         'computed': f'L1 decaps_s min={l1[0]} max={l1[1]} (n={len(dec.get("L1", []))})',
         'definition': 'min and max of decaps_s over the L1 trial lines in records/bench_output.txt',
         'files': [RAW],
         'status': 'match' if l1 == (13.424, 14.997) else 'mismatch'},
        {'claimed': 'L3 decapsulation 51.834-53.940 s',
         'computed': f'L3 decaps_s min={l3[0]} max={l3[1]} (n={len(dec.get("L3", []))})',
         'definition': 'min and max of decaps_s over the L3 trial lines in records/bench_output.txt',
         'files': [RAW],
         'status': 'match' if l3 == (51.834, 53.940) else 'mismatch'},
        {'claimed': 'L5 decapsulation 120.180-123.953 s',
         'computed': f'L5 decaps_s min={l5[0]} max={l5[1]} (n={len(dec.get("L5", []))})',
         'definition': 'min and max of decaps_s over the L5 trial lines in records/bench_output.txt',
         'files': [RAW],
         'status': 'match' if l5 == (120.180, 123.953) else 'mismatch'},
        {'claimed': '15/15 trials correct',
         'computed': f'ok true on {n_ok} of {n_trials} trial lines (Toy16, Toy32, L1, L3, L5)',
         'definition': 'count of trial lines with ok true over all trial lines in records/bench_output.txt',
         'files': [RAW],
         'status': 'match' if (n_ok, n_trials) == (15, 15) else 'mismatch'},
    ]

    files = files_under(STUDY)
    notes = ('protocol: no PROTOCOL/PREREGISTRATION/spec file in the study tree, so protocol_git_date is empty; '
             'host_and_command.txt says "shared cloud VM ... no CPU pinning, no frequency control"; '
             'bench run is one process per size, sequential per host_and_command.txt command line.')
    out = {
        'row_no': 1000 + 34,
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
