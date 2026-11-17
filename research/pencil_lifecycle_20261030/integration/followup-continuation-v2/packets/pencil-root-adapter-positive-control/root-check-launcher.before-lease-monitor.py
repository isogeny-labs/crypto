#!/usr/bin/env python3
import datetime, hashlib, json, os, signal, subprocess, sys, time, traceback
from pathlib import Path
P=Path(__file__).resolve().parent;F=json.loads((P/'root-check-freeze.json').read_text());START=time.monotonic()
UTC=datetime.datetime.now(datetime.timezone.utc); DEADLINE=datetime.datetime.fromisoformat(F['absolute_deadline']); END=START+min(1800,(DEADLINE-UTC).total_seconds())
W=Path(F['workspace']);OUT=W/'root-check-launch1';UNIT=F['unit'];R={'status':'failed','unit':UNIT,'errors':[],'unit_launch_attempted':False,'observations':[]};INTERRUPTED=False
KEYS=['Id','LoadState','ActiveState','SubState','MainPID','InvocationID','ControlGroup','Result','ExecMainCode','ExecMainStatus','MemoryPeak','CPUUsageNSec']
def sig(sig,frame):
 global INTERRUPTED
 INTERRUPTED=True
 raise RuntimeError('signal '+str(sig))
for s in (signal.SIGTERM,signal.SIGINT):signal.signal(s,sig)
def call(argv,timeout=5):
 t=max(.1,min(timeout,END-time.monotonic()))
 x=subprocess.run(argv,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=t)
 R.setdefault('queries',[]).append({'argv':argv,'exit_code':x.returncode,'stdout':x.stdout.decode(errors='replace'),'stderr':x.stderr.decode(errors='replace')})
 if x.returncode:raise RuntimeError('command failed: '+repr(argv))
 return x.stdout.decode()
def show():
 value=call(['/usr/bin/systemctl','show',UNIT]+['--property='+k for k in KEYS]);d=dict(line.split('=',1) for line in value.splitlines() if '=' in line);R['observations'].append(d);return d
def empty(cgroup):
 if not cgroup:return True
 path=Path('/sys/fs/cgroup')/cgroup.lstrip('/')
 if not path.exists():return True
 return not (path/'cgroup.procs').read_text().strip() and 'populated 0' in (path/'cgroup.events').read_text()
try:
 assert END-START>=300,'insufficient absolute deadline margin'
 assert not OUT.exists() and not (W/'root-check-attempt1').exists(),'no duplicate attempt'
 OUT.mkdir(mode=0o700)
 for name,digest in F['recipe_files'].items():assert hashlib.sha256((P/name).read_bytes()).hexdigest()==digest,name
 before=show();assert before['LoadState']=='not-found','unit already exists'
 props={'User':str(F['uid']),'Group':str(F['gid']),'Type':'exec','RemainAfterExit':'yes','CPUQuota':'200%','MemoryMax':str(16*1024**3),'MemorySwapMax':'0','OOMPolicy':'stop','TasksMax':'512','KillMode':'control-group','TimeoutStopSec':'5s','RuntimeMaxSec':str(int(END-time.monotonic()-20))+'s','NoNewPrivileges':'yes','CapabilityBoundingSet':'','ProtectControlGroups':'yes','RestrictNamespaces':'yes','PrivateNetwork':'yes','PrivateTmp':'no','ProtectSystem':'strict','ProtectHome':'read-only','ReadWritePaths':str(W),'WorkingDirectory':str(W/'source'),'UMask':'0077','StandardOutput':'append:'+str(OUT/'unit.stdout'),'StandardError':'append:'+str(OUT/'unit.stderr')}
 argv=['/usr/bin/sudo','-n','/usr/bin/systemd-run','--no-block','--unit='+UNIT]+['--property='+k+'='+v for k,v in props.items()]+['--setenv=PENCIL_CHECK_END_MONOTONIC='+str(END),'/usr/bin/python3','-B',str(P/'root-check-worker.py')]
 R['launch_argv']=argv;R['properties']=props;R['unit_launch_attempted']=True
 call(argv,10);owned=None;cgroup=None
 while time.monotonic()<END-15:
  if INTERRUPTED:raise RuntimeError('interrupted')
  d=show()
  if d.get('InvocationID'):
   if owned is None:owned=d['InvocationID'];cgroup=d.get('ControlGroup');R['invocation_id']=owned;R['cgroup']=cgroup
   else:assert d['InvocationID']==owned,'unit ownership changed'
  if d.get('ActiveState')=='failed' or (d.get('ActiveState')=='active' and d.get('SubState')=='exited') or d.get('LoadState')=='not-found':
   R['terminal']=d;break
  time.sleep(.5)
 else:raise RuntimeError('unit observation deadline')
 assert owned and R['terminal'].get('ExecMainStatus')=='0' and R['terminal'].get('Result')=='success','unit did not succeed'
 worker=json.loads((W/'root-check-attempt1/worker-receipt.json').read_text());assert worker['status']=='passed' and worker['invocation_id']==owned and worker['post_inputs_verified']
 R['worker_receipt_sha256']=hashlib.sha256((W/'root-check-attempt1/worker-receipt.json').read_bytes()).hexdigest();R['status']='passed'
except BaseException as e:R['errors'].append({'repr':repr(e),'traceback':traceback.format_exc()})
finally:
 if R['unit_launch_attempted']:
  try:
   call(['/usr/bin/sudo','-n','/usr/bin/systemctl','stop','--no-block',UNIT],5)
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
 if OUT.is_dir():(OUT/'launch-receipt.json').write_text(json.dumps(R,indent=2)+'\n')
print(json.dumps({'status':R['status'],'elapsed_seconds':R['elapsed_seconds'],'cleanup_complete':R.get('cleanup_complete',False)}))
raise SystemExit(0 if R['status']=='passed' and not INTERRUPTED and time.monotonic()<END else 1)
