#!/usr/bin/env python3
"""Prospective Q0-Q4 software supervisor. NOT ADMITTED. Never run while bindings are null."""
import argparse, hashlib, io, json, os, pathlib, re, shutil, signal, stat, subprocess, tarfile, time, uuid, tempfile
P=pathlib.Path
H=lambda b:hashlib.sha256(b).hexdigest()
FIX=P('/var/tmp/pencil-runtime-fixtures-20261031-v1')
MEMBERS=P('/var/tmp/pencil-local-image-assembly-proposal-20261031-v1/runtime-members.json')
OWNER='f6f6016e549ab19d9e92a7a89a5a89d095de068775dd003ab5666217726b48b7'
class Stop(Exception):pass

def require(x,msg):
 if not x:raise Stop(msg)
def strict(raw):
 def unique(pairs):
  d={}
  for k,v in pairs:
   require(k not in d,'duplicate JSON key');d[k]=v
  return d
 return json.loads(raw,object_pairs_hook=unique)
def safe_inspect(x):
 if isinstance(x,list):return [safe_inspect(v) for v in x]
 if isinstance(x,dict):
  return {k:([{'name':v.split('=',1)[0],'sha256':H(v.encode())} for v in a] if k=='Env' and isinstance(a,list) else safe_inspect(a)) for k,a in x.items()}
 return x

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--image-id',required=True);ap.add_argument('--binary-sha256',required=True);ap.add_argument('--binding',required=True);ap.add_argument('--receipts',required=True);args=ap.parse_args()
 require(re.fullmatch('sha256:[0-9a-f]{64}',args.image_id),'image ID shape')
 require(re.fullmatch('[0-9a-f]{64}',args.binary_sha256),'binary hash shape')
 out=P(args.receipts);require(out.is_absolute() and not out.exists(),'fresh absolute receipts required');out.mkdir(mode=0o700)
 start=time.monotonic();deadline=start+300;work_deadline=deadline-30;seq=0;owned=None;pending=None;failure=None;clean=False;passed=[];pids_observations=[];active_deadline=None;unreaped_client=False
 config=out/'empty-docker-config';config.mkdir(mode=0o700)
 binding_raw=P(args.binding).read_bytes();binding=strict(binding_raw)
 # All external joins must be filled and independently reviewed before admission.
 require(binding.get('admitted') is True,'proposal not admitted')
 require(binding.get('image_id')==args.image_id and binding.get('binary_sha256')==args.binary_sha256,'argument binding mismatch')
 fixture_manifest_raw=(FIX/'manifest.json').read_bytes()
 require(binding.get('fixture_manifest_sha256')==H(fixture_manifest_raw),'fixture manifest binding')
 fixture_manifest=strict(fixture_manifest_raw)
 def verify_fixture_members():
  require(isinstance(fixture_manifest,dict),'fixture manifest format')
  for rel,sha in fixture_manifest.items():
   path=P(rel);require(not path.is_absolute() and '..' not in path.parts,'unsafe fixture manifest path')
   f=FIX/path
   for component in [f,*f.parents]:require(not component.is_symlink(),'fixture symlink component')
   st=f.stat();require(stat.S_ISREG(st.st_mode) and st.st_nlink==1,'fixture manifest member type/alias')
   require(H(f.read_bytes())==sha,'fixture manifest member digest '+rel)
  require({str(f.relative_to(FIX)) for f in FIX.rglob('*') if f.is_file()}==set(fixture_manifest)|{'manifest.json'},'unexpected fixture member')
 verify_fixture_members()
 require(binding.get('runtime_members_sha256')==H(MEMBERS.read_bytes()),'runtime member binding')
 require(binding.get('reviewed_supervisor_sha256')==H(P(__file__).read_bytes()),'supervisor review binding')
 docker=P(binding['docker_client']);require(docker.is_absolute() and docker.is_file(),'Docker client path');require(H(docker.read_bytes())==binding['docker_client_sha256'],'Docker client digest')
 expected_build=binding['build_info'];require(expected_build['ecbench_sha256']==args.binary_sha256 and expected_build['fresh_owner_sha256']==OWNER,'build-info binary joins')
 require(re.fullmatch('[0-9a-f]{40}',expected_build['source_revision']),'final source revision')
 require(expected_build['architecture']=='linux/amd64' and expected_build['target']=='x86_64-unknown-linux-musl' and expected_build['default_features'] is False,'build-info target contract')
 require(expected_build['source_repository']=='https://github.com/aburan28/crypto','source repository metadata contract')
 for key in ['final_build_receipt_sha256','final_source_manifest_sha256','prospective_review_sha256']:
  require(re.fullmatch('[0-9a-f]{64}',binding.get(key,'')),'unfilled '+key)
 prefix=['/usr/bin/sudo','-n','/usr/bin/env','-i','PATH=/usr/bin:/bin','HOME='+str(config),'DOCKER_CONFIG='+str(config),str(docker),'--host','unix:///var/run/docker.sock']
 env={'PATH':'/usr/bin:/bin','HOME':str(config),'LANG':'C'}
 def command(label,argv,cap=20,cleanup=False,limit=2*1024**2,inspect=False):
  nonlocal seq,unreaped_client
  stop=deadline if cleanup else work_deadline
  if active_deadline is not None and not cleanup:stop=min(stop,active_deadline)
  remaining=stop-time.monotonic();require(remaining>2,'deadline reserve exhausted')
  timeout=min(cap,remaining-2);seq+=1;d=out/f'{seq:03d}-{label}';d.mkdir();t=time.monotonic();proc=None;result={};so=None;se=None
  try:
   # Inspection bytes never have a named persistent raw file. Failure leaves only hash/size.
   so=tempfile.TemporaryFile(mode='w+b') if inspect else (d/'stdout').open('w+b')
   se=(d/'stderr').open('wb')
   proc=subprocess.Popen(argv,stdout=so,stderr=se,env=env,start_new_session=True)
   unreaped_client=True
   while proc.poll() is None:
    if time.monotonic()-t>=timeout or os.fstat(so.fileno()).st_size>limit or os.fstat(se.fileno()).st_size>2*1024**2:
     raise Stop('client timeout/output cap; container ownership must reconcile')
    time.sleep(.03)
   result['exit_code']=proc.returncode
   so.seek(0);raw=so.read(limit+1);require(len(raw)<=limit,'stdout cap')
   if inspect:
    obj=strict(raw) if proc.returncode==0 else None
    (d/'stdout.json').write_text(json.dumps(safe_inspect(obj),indent=2)+'\n')
   else:obj=None
   return proc.returncode,raw,obj
  finally:
   # Reap the exact launched client on every exception/signal path before Docker cleanup.
   previous={s:signal.signal(s,signal.SIG_IGN) for s in (signal.SIGINT,signal.SIGTERM)}
   try:
    if proc is not None and proc.poll() is None:
     result['client_termination_required']=True
     try:os.killpg(proc.pid,signal.SIGTERM)
     except ProcessLookupError:pass
     try:proc.wait(timeout=1)
     except subprocess.TimeoutExpired:
      try:os.killpg(proc.pid,signal.SIGKILL)
      except ProcessLookupError:pass
      proc.wait(timeout=1)
    if proc is not None:
     result['client_terminal_exit']=proc.poll();unreaped_client=proc.poll() is None
    require(proc is None or proc.poll() is not None,'owned client not terminal; cleanup unsafe')
    if so is not None:
     if inspect:
      so.seek(0);data=so.read(limit+1);result['inspection_raw_sha256']=H(data);result['inspection_raw_bytes_read']=len(data);result['inspection_raw_persisted']=False
     so.close()
    if se is not None:se.close()
   finally:
    if so is not None and not so.closed:so.close()
    if se is not None and not se.closed:se.close()
    for signum,handler in previous.items():signal.signal(signum,handler)
    result.update({'argv':argv,'started_monotonic':t,'ended_monotonic':time.monotonic(),'cap_seconds':timeout,'environment_names':sorted(env),'docker_environment':'empty allowlist; no endpoint/TLS/proxy/credential selectors','supervisor_deadline':deadline})
    (d/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
 def cli(label,*argv,**kw):return command(label,prefix+list(argv),**kw)
 def identity(cleanup=False):
  nonlocal owned
  require(pending is not None,'no owned container identity')
  target=owned or pending['name'];rc,raw,obj=cli('identity','inspect',target,inspect=True,cleanup=cleanup)
  require(rc==0 and isinstance(obj,list) and len(obj)==1,'container identity unavailable')
  c=obj[0];require(c['Name']=='/'+pending['name'] and c['Image']==args.image_id and c['Config']['Labels'].get('isolab.runtime-qualification.owner')==pending['nonce'],'container identity mismatch')
  require(re.fullmatch('[0-9a-f]{64}',c['Id']),'container ID shape')
  if owned:require(c['Id']==owned,'container ID changed')
  owned=c['Id'];return c
 def cleanup_one(final=False):
  nonlocal owned,pending,clean
  require(not unreaped_client,'owned client not reaped; retain container without cleanup race')
  if pending is None:return
  c=identity(cleanup=final)
  if c['State']['Running']:
   rc,_,_=cli('kill','kill',owned,cleanup=True,cap=2);require(rc==0,'kill failed')
   rc,_,_=cli('wait-cleanup','wait',owned,cleanup=final);require(rc==0,'wait failed')
   c=identity(cleanup=final)
  require(not c['State']['Running'],'container not confirmed stopped')
  rc,_,_=cli('remove','rm',owned,cleanup=final);require(rc==0,'remove failed')
  # Successful exact-ID rm is the confirmed removal receipt; never broad prune.
  owned=None;pending=None;clean=True
 def fixture_snapshot():
  verify_fixture_members()
  d=FIX/'prepared/fixtures';rows={}
  require(not d.is_symlink() and stat.S_IMODE(d.stat().st_mode)==0o555,'fixture directory mode')
  for p in d.iterdir():
   st=p.lstat();require(stat.S_ISREG(st.st_mode) and st.st_nlink==1 and stat.S_IMODE(st.st_mode)==0o444,'fixture file identity/mode');rows[p.name]=H(p.read_bytes())
  return rows
 def create(case,entry,arguments):
  nonlocal pending,owned,clean
  require(pending is None and owned is None,'one container at a time');nonce=str(uuid.uuid4());pending={'name':'isolab-runtime-q-'+nonce,'nonce':nonce,'case':case};clean=False
  argv=['create','--pull=never','--name',pending['name'],'--label','isolab.runtime-qualification.owner='+nonce,'--network','none','--ipc','private','--read-only','--cap-drop','ALL','--security-opt','no-new-privileges','--pids-limit','1','--memory','512m','--memory-swap','512m','--cpus','1','--user','1000:1000','--entrypoint',entry,'--mount','type=bind,src='+str(FIX/'prepared/fixtures')+',dst=/fixtures,readonly','--tmpfs','/tmp:rw,noexec,nosuid,nodev,size=16m,mode=1777','--tmpfs','/work:rw,noexec,nosuid,nodev,size=16m,mode=1777',args.image_id]+arguments
  rc,raw,_=cli('create-'+case,*argv);require(rc==0,'create failure or ambiguity');owned=raw.decode().strip();c=identity()
  hc=c['HostConfig'];require(hc['NetworkMode']=='none' and hc['ReadonlyRootfs'] and hc['PidsLimit']==1 and hc['Memory']==512*1024**2 and hc['MemorySwap']==512*1024**2 and hc['NanoCpus']==10**9,'container limits mismatch')
  require(c['Config']['User']=='1000:1000' and c['Config']['Entrypoint']==[entry],'container executable/user mismatch')
  require(hc.get('CapDrop')==['ALL'] and 'no-new-privileges' in hc.get('SecurityOpt',[]),'privilege restriction mismatch')
  require(not hc.get('Privileged') and hc.get('PidMode','')=='' and hc.get('UTSMode','')=='' and hc.get('IpcMode')=='private' and not hc.get('Devices') and not hc.get('DeviceRequests') and not hc.get('CapAdd'),'namespace/device/capability mismatch')
  require(set(hc.get('SecurityOpt',[]))=={'no-new-privileges'},'non-default seccomp/security option')
  mounts=c['Mounts'];require(len({m['Destination'] for m in mounts})==len(mounts),'duplicate mount destination');require(all(m['Type'] in ['bind','tmpfs'] for m in mounts),'unapproved mount type');require(all(m['Destination'] in ['/tmp','/work'] for m in mounts if m['Type']=='tmpfs'),'unapproved tmpfs destination');binds=[m for m in mounts if m['Type']=='bind'];require(len(binds)==1 and binds[0]['Source']==str(FIX/'prepared/fixtures') and binds[0]['Destination']=='/fixtures' and not binds[0]['RW'],'mount mismatch')
  require(set(hc['Tmpfs'])=={'/tmp','/work'},'unexpected tmpfs')
  for options in hc['Tmpfs'].values():require(set(options.split(','))=={'rw','noexec','nosuid','nodev','size=16m','mode=1777'},'tmpfs options mismatch')
  return c
 def copy_tar(path,label,maxbytes):
  rc,raw,_=cli('copy-'+label,'cp',owned+':'+path,'-',limit=maxbytes);require(rc==0,'copy failed');result={}
  with tarfile.open(fileobj=io.BytesIO(raw),mode='r:*') as tf:
   for m in tf:
    parts=P(m.name).parts;require(not m.name.startswith('/') and '..' not in parts and m.name not in result,'unsafe archive member')
    require(m.isfile() or m.isdir(),'archive special file')
    result[m.name]={'mode':m.mode,'uid':m.uid,'gid':m.gid,'directory':m.isdir(),'bytes':b'' if m.isdir() else tf.extractfile(m).read()}
  return result
 def pids_sample(c,case):
  pid=c['State'].get('Pid',0);obs={'case':case,'configured_pids_limit':1,'effective_kernel_observed':False}
  if pid and c['State']['Running']:
   try:
    lines=P(f'/proc/{pid}/cgroup').read_text().splitlines()
    for line in lines:
     hierarchy,controllers,path=line.split(':',2)
     if hierarchy=='0' and controllers=='':
      require(path.startswith('/') and '..' not in P(path).parts,'cgroup path');v=(P('/sys/fs/cgroup')/path.lstrip('/')/'pids.max').read_text().strip();obs.update({'effective_kernel_observed':True,'pids_max':v});require(v=='1','effective pids.max differs')
   except (OSError,ProcessLookupError):pass
  pids_observations.append(obs)
 def run_case(case,entry,arguments,expected_code,needle=None):
  nonlocal active_deadline
  create(case,entry,arguments);active_deadline=time.monotonic()+20
  rc,_,_=cli('start-'+case,'start',owned);require(rc==0,'start failed');c=identity();pids_sample(c,case)
  rc,raw,_=cli('wait-'+case,'wait',owned,cap=20);require(rc==0,'wait failed');code=int(raw.strip());require(code==expected_code,'unexpected process exit')
  c=identity();require(not c['State']['Running'] and c['State']['ExitCode']==expected_code,'terminal state mismatch');active_deadline=None
  rc,raw,_=cli('logs-'+case,'logs',owned);require(rc==0,'logs failed')
  # Docker logs may put stderr on client's stderr. Read both preserved channels.
  err=(out/f'{seq:03d}-logs-{case}'/'stderr').read_bytes();combined=raw+err
  if needle:require(needle.encode() in combined,'expected rejection absent');require(b'ecbench.diagnostic_preflight_file_report/v1' not in combined,'negative control success JSON')
  else:require(strict(raw)==strict((FIX/'expected-report.json').read_bytes()),'positive report mismatch')
  rc,diff,_=cli('diff-'+case,'diff',owned);require(rc==0 and not diff.strip(),'unexpected writable-layer change')
  cleanup_one();passed.append(case)
 def interrupted(signum,frame):raise Stop('supervisor signal '+str(signum))
 signal.signal(signal.SIGTERM,interrupted);signal.signal(signal.SIGINT,interrupted)
 try:
  before=fixture_snapshot();(out/'bindings.json').write_bytes(binding_raw)
  rc,raw,info=cli('daemon-info','info','--format','{{json .}}',inspect=True);require(rc==0 and info.get('OSType')=='linux' and info.get('PidsLimit') is True,'local daemon PID-limit support unavailable')
  rc,raw,ims=cli('image','image','inspect',args.image_id,inspect=True);require(rc==0 and len(ims)==1,'exact local image missing');im=ims[0]
  require(im['Id']==args.image_id and im['Os']=='linux' and im['Architecture']=='amd64' and not im['Config'].get('OnBuild'),'image identity/architecture/OnBuild')
  require(im['Config']['User']=='0:0' and not im['Config'].get('Entrypoint') and not im['Config'].get('Volumes'),'overlay image config/anonymous volume mismatch')
  for v in im['Config'].get('Env',[]):
   key=v.split('=',1)[0];require(key in binding['approved_image_environment_names'],'unreviewed image environment key')
  create('Q0','/usr/local/bin/ecbench',['--help'])
  for path,expected,label in [('/usr/local/bin/ecbench',args.binary_sha256,'ecbench'),('/usr/local/bin/isolab-fresh-owner',OWNER,'owner')]:
   archive=copy_tar(path,label,100*1024**2);require(len(archive)==1,'unexpected executable archive');x=next(iter(archive.values()));require(not x['directory'] and x['mode']==0o755 and x['uid']==0 and x['gid']==0 and H(x['bytes'])==expected,'executable bytes/mode');(out/(label+'.elf')).write_bytes(x['bytes'])
   rc,raw,_=command('elf-'+label,['/usr/bin/readelf','-h','-l','-d',str(out/(label+'.elf'))]);require(rc==0 and b'Advanced Micro Devices X86-64' in raw and b'INTERP' not in raw and b'(NEEDED)' not in raw,'static ELF check')
  # This prospective extension reads /opt only to inventory every ancestor mode.
  archive=copy_tar('/opt','opt',64*1024**2);runtime=strict(MEMBERS.read_bytes());expected_paths={'opt','opt/isolab','opt/isolab/specs','opt/isolab/specs/smoke-prime.json','opt/isolab/build-info.json'}|{'opt/isolab/recovery'+r['path'][len('runtime'):] for r in runtime}
  require(set(archive)==expected_paths,'unexpected /opt members; no broad extraction')
  inventory=[]
  directory_paths={'opt','opt/isolab','opt/isolab/specs'}|{'opt/isolab/recovery'+r['path'][len('runtime'):] for r in runtime if r['kind']=='directory'}
  for name,x in archive.items():
   require(x['directory']==(name in directory_paths),'archive kind mismatch')
   require(x['uid']==0 and x['gid']==0,'image file ownership');require((x['mode']&0o005)==0o005 if x['directory'] else True,'ancestor traversal')
   if name=='opt/isolab/build-info.json':require(strict(x['bytes'])==expected_build and H(x['bytes'])==binding['build_info_sha256'],'build-info mismatch')
   if name=='opt/isolab/specs/smoke-prime.json':require(H(x['bytes'])==binding['smoke_prime_sha256'],'smoke fixture mismatch')
   if name.startswith('opt/isolab/recovery'):
    r=next(z for z in runtime if 'opt/isolab/recovery'+z['path'][len('runtime'):]==name);require(x['mode']==int(r['proposed_context_mode'],8),'runtime mode');require(x['directory']==(r['kind']=='directory'),'runtime member kind')
    if r['kind']=='file':require(len(x['bytes'])==r['bytes'] and H(x['bytes'])==r['sha256'],'runtime file length/digest')
   inventory.append({'path':name,'mode':oct(x['mode']),'uid':x['uid'],'gid':x['gid'],'sha256':None if x['directory'] else H(x['bytes'])})
  (out/'image-file-inventory.json').write_text(json.dumps(inventory,indent=2)+'\n');cleanup_one();passed.append('Q0')
  prep=strict((FIX/'prepared/preparation.json').read_bytes());run_case('Q1','/usr/local/bin/ecbench',['diagnostic-preflight','--request','/fixtures/request.json','--request-sha256',prep['rewritten_request_sha256']],0)
  run_case('Q2','/usr/local/bin/ecbench',['diagnostic-preflight','--request','/fixtures/request.json','--request-sha256',prep['wrong_request_sha256']],2,'request hash mismatch')
  run_case('Q3','/usr/local/bin/isolab-fresh-owner',['--check','/fixtures/owner-invalid.json',prep['wrong_owner_invalid_sha256']],125,'fresh owner: config digest mismatch')
  require(fixture_snapshot()==before,'fixture changed');passed.append('Q4')
 except BaseException as e:failure=type(e).__name__+': '+str(e)
 finally:
  if pending:
   active_deadline=None
   try:cleanup_one(final=True)
   except BaseException as e:(out/'CLEANUP-INCOMPLETE').write_text(str(e)+'\n');failure=(failure or '')+'; cleanup incomplete '+str(e)
  effective=all(x['effective_kernel_observed'] for x in pids_observations) and len(pids_observations)==3
  result={'schema':'software.runtime-qualification-supervisor/v1','passed_cases':passed,'failure':failure,'cleanup_complete':pending is None,'elapsed_seconds':time.monotonic()-start,'cap_seconds':300,'image_id':args.image_id,'binary_sha256':args.binary_sha256,'effective_pids_observations':pids_observations,'effective_pids_observed_for_all_active_cases':effective,'effective_before_start_proved':False,'qualification_complete':False,'status':'checks_retained_pending_effective_prestart_pid_gate' if failure is None else 'failed','scientific_execution':False,'note':'The draft requires effective pids.max before start; configuration/support or after-start sampling does not satisfy that stronger gate.'}
  (out/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
 return 1 if failure else 3
if __name__=='__main__':
 try:raise SystemExit(main())
 except BaseException as e:
  import sys
  if isinstance(e,SystemExit):raise
  if '--receipts' in sys.argv:
   d=P(sys.argv[sys.argv.index('--receipts')+1])
   if d.is_dir() and not (d/'receipt.json').exists():
    (d/'failure-fallback.json').write_text(json.dumps({'status':'outer_failure','error':type(e).__name__+': '+str(e),'docker_actions_started':None,'note':'Unknown action state; reconcile retained command and owned-container receipts. This fallback does not assert no actions.'},indent=2)+'\n')
  raise SystemExit('pre-admission stop: '+str(e))
