#!/usr/bin/python3
"""One native redirected-log ownership fixture; no compiler or research payload."""
import argparse,ast,hashlib,importlib.util,json,os,stat,subprocess,sys,time
from pathlib import Path
P=Path(__file__).resolve().parent
UNIT='pencil-root-log-qualification-t1971-20261109.service'
OUT=Path('/home/ec2-user/pencil-validation-receipts/t1971-root-log-qualification-20261109')
IO_SHA='2034c58e36bb3e3d6727dbd6c640ff437c695f472810e6ae596e6ac339dc854c'
IO=P.parent/'root-library-validation-v2-20261109/durable_io.py'
LAUNCHER=P.parent/'root-library-validation-v2-20261109/durable-launcher.py'
LAUNCHER_SHA='da57c82264e5fab555526b772221f0933fb9d114d2cf2f2852f2189a7eef4aaf'
def extract(names,env):
 raw=LAUNCHER.read_bytes();assert hashlib.sha256(raw).hexdigest()==LAUNCHER_SHA
 nodes=[n for n in ast.parse(raw).body if isinstance(n,ast.FunctionDef) and n.name in names];assert {n.name for n in nodes}==set(names)
 exec(compile(ast.Module(body=nodes,type_ignores=[]),str(LAUNCHER),'exec'),env);return env
def helper():
 assert hashlib.sha256(IO.read_bytes()).hexdigest()==IO_SHA
 s=importlib.util.spec_from_file_location('fixture_io',IO);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def worker():
 io=helper();assert os.getuid()==1000 and os.getgid()==1000
 print('NATIVE_LOG_STDOUT_SENTINEL',flush=True);print('NATIVE_LOG_STDERR_SENTINEL',file=sys.stderr,flush=True)
 time.sleep(3)
 ownership={}
 for name in ['unit.stdout','unit.stderr']:
  p=OUT/name;s=p.lstat();assert stat.S_ISREG(s.st_mode) and s.st_uid==1000 and s.st_gid==1000 and stat.S_IMODE(s.st_mode)==0o600
  ownership[name]={'uid':s.st_uid,'gid':s.st_gid,'mode':stat.S_IMODE(s.st_mode)}
 prefixes=io.flush_logs(OUT)
 assert b'NATIVE_LOG_STDOUT_SENTINEL' in (OUT/'unit.stdout').read_bytes()
 assert b'NATIVE_LOG_STDERR_SENTINEL' in (OUT/'unit.stderr').read_bytes()
 io.atomic_json(OUT/'worker-result.json',{'passed':True,'uid':os.getuid(),'invocation':os.environ['INVOCATION_ID'],'ownership':ownership,'prefixes':prefixes})
 return 0

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--worker',action='store_true');a=ap.parse_args()
 if a.worker:return worker()
 start=time.monotonic();end=start+50;io=helper();assert os.getuid()==1000 and not OUT.exists()
 OUT.mkdir(mode=0o700);result={'passed':False,'unit':UNIT,'errors':[],'launch_attempted':False,'compiler_invoked':False};owned=None;cg=None
 def save():io.atomic_json(OUT/'result.json',result)
 def cmd(args):
  remaining=end-time.monotonic();assert remaining>0
  r=subprocess.run(args,capture_output=True,text=True,timeout=min(4,remaining));return r
 def show():
  r=cmd(['/usr/bin/systemctl','show',UNIT,'--property=LoadState,ActiveState,SubState,MainPID,InvocationID,ControlGroup,Result,ExecMainStatus']);d=dict(x.split('=',1) for x in r.stdout.splitlines() if '=' in x);assert r.returncode==0 or r.returncode==1 and d.get('LoadState')=='not-found';return d
 def empty():
  if not cg:return False
  p=Path('/sys/fs/cgroup')/cg.lstrip('/')
  return not p.exists() or (not (p/'cgroup.procs').read_text().strip() and 'populated 0' in (p/'cgroup.events').read_text())
 try:
  assert show()['LoadState']=='not-found'
  extract(['precreate_unit_logs'],{'Path':Path,'os':os})['precreate_unit_logs'](OUT)
  props={'User':'1000','Group':'1000','Type':'exec','RemainAfterExit':'yes','Restart':'no','CPUQuota':'100%','MemoryMax':str(128*1024**2),'MemorySwapMax':'0','TasksMax':'32','OOMPolicy':'stop','KillMode':'control-group','TimeoutStopSec':'5s','RuntimeMaxSec':'35s','NoNewPrivileges':'yes','PrivateTmp':'no','PrivateNetwork':'yes','ProtectSystem':'strict','ProtectHome':'read-only','ReadWritePaths':str(OUT),'WorkingDirectory':str(P),'UMask':'0077','StandardOutput':'append:'+str(OUT/'unit.stdout'),'StandardError':'append:'+str(OUT/'unit.stderr')}
  result['properties']=props;result['launch_attempted']=True;save()
  argv=['/usr/bin/sudo','-n','/usr/bin/systemd-run','--no-block','--unit='+UNIT]+['--property='+k+'='+v for k,v in props.items()]+['/usr/bin/python3','-B',str(P/'qualify.py'),'--worker']
  r=cmd(argv);result['launch_exit']=r.returncode;result['launch_stderr']=r.stderr;assert r.returncode==0
  captured={'unit_launch_attempted':True,'observations':[]}
  def capture_call(argv,record_checkpoint=True):
   assert record_checkpoint is False
   r=cmd(argv);assert r.returncode==0;return r.stdout
  def failed_checkpoint(_):
   if captured.get('invocation_id') and captured.get('cgroup'):raise OSError('injected after ownership capture')
  capture=extract(['show'],{'call':capture_call,'UNIT':UNIT,'KEYS':['LoadState','ActiveState','SubState','MainPID','InvocationID','ControlGroup','Result','ExecMainStatus'],'R':captured,'checkpoint':failed_checkpoint})
  capture_end=min(end-10,time.monotonic()+5)
  while time.monotonic()<capture_end:
   try:d=capture['show']()
   except OSError as e:
    assert str(e)=='injected after ownership capture'
    result['ownership_survived_checkpoint_failure']=True;d=captured['observations'][-1]
   if captured.get('invocation_id') and captured.get('cgroup'):
    owned=captured['invocation_id'];cg=captured['cgroup'];result['captured_invocation']=owned;result['captured_cgroup']=cg;result['initial_state']=d;save();break
   time.sleep(.1)
  assert owned and cg and result.get('ownership_survived_checkpoint_failure'),'missing initial ownership'
  c=Path('/sys/fs/cgroup')/cg.lstrip('/');controls={n:(c/n).read_text().strip() for n in ['cpu.max','memory.max','memory.swap.max','pids.max']};quota,period=controls['cpu.max'].split();assert int(quota)<=int(period) and controls['memory.max']==str(128*1024**2) and controls['memory.swap.max']=='0' and controls['pids.max']=='32';result['native_controls']=controls
  while time.monotonic()<end-10:
   d=show();assert d.get('InvocationID')==owned,'ownership changed'
   if d.get('SubState')=='exited' or d.get('ActiveState')=='failed':break
   time.sleep(.1)
  assert d.get('SubState')=='exited' and d.get('Result')=='success' and d.get('ExecMainStatus')=='0'
  w=json.loads((OUT/'worker-result.json').read_text());assert w['passed'] and w['invocation']==owned
  result['worker']=w;result['supervisor_prefixes']=io.flush_logs(OUT);result['payload_passed']=True
 except BaseException as error:result['errors'].append(repr(error))
 finally:
  if result['launch_attempted']:
   try:
    d=show();assert owned and cg and d.get('InvocationID')==owned,'unresolved owner; no name-only cleanup'
    r=cmd(['/usr/bin/sudo','-n','/usr/bin/systemctl','stop','--no-block',UNIT]);assert r.returncode==0
    while time.monotonic()<end:
     d=show();assert not d.get('InvocationID') or d['InvocationID']==owned
     if d.get('ActiveState') in ('inactive','failed') and d.get('MainPID','0')=='0' and empty():result['owned_cleanup']=True;result['terminal_state']=d;break
     time.sleep(.1)
    assert result.get('owned_cleanup')
   except BaseException as error:result['errors'].append('cleanup:'+repr(error))
  result['elapsed_seconds']=time.monotonic()-start;result['passed']=bool(result.get('payload_passed') and result.get('owned_cleanup') and not result['errors'] and result['elapsed_seconds']<50);save()
 print(json.dumps(result));return 0 if result['passed'] else 1
if __name__=='__main__':raise SystemExit(main())
