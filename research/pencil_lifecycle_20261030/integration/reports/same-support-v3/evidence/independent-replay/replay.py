from pathlib import Path
import subprocess,os,time,json,hashlib
p=Path(__file__).parent;m=Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r'); binary=m/'toolchain/bin/lean'
env=os.environ.copy();env['LEAN_PATH']=str(p)+':'+str(m/'deps')
records=[]
def run(label,args):
 t=time.monotonic(); r=subprocess.run(['timeout','120',str(binary),*args],cwd=p,env=env,capture_output=True)
 (p/(label+'.stdout')).write_bytes(r.stdout);(p/(label+'.stderr')).write_bytes(r.stderr)
 rec={'label':label,'argv':['timeout','120',str(binary),*args],'LEAN_PATH':env['LEAN_PATH'],'exit_code':r.returncode,'elapsed_seconds':time.monotonic()-t};records.append(rec);(p/'outcomes.json').write_text(json.dumps(records,indent=2));print(json.dumps(rec),flush=True);return r.returncode
with (p/'DEPENDENCY_SHA256SUMS').open('w') as f:
 for x in sorted((m/'deps').rglob('*')):
  if x.is_file(): f.write(hashlib.sha256(x.read_bytes()).hexdigest()+'  '+str(x)+'\n')
(p/'TOOLCHAIN_SHA256SUMS').write_text(hashlib.sha256(binary.read_bytes()).hexdigest()+'  '+str(binary)+'\n')
if run('version',['--version']):raise SystemExit(1)
for name in ['ProjectiveLawBoundary','RuntimeSemantics','SameSupport','SameSupportClassifier','WeightedSameSupport']:
 if run(name,['-j1','-M4096','-o',name+'.olean',name+'.lean']):raise SystemExit(1)
helper='import WeightedSameSupport\n'
for n in ['SameSupportCandidate.weighted_valid_inputs_exclude_same_support_error','SameSupportCandidate.cross_scale','SameSupportCandidate.validity_coefficients','SameSupportCandidate.valid_inputs_exclude_same_support_error','ProjectiveLawRuntime.classifySameU22_ne_zeroSlope','ProjectiveLawBoundary.same_u_zero_slope_delta_is_inverse']:
 helper+='#check '+n+'\n#print axioms '+n+'\n'
(p/'IndependentCheck.lean').write_text(helper)
if run('IndependentCheck',['-j1','-M4096','IndependentCheck.lean']):raise SystemExit(1)
(p/'REPLAY_SHA256SUMS').write_text(''.join(hashlib.sha256(x.read_bytes()).hexdigest()+'  '+x.name+'\n' for x in sorted(p.iterdir()) if x.is_file() and x.name!='REPLAY_SHA256SUMS'))
