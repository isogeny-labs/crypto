#!/usr/bin/env python3
# Row 1036 (20261010-isogeny-campaign-and-profiler). Reads committed files only; prints one JSON object.
# The three instruction-reduction figures are not in the listed study dir; their per-round raw
# instruction counts are in research/isogeny_catalogue_setup_20261009/PROFILE_SUMMARY.json.
import json, os, re, statistics, subprocess

ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 1036
SLUG = '20261010-isogeny-campaign-and-profiler'
STUDY_DIR = 'research/p192_p224_hundred_large_prime_isogenies_20261010'
PROF_DIR = 'research/isogeny_catalogue_setup_20261009'
SUMMARY = STUDY_DIR + '/SUMMARY.json'
PROFILE = PROF_DIR + '/PROFILE_SUMMARY.json'
PROTO = STUDY_DIR + '/PROTOCOL.md'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1036.py'

OFF_RX = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_./-]+|Modal volume|not committed|scratch')
AA_RX = re.compile(r'\bA/A\b|null[- ]control|negative control|\bnull (?:result|control|arm)', re.I)
ISO_RX = re.compile(r'isolat|isolcpus|nohz|CPU pinning|frequency control|governor|shared cloud|shared Apple|concurrent walks')


def files_under(paths, exts=('.md', '.typ', '.txt', '.json', '.sh', '.py', '.mmd')):
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
    summ = json.load(open(SUMMARY, encoding='utf-8'))
    recs = summ['records']
    n_pass = sum(1 for r in recs if r.get('status') == 'PASS')
    by_ell = {}
    for r in recs:
        if r.get('status') == 'PASS':
            by_ell[r['ell']] = by_ell.get(r['ell'], 0) + 1

    prof = json.load(open(PROFILE, encoding='utf-8'))
    rows = {r['case']: r for r in prof['rows']}
    per = {}
    for case, r in rows.items():
        ref_med = statistics.median(r['reference'])
        cand_med = statistics.median(r['candidate'])
        red = 100.0 * (1.0 - cand_med / ref_med)
        per[case] = (ref_med, cand_med, red, r['reduction_percent'], r['rounds'], r['correctness'])

    def check(case, claimed):
        ref_med, cand_med, red, stored, rounds, corr = per[case]
        ok = round(red, 6) == round(claimed, 6) and round(stored, 6) == round(claimed, 6)
        return ('match' if ok else 'mismatch'), (
            f'ref_median={ref_med:.0f} cand_median={cand_med:.0f} reduction=100*(1-cand/ref)={red:.6f}% '
            f'stored reduction_percent={stored:.6f}% rounds={rounds} correctness={corr}')

    s_screen, t_screen = check('screen-p224', 67.958860)
    s_search, t_search = check('search-p192', 5.292412)
    s_verify, t_verify = check('verify-p224', 0.147796)

    claims = [
        {'claimed': '100 maps across P192/2341 and P224/1117',
         'computed': f'SUMMARY.json PASS records={n_pass} of {len(recs)}; PASS by ell={by_ell}',
         'definition': 'count of records with status PASS in SUMMARY.json, grouped by ell',
         'files': [SUMMARY],
         'status': 'match' if n_pass == 100 and by_ell == {2341: 50, 1117: 50} else 'mismatch'},
        {'claimed': '67.958860% (screen)',
         'computed': t_screen,
         'definition': 'screen-p224 case: reduction from the median of 5 interleaved reference and candidate Callgrind Ir values',
         'files': [PROFILE],
         'status': s_screen},
        {'claimed': '5.292412% (search)',
         'computed': t_search,
         'definition': 'search-p192 case: as above',
         'files': [PROFILE],
         'status': s_search},
        {'claimed': '0.147796% (replay)',
         'computed': t_verify,
         'definition': ('verify-p224 case (the headline labels this row "replay"; the source table labels it verify-p224): '
                        'as above'),
         'files': [PROFILE],
         'status': s_verify},
    ]

    studyfiles = files_under([STUDY_DIR])
    notes = ('the three percentages and the "(screen)/(search)/(replay)" labels do not appear in the listed study dir; '
             'their source is research/isogeny_catalogue_setup_20261009 (RESULTS.md table and PROFILE_SUMMARY.json). '
             'verify-p224 is the source label for the headline "replay" figure. '
             'PROFILE_SUMMARY.json unit: "Callgrind Ir, ARM Linux, all traced processes"; wall_time_claim false. '
             'protocol: PROTOCOL.md at the study root. offrepo_refs is capped at 200 lines. '
             'PROTOCOL.md and PROFILE_SUMMARY.json both show the same add commit, so the add-commit order does not show which came first.')
    out = {
        'row_no': 1000 + 36,
        'slug': SLUG,
        'claims': claims,
        'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                      'files': [], 'status': 'not_claimed_between_arms'},
        'offrepo_refs': grep(studyfiles, OFF_RX),
        'aa_lines': grep(studyfiles, AA_RX),
        'isolation_lines': grep(studyfiles, ISO_RX),
        'protocol_git_date': git_added(PROTO),
        'first_raw_git_date': git_added(PROFILE),
        'script': SCRIPT,
        'notes': notes,
    }
    print(json.dumps(out, ensure_ascii=False))


if __name__ == '__main__':
    main()
