"""Prospective fixed software fixtures; never imports launcher top-level or starts payloads."""
import ast,datetime,hashlib,io,json,math,signal,sys,time
from pathlib import Path
P=Path(__file__).resolve().parent
START=time.monotonic()
class WorkDeadline(BaseException):pass
def alarm(*_):raise WorkDeadline('qualification deadline')
signal.signal(signal.SIGALRM,alarm);signal.alarm(50)
assert sys.version_info[:2]==(3,9),'require existing Python3.9'
PIN=json.loads((P/'qualification-pins.json').read_text())
def functions(name,names):
 raw=(P/name).read_bytes();assert hashlib.sha256(raw).hexdigest()==PIN[name]
 tree=ast.parse(raw);nodes=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in names]
 assert {n.name for n in nodes}==set(names)
 return compile(ast.Module(body=nodes,type_ignores=[]),name,'exec')
WC=functions('durable-worker.py',['retain_permit_anomaly','read_permit_snapshot','check_permit'])
SC=functions('durable-launcher.py',['show','permit'])
class Clock:
 def __init__(self):self.wall=100.;self.mono=0.
 def time(self):return self.wall
 def monotonic(self):return self.mono
class Reader:
 def __init__(self,rows,clock,delay=0):self.rows=rows;self.clock=clock;self.calls=0;self.delay=delay
 def __truediv__(self,_):return self
 def open(self,*_):
  if self.calls==1:self.clock.mono+=self.delay
  row=self.rows[self.calls];self.calls+=1
  return io.BytesIO(row if isinstance(row,bytes) else json.dumps(row).encode())
def permit(expiry,identity='owner'):return {'invocation_id':identity,'lease':{'id':'lease'},'expires_unix':expiry}
def worker(rows,delay=0):
 clock=Clock();reader=Reader(rows,clock,delay)
 e={'time':clock,'Path':lambda _:reader,'F':{'durable_run':'unused','lease':{'id':'lease'}},'R':{'invocation_id':'owner'},'json':json,'math':math,'hashlib':hashlib,'INTERRUPTED':False,'END':1000}
 exec(WC,e);return e,reader
cases=[]
def test(name,fn):
 fn();cases.append({'case':name,'passed':True})
def accepts(rows,reads=1,delay=0):
 e,r=worker(rows,delay);e['check_permit']();assert r.calls==reads
 if reads==2:assert len(e['R']['permit_anomalies'][-1]['snapshots'])==2
 return e
def rejects(rows,reads,contains,delay=0,changes=None):
 e,r=worker(rows,delay);e.update(changes or {})
 try:e['check_permit']()
 except (AssertionError,ValueError,KeyError,TypeError) as err:
  assert contains in str(err),(contains,str(err));assert r.calls==reads;assert e['R'].get('permit_anomalies');return
 raise AssertionError('unexpected permit acceptance')
test('fresh_single_read',lambda:accepts([permit(110)]))
test('expired_then_fresh',lambda:accepts([permit(99),permit(110)],2))
test('both_expired_fail',lambda:rejects([permit(99),permit(99)],2,'expired'))
test('identity_first_no_retry',lambda:rejects([permit(110,'other')],1,'identity'))
test('identity_second_fail',lambda:rejects([permit(99),permit(110,'other')],2,'identity'))
test('parse_first_no_retry',lambda:rejects([b'{'],1,'Expecting'))
test('parse_second_fail',lambda:rejects([permit(99),b'{'],2,'Expecting'))
test('oversize_no_retry',lambda:rejects([b' '*4097],1,'size'))
test('nonfinite_expiry_no_retry',lambda:rejects([permit(float('nan'))],1,'format'))
test('boolean_expiry_no_retry',lambda:rejects([permit(True)],1,'format'))
test('reread_bound_exceeded',lambda:rejects([permit(99),permit(110)],2,'bound',.251))
test('reread_bound_edge',lambda:accepts([permit(99),permit(110)],2,.25))
test('signal_during_reread',lambda:rejects([permit(99),permit(110)],2,'deadline/signal',changes={'INTERRUPTED':True}))
test('deadline_during_reread',lambda:rejects([permit(99),permit(110)],2,'deadline/signal',changes={'END':39}))
class DateTime(datetime.datetime):
 @classmethod
 def now(cls,tz=None):return datetime.datetime.fromtimestamp(100,tz)
class DT:datetime=DateTime;timezone=datetime.timezone
def supervisor(cleaning=False,interrupt=False,state='active',owner='owner'):
 clock=Clock();writes=[];order=[]
 class Dest:
  def __truediv__(self,_):return self
 e={'R':{'unit_launch_attempted':True,'invocation_id':'owner','cgroup':'/same','observations':[]},'CLEANING':cleaning,'INTERRUPTED':interrupt,'UNIT':'unit','KEYS':[],'time':clock,'datetime':DT,'HB_EXPIRY':datetime.datetime.fromtimestamp(200,datetime.timezone.utc),'DEADLINE':datetime.datetime.fromtimestamp(1000,datetime.timezone.utc),'F':{'lease':{'id':'lease'},'limits':{'supervisor_permit_seconds':10}},'D':Dest()}
 e['call']=lambda *a,**k:'InvocationID='+owner+'\nControlGroup=/same\nActiveState='+state+'\nSubState=running\n'
 def write(path,value):writes.append(value);order.append('permit')
 e['atomic_json']=write;e['checkpoint']=lambda _:order.append('checkpoint');exec(SC,e)
 return e,writes,order
def active_order():
 e,w,o=supervisor();e['show']();assert o==['permit','checkpoint'];assert w[0]['expires_unix']==110;assert w[0]['sequence']==1
 e['HB_EXPIRY']=datetime.datetime.fromtimestamp(120,datetime.timezone.utc);e['show']();assert w[-1]['expires_unix']==105 and w[-1]['sequence']==2

def no_permit(**kwargs):
 e,w,o=supervisor(**kwargs);e['show']();assert not w and o==['checkpoint']
def wrong_owner():
 e,w,o=supervisor(owner='other')
 try:e['show']()
 except AssertionError:assert not w and not o;return
 raise AssertionError('owner changed accepted')
test('owner_check_then_permit_before_checkpoint_and_lease_margin',active_order)
test('cleaning_no_permit',lambda:no_permit(cleaning=True))
test('interrupted_no_permit',lambda:no_permit(interrupt=True))
test('terminal_no_permit',lambda:no_permit(state='failed'))
test('changed_owner_no_permit',wrong_owner)
def heartbeat_order():
 tree=ast.parse((P/'durable-launcher.py').read_text());f=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='heartbeat')
 assert not any(isinstance(n,ast.Call) and isinstance(n.func,ast.Name) and n.func.id=='checkpoint' for n in ast.walk(f))
test('heartbeat_defers_cumulative_checkpoint_source_check',heartbeat_order)
result={'schema':'root-v4-permit-software-qualification/v1','passed':True,'cases':cases,'elapsed_seconds':time.monotonic()-START,'python':sys.version,'source_pins':PIN,'native_payload_started':False,'network_used':False,'real_conductor_used':False}
assert len(sys.argv)==3 and sys.argv[1]=='--output'
out=Path(sys.argv[2]);out.mkdir(mode=0o700,parents=False,exist_ok=False);(out/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'passed':True,'cases':len(cases)}))
