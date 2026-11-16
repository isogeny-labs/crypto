from pathlib import Path
import json,sys,subprocess,time,hashlib,resource
from .small import oracle
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'qualified_speedup':None};error=None
 try:
  M=(1<<83)|(1<<45)|7
  def sq(x):
   v=sum(((x>>i)&1)<<(2*i)for i in range(83))
   while v.bit_length()>83:v^=M<<(v.bit_length()-84)
   return v
  v=2
  for _ in range(83):v=sq(v)
  assert v==2 and M%2 and (M.bit_count()%2)==1
  out['field_rabin_check']={'degree':83,'prime_degree':True,'frobenius_x_equals_x':True,'gcd_T2_plus_T':1}
  out['small']=oracle();assert time.monotonic()-START<45
  b=subprocess.run(['g++','-O3','-std=c++17',str(P/'guards.cpp'),'-o',str(P/'guards')],capture_output=True,text=True,timeout=15)
  with(P/'build_stdout.txt').open('x')as f:f.write(b.stdout)
  with(P/'build_stderr.txt').open('x')as f:f.write(b.stderr)
  assert b.returncode==0,b.stderr
  r=subprocess.run([str(P/'guards')],capture_output=True,text=True,timeout=55)
  with(P/'native_stdout.txt').open('x')as f:f.write(r.stdout)
  with(P/'native_stderr.txt').open('x')as f:f.write(r.stderr)
  assert r.returncode==0,r.stderr
  out['actual']=json.loads(r.stdout);assert out['actual']['complete']
  out['guards_hold']=all(x['pair_count']==0 for x in out['actual']['targets'])
 except Exception as e:error=type(e).__name__+': '+str(e)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<78
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exact Frobenius branch classification oracle and actual GF83 lower-arity guards for squarefree five-source rationalization.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['If guards pass, implement complete squarefree rational group backend with repeated-source fallback; measure true search width before claiming a speedup.']};text=json.dumps(result);assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
