import ast,json,tempfile,types,subprocess
from pathlib import Path
p=Path('/home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/nonbranch-replay-continuation-v2-20261105')
source=(p/'launch.py').read_text();tree=ast.parse(source)
functions=ast.Module(body=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ('terminal_owner','phase_run')],type_ignores=[])
class Clock:
 def monotonic(self):return 0.
 def time(self):return 0.
 def sleep(self,x):pass
class Proc:
 pid=987654321
 def wait(self,timeout):return 0
 def poll(self):return 0
cases=[]
with tempfile.TemporaryDirectory(prefix='supervisor-simulation-') as d:
 base=Path(d)
 for case,contents in [('malformed','{'),('missing_status','{}'),('missing_file',None),('passed','{"status":"passed"}')]:
  w=base/case;w.mkdir();o=base/(case+'-outer');o.mkdir()
  if contents is not None:(w/'compiler-receipt.json').write_text(contents)
  states=iter([{'LoadState':'not-found'},{'InvocationID':'owned','ActiveState':'active','ControlGroup':'/unused'},{'InvocationID':'owned','ActiveState':'inactive','ControlGroup':''}])
  ns={'Path':Path,'json':json,'time':Clock(),'start':0.,'DEADLINE':1793937600,'O':o,'W':w,'subprocess':types.SimpleNamespace(Popen=lambda *a,**k:Proc(),TimeoutExpired=subprocess.TimeoutExpired),'os':types.SimpleNamespace(killpg=lambda *a:(_ for _ in ()).throw(AssertionError('unexpected kill'))),'signal':types.SimpleNamespace(SIGKILL=9),'unit_state':lambda *a:next(states)}
  exec(compile(functions,'<extracted-source-functions>','exec'),ns)
  row=ns['phase_run'](['SIMULATED'],'SIMULATED','compiler',600)
  assert row['terminal_owner_confirmed'] and row['passed']==(case=='passed')
  assert bool(row['receipt_error'])==(case!='passed')
  cases.append({'name':case,'status':'passed','receipt_error_recorded':bool(row['receipt_error'])})
  # Run the exact published-source phase loop, substituting only phase execution and durable outputs.
  outer=next(n for n in tree.body if isinstance(n,ast.Try));loop=next(n for n in outer.body if isinstance(n,ast.For));calls=[];records=[]
  def fake_phase(cmd,unit,phase,limit):
   calls.append(phase)
   return dict(row) if phase=='compiler' else {'phase':phase,'passed':True,'terminal_owner_confirmed':True}
  ns.update(results=[],durable=lambda n,obj:records.append((n,obj)),phase_run=fake_phase,P=p,a=types.SimpleNamespace(freeze_sha256='0'*64))
  exec(compile(ast.Module(body=[loop],type_ignores=[]),'<extracted-source-loop>','exec'),ns)
  assert calls==['pre','compiler','post'],calls
  cases[-1]['post_attempted']=True
 # Ownership unknown must stop before post.
 calls=[]
 def unknown(cmd,unit,phase,limit):
  calls.append(phase);return {'phase':phase,'passed':phase=='pre','terminal_owner_confirmed':phase=='pre'}
 ns.update(results=[],phase_run=unknown)
 exec(compile(ast.Module(body=[loop],type_ignores=[]),'<extracted-source-loop>','exec'),ns)
 assert calls==['pre','compiler']
 cases.append({'name':'unknown_terminal_owner_blocks_post','status':'passed'})
 assert not ns['terminal_owner']({'InvocationID':'other','ActiveState':'inactive','ControlGroup':''},'owned')
 cases.append({'name':'changed_invocation_rejected','status':'passed'})
(p/'supervisor-simulation.json').write_text(json.dumps({'schema':'simulated-source-controls/v1','status':'passed','cases':cases,'native_compiler_invocations':0,'systemd_invocations':0,'scope':'Extracted exact helper functions and phase loop with mocked services/clock; not live runtime qualification','pcc':'unverified'},indent=2)+'\n')
print(json.dumps({'cases':len(cases),'status':'passed','native_compiler_invocations':0,'systemd_invocations':0}))
