#!/usr/bin/python3
"""Single prospective continuation. Invoke only after publication and independent review."""
from pathlib import Path
import argparse,hashlib,json,os,subprocess,time,signal
P=Path(__file__).resolve().parent
W=Path('/home/ec2-user/pencil-native-pivot-control-successor-20261111')
O=Path(str(W)+'.outer')
DEADLINE=1794398400
BOOT='31b37cfb156242ff8b0ffadc5cec5a12'
ap=argparse.ArgumentParser();ap.add_argument('--execute',action='store_true');ap.add_argument('--freeze-sha256',required=True)
ap.add_argument('--release-record',required=True);ap.add_argument('--release-sha256',required=True)
a=ap.parse_args()
assert a.execute,'Preparation only; explicit execution required after independent review and publication'
assert hashlib.sha256((P/'freeze.json').read_bytes()).hexdigest()==a.freeze_sha256,'freeze mismatch'
F=json.loads((P/'freeze.json').read_text())
assert str(W)==F['workspace'],'workspace binding mismatch'
for name,digest in F['files'].items():assert hashlib.sha256((P/name).read_bytes()).hexdigest()==digest,('frozen input',name)
assert Path('/proc/sys/kernel/random/boot_id').read_text().strip().replace('-','')==BOOT,'boot changed'
# Existing coordinator release binds this exact successor after serial reconciliation.
release_path=Path(a.release_record).resolve()
assert hashlib.sha256(release_path.read_bytes()).hexdigest()==a.release_sha256,'release pin'
release=json.loads(release_path.read_text())
assert release['schema']=='solve4-pivot-swap-coordinator-release/v1' and release['released'] is True
assert release['freeze_sha256']==a.freeze_sha256
assert release['attempt_limit']==1 and release['automatic_retry'] is False
assert release['campaign_total_seconds']==129600 and release['deadline']=='2026-11-11T12:00:00Z'
assert type(F['prior_attempt_conservative_debit_seconds']) is int, 'campaign ledger binding pending'
assert type(F['remaining_before_attempt_seconds']) is int
assert 0<=F['prior_attempt_conservative_debit_seconds']<=129600-2400
assert F['remaining_before_attempt_seconds']>=2400
assert F['prior_attempt_conservative_debit_seconds']+F['remaining_before_attempt_seconds']==129600
assert release['prior_debit_seconds']==F['prior_attempt_conservative_debit_seconds']
assert release['remaining_before_attempt_seconds']==F['remaining_before_attempt_seconds']
assert type(F['materializer_conservative_debit_seconds']) is int and F['materializer_conservative_debit_seconds']>=0
assert release['materializer_conservative_debit_seconds']==F['materializer_conservative_debit_seconds']
assert release['serial_materializer_terminal_confirmed'] is True
assert release['coordinator_live_lease_confirmed'] is True
assert set(release['lease'])=={'task_id','attempt_id','lease_id','fencing_epoch'}
assert type(release['lease']['fencing_epoch']) is int and release['lease']['fencing_epoch']>0
for key in ('task_id','attempt_id','lease_id'):assert isinstance(release['lease'][key],str) and release['lease'][key]
assert release['published_github_commit'] and release['independent_review_commit']
assert set(release['receipts'])=={'publication','independent_review','ledger','materializer_terminal','lease_readback'}
for name,binding in release['receipts'].items():
 path=Path(binding['path']);assert path.is_absolute() and path.is_file()
 assert hashlib.sha256(path.read_bytes()).hexdigest()==binding['sha256'],('release receipt',name)
# Coordinator authenticates these receipts and maintains the live lease throughout.
# Phase ownership/cgroup checks below remain the original runtime authority.
assert time.time()+2400<DEADLINE,'insufficient absolute deadline allowance'
assert not W.exists(),'existing native workspace; no retries'
O.mkdir(mode=0o700)  # exclusive admission: existing outer directory also rejects repeats
start=time.monotonic()
def durable(name,obj):
 t=O/(name+'.tmp');d=O/name
 with t.open('x') as f:json.dump(obj,f,sort_keys=True);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(t,d);fd=os.open(O,os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
def unit_state(unit,end):
 remaining=end-time.monotonic()
 assert remaining>0,'no supervisor query budget'
 r=subprocess.run(['systemctl','show',unit,'--property=LoadState,ActiveState,InvocationID,ControlGroup'],capture_output=True,text=True,timeout=min(2,remaining))
 state=dict(line.split('=',1) for line in r.stdout.splitlines() if '=' in line)
 assert r.returncode==0 or (r.returncode==1 and state.get('LoadState')=='not-found'),'unit query failed'
 return state
def terminal_owner(state,invocation):
 if not invocation or state.get('InvocationID')!=invocation:return False
 if state.get('ActiveState') not in ('inactive','failed'):return False
 cg=state.get('ControlGroup')
 if cg is None:return False
 if cg=='':return True  # systemd reports no attached control group on terminal unit
 path=Path('/sys/fs/cgroup')/cg.lstrip('/')
 if not path.exists():return True
 files=list(path.rglob('cgroup.procs'))
 if (path/'cgroup.procs') not in files:files.append(path/'cgroup.procs')
 return all(not f.read_text().strip() for f in files)
def manager_terminal(unit,invocation,end):
 if not invocation:return False
 state=unit_state(unit,end)
 if state.get('LoadState')!='not-found' or state.get('InvocationID') or state.get('ActiveState') not in ('inactive','failed'):return False
 if Path('/sys/fs/cgroup/system.slice',unit if unit.endswith('.service') else unit+'.service').exists():return False
 remaining=end-time.monotonic()
 if remaining<=0:return False
 name=unit if unit.endswith('.service') else unit+'.service'
 r=subprocess.run(['journalctl','-b','-u',name,'--no-pager','-o','json','-n','30'],capture_output=True,text=True,timeout=min(2,remaining))
 if r.returncode:return False
 records=[json.loads(line) for line in r.stdout.splitlines() if line]
 exact=[v for v in records if v.get('_PID')=='1' and v.get('_UID')=='0' and v.get('_BOOT_ID')==BOOT and v.get('UNIT')==name and v.get('INVOCATION_ID')==invocation and v.get('_SYSTEMD_UNIT')=='init.scope']
 starts=[int(v['__MONOTONIC_TIMESTAMP']) for v in exact if v.get('MESSAGE_ID')=='39f53479d3a045ac8e11786248231fbf' and v.get('JOB_TYPE')=='start' and v.get('JOB_RESULT')=='done']
 successes=[int(v['__MONOTONIC_TIMESTAMP']) for v in exact if v.get('MESSAGE_ID')=='7ad2d189f7e94e70a38c781354912448']
 if not starts or not successes or min(successes)<max(starts):return False
 durable(name+'-manager-terminal.json',{'invocation_id':invocation,'records':exact,'source':'local privileged manager journal','boot':BOOT})
 return True
def phase_run(cmd,unit,phase,limit):
 phase_start=time.monotonic();end=min(phase_start+limit,start+2400,time.monotonic()+DEADLINE-time.time())
 errors=[];invocation=None;proc=None;rc=None;terminal=False;timed_out=False
 log_out=O/(phase+'-systemd.stdout');log_err=O/(phase+'-systemd.stderr')
 try:
  assert unit_state(unit,end).get('LoadState')=='not-found','unit already exists; no submission'
  with log_out.open('xb') as so,log_err.open('xb') as se:
   proc=subprocess.Popen(cmd,stdout=so,stderr=se,start_new_session=True)
   # Capture invocation identity before any unit-directed cleanup. Never act on a changed owner.
   capture_end=min(end-12,time.monotonic()+5)
   while time.monotonic()<capture_end:
    state=unit_state(unit,end);invocation=state.get('InvocationID') or None
    if invocation:break
    if proc.poll() is not None:break
    time.sleep(.1)
   assert invocation,'missing invocation identity; custody unresolved'
   try:rc=proc.wait(timeout=max(.01,end-12-time.monotonic()))
   except subprocess.TimeoutExpired:timed_out=True;errors.append('bounded wait expired')
   state=unit_state(unit,end)
   terminal=terminal_owner(state,invocation)
   if state.get('InvocationID')!=invocation:
    terminal=manager_terminal(unit,invocation,end)
    assert terminal,'invocation changed without manager terminal custody'
   if not terminal:
    # Only the captured invocation can receive a stop request; systemd stop grace is pinned to 5s.
    r=subprocess.run(['sudo','-n','systemctl','stop','--no-block',unit],capture_output=True,text=True,timeout=min(2,max(.01,end-time.monotonic())))
    assert r.returncode==0,'owned stop request failed'
    while time.monotonic()<end-3:
     state=unit_state(unit,end)
     terminal=terminal_owner(state,invocation)
     if state.get('InvocationID')!=invocation:
      terminal=manager_terminal(unit,invocation,end)
      assert terminal,'invocation changed during cleanup without manager custody'
     if terminal:break
     time.sleep(.1)
 except Exception as e:errors.append(repr(e))
 finally:
  # Reap only our own wait-client process group. This does not establish service termination.
  if proc is not None:
   if proc.poll() is None:
    try:os.killpg(proc.pid,signal.SIGKILL)
    except ProcessLookupError:pass
   try:rc=proc.wait(timeout=max(.01,min(1,end-time.monotonic())))
   except Exception as e:errors.append('client reap '+repr(e))
  if not terminal:errors.append('terminal service/cgroup custody unconfirmed; no next phase')
 native=None;receipt_error=None
 try:
  native=json.loads((W/(phase+'-receipt.json')).read_text())
  assert isinstance(native,dict) and native.get('status') in ('passed','failed'),'missing or invalid native status'
 except Exception as e:receipt_error=repr(e)
 elapsed=time.monotonic()-phase_start
 passed=rc==0 and not errors and receipt_error is None and native.get('status')=='passed' and terminal and elapsed<limit and time.time()<DEADLINE
 return {'phase':phase,'argv':cmd,'returncode':rc,'elapsed_seconds':elapsed,'invocation_id':invocation,'terminal_owner_confirmed':terminal,'wait_timeout':timed_out,'errors':errors,'native_receipt_present':native is not None,'receipt_error':receipt_error,'passed':passed,'systemd_stdout':str(log_out),'systemd_stderr':str(log_err)}
results=[]
try:
 durable('coordinator-release.json',release)
 durable('admission.json',{'freeze_sha256':a.freeze_sha256,'start_utc_seconds':time.time(),'campaign_seconds':129600,'prior_debit_seconds':release['prior_debit_seconds'],'remaining_before_attempt_seconds':release['remaining_before_attempt_seconds'],'lane':'solve4_pivot_swap','successor_attempt_limit':1,'coordinator_release_sha256':a.release_sha256,'boot_id':BOOT,'continuation_ceiling_seconds':2400,'deadline':DEADLINE,'retries':0})
 for phase,limit in [('pre',900),('compiler',600),('post',900)]:
  if phase=='compiler' and not results[-1]['passed']:break
  # Post-custody remains required after any compiler attempt, including failed attempts.
  assert time.time()+limit<DEADLINE and time.monotonic()-start+limit<=2400,'insufficient remaining phase allowance'
  unit='pencil-pivot-control-successor-20261111-'+phase
  cmd=['sudo','-n','systemd-run','--unit='+unit,'--wait',
   '--property=User=ec2-user','--property=WorkingDirectory='+str(P),
   '--property=CPUQuota=100%','--property=MemoryMax=8G','--property=MemorySwapMax=0',
   '--property=TasksMax=128','--property=KillMode=control-group','--property=OOMPolicy=stop',
   '--property=RuntimeMaxSec='+str(limit-15),'--property=TimeoutStopSec=5s','--property=SendSIGKILL=yes',
   '--property=StandardOutput=file:'+str(O/(phase+'.stdout')),
   '--property=StandardError=file:'+str(O/(phase+'.stderr')),
   '/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(limit-20)+'s',
   '/usr/bin/python3',str(P/'replay.py'),phase,a.freeze_sha256]
  durable(phase+'-command.json',{'argv':cmd,'limit_seconds':limit,'submitted':False})
  row=phase_run(cmd,unit,phase,limit)
  durable(phase+'-outer-exit.json',row);results.append(row)
  passed=row['passed']
  if not row['terminal_owner_confirmed']:break
  # No automatic retry. A failed pre ends here; compiler failure still gets post custody.
  if phase=='pre' and not passed:break
 passed=len(results)==3 and all(r['passed'] for r in results) and time.monotonic()-start<2400 and time.time()<DEADLINE
 durable('completion.json',{'status':'passed' if passed else 'failed','phases':results,'elapsed_seconds':time.monotonic()-start,'machine_review_complete':passed,'costs_are_worker_wall_not_cpu':True})
 raise SystemExit(0 if passed else 1)
except Exception as e:
 durable('launcher-failure.json',{'error':repr(e),'phases':results,'elapsed_seconds':time.monotonic()-start,'status':'failed'})
 raise
