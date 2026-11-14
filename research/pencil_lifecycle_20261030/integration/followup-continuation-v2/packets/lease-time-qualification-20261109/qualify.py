#!/usr/bin/python3
"""Python3.9 parser + actual heartbeat function fixtures; no live network/lease."""
import argparse,ast,datetime,hashlib,io,json,re,signal,sys,time,types,urllib.request,urllib.error
from pathlib import Path
from unittest.mock import patch
P=Path(__file__).resolve().parent
SOURCE=P.parent/'root-library-validation-v3-20261109/durable-launcher.py'
SOURCE_SHA='1be89bfce43a31d4cdfd2fb41400a7decbfadfec0b3bb11b887099c31d654fb6'
NOW=datetime.datetime(2026,11,9,18,9,0,tzinfo=datetime.timezone.utc)
class FixedDateTime(datetime.datetime):
 @classmethod
 def now(cls,tz=None):return NOW if tz is not None else NOW.replace(tzinfo=None)
def extracted(raw,names,env):
 nodes=[n for n in ast.parse(raw).body if isinstance(n,ast.FunctionDef) and n.name in names];assert {n.name for n in nodes}==set(names)
 exec(compile(ast.Module(body=nodes,type_ignores=[]),str(SOURCE),'exec'),env);return env

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--output',type=Path,required=True);args=ap.parse_args()
 assert sys.version_info[:2]==(3,9),'qualification requiresactualPython3.9'
 raw=SOURCE.read_bytes();assert hashlib.sha256(raw).hexdigest()==SOURCE_SHA
 assert args.output.is_absolute() and not args.output.exists();args.output.mkdir(mode=0o700)
 class Stop(BaseException):pass
 def stop(*_):raise Stop('50secondqualificationdeadline')
 signal.signal(signal.SIGALRM,stop);signal.alarm(50);start=time.monotonic();rows=[];cases=[]
 parse=extracted(raw,['parse_rfc3339'],{'re':re,'datetime':datetime})['parse_rfc3339']
 def reject(fn):
  try:fn()
  except (ValueError,AssertionError,RuntimeError,OverflowError):return
  raise AssertionError('invalidinputaccepted')
 def equal(value,expected):
  actual=parse(value);assert actual==expected and actual.tzinfo==datetime.timezone.utc,(value,actual,expected)
 for n in range(10):
  f='123456789'[:n];value='2026-11-09T18:10:34'+('.'+f if n else '')+'Z';expected=datetime.datetime(2026,11,9,18,10,34,int((f+'000000')[:6]),tzinfo=datetime.timezone.utc)
  cases.append(('fraction_'+str(n),lambda v=value,e=expected:equal(v,e)))
 offsets=[('Z',0),('+00:00',0),('+05:30',330),('-04:00',-240),('+23:59',1439),('-23:59',-1439),('+00:01',1)]
 for zone,mins in offsets:
  value='2026-11-09T18:10:34.65128'+zone;expected=datetime.datetime(2026,11,9,18,10,34,651280,tzinfo=datetime.timezone.utc)-datetime.timedelta(minutes=mins)
  cases.append(('offset_'+zone,lambda v=value,e=expected:equal(v,e)))
 invalid=[None,1,True,'','2026-11-09T18:10:34','2026-11-09t18:10:34Z','2026-11-09T18:10:34z','2026-11-09 18:10:34Z','2026-11-09T18:10:34.Z','2026-11-09T18:10:34.1234567890Z','2026-11-09T18:10:34+24:00','2026-11-09T18:10:34+00:60','2026-11-09T18:10:34-00:00','2026-02-30T18:10:34Z','2026-11-09T24:10:34Z','2026-11-09T18:60:34Z','2026-11-09T18:10:60Z','2026-11-09T18:10:34+00','2026-11-09T18:10:34UTC','2026-11-09T18:10:34Z\n']
 for i,v in enumerate(invalid):cases.append(('reject_'+str(i),lambda v=v:reject(lambda:parse(v))))
 for f in ['9999999','99999999','999999999']:
  cases.append(('floor_'+str(len(f)),lambda f=f:equal('2026-11-09T23:59:59.'+f+'Z',datetime.datetime(2026,11,9,23,59,59,999999,tzinfo=datetime.timezone.utc))))
 lease={'task_id':'fixture-task','attempt_id':'fixture-attempt','lease_id':'fixture-lease','fencing_epoch':3}
 def hb(expiry,expected=True,change=None,previous=None,expected_requests=1):
  response={'task_id':lease['task_id'],'attempt_id':lease['attempt_id'],'id':lease['lease_id'],'fencing_epoch':3,'expires_at':expiry}
  if change:response.update(change)
  env=extracted(raw,['parse_rfc3339','heartbeat'],{'re':re,'datetime':types.SimpleNamespace(datetime=FixedDateTime,timezone=datetime.timezone),'time':time,'urllib':urllib,'json':json,'F':{'lease':lease},'CREDENTIALS':{'endpoint':'http://fixture.invalid','token':'fixture-token'},'HB_NEXT':0,'HB_EXPIRY':previous,'R':{},'checkpoint':lambda _:None})
  class Response(io.StringIO):
   status=200
  with patch.object(urllib.request,'urlopen',return_value=Response(json.dumps(response))) as mocked:
   if expected:
    env['heartbeat'](True);assert env['HB_EXPIRY']==parse(expiry) and len(env['R']['heartbeats'])==1
   else:
    reject(lambda:env['heartbeat'](True));assert env['HB_EXPIRY']==previous
   assert mocked.call_count==expected_requests
 for n in range(10):
  f='651281234'[:n];v='2026-11-09T18:10:34'+('.'+f if n else '')+'Z';cases.append(('heartbeat_fraction_'+str(n),lambda v=v:hb(v)))
 cases.extend([
  ('heartbeat_observed_five_digits_offset',lambda:hb('2026-11-09T18:10:34.65128+00:00')),
  ('heartbeat_exact30_rejected',lambda:hb('2026-11-09T18:09:30Z',False)),
  ('heartbeat_floor_below_microsecond_rejected',lambda:hb('2026-11-09T18:09:30.0000009Z',False)),
  ('heartbeat_over30_accepted',lambda:hb('2026-11-09T18:09:30.000001Z')),
  ('heartbeat_prior_under15_rejected_without_request',lambda:hb('2026-11-09T18:10:34Z',False,previous=NOW+datetime.timedelta(seconds=14,microseconds=999999),expected_requests=0)),
  ('heartbeat_prior_exact15_accepted',lambda:hb('2026-11-09T18:10:34Z',previous=NOW+datetime.timedelta(seconds=15))),
  ('heartbeat_malformed_expiry_rejected',lambda:hb('2026-11-09T18:10:34.badZ',False))])
 for k in ['task_id','attempt_id','id','fencing_epoch']:
  cases.append(('heartbeat_wrong_'+k,lambda k=k:hb('2026-11-09T18:10:34Z',False,{k:'wrong'})))
 assert len(cases)==61
 for name,fn in cases:
  try:fn();rows.append({'case':name,'passed':True})
  except Exception as e:rows.append({'case':name,'passed':False,'error':repr(e)})
 signal.alarm(0);r={'schema':'lease-time.synthetic-qualification/v1','source_sha256':SOURCE_SHA,'python':sys.version,'cases':rows,'passed':len(rows)==61 and all(x['passed'] for x in rows),'elapsed_seconds':time.monotonic()-start,'network_used':False,'live_lease_mutated':False,'native_unit_started':False,'compiler_started':False}
 (args.output/'result.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r));return 0 if r['passed'] else 1
if __name__=='__main__':raise SystemExit(main())
