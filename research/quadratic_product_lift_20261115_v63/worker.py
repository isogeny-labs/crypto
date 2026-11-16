from pathlib import Path
import json,hashlib,base64,gzip,resource,sys,time
from .encoder import encode,extend,layout,masks,evaluate
P=Path(__file__).resolve().parent;START=time.monotonic()
def fm(a,b,n,mod):
 v=0
 for i in range(n):
  if b>>i&1:v^=a<<i
 for i in range(v.bit_length()-1,n-1,-1):
  if v>>i&1:v^=mod<<(i-n)
 return v

def literal(bits,n,mod,d,z,B):
 m=(1<<d)-1;a=bits&m;b=(bits>>d)&m;c=(bits>>(2*d))&m;u=bits>>(3*d)
 def eq(x,y,t):
  xy=fm(x,y,n,mod);xt=fm(x,t,n,mod);yt=fm(y,t,n,mod)
  return fm(xy,xy,n,mod)^fm(xt,xt,n,mod)^fm(yt,yt,n,mod)^fm(xy,t,n,mod)^B
 return eq(a,b,u)|(eq(u,c,z)<<n)

def main():
 out={'tiny_states':0,'tiny_mutations':0,'augmented_states':0,'profiles':[]};error=None
 try:
  profiles=[(1,3,1,0,0),(2,7,1,2,1),(2,7,1,3,3),(3,11,2,4,1),(3,11,2,7,3),(4,19,2,6,7),(4,19,2,15,15)]
  for n,mod,d,z,B in profiles:
   s=encode(n,mod,d,z,B);rows=masks(s);v=3*d+n;roots=[]
   for bits in range(1<<v):
    ext=extend(bits,n,d);want=literal(bits,n,mod,d,z,B);assert evaluate(rows,ext)==want
    if want==0:roots.append(bits)
    for i in range(v,s['n_vars']):assert evaluate(rows,ext^(1<<i))>>(2*n)!=0;out['tiny_mutations']+=1
    out['tiny_states']+=1
   out['profiles'].append({'n':n,'modulus':mod,'d':d,'z':z,'B':B,'original_roots':len(roots),'ordered_root_masks':roots})
   if n<=2:
    got=[]
    for full in range(1<<s['n_vars']):
     if evaluate(rows,full)==0:
      bits=full&((1<<v)-1);assert full==extend(bits,n,d);got.append(bits)
     out['augmented_states']+=1
    assert sorted(got)==roots
  desc=json.loads(Path('research/general_binary_cubic_repair_20261115_v60/full_ordinary_instance.json').read_text());n=desc['field_degree'];mod=desc['modulus'];d=desc['d'];z=desc['z'];B=desc['B'];begin=time.monotonic();s=encode(n,mod,d,z,B);rows=masks(s);out['encoding_seconds']=time.monotonic()-begin
  old=json.loads(Path('research/ordinary_boolean_encoding_20261115_v61/neutral.json').read_text());out.update(n_vars=s['n_vars'],equation_count=s['equation_count'],term_occurrences=s['term_occurrences'],max_degree=s['max_degree'],reference_term_occurrences=old['term_occurrences'],term_ratio=s['term_occurrences']/old['term_occurrences'],prediction_term_ratio_lt_half=s['term_occurrences']*2<old['term_occurrences'])
  known=json.loads(Path('research/ordinary_boolean_encoding_20261115_v61/raw.json').read_text())['out']['known_original_root_hex'];out['extended_root_hex']=[hex(extend(int(x,16),n,d))for x in known];out['original_root_hex']=known;samples=[0,(1<<(3*d+n))-1]+[int(x,16)for x in known];state=63001
  for _ in range(16):state=(state*6364136223846793005+1442695040888963407)&((1<<192)-1);samples.append(state&((1<<(3*d+n))-1))
  out['actual_receipts']=[]
  for bits in samples:
   want=literal(bits,n,mod,d,z,B);assert evaluate(rows,extend(bits,n,d))==want;out['actual_receipts'].append({'original_hex':hex(bits),'residual_hex':hex(want)})
  for i in range(3*d+n,s['n_vars']):assert evaluate(rows,int(out['extended_root_hex'][0],16)^(1<<i))>>(2*n)!=0
  payload=json.dumps(s,separators=(',',':')).encode();(P/'neutral.json').write_bytes(payload);(P/'neutral.json.gz.b64').write_bytes(base64.b64encode(gzip.compress(payload,mtime=0)));out['input_sha256']=hashlib.sha256(payload).hexdigest();out['input_bytes']=len(payload)
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Independent literal-coordinate and unique-extension tests of sparse quadratic lifting.','evidence':{'out':out,'error':error,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'worker_seconds':time.monotonic()-START,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['If exactness passes, run changed quadratic encoding through matched baseline diagnostics; compare full lifetime and memory, no timeout ratios.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
