#!/usr/bin/env python3
"""Bookkeeping of native values/hashes only; publication remains explicit."""
import json
from pathlib import Path
import subprocess
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]
REL=str(HERE.relative_to(ROOT))


def bench(*args):
    subprocess.run(['python3','tools/benchrec.py',*map(str,args)],cwd=ROOT,check=True)


def metric(record,name,value,unit,n,note):
    bench('metric',record,'--name',name,'--value',value,'--unit',unit,'--n',n,'--better','neutral','--note',note)


v=json.loads((HERE/'summary.json').read_text())
records=[
 'benchmarks/records/20261119T1908Z-conductor-prime33555419-crossing.json',
 'benchmarks/records/20261119T1914Z-conductor33555419-endpoint-pilot.json',
 'benchmarks/records/20261119T1918Z-conductor33555419-scalars.json']
for record in records:
    bench('add-raw',record,f'{REL}/raw-results.tar.xz')
metric(records[0],'certified_end_conductor_ratio',v['End_conductor_target'],'ratio',1,
       'Fixed ordinary CM source1/target33555419;26-bit prime ratio, not original80-bit completion; arithmetic certificate plus standard order argument.')
for row in v['whole_construction_attempts']:
    wall=dict(x.split('=') for x in row['resource_text'].split())['wall_s']
    metric(records[0],row['label']+'.whole_constructor_seconds',wall,'wall_s',1,
           'L0 whole quotient construction plus supplied point evaluations;1 auxiliary probe or33 actual inputs; not separated phase costs; RSS unobserved.')
for row in v['ECDLP']['groups']:
    if row['mean_solve_seconds'] is not None:
        metric(records[1],row['curve_slug']+'.rho.mean_solve',row['mean_solve_seconds'],'wall_s',row['attempts'],
               'Native L0 mean, one shared target/two rounds; setup included, canonicalizations unpriced; no endpoint ratio admission.')
for line in (HERE/'results/pilot-l0/records.jsonl').read_text().splitlines():
    r=json.loads(line)
    if r['arm']=='ic-bounded':
        metric(records[1],f'ic_exhausted.seq{r["seq"]}.attempt_seconds',r['time']['solve_wall_ns']/1e9,'wall_s',1,
               'Direct native ns-to-seconds conversion;10000 trials exhausted, zero relations, no recovered logarithm; not solve time.')
for row in v['scalar']['descriptive_by_curve']:
    for kind in ['mean','median']:
        metric(records[2],row['curve_slug']+f'.{kind}_scalar_per_mul',row[kind+'_ns_per_mul']/1e9,'wall_s',row['samples'],
               'Native ns converted to seconds; L0 batch64 throughput; setup/verification/output excluded; warmups excluded from statistic.')
summaries=[
 'Full fixed prime33555419 End-conductor crossing with33 original-subgroup images and normalized dual controls;3 identical full constructor outputs. Failed launcher retained.',
 'Original53bit subgroup:4 verified rho solves and4 IC exhaustions, one shared target/two rounds;8 exact native replays and independent PARI checks. Failed plan preserved.',
 '32 bases/model, source/image/u2,6 measured rounds plus warmup;672 independently checked scalar batches. Failed launcher preserved.']
for record,summary in zip(records,summaries):
    p=ROOT/record;d=json.loads(p.read_text());assert d['status']=='open'
    d['summary']=summary+' All times descriptive L0; record remains open pending authenticated S3 publication.'
    d['reports']={'pdf':f'{REL}/reports/report.pdf','svg':[f'{REL}/reports/rho.svg',f'{REL}/reports/scalar.svg',f'{REL}/reports/map.svg'],'png':[f'{REL}/reports/rho.png',f'{REL}/reports/scalar.png']}
    d['validation']=f'{REL}/summary.json'
    p.write_text(json.dumps(d,indent=2,sort_keys=True)+'\n')
