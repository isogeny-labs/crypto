#!/usr/bin/env python3
# Row 1035 (20261011-conductor-gap-round3). Reads committed files only; prints one JSON object.
import json, os, re, subprocess

ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 1035
SLUG = '20261011-conductor-gap-round3'
STUDY = ['research/isogeny_conductor_gap_round3_20261011/README.md',
         'research/isogeny_conductor_gap_round3_20261011/reports/run-20261011T1714Z/report.pdf',
         'research/isogeny_conductor_gap_round3_20261011/reports/run-20261011T1727Z/report.pdf']
STUDY_DIR = 'research/isogeny_conductor_gap_round3_20261011'
E4 = STUDY_DIR + '/e4/records.jsonl'
SESS = [STUDY_DIR + '/sessions/e5_combined_n17_t0/records.jsonl',
        STUDY_DIR + '/sessions/e5_combined_n17_t1/records.jsonl']
PROTO = STUDY_DIR + '/PROTOCOL.md'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1035.py'

OFF_RX = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_./-]+|Modal volume|not committed|scratch')
AA_RX = re.compile(r'\bA/A\b|null[- ]control|negative control|\bnull (?:result|control|arm)', re.I)
ISO_RX = re.compile(r'isolat|isolcpus|nohz|CPU pinning|frequency control|governor|shared cloud|unisolated', re.I)


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


def within(claimed, computed):
    return abs(computed - claimed) / claimed <= 0.01


def jl(path):
    with open(path, encoding='utf-8') as h:
        return [json.loads(x) for x in h if x.strip()]


def main():
    e4 = jl(E4)
    sysr = [r for r in e4 if r.get('kind') == 'system']
    n_sys = len(sysr)
    n_done = sum(1 for r in sysr if not r.get('timed_out'))

    def key(r):
        return (r['pair'], r['curve_key'], r['role'], r['target_kind'], r['target_index'], r['base_size'])
    direct = {key(r): r['field_ops'] for r in sysr if r['method'] == 'direct'}
    ratios = {2: {}, 4: {}}
    for r in sysr:
        if r['method'] == 'quotient':
            k = key(r)
            if k in direct:
                ratios[r['subgroup_order']][k] = direct[k] / r['field_ops']

    def rng(h, tk=None):
        v = [x for k, x in ratios[h].items() if tk is None or k[3] == tk]
        return (min(v), max(v), len(v)) if v else (None, None, 0)

    r2r, r4r = rng(2, 'random'), rng(4, 'random')
    r2a, r4a = rng(2), rng(4)

    blind = {}
    comb = {}
    for path in SESS:
        recs = [r for r in jl(path) if not r.get('warmup')]
        sess = path.split('/')[-2]
        for rec in recs:
            k = (sess, rec['round'])
            if rec['arm'] == 'f4-blind':
                blind[k] = rec
            elif rec['arm'] == 'f4-combined':
                comb[k] = rec
    e5_ratio = {k: comb[k]['solver']['ops'] / blind[k]['solver']['ops'] for k in comb}
    e5_comb_wall = {k: comb[k]['online']['wall_ns'] / 1e9 for k in comb}
    e5_blind_wall = {k: blind[k]['online']['wall_ns'] / 1e9 for k in blind}
    rmin, rmax = min(e5_ratio.values()), max(e5_ratio.values())
    cmin, cmax = min(e5_comb_wall.values()), max(e5_comb_wall.values())
    bmin, bmax = min(e5_blind_wall.values()), max(e5_blind_wall.values())

    def fmt(k, d):
        return {kk: round(vv, 4) for kk, vv in d.items()}

    claims = [
        {'claimed': 'E4 direct/quotient F_p row-reduction multiplication ratio 4.12-4.26 (order 2)',
         'computed': (f'random-target ratio min={r2r[0]:.4f} max={r2r[1]:.4f} n={r2r[2]}; '
                      f'all-target ratio min={r2a[0]:.4f} max={r2a[1]:.4f} n={r2a[2]}'),
         'definition': ('field_ops of method=direct (subgroup_order 1) divided by field_ops of method=quotient '
                        '(subgroup_order 2) with the same pair, curve_key, role, target_kind, target_index and base_size '
                        '(matched B); range over random targets as in the README table; all-target range also given'),
         'files': [E4],
         'status': ('partial' if within(4.12, r2r[0]) and within(4.26, r2r[1]) and not (
             within(4.12, r2a[0]) and within(4.26, r2a[1])) else
             ('match' if within(4.12, r2a[0]) and within(4.26, r2a[1]) else 'mismatch'))},
        {'claimed': 'E4 direct/quotient F_p row-reduction multiplication ratio 17.6-19.6 (order 4)',
         'computed': (f'random-target ratio min={r4r[0]:.4f} max={r4r[1]:.4f} n={r4r[2]}; '
                      f'all-target ratio min={r4a[0]:.4f} max={r4a[1]:.4f} n={r4a[2]}'),
         'definition': ('as above with subgroup_order 4; range over random targets as in the README table; '
                        'all-target range also given'),
         'files': [E4],
         'status': ('partial' if within(17.6, r4r[0]) and within(19.6, r4r[1]) and not (
             within(17.6, r4a[0]) and within(19.6, r4a[1])) else
             ('match' if within(17.6, r4a[0]) and within(19.6, r4a[1]) else 'mismatch'))},
        {'claimed': '1,128 systems complete',
         'computed': f'system records={n_sys}; timed_out false={n_done}',
         'definition': 'count of kind=system records in e4/records.jsonl and count with timed_out false',
         'files': [E4],
         'status': 'match' if (n_sys, n_done) == (1128, 1128) else 'mismatch'},
        {'claimed': 'E5 combined/blind F4 operations 234-529x',
         'computed': f'combined/blind solver.ops per (session, round): ' +
                     str({f'{k[0]} r{k[1]}': round(v, 3) for k, v in e5_ratio.items()}),
         'definition': 'solver.ops of f4-combined divided by solver.ops of f4-blind, non-warmup records, per session and round',
         'files': SESS,
         'status': 'match' if within(234, rmin) and within(529, rmax) else 'mismatch'},
        {'claimed': 'E5 combined online 324.5-613.2 s',
         'computed': f'combined online.wall_ns min={cmin:.3f} s max={cmax:.3f} s',
         'definition': 'online.wall_ns / 1e9 of f4-combined non-warmup records',
         'files': SESS,
         'status': 'match' if round(cmin, 1) == 324.5 and round(cmax, 1) == 613.2 else 'mismatch'},
        {'claimed': 'E5 blind online 0.9-2.0 s',
         'computed': f'blind online.wall_ns min={bmin:.3f} s max={bmax:.3f} s',
         'definition': 'online.wall_ns / 1e9 of f4-blind non-warmup records, compared at printed precision',
         'files': SESS,
         'status': 'match' if round(bmin, 1) == 0.9 and round(bmax, 1) == 2.0 else 'mismatch'},
    ]

    studyfiles = files_under([STUDY_DIR + '/README.md', STUDY_DIR + '/reports', STUDY_DIR + '/PROTOCOL.md'])
    notes = ('E4 ratio: field_ops is the row-reduction multiplication count in the README table (B=32 random target direct = 145,630, which matches). '
             'E5 blind wall: online.wall_ns (solve_wall_ns agrees to 1 ms). '
             f'E5 ratio per run (combined/blind): {fmt(None, {})} ').strip()
    notes = notes.replace(" {}", "")
    out = {
        'row_no': 1000 + 35,
        'slug': SLUG,
        'claims': claims,
        'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                      'files': [], 'status': 'not_claimed_between_arms'},
        'offrepo_refs': grep(files_under([STUDY_DIR]), OFF_RX),
        'aa_lines': grep(studyfiles, AA_RX),
        'isolation_lines': grep(studyfiles, ISO_RX),
        'protocol_git_date': git_added(PROTO),
        'first_raw_git_date': git_added(E4),
        'script': SCRIPT,
        'notes': notes,
    }
    print(json.dumps(out, ensure_ascii=False))


if __name__ == '__main__':
    main()
