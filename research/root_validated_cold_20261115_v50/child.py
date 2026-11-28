from pathlib import Path
import ctypes
from research.native_validation_20261115_v49.adapter import verify
from types import SimpleNamespace
import json,hashlib,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.root_conditioned_fibers_20261115_v45 import solver as s
from research.certified_support_20261115_v44 import solver as prior
from research.compiled_subspace_20261115_v31 import factor
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()

def solve_chart(method,h_index):
 rule=json.loads((P/'DATA.json').read_text());prior.verify_certificates();f=BinaryField(83,prior.MOD);ctx=s.c.compile_context(f,0,prior.Z,prior.QY^prior.Z);cover=json.loads((R/rule['centroid_receipt']).read_text())['out']['actual']['domain'];h=cover['particular']^s.b.xor(cover['basis'][j]for j in range(18)if(h_index^(h_index>>1))>>j&1);assert h
 C=s.b.Context(f,prior.Z,ctx['d'],[h],[0],(35,52,69));C.prepare_blocks();fiber=C.block(h,0);assert len(fiber['basis'])==73;base,cols=s.model(f,ctx,h,fiber)
 text=str(len(cols))+'\n'+'\n'.join(' '.join(format(c,'x')for c in H)for H in [base]+cols)+'\n';text+=' '.join(format(v,'x') for v in [h,ctx['c'],ctx['d'],prior.Z])+'\n'
 built=SimpleNamespace(stderr='');compile_seconds=0

 t=time.monotonic();run=subprocess.run([str(P/(method+'_scan'))],input=text,capture_output=True,text=True,timeout=60);assert run.returncode==0,run.stderr;native=json.loads(run.stdout);native_wall=time.monotonic()-t
 checkpoint_start=time.monotonic();lib=load_library();assert verify(lib,base,cols,native['checkpoints'])==0;checks=len(native['checkpoints'])
 checkpoint_seconds=time.monotonic()-checkpoint_start;supported=[];seen=set();validation_start=time.monotonic();mask=(1<<83)-1;entries=[]
 for x,tstr in native['candidates']:
  t=int(tstr,16);assert t<1<<73;uv=fiber['particular']^s.b.apply(fiber['basis'],t);H=tuple(s.c.norm_coefficients(f,ctx,h,uv&mask,uv>>83));assert s.evaluate(f,H,x)==0
  if t in seen:continue
  seen.add(t);entries.append((t,uv,H));assert time.monotonic()-START<75
 flags=support_flags(lib,[H for t,uv,H in entries])
 for (t,uv,H),good in zip(entries,flags):
  if good:
   result=factor.supported_roots(SimpleNamespace(p=2,field=f),H,[1<<i for i in range(18)]);assert result['supported']and len(result['roots'])==5 and all(m==1 and root for root,m in result['roots']);supported.append(dict(parameter=t,u=uv&mask,v=uv>>83,H=H,roots=result['roots']))
 supported.sort(key=lambda row:row['parameter'])
 complete=native['complete']and native['visited']==(1<<18)-1
 return dict(h=h,k=0,native=native,native_stderr=run.stderr,compile_stderr=built.stderr,checkpoint_seconds=checkpoint_seconds,candidate_validation_seconds=time.monotonic()-validation_start,compile_seconds=compile_seconds,native_wall_seconds=native_wall,independent_checkpoint_systems=checks,distinct_candidates=len(seen),supported=supported,norm_family_complete=complete,supported_norm_count_exact=len(supported)if complete else None,scope='One fixed h k0 norm family, certified x0 excluded; all other h/k and original chain open')

def load_library():
 lib=ctypes.CDLL(str(P/'validation.so'));lib.support83.argtypes=[ctypes.POINTER(ctypes.c_uint64),ctypes.c_size_t,ctypes.POINTER(ctypes.c_uint8)];lib.support83.restype=ctypes.c_int;return lib

def support_flags(lib,polys):
 result=[]
 for start in range(0,len(polys),2048):
  batch=polys[start:start+2048];words=[]
  for H in batch:
   for v in H:words.extend([v&((1<<64)-1),v>>64])
  data=(ctypes.c_uint64*len(words))(*words);out=(ctypes.c_uint8*len(batch))();assert lib.support83(data,len(batch),out)==0;result.extend(bool(v)for v in out)
 return result

if __name__=='__main__':
 begin=time.monotonic();method=sys.argv[1];rule=json.loads((P/'DATA.json').read_text());rows=[]
 for index in rule['h_indices']:
  result=solve_chart(method,index);native=result['native'];assert result['norm_family_complete'];fingerprint=hashlib.sha256(json.dumps(sorted((x,int(t,16))for x,t in native['candidates']),separators=(',',':')).encode()).hexdigest()
  rows.append(dict(h_index=index,h=result['h'],norms=result['supported'],candidate_sha256=fingerprint,candidates=len(native['candidates']),complete=result['norm_family_complete'],native_seconds=result['native_wall_seconds'],checkpoint_seconds=result['checkpoint_seconds'],candidate_validation_seconds=result['candidate_validation_seconds'],native_peak_rss_kib=native['peak_rss_kib']))
 print(json.dumps(dict(method=method,charts=rows,internal_total_seconds=time.monotonic()-begin,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss),separators=(',',':')))
