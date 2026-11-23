from pathlib import Path
import base64,gzip,hashlib,json,resource,sys,time
from .encoder import encode,pack,values,evaluate
from research.original_chain_lift_20261115_v55.worker import eq
P=Path(__file__).resolve().parent;START=time.monotonic()
def direct(v,z,B,mod,n):
 a,b,c,u=v;return eq(a,b,u,B,mod)|(eq(u,c,z,B,mod)<<n)
def main():
 out={'tiny_states':0,'tiny_profiles':[],'actual_receipts':[]};error=None
 try:
  for n,mod,d,z,B in[(2,7,1,2,1),(2,7,1,3,3),(3,11,2,4,1),(3,11,2,7,3),(4,19,2,6,7),(4,19,2,15,15)]:
   system=encode(n,mod,d,z,B);count=roots=0
   for bits in range(1<<system['n_vars']):
    want=direct(values(bits,n,d),z,B,mod,n);assert evaluate(system['equations'],bits)==want;roots+=want==0;count+=1
   out['tiny_states']+=count;out['tiny_profiles'].append({'n':n,'modulus':mod,'d':d,'z':z,'B':B,'states':count,'roots':roots})
  instance=json.loads(Path('research/general_binary_cubic_repair_20261115_v60/full_ordinary_instance.json').read_text());n=instance['field_degree'];mod=instance['modulus'];d=instance['d'];z=instance['z'];B=instance['B'];system=encode(n,mod,d,z,B,instance['A'],instance['source_ordinate']);system['equations_sha256']=hashlib.sha256(json.dumps(system['equations'],separators=(',',':')).encode()).hexdigest();payload=json.dumps(system,separators=(',',':')).encode()
  with(P/'neutral.json').open('xb')as file:file.write(payload)
  archived=base64.b64encode(gzip.compress(payload,mtime=0))
  with(P/'neutral.json.gz.b64').open('xb')as file:file.write(archived)
  state=61001;mask=(1<<(3*d+n))-1;samples=[0,mask]
  for _ in range(16):
   value=0
   for i in range(3):state=(state*6364136223846793005+1442695040888963407)&((1<<64)-1);value|=state<<(64*i)
   samples.append(value&mask)
  expected=json.loads(Path('research/full_ordinary_cubic_20261115_v60/raw.json').read_text())['out']['original_tuples'];samples +=[pack(v,n,d)for v in expected]
  mutation=None
  for i in range(system['n_vars']):
   bad=pack(expected[0],n,d)^(1<<i)
   if direct(values(bad,n,d),z,B,mod,n):mutation=bad;break
  assert mutation is not None;samples.append(mutation)
  for bits in samples:
   want=direct(values(bits,n,d),z,B,mod,n);assert evaluate(system['equations'],bits)==want;out['actual_receipts'].append({'assignment_hex':hex(bits),'literal_coordinate_output_hex':hex(want)})
  out.update(input_sha256=hashlib.sha256(payload).hexdigest(),input_bytes=len(payload),archive_bytes=len(archived),equations_sha256=system['equations_sha256'],n_vars=system['n_vars'],equation_count=system['equation_count'],term_occurrences=system['term_occurrences'],max_degree=system['max_degree'],known_original_root_hex=sorted(hex(pack(v,n,d))for v in expected),mutation_rejected=True)
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=70:error='late serialized output'
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Exact original-ANF encoding with independent exhaustive tiny and full-width coordinate replay.','evidence':{'error':error,'out':out,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Pin source/binary/configuration and run bounded exact same-ANF baseline diagnostics; charge encoding in later end-to-end comparisons.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<80;print(text)
if __name__=='__main__':main()
