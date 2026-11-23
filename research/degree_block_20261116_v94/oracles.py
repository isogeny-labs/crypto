import json,sys,time,copy,random
from .common import P,finish
from .kernel import preprocess,residual_roots
from .verifier import verify,evaluate
from research.source_guards_20261116_v93.kernel import polynomial
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
START=time.monotonic();out={'tiny':[],'actual':[],'random_mq':[],'tamper_rejections':0};error=None
def rows(c):
 p=polynomial(c['n'],c['mod'],c['d'],0,c['u'],0,0,0,c['B'])
 return [sorted(m for m,a in p.items()if a>>j&1)for j in range(c['n'])]
try:
 d=json.loads((P/'DATA.json').read_text())
 for n,mod,dim in d['tiny']:
  f=Field(2,mod)
  for B in[1,2]:
   for u in range(1<<n):
    c={'n':n,'mod':mod,'d':dim,'u':u,'B':B};eq=rows(c);r=preprocess(eq,2*dim);assert verify(eq,2*dim,r);roots=residual_roots(r)
    expected=[x|(y<<dim)for y in range(1<<dim)for x in range(1<<dim)if s3(x,y,u,0,B,f)==0];assert roots==expected
    out['tiny'].append({'n':n,'d':dim,'B':B,'u':u,'root_count':len(roots),'residual_bits':r['n'],'unsat':r['unsat']})
 rng=random.Random(941611)
 for i in range(80):
  n=1+i%7;mon=[0]+[1<<j for j in range(n)]+[(1<<j)|(1<<k)for j in range(n)for k in range(j)]
  eq=[[m for m in mon if rng.randrange(4)==0]for _ in range(1+i%10)];r=preprocess(eq,n);assert verify(eq,n,r)
  expected=[z for z in range(1<<n)if all(evaluate(p,z)==0 for p in eq)];assert residual_roots(r)==expected;out['random_mq'].append({'n':n,'roots':len(expected)})
 for c in d['cases']:
  eq=rows(c);r=preprocess(eq,2*c['d']);assert verify(eq,2*c['d'],r)
  record={'id':c['id'],'certificate':r,'residual_bits':r['n'],'residual_equations':len(r['polys'])}
  if r['n']<=d['oracle_residual_cap']:record['roots']=residual_roots(r,d['oracle_residual_cap'])
  else:record['roots']=None
  out['actual'].append(record)
 # Fault injection uses a known nontrivial stage, not an assumed fixture outcome.
 for record in out['actual']:
  r=record['certificate'];c=next(c for c in d['cases']if c['id']==record['id']);eq=rows(c)
  if r['stages']and not r['unsat']:
   bad=copy.deepcopy(r);bad['maps'][0]=[0]if bad['maps'][0]!=[0]else[]
   try:verify(eq,2*c['d'],bad)
   except AssertionError:out['tamper_rejections']+=1
   else:raise AssertionError('tampered certificate accepted')
 finish(out,None,START,'Independent literal S3 and random MQ oracles, full substitution certificates.','Measure the changed preprocessed baseline; generic pair cases may be easy after linear elimination.')
except Exception as e:finish(out,repr(e),START,'Certificate/oracle diagnostic.','Preserve failed variant; repair only in a new immutable variant.')
