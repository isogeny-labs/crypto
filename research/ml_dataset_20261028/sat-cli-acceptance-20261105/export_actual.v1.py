#!/usr/bin/env python3
"""Read-only retained-evidence adapter; no native commands, analyzer run or generation."""
import hashlib,importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path('/var/tmp/sat-cli-custody-acceptance-20261103-v1')
TOOL=Path('/home/ec2-user/ml-dataset-resume-20261029/boolean-cli-canonical-tooling-snapshot-20261102/tools/experiment_protocol.py')
def require(ok,message):
    if not ok:raise RuntimeError(message)
def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for block in iter(lambda:f.read(65536),b''):h.update(block)
    return h.hexdigest()
def load(p):
    def unique(pairs):
        r={}
        for k,v in pairs:require(k not in r,'duplicate JSON field');r[k]=v
        return r
    return json.loads(p.read_text(),object_pairs_hook=unique)
def save(p,d):
    with p.open('x') as f:json.dump(d,f,indent=2);f.write('\n')
def file(p,kind='raw'):
    require(p.is_file() and not p.is_symlink(),'nonregular artifact');return {'path':str(p),'bytes':p.stat().st_size,'sha256':sha(p),'kind':kind}
def main():
    output=ROOT/'experiment.export.json';require(not output.exists() and not (ROOT/'experiment.export.refused.json').exists(),'existing adapter attempt; reconcile')
    pins=load(Path(__file__).with_name('tooling-pins.json'))
    for p,h in pins['files'].items():require(sha(Path(p))==h,'canonical tooling changed')
    for p,expected in pins['directories'].items():require({str(f.relative_to(p)):sha(f) for f in Path(p).rglob('*') if f.is_file() and f.suffix in ('.py','.so','.json')}==expected,'pinned dependency changed')
    spec=importlib.util.spec_from_file_location('canonical_tool',TOOL);tool=importlib.util.module_from_spec(spec);spec.loader.exec_module(tool)
    ready=load(ROOT/'ready.json');start=load(ROOT/'attempt-start.json');terminal=load(ROOT/'attempt-result.json');analysis=load(ROOT/'analysis.json');protocol=load(ROOT/'protocol.json')
    require(start['ready_sha256']==sha(ROOT/'ready.json') and start['protocol_sha256']==sha(ROOT/'protocol.json')==ready['protocol_sha256'],'original readiness/protocol mismatch')
    require(protocol['freeze']['status']=='frozen' and terminal.get('finished_at') and terminal['cleanup']['settled'] is True,'nonterminal or unfrozen attempt')
    require(terminal['status'] in ['PASS','FAIL'] and analysis['status'] in ['PASS','FAIL'],'unknown terminal status')
    require(protocol['extensions']['canonical_export_adapter']['sha256']==sha(Path(__file__)),'unbound export adapter')
    for n,h in ready['input_sha256'].items():require(sha(ROOT/n)==h,'frozen input changed')
    require(protocol['analysis_plan']['script']['sha256']==sha(ROOT/'driver.py'),'analysis instrument differs')
    paths=sorted(ROOT.rglob('*'));require(not any(p.is_symlink() for p in paths),'symlink evidence')
    evidence=[file(p) for p in paths if p.is_file()];e=tool.export_skeleton(protocol,str(ROOT/'protocol.json'),'local','sat-cli-initial-custody-acceptance-20261103-v1')
    dirty=None;clean=[]
    for name in ['source-before','source-after']:
        path=ROOT/(name+'.json')
        if path.exists():
            for row in load(path):
                if row['argv'][-3:]==['status','--porcelain=v1','--untracked-files=all'] and row['exit_code']==0 and not row.get('timed_out'):
                    if row['stdout'].strip():clean.append(False)
                    elif not row.get('exception'):clean.append(True)
    if False in clean:dirty=True
    elif clean==[True,True]:dirty=False
    passed=analysis['status']=='PASS' and terminal['status']=='PASS';require(not passed or dirty is False,'PASS without actual clean source checks')
    e['source']={'repo':protocol['repository'],'commit':ready['source_commit'],'dirty':dirty};e['producer']={'runtime':'codex_cli','model':None,'session':None,'role':'evidence-adapter'}
    for arm in e['arms']:arm.update(commit=ready['source_commit'],binary_sha256=ready['binary_sha256'])
    schedule=load(ROOT/'schedule.json');binding=load(ROOT/'policy-bindings.json') if (ROOT/'policy-bindings.json').exists() else None;observations=[]
    for cell in sorted((ROOT/'run/cells').glob('*')):
        require(cell.name in ['000000000000','000000000001'],'unexpected retained cell directory')
        i=int(cell.name);attempt=cell/'attempt.json'
        if not attempt.exists():observations.append({'index':i,'attempted':False,'retained_directory':True});continue
        a=load(attempt);cfg=load(cell/'config.json');expected={k:v for k,v in schedule[i].items() if k!='index'};expected['channels']=load(ROOT/'channels.json')
        require(binding and a['schema']=='crypto.ml-dataset.attempt/v1' and type(a['cell_index']) is int and a['cell_index']==i and a['plan_sha256']==binding['plan_binding_sha256'],'attempt plan/index mismatch')
        require(json.dumps(cfg,sort_keys=True)==json.dumps(expected,sort_keys=True),'attempt configuration differs from fixed schedule')
        require(load(cell/'plan.json')==load(ROOT/'plan.json') and sha(cell/'run.json')==binding['expected_run_sha256'] and sha(cell/'protocol.json')==ready['protocol_sha256'],'attempt original bindings differ')
        m=load(cell/'manifest.json') if (cell/'manifest.json').exists() else None;status='infra_error';native=None
        if m:
            require(m['cell_index']==i and m['config']==cfg and m['plan_sha256']==a['plan_sha256'],'manifest identity differs')
            native=m['outcome'];supervisor=native.get('supervisor',{});exit_code=supervisor.get('exit_code')
            status='timeout' if native.get('status')=='timeout' else 'succeeded' if exit_code==0 else 'failed' if exit_code is not None else 'infra_error'
        if passed:require(m and status=='succeeded' and native.get('initial_checked') is True and native.get('solver_completed') is False,'PASS lacks genuine partial initial evidence')
        counted={'pipeline_contract_pass':int(passed)}
        if passed:counted['cell_contract_pass']=1
        e['runs'].append(tool.run_row(run_id=f'cell-{i}',arm='sat-twist' if cfg['twist'] else 'sat-plain',instance_id='0',seed=cfg['seed'],status=status,attempts=1,artifacts=[x for x in evidence if Path(x['path']).is_relative_to(cell)],measurements={'counted':counted},verification={'status':'verified' if passed else 'not_run','verifier':str(ROOT/'analysis.json') if passed else None}))
        observations.append({'index':i,'attempted':True,'native_outcome':native,'execution_status':status})
    if passed:require(len(e['runs'])==2,'PASS has missing attempts')
    e['flow']=tool.compute_flow(e['runs']);e['files']=evidence;e['analysis']['script']=file(ROOT/'driver.py','analysis');e['analysis']['outputs']=[file(ROOT/'analysis.json','analysis'),file(ROOT/'attempt-result.json','receipt')]
    e['notes']=[{'kind':'observation','by':'retained-evidence-adapter','text':json.dumps({'planned':2,'attempted':len(e['runs']),'unattempted':2-len(e['runs']),'cells':observations,'terminal':terminal,'sat_training_admitted':False},sort_keys=True)},{'kind':'observation','by':'retained-evidence-adapter','text':'succeeded denotes actual worker exit only. Initial checked partial1/2 is not completed SAT search, UNSAT, scalar recovery or SAT training projection. No measurements inferred.'}]
    errors=tool.schema_errors(e,'experiment.export/v1');require(not errors,'canonical schema errors '+repr(errors));findings=tool.lint_export(e,protocol,ROOT,True)
    for item in evidence:require(sha(Path(item['path']))==item['sha256'],'evidence moved during export')
    save(output,e);save(ROOT/'experiment.export.validation.json',{'schema_errors':errors,'canonical_findings':findings,'actual_flow':e['flow'],'canonical_complete_pass':passed and len(e['runs'])==2 and not findings,'export_sha256':sha(output),'native_execution':False})
if __name__=='__main__':
    try:main()
    except BaseException as exc:
        refusal=ROOT/'experiment.export.refused.json'
        if ROOT.is_dir() and not refusal.exists():save(refusal,{'error':repr(exc),'native_execution':False,'retained_conflicts':[file(p) for p in sorted((ROOT/'run/cells').glob('*/*.json')) if p.is_file() and not p.is_symlink()]})
        raise
