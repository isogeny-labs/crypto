#!/usr/bin/env python3
"""Calls the native evaluator/PARI oracle; only compares serialized receipts."""
import json
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
CANDIDATE = '/tmp/conductor-twist-20261119/complete_twisted_pipeline'
BASELINE = '/tmp/conductor-prepared-norm-20261118/complete_pipeline'


def call(label, argv, source, expected=0):
    with source.open('rb') as inp, (OUT/(label+'.stdout')).open('wb') as out, (OUT/(label+'.stderr')).open('wb') as err:
        result = subprocess.run(argv, stdin=inp, stdout=out, stderr=err, timeout=60)
    (OUT/(label+'.exit')).write_text(str(result.returncode)+'\n')
    commands.append({'label': label, 'argv': argv, 'stdin': str(source), 'exit': result.returncode})
    assert result.returncode == expected, label
    if expected == 0:
        assert not (OUT/(label+'.stderr')).read_bytes(), label
        return json.loads((OUT/(label+'.stdout')).read_bytes())
    assert not (OUT/(label+'.stdout')).read_bytes(), label


OUT = HERE/'qualification-a'
OUT.mkdir()
commands=[]
rows=[]
for path in sorted((HERE/'inputs').glob('*.txt')):
    scale=path.stem.split('-d')[1]
    for mode, extra in [('affine',[]),('prepared',['--prepared-norm','--projective-batch','7'])]:
        label=path.stem+'-'+mode
        row=call(label,[CANDIDATE,'--evaluate','64','1','--twist-scale',scale,'--pari-reference',*extra],path)
        assert row['pari_direct_reference'] and row['status']=='complete_evaluation'
        rows.append({'label':label,'point_inputs':len(row['images']),'direct_reference':True})
for name,window,block in [('control19',4,1),('control5',2,2),('control10009',64,64)]:
    source=ROOT/'research/prime_conductor_gap80_20261031/evaluator-20261118/inputs'/(name+'.txt')
    args=['--evaluate',str(window),str(block),'--prepared-norm','--projective-batch','7']
    call(name+'-old',[BASELINE,*args],source)
    call(name+'-new',[CANDIDATE,*args],source)
    assert (OUT/(name+'-old.stdout')).read_bytes()==(OUT/(name+'-new.stdout')).read_bytes()
source=HERE/'inputs/p71-d7.txt'
for label,args in [
    ('zero',['--twist-scale','0']),('negative',['--twist-scale','-1']),
    ('equal-p',['--twist-scale','71']),('above-p',['--twist-scale','72']),
    ('duplicate',['--twist-scale','2','--twist-scale','2']),
    ('missing',['--twist-scale']),('unknown',['--twist-scale=2']),
    ('wrong-source',['--twist-scale','1'])]:
    call('reject-'+label,[CANDIDATE,'--evaluate','64','2',*args],source,2)
(OUT/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
(HERE/'qualification.json').write_text(json.dumps({'scope':'native correctness only; no benchmark timing','rows':rows,'default_byte_matches':3,'negative_cli_controls':8,'full_80bit_crossing_complete':False},indent=2)+'\n')
print('qualified',len(rows),'direct PARI maps;3 default byte matches;8 negative controls')
