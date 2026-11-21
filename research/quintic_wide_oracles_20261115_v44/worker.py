from pathlib import Path
import hashlib,json,resource,sys,time
from rootfinder import binaryfield as b
from research.binary_wide_20261115_v41.backend import WideBinaryField
from research.certified_support_20261115_v44 import solver as old
from research.quintic_discriminant_20261115_v44 import solver as s
P=Path(__file__).resolve().parent;START=time.monotonic()

def main():
 profiles=[];error=None
 try:
  data=json.loads((P/'DATA.json').read_text())
  for n,mod in data['fields']:
   begin=time.monotonic();f=WideBinaryField(n,mod);lam=old.compile_basis(f,[1<<i for i in range(18)])[0];rows=[]
   for roots in data['root_bags']:
    H=(1,)
    for x in roots:H=b._mul(H,(x,1),f)
    repeat=len(set(roots))<5;expected=all(x<1<<18 for x in roots)
    assert s.repeated(f,H)==repeat
    assert (len(b._gcd(H,(H[1],0,H[3],0,1),f))>1)==repeat
    assert s.supported(f,H,lam)==expected
    rows.append(dict(roots=roots,H=H,repeated=repeat,supported=expected))
   profiles.append(dict(n=n,modulus=mod,cases=len(rows),seconds=time.monotonic()-begin,rows=rows));assert time.monotonic()-START<68
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 out=dict(profiles=profiles,error=error);trial=json.dumps(out,separators=(',',':'))
 if time.monotonic()-START>=75:out['error']='late serialized output';error=out['error']
 raw=json.dumps(out,sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Cross-binary-width discriminant and multiplicity-safe support controls; no curve or all-roots speed claim.',evidence=dict(error=error,profiles=[{k:v for k,v in x.items()if k!='rows'}for x in profiles],worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Derive a separate odd-characteristic low-degree resultant kernel; exploit coefficient-cell repeated-root guards only after measuring whole search.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
