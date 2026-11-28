from .common import P,finish
from .kernel import Arithmetic,predicate,roots_product,literal_split
import json,time,random,itertools
START=time.monotonic();out={'tiny':[],'large':[],'repeated_shortcut_counterexamples':0,'original_complete':False,'original_exact_root_count':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
 for spec in d['tiny']:
  f=Arithmetic(spec['p'],spec['mod']);inputs=itertools.product(range(f.q),repeat=5)if spec['exhaustive']else([rng.randrange(f.q)for _ in range(5)]for _ in range(64));cases=0;yes=0
  for coeff in inputs:
   H=list(coeff)+[1];want=literal_split(H,f);v,L=predicate(H,f);assert v==want;cases+=1;yes+=int(want)
  for a in range(f.q):
   H=roots_product([a]*5,f);v,L=predicate(H,f);assert v and literal_split(H,f)
   if L:out['repeated_shortcut_counterexamples']+=1
   if f.p==2:assert predicate(H,f,5)[0]==v
  out['tiny'].append({'p':f.p,'n':f.n,'q':f.q,'cases':cases,'split_count':yes,'repeated_controls':f.q})
 for spec in d['large']:
  f=Arithmetic(spec['p'],spec['mod']);positive=roots_product([0,1,1,2,3],f);assert predicate(positive,f)[0]
  if f.p==2:
   c=next(a for a in range(1,256)if f.trace(a)==1);quad=[c,1,1]
  else:
   delta=next(a for a in range(2,256)if f.power(a,(f.q-1)//2)==f.neg(1));quad=[f.neg(delta),0,1]
  negative=[0,0,0]+quad;assert not predicate(negative,f)[0];out['large'].append({'id':spec['id'],'positive':positive,'negative':negative,'positive_accepted':True,'negative_rejected':True})
 finish(out,None,START,'Exact monicquintic whole-field splitting predicate retainsmultiplicity acrossbinary,oddextensionsandNISTprimes.','Compilelocal binaryconstraints onsharedparameterbits;certificate/componentcoverage isnot rootfindingsearch orrelease.')
except Exception as e:finish(out,repr(e),START,'Localnilpotencyoracle.','Preservefailureandreview characteristic/repeatedfactor scope;repairnewimmutablevariant only.')
