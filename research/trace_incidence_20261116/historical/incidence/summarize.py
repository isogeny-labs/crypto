"""Derive cold-cost and exact seed-order retry comparisons, no re-timing."""
import hashlib
import json
from pathlib import Path
import statistics

ROOT=Path(__file__).parent


def main():
    primary_path=ROOT/'benchmark_results.json'
    control_path=ROOT.parent/'repo_controls/pair_benchmark.json'
    primary=json.loads(primary_path.read_text())['records']
    control=json.loads(control_path.read_text())
    if control['complete']!=60:raise RuntimeError('Strong-control panel incomplete')
    native={r['fixture_id']:r for r in control['native_pairs']}
    summary=[];retry=[]
    for B in (8,16,32,64,128):
      for arm in ('horner','blocked'):
        rows=[r for r in primary if r['kind']=='natural' and r['B']==B and r['arm']==arm]
        summary.append({'B':B,'arm':arm,'cases':len(rows),
            'median_oracle_cpu_seconds':statistics.median(r['solver_total_cpu_seconds'] for r in rows),
            'median_pool_plus_oracle_cpu_seconds':statistics.median(r['complete_call_plus_pool_cpu_seconds'] for r in rows),
            'median_cold_process_plus_pool_cpu_seconds':statistics.median(r['complete_call_plus_pool_cpu_seconds']+r['process_startup_and_serialization_cpu_seconds'] for r in rows),
            'median_paired_oracle_slowdown_vs_native_pairs':statistics.median(r['solver_total_cpu_seconds']/native[r['fixture_id']]['median_cold_cpu_seconds'] for r in rows),
            'median_paired_pool_plus_oracle_slowdown_vs_native_pairs':statistics.median(r['complete_call_plus_pool_cpu_seconds']/(native[r['fixture_id']]['generation_cpu_seconds']+native[r['fixture_id']]['median_cold_cpu_seconds']) for r in rows)})
      for curve in (0,1,2):
       for arm in ('horner','blocked','cpp-pair'):
        attempts=[];oracle=pool=0.;status='censored_after_four_pools'
        for seed in range(4):
            fid=f'b{B:03d}_c{curve}_natural_s{seed}'
            if arm=='cpp-pair':
                r=native[fid];cost=r['median_cold_cpu_seconds'];gen=r['generation_cpu_seconds']
            else:
                r=next(r for r in primary if r['fixture_id']==fid and r['arm']==arm)
                cost=r['solver_total_cpu_seconds'];gen=r['pool_generation_cpu_seconds']
            oracle+=cost;pool+=gen;attempts.append({'fixture_id':fid,'status':r['status']})
            if r['status']=='relation':status='informative_relation';break
        retry.append({'B':B,'curve':curve,'arm':arm,'status':status,'attempts':attempts,
            'charged_oracle_cpu_seconds':oracle,'charged_pool_generation_cpu_seconds':pool,
            'charged_pool_plus_oracle_cpu_seconds':oracle+pool,
            'interpretation':'Finite success cost only for informative_relation; all-miss row is a spent-work lower bound, not an unsatisfiability claim.'})
    result={'schema':'incidence-analysis/v1','primary_sha256':hashlib.sha256(primary_path.read_bytes()).hexdigest(),
        'native_control_sha256':hashlib.sha256(control_path.read_bytes()).hexdigest(),
        'scope':'Derived exploratory comparisons, no new timings. Native C++ pair baseline versus Python-FLINT elimination, same public fixtures; implementation languages differ.',
        'cost_scope':'Pool+oracle includes all computation in initialized processes. Cold primary adds separate process/import/serialization CPU. Seeded retry uses persistent process scope and explicit failed pools.',
        'summary':summary,'seed_order_retry':retry}
    (ROOT/'analysis_results.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'summary_rows':len(summary),'retry_rows':len(retry),'retry_censored_rows':sum(r['status']!='informative_relation' for r in retry)},indent=2))


if __name__=='__main__':main()
