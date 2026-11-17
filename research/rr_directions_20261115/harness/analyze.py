"""Tabulate results.jsonl of run_panel.py: one row per (m, ell, arm, solver).

    python3 analyze.py RUNDIR [--json out.json] [--md out.md]
"""

import argparse
import json
import os
import statistics


def median(xs):
    return statistics.median(xs) if xs else None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('rundir', nargs='+', help='one or more run directories; their results.jsonl are merged')
    ap.add_argument('--json')
    ap.add_argument('--md')
    args = ap.parse_args()
    rows = []
    for rd in args.rundir:
        rows += [json.loads(l) for l in open(os.path.join(rd, 'results.jsonl'))]
    rows = [r for r in rows if 'skipped' not in r['result']]
    # the Rust panel records the base with the 2-torsion point included; the
    # Python cross-check recorded the x != 0 count (see PREREGISTRATION amendment 1)
    for r in rows:
        for k in ('truth_ordered_total', 'truth_ordered_distinct'):
            if k + '_incl_2torsion' in r:
                r[k] = r[k + '_incl_2torsion']
    cells = {}
    for r in rows:
        key = (r['m'], r['ell'], r['arm'], r['solver'])
        cells.setdefault(key, []).append(r)
    table = []
    for key in sorted(cells):
        m, ell, arm, solver = key
        rs = cells[key]
        res = [r['result'] for r in rs]
        row = {'m': m, 'ell': ell, 'arm': arm, 'solver': solver, 'draws': len(rs),
               'n_vars': rs[0]['n_vars'], 'n_polys': rs[0]['n_polys'], 'degree': rs[0]['degree'],
               'decided': sum(1 for x in res if x.get('decided')),
               'wall_median_s': median([x.get('wall_outer_s', 0) for x in res]),
               'wall_max_s': max(x.get('wall_outer_s', 0) for x in res)}
        if solver == 'f4':
            dec = [x for x in res if x.get('decided')]
            row.update({
                'solving_degree': sorted(x.get('solving_degree') for x in dec),
                'degree_reached': sorted(x.get('degree_reached') for x in dec),
                'degree_reached_timeouts': sorted(x.get('degree_reached') for x in res if not x.get('decided')),
                'oversize_matrices': sorted('%dx%d@d%d' % (x.get('oversize_rows') or 0, x.get('oversize_cols') or 0,
                                                             x.get('oversize_degree') or 0)
                                            for x in res if x.get('oversize')),
                'matrix_rows_max_median': median([x.get('matrix_rows_max') for x in dec]),
                'matrix_rows_max_max': max([x.get('matrix_rows_max') or 0 for x in res] or [0]),
                'word_xors_median': median([x.get('word_xors') for x in dec]),
                'truth_mismatch': sum(1 for r in rs if r['result'].get('decided') and
                                      r['result'].get('n_solutions') !=
                                      (r['truth_ordered_distinct'] if arm == 'incidence' else r['truth_ordered_total'])),
                'unverified_solutions': sum((x.get('n_solutions') or 0) - (x.get('verified_solutions') or 0)
                                            for x in dec if x.get('n_solutions') is not None and x.get('n_solutions') <= 64),
            })
        else:
            dec = [x for x in res if x.get('decided')]
            row.update({
                'sat': sum(1 for x in res if x.get('status') == 'sat'),
                'unsat': sum(1 for x in res if x.get('status') == 'unsat'),
                'conflicts_median': median([x.get('conflicts') for x in dec if x.get('conflicts') is not None]),
                'conflicts_max': max([x.get('conflicts') or 0 for x in res] or [0]),
                'models_failed_verification': sum(1 for x in res if x.get('status') == 'sat' and not x.get('model_verified')),
                'truth_mismatch': sum(1 for r in rs if r['result'].get('decided') and
                                      ((r['result']['status'] == 'sat') !=
                                       ((r['truth_ordered_distinct'] if arm == 'incidence' else r['truth_ordered_total']) > 0))),
            })
        table.append(row)
    if args.json:
        json.dump(table, open(args.json, 'w'), indent=1)
    lines = ['| m | ℓ | arm | solver | vars | eqs | deg | decided | solving deg (F4) / conflicts med (SAT) | max rows / max conflicts | wall med s | wall max s | truth mismatch |',
             '|--:|--:|:--|:--|--:|--:|--:|--:|:--|--:|--:|--:|--:|']
    for r in table:
        if r['solver'] == 'f4':
            mid = 'sd ' + ','.join(str(v) for v in r['solving_degree']) + (
                ' (undecided reached ' + ','.join(str(v) for v in r['degree_reached_timeouts']) + ')' if r['degree_reached_timeouts'] else '')
            if r['oversize_matrices']:
                mid += '; refused ' + r['oversize_matrices'][0] + ('..' + r['oversize_matrices'][-1] if len(r['oversize_matrices']) > 1 else '')
            mx = str(r['matrix_rows_max_max'])
        else:
            mid = 'sat %d unsat %d, confl med %s' % (r['sat'], r['unsat'], r['conflicts_median'])
            mx = str(r['conflicts_max'])
        lines.append('| %d | %d | %s | %s | %d | %d | %d | %d/%d | %s | %s | %.1f | %.1f | %d |' % (
            r['m'], r['ell'], r['arm'], r['solver'], r['n_vars'], r['n_polys'], r['degree'],
            r['decided'], r['draws'], mid, mx, r['wall_median_s'], r['wall_max_s'], r['truth_mismatch']))
    md = '\n'.join(lines)
    print(md)
    if args.md:
        open(args.md, 'w').write(md + '\n')


if __name__ == '__main__':
    main()
