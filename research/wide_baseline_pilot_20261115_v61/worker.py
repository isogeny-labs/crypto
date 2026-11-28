from pathlib import Path
import hashlib,json,os,resource,signal,subprocess,sys,time
P=Path(__file__).resolve().parent;START=time.monotonic()
def invoke(command,env):
 def limits():resource.setrlimit(resource.RLIMIT_AS,(3*1024**3,3*1024**3))
 begin=time.monotonic();p=subprocess.Popen(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,env=env,start_new_session=True,preexec_fn=limits);timedout=False
 try:
  try:stdout,stderr=p.communicate(timeout=9)
  except subprocess.TimeoutExpired:
   timedout=True;os.killpg(p.pid,signal.SIGKILL);stdout,stderr=p.communicate()
  return {'command':command,'stdout':stdout,'stderr':stderr,'exit_code':p.returncode,'outer_timeout':timedout,'wall_seconds':time.monotonic()-begin}
 finally:
  try:os.killpg(p.pid,signal.SIGKILL)
  except ProcessLookupError:pass

def main():
 out={'records':[],'qualified_ratio':None,'F6_contract_status':'No137-variable all-root entrypoint in inspected extension; CLI exposes small controls and necessity proof caps20variables.'};error=None
 try:
  data=json.loads((P/'DATA.json').read_text());inputfile=Path(data['input']);before=hashlib.sha256(inputfile.read_bytes()).hexdigest();known=json.loads(Path(data['encoding_receipt']).read_text())['out'];assert before==known['input_sha256'];expected={int(x,16)for x in known['known_original_root_hex']};env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',ROOTFINDER_INSTANCE_SHA256=before,F4_F2_MAX_ROWS='20000',F4_F2_MAX_COLS='40000',TMPDIR=str(P/'scratch'))
  for name in list(env):
   if name.startswith(('KIC_F4_','KIC_GF2_'))or name in['KIC_LINEAR_ELIM','SOLVER_SPLIT_RULE','IC_GROEBNER_CACHE_URL']:env.pop(name,None)
  (P/'scratch').mkdir()
  for engine in data['engines']:
   command=[part.replace('{input}',str(inputfile)).replace('{sha}',before)for part in engine['command']];record=invoke(command,env);parsed=None
   try:parsed=json.loads(record['stdout'])
   except json.JSONDecodeError:pass
   complete=False;roots=set()
   if parsed is not None:
    if parsed.get('schema')=='scientific_result_v1':e=parsed.get('evidence',{});complete=e.get('all_original_roots_complete')is True;rawroots=e.get('roots',[])
    else:complete=parsed.get('complete')is True;rawroots=parsed.get('roots_hex',[])
    roots={int(v,16)for v in rawroots};assert roots<=expected
    if complete:assert roots==expected
   assert hashlib.sha256(inputfile.read_bytes()).hexdigest()==before
   record.update(label=engine['label'],implementation_scope=engine['scope'],parsed=parsed,complete_matching_roots=complete,returned_verified_root_lower_bound=len(roots),qualified_speedup=None);payload=json.dumps(record,sort_keys=True,separators=(',',':')).encode()
   with(P/(engine['label']+'.json')).open('xb')as file:file.write(payload)
   out['records'].append({'label':engine['label'],'exit_code':record['exit_code'],'outer_timeout':record['outer_timeout'],'wall_seconds':record['wall_seconds'],'complete_matching_roots':complete,'verified_root_lower_bound':len(roots),'status':parsed.get('status')if parsed else'no_complete_JSON','reason':parsed.get('reason',parsed.get('evidence',{}).get('termination_reason'))if parsed else None,'raw_sha256':hashlib.sha256(payload).hexdigest()});assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=75:error='late serialized output'
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Bounded source-identified same-ANF wide baseline diagnostics; incomplete calls yield no speed ratios.','evidence':{'error':error,'out':out,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Use actual engine/import/degree/memory diagnostics to change the next baseline experiment; do not rerun an unchanged timeout or call inheritedF4 anF6. Add descriptor encoding inside any future end-to-end comparison.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
