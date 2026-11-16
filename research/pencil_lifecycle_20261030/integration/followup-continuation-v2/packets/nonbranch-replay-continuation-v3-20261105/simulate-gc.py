from pathlib import Path
import ast,json,types,tempfile,hashlib,subprocess
P=Path(__file__).resolve().parent
tree=ast.parse((P/'launch.py').read_text());body=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ('manager_terminal','terminal_owner','phase_run')]
BOOT='7643b0496f9d444aa34577d07d463277';unit='pencil-nonbranch-t1651-v3-pre.service';inv='owned';absent={'LoadState':'not-found','InvocationID':'','ActiveState':'inactive','ControlGroup':''}
def records():
 a={'_PID':'1','_UID':'0','_BOOT_ID':BOOT,'UNIT':unit,'INVOCATION_ID':inv,'_SYSTEMD_UNIT':'init.scope','MESSAGE_ID':'39f53479d3a045ac8e11786248231fbf','JOB_TYPE':'start','JOB_RESULT':'done','__MONOTONIC_TIMESTAMP':'10'}
 b=dict(a,MESSAGE_ID='7ad2d189f7e94e70a38c781354912448',__MONOTONIC_TIMESTAMP='20');return [a,b]
class Clock:
 def monotonic(self):return 0.
 def time(self):return 0.
 def sleep(self,x):pass
class Proc:
 pid=987654321
 def poll(self):return 0
 def wait(self,timeout):return 0
cases=[]
for name in ['valid_gc','wrong_uid','wrong_pid','wrong_boot','wrong_unit','wrong_invocation','missing_start','missing_success','reversed_order','live_replacement','cgroup_remains']:
 rows=records();state=dict(absent);exists=False
 if name.startswith('wrong_'):
  key={'wrong_uid':'_UID','wrong_pid':'_PID','wrong_boot':'_BOOT_ID','wrong_unit':'UNIT','wrong_invocation':'INVOCATION_ID'}[name]
  for r in rows:r[key]='wrong'
 if name=='missing_start':rows=rows[1:]
 if name=='missing_success':rows=rows[:1]
 if name=='reversed_order':rows[1]['__MONOTONIC_TIMESTAMP']='5'
 if name=='live_replacement':state.update(LoadState='loaded',InvocationID='replacement',ActiveState='active')
 if name=='cgroup_remains':exists=True
 class FakePath:
  def __init__(self,*a):pass
  def exists(self):return exists
 saved=[]
 ns={'Path':FakePath,'time':Clock(),'json':json,'BOOT':BOOT,'unit_state':lambda *a:state,'subprocess':types.SimpleNamespace(run=lambda *a,**k:types.SimpleNamespace(returncode=0,stdout='\n'.join(json.dumps(r) for r in rows))),'durable':lambda name,obj:saved.append(obj)}
 exec(compile(ast.Module(body=body,type_ignores=[]),'<exact-source-functions>','exec'),ns)
 result=ns['manager_terminal'](unit,inv,100)
 assert result==(name=='valid_gc'),name
 assert bool(saved)==result
 cases.append({'name':name,'status':'passed'})
# Actual phase helper sees captured identity, then a collected unit, and reaches passed through journal proof.
with tempfile.TemporaryDirectory(prefix='norm-gc-simulation-') as tmp:
 w=Path(tmp);(w/'pre-receipt.json').write_text('{"status":"passed"}')
 states=iter([dict(absent),{'LoadState':'loaded','InvocationID':inv,'ActiveState':'active','ControlGroup':'/unused'},dict(absent),dict(absent)])
 ns={'Path':Path,'time':Clock(),'json':json,'BOOT':BOOT,'unit_state':lambda *a:next(states),'subprocess':types.SimpleNamespace(Popen=lambda *a,**k:Proc(),TimeoutExpired=subprocess.TimeoutExpired,run=lambda *a,**k:types.SimpleNamespace(returncode=0,stdout='\n'.join(json.dumps(r) for r in records()))),'durable':lambda *a:None,'O':w,'W':w,'start':0.,'DEADLINE':1793937600,'os':types.SimpleNamespace(killpg=lambda *a:(_ for _ in ()).throw(AssertionError('unexpected kill'))),'signal':types.SimpleNamespace(SIGKILL=9)}
 exec(compile(ast.Module(body=body,type_ignores=[]),'<exact-source-functions>','exec'),ns)
 row=ns['phase_run'](['SIMULATED'],unit,'pre',900)
 assert row['passed'] and row['terminal_owner_confirmed'],row
 cases.append({'name':'phase_gc_success_path','status':'passed'})
(P/'gc-simulation.json').write_text(json.dumps({'schema':'simulated-source-controls/v1','status':'passed','launcher_sha256':hashlib.sha256((P/'launch.py').read_bytes()).hexdigest(),'cases':cases,'compiler_invocations':0,'systemd_invocations':0,'journal_queries':0,'scope':'Exact source helpers with mocked manager records, services and clock; not live runtime qualification','pcc':'unverified'},indent=2)+'\n')
print('passed',len(cases),'mock controls; no native calls')
