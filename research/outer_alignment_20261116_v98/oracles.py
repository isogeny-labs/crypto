import json,time
from .common import P,finish
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
START=time.monotonic();out={'cases':[]};error=None
try:
 d=json.loads((P/'DATA.json').read_text())
 for n,mod,dim in d['tiny']:
  f=Field(2,mod);q=1<<n;D=range(1<<dim);adj={(a,b):[z for z in range(q)if s3(a,b,z,0,1,f)==0]for a in range(q)for b in range(q)}
  for T in range(q):
   expected=[]
   for x1 in D:
    for x2 in D:
     for u1 in adj[x1,x2]:
      for x3 in D:
       for u2 in adj[u1,x3]:
        for x4 in D:
         for u3 in adj[u2,x4]:
          for x5 in D:
           if T in adj[u3,x5]:expected.append((x1,x2,x3,x4,x5,u1,u2,u3))
   expected.sort();stats={}
   for method in['join','block','filtered']:
    r=solve({'n':n,'mod':mod,'d':dim,'T':T},method);assert r['complete']and r['roots']==expected;stats[method]=r['stats']
   out['cases'].append({'n':n,'d':dim,'T':T,'root_count':len(expected),'stats':stats})
 finish(out,None,START,'Complete original8-coordinate chain roots: literal adjacency,separatorjoin,rebracket andcertifiedfilter agree.','Measure totalcoldcost beforeintegration;profileactualsource-generatedseparatorimage,notuniformfield alone.')
except Exception as e:finish(out,repr(e),START,'Whole-chain oracle.','Preservecounterexample;repaironlynewimmutablevariant andkeepalloriginalintermediatebranches.')
