#!/usr/bin/env python3
import datetime, hashlib, json, os, re, signal, subprocess, sys, time, traceback, urllib.request, urllib.error
from pathlib import Path
from durable_io import atomic_json, flush_logs, load_admission, seal_outputs, durable_limits
def parse_rfc3339(value):
 # RFC3339Nano expiry, conservatively floored to Python microseconds.
 if not isinstance(value,str):raise ValueError('RFC3339 string required')
 match=re.fullmatch(r'([0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2})(?:\.([0-9]{1,9}))?(Z|[+-][0-9]{2}:[0-9]{2})',value)
 if match is None:raise ValueError('invalid RFC3339 format')
 base,fraction,zone=match.groups()
 if int(base[17:19])>59:raise ValueError('leap seconds unsupported')
 if zone=='-00:00':raise ValueError('unknown timezone offset')
 if zone!='Z' and (int(zone[1:3])>23 or int(zone[4:6])>59):raise ValueError('invalid timezone offset')
 normalized=base+('.'+((fraction+'000000')[:6]) if fraction else '')+('+00:00' if zone=='Z' else zone)
 return datetime.datetime.fromisoformat(normalized).astimezone(datetime.timezone.utc)

P=Path(__file__).resolve().parent;F=load_admission(P);START=time.monotonic()
UTC=datetime.datetime.now(datetime.timezone.utc); DEADLINE=parse_rfc3339(F['absolute_deadline']); END=START+min(1800,(DEADLINE-UTC).total_seconds())
W=Path(F['workspace']);D=Path(F['durable_run']);OUT=D/'supervisor';UNIT=F['unit'];R={'status':'failed','unit':UNIT,'errors':[],'unit_launch_attempted':False,'observations':[]};INTERRUPTED=False;CLEANING=False;OWNED_OUTPUT=False
def checkpoint(phase):
 R['phase']=phase;R['elapsed_seconds']=time.monotonic()-START
 if OWNED_OUTPUT:
  try:
   if not CLEANING:R["durable_size"]=durable_limits(D,F["limits"])
   atomic_json(OUT/'launch-receipt.json',R);flush_logs(OUT)
  except OSError as error:
   R['errors'].append({'durability_failure':type(error).__name__})
   if not CLEANING:raise
KEYS=['Id','LoadState','ActiveState','SubState','MainPID','InvocationID','ControlGroup','Result','ExecMainCode','ExecMainStatus','MemoryPeak','CPUUsageNSec']
def sig(sig,frame):
 global INTERRUPTED
 INTERRUPTED=True
 raise RuntimeError('signal '+str(sig))
for s in (signal.SIGTERM,signal.SIGINT):signal.signal(s,sig)
HB_NEXT=0; HB_EXPIRY=None
CREDENTIALS=json.loads(Path('/home/ec2-user/.conductor/credentials').read_text())
def heartbeat(force=False):
 global HB_NEXT,HB_EXPIRY
 now=datetime.datetime.now(datetime.timezone.utc)
 if HB_EXPIRY is not None and (HB_EXPIRY-now).total_seconds()<15:raise RuntimeError('lease expiry safety boundary')
 if not force and time.monotonic()<HB_NEXT:return
 HB_NEXT=time.monotonic()+20
 request=urllib.request.Request(CREDENTIALS['endpoint'].rstrip('/')+'/v1/leases/heartbeat',data=json.dumps(F['lease']).encode(),headers={'Authorization':'Bearer '+CREDENTIALS['token'],'Content-Type':'application/json'},method='POST')
 try:
  with urllib.request.urlopen(request,timeout=5) as response:
   value=json.load(response);status=response.status
  assert status==200
  for k in ['task_id','attempt_id','fencing_epoch']:assert value[k]==F['lease'][k],'lease identity mismatch'
  assert value['id']==F['lease']['lease_id'],'lease id mismatch'
  expiry=parse_rfc3339(value['expires_at'])
  assert (expiry-now).total_seconds()>30,'lease renewal too short'
  HB_EXPIRY=expiry
  R.setdefault('heartbeats',[]).append({'status':200,'task_id':value['task_id'],'attempt_id':value['attempt_id'],'lease_id':value['id'],'fencing_epoch':value['fencing_epoch'],'expires_at':value['expires_at']})
  checkpoint('lease-renewed')
 except urllib.error.HTTPError as error:
  R.setdefault('heartbeats',[]).append({'status':error.code})
  if error.code in (401,403,404,409):raise RuntimeError('definitive lease loss HTTP'+str(error.code))
  HB_NEXT=time.monotonic()+5
  if HB_EXPIRY is None:raise RuntimeError('initial lease renewal failed')
 except (urllib.error.URLError,TimeoutError,OSError) as error:
  R.setdefault('heartbeats',[]).append({'transport_error_type':type(error).__name__})
  HB_NEXT=time.monotonic()+5
  if HB_EXPIRY is None:raise RuntimeError('no maintained initial lease')

def call(argv,timeout=5,record_checkpoint=True):
 t=max(.1,min(timeout,END-time.monotonic()))
 x=subprocess.run(argv,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=t)
 R.setdefault('queries',[]).append({'argv':argv,'exit_code':x.returncode,'stdout':x.stdout.decode(errors='replace'),'stderr':x.stderr.decode(errors='replace')})
 if record_checkpoint:checkpoint('query-recorded')
 if x.returncode:raise RuntimeError('command failed: '+repr(argv))
 return x.stdout.decode()
def show():
 value=call(['/usr/bin/systemctl','show',UNIT]+['--property='+k for k in KEYS],record_checkpoint=False)
 d=dict(line.split('=',1) for line in value.splitlines() if '=' in line)
 if R['unit_launch_attempted'] and d.get('InvocationID'):
  if R.get('invocation_id'):assert d['InvocationID']==R['invocation_id'],'unit ownership changed'
  else:R['invocation_id']=d['InvocationID']
  if d.get('ControlGroup'):
   if R.get('cgroup'):assert d['ControlGroup']==R['cgroup'],'unit cgroup changed'
   R['cgroup']=d['ControlGroup']
 R['observations'].append(d);checkpoint('query-recorded');return d
def permit():
 assert R.get('invocation_id') and HB_EXPIRY is not None
 now=datetime.datetime.now(datetime.timezone.utc)
 expires=min(HB_EXPIRY.timestamp()-15,now.timestamp()+F['limits']['supervisor_permit_seconds'],DEADLINE.timestamp()-40)
 assert expires>now.timestamp() and not INTERRUPTED
 atomic_json(D/'supervisor-permit.json',{'invocation_id':R['invocation_id'],'lease':F['lease'],'expires_unix':expires})
def empty(cgroup):
 if not cgroup:return True
 path=Path('/sys/fs/cgroup')/cgroup.lstrip('/')
 if not path.exists():return True
 return not (path/'cgroup.procs').read_text().strip() and 'populated 0' in (path/'cgroup.events').read_text()
def precreate_unit_logs(directory):
 directory=Path(directory)
 for name in ('unit.stdout','unit.stderr'):
  fd=os.open(directory/name,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
  try:
   os.fchmod(fd,0o600)
   st=os.fstat(fd)
   assert st.st_uid==os.geteuid() and st.st_gid==os.getegid(),'caller-owned unit log required'
   assert st.st_mode&0o777==0o600,'private unit log required'
   os.fsync(fd)
  finally:os.close(fd)
 fd=os.open(directory,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
try:
 assert END-START>=300,'insufficient absolute deadline margin'
 assert not D.exists(),'no duplicate durable attempt directory'
 D.mkdir(mode=0o700);OUT.mkdir(mode=0o700);OWNED_OUTPUT=True;checkpoint('supervisor-start')
 for name,digest in F['recipe_files'].items():assert hashlib.sha256((P/name).read_bytes()).hexdigest()==digest,name
 heartbeat(force=True)
 before=show();assert before['LoadState']=='not-found','unit already exists'
 precreate_unit_logs(OUT);checkpoint('caller-owned-unit-logs-created')
 props={'User':str(F['uid']),'Group':str(F['gid']),'Type':'exec','RemainAfterExit':'yes','CPUQuota':'200%','MemoryMax':str(16*1024**3),'MemorySwapMax':'0','OOMPolicy':'stop','TasksMax':'512','KillMode':'control-group','TimeoutStopSec':'5s','RuntimeMaxSec':str(int(END-time.monotonic()-20))+'s','NoNewPrivileges':'yes','CapabilityBoundingSet':'','ProtectControlGroups':'yes','RestrictNamespaces':'yes','PrivateNetwork':'yes','PrivateTmp':'no','ProtectSystem':'strict','ProtectHome':'read-only','ReadWritePaths':str(W)+' '+str(D),'WorkingDirectory':str(W/'source'),'UMask':'0077','StandardOutput':'append:'+str(OUT/'unit.stdout'),'StandardError':'append:'+str(OUT/'unit.stderr')}
 argv=['/usr/bin/sudo','-n','/usr/bin/systemd-run','--no-block','--unit='+UNIT]+['--property='+k+'='+v for k,v in props.items()]+['--setenv=PENCIL_CHECK_END_MONOTONIC='+str(END),'/usr/bin/python3','-B',str(P/'durable-worker.py')]
 R['launch_argv']=argv;R['properties']=props;R['unit_launch_attempted']=True;checkpoint('before-unit-launch')
 call(argv,10,record_checkpoint=False);owned=None;cgroup=None
 while time.monotonic()<END-15:
  heartbeat()
  if INTERRUPTED:raise RuntimeError('interrupted')
  d=show()
  if d.get('InvocationID'):
   if owned is None:owned=d['InvocationID'];cgroup=d.get('ControlGroup');R['invocation_id']=owned;R['cgroup']=cgroup
   else:assert d['InvocationID']==owned,'unit ownership changed'
  if d.get('ControlGroup'):
   if cgroup:assert cgroup==d['ControlGroup'],'unit cgroup changed'
   cgroup=d['ControlGroup'];R['cgroup']=cgroup
  if R.get('invocation_id'):permit()
  if d.get('ActiveState')=='failed' or (d.get('ActiveState')=='active' and d.get('SubState')=='exited') or d.get('LoadState')=='not-found':
   R['terminal']=d;break
  time.sleep(.5)
 else:raise RuntimeError('unit observation deadline')
 assert owned and R['terminal'].get('ExecMainStatus')=='0' and R['terminal'].get('Result')=='success','unit did not succeed'
 worker=json.loads((D/'worker/worker-receipt.json').read_text());assert worker['status']=='passed' and worker['invocation_id']==owned and worker['post_inputs_verified']
 assert cgroup and worker['cgroup']['path']==str(Path('/sys/fs/cgroup')/cgroup.lstrip('/')),'worker and supervisor cgroup join'
 R['worker_receipt_sha256']=hashlib.sha256((D/'worker/worker-receipt.json').read_bytes()).hexdigest();R['status']='passed'
except BaseException as e:R['errors'].append({'repr':repr(e),'traceback':traceback.format_exc()})
finally:
 CLEANING=True
 if R['unit_launch_attempted']:
  try:
   current=show()
   if current.get('LoadState')!='not-found':
    assert R.get('invocation_id') and current.get('InvocationID')==R['invocation_id'],'cannot stop unit without exact owned InvocationID'
    call(['/usr/bin/sudo','-n','/usr/bin/systemctl','stop','--no-block',UNIT],5)
   assert R.get('cgroup'),'no captured owned cgroup; cleanup unresolved'
   while time.monotonic()<END-2:
    d=show();current=d.get('InvocationID')
    if current and R.get('invocation_id'):assert current==R['invocation_id'],'cleanup ownership changed'
    if d.get('ActiveState') in ('inactive','failed') and d.get('MainPID','0')=='0' and empty(R.get('cgroup')):
     R['cleanup_complete']=True;R['stopped']=d;break
    if d.get('LoadState')=='not-found' and empty(R.get('cgroup')):
     R['cleanup_complete']=True;R['stopped']=d;break
    time.sleep(.2)
   assert R.get('cleanup_complete'),'unit/cgroup not proven stopped'
  except BaseException as e:R['status']='failed';R['errors'].append({'cleanup':repr(e),'traceback':traceback.format_exc()})
 if R['errors'] or INTERRUPTED or time.monotonic()>=END:R['status']='failed'
 R['elapsed_seconds']=time.monotonic()-START;R['absolute_deadline']=F['absolute_deadline']
 if OWNED_OUTPUT:
  payload_status=R['status'];R['payload_status']=payload_status
  R['status']='pending_seal';R['seal_status']='pending';checkpoint('terminal-before-seal')
  try:
   assert R.get('cleanup_complete'),'cannot seal before owned cleanup'
   seal_outputs(D)
   manifest_sha256=hashlib.sha256((D/'terminal-output-manifest.json').read_bytes()).hexdigest()
   completion_utc=datetime.datetime.now(datetime.timezone.utc)
   if R['errors'] or INTERRUPTED or time.monotonic()>=END or completion_utc>=DEADLINE:payload_status='failed'
   atomic_json(D/'completion.json',{'status':payload_status,'seal_status':'passed','terminal_manifest_sha256':manifest_sha256,'recorded_utc':completion_utc.isoformat(),'elapsed_seconds':time.monotonic()-START,'absolute_deadline':F['absolute_deadline']})
   R['status']=payload_status;R['seal_status']='passed'
  except BaseException as error:
   R['status']='failed';R['seal_status']='failed';R['errors'].append({'sealing':repr(error)})
   checkpoint('terminal-unsealed')
print(json.dumps({'status':R['status'],'elapsed_seconds':R['elapsed_seconds'],'cleanup_complete':R.get('cleanup_complete',False)}))
raise SystemExit(0 if R['status']=='passed' and not INTERRUPTED and time.monotonic()<END else 1)
