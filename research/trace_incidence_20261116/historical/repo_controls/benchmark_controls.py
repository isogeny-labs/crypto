#!/usr/bin/env python3
"""Bounded natural-pool benchmark: strong C++ pairs and existing m2 target scan.

Five batches per fixture. Native repetition counts target 0.085 CPU seconds
using a 32-call probe; warm batch durations may fall below that target and
their actual elapsed times are retained. Every repetition reparses and
rebuilds its index. Quotient calls are repeated to at least 0.08 CPU seconds
per batch. Output includes every
miss and all preprocessing/recovery. Repetitions estimate the deterministic
cost on one fixture, not independent relation trials. Costs per successful
relation use all 12 independently generated natural fixtures at each B.
"""
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import statistics
import subprocess
import sys
import time
from controls import solve_pair, solve_pair_cpp, solve_quotient_scan, verify_witness

ROOT=Path(__file__).resolve().parent
FIXTURES=Path('/workspace/ecdlp_research/prime_fixtures')


def one_quotient_batch(fixture,threshold=0.08):
    begin_cpu,begin_wall=time.process_time(),time.perf_counter()
    calls=0
    phases={}
    while True:
        result=solve_quotient_scan(fixture)
        calls+=1
        for k,v in result['phase_cpu_seconds'].items():
            phases[k]=phases.get(k,0)+v
        elapsed=time.process_time()-begin_cpu
        if elapsed>=threshold:
            break
    result.update({'batch_repetitions':calls,'batch_cpu_seconds':elapsed,
        'batch_wall_seconds':time.perf_counter()-begin_wall,
        'cold_cpu_seconds':elapsed/calls,
        'mean_phase_cpu_seconds':{k:v/calls for k,v in phases.items()}})
    return result


def summarize(rows,algorithm):
    out=[]
    for B in sorted({r['B'] for r in rows}):
        selected=[r for r in rows if r['B']==B]
        count=len(selected)
        successes=sum(r['status']=='relation' for r in selected)
        costs=[r['median_cold_cpu_seconds'] for r in selected]
        generation=[r['generation_cpu_seconds'] for r in selected]
        out.append({'algorithm':algorithm,'B':B,'fixtures':count,
            'successes':successes,'misses':count-successes,
            'median_oracle_cpu_seconds':statistics.median(costs),
            'mean_oracle_cpu_seconds':statistics.mean(costs),
            'median_generation_cpu_seconds':statistics.median(generation),
            'mean_generation_cpu_seconds':statistics.mean(generation),
            'median_generation_plus_oracle_cpu_seconds':statistics.median(
                x+y for x,y in zip(costs,generation)),
            'empirical_seeded_retry_oracle_cpu_seconds':sum(costs)/successes if successes else None,
            'empirical_seeded_retry_generation_plus_oracle_cpu_seconds':
                (sum(costs)+sum(generation))/successes if successes else None,
            'empirical_attempts_per_success':count/successes if successes else None,
            'max_peak_rss_kib':max(max(t['peak_rss_kib'] for t in r['trials']) for r in selected)})
    return out


def main():
    os.sched_setaffinity(0,{2})
    paths=[p for p in sorted(FIXTURES.glob('b*_natural_s*.json'))
           if not p.name.endswith('.validation.json')]
    rows_native=[];rows_quotient=[]
    for n,path in enumerate(paths):
        fixture=json.loads(path.read_text())
        # Independent validation outside every attack timer; no validation
        # input is passed to the solver.
        truth=json.loads(path.with_suffix('.validation.json').read_text())
        probe=solve_pair_cpp(fixture,repetitions=32)
        reps=max(1,math.ceil(0.085/(probe['batch_cpu_seconds']/32)))
        native=[]
        for _ in range(5):
            trial=solve_pair_cpp(fixture,repetitions=reps)
            trial['cold_cpu_seconds']=trial['batch_cpu_seconds']/trial['batch_repetitions']
            native.append(trial)
        quotient=[one_quotient_batch(fixture) for _ in range(5)]
        for arm,trials in (('cpp-pair',native),('m2-quotient-scan',quotient)):
            for trial in trials:
                assert (trial['status']=='relation')==truth['signed_zero_sum_exists'],(path,arm)
                if trial['status']=='relation':
                    check=verify_witness(fixture,trial['witness'])
                    assert check['recovered_scalar_verified'] and check['k']==truth['secret_k']
            row={'fixture_id':fixture['id'],'B':fixture['B'],'kind':fixture['kind'],
                'curve':{k:fixture['curve'][k] for k in ('p','a','b','r')},
                'status':trials[0]['status'],
                'generation_cpu_seconds':fixture['generation']['cpu_seconds'],
                'generation_wall_seconds':fixture['generation']['wall_seconds'],
                'median_cold_cpu_seconds':statistics.median(t['cold_cpu_seconds'] for t in trials),
                'min_cold_cpu_seconds':min(t['cold_cpu_seconds'] for t in trials),
                'max_cold_cpu_seconds':max(t['cold_cpu_seconds'] for t in trials),
                'fixture_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
                'trials':trials}
            (rows_native if arm=='cpp-pair' else rows_quotient).append(row)
        print(json.dumps({'complete':n+1,'of':len(paths),'id':fixture['id'],
            'status':native[0]['status'],
            'cpp_pair_ms':rows_native[-1]['median_cold_cpu_seconds']*1000,
            'quotient_scan_ms':rows_quotient[-1]['median_cold_cpu_seconds']*1000}),flush=True)
        # Write recoverable progress after each complete fixture.
        result={'schema':'repo-controls-benchmark/v1','natural_fixtures':len(paths),
            'complete':n+1,'cpu_affinity':[2],'threads':1,'batches_per_fixture':5,
            'quotient_minimum_batch_cpu_seconds':0.08,
            'native_probe_target_batch_cpu_seconds':0.085,
            'native_calibration_note':'32-call cold probe; actual warmed batch durations may fall below target; use measured batch CPU and repetitions',
            'renewal_estimator_scope':'Legacy empirical_seeded_retry fields are sum(cost of all independent natural pools)/number of successes, not a fixed-seed stop-at-first-hit sequence',
            'host':platform.platform(),
            'scope':'Portable C++ control and Python-FLINT quotient port; exact Rust binary not replayed. Toy prime curves, exploratory shared-host wall times.',
            'compiler':subprocess.check_output(['g++','--version'],text=True).splitlines()[0],
            'compile_command':'g++ -O3 -std=c++17 -Wall -Wextra pair_control.cpp -o pair_control',
            'native_pairs':rows_native,'quotient_scans':rows_quotient,
            'summary_native_pairs':summarize(rows_native,'cpp-pair'),
            'summary_quotient_scans':summarize(rows_quotient,'m2-quotient-scan'),
            'source_sha256':{name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest()
                for name in ('controls.py','pair_control.cpp','benchmark_controls.py','pair_control')}}
        (ROOT/'pair_benchmark.json').write_text(json.dumps(result,indent=2)+'\n')


if __name__=='__main__':
    main()
