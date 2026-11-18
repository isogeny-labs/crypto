from pathlib import Path
import ctypes,hashlib,itertools,json,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.certified_support_20261115_v44 import solver as old
from research.root_conditioned_fibers_20261115_v45 import solver as linear
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()

def run():
 f=BinaryField(83,old.MOD);lam=old.compile_basis(f,[1<<i for i in range(18)])[0];polys=[];expected=[]
 for cs in itertools.product(range(2),repeat=5):
  H=list(cs)+[1];polys.append(H);work=H[:]
  for x in [0,1]:
   while len(work)>1 and linear.evaluate(f,work,x)==0:
    q=[0]*(len(work)-1);q[-1]=1
    for j in range(len(q)-2,-1,-1):q[j]=work[j+1]^(q[j+1]if x else 0)
    work=q
  expected.append(len(work)==1)
 cases=json.loads((R/'research/subspace_factor_oracles_20261115_v30/raw.json').read_text())['out']['actual']['records']
 for row in cases:polys.append(row['H']);expected.append(row['result']['supported'])
 text=(R/'research/root_conditioned_native_20261115_v45/matrix.txt').read_text().split();m=int(text[0]);numbers=[int(v,16)for v in text[1:]];base=numbers[:6];cols=[numbers[j:j+6]for j in range(6,len(numbers),6)];assert len(cols)==m
 records=json.loads((R/'research/root_conditioned_native_20261115_v45/raw.json').read_text())['out']['native']['candidates']
 for x,t in records:
  bits=int(t,16);H=[a^linear.b.xor(c[j]for i,c in enumerate(cols)if bits>>i&1)for j,a in enumerate(base)];polys.append(H);expected.append(False)
 begin=time.monotonic();built=subprocess.run(['g++','-O3','-std=c++17','-shared','-fPIC',str(P/'validation.cpp'),'-o',str(P/'validation.so')],capture_output=True,text=True,timeout=12);assert built.returncode==0,built.stderr;compile_seconds=time.monotonic()-begin
 lib=ctypes.CDLL(str(P/'validation.so'));lib.support83.argtypes=[ctypes.POINTER(ctypes.c_uint64),ctypes.c_size_t,ctypes.POINTER(ctypes.c_uint8)];lib.support83.restype=ctypes.c_int;packed=[]
 for H in polys:
  for v in H:packed.extend([v&((1<<64)-1),v>>64])
 data=(ctypes.c_uint64*len(packed))(*packed);out=(ctypes.c_uint8*len(polys))();t=time.monotonic();assert lib.support83(data,len(polys),out)==0;native_seconds=time.monotonic()-t;got=[bool(v)for v in out];assert got==expected
 for H,want in zip(polys,expected):assert old.generic(f,tuple(H),lam)==want
 verification=check_rows(lib,f,base,cols)
 return dict(row_verification=verification,cases=len(polys),supported=sum(expected),rejected=len(polys)-sum(expected),flags=got,compile_seconds=compile_seconds,native_batch_seconds=native_seconds,compile_stderr=built.stderr)

def check_rows(lib,f,base,cols):
 from research.native_validation_20261115_v49.adapter import verify
 rows=[]
 for name in ['root_conditioned_native_20261115_v45','root_compressed_native_20261115_v48']:
  rows+=json.loads((R/'research'/name/'raw.json').read_text())['out']['native']['checkpoints']
 for x,t in json.loads((R/'research/root_conditioned_native_20261115_v45/raw.json').read_text())['out']['native']['candidates'][:8]:
  hom=linear.b.solve(linear.b.rows_from([linear.evaluate(f,H,x)for H in cols],0,83),73);sol=linear.condition(f,base,cols,x);assert sol is not None
  rows.append(dict(x=x,rank=hom['rank'],consistent=True,particular=format(sol['particular'],'x'),basis=[format(t,'x')for t in sol['basis']]))
 assert verify(lib,base,cols,rows)==0
 synthetic=[[1,0,0,0,0,0],[2,0,0,0,0,0],[3,0,0,0,0,0]];row=dict(x=1,rank=2,consistent=True,particular='0',basis=['7']);assert verify(lib,[0]*6,synthetic,[row])==0
 false=dict(row,consistent=False);assert verify(lib,[4,0,0,0,0,0],synthetic,[false])==0
 for bad in [dict(row,rank=1),dict(row,particular='1'),dict(row,basis=['1'])]:assert verify(lib,[0]*6,synthetic,[bad])!=0
 return dict(actual_systems=len(rows),synthetic_valid_systems=2,rejected_corruptions=3,portable_field_arithmetic=True)

def main():
 out={};error=None
 try:out=run()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=26:error='late serialized output'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Native repeated-root-safe GF83 low18bit quintic support with independent known-factor controls.',evidence=dict(error=error,summary={k:v for k,v in out.items()if k!='flags'},worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Integrate native predicate into both cold comparison arms and repeat a newly frozen complete conditional-chart panel; retain positive extraction and serialization costs.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<30;print(text)
if __name__=='__main__':main()
