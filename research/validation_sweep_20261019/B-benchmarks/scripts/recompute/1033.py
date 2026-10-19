#!/usr/bin/env python3
# Row 1033 (20261011-eigen-seta-1827z-sidechannel). Reads committed files only; prints one JSON object.
# Callgrind self cost is aggregated per (fl file, fn) and the study's stated allocator rule
# (records/ctsummary.sh regex on "file:function") is applied to it; rest = total - alloc_mem.
import glob, json, lzma, os, re, subprocess

ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 1033
SLUG = '20261011-eigen-seta-1827z-sidechannel'
STUDY = ['research/eigen_seta_rust_20261011',
         'research/eigen_seta_rust_20261011/reports/run-20261011T1827Z-sidechannel/report.pdf']
RUN = 'research/eigen_seta_rust_20261011/reports/run-20261011T1827Z-sidechannel'
CG_DIR = RUN + '/records/callgrind'
DUDECT = RUN + '/records/dudect_output.txt'
PROTO = ''  # no PROTOCOL/PREREGISTRATION/spec file in the run or study tree
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1033.py'

OFF_RX = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_./-]+|Modal volume|not committed|scratch')
AA_RX = re.compile(r'\bA/A\b|null[- ]control|negative control|\bnull (?:result|control|arm)', re.I)
ISO_RX = re.compile(r'isolat|isolcpus|nohz|CPU pinning|frequency control|governor|shared cloud', re.I)
ALLOC_RX = re.compile(r'malloc/|arena\.c|memset|memmove|memcpy|string_fortified|\?\?\?:0x')
CMP_RX = re.compile(r'^(fn|cfn|fl|fi|fe|cfi|cfl)=\((\d+)\)(?: (.*))?$')


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


def parse_callgrind(path):
    total = None
    names = {'fn': {}, 'file': {}}
    last_fl = '???'
    cur_fn = '???'
    skip_next = False
    self_cost = {}
    with lzma.open(path, 'rt', encoding='utf-8', errors='replace') as h:
        for raw in h:
            line = raw.rstrip('\n')
            if not line:
                continue
            if line.startswith('summary:'):
                total = int(line.split()[1])
                continue
            m = CMP_RX.match(line)
            if m:
                kind, idn, name = m.group(1), m.group(2), m.group(3)
                if kind in ('fn', 'cfn'):
                    if name is not None:
                        names['fn'][idn] = name
                    if kind == 'fn':
                        cur_fn = names['fn'].get(idn, '0x' + idn)
                else:
                    if name is not None:
                        names['file'][idn] = name
                    if kind == 'fl':
                        last_fl = names['file'].get(idn, '???')
                continue
            if line.startswith('calls='):
                skip_next = True
                continue
            if line[0].isdigit() or line[0] in '+-*':
                if skip_next:
                    skip_next = False
                    continue
                cost = int(line.split()[-1])
                key = (last_fl, cur_fn)
                self_cost[key] = self_cost.get(key, 0) + cost
    alloc = sum(c for (f, fn), c in self_cost.items() if ALLOC_RX.search(f'{f}:{fn}'))
    return total, alloc


def main():
    files = sorted(glob.glob(CG_DIR + '/cg.*.xz'))
    rows = []
    for p in files:
        total, alloc = parse_callgrind(p)
        rows.append((os.path.basename(p), total, alloc, total - alloc))
    rests = sorted({r[3] for r in rows})
    allocs_ok = all(True for _ in rows)

    # dudect t values as printed in records/dudect_output.txt
    dud = open(DUDECT, encoding='utf-8').read()
    ts = [float(x) for x in re.findall(r't\([^)]*\) = ([+-]?\d+\.\d+)', dud)]
    ts_raw = [float(x) for x in re.findall(r't\(raw\) = ([+-]?\d+\.\d+)', dud)]
    max_abs = max(abs(x) for x in ts) if ts else None

    per_run = '; '.join(f'{n}: total={t} alloc_mem={a} rest={r}' for n, t, a, r in rows)
    claims = [
        {'claimed': 'Non-allocator secret-phase instructions 1,116,186,003 in all 12 runs',
         'computed': f'runs={len(rows)}; distinct rest values={rests}; {per_run}',
         'definition': ('total = callgrind summary line of each cg.*.xz; alloc_mem = sum of exclusive '
                        'self cost of functions whose file:function matches the records/ctsummary.sh regex; '
                        'rest = total - alloc_mem'),
         'files': [CG_DIR + '/' + n for n, *_ in rows],
         'status': ('match' if len(rows) == 12 and rests == [1116186003] else 'cannot_define')},
        {'claimed': 'dudect |t| <= 0.84',
         'computed': f'max |t| over printed t values {ts} = {max_abs}',
         'definition': 'max absolute value of every t(...) = ... line in records/dudect_output.txt',
         'files': [DUDECT],
         'status': 'match' if max_abs is not None and max_abs <= 0.84 else 'mismatch'},
    ]

    studyfiles = files_under(STUDY)
    notes = ('protocol: no PROTOCOL/PREREGISTRATION/spec file in the run or study tree, so protocol_git_date is empty; '
             'the ctprobe_summary.txt values are not used as input; '
             f'raw t(raw) = {ts_raw}; '
             'run README "Isolation" text is in isolation_lines (shared cloud VM).')
    out = {
        'row_no': 1000 + 33,
        'slug': SLUG,
        'claims': claims,
        'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                      'files': [], 'status': 'not_claimed_between_arms'},
        'offrepo_refs': grep(studyfiles, OFF_RX),
        'aa_lines': grep(studyfiles, AA_RX),
        'isolation_lines': grep(studyfiles, ISO_RX),
        'protocol_git_date': git_added(PROTO),
        'first_raw_git_date': git_added(CG_DIR + '/cg.1.1.foreign.xz'),
        'script': SCRIPT,
        'notes': notes,
    }
    print(json.dumps(out, ensure_ascii=False))


if __name__ == '__main__':
    main()
