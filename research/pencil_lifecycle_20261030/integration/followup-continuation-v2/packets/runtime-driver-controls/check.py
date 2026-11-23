import importlib.util,json,os,signal,sys,tempfile,threading,time,hashlib
from pathlib import Path
from datetime import datetime,timezone,timedelta
P=Path(__file__).resolve().parent
source=P.parent/'runtime-replacement-assembler/assemble.py'
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
bound={str(p):h(p) for p in [source,P/'check.py',P/'PLAN.md']}
private=Path(tempfile.mkdtemp(prefix='runtime-driver-controls-',dir='/var/tmp'))
(P/'location.json').write_text(json.dumps({'private_directory':str(private),'sources':bound},indent=2)+'\n')
spec=importlib.util.spec_from_file_location('bound_assembler',source);a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
future=(datetime.now(timezone.utc)+timedelta(minutes=5)).isoformat()
cases=[
 ('success',"import sys;print('ok');print('diagnostic',file=sys.stderr)",2,None,0),
 ('nonzero','import sys;sys.exit(7)',2,None,7),
 ('timeout','import time;time.sleep(5)',.2,'client timeout',-signal.SIGTERM),
 ('kill_escalation',"import signal,time;signal.signal(signal.SIGTERM,signal.SIG_IGN);print('READY',flush=True);time.sleep(5)",.2,'client timeout',-signal.SIGKILL),
 ('output_limit',"import sys;sys.stdout.write('x'*(17*1024*1024));sys.stdout.flush()",3,'output cap',None),
 ('pending_during','import time;time.sleep(5)',2,'interrupted',-signal.SIGTERM),
 ('pending_before','raise Exception("must not execute")',2,'interrupted',None),
 ('expired_deadline','raise Exception("must not execute")',2,'deadline reserve',None)]
rows=[]
for name,program,cap,expected_error,expected_exit in cases:
 out=private/name;out.mkdir();driver=a.Driver(out,time.monotonic(),'2000-01-01T00:00:00Z' if name=='expired_deadline' else future)
 driver.prefix=[sys.executable,'-c'];timer=None
 if name=='pending_before':driver.pending_signal=signal.SIGTERM
 if name=='pending_during':timer=threading.Timer(.2,lambda:setattr(driver,'pending_signal',signal.SIGTERM));timer.start()
 caught=None;ret=None
 try:ret=driver.call(name,[program],cap)
 except BaseException as e:caught=type(e).__name__+': '+str(e)
 finally:
  if timer:timer.cancel();timer.join()
 rec=driver.calls[-1] if driver.calls else None
 pid=rec.get('pid') if rec else None;absent=(not Path('/proc',str(pid)).exists()) if pid else None
 passed=(caught is None if expected_error is None else caught is not None and expected_error in caught)
 if expected_exit is not None:passed=passed and rec is not None and rec['client_terminal_exit']==expected_exit
 if name in ['pending_before','expired_deadline']:passed=passed and not driver.calls
 else:passed=passed and rec is not None and rec['client_unknown'] is False and absent is True
 if name=='success':passed=passed and ret[1]==b'ok\n' and ret[2]==b'diagnostic\n'
 if name=='kill_escalation':passed=passed and (out/'001-kill_escalation/stdout').read_bytes()==b'READY\n'
 row={'case':name,'passed':bool(passed),'error':caught,'client_receipt':rec,'pid_absent_after_call':absent,'child_started':pid is not None}
 rows.append(row);(private/(name+'.json')).write_text(json.dumps(row,indent=2)+'\n');print(json.dumps(row),flush=True)
unchanged=all(h(Path(p))==d for p,d in bound.items())
summary={'schema':'runtime.driver.local-controls/v1','controls':rows,'passed':all(r['passed'] for r in rows),'source_hashes':bound,'sources_unchanged':unchanged,'private_directory':str(private),'docker_calls':0,'sudo_calls':0,'scientific_execution':False}
(P/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
raise SystemExit(0 if summary['passed'] and unchanged else 1)
