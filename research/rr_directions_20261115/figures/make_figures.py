"""Plot recorded values only (research-visuals): no computation beyond reading files.

    python3 make_figures.py

Figures:
  lattice_count.svg/.png  — pick 3: box-passing (a,b) per target against the
                            heuristic and against the genuine Vieta/relation counts.
  panel_*.svg/.png        — pick 2: solving degree, matrix rows, conflicts, wall per cell
                            (from runs/<run>/results.jsonl), when present.
"""

import json
import os
import statistics
import sys

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
STUDY = os.path.dirname(HERE)


def save(fig, name):
    fig.savefig(os.path.join(HERE, name + '.svg'))
    fig.savefig(os.path.join(HERE, name + '.png'), dpi=150)
    plt.close(fig)


def lattice():
    path = os.path.join(STUDY, 'pick3_lattice_count', 'lattice_count.json')
    rows = json.load(open(path))
    ps = [r['p'] for r in rows]
    fig, ax = plt.subplots(figsize=(6.4, 4.2))
    ax.plot(ps, [r['N_box_mean'] for r in rows], 'o-', label='box-passing (a,b) per target, measured')
    ax.plot(ps, [r['N_box_heuristic'] for r in rows], 'x--', label='p²·∏(box_k/p), heuristic')
    ax.plot(ps, [max(r['N_split_mean'], 0.05) for r in rows], 's-', label='splitting over [0,B] (Vieta image), measured (0 drawn at 0.05)')
    ax.plot(ps, [max(r['N_rel_mean'], 0.05) for r in rows], 'd:', label='genuine relations, measured (0 drawn at 0.05)')
    ax.set_xscale('log')
    ax.set_yscale('log')
    ax.set_xlabel('p (B = round(p^{1/3}), m = 3, 8 targets per p)')
    ax.set_ylabel('count per target (log)')
    ax.set_title('Pick 3: the coefficient-box relaxation admits ~p^{(m-1)/2}·∏C(m,k)\n'
                 'candidates per target against ≤ 1 genuine one (lattice_count.json)')
    ax.legend(fontsize=7)
    ax.grid(True, which='both', alpha=0.3)
    save(fig, 'lattice_count')


def panel(run):
    path = os.path.join(STUDY, 'runs', run, 'results.jsonl')
    if not os.path.exists(path):
        return
    rows = [json.loads(l) for l in open(path)]
    cells = {}
    for r in rows:
        cells.setdefault((r['arm'], r['solver']), {}).setdefault(r['m'], []).append(r)
    fig, axes = plt.subplots(1, 3, figsize=(12, 4))
    styles = {('incidence', 'f4'): 'o-', ('norm', 'f4'): 's-', ('incidence', 'sat'): 'o--', ('norm', 'sat'): 's--'}
    for (arm, solver), byM in sorted(cells.items()):
        ms = sorted(byM)
        st = styles.get((arm, solver), 'x-')
        label = '%s / %s' % (arm, solver)
        if solver == 'f4':
            sd = [statistics.median([x['result'].get('solving_degree') or 0 for x in byM[m] if x['result'].get('decided')] or [float('nan')]) for m in ms]
            axes[0].plot(ms, sd, st, label=label)
            rowsmax = [max(x['result'].get('matrix_rows_max') or 0 for x in byM[m]) for m in ms]
            axes[1].plot(ms, rowsmax, st, label=label)
        else:
            confl = [statistics.median([x['result'].get('conflicts') or 0 for x in byM[m] if x['result'].get('decided')] or [float('nan')]) for m in ms]
            axes[1].plot(ms, confl, st, label=label + ' (conflicts)')
        wall = [statistics.median([x['result'].get('wall_outer_s', 0) for x in byM[m]]) for m in ms]
        dec = ['%d/%d' % (sum(1 for x in byM[m] if x['result'].get('decided')), len(byM[m])) for m in ms]
        axes[2].plot(ms, wall, st, label=label + ' decided ' + ' '.join(dec))
    axes[0].set_title('F4 solving degree (median of decided draws)')
    axes[0].set_xlabel('m'); axes[0].set_ylabel('degree')
    axes[1].set_title('F4 max matrix rows / SAT median conflicts')
    axes[1].set_yscale('log'); axes[1].set_xlabel('m')
    axes[2].set_title('wall per solve, median (s; practicality note)')
    axes[2].set_yscale('log'); axes[2].set_xlabel('m')
    for ax in axes:
        ax.grid(True, alpha=0.3)
        ax.legend(fontsize=6)
    fig.suptitle('Pick 2: SAT against F4 on the Riemann–Roch systems, run %s (8 draws per cell: 4 planted, 4 unplanted)' % run, fontsize=9)
    fig.tight_layout()
    save(fig, 'panel_' + run)


if __name__ == '__main__':
    lattice()
    for run in sys.argv[1:]:
        panel(run)
    print('ok')
