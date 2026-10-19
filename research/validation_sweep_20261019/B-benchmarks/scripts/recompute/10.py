#!/usr/bin/env python3
# Row 10 recompute (20261017-volcano-C-finite-field). Reads committed files only.
import json, subprocess, tarfile, collections

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/10.py'
E = 'research/full_volcano_graphwide_ic_20261017/execution/finite-field-k1-evidence'
README = E + '/README.md'
TAR = E + '/runs.tar.xz'
TAR_MEMBER = 'runs/prefix.json'
STATS = E + '/stratum-statistics.json'
PROTO = 'research/full_volcano_graphwide_ic_20261017/execution/FINITE_FIELD_K1_PROTOCOL.md'
FIRST_RAW = E + '/extracted-match.json'


def gd(path):
    out = subprocess.run(['git', '-C', R, 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True).stdout.strip()
    return out.replace('\n', ';')


t = tarfile.open(R + '/' + TAR, 'r:xz')
p = json.load(t.extractfile(TAR_MEMBER))
rows = p['rows']
stats = json.load(open(R + '/' + STATS))

verified = sum(1 for r in rows if r['status'] == 'verified')
requested = p['summary']['requested']
rel_sum = sum(r['relation_report']['relation_count'] for r in rows)
q_sum = sum(r['relation_report']['queries'] for r in rows)
e2e = [r['end_to_end_ns'] for r in rows]
e2e_mean = sum(e2e) / len(e2e)
exact = [g for g in stats['groups'] if g['conductor'] is not None]
null = [g for g in stats['groups'] if g['conductor'] is None]
cmin = min(g['conductor'] for g in exact)
cmax = max(g['conductor'] for g in exact)
n_unknown_records = stats['records_unknown_conductor']
n_exact_records = stats['records_exact_conductor']
n_vertices = len(set(r['graph_vertex'] for r in rows))

claims = [
    {'claimed': '3,738/3,738 prefix targets verified',
     'computed': f'verified={verified} of rows={len(rows)}; summary.verified={p["summary"]["verified"]}, summary.requested={requested}',
     'definition': 'count of rows with status=="verified" in runs/prefix.json (inside runs.tar.xz), over total rows',
     'files': [TAR + ':' + TAR_MEMBER],
     'status': 'match' if verified == 3738 and requested == 3738 else 'mismatch'},
    {'claimed': '15,042 equations',
     'computed': f'sum relation_report.relation_count={rel_sum}',
     'definition': 'sum of relation_report.relation_count over rows (headline word "equations"; the field is relation_count)',
     'files': [TAR + ':' + TAR_MEMBER],
     'status': 'match' if rel_sum == 15042 else 'mismatch'},
    {'claimed': '9,171 queries',
     'computed': f'sum relation_report.queries={q_sum}',
     'definition': 'sum of relation_report.queries over rows',
     'files': [TAR + ':' + TAR_MEMBER],
     'status': 'match' if q_sum == 9171 else 'mismatch'},
    {'claimed': '517 vertices',
     'computed': f'distinct graph_vertex over rows={n_vertices}; records_exact_conductor={n_exact_records} (2 targets per vertex in rows; vertex-to-conductor map not in committed rows)',
     'definition': 'no committed field maps a vertex to a conductor label; 517 is not derivable from these files',
     'files': [TAR + ':' + TAR_MEMBER, STATS],
     'status': 'cannot_define'},
    {'claimed': '24 strata',
     'computed': f'groups with non-null conductor={len(exact)}; null bucket groups={len(null)}',
     'definition': 'count of groups in stratum-statistics.json whose conductor is not null',
     'files': [STATS],
     'status': 'match' if len(exact) == 24 else 'mismatch'},
    {'claimed': 'conductor 2,985,984–967,458,816',
     'computed': f'min conductor={cmin}; max conductor={cmax}',
     'definition': 'min and max of conductor over the non-null groups in stratum-statistics.json',
     'files': [STATS],
     'status': 'match' if (cmin, cmax) == (2985984, 967458816) else 'mismatch'},
    {'claimed': '1,352 unknown',
     'computed': f'records_unknown_conductor={n_unknown_records} (records, not vertices)',
     'definition': 'no committed field gives a vertex count for unknown conductor; records_unknown_conductor is a record count',
     'files': [STATS],
     'status': 'cannot_define'},
    {'claimed': 'end-to-end ns min/mean/max 121,110 / 211,175.290 / 3,620,010, n=3,738',
     'computed': f'min={min(e2e)} mean={e2e_mean:.3f} max={max(e2e)} n={len(e2e)}',
     'definition': 'min, arithmetic mean, max and count of end_to_end_ns over rows',
     'files': [TAR + ':' + TAR_MEMBER],
     'status': 'match' if (min(e2e), round(e2e_mean, 3), max(e2e), len(e2e)) == (121110, 211175.290, 3620010, 3738) else 'mismatch'},
]

out = {
    'row_no': 10,
    'slug': '20261017-volcano-C-finite-field',
    'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                  'files': [], 'status': 'not_claimed'},
    'offrepo_refs': [
        f'{README}:21: [Self-contained Drive archive](https://drive.google.com/file/d/1Xi7mjLCskYEauXlwzAfQQtOoHw6waop-/view)'],
    'aa_lines': [],
    'isolation_lines': [],
    'protocol_git_date': gd(PROTO),
    'first_raw_git_date': gd(FIRST_RAW),
    'script': SCRIPT,
    'notes': ('Protocol path used: ' + PROTO + ' (the row does not name one). First raw file used: '
              + FIRST_RAW + ' (first listed candidate). Prefix JSON summary timing_status: '
              + 'UNQUALIFIED_DESCRIPTIVE. Study README has no A/A, null-control or isolation line. '
              'The 517 and 1,352 claims are not defined by the committed rows.'),
}
print(json.dumps(out, ensure_ascii=False))
