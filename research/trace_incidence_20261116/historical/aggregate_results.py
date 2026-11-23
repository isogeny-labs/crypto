"""Join frozen panels without selecting successful instances or fitting exponents."""
import csv
import hashlib
import json
from pathlib import Path
import statistics

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent


def write_csv(path, rows):
    with path.open('w', newline='') as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def main():
    incidence = json.loads((ROOT / 'incidence/benchmark_results.json').read_text())['records']
    controls = json.loads((ROOT / 'repo_controls/pair_benchmark.json').read_text())
    assert controls['complete'] == 60, 'Control panel is still running'
    fresh_memory = json.loads((ROOT / 'repo_controls/incidence_parity_and_memory.json').read_text())
    quotient_rss = {row['id']: row['vm_hwm_kib'] for row in fresh_memory['fresh_memory_quotient']}
    normalized = []
    for row in incidence:
        if row['kind'] != 'natural':
            continue
        normalized.append({'fixture_id': row['fixture_id'], 'B': row['B'],
            'algorithm': 'incidence-' + row['arm'], 'status': row['status'],
            'oracle_cpu_seconds': row['solver_total_cpu_seconds'],
            'pool_generation_cpu_seconds': row['pool_generation_cpu_seconds'],
            'pool_plus_oracle_cpu_seconds': row['complete_call_plus_pool_cpu_seconds'],
            'peak_process_rss_kib': row['peak_process_rss_kib']})
    for key, algorithm in [('native_pairs', 'cpp-pairs'), ('quotient_scans', 'quotient-target-scan')]:
        assert len(controls[key]) == 60
        for row in controls[key]:
            normalized.append({'fixture_id': row['fixture_id'], 'B': row['B'],
                'algorithm': algorithm, 'status': row['status'],
                'oracle_cpu_seconds': row['median_cold_cpu_seconds'],
                'pool_generation_cpu_seconds': row['generation_cpu_seconds'],
                'pool_plus_oracle_cpu_seconds': row['median_cold_cpu_seconds'] + row['generation_cpu_seconds'],
                'peak_process_rss_kib': (quotient_rss[row['fixture_id']] if key == 'quotient_scans'
                    else max(trial['peak_rss_kib'] for trial in row['trials']))})
    algorithms = ['cpp-pairs', 'quotient-target-scan', 'incidence-horner', 'incidence-blocked']
    summary = []
    retries = []
    for algorithm in algorithms:
        for B in (8, 16, 32, 64, 128):
            group = [row for row in normalized if row['algorithm'] == algorithm and row['B'] == B]
            assert len(group) == 12
            summary.append({'algorithm': algorithm, 'B': B, 'natural_attempts': len(group),
                'hits': sum(row['status'] == 'relation' for row in group),
                'misses': sum(row['status'] == 'none' for row in group),
                'median_oracle_ms': 1000 * statistics.median(row['oracle_cpu_seconds'] for row in group),
                'median_pool_generation_ms': 1000 * statistics.median(row['pool_generation_cpu_seconds'] for row in group),
                'median_pool_plus_oracle_ms': 1000 * statistics.median(row['pool_plus_oracle_cpu_seconds'] for row in group),
                'max_absolute_process_rss_kib': max(row['peak_process_rss_kib'] for row in group)})
            for curve in range(3):
                ordered = sorted([row for row in group if f'_c{curve}_' in row['fixture_id']], key=lambda row: row['fixture_id'])
                used = []
                for row in ordered:
                    used.append(row)
                    if row['status'] == 'relation':
                        break
                solved = used[-1]['status'] == 'relation'
                retries.append({'algorithm': algorithm, 'B': B, 'curve': curve,
                    'status': 'solved' if solved else 'right-censored-after-four-pools',
                    'charged_fixture_ids': [row['fixture_id'] for row in used],
                    'pools_charged': len(used),
                    'oracle_cpu_seconds': sum(row['oracle_cpu_seconds'] for row in used),
                    'pool_generation_cpu_seconds': sum(row['pool_generation_cpu_seconds'] for row in used),
                    'total_cpu_seconds': sum(row['pool_plus_oracle_cpu_seconds'] for row in used)})
    write_csv(ROOT / 'incidence_attempts.csv', normalized)
    write_csv(ROOT / 'incidence_scaling.csv', summary)
    sources = ['incidence/benchmark_results.json', 'incidence/benchmark_summary.json',
               'repo_controls/pair_benchmark.json', 'prime_fixtures/manifest.json',
               'independent_pool_audit.json', 'cross_check_results.json',
               'repo_controls/incidence_parity_and_memory.json']
    retry_counts = {algorithm: {
        'solved': sum(row['algorithm'] == algorithm and row['status'] == 'solved' for row in retries),
        'right_censored': sum(row['algorithm'] == algorithm and row['status'] != 'solved' for row in retries)
    } for algorithm in algorithms}
    report = {'schema': 'integrated-ecdlp-experiment/v1',
        'result': 'Measured divisor-translation improvement; no faster complete prime-field ECDLP algorithm established. Implicit incidence variants lose to the strong native pair control.',
        'timing_scope': 'Complete fresh-state algorithm calls, with public pool generation added per attempt. Library/process startup retained in raw panels separately. Pinned CPUs on a shared host, one native thread; exploratory timings.',
        'memory_scope': 'Absolute fresh-process peaks include runtime/backend; these are not incremental algorithm allocation. Quotient memory uses the separate fresh VmHWM audit, not the record-retaining timing driver. Pair C++ uses O(B) auxiliary storage; no B^2 pair table.',
        'pool_design': '60 natural fixtures: three curves and four fixed public seeds at each B, r approximately 8 B^3. Planted and negative-alias controls are excluded from performance statistics.',
        'retry_scope': 'Fixed seed order 0..3 per curve, stopping at first informative relation; preceding misses charged. Four misses are censored, not a completed solve or proof of failure.',
        'seeded_retry_counts': retry_counts,
        'complexity_target': 'For B=Theta(r^(1/3)), a complete O(B^(3/2-epsilon) polylog(r)) algorithm would beat rho. Neither candidate supplies that bound; no exponent fitted to toy timings.',
        'structured_scope': 'Random labelled pools and structured PKM/isogeny factor bases remain separate. No free-label construction assumed.',
        'source_sha256': {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest() for name in sources},
        'summary': summary, 'complete_seeded_retries': retries}
    (ROOT / 'experiment_summary.json').write_text(json.dumps(report, indent=2) + '\n')
    fig, axes = plt.subplots(1, 2, figsize=(11, 5))
    styles = [('cpp-pairs', '#175477', 'o', 'C++ batched pairs'),
              ('quotient-target-scan', '#986a19', 's', 'Repository quotient/GCD port'),
              ('incidence-horner', '#b14934', '^', 'Implicit GCD: Horner'),
              ('incidence-blocked', '#136f63', 'D', 'Implicit GCD: blocked')]
    for ax, key, title in zip(axes, ('oracle_cpu_seconds', 'pool_plus_oracle_cpu_seconds'),
                              ('Complete oracle call', 'Public pool generation + oracle')):
        for algorithm, color, marker, label in styles:
            degrees = [8, 16, 32, 64, 128]
            groups = [[1000 * row[key] for row in normalized if row['algorithm'] == algorithm and row['B'] == B] for B in degrees]
            ax.plot(degrees, [statistics.median(group) for group in groups], color=color, marker=marker, label=label)
            quartiles = [statistics.quantiles(group, n=4) for group in groups]
            ax.fill_between(degrees, [q[0] for q in quartiles], [q[2] for q in quartiles], color=color, alpha=.10)
        ax.set(xscale='log', yscale='log', xlabel='Points per pool B', ylabel='CPU per natural attempt (ms)', title=title)
        ax.set_xticks([8, 16, 32, 64, 128], labels=[8, 16, 32, 64, 128])
        ax.grid(True, alpha=.2)
    axes[0].legend(fontsize=8)
    fig.suptitle('Random labelled three-pool incidence, r ≈ 8B³')
    fig.tight_layout(rect=(0, .15, 1, .93))
    fig.text(.5, .02, 'Medians and interquartile bands across 12 natural cases per size, including misses.\nInput preprocessing, witness recovery and scalar verification charged; shared host, one thread. No fitted exponent.', ha='center', fontsize=9)
    for extension in ('png', 'pdf'):
        fig.savefig(ROOT / ('incidence_scaling.' + extension), dpi=180, bbox_inches='tight')
    plt.close(fig)
    print(json.dumps({'summary': summary, 'output': str(ROOT / 'experiment_summary.json')}, indent=2))


if __name__ == '__main__':
    main()
