from pathlib import Path
import json,time,random,hashlib
from .common import P,finish
from .kernel import transform,embed,compress,scalar_reconstruct,original_readback,accepted,word
from research.correction_basis_checks_20261117_v128b.kernel import tiny,compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
START=time.monotonic();out={'tiny_reference_assignments':0,'tiny_variants':[],'actual':None,'original_complete':False};error=None

def graphhash(G,oo,rr):return hashlib.sha256(json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode()).hexdigest()
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);G,oo,rr,meta=tiny(data['tiny_d'],1);target=oo[-1]['wires'][0][0];truth=[];oldmodels=set();slice_expected=set()
 for x in range(1<<G.n):
  values=G.evaluate(x);truth.append(values[target]);roots=tuple(word(values,r)for r in rr)
  if not values[target]:slice_expected.add(x)
  if accepted(values,oo):oldmodels.add((x,roots))
  out['tiny_reference_assignments']+=1
 assert (0,tuple(word(G.evaluate(0),r)for r in rr))in oldmodels;assert len(set(word(G.evaluate(0),r)for r in rr))<5
 for v in data['tiny_directions']:
  p=(v&-v).bit_length()-1;assert all(truth[x]^truth[x^v]==1 for x in range(1<<G.n));H,nn,roots,selectors,h,stats=transform(G,oo,rr,target,v,p,data['max_nodes']);models=set();slice_found=set();checks=0
  for z in range(1<<H.n):
   values=H.evaluate(z);x=word(values,selectors);expected=scalar_reconstruct(z,p,v,truth[embed(z,p)]);assert x==expected and truth[x]==0;assert compress(x^(v if x>>p&1 else 0),p)==z;slice_found.add(x);old=G.evaluate(x);assert [values[w]for row in nn for field in row['wires']for w in field]==[old[w]for row in oo for field in row['wires']for w in field];recovered=tuple(word(values,r)for r in roots);assert recovered==tuple(word(old,r)for r in rr)
   if accepted(values,nn):models.add((x,recovered))
   checks+=1
  assert models==oldmodels and slice_found==slice_expected;out['tiny_variants'].append({**stats,'exhaustive_assignments':checks,'exact_slice_count':len(slice_found),'exact_component_tuple_count':len(models),'component_tuple_sha256':hashlib.sha256(json.dumps(sorted(models),separators=(',',':')).encode()).hexdigest()})
 actual_start=time.monotonic();cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];cert=json.loads(Path(data['certificate']).read_bytes());assert cert['error']is None and cert['out']['actual']['tangent_rows']==[4,6];v=int(data['direction_hex'],16);assert data['direction_hex']==cert['out']['actual']['unit_directions_hex'][0];G,oo,rr,meta=compile_all(cfg,chart,8);source_seconds=time.monotonic()-actual_start;source_hash=graphhash(G,oo,rr);assert source_hash==data['source_graph_sha256']==cert['out']['actual']['graph_sha256'];target=oo[-1]['wires'][0][0];source_clauses=len(cnf(G,oo));begin=time.monotonic();H,nn,roots,selectors,h,stats=transform(G,oo,rr,target,v,data['pivot'],data['max_nodes']);transform_seconds=time.monotonic()-begin;new_clauses=len(cnf(H,nn));out['actual']={**stats,'source_graph_sha256':source_hash,'source_variables':len(G.nodes)-2,'source_clauses':source_clauses,'new_clauses':new_clauses,'clause_ratio':new_clauses/source_clauses,'profile_admitted_for_changed_diagnostic':new_clauses/source_clauses<=data['max_clause_ratio_for_next_diagnostic'],'source_compile_seconds':source_seconds,'transform_seconds':transform_seconds,'equivalence_checks':0,'original_theta_q_readback_checks':0,'no_native_solve':True}
 for _ in range(data['actual_random_checks']):
  z=rng.randrange(1<<H.n);values=H.evaluate(z);x=word(values,selectors);base=embed(z,data['pivot']);base_values=G.evaluate(base);assert x==scalar_reconstruct(z,data['pivot'],v,base_values[target]);old=G.evaluate(x);assert old[target]==0;assert compress(x^(v if x>>data['pivot']&1 else 0),data['pivot'])==z;assert [values[w]for row in nn for field in row['wires']for w in field]==[old[w]for row in oo for field in row['wires']for w in field];assert [word(values,r)for r in roots]==[word(old,r)for r in rr];out['actual']['equivalence_checks']+=1
  theta=original_readback(chart,x);independent=chart['particular']
  for i,col in enumerate(chart['columns']):
   if values[selectors[i]]:independent^=col
  assert theta==independent and theta>>76==0 and theta>>73==independent>>73;out['actual']['original_theta_q_readback_checks']+=1
 blob=json.dumps({'nodes':H.nodes,'outputs':nn,'roots':roots,'selector_readback':selectors,'cofactor':h},separators=(',',':')).encode();out['actual']['graph_sha256']=hashlib.sha256(blob).hexdigest();out['actual']['profile_after_oracles_seconds']=time.monotonic()-actual_start;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
finish(out,error,START,'Exact unit-cofactor compiler complete small-set oracles and actual total graph profile','If admitted,freeze a changed hard diagnostic with reconstructed selector/quotient readback;otherwise derive the delayed-basis structured cofactor to reduce duplicate work before another native run.')
