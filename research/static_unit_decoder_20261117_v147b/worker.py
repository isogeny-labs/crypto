import time
START=time.monotonic()
import json,random,hashlib,itertools
from pathlib import Path
from .common import P,finish
from .kernel import BudgetCircuit,Bit,decoder,planes,patterns,prefix_rhs,final_product,affine,prune,cnf
from research.typed_cell_planes_20261117_v145b.kernel import context
from research.delayed_block_20261117_v137b.kernel import complement,coefficients,solve,solutions,row_action
from research.precision_filtration_20261117_v132b.kernel import evaluate_family,filtration
from research.residue_quotient_20261116_v99b.kernel import basis,apply,preimage,pmul
from research.shifted_tangent_20261117_v144b.kernel import packed
from research.simple_prefix_lift_20261116_v117b.kernel import normalize
out={'matrix_rhs_oracles':0,'tiny_original_coefficient_assignments':0,'tiny_exact_coefficient_tuples':None,'actual':None,'original_complete':False,'all_registered_contexts_complete':False};error=None

def product(roots):
 a=[1]
 for root in roots:
  b=[0]*(len(a)+1)
  for j,v in enumerate(a):b[j]^=pmul(v,root);b[j+1]^=v
  a=b
 return a

def witness(rec):
 state=0;v=rec['root']
 while v>1:
  j,lo,hi=rec['nodes'][v]
  if lo:v=lo
  else:state|=1<<j;v=hi
 assert v==1;return state
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);cap=data['node_cap']
 C=BudgetCircuit(10,cap);R=[[Bit(C,1),Bit(C,2),Bit(C,3)],[Bit(C,4),Bit(C,5),Bit(C,6)]];b=[Bit(C,7+i)for i in range(2)];Z=[Bit(C,9+i)for i in range(3)];x,valid,admitted,_=decoder(R,b,Z,Bit(C,1));inputs,full=patterns(10);v=planes(C,inputs,full)
 for code in range(32):
  rows=[1|((code&3)<<1),code>>2]
  for rhs in range(4):
   got=[]
   for z in range(8):
    state=code|(rhs<<5)|(z<<7);assert v[admitted.v]>>state&1
    if v[valid.v]>>state&1:got.append(sum(((v[a.v]>>state)&1)<<j for j,a in enumerate(x)))
   expected={a for a in range(8)if row_action(rows,a)==rhs};assert len(got)==len(set(got))and set(got)==expected;out['matrix_rhs_oracles']+=1
 C=BudgetCircuit(15,cap);F=[];offset=2
 for j in range(5):
  degree=5-j;F.append([int(j>=2)]+list(range(offset,offset+degree)));offset+=degree
 F.append([1]);checks,roots=final_product(C,F,2);inputs,full=patterns(15);v=planes(C,inputs,full);actual=full
 for a in checks:actual&=full^v[a]
 expected=set()
 for raw in itertools.product([0,1],repeat=2):
  for high in itertools.product([2,3],repeat=3):
   H=normalize(product(raw+high),2);code=0;offset=0
   for j in range(5):
    for k in range(1,6-j):code|=((H[j]>>k)&1)<<offset;offset+=1
   expected.add(code)
 assert actual==sum(1<<a for a in expected);out['tiny_original_coefficient_assignments']=1<<15;out['tiny_exact_coefficient_tuples']=len(expected)
 cfg=json.loads(Path(data['cfg']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];base,cols,images,selectors,Ds=context(cfg,chart,int('25bc29bd1e36b336',16));K=[int('25bc29bd1e36b336',16)]
 for a in filtration(cols,16)['kernel']:
  if len(basis(K+[a])[0])>len(K):K.append(a)
 U=complement(K,63);UC=[coefficients(cols,a)for a in U];census=json.loads(Path(data['census']).read_bytes());assert census['error']is None and census['out']['complete_early_census']and all(census['out']['rank_distribution'].get(str(r),0)==0 for r in [0,1]);build=time.monotonic();C=BudgetCircuit(61,cap);q=list(range(2,58));Z=[Bit(C,58+i)for i in range(5)];Fq=affine(C,base,UC,q,18);rhs0=prefix_rhs(C,Fq,18,23);rhs=[Bit(C,rhs0[k-1][1])for k in range(18,24)]
 from research.typed_cell_planes_20261117_v145b.kernel import plane_rows
 R=plane_rows([[Bit(C,a)for a in row]for row in Fq],Ds,Bit(C,1));assert census['out']['plane_sha256'][0][0]==hashlib.sha256(b'\xff'*(1<<21)).hexdigest(),'unit_certificate_hash';R[0][0]=Bit(C,1);before_decoder=len(C.nodes);x,valid,admitted,ranks=decoder(R,rhs,Z,Bit(C,1));after_decoder=len(C.nodes);reconstructed=[]
 for j,row in enumerate(Fq):
  values=[]
  for k,a in enumerate(row):
   wire=a
   for lane,D in enumerate(Ds):
    if D[j]>>k&1:wire=C.xor(wire,x[lane].v)
   values.append(wire)
  reconstructed.append(values)
 selector=[]
 for j in range(63):
  wire=0
  for a,z in zip(U,q):
   if a>>j&1:wire=C.xor(wire,z)
  for a,z in zip(K,x):
   if a>>j&1:wire=C.xor(wire,z.v)
  selector.append(wire)
 checks,roots=final_product(C,reconstructed,18);extra=prefix_rhs(C,reconstructed,18,25);checks+=[extra[k-1][1]for k in range(18,26)]+[C.op('not',valid.v)];assert time.monotonic()-START<65,'formulationguard';G,outputs=prune(C,[{'exact_raw_source_product':True,'wires':[checks]},{'recovery':True,'wires':[selector]+roots}]);clause_count=len(cnf(G,outputs[:1]));graph_seconds=time.monotonic()-build;fingerprint=hashlib.sha256(json.dumps({'nodes':G.nodes,'outputs':outputs},separators=(',',':')).encode()).hexdigest()
 # Complete fixed-quotient fibers,including both singular strata.
 guard=json.loads(Path(data['guards']).read_bytes());E=filtration(cols,8)['lower_columns'];EU=[apply(E,a)for a in U];piv,_=basis(EU);cases=[0]
 for rec in guard['out']['strata']:
  s=witness(rec);cases.append(preimage(apply(images,s),piv))
 while len(cases)<data['actual_cases']:
  a=rng.getrandbits(56)
  if a not in cases:cases.append(a)
 total=len(cases)*32;full=(1<<total)-1;input_planes=[]
 for j in range(56):input_planes.append(sum(((a>>j)&1)<<(i*32+z)for i,a in enumerate(cases)for z in range(32)))
 for j in range(5):input_planes.append(sum(((z>>j)&1)<<(i*32+z)for i in range(len(cases))for z in range(32)))
 values=planes(C,input_planes,full);satisfying=full
 for a in checks:satisfying&=full^values[a]
 tuples=0;accepted_products=0
 for i,a in enumerate(cases):
  F=evaluate_family(base,cols,apply(U,a));r=packed(F,Ds,18,16,23)['correction_masks'];rows=[r[k-16][1]for k in range(18,24)];from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys
  from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
  audit=jet(bitpolys(F),bitpolys([0]*6),18,23,Arithmetic(2,[0,1]));b=sum(audit[k-1]['correction'][1]<<j for j,k in enumerate(range(18,24)));reference=solutions(solve(rows,b,7));got=[]
  for z in range(32):
   state=i*32+z;assert values[admitted.v]>>state&1
   if values[valid.v]>>state&1:
    delayed=sum(((values[t.v]>>state)&1)<<j for j,t in enumerate(x));got.append(delayed);source=sum(((values[t]>>state)&1)<<j for j,t in enumerate(selector));assert source==apply(U,a)^apply(K,delayed)
   if satisfying>>state&1:
    source=sum(((values[t]>>state)&1)<<j for j,t in enumerate(selector));H=normalize(evaluate_family(base,cols,source),18);raw=[sum(((values[t]>>state)&1)<<j for j,t in enumerate(root))for root in roots];assert product(raw)==H;accepted_products+=1
  assert len(got)==len(set(got))and set(got)==reference;tuples+=len(got)
 out['actual']={'scope':'fixed conditional k3source-polynomial product,not original full roots','inputs':61,'quotient_inputs':56,'free_slots':5,'unpruned_nodes':len(C.nodes),'variables':len(G.nodes)-2,'clauses':clause_count,'old_C138_clauses':data['old_clauses'],'clause_ratio':clause_count/data['old_clauses'],'before_decoder_nodes':before_decoder,'after_decoder_nodes':after_decoder,'complete_quotient_cases':len(cases),'free_assignments_checked':total,'exact_necessary_fiber_tuples':tuples,'accepted_raw_products':accepted_products,'graph_build_seconds':graph_seconds,'graph_sha256':fingerprint,'development_cost_target_met':clause_count<=data['development_clause_limit'],'certified_static_first_pivot':True,'full_graph_intermediate_retained':False};out['all_registered_contexts_complete']=True;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
import lzma
graph={'status':'not_constructed'}if out['actual']is None else {'status':'complete','n':G.n,'nodes':G.nodes,'outputs':outputs,'graph_sha256':fingerprint}
graph_bytes=lzma.compress(json.dumps(graph,separators=(',',':')).encode(),preset=9)
with(P/'actual_graph.json.xz').open('xb')as graph_file:graph_file.write(graph_bytes)
out['graph_artifact_sha256']=hashlib.sha256(graph_bytes).hexdigest()
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Singularity-safeK16decoder with exact source-root-product graph and independent complete component oracles','If graphcosttargetpasses,freeze original recovery and a changed bounded native diagnostic;otherwise redesign symbolic decoder/pivotsharing using measured costs. No circuit-size result certifies matched hard performance.')
