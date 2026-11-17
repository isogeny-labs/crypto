"""Exclusive bounded process capture. This wrapper does not grant admission."""
from pathlib import Path
import argparse,datetime,hashlib,json,os,signal,subprocess,time

def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def canonical_protocol(p):return hashlib.sha256(json.dumps({k:v for k,v in p.items()if k!='freeze'},sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode()).hexdigest()
def capture(argv,out,timeout,bindings):
 if not 0<timeout<=120:raise ValueError('deadline outside envelope')
 out=Path(out);out.mkdir(exist_ok=False)
 record={'schema':'monomial-process-receipt/v1','argv':argv,'started_at':now(),'timeout_seconds':timeout,**bindings};proc=None;started=time.monotonic()
 # A prelaunch marker makes an interrupted attempt visible; never overwrite/resume it.
 (out/'attempt.json').write_text(json.dumps(record,indent=2)+'\n')
 try:
  with(out/'rows.jsonl').open('xb')as stdout,(out/'stderr.log').open('xb')as stderr:
   try:
    proc=subprocess.Popen(argv,stdout=stdout,stderr=stderr,start_new_session=True)
    try:record['exit_code']=proc.wait(timeout=timeout);record['timed_out']=False
    except subprocess.TimeoutExpired:
     os.killpg(proc.pid,signal.SIGKILL);record['exit_code']=proc.wait();record['timed_out']=True
   finally:
    if proc is not None:
     try:os.killpg(proc.pid,signal.SIGKILL)
     except ProcessLookupError:pass
 except OSError as e:record.update(exit_code=None,timed_out=False,spawn_error=type(e).__name__+': '+str(e))
 record.update(finished_at=now(),elapsed_seconds=time.monotonic()-started,raw_sha256=sha(out/'rows.jsonl'),raw_bytes=(out/'rows.jsonl').stat().st_size,stderr_sha256=sha(out/'stderr.log'),stderr_bytes=(out/'stderr.log').stat().st_size)
 with(out/'producer-receipt.json').open('x')as f:json.dump(record,f,indent=2);f.write('\n')
 return record

def main():
 p=argparse.ArgumentParser();p.add_argument('--runner',type=Path,required=True);p.add_argument('--protocol',type=Path,required=True);p.add_argument('--admission',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args()
 protocol=json.loads(a.protocol.read_text());admission=json.loads(a.admission.read_text());digest=canonical_protocol(protocol)
 if protocol['freeze']['status']!='frozen' or protocol['freeze']['protocol_sha256']!=digest:raise ValueError('unfrozen or changed protocol')
 if admission['status']!='admitted' or admission['protocol_sha256']!=digest:raise ValueError('matching admission receipt required')
 if datetime.datetime.now(datetime.timezone.utc)>=datetime.datetime.fromisoformat(admission['deadline_utc'].replace('Z','+00:00')):raise ValueError('expired admission')
 binary=sha(a.runner);expected={x['instrument']['binary_sha256']for x in protocol['arms']}
 if expected!={binary} or admission['binary_sha256']!=binary:raise ValueError('binary mismatch')
 if admission['max_attempts']!=1 or admission['timeout_seconds']!=120:raise ValueError('wrong finite envelope')
 # Resource/budget enforcement belongs to the admitted EKS Job and operator receipt.
 remaining=(datetime.datetime.fromisoformat(admission['deadline_utc'].replace('Z','+00:00'))-datetime.datetime.now(datetime.timezone.utc)).total_seconds()
 record=capture([str(a.runner.resolve()),'--run-admitted'],a.out,min(120,remaining),{'protocol_sha256':digest,'protocol_file_sha256':sha(a.protocol),'binary_sha256':binary,'admission_sha256':sha(a.admission),'source_commit':admission['source_commit']})
 raise SystemExit(0 if record['exit_code']==0 and not record['timed_out']else 1)
if __name__=='__main__':main()
