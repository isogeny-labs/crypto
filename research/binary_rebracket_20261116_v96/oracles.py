import json,time
from .common import P,finish
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
START=time.monotonic();out={'tiny':[],'actual':[],'zero_unique_total':0,'infinity_unique_total':0};error=None
try:
 d=json.loads((P/'DATA.json').read_text())
 for n,mod,dim in d['tiny']:
  f=Field(2,mod)
  for B in[1,2]:
   for a in range(1<<n):
    for b in range(1<<n):
     c={'n':n,'mod':mod,'d':dim,'a':a,'b':b,'B':B,'source_cap_bits':12,'diagonal_cap_bits':12,'pair_cap_bits':12};expected=[(x,y,z)for x in range(1<<dim)for y in range(1<<dim)for z in range(1<<n)if s3(a,x,z,0,B,f)==s3(z,y,b,0,B,f)==0];r=solve(c,'rebracket');s=solve(c,'source');assert r['complete']and s['complete']and r['roots']==s['roots']==expected
     out['zero_unique_total']+=r['stats']['zero_unique'];out['infinity_unique_total']+=r['stats']['infinity_unique'];out['tiny'].append({'n':n,'d':dim,'B':B,'a':a,'b':b,'root_count':len(expected),'stats':r['stats']})
 assert out['zero_unique_total']>0 and out['infinity_unique_total']>0
 for c in d['actual']:
  r=solve(c,'rebracket');f=Field(2,c['mod'])
  for x,y,z in r['roots']:assert x<1<<c['d']and y<1<<c['d']and s3(c['a'],x,z,0,c['B'],f)==s3(z,y,c['b'],0,c['B'],f)==0
  if 'planted' in c:assert tuple(c['planted'])in r['roots']
  out['actual'].append({'id':c['id'],**r})
 finish(out,None,START,'Exact two-factor binary rebracketing including zero bridges,infinity and original intermediate reconstruction.','Freeze cold comparison with strong one-source quadratic scan; actual18bit block is conditional,not wholechain.')
except Exception as e:finish(out,repr(e),START,'Rebracketing exact oracle.','Preserve refutation and inspect exceptional branches; repaired proof/source needs new variant.')
