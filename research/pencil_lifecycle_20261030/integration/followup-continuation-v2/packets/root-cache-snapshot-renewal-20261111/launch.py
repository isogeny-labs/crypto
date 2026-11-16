from pathlib import Path
import json,hashlib,subprocess,time,os,datetime
s=Path(__file__).resolve().parent
h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
r=json.loads((s/'release.json').read_text())
assert r['released'] is True and r['prior_payload_terminal'] is True and r['no_active_cache_writer'] is True
assert r['campaign_total_seconds']==129600 and r['deadline_utc']=='2026-11-11T12:00:00Z'
assert type(r['prior_debit_seconds']) is int and type(r['remaining_before_attempt_seconds']) is int
assert r['prior_debit_seconds']>=0 and r['prior_debit_seconds']+r['remaining_before_attempt_seconds']==129600
assert r['remaining_before_attempt_seconds']>=60 and r['attempt_limit']==1 and r['automatic_retry'] is False
assert h(__file__)==r['caller_sha256']
assert set(r['receipts'])=={'publication','independent_review','ledger','serial_terminal','lease_readback'}
for name,row in r['receipts'].items():
 assert Path(row['path']).is_absolute() and h(row['path'])==row['sha256'],name
cfgpath=s/'snapshot-launch.json';assert h(cfgpath)==r['command_sha256'];cfg=json.loads(cfgpath.read_text())
assert r['source_publication'] and r['qualification_passed'] is True
assert h(r['qualification_result_path'])==r['qualification_result_sha256']
for n,digest in [('snapshot-cache.py',cfg['source_sha256']),('durable_io.py',cfg['helper_sha256'])]:
 assert h(s/n)==digest
 rel=str((s/n).relative_to('/home/ec2-user/crypto-pencil-evidence-recovery-20261105'));raw=subprocess.check_output(['git','show',r['source_publication']+':'+rel],cwd='/home/ec2-user/crypto-pencil-evidence-recovery-20261105');assert hashlib.sha256(raw).hexdigest()==digest
out=Path(cfg['output']);c=Path(cfg['caller_control']);unit='pencil-root-cache-snapshot-renewal-20261111.service';assert not out.exists()and not c.exists();start=time.monotonic();end=start+60;assert time.time()+60<datetime.datetime.fromisoformat(r['deadline_utc'].replace('Z','+00:00')).timestamp()
def run(cmd):return subprocess.run(cmd,capture_output=True,text=True,timeout=max(.1,min(3,end-time.monotonic())))
def show():
 a=run(['systemctl','show',unit,'--property=LoadState,ActiveState,SubState,InvocationID,ControlGroup,MainPID,ExecMainStatus,Result,CPUQuotaPerSecUSec,MemoryMax,MemorySwapMax,TasksMax']);v=dict(x.split('=',1)for x in a.stdout.splitlines()if'='in x);assert a.returncode==0 or v.get('LoadState')=='not-found';return v
assert show().get('LoadState')=='not-found';c.mkdir(mode=0o700)
def save(n,v):
 with(c/n).open('x')as f:json.dump(v,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
 fd=os.open(c,os.O_DIRECTORY);os.fsync(fd);os.close(fd)
for n in ['unit.stdout','unit.stderr']:
 fd=os.open(c/n,os.O_CREAT|os.O_EXCL|os.O_WRONLY,0o600);os.fsync(fd);os.close(fd)
save('release.json',r);save('command.json',cfg);initial=None;inv=None;cg=None;terminal=None;stopped=None;errors=[];caps={};passed=False
try:
 launch=run(cfg['argv']);save('launch.json',{'returncode':launch.returncode,'stdout':launch.stdout,'stderr':launch.stderr});assert launch.returncode==0
 while time.monotonic()<min(end-8,start+5):
  initial=show()
  if initial.get('InvocationID') and initial.get('ControlGroup'):break
  time.sleep(.05)
 assert initial and initial.get('InvocationID')and initial.get('ControlGroup'),'missing owner'
 inv=initial['InvocationID'];cg=Path('/sys/fs/cgroup')/initial['ControlGroup'].lstrip('/')
 save('initial-owner.json',{'state':initial,'boot_id':Path('/proc/sys/kernel/random/boot_id').read_text().strip()})
 for n in ['cpu.max','memory.max','memory.swap.max','pids.max']:caps[n]=(cg/n).read_text().strip()
 save('native-limits.json',caps);cpu=caps['cpu.max'].split();assert int(cpu[0])==int(cpu[1]);assert caps['memory.max']=='1073741824'and caps['memory.swap.max']=='0'and caps['pids.max']=='32'
 while time.monotonic()<end-6:
  state=show();assert state.get('InvocationID')==inv,'changed invocation'
  if state.get('MainPID')=='0' and state.get('SubState') in ['exited','dead','failed']:terminal=state;break
  time.sleep(.1)
 assert terminal is not None,'snapshot terminal deadline';save('native-terminal.json',terminal)
 result=json.loads((out/'result.json').read_text());assert terminal['ExecMainStatus']=='0'and terminal['Result']=='success'and result['status']=='passed';assert h(out/'current-cache.json')==result['cache_sha256'];assert h(out/'changes.json')==result['changes_sha256'];passed=True
except BaseException as e:errors.append(repr(e))
finally:
 try:
  if inv:
   state=show();assert state.get('InvocationID')==inv,'owner changed before cleanup'
   stop=run(['sudo','-n','systemctl','stop',unit]);assert stop.returncode==0
   stopped=show();assert stopped.get('LoadState')=='not-found' or (stopped.get('InvocationID')==inv and stopped.get('MainPID')=='0'and stopped.get('ActiveState')in['inactive','failed'])
   assert not cg.exists() or all(not f.read_text().strip()for f in cg.rglob('cgroup.procs'))
  else:raise AssertionError('owner missing; custody unresolved')
 except BaseException as e:passed=False;errors.append('cleanup '+repr(e))
 elapsed=time.monotonic()-start;passed=passed and not errors and elapsed<60
 save('terminal.json',{'status':'passed'if passed else'failed','invocation_id':inv,'terminal':terminal,'stopped':stopped,'cgroup':str(cg),'cgroup_empty_or_absent':cg is not None and(not cg.exists()or all(not f.read_text().strip()for f in cg.rglob('cgroup.procs'))),'elapsed_seconds':elapsed,'charged_seconds':60,'errors':errors,'native_result_sha256':h(out/'result.json')if(out/'result.json').exists()else None})
 print(json.dumps({'passed':passed,'elapsed_seconds':elapsed,'invocation':inv,'errors':errors,'output':str(out)}))
