"""Standalone scientific timing figure from saved complete-call measurements."""
import csv
import json
from pathlib import Path
import statistics

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent


def main():
    rows = []
    for folder in ('results', 'results_large'):
        rows.extend(json.loads((ROOT / 'fast_trace' / folder / 'summary.json').read_text())['summary'])
    with (ROOT / 'trace_scaling.csv').open('w', newline='') as file:
        writer = csv.DictWriter(file, fieldnames=['p', 'degree', 'kind', 'trace_cpu_ms', 'dense_cpu_ms', 'dense_over_trace'])
        writer.writeheader()
        for row in rows:
            writer.writerow({'p': row['p'], 'degree': row['degree'], 'kind': row['kind'],
                             'trace_cpu_ms': 1000 * row['trace']['median_cpu_seconds'],
                             'dense_cpu_ms': 1000 * row['dense']['median_cpu_seconds'],
                             'dense_over_trace': row['dense_over_trace']})
    fig, axes = plt.subplots(1, 2, figsize=(10, 4.6))
    for ax, kind, title in zip(axes, ('generic', 'collision'), ('Generic support', 'All translated abscissae collide')):
        degrees = sorted({row['degree'] for row in rows})
        for method, color, marker in [('trace', '#136f63', 'o'), ('dense', '#b14934', 's')]:
            groups = [[1000 * row[method]['median_cpu_seconds'] for row in rows if row['kind'] == kind and row['degree'] == degree] for degree in degrees]
            ax.plot(degrees, [statistics.median(group) for group in groups], marker=marker, color=color, label=method)
            ax.fill_between(degrees, [min(group) for group in groups], [max(group) for group in groups], color=color, alpha=.15)
        ax.set(xscale='log', yscale='log', xlabel='Input divisor degree', ylabel='Complete translation CPU (ms)', title=title)
        ax.set_xticks(degrees, labels=degrees)
        ax.grid(True, alpha=.2)
        ax.legend()
    fig.suptitle('Trace versus dense matrix translation; complete calls')
    fig.tight_layout(rect=(0, .16, 1, .93))
    fig.text(.5, .025, 'Two primes near 10⁶; three fresh-process samples per cell. Shading spans prime-specific medians.\nShared host, one native thread. Includes lifting and exceptions; excludes input-divisor construction.', ha='center', fontsize=9)
    for extension in ('png', 'pdf'):
        fig.savefig(ROOT / ('trace_scaling.' + extension), dpi=180, bbox_inches='tight')
    plt.close(fig)


if __name__ == '__main__':
    main()
