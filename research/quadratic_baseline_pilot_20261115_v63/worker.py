from pathlib import Path
import hashlib,json,os,resource,signal,subprocess,sys,time
from research.quadratic_product_lift_20261115_v63.encoder import encode,extend
from research.quadratic_product_lift_20261115_v63.worker import literal
P=Path(__file__).resolve().parent;START=time.monotonic()
def invoke(command,env,label):
 def limits():
  resource.setrlimit(resource.RLIMIT_AS,(3*1024**3,3*1024**3));resource.setrlimit(resource.RLIMIT_FSIZE,(64*1024**2,64*1024**2))
 begin=time.monotonic();timedout=False
 with(P/(label+'.stdout')).open('xb')as out,(P/(label+'.stderr')).open('xb')as err:
  p=subprocess.Popen(command,stdout=out,stderr=err,env=env,start_new_session=True,preexec_fn=limits)
  try:
   while True:
    pid,status,usage=os.wait4(p.pid,os.WNOHANG)
    if pid:break
    if time.monotonic()-begin>=9:
     timedout=True;os.killpg(p.pid,signal.SIGKILL);pid,status,usage=os.wait4(p.pid,0);break
    time.sleep(.01)
   p.returncode=os.waitstatus_to_exitcode(status)
  finally:
   try:os.killpg(p.pid,signal.SIGKILL)
   except ProcessLookupError:pass
 return {'command':command,'stdout':(P/(label+'.stdout')).read_text(errors='replace'),'stderr':(P/(label+'.stderr')).read_text(errors='replace'),'exit_code':p.returncode,'outer_timeout':timedout,'wall_seconds':time.monotonic()-begin,'wait4_peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime}

def main():
 out={'records':[],'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text());bindings=json.loads(Path(data['bindings']).read_text())
  for b in bindings:
   for k in ['original','mirror']:assert hashlib.sha256(Path(b[k]).read_bytes()).hexdigest()==b['sha256']
  out['bindings_verified']=len(bindings);desc=json.loads(Path(data['descriptor']).read_text());n=desc['field_degree'];d=desc['d'];mod=desc['modulus'];z=desc['z'];B=desc['B'];begin=time.monotonic();system=encode(n,mod,d,z,B);payload=json.dumps(system,separators=(',',':')).encode();out['charged_encoding_seconds']=time.monotonic()-begin
  frozen=Path(data['input']).read_bytes();assert payload==frozen;sha=hashlib.sha256(frozen).hexdigest();known=json.loads(Path(data['receipt']).read_text())['out'];assert sha==known['input_sha256'];expected={int(v,16)for v in known['extended_root_hex']};lowmask=(1<<(3*d+n))-1
  env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',ROOTFINDER_INSTANCE_SHA256=sha,TMPDIR=str(P/'scratch'))
  for k in list(env):
   if k.startswith(('KIC_F4_','KIC_GF2_'))or k in ['KIC_LINEAR_ELIM','SOLVER_SPLIT_RULE','IC_GROEBNER_CACHE_URL','F4_F2_MAX_ROWS','F4_F2_MAX_COLS']:env.pop(k,None)
  env['LD_LIBRARY_PATH']=str(Path(data['library_dir']).resolve());(P/'scratch').mkdir()
  for engine in data['engines']:
   command=[v.replace('{input}',data['input']).replace('{sha}',sha)for v in engine['command']];r=invoke(command,env,engine['label']);parsed=None
   try:parsed=json.loads(r['stdout'])
   except json.JSONDecodeError:pass
   complete=False;roots=set()
   if parsed:
    if parsed.get('schema')=='scientific_result_v1':e=parsed.get('evidence',{});complete=e.get('all_original_roots_complete')is True;rh=e.get('roots',[])
    else:complete=parsed.get('complete')is True;rh=parsed.get('roots_hex',[])
    roots={int(v,16)for v in rh}
    for full in roots:
     original=full&lowmask;assert extend(original,n,d)==full;assert literal(original,n,mod,d,z,B)==0
    assert roots<=expected
    if complete:assert roots==expected and not r['outer_timeout']
   r.update(label=engine['label'],scope=engine['scope'],parsed=parsed,complete_matching_roots=complete,verified_original_root_lower_bound=len(roots),qualified_speedup=None);raw=json.dumps(r,sort_keys=True,separators=(',',':')).encode()
   with(P/(engine['label']+'.json')).open('xb')as f:f.write(raw)
   out['records'].append({k:r[k]for k in ['label','exit_code','outer_timeout','wall_seconds','wait4_peak_rss_kib','complete_matching_roots','verified_original_root_lower_bound']});out['records'][-1].update(status=parsed.get('status')if parsed else'no_JSON',reason=parsed.get('reason',parsed.get('evidence',{}).get('termination_reason'))if parsed else None,raw_sha256=hashlib.sha256(raw).hexdigest());assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Changed bijective quadratic-encoding baseline diagnostics with charged encoding and per-child wait4 memory.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Analyze degree-width tradeoff and matrix memory; no speed ratio from incomplete calls. Next test constant-block elimination and exact Gray updates.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
