from itertools import product
from pathlib import Path
import json,time
from research.prime_orbit_roots_20261116_v71.solver import solve
from research.prime_orbit_roots_20261116_v71.field import Prime
from research.translation_orbits_20261115_v69.algebra import literal_s3
from .portfolio import portfolio,method
from .common import P,finish
START=time.monotonic()
def main():
 out={'rows':[],'oracle_systems':0,'literal_checks':0,'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text())
  for prime in [3,5,7,17]:
   f=Prime(prime)
   for beta in product(range(2),repeat=3):
    for lo,hi in [(0,prime-1),(0,0),(prime//2,prime-1)]:
     inst={'p':prime,'A':1,'B':1,'z':1,'beta':list(beta),'shift_low':lo,'shift_high':hi};expected=[]
     for t in range(lo,hi+1):
      a,b,c=[f.add(x,t)for x in beta]
      for u in range(prime):
       out['literal_checks']+=1
       if literal_s3(a,b,u,1,1,f)==0 and literal_s3(u,c,1,1,1,f)==0:expected.append([t,u])
     assert portfolio(inst)['roots']==solve(inst,'factor')['roots']==expected;out['oracle_systems']+=1
  for field in data['fields']:
   for bits in data['bits']:
    inst=json.loads((P/(field+'_'+str(bits)+'.json')).read_text());results={};times={}
    for m in ['factor','difference']:
     assert time.monotonic()-START<67
     begin=time.monotonic();results[m]=solve(inst,m);times[m]=time.monotonic()-begin
    assert results['factor']['roots']==results['difference']['roots'];roots=results['factor']['roots']
    out['rows'].append({'field':field,'source_bits':bits,'interval_cardinality':inst['shift_high']+1,'phase_seconds':times,'dispatch_method':method(inst),'roots':roots,'complete':True,'exact_root_count':len(roots)})
  out['complete']=True
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Five-field frozen interval crossover ladder and independent literal interval oracles.','Retain losses and compare preregistered cold methods without training on this exposed panel.')
if __name__=='__main__':main()
