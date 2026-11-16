from pathlib import Path
import json,hashlib,resource,sys,time
from research.prime_orbit_roots_20261116_v71.field import Prime
from research.prime_orbit_roots_20261116_v71.solver import solve
from lab.research.nist_fields import preset,prove_prime
P=Path(__file__).resolve().parent;START=time.monotonic()
def add(Pt,Qt,A,f):
 if Pt is None:return Qt
 if Qt is None:return Pt
 x,y=Pt;u,v=Qt
 if x==u:
  if f.add(y,v)==0:return None
  slope=f.mul(f.add(f.scale(f.sq(x),3),A),f.inv(f.scale(y,2)))
 else:slope=f.mul(f.sub(v,y),f.inv(f.sub(u,x)))
 w=f.sub(f.sub(f.sq(slope),x),u);return w,f.sub(f.mul(slope,f.sub(x,w)),y)
def on(point,A,B,f):
 return point is None or f.sq(point[1])==f.add(f.add(f.mul(f.sq(point[0]),point[0]),f.mul(A,point[0])),B)
def main():
 out={'fields':[],'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text())
  for name in data['fields']:
   v=preset(name);f=Prime(v['p']);A=v['a'];B=v['b'];proof=prove_prime(f.p,data['flint_library']);assert proof['proven_prime'];assert on(v['generator'],A,B,f);points=[];x=data['point_start']+sum(map(ord,name))
   for _ in range(data['point_trials']):
    rhs=f.add(f.add(f.mul(f.sq(x),x),f.mul(A,x)),B);y=f.sqrt(rhs)
    if y is not None and y!=0:points.append((x,y))
    x+=1
    if len(points)==3:break
   assert len(points)==3;S=add(add(points[0],points[1],A,f),points[2],A,f);assert S is not None and on(S,A,B,f);start=min(x for x,y in points);beta=[x-start for x,y in points];upper=(1<<data['source_bits'])-1-max(beta);assert 0<=start<=upper;target=(S[0],f.neg(S[1]));assert on(target,A,B,f);u0=add(points[0],points[1],A,f);assert u0 is not None
   inst={'kind':'prime','p':f.p,'A':A,'B':B,'z':target[0],'beta':beta,'shift_low':0,'shift_high':upper,'field':name,'case_role':'exposed planted correctness fixture;not a hard holdout','contract':'all canonical interval shifts and finite original S3 intermediates for fixed ordered offsets'}
   with(P/(name+'.json')).open('x')as file:json.dump(inst,file,indent=2)
   begin=time.monotonic();factor=solve(inst,'factor');factor_seconds=time.monotonic()-begin;begin=time.monotonic();scan=solve(inst,'difference');scan_seconds=time.monotonic()-begin;assert factor['roots']==scan['roots'];assert[start,u0[0]]in factor['roots'];out['fields'].append({'name':name,'p_bits':f.p.bit_length(),'prime_proof':{k:proof[k]for k in ['method','integer_hex','proven_prime','library_sha256']},'standard_generator_on_curve':True,'source_points':points,'target':target,'known_solution':[start,u0[0]],'roots':factor['roots'],'candidate_shifts':factor['candidate_shifts'],'factor_seconds_development':factor_seconds,'finite_difference_seconds_development':scan_seconds,'input_sha256':hashlib.sha256((P/(name+'.json')).read_bytes()).hexdigest()});assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exact conditional orbit roots on allfive source-identifiedNISTprimefields with standardcurvecoefficients,provedprimes,andcompleteintervalscan oracles.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Freeze many-field cold/memory diagnostics with allfields andstrong finite-difference baseline;fullGBqualification andglobalshape searchremain separate.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
