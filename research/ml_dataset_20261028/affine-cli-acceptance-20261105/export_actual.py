#!/usr/bin/env python3
"""Read-only retained-evidence adapter; no native commands, analyzer run or generation."""
import argparse,hashlib,json,sys,types,os,stat
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path('/var/tmp/affine-cli-custody-acceptance-20261105-v3')
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
def execution_status(native):
    # Resource-censored records are not completed even if a worker exited zero.
    status=native.get('status')
    if status=='timeout':return 'timeout'
    if status=='trace_budget_exhausted':return 'cancelled'
    exit_code=native.get('supervisor',{}).get('exit_code')
    if type(exit_code) is not int:return 'infra_error'
    if status=='worker_failed':return 'failed'
    return 'succeeded' if exit_code==0 else 'failed'
def check_pass_cell(manifest,path,index,analysis):
    require(manifest.get('schema')=='crypto.ml-dataset.cell/v1','wrong cell schema')
    native=manifest['outcome']
    require(execution_status(native)=='succeeded','PASS without successful worker')
    require(native.get('status')=='partial' and native.get('solver_completed') is False and native.get('scalar_recovered_without_witness') is False,'finite identity scope mismatch')
    require(analysis.get('schema')=='local-affine-cli-custody-analysis/v1' and analysis.get('checked_finite_identities')==2 and analysis.get('solver_completed_cells')==0,'wrong affine analysis')
    rows=[r for r in analysis['cells'] if type(r.get('cell_index')) is int and r['cell_index']==index]
    require(len(rows)==1 and rows[0]['manifest_sha256']==sha(path) and rows[0]['actual_outcome']==native and rows[0]['twist']==manifest['config']['twist'],'analysis cell mismatch')

def exact_module(name,path,expected_sha):
    path=Path(path)
    with os.fdopen(os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK),'rb') as f:
        info=os.fstat(f.fileno());require(stat.S_ISREG(info.st_mode) and info.st_size<=4*1024*1024,'source size/type')
        source=f.read(4*1024*1024+1)
    require(len(source)==info.st_size and hashlib.sha256(source).hexdigest()==expected_sha,'executed source bytes differ')
    module=types.ModuleType(name);module.__file__=str(path);sys.modules[name]=module
    exec(compile(source,str(path),'exec'),module.__dict__)
    return module

def bound_driver(root,ready):
    return exact_module('bound_analysis',root/'driver.py',ready['input_sha256']['driver.py'])

def command_pass(root,ready,name):
    commands=load(root/'commands.json');template=commands[name]
    expected=[a.replace('${CRYPTO}',ready['binary_path']).replace('${ROOT}',str(root)).replace('${PROTOCOL_SHA}',ready['protocol_sha256']) for a in template]
    if name=='custody-restore':expected=[a.replace(str(root/'bundle'),str(root/'s3-readback/bundle')) for a in expected]
    d=root/'commands'/name;r=load(d/'receipt.json')
    require(r['argv']==expected==load(d/'argv.json'),'actual command identity differs')
    require(r['exit_code']==0 and not r['timed_out'] and not r['exception'] and not r['interruption'] and r['cleanup']['settled'] is True,'command did not pass')
    require(r['protocol_sha256_before']==r['protocol_sha256_after']==ready['protocol_sha256'],'command protocol differs')
    require(r['binary_sha256_before']==r['binary_sha256_after']==ready['binary_sha256'],'command binary differs')
    for stream in ['stdout','stderr']:require(sha(d/stream)==r[stream+'_sha256'],'command stream differs')

def cell_contract(root,ready,manifest,index):
    # A later pipeline or S3 failure must not erase this earlier checked cell.
    require(manifest is not None,'no sealed native outcome')
    o=manifest['outcome'];cfg=manifest['config']
    require(manifest['schema']=='crypto.ml-dataset.cell/v1' and execution_status(o)=='succeeded' and o['status']=='partial' and o['solver_completed'] is False and o['scalar_recovered_without_witness'] is False,'not a checked partial execution')
    require(cfg['family']=='binary' and cfg['solver']=='f4' and cfg['n']==23 and cfg['m']==3 and cfg['factor_base_size']==4 and cfg['seed']==42 and type(cfg['twist']) is bool,'wrong cell profile')
    require(cfg['channels']=={'schema':'crypto.ml-dataset.channel-requests/v3','affine_boolean_row':{'mode':'affine_unit_row','pattern':1,'row_index':0}},'wrong finite row request')
    driver=bound_driver(root,ready);limits=load(root/'limits.json');binding=load(root/'policy-bindings.json')
    run_sha=sha(root/'expected-run.json');require(run_sha==binding['expected_run_sha256']==sha(root/'run/run.json'),'wrong original run binding')
    for name in ['generate','verify']:
        command_pass(root,ready,name)
        policy=load(root/'policies'/f'{name}.json');receipt=load(root/'replay'/f'{name}.json')
        expected={'schema':'crypto.ml-dataset.affine-replay-policy/v1','mode':'checked_affine_unit_identity_v1','run_sha256':run_sha,'receipt_path':str(root/'replay'/f'{name}.json'),'cell_indices':[0,1],'seconds':limits['replay_seconds_per_command'],'work_per_cell':limits['replay_work_per_cell'],'bytes_per_cell':limits['replay_bytes_per_cell'],'stream_bytes_per_cell':limits['replay_stream_bytes_per_cell'],'payload_bytes_per_cell':limits['replay_payload_bytes_per_cell'],'aggregate_work':limits['replay_aggregate_work_per_command'],'aggregate_bytes':limits['replay_aggregate_bytes_per_command']}
        require(policy==expected,'cell replay allowance differs')
        entries=[e for e in binding['policy_files'] if e['path']==name+'.json']
        require(len(entries)==1 and entries[0]['sha256']==sha(root/'policies'/f'{name}.json'),'cell policy binding differs')
        driver.check_replay_receipt(name,receipt,policy,run_sha)
        require(receipt['slots'][index]['cell_index']==index,'cell replay slot differs')
    require(load(root/'attempt-result.json').get('post_verify_inventory_sha256')==sha(root/'post-verify-inventory.json'),'original terminal snapshot binding differs')
    snapshot=load(root/'post-verify-inventory.json')
    require(snapshot['schema']=='local-affine-post-verify-inventory/v1' and snapshot['run_sha256']==run_sha,'post-verify inventory run binding differs')
    require(snapshot['verify_command_receipt_sha256']==sha(root/'commands/verify/receipt.json') and snapshot['verify_replay_receipt_sha256']==sha(root/'replay/verify.json'),'post-verify receipt binding differs')
    require([row['cell_index'] for row in snapshot['cells']]==[0,1],'post-verify cell inventory differs')
    cell=root/'run/cells'/f'{index:012d}';row=snapshot['cells'][index]
    require(row['manifest_sha256']==sha(cell/'manifest.json'),'sealed manifest changed after native verify')
    require(row['files']==driver.file_inventory(cell),'sealed payload inventory changed after native verify')
    # Check this cell only: corruption of another cell does not erase its valid sibling.
    paths=set()
    for artifact in manifest['artifacts']:
        rel=Path(artifact['path'])
        require(not rel.is_absolute() and rel.parts and '..' not in rel.parts and '.' not in rel.parts and str(rel)==artifact['path'],'unsafe sealed artifact path')
        require(artifact['path'] not in paths,'duplicate sealed artifact');paths.add(artifact['path'])
        for segment in [cell/Path(*rel.parts[:n]) for n in range(1,len(rel.parts)+1)]:require(not segment.is_symlink(),'sealed artifact symlink')
        path=cell/rel;require(path.is_file() and type(artifact['bytes']) is int and artifact['bytes']>=0 and path.stat().st_size==artifact['bytes'] and sha(path)==artifact['sha256'],'sealed artifact bytes differ')
    require(paths,'empty sealed artifact registry')
    return True

def check_pipeline(root,ready,analysis,protocol):
    # Import only exact protocol-bound local analysis; never invoke run_acceptance.
    driver=bound_driver(root,ready)
    require(driver.analyze(root,ready['input_sha256']['public_projection.py'])==analysis,'retained analysis differs from recomputation')
    for label in ['source-before','source-after']:
        records=load(root/(label+'.json'))
        require(len(records)==3,'missing source checks')
        for record,tail,value in zip(records,[['rev-parse','HEAD'],['rev-parse',ready['source_commit']+'^{tree}'],['status','--porcelain=v1','--untracked-files=all']],[ready['source_commit'],ready['source_tree'],'']):
            require(record['argv']==['git','-C',ready['source_path'],*tail] and record['exit_code']==0 and record['stdout'].strip()==value and not record.get('exception') and not record.get('timed_out'),'actual source identity changed')
    commands=load(root/'commands.json')
    for name,template in commands.items():
        expected=[a.replace('${CRYPTO}',ready['binary_path']).replace('${ROOT}',str(root)).replace('${PROTOCOL_SHA}',ready['protocol_sha256']) for a in template]
        if name=='custody-restore':expected=[a.replace(str(root/'bundle'),str(root/'s3-readback/bundle')) for a in expected]
        receipt=load(root/'commands'/name/'receipt.json')
        require(receipt['argv']==expected,'actual argv differs: '+name)
    for name in list(commands)+['s3-custody','s3-readback-custody-verify']:
        directory=root/'commands'/name;r=load(directory/'receipt.json')
        require(r['argv']==load(directory/'argv.json'),'command argv receipt mismatch')
        require(r['exit_code']==0 and not r['timed_out'] and not r['exception'] and not r['interruption'] and r['cleanup']['settled'] is True,'command incomplete: '+name)
        require(r['protocol_sha256_before']==r['protocol_sha256_after']==ready['protocol_sha256'],'command protocol mismatch')
        require(r['binary_sha256_before']==r['binary_sha256_after']==ready['binary_sha256'],'command binary mismatch')
        for stream in ['stdout','stderr']:require(sha(directory/stream)==r[stream+'_sha256'],'command output changed')
    remote=load(root/'commands/s3-custody/receipt.json')['argv']
    require(len(remote)==6 and remote[1:]==[str(root/'s3_custody.py'),'--root',str(root),'--authorization',ready['custody_authorization']],'wrong S3 helper argv')
    require(load(root/'commands/s3-readback-custody-verify/receipt.json')['argv']==[ready['binary_path'],'dataset','custody','verify','--bundle',str(root/'s3-readback/bundle')],'wrong downloaded bundle verifier argv')
    custody=load(root/'s3-readback/PASS.json')
    require(custody==load(root/'s3-readback/result.json') and custody['status']=='PASS','actual S3 readback missing')
    require(custody['files']==load(root/'s3-readback/inventory.json') and len(custody['files'])>0,'empty or inconsistent readback inventory')
    for item in custody['files']:
        rel=Path(item['path']);require(not rel.is_absolute() and '..' not in rel.parts,'unsafe custody path')
        for base in [root/'bundle',root/'s3-readback/bundle']:
            p=base/rel;require(p.stat().st_size==item['bytes'] and sha(p)==item['sha256'],'actual S3 restored bytes differ')
    authorization=load(Path(ready['custody_authorization']))
    logs=load(root/'s3-readback/receipts.json');require(len(logs)==2*len(custody['files']),'missing actual remote receipts')
    for i,item in enumerate(custody['files']):
        for j,operation in enumerate(['put','get']):
            record=logs[2*i+j];require(record['operation']==operation and record['key']==authorization['prefix']+item['path'] and record['exit_code']==0 and record['bytes']==item['bytes'] and record['sha256']==item['sha256'],'remote receipt binding mismatch')
            for stream in ['stdout','stderr']:require(sha(root/'s3-readback'/f'{2*i+j+1:05d}.{stream}')==record[stream+'_sha256'],'remote output digest differs')
    require(analysis['remote_custody_status']=='PASS','analysis lacks S3 gate')

def main(root,output_dir):
    global ROOT
    ROOT=Path(root).resolve();output_dir=Path(output_dir).resolve()
    require(not output_dir.is_relative_to(ROOT) and not ROOT.is_relative_to(output_dir),'output must be separate from sealed evidence root')
    output_dir.mkdir(parents=True,exist_ok=False)
    output=output_dir/'experiment.export.json' 
    require(sha(Path(__file__).with_name('tooling-pins.json'))=='abfd287091aca67240f537e02ec07bf2aa3f08ef018090b03ff0e1155cc2ced8','unbound canonical tooling pins')
    pins=load(Path(__file__).with_name('tooling-pins.json'))
    for p,h in pins['files'].items():require(sha(Path(p))==h,'canonical tooling changed')
    for p,expected in pins['directories'].items():require({str(f.relative_to(p)):sha(f) for f in Path(p).rglob('*') if f.is_file() and f.suffix in ('.py','.so','.json')}==expected,'pinned dependency changed')
    # Disable dependency cache reads as well as writes; canonical module itself is compiled from exact bytes.
    sys.pycache_prefix=str(output_dir/'unused-empty-pycache');require(not Path(sys.pycache_prefix).exists(),'cache directory must not exist')
    tool=exact_module('canonical_tool',TOOL,pins['files'][str(TOOL)])
    ready=load(ROOT/'ready.json');start=load(ROOT/'attempt-start.json');terminal=load(ROOT/'attempt-result.json');analysis=load(ROOT/'analysis.json') if (ROOT/'analysis.json').exists() else {'status':'NOT_EXECUTED'};protocol=load(ROOT/'protocol.json')
    require(start['ready_sha256']==sha(ROOT/'ready.json') and start['protocol_sha256']==sha(ROOT/'protocol.json')==ready['protocol_sha256'],'original readiness/protocol mismatch')
    require(protocol['freeze']['status']=='frozen' and terminal.get('finished_at') and terminal['cleanup']['settled'] is True,'nonterminal or unfrozen attempt')
    require(terminal['status'] in ['PASS','FAIL','PARTIAL','INTERRUPTED','NOT_EXECUTED'] and analysis['status'] in ['PASS','FAIL','PARTIAL','INTERRUPTED','NOT_EXECUTED'],'unknown terminal status')
    require(protocol['extensions']['canonical_export_adapter']['sha256']==sha(Path(__file__)),'unbound export adapter')
    require(ready['input_sha256']==protocol['extensions']['fixed_input_bindings'],'readiness input inventory differs from frozen protocol')
    require(set(ready['input_sha256'])=={'driver.py','commands.json','channels.json','factor-base-policies.json','schedule.json','limits.json','s3_custody.py','public_projection.py','required-regressions.json'},'incomplete input inventory')
    for n,h in ready['input_sha256'].items():require(sha(ROOT/n)==h,'frozen input changed')
    require(protocol['analysis_plan']['script']['sha256']==sha(ROOT/'driver.py'),'analysis instrument differs')
    paths=sorted(ROOT.rglob('*'));require(not any(p.is_symlink() for p in paths),'symlink evidence')
    evidence=[file(p) for p in paths if p.is_file()];e=tool.export_skeleton(protocol,str(ROOT/'protocol.json'),'local','affine-cli-custody-acceptance-20261105-v1')
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
    if passed:check_pipeline(ROOT,ready,analysis,protocol)
    e['source']={'repo':protocol['repository'],'commit':ready['source_commit'],'dirty':dirty};e['producer']={'runtime':'codex_cli','model':None,'session':None,'role':'evidence-adapter'}
    for arm in e['arms']:arm.update(commit=ready['source_commit'],binary_sha256=ready['binary_sha256'])
    schedule=load(ROOT/'schedule.json');require([r['index'] for r in schedule]==[0,1] and [r['twist'] for r in schedule]==[False,True] and all(r['seed']==42 and r['n']==23 and r['m']==3 for r in schedule),'wrong planned cohort');binding=load(ROOT/'policy-bindings.json') if (ROOT/'policy-bindings.json').exists() else None;observations=[]
    require(all(p.name in ['000000000000','000000000001'] for p in (ROOT/'run/cells').glob('*')),'unexpected cell directory')
    for i in range(2):
        cell=ROOT/'run/cells'/f'{i:012d}'
        require(cell.name in ['000000000000','000000000001'],'unexpected retained cell directory')
        i=int(cell.name);attempt=cell/'attempt.json'
        if not attempt.exists():observations.append({'index':i,'attempted':False,'retained_directory':cell.exists(),'seed':schedule[i]['seed'],'twist':schedule[i]['twist']});continue
        a=load(attempt);cfg=load(cell/'config.json');expected={k:v for k,v in schedule[i].items() if k!='index'};expected['channels']=load(ROOT/'channels.json')
        require(binding and a['schema']=='crypto.ml-dataset.attempt/v1' and type(a['cell_index']) is int and a['cell_index']==i and a['plan_sha256']==binding['plan_binding_sha256'],'attempt plan/index mismatch')
        require(json.dumps(cfg,sort_keys=True)==json.dumps(expected,sort_keys=True),'attempt configuration differs from fixed schedule')
        require(load(cell/'plan.json')==load(ROOT/'plan.json') and sha(cell/'run.json')==binding['expected_run_sha256'] and sha(cell/'protocol.json')==ready['protocol_sha256'],'attempt original bindings differ')
        m=load(cell/'manifest.json') if (cell/'manifest.json').exists() else None;status='interrupted' if terminal['status']=='INTERRUPTED' else 'infra_error';native=None
        if m:
            require(m['cell_index']==i and m['config']==cfg and m['plan_sha256']==a['plan_sha256'],'manifest identity differs')
            native=m['outcome'];status=execution_status(native)
        if passed:
            require(m is not None,'PASS missing sealed manifest')
            check_pass_cell(m,cell/'manifest.json',i,analysis)
        cell_pass=False;cell_error=None
        try:cell_pass=cell_contract(ROOT,ready,m,i)
        except (RuntimeError,KeyError,ValueError,OSError,TypeError) as exc:cell_error=str(exc)
        require(not passed or cell_pass,'pipeline PASS without independent cell gate')
        counted={'pipeline_contract_pass':int(passed),'cell_contract_pass':int(cell_pass)}
        e['runs'].append(tool.run_row(run_id=f'cell-{i}',arm='affine-twist' if cfg['twist'] else 'affine-plain',instance_id='0',seed=cfg['seed'],status=status,attempts=1,artifacts=[x for x in evidence if Path(x['path']).is_relative_to(cell)],measurements={'counted':counted},verification={'status':'verified' if cell_pass else 'not_run','verifier':str(ROOT/'replay/verify.json') if cell_pass else None}))
        observations.append({'index':i,'attempted':True,'native_outcome':native,'execution_status':status,'cell_contract_pass':cell_pass,'cell_gate_error':cell_error})
    if passed:require(len(e['runs'])==2,'PASS has missing attempts')
    e['flow']=tool.compute_flow(e['runs']);e['files']=evidence;e['analysis']['script']=file(ROOT/'driver.py','analysis');e['analysis']['outputs']=[file(p,'analysis') for p in [ROOT/'analysis.json',ROOT/'attempt-result.json'] if p.exists()]
    e['notes']=[{'kind':'observation','by':'retained-evidence-adapter','text':json.dumps({'planned':2,'attempted':len(e['runs']),'unattempted':2-len(e['runs']),'cells':observations,'terminal':terminal,'finite_identity_scope_only':True,'synthetic_fixture':protocol.get('extensions',{}).get('synthetic_fixture',False)},sort_keys=True)},{'kind':'observation','by':'retained-evidence-adapter','text':'succeeded denotes an uncensored actual worker exit only; timeout and trace-budget censorship remain not_completed. Native partial outcomes remain explicit. Completed canonical execution is not completed algebraic search. Metrics are exact acceptance-gate booleans only; no degree, scalar recovery, full trajectory or performance inferred.'}]
    errors=tool.schema_errors(e,'experiment.export/v1');require(not errors,'canonical schema errors '+repr(errors));findings=tool.lint_export(e,protocol,ROOT,True)
    for item in evidence:require(sha(Path(item['path']))==item['sha256'],'evidence moved during export')
    save(output,e);save(output_dir/'experiment.export.validation.json',{'schema_errors':errors,'canonical_findings':findings,'actual_flow':e['flow'],'canonical_complete_pass':passed and len(e['runs'])==2 and not findings,'export_sha256':sha(output),'native_execution':False})
if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--root',required=True);parser.add_argument('--output-dir',required=True);args=parser.parse_args()
    main(args.root,args.output_dir)
