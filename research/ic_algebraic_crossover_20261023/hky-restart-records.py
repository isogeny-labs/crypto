#!/usr/bin/env python3
"""Metadata only: hashes, host facts and mapping the Rust analysis to an export.

No solver is launched here. Equation building, closure and verdict/rank
validation and aggregation remain in the frozen native executable.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import platform
import subprocess
import sys

STUDY = Path(__file__).resolve().parent
ROOT = STUDY.parents[1]
PROTOCOL = STUDY / 'protocol-hky-restart-20261026.json'
RUN = STUDY / 'pipeline-receipts/completed-guess-hky-restart-20261026'
sys.path.insert(0, str(ROOT.parent / 'crypto-autoresearcher/tools'))
import experiment_protocol as protocol_tool

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def guard():
    p = json.loads(PROTOCOL.read_text())
    assert p['freeze']['status'] == 'frozen', 'draft protocol: no research launch'
    errors = protocol_tool.schema_errors(p,'experiment.protocol/v1') or protocol_tool.lint_protocol(p,STUDY)
    assert not errors, errors
    assert subprocess.check_output(['git','-C',str(ROOT.parent/'crypto-autoresearcher'),'rev-parse','HEAD'],text=True).strip() == p['extensions']['protocol_tools_commit'], 'protocol checker revision changed'
    assert p['arms'][0]['instrument']['binary_sha256'] == sha(p['extensions']['frozen_binary_path']), 'binary changed'
    assert p['arms'][1]['instrument'] == p['arms'][0]['instrument'], 'A/A instrument changed'
    assert sha(STUDY/'inputs/guess-ell8.jsonl') == p['extensions']['input_sha256'], 'inputs changed'
    assert sha(STUDY/'pipeline-receipts/guess-completion-20261025-01/receipt.json') == p['extensions']['producer_receipt_sha256'], 'producer changed'
    assert sha(ROOT/'examples/ic_guess_completed_audit.rs') == p['analysis_plan']['script']['sha256'], 'analysis changed'
    for filename, expected in p['extensions']['orchestration_sha256'].items():
        assert sha(STUDY/filename) == expected, 'orchestration changed: '+filename
    return p

def host():
    p = guard()
    cpu = next((s.split(':',1)[1].strip() for s in Path('/proc/cpuinfo').read_text().splitlines() if s.startswith('model name')),'unknown')
    info = {'cpu_model':cpu,'kernel_release':platform.release(),'logical_cpus':os.cpu_count(),
        'allowed_cpus':sorted(os.sched_getaffinity(0)),'threads':1,'binary_sha256':p['arms'][0]['instrument']['binary_sha256'],
        'instrument_source_commit':p['arms'][0]['instrument']['commit'],
        'repository_context':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
        'isolation_tier':'L0','wall_valid':False,'resource_caps':{'address_space_bytes':8589934592,'per_child_seconds':900},
        'load_and_psi':{str(f):f.read_text() for f in (Path('/proc/loadavg'),Path('/proc/pressure/cpu'),Path('/proc/pressure/memory')) if f.exists()},
        'environment_recorded':{'RAYON_NUM_THREADS':'1','OMP_NUM_THREADS':'1','HKY_CHUNK':'1024'},'credential_values_recorded':False}
    filename = 'host.json' if not (RUN/'host.json').exists() else 'host-resume-'+datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')+'.json'
    (RUN/filename).write_text(json.dumps(info,indent=2)+'\n')
    (RUN/'current-host.txt').write_text(filename+'\n')

def snapshot(final=False):
    p = guard()
    native = json.loads((RUN/'native-summary.json').read_text())
    mapped = []
    for row in native['rows']:
        folder = RUN/row['arm']/f"d{row['draw']}-k{row['guesses']}-b{row['branch']}"
        if not (folder/'ended.txt').exists(): continue
        status = json.loads((folder/'status.json').read_text())
        kind = row['classification']
        outcome = 'succeeded' if kind=='verified' else ('timeout' if kind=='timeout' else ('interrupted' if status.get('interrupted_by_restore') else 'failed'))
        record = {'run_id':folder.name+'-'+row['arm'],'arm':row['arm'],'instance_id':folder.name,'seed':0,'replicate':0,
            'warmup':False,'status':outcome,'wall_seconds':None,'tier':'L0','verification':'verified' if kind=='verified' else 'not_run'}
        if kind=='verified':
            for metric in ('closure_rank','full_columns','plain_rank','hky_rounds'): record[metric]=row['replay'][metric]
        mapped.append(record)
    with (RUN/'rows.jsonl').open('w') as out:
        for row in mapped: out.write(json.dumps(row,sort_keys=True)+'\n')
    if final:
        assert len(mapped)==248 and native['terminated']==248, 'incomplete: no final export'
        target=STUDY/'exports/hky-restart-20261026/export.json';target.parent.mkdir(parents=True,exist_ok=True)
        prefix=os.path.relpath(RUN,target.parent)
        e=protocol_tool.from_rows(RUN/'rows.jsonl',p,os.path.relpath(PROTOCOL,target.parent),'IC-balanced-guess-HKY-restart-20261026',{},prefix)
        e['host']['summary']=json.loads((RUN/'host.json').read_text());e['host']['path']=prefix+'/host.json'
        entry=protocol_tool.file_entry(RUN,'host.json','host');entry['path']=prefix+'/host.json';e['files'].append(entry)
        e['backend_record']={'scope':'native Boolean closure replication; every clock invalid','native_summary_status':native['status'],
            'checkpoint_policy':'application SHA256 manifest','producer_unknown_cost_unchanged':True}
        errors=protocol_tool.schema_errors(e,'experiment.export/v1') or protocol_tool.lint_export(e,p,None,False)
        assert not errors,errors
        protocol_tool.write_once(target,e)
        print('Final export written once:',target)

def restore():
    guard()
    subprocess.run(['sha256sum','--check','SHA256SUMS'],cwd=RUN,check=True,stdout=subprocess.DEVNULL)
    for started in RUN.glob('hky*/d*-k*-b*/started.txt'):
        folder=started.parent
        if not (folder/'ended.txt').exists():
            (folder/'status.json').write_text(json.dumps({'exit_code':None,'interrupted_by_restore':True,'scope':'completion absent from restored generation; no retry'})+'\n')
            (folder/'ended.txt').write_text(datetime.datetime.now(datetime.timezone.utc).isoformat()+'\n')
    print('Restore hashes verified; incomplete prior processes retained as interrupted.')

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('mode',choices=['guard','host','snapshot','final','restore']);args=a.parse_args()
    if args.mode=='guard':guard()
    elif args.mode=='host':host()
    elif args.mode=='restore':restore()
    else:snapshot(args.mode=='final')
