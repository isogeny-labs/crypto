from pathlib import Path
from types import SimpleNamespace
import ctypes,hashlib,json,subprocess,time
from rootfinder.binaryfield import BinaryField
from research.fused_affine_norm_20261115_v56 import compiler as general
from research.native_validation_20261115_v49.adapter import verify
from research.compiled_subspace_20261115_v31 import factor

def support(lib,polys):
 lib.support83.argtypes=[ctypes.POINTER(ctypes.c_uint64),ctypes.c_size_t,ctypes.POINTER(ctypes.c_uint8)];lib.support83.restype=ctypes.c_int;result=[]
 for start in range(0,len(polys),2048):
  batch=polys[start:start+2048];words=[]
  for H in batch:
   for v in H:words.extend([v&((1<<64)-1),v>>64])
  packed=(ctypes.c_uint64*len(words))(*words);flags=(ctypes.c_uint8*len(batch))();assert lib.support83(packed,len(batch),flags)==0;result.extend(bool(v)for v in flags)
 return result

def solve(instance,executable,library):
 begin=time.monotonic();f=BinaryField(instance['field_degree'],instance['modulus']);A,B,z,y,h,k=[instance[j]for j in ['A','B','z','y','h','k']];d=instance['d'];assert f.degree==83 and d==18 and z>=1<<d
 base,columns,fiber=general.model(f,A,B,z,y,h,k,d)
 if fiber is None:return dict(complete=True,solutions=[],candidate_pairs=[],reason='empty_coefficient_span_fiber',total_seconds=time.monotonic()-begin)
 n=f.degree;mask=(1<<n)-1;m=len(fiber['basis']);assert m<=83
 def image(bits):return tuple(general.norm.linear.xor(H[j]for i,H in enumerate(columns)if bits>>i&1)for j in range(6))
 mapped=image(fiber['particular']);smallbase=tuple(a^v for a,v in zip(base,mapped));smallcols=[image(v)for v in fiber['basis']];p=fiber['particular']
 text=str(m)+'\n'+'\n'.join(' '.join(format(v,'x')for v in H)for H in [smallbase]+smallcols)+'\n';context=[A,B,z,y,h,k,p&mask,p>>n];text+=' '.join(format(v,'x')for v in context)+'\n';text+='\n'.join(format(v&mask,'x')+' '+format(v>>n,'x')for v in fiber['basis'])+'\n'
 t=time.monotonic();run=subprocess.run([str(executable)],input=text,capture_output=True,text=True,timeout=60);wall=time.monotonic()-t;assert run.returncode==0,run.stderr;native=json.loads(run.stdout);assert native['complete']and native['visited']==1<<18
 lib=ctypes.CDLL(str(library));assert verify(lib,smallbase,smallcols,native['checkpoints'])==0
 entries=[];seen=set()
 for x,encoded in native['candidates']:
  tag=int(encoded,16);assert tag<1<<m;packed=p^general.norm.linear.apply(fiber['basis'],tag);r,s=packed&mask,packed>>n;H=general.coefficients(f,A,B,z,y,h,k,r,s);assert general.c.old.evaluate(f,H,x)==0
  if tag in seen:continue
  seen.add(tag);entries.append((tag,r,s,H))
 flags=support(lib,[H for tag,r,s,H in entries]);solutions=[]
 for(tag,r,s,H),flag in zip(entries,flags):
  if flag:
   out=factor.supported_roots(SimpleNamespace(p=2,field=f),H,[1<<i for i in range(d)]);assert out['supported']and sum(count for root,count in out['roots'])==5
   solutions.append(dict(parameter=tag,r=r,s=s,H=H,roots=out['roots']))
 solutions.sort(key=lambda row:row['parameter'])
 return dict(complete=True,solutions=solutions,candidate_pairs=sorted((x,int(t,16))for x,t in native['candidates']),native=native,native_stdout=run.stdout,native_stderr=run.stderr,input_sha256=hashlib.sha256(text.encode()).hexdigest(),fiber_dimension=m,native_wall_seconds=wall,total_seconds=time.monotonic()-begin)
