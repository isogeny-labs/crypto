from itertools import product
import json,time,hashlib
from .common import P,finish
from .codec import field,encode,evaluate
from .binary import anf,evaluate as beval,decode
from .oracle import roots
from .block import solve
from research.twist_three_isogeny_20261116_v86b.curve import scale
START=time.monotonic();out={'small':[],'actual_fields':[],'complete':False,'qualified_speedup':None};error=None
try:
 d=json.load(open(P/'DATA.json'))
 for c in d['cases']:
  f=field(c);g=encode(c);want=roots(c);got=set()
  for xs in product(c['D'],repeat=c['arity']):
   for us in product(range(f.q),repeat=c['arity']-2):
    row=xs+us
    if all(evaluate(e,row,f)==0 for e in g['equations']):got.add(row)
  assert got==want;block,stats=solve(c);assert set(block)==want
  for x in range(f.q):
   for j in range(c['arity']):
    e=g['equations'][c['arity']-1+j];row=[0]*len(g['variables']);row[j]=x;assert (evaluate(e,row,f)==0)==(x in c['D'])
  booleans=None
  if f.p==2:
   b=anf(g);booleans=set();n=b['n_vars'];assert n<=16
   for mask in range(1<<n):
    if all(beval(e,mask)==0 for e in b['equations']):booleans.add(decode(mask,f.n,len(g['variables'])))
   assert booleans==want
  out['small'].append({'id':c['id'],'field':g['field'],'arity':c['arity'],'root_count':len(want),'original_equations':len(g['equations']),'original_terms':sum(map(len,g['equations'])),'boolean_variables':None if f.p!=2 else b['n_vars'],'boolean_terms':None if f.p!=2 else sum(map(len,b['equations'])),'root_sha256':hashlib.sha256(json.dumps(sorted(want)).encode()).hexdigest(),'block_stats':stats})
 presets=json.load(open(P/'NIST.json'))['curves']
 for x in presets:
  p=int(x['p_hex'],16);A=int(x['a_hex'],16);B=int(x['b_hex'],16);G=(int(x['gx_hex'],16),int(x['gy_hex'],16));U=scale(2,G,p,A);Q=scale(3,G,p,A);c={'id':x['name']+'_codec_fixture','p':p,'mod':[0,1],'A':A,'B':B,'D':[G[0]],'arity':3,'z':Q[0]};g=encode(c);row=(G[0],G[0],G[0],U[0]);assert all(evaluate(e,row,field(c))==0 for e in g['equations']);name=x['name']+'_encoded.json';(P/name).write_text(json.dumps(g,separators=(',',':')));out['actual_fields'].append({'id':c['id'],'verified_fixture_roots':1,'completeness_claim':False,'encoded_sha256':hashlib.sha256((P/name).read_bytes()).hexdigest()})
 hashes=[]
 for A in[0,1]:
  mod=[0]*84
  for i in[0,1,2,45,83]:mod[i]=1
  c={'id':f'GF83_A{A}_codec_fixture','p':2,'mod':mod,'A':A,'B':1,'D':[1],'arity':3,'z':1};g=encode(c);assert all(evaluate(e,(1,1,1,0),field(c))==0 for e in g['equations']);name=f'GF83_A{A}_encoded.json';(P/name).write_text(json.dumps(g,separators=(',',':')));h=hashlib.sha256(json.dumps(g['equations'],sort_keys=True).encode()).hexdigest();hashes.append(h);out['actual_fields'].append({'id':c['id'],'verified_fixture_roots':1,'completeness_claim':False,'equations_sha256':h,'encoded_sha256':hashlib.sha256((P/name).read_bytes()).hexdigest()})
 assert hashes[0]==hashes[1];out['identical_binary_systems_not_distinct_hard_instances']=True;out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Exact original sparse polynomial encoder,Boolean descent and backward block solver match independent exhaustive root sets.','Run matched genuine native F4/F5 cold calibrations from originalfield inputs;retain unsupportedF6 and allhard-field gaps.')
