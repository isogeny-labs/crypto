import time
START=time.monotonic()
from pathlib import Path
import json,random,hashlib
from .common import P,finish
from .kernel import fused,tiny_family
from research.unit_cofactor_20261117_v134b.kernel import embed,compress,scalar_reconstruct,word,accepted,original_readback
from research.precision_filtration_20261117_v132b.kernel import actual_family,evaluate_family
from research.correction_basis_checks_20261117_v128b.kernel import tiny,compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
out={'tiny_reference_assignments':0,'tiny_variants':[],'actual':None,'original_complete':False};error=None

def coeffwords(values,groups):return [word(values,g)for g in groups]
def digest(G,oo,rr,selectors,h,coeffs):return hashlib.sha256(json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr,'selector_readback':selectors,'cofactor':h,'coefficient_readback':coeffs},separators=(',',':')).encode()).hexdigest()
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);base,cols=tiny_family(2);G,oo,rr,meta=tiny(2,1);target=oo[-1]['wires'][0][0];truth=[];reference=set();slice_expected=set()
 for x in range(1<<G.n):
  old=G.evaluate(x);truth.append(old[target]);out['tiny_reference_assignments']+=1
  if not old[target]:slice_expected.add(x)
  if accepted(old,oo):reference.add((x,tuple(word(old,r)for r in rr)))
 for v in data['tiny_directions']:
  H,nn,roots,selectors,h,coeffs,stats=fused(base,cols,2,1,v,1,data['max_nodes']);found=set();slices=set();checks=0
  for z in range(1<<H.n):
   values=H.evaluate(z);theta=word(values,selectors);expected=scalar_reconstruct(z,1,v,truth[embed(z,1)]);assert theta==expected and truth[theta]==0;assert compress(theta^(v if theta>>1&1 else 0),1)==z;slices.add(theta);assert coeffwords(values,coeffs)==evaluate_family(base,cols,theta);old=G.evaluate(theta);assert [values[w]for row in nn for group in row['wires']for w in group]==[old[w]for row in oo for group in row['wires']for w in group];recovered=tuple(word(values,r)for r in roots);assert recovered==tuple(word(old,r)for r in rr)
   if accepted(values,nn):found.add((theta,recovered))
   checks+=1
  assert found==reference and slices==slice_expected;assert any(len(set(roots))<5 for _,roots in found);out['tiny_variants'].append({**stats,'direction_hex':hex(v),'exhaustive_assignments':checks,'exact_component_tuples':len(found),'exact_slice_count':len(slices),'tuple_sha256':hashlib.sha256(json.dumps(sorted(found),separators=(',',':')).encode()).hexdigest()})
 begin=time.monotonic();cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];cert=json.loads(Path(data['certificate']).read_bytes());prior=json.loads(Path(data['prior_profile']).read_bytes());assert cert['error']is None and prior['error']is None;assert data['direction_hex']==cert['out']['actual']['unit_directions_hex'][0];base,cols=actual_family(cfg,chart);v=int(data['direction_hex'],16);G,oo,rr,meta=compile_all(cfg,chart,8);assert hashlib.sha256(json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode()).hexdigest()==cert['out']['actual']['graph_sha256'];source_seconds=time.monotonic()-begin;stamp=time.monotonic();H,nn,roots,selectors,h,coeffs,stats=fused(base,cols,18,8,v,data['pivot'],data['max_nodes']);fusion_seconds=time.monotonic()-stamp;clauses=len(cnf(H,nn));out['actual']={**stats,'clauses':clauses,'prior_unit_clauses':prior['out']['actual']['new_clauses'],'prior_unit_variables':prior['out']['actual']['pruned_nodes'],'clause_ratio_to_prior_unit':clauses/prior['out']['actual']['new_clauses'],'clauses_removed':prior['out']['actual']['new_clauses']-clauses,'profile_admitted_for_changed_diagnostic':clauses<prior['out']['actual']['new_clauses'],'source_compile_seconds':source_seconds,'fusion_seconds':fusion_seconds,'coefficient_equivalence_checks':0,'constraint_root_equivalence_checks':0,'original_theta_q_readback_checks':0,'graph_sha256':digest(H,nn,roots,selectors,h,coeffs),'no_native_solve':True}
 assert all(int(a,16)&((1<<17)-1)==0 for a in stats['direction_polynomials_hex'])
 for _ in range(data['actual_random_checks']):
  z=rng.randrange(1<<H.n);values=H.evaluate(z);theta=word(values,selectors);x=embed(z,data['pivot']);target=oo[-1]['wires'][0][0];oldx=G.evaluate(x);assert theta==scalar_reconstruct(z,data['pivot'],v,oldx[target]);old=G.evaluate(theta);assert old[target]==0;assert coeffwords(values,coeffs)==evaluate_family(base,cols,theta);out['actual']['coefficient_equivalence_checks']+=1;assert [values[w]for row in nn for group in row['wires']for w in group]==[old[w]for row in oo for group in row['wires']for w in group];assert [word(values,r)for r in roots]==[word(old,r)for r in rr];out['actual']['constraint_root_equivalence_checks']+=1
  original=original_readback(chart,theta);independent=chart['particular']
  for j,col in enumerate(chart['columns']):
   if values[selectors[j]]:independent^=col
  assert original==independent and original>>76==0;out['actual']['original_theta_q_readback_checks']+=1
 out['actual']['profile_after_oracles_seconds']=time.monotonic()-begin;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
finish(out,error,START,'Exact affine coefficient-level unit fusion:complete small sets and actual whole-graph profile','If admitted,freeze a changed diagnostic on this new graph and reconstructed theta/q;otherwise derive a delayed-coordinate form before another native run;retain all original all-roots and matched-baseline obligations.')
