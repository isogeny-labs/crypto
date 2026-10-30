#!/usr/bin/env python3
"""Lossless aggregate-workflow experiment.export/v1 adapter; never runs native code.

Requires explicit invoked-workflow receipts, never constructs missing invocations.
The official checker remains authoritative; failed admission is a raw-only result.
"""
import argparse
import contextvars
import datetime
import hashlib
import importlib.util
import importlib.metadata
import json
import math
import os
import resource
from pathlib import Path
import shutil
import subprocess
import sys
import time

PINS = {
 'tools/experiment_protocol.py': '0f46a2b856bb1df2d155c4a676b46c6118d82c45cf8e5cc0492315aeb45c84e4',
 'schemas/experiment-export.v1.json': '108f0ae9c1be3ed141bca3c22216eba049f2c76677f11013109a4740f19bd757',
 'schemas/experiment-protocol.v1.json': 'a9b948f2be323f862b98301eb7a5aa147ecc29db35041737f0aa3dfd03ad2443',
}
DEADLINE = contextvars.ContextVar('adapter_deadline', default=None)
CHECKER_DEPENDENCIES = {'jsonschema':'4.23.0','jsonschema-specifications':'2025.9.1','referencing':'0.37.0','rpds-py':'2026.9.1','attrs':'26.1.0','typing-extensions':'4.16.0'}
ARMS = ('native-sat', 'copied-source-controls')
REPRESENTATION = 'two-invoked-software-workflows-with-subordinate-native-ledgers/v1'

def require(condition, text):
    if not condition: raise ValueError(text)

def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(1024**2), b''):
            require(DEADLINE.get() is None or time.monotonic() < DEADLINE.get(), 'unified export deadline exhausted')
            h.update(block)
    return h.hexdigest()

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False).encode()

def pairs(items):
    result = {}
    for key, value in items:
        require(key not in result, 'duplicate JSON key: ' + key)
        result[key] = value
    return result

def load(path):
    path = Path(path)
    require(path.is_file() and not path.is_symlink() and path.stat().st_size <= 32*1024**2, 'invalid JSON input')
    return json.loads(path.read_bytes(), object_pairs_hook=pairs,
                      parse_constant=lambda x: (_ for _ in ()).throw(ValueError(x)))

def write(path, value):
    with Path(path).open('xb') as f:
        f.write(canonical(value)+b'\n'); f.flush(); os.fsync(f.fileno())

def exact(value):
    require(type(value) is int and value >= 0, 'invalid exact nonnegative count')
    return str(value) if value > 2**53-1 else value

def entry(path, base, kind='raw'):
    return {'path': os.path.relpath(path, base), 'sha256': sha(path), 'bytes': path.stat().st_size, 'kind': kind}

def assert_checker_runtime(settings):
    require(sha(Path(sys.executable))==settings['checker_python_sha256'],'driver interpreter differs from frozen checker')
    actual={name:importlib.metadata.version(name) for name in CHECKER_DEPENDENCIES}
    require(actual==settings['dependencies']==CHECKER_DEPENDENCIES,'checker dependencies differ from frozen requirements')
    import jsonschema
    require(hasattr(jsonschema,'Draft202012Validator'),'checker runtime lacks Draft202012Validator')
    return {'interpreter':sys.executable,'interpreter_sha256':sha(Path(sys.executable)),
            'python_version':sys.version,'dependencies':actual,'jsonschema_module':jsonschema.__file__,
            'draft202012_validator_available':True}

def official(tooling):
    for rel, expected in PINS.items(): require(sha(tooling/rel) == expected, 'official tooling hash mismatch: '+rel)
    spec = importlib.util.spec_from_file_location('pinned_official', tooling/'tools/experiment_protocol.py')
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    return module

def admit(attempt):
    receipt = load(attempt/'receipt.json'); protocol = load(attempt/'protocol.json')
    fixture = protocol['extensions']['ml_dataset_sat_fixture']
    require(fixture.get('export_representation') == REPRESENTATION, 'aggregate workflow representation not preregistered')
    require(protocol['sample_size']['instances'] == 1 and protocol['sample_size']['replicates'] == 1 and
            protocol['sample_size']['warmups'] == 0, 'aggregate population must be two arms x one instance x one replicate')
    require([a['id'] for a in protocol['arms']] == list(ARMS), 'wrong workflow arms')
    require(receipt['protocol']['file_sha256'] == sha(attempt/'protocol.json') and
            receipt['protocol']['body_sha256'] == protocol['freeze']['protocol_sha256'], 'protocol receipt mismatch')
    require(sha(attempt/'frozen-crypto') == fixture['binary_sha256'], 'binary mismatch')
    require(receipt['native_preflight']['source_commit'] == fixture['source_commit'] and
            receipt['native_preflight']['binary_sha256'] == fixture['binary_sha256'] and
            receipt['native_preflight']['source_dirty'] is False, 'source preflight mismatch')
    require({'protocol.json','frozen-crypto'} <= set(receipt['retained_protocol']['frozen_files']), 'incomplete retained bindings')
    for rel, expected in receipt['retained_protocol']['frozen_files'].items():
        p = Path(rel); require(not p.is_absolute() and '..' not in p.parts, 'invalid retained path')
        require((attempt/p).is_file() and not (attempt/p).is_symlink() and sha(attempt/p) == expected,
                'retained protocol custody changed')
    workflows = receipt.get('workflows', [])
    require(len(workflows) == 2 and [w['arm'] for w in workflows] == list(ARMS), 'missing invoked workflow receipts; raw evidence only')
    all_names = [c['name'] for c in receipt['commands']]
    assigned = [n for w in workflows for n in w['command_names']]
    require(len(set(all_names)) == len(all_names) and len(set(assigned)) == len(assigned) and set(assigned)==set(all_names),
            'workflow command custody is not a partition')
    for w in workflows:
        require(w.get('invoked') is True and w.get('started_utc') and w.get('finished_utc') and
                w.get('status') in ('succeeded','failed','timeout','interrupted','cancelled','infra_error'), 'uninvoked or unterminated workflow')
        require(w.get('reason'), 'workflow terminal reason required')
    candidates=receipt['candidates']; ids=[c['cell_index'] for c in candidates]
    require(len(set(ids))==len(ids) and set(ids).isdisjoint(receipt['unrun_cells']) and
            sorted(ids+receipt['unrun_cells'])==list(range(16)), 'candidate/unrun ledger must cover exact frozen sixteen cells')
    controls=receipt['controls']
    require(len({c['name'] for c in controls})==len(controls) and
            set(c['name'] for c in controls)<= {'map-substitution','forged-reason','unpaired-decision','dynamic-clause'}, 'invalid controls ledger')
    if workflows[0]['status']=='succeeded':
        require(receipt.get('selected_pair') and receipt.get('transition_witness'), 'native workflow success without qualified fixture')
    if workflows[1]['status']=='succeeded':
        require(workflows[0]['status']=='succeeded' and len(controls)==4 and all(c.get('passed') is True for c in controls), 'controls success without exact four passing controls')
    return receipt, protocol

def export_attempt(attempt, destination, tooling, checker_python, timeout=30):
    attempt, destination, tooling = Path(attempt).resolve(), Path(destination).resolve(), Path(tooling).resolve()
    require(not destination.exists(), 'new export directory required')
    started=time.monotonic()
    cpu_before=resource.getrusage(resource.RUSAGE_SELF)
    children_before=resource.getrusage(resource.RUSAGE_CHILDREN)
    deadline=started+timeout
    destination.mkdir(parents=True)
    gate={'schema':'ml-dataset.unified-export-gate/v1', 'status':'failed', 'native_execution_performed':False,
          'scientific_samples_added':0, 'clock_valid_for_comparison':False}
    deadline_token=DEADLINE.set(deadline)
    try:
        api=official(tooling)
        receipt, protocol=admit(attempt)
        settings=protocol['extensions']['ml_dataset_sat_fixture']['unified_export_tooling']
        gate['checker_runtime']=assert_checker_runtime(settings)
        require(settings['file_sha256']==PINS and sha(Path(checker_python))==settings['checker_python_sha256'], 'checker/interpreter not frozen in protocol')
        # Snapshot mutable finalization inputs. All other existing artifacts are
        # referenced and hash guarded; no duplicate multi-GiB dataset copy.
        existing=[]
        for p in sorted(attempt.rglob('*')):
            if p == destination or destination in p.parents: continue
            require(not p.is_symlink(), 'symlink in retained attempt')
            if p.is_file(): existing.append(p)
        before={str(p):sha(p) for p in existing}
        snapshots=destination/'snapshot'; snapshots.mkdir()
        for name in ('receipt.json','protocol.json','host.json','inventory.json'):
            if (attempt/name).exists(): shutil.copyfile(attempt/name,snapshots/name)
        for rel in PINS:
            target=destination/'tooling'/rel; target.parent.mkdir(parents=True,exist_ok=True); shutil.copyfile(tooling/rel,target)
        official(destination/'tooling')
        e=api.export_skeleton(protocol,'snapshot/protocol.json','local','SAT-workflow-'+destination.name)
        e['producer']={'runtime':'python','model':None,'session':None,'role':'software-validation-adapter'}
        f=protocol['extensions']['ml_dataset_sat_fixture']
        e['source']={'repo':protocol['repository'],'commit':f['source_commit'],'dirty':False}
        for arm in e['arms']: arm.update(binary_sha256=f['binary_sha256'],commit=f['source_commit'])
        e['host']['path']='snapshot/host.json'; e['host']['sha256']=sha(snapshots/'host.json')
        commands={c['name']:c for c in receipt['commands']}
        control_names=set(('map-substitution','forged-reason','unpaired-decision','dynamic-clause'))
        started_controls={n for n in control_names if 'control-'+n in commands or (attempt/'controls'/n).exists()} | {c['name'] for c in receipt['controls']}
        completed_controls={c['name'] for c in receipt['controls'] if c.get('status')!='started' and 'passed' in c}
        for w in receipt['workflows']:
            cs=[commands[n] for n in w['command_names']]
            counts={'workflow-invocations':1,'cli-command-receipts':len(cs),
                    'cli-command-nonzero':sum(c.get('exit_code') not in (0,None) for c in cs)}
            audits=[]
            if w['arm']=='native-sat':
                counts['candidate-cells-started']=len(receipt['candidates'])
                counts['candidate-cells-unrun']=len(receipt['unrun_cells'])
                checked=[c for c in receipt['candidates'] if c['status']=='checked']
                counts['verified-decisions']=sum(int(exact(c['verified_decisions'])) for c in checked)
                counts['qualified-transition']=int(receipt.get('transition_witness') is not None)
                for c in receipt['candidates']:
                    if c.get('audit_path'):
                        p=attempt/c['audit_path']; require(sha(p)==c['audit_sha256'],'audit hash mismatch'); audits.append(load(p))
            else:
                counts['controls-started']=len(started_controls)
                counts['controls-completed']=len(completed_controls)
                counts['controls-incomplete']=len(started_controls-completed_controls)
                counts['controls-passed']=sum(c.get('passed') is True for c in receipt['controls'])
            # CPU sums are diagnostic only. Native process peak RSS is a
            # cumulative child high-water mark, not a workflow memory maximum.
            cpu=lambda k: sum(c[k] for c in cs) if cs and all(k in c for c in cs) else None
            r=api.run_row(run_id=w['arm']+'-workflow',arm=w['arm'],instance_id='0',seed=20261030,
                status=w['status'],started_at=w['started_utc'],finished_at=w['finished_utc'],
                backend_ref='snapshot/receipt.json#/workflows/'+str(len(e['runs'])),
                measurements={'counted':{k:exact(v) for k,v in counts.items()},'wall_valid':False,
                              'cpu_user_seconds':cpu('child_user_cpu_seconds'),'cpu_sys_seconds':cpu('child_system_cpu_seconds'),
                              'counters':{}},
                isolation={'tier':'none','contended':None,'blockers':['Software correctness workflow; no timing isolation or performance claim']},
                verification={'status':'verified' if w['status']=='succeeded' else 'not_run' if w['reason']=='missing_native_precondition' else 'error' if w['status'] in ('failed','infra_error') else 'no_claim','verifier':'frozen driver mechanical predicates; reason='+w['reason']+'; no solver-global proof'},
                artifacts=[entry(snapshots/'receipt.json',destination,'receipt')])
            e['runs'].append(r)
        e['flow']=api.compute_flow(e['runs'])
        script=protocol['analysis_plan']['script']
        if script:
            matches=[p for p in existing if p.name==Path(script['path']).name and before[str(p)]==script['sha256']]
            require(len(matches)==1,'frozen analysis script unavailable or ambiguous')
            e['analysis']['script']=entry(matches[0],destination,'analysis')
        files=[]
        for p in existing:
            if p.name in ('receipt.json','protocol.json','host.json','inventory.json') and p.parent==attempt: continue
            files.append(entry(p,destination))
        files += [entry(p,destination,'receipt' if p.name=='receipt.json' else 'other') for p in snapshots.iterdir()]
        files += [entry(destination/'tooling'/rel,destination,'other') for rel in PINS]
        e['files']=files
        e['backend_record']={'representation':REPRESENTATION,'receipt':receipt,
            'cell_flow':{'planned':16,'started':len(receipt['candidates']),'unrun':receipt['unrun_cells'],
                         'unrun_reason':receipt.get('unrun_reason','stopped_before_admission')},
            'control_flow':{'planned':4,'started_names':sorted(started_controls),'completed_names':sorted(completed_controls),
                            'incomplete_names':sorted(started_controls-completed_controls),'unrun_names':sorted(control_names-started_controls)},
            'cost_accounting':'All command wall/CPU/high-water RSS and every native artifact retained by hash. No scoped counter is total solver cost; null remains unavailable. Adapter finalization/checker/archive work lies outside workflow CLI command sums; later gate and archive receipts are separate cost scopes.'}
        e['notes']=[{'kind':'observation','text':'Two actually invoked software workflows; subordinate cells/commands are not independent measured arm samples. All wall clocks invalid for comparison. '+
                    ('SOFTWARE UNIT FIXTURE ONLY; constructed records, no native execution or research evidence.' if receipt.get('software_unit_fixture') else 'Native outcomes remain bounded software-validation observations.'),'by':'adapter'}]
        write(destination/'export.json',e)
        findings=api.schema_errors(e,'experiment.export/v1') or api.lint_export(e,protocol,destination,True)
        require(not findings,'in-process official validation failed: '+repr(findings))
        for label,args in [('check-export',['check-export',str(destination/'export.json'),'--protocol',str(snapshots/'protocol.json'),'--verify-files']),
                           ('flow',['flow',str(destination/'export.json')])]:
            remaining=deadline-time.monotonic(); require(remaining>0,'unified export deadline exhausted')
            argv=[str(checker_python),str(destination/'tooling/tools/experiment_protocol.py'),*args]
            result=subprocess.run(argv,capture_output=True,timeout=remaining,check=False)
            (destination/(label+'.stdout')).write_bytes(result.stdout); (destination/(label+'.stderr')).write_bytes(result.stderr)
            gate[label]={'argv':argv,'exit_code':result.returncode,'stdout_sha256':sha(destination/(label+'.stdout')),'stderr_sha256':sha(destination/(label+'.stderr'))}
            require(result.returncode==0,'official '+label+' rejected export')
        official(destination/'tooling')
        require(sha(Path(checker_python))==settings['checker_python_sha256'],'checker interpreter changed')
        require(all(sha(Path(p))==h for p,h in before.items()),'raw attempt changed during adapter')
        gate.update(status='passed',export_sha256=sha(destination/'export.json'),
                    fixture_acceptance=(all(w['status']=='succeeded' for w in receipt['workflows']) and
                        receipt.get('status')=='native_fixture_passed_unified_export_pending' and
                        not receipt.get('retention_error') and bool(receipt.get('source_inventories_unchanged')) and
                        all(v is True for v in receipt['source_inventories_unchanged'].values())))
    except Exception as error:
        gate['error']=repr(error)
    finally:
        DEADLINE.reset(deadline_token)
    cpu_after=resource.getrusage(resource.RUSAGE_SELF)
    children_after=resource.getrusage(resource.RUSAGE_CHILDREN)
    gate['cost_scope']='adapter admission, hashing, serialization and official checker/flow only; excludes native workflows, later archive and final receipt writes'
    gate['adapter_cpu_user_seconds_diagnostic']=cpu_after.ru_utime-cpu_before.ru_utime
    gate['adapter_cpu_system_seconds_diagnostic']=cpu_after.ru_stime-cpu_before.ru_stime
    gate['checker_child_cpu_user_seconds_diagnostic']=children_after.ru_utime-children_before.ru_utime
    gate['checker_child_cpu_system_seconds_diagnostic']=children_after.ru_stime-children_before.ru_stime
    gate['elapsed_seconds_diagnostic']=time.monotonic()-started
    gate['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
    write(destination/'gate.json',gate)
    return gate

def main():
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('attempt','out','tooling','checker-python'): p.add_argument('--'+name,required=True)
    p.add_argument('--seconds',type=float,default=30)
    a=p.parse_args(); gate=export_attempt(a.attempt,a.out,a.tooling,a.checker_python,a.seconds)
    print(json.dumps(gate)); return 0 if gate['status']=='passed' else 2
if __name__=='__main__': sys.exit(main())
