from pathlib import Path
from itertools import combinations_with_replacement
from collections import Counter
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.binary_norm_coverage_20261115_v57.oracle import points,jets,sumxor
from research.positive_binary_charts_20261115_v53 import math as curve
from research.generic_block_20261115_v27 import backend as linear
from .solver import shifted,coefficients
P=Path(__file__).resolve().parent;START=time.monotonic()
def interpolate4(f,e,A,ps,Q):
 rows=[];rhs=[]
 for point,m in Counter(list(ps)+[Q]).items():
  X,Y=jets(e,A,point,m);X2=[sumxor(e.mul(X[i],X[j-i])for i in range(j+1))for j in range(m)]
  for j in range(m):
   vals=[X[j],1 if j==0 else 0,Y[j]]
   for shift in[0,f.degree]:
    row=[(v>>shift)&e.mask for v in vals];value=(X2[j]>>shift)&e.mask
    if any(row)or value:rows.append(row);rhs.append(value)
 result=linear.solve(curve.Ops(f),rows,rhs,3);assert result is not None and not result['basis'];return result['particular']
def run(case):
 f=BinaryField(case['n'],case['modulus']);A=case['A'];B=case['B'];d=case['d'];z,y=case['target'];result=shifted(f,A,B,z,y,d);Qp=curve.add(f,A,B,(z,y),(0,1));assert tuple(result['translated_target'])==Qp
 e,pool=points(f,A,B,d);expected=set();witnesses=[]
 for ps in combinations_with_replacement(pool,3):
  T=curve.total(e,A,B,ps)
  if T is None or T!=(Qp[0],Qp[1]^Qp[0]):continue
  counts=Counter(ps+(Qp,))
  if not all(counts[(x,v)]==counts[(x,e.conjugate(v))]for x,v in counts):continue
  r,s,h=interpolate4(f,e,A,ps,Qp);C=coefficients(f,A,B,*Qp,h,r);expectedC=(1,)
  for x,v in ps:expectedC=curve.polynomial.b._mul(expectedC,(x,1),f)
  assert C==expectedC;expected.add((h,r,C));witnesses.append({'points':ps,'function':[r,s,h],'C':C})
 got={(r['h'],r['r'],r['C'])for r in result['solutions']};assert got==expected and got
 original=tuple(case['original_values']);assert original in result['original_tuples'];assert any(curve.polynomial.b._mul(row['C'],(1,0,1),f)==tuple(case['H'])for row in result['solutions'])
 return {'n':f.degree,'d':d,'target':case['target'],'translated_target':Qp,'chart':result,'independent_Hermite_witnesses':witnesses}
def main():
 out=[];error=None
 try:
  cases=json.loads(Path('research/binary_norm_guard_counterexample_20261115_v57/raw.json').read_text())['out']
  for case in cases:out.append(run(case));assert time.monotonic()-START<65
  # Pole and trace guards fail explicitly; no incomplete empty answer.
  f=BinaryField(7,131)
  for A,z,y in[(0,95,14),(1,0,1)]:
   try:shifted(f,A,1,z,y,2)
   except ValueError:pass
   else:raise AssertionError('guard was not enforced')
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=75:error='late serialized output'
 if error:
  for r in out:r['chart']['complete']=False;r['chart']['original_tuples_verified_lower_bound']=r['chart'].pop('original_tuples',[])
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Complete torsion-shifted cubic charts restore the exact original-root counterexamples and match independent multiplicity-aware group/Hermite oracles.','evidence':{'error':error,'cases':[{'n':v['n'],'h_values':v['chart']['h_values'],'candidates':v['chart']['candidates'],'norm_solutions':len(v['chart']['solutions']),'original_tuples':len(v['chart'].get('original_tuples',[]))}for v in out],'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'complete_global_root_search':False,'release_gate_claim':False},'next_actions':['Test the exact main-plus-shifted chart union on complete tiny original root sets before deriving a native full-width outer solver.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
