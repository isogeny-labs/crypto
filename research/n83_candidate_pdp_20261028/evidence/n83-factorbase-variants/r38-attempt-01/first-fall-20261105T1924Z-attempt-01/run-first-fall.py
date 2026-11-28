#!/usr/bin/env python3
"""Run the frozen V2 first-fall cases for the existing spike38 inputs."""
import argparse, datetime, hashlib, json, os, pathlib, platform, re, shutil, signal, subprocess, sys, tempfile, time

PROTOCOL_ID = "N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2"
PROTOCOL_FREEZE_SHA256 = "50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9"
PROTOCOL_FILE_SHA256 = "8a9143e9588ce1ec5c204362b734f790696de90569ce1de3c086e4645e605885"
SOURCE_COMMIT = "52665c7a6e8308f86201e10c9305afa346fd134b"
FALL_SHA256 = "33c859d58f30f6facf483b0df61c9ffe7339298513c15001b6583605c11c1a14"
INPUT_MANIFEST_SHA256 = "93b351b181b135c74cff0da0386c704d46eb777b302ed3b440ad18a249d52ca1"
S3_ROOT = "s3://isolab-results-104186811093-us-east-2/runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike38/attempt-01/data-v1"
STOP = False

def sha_bytes(b): return hashlib.sha256(b).hexdigest()
def sha_file(p):
    h=hashlib.sha256()
    with open(p,'rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''): h.update(b)
    return h.hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write_json(p,x):
    p=pathlib.Path(p); p.parent.mkdir(parents=True,exist_ok=True); p.write_text(json.dumps(x,indent=2,sort_keys=True)+'\n')
def aws(args, *, check=True, capture=True):
    p=subprocess.run(['aws',*args,'--region','us-east-2','--only-show-errors'],text=True,capture_output=capture)
    if check and p.returncode: raise RuntimeError(f"aws command failed ({p.returncode}): {' '.join(args)}: {p.stderr[-1000:] if capture else ''}")
    return p
def put(path,key): aws(['s3','cp',str(path),f"{S3_ROOT}/{key}"])
def check_empty_prefix():
    p=subprocess.run(['aws','s3api','list-objects-v2','--bucket','isolab-results-104186811093-us-east-2','--prefix','runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike38/attempt-01/data-v1/','--region','us-east-2','--output','json'],text=True,capture_output=True,check=True)
    x=json.loads(p.stdout)
    if x.get('Contents'): raise RuntimeError(f"results prefix is not empty ({len(x['Contents'])} listed); preserve it")
    return {'prefix':S3_ROOT+'/','key_count':x.get('KeyCount',0),'contents':0}
def process_snapshot():
    p=subprocess.run(['ps','-eo','pid=,args='],text=True,capture_output=True,check=True)
    hits=[]
    for line in p.stdout.splitlines():
        if re.search(r'(^|/)(fall)\s+(run|replay)\b',line): hits.append(line.strip())
    return hits
def run_cmd(args, seconds, env):
    start=time.monotonic()
    p=subprocess.run(['timeout','--signal=TERM','--kill-after=10',f'{seconds}s',*map(str,args)],text=True,capture_output=True,env=env)
    return p,round(time.monotonic()-start,6)
def preflight(a):
    out=pathlib.Path(a.out).resolve(); inputs=pathlib.Path(a.inputs).resolve(); manifest_path=pathlib.Path(a.manifest).resolve(); binary=pathlib.Path(a.fall).resolve()
    protocol_path=pathlib.Path(a.protocol).resolve();
    manifest=json.loads(manifest_path.read_text()); protocol=json.loads(protocol_path.read_text()); cases=json.loads((inputs/'inputs.json').read_text())
    assert sha_file(manifest_path)==INPUT_MANIFEST_SHA256
    assert sha_file(protocol_path)==PROTOCOL_FILE_SHA256
    assert protocol['protocol_id']==PROTOCOL_ID and protocol['freeze']['protocol_sha256']==PROTOCOL_FREEZE_SHA256 and protocol['freeze']['status']=='frozen'
    assert sha_file(binary)==FALL_SHA256
    assert manifest['protocol_id']==PROTOCOL_ID and manifest['protocol_sha256']==PROTOCOL_FREEZE_SHA256 and manifest['case_count']==48
    assert len(cases)==48 and len(manifest['cases'])==48
    expected={x['id']:x for x in manifest['cases']}
    for item in cases:
        cid=item['id']; row=expected[cid]; sysfile=inputs/cid/'system.sing'; inputfile=inputs/cid/'input.json'
        assert item['candidate_id']=='spike38' and item['arm']=='spike38'
        assert sysfile.is_file() and inputfile.is_file()
        assert sysfile.stat().st_size==row['system_bytes'] and sha_file(sysfile)==row['system_sha256']==item['source_sha256']
        assert inputfile.stat().st_size==row['input_bytes'] and sha_file(inputfile)==row['input_sha256']
        assert item['family']==row['family'] and item['m']==row['m'] and item['target_kind']==row['target_kind']
    if out.exists() and any(out.iterdir()): raise RuntimeError('attempt output directory is nonempty; preserve and use a new attempt')
    out.mkdir(parents=True,exist_ok=True)
    empty=check_empty_prefix()
    active=process_snapshot()
    if active: raise RuntimeError('matching fall process already active: '+repr(active))
    cpus=os.sched_getaffinity(0)
    if int(a.cpu) not in cpus: raise RuntimeError(f"requested CPU {a.cpu} outside allowed set {sorted(cpus)}")
    mem={}
    try:
        for line in pathlib.Path('/proc/meminfo').read_text().splitlines():
            if line.startswith(('MemAvailable:','MemTotal:')): mem[line.split(':')[0]]=int(line.split()[1])*1024
    except Exception: pass
    selfcheck,elapsed=run_cmd([binary,'selfcheck'],60,os.environ.copy())
    if selfcheck.returncode: raise RuntimeError(f"fall selfcheck failed: {selfcheck.stderr[-1200:]}")
    record={'schema':'n83.first-fall-preflight/v1','candidate_id':'spike38','attempt':'01','protocol_id':PROTOCOL_ID,'protocol_file_sha256':PROTOCOL_FILE_SHA256,'protocol_frozen_sha256':PROTOCOL_FREEZE_SHA256,'protocol_source_commit':SOURCE_COMMIT,'fall_binary_sha256':FALL_SHA256,'input_manifest_sha256':INPUT_MANIFEST_SHA256,'input_root':str(inputs),'case_count':len(cases),'all_source_hashes_match':True,'fresh_s3_prefix':empty,'matching_fall_processes':active,'fall_selfcheck_exit_code':selfcheck.returncode,'fall_selfcheck_elapsed_seconds':elapsed,'fall_selfcheck_stdout':selfcheck.stdout,'fall_selfcheck_stderr':selfcheck.stderr,'host':{'platform':platform.platform(),'python':sys.version,'cpu_count':os.cpu_count(),'allowed_cpus':sorted(cpus),'selected_cpu':int(a.cpu),'memory_bytes':mem},'limits':{'threads':1,'memory_gib':8,'per_case_seconds':60,'per_replay_seconds':60,'outer_wall_seconds':12850,'termination_grace_seconds':45,'max_attempts':1,'retries':0,'wall_valid':False,'compute_ceiling_usd':2.90,'artifact_ceiling_usd':0.25,'total_ceiling_usd':3.15},'cost_observation_usd':{'current_day_to_date':77.0045996521,'month_to_date':582.4529986551,'daily_cap':200,'monthly_cap':1000,'spot_usd_per_hour':0.8015},'prepared_utc':utc(),'status':'preflight_passed'}
    write_json(out/'PRE-RUN.json',record)
    return record

def signal_stop(signum,frame):
    global STOP; STOP=True

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--inputs',required=True); ap.add_argument('--manifest',required=True); ap.add_argument('--protocol',required=True); ap.add_argument('--fall',required=True); ap.add_argument('--out',required=True); ap.add_argument('--cpu',type=int,default=0); ap.add_argument('--run',action='store_true'); a=ap.parse_args()
    out=pathlib.Path(a.out).resolve(); inputs=pathlib.Path(a.inputs).resolve(); manifest=json.loads(pathlib.Path(a.manifest).read_text());
    pf=preflight(a); put(out/'PRE-RUN.json','metadata/PRE-RUN.json')
    print(json.dumps({'preflight':'passed','cases':pf['case_count'],'fresh_s3_prefix':S3_ROOT+'/','run_requested':a.run}),flush=True)
    if not a.run: return 0
    signal.signal(signal.SIGTERM,signal_stop); signal.signal(signal.SIGINT,signal_stop)
    env=os.environ.copy(); env.update({'ISOLAB_RESULTS_URI':S3_ROOT,'AWS_RETRY_MODE':'standard','AWS_MAX_ATTEMPTS':'1'})
    rows=[]; started=utc();
    for ordinal,item in enumerate(json.loads((inputs/'inputs.json').read_text()),1):
        if STOP: break
        cid=item['id']; begin=utc(); row={'ordinal':ordinal,'id':cid,'family':item['family'],'m':item['m'],'target_kind':item['target_kind'],'input_sha256':item['source_sha256'],'case_status':'started','started_at':begin,'wall_valid':False,'attempts':1,'retries':0}
        started_path=out/f'attempts/{cid}-started.json'; write_json(started_path,row); put(started_path,f'attempts/{cid}-started.json')
        with tempfile.TemporaryDirectory(prefix=f'n83-ffd-spike38-{cid}-') as td:
            case_root=pathlib.Path(td); (case_root/cid).mkdir(); (case_root/'inputs.json').write_text(json.dumps([item],indent=2)+'\n'); os.symlink(inputs/cid/'system.sing',case_root/cid/'system.sing')
            case_out=out/'certificates'/cid
            case_out.parent.mkdir(parents=True,exist_ok=True)
            cmd,elapsed=run_cmd([a.fall,'run',case_root,case_out],60,env)
            row.update({'run_exit_code':cmd.returncode,'run_elapsed_wall_seconds':elapsed,'run_stdout':cmd.stdout,'run_stderr':cmd.stderr})
            cert=case_out/f'{cid}.json';
            if cmd.returncode==0 and cert.is_file():
                row['certificate_sha256']=sha_file(cert); row['certificate_bytes']=cert.stat().st_size
                replay,replay_elapsed=run_cmd([a.fall,'replay',case_root,cert],60,env)
                row.update({'replay_exit_code':replay.returncode,'replay_elapsed_wall_seconds':replay_elapsed,'replay_stdout':replay.stdout,'replay_stderr':replay.stderr,'replay_passed':replay.returncode==0})
                if replay.returncode==0:
                    cert_uri=f'{S3_ROOT}/certificates/{cid}.json'; tmp=pathlib.Path(td)/'s3-readback.json'; aws(['s3','cp',cert_uri,str(tmp)]); row.update({'certificate_s3_uri':cert_uri,'s3_readback_sha256':sha_file(tmp),'s3_readback_bytes':tmp.stat().st_size,'s3_readback_matches':sha_file(tmp)==row['certificate_sha256'] and tmp.stat().st_size==row['certificate_bytes']})
                    result=json.loads(cert.read_text()); row.update({'status':result.get('status'),'first_fall_degree':result.get('first_fall_degree'),'first_fall_lower_bound':result.get('first_fall_lower_bound')})
                else: row['case_status']='failed_replay'
                if row.get('replay_passed') and row.get('s3_readback_matches'): row['case_status']='completed'
            else:
                row['case_status']='timeout' if cmd.returncode==124 else 'failed_run'
                if cert.exists(): row['partial_certificate_sha256']=sha_file(cert)
        row['finished_at']=utc(); terminal=out/f'attempts/{cid}-terminal.json'; write_json(terminal,row); put(terminal,f'attempts/{cid}-terminal.json'); rows.append(row)
        partial={'schema':'n83.first-fall-partial-summary/v1','candidate_id':'spike38','attempt':'01','completed_records':len(rows),'planned_records':48,'interrupted':STOP,'rows':rows,'updated_utc':utc(),'wall_valid':False}
        write_json(out/'partial-summary.json',partial); put(out/'partial-summary.json',f'checkpoints/after-{ordinal:02d}.json')
        print(json.dumps({'case':cid,'status':row['case_status'],'first_fall_degree':row.get('first_fall_degree'),'first_fall_lower_bound':row.get('first_fall_lower_bound'),'replay_passed':row.get('replay_passed'),'s3_readback_matches':row.get('s3_readback_matches')},separators=(',',':')),flush=True)
    counts={}
    for r in rows: counts[r['case_status']]=counts.get(r['case_status'],0)+1
    summary={'schema':'n83.first-fall-panel-summary/v1','candidate_id':'spike38','attempt':'01','protocol_id':PROTOCOL_ID,'protocol_sha256':PROTOCOL_FREEZE_SHA256,'input_manifest_sha256':INPUT_MANIFEST_SHA256,'planned_cases':48,'attempted_cases':len(rows),'completed_cases':sum(r['case_status']=='completed' for r in rows),'outcome_counts':counts,'first_fall_exact_counts':{},'first_fall_lower_bound_counts':{},'last_fall_degree':None,'last_fall_lower_bound':None,'closure_rank':None,'closure_status':'not_run_by_pinned_fall_binary','rows':rows,'started_at':started,'finished_at':utc(),'interrupted':STOP,'wall_valid':False,'scope':'First-fall certificates only; no LFD closure or degree-of-regularity claim.'}
    for r in rows:
        if r.get('first_fall_degree') is not None:
            k=str(r['first_fall_degree']); summary['first_fall_exact_counts'][k]=summary['first_fall_exact_counts'].get(k,0)+1
        elif r.get('first_fall_lower_bound') is not None:
            k=str(r['first_fall_lower_bound']); summary['first_fall_lower_bound_counts'][k]=summary['first_fall_lower_bound_counts'].get(k,0)+1
    write_json(out/'summary.json',summary)
    write_json(out/'terminal.json',{'schema':'n83.first-fall-terminal/v1','candidate_id':'spike38','attempt':'01','exit_code':0 if len(rows)==48 and summary['completed_cases']==48 else 2,'attempted_cases':len(rows),'completed_cases':summary['completed_cases'],'finished_at':summary['finished_at'],'wall_valid':False})
    file_rows=[]
    for p in sorted(out.rglob('*')):
        if not p.is_file() or p.name in ('manifest.sha256','S3_PERSISTENCE_INDEX.json'): continue
        rel=p.relative_to(out).as_posix(); b=p.read_bytes(); file_rows.append({'path':rel,'bytes':len(b),'sha256':sha_bytes(b),'s3_uri':f'{S3_ROOT}/results/{rel}'})
    write_json(out/'FILE-LAYOUT.json',{'schema':'n83.first-fall-file-layout/v1','candidate_id':'spike38','attempt':'01','root':'results/','files':file_rows,'case_structure':{'source':'attempt-09 S3 candidate-inputs/inputs.json plus per-case input.json and system.sing','runner':'one-case temporary inputs.json referencing source system by symlink; original system bytes remain at attempt-09 S3 URI','outputs':'attempts/<id>-started.json, attempts/<id>-terminal.json, certificates/<id>/<id>.json, certificates/<id>/<id>.json replayed in a separate process','partial_checkpoints':'checkpoints/after-NN.json snapshots of cumulative case rows'},'semantics':'No duplicate factor-base point shards; original indexed source inputs remain in attempt-09 results.'})
    files=[]
    for p in sorted(out.rglob('*')):
        if not p.is_file() or p.name in ('manifest.sha256','S3_PERSISTENCE_INDEX.json'): continue
        rel=p.relative_to(out).as_posix(); b=p.read_bytes(); files.append({'path':rel,'bytes':len(b),'sha256':sha_bytes(b),'s3_uri':f'{S3_ROOT}/results/{rel}'})
    write_json(out/'manifest.sha256',{'schema':'n83.file-manifest/v1','files':[{'path':x['path'],'bytes':x['bytes'],'sha256':x['sha256']} for x in files]})
    files.append({'path':'manifest.sha256','bytes':(out/'manifest.sha256').stat().st_size,'sha256':sha_file(out/'manifest.sha256'),'s3_uri':f'{S3_ROOT}/results/manifest.sha256'})
    write_json(out/'S3_PERSISTENCE_INDEX.json',{'schema':'n83.first-fall-output-index/v1','candidate_id':'spike38','attempt':'01','files':files})
    sync=subprocess.run(['aws','s3','sync',str(out),f'{S3_ROOT}/results/','--region','us-east-2','--only-show-errors'],text=True,capture_output=True)
    if sync.returncode: raise RuntimeError('final S3 sync failed: '+sync.stderr[-1000:])
    print(json.dumps({'summary':summary,'indexed_files':len(files),'result_index_sha256':sha_file(out/'S3_PERSISTENCE_INDEX.json')},separators=(',',':')),flush=True)
    return 0 if len(rows)==48 and summary['completed_cases']==48 else 2
if __name__=='__main__':
    try: sys.exit(main())
    except Exception as e:
        print(json.dumps({'fatal':type(e).__name__,'message':str(e),'utc':utc()},separators=(',',':')),file=sys.stderr,flush=True); sys.exit(1)
