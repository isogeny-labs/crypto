import time
START=time.monotonic()
from pathlib import Path
import json,itertools,random,hashlib
from .common import P,finish
from .kernel import reduction,numeric,catalog,emit_catalog,build
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.delayed_block_20261117_v137b.kernel import row_action,coefficients
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys,image_representatives
from research.precision_filtration_20261117_v132b.kernel import filtration,evaluate_family,actual_family
from research.coefficient_fusion_20261117_v136b.kernel import tiny_family
from research.correction_basis_checks_20261117_v128b.kernel import tiny,compile_all
from research.unit_cofactor_20261117_v134b.kernel import word,accepted,original_readback
from research.residue_quotient_20261116_v99b.kernel import apply,basis,preimage
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
out={'matrix_rhs_oracles':0,'generic_catalog_state_checks':0,'tiny':None,'actual':None,'original_complete':False};error=None

def table_for(base,cols,K,d,ell,ns):
 early=filtration(cols,max(ns)-ell+1)['lower_columns'];reps=image_representatives(early,8);f=Arithmetic(2,[0,1]);records=[]
 for x in reps:
  F=evaluate_family(base,cols,x);rows=[0]*len(ns)
  for j,v in enumerate(K):
   q=jet(bitpolys(F),bitpolys(coefficients(cols,v)),d,max(ns),f)
   for i,n in enumerate(ns):rows[i]|=q[n-1]['tangent'][1]<<j
  records.append({'signature_hex':hex(apply(early,x)),'rows':rows})
 return records

def fingerprint(G,oo,rr,ss,zz,state,b,ff):return hashlib.sha256(json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr,'selector_readback':ss,'delayed_readback':zz,'state_readback':state,'cofactors':b,'coefficient_readback':ff},separators=(',',':')).encode()).hexdigest()
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);records=[]
 for code in range(512):
  rows=[code>>(3*i)&7 for i in range(3)];rec=reduction(rows,3);records.append(rec)
  for rhs in range(8):
   found=[]
   for t in range(8):
    z,valid=numeric(rec,rhs,t)
    if valid:found.append(z)
   assert len(found)==len(set(found));assert set(found)=={z for z in range(8)if row_action(rows,z)==rhs};out['matrix_rhs_oracles']+=1
 C=Circuit(9);ops=emit_catalog(C,list(range(2,11)),records,3,3);expected,slots,left=catalog(records,3,3)
 for code in range(512):
  values=C.evaluate(code);assert [values[w]for w in ops['flat']]==expected[code];out['generic_catalog_state_checks']+=1
 base,cols=tiny_family(2);K=data['tiny_K'];ns=data['tiny_precisions'];table=table_for(base,cols,K,2,data['tiny_ell'],ns);H,nn,roots,selectors,zz,states,bb,ff,stats=build(base,cols,2,data['tiny_count'],K,data['tiny_ell'],ns,table,data['max_early_rank'],data['max_nodes']);G,oo,rr,meta=tiny(2,data['tiny_count']);consumed=[i for i,r in enumerate(meta)if r['precision']in ns and r['coefficient']==1];reference=set();slice_expected=set()
 for theta in range(1<<G.n):
  values=G.evaluate(theta)
  if not any(values[oo[1]['wires'][0][i]]for i in consumed):slice_expected.add(theta)
  if accepted(values,oo):reference.add((theta,tuple(word(values,r)for r in rr)))
 found=set();slice_found=set();accepted_count=0;guard_count=0
 for z in range(1<<H.n):
  values=H.evaluate(z);theta=word(values,selectors);old=G.evaluate(theta);assert [word(values,g)for g in ff]==evaluate_family(base,cols,theta);assert [word(values,r)for r in roots]==[word(old,r)for r in rr];guards=not any(values[w]for row in nn[2:]for g in row['wires']for w in g)
  if guards:guard_count+=1;slice_found.add(theta);assert not any(old[oo[1]['wires'][0][i]]for i in consumed)
  if accepted(values,nn):found.add((theta,tuple(word(values,r)for r in roots)));accepted_count+=1
 assert found==reference and slice_found==slice_expected and accepted_count==len(found)and guard_count==len(slice_found);assert any(len(set(rs))<5 for _,rs in found);out['tiny']={**stats,'reference_assignments':1<<G.n,'compiled_assignments':1<<H.n,'exact_component_tuples':len(found),'exact_consumed_zero_slice':len(slice_found),'tuple_sha256':hashlib.sha256(json.dumps(sorted(found),separators=(',',':')).encode()).hexdigest()}
 begin=time.monotonic();cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];prior=json.loads(Path(data['rank_receipt']).read_bytes());assert prior['error']is None and prior['out']['actual']['complete'];a=prior['out']['actual'];assert hashlib.sha256(json.dumps(a['records'],separators=(',',':')).encode()).hexdigest()==a['matrix_table_sha256'];K=[int(v,16)for v in a['kernel_hex']];base,cols=actual_family(cfg,chart);H,nn,roots,selectors,zz,states,bb,ff,stats=build(base,cols,18,8,K,data['actual_ell'],data['actual_precisions'],a['records'],data['max_early_rank'],data['max_nodes']);assert stats['U_hex']==a['quotient_basis_hex'];clauses=len(cnf(H,nn));baseline=json.loads(Path(data['prior_profile']).read_bytes());assert baseline['error']is None;old_clauses=baseline['out']['actual']['clauses'];out['actual']={**stats,'clauses':clauses,'baseline_clauses':old_clauses,'clause_ratio':clauses/old_clauses,'profile_admitted_for_changed_diagnostic':clauses<=data['admission_clause_ratio']*old_clauses,'coefficient_checks':0,'root_unconsumed_checks':0,'guard_pass_samples':0,'inverse_readback_checks':0,'catalog_state_checks':0,'graph_sha256':fingerprint(H,nn,roots,selectors,zz,states,bb,ff),'no_native_solve':True};G,oo,rr,meta=compile_all(cfg,chart,8);consumed=[i for i,r in enumerate(meta)if r['precision']in data['actual_precisions']and r['coefficient']==1];T=[int(v,16)for v in stats['U_hex']+stats['K_hex']];inv,_=basis(T);ny=stats['quotient_inputs']
 # Independently verify the actual256-state emitted catalog too.
 C=Circuit(8);recs=[reduction(r['rows'],3)for r in a['records']];ops=emit_catalog(C,list(range(2,10)),recs,3,3);wanted,_,_=catalog(recs,3,3)
 for i in range(256):
  cv=C.evaluate(i);assert [cv[w]for w in ops['flat']]==wanted[i];out['actual']['catalog_state_checks']+=1
 early=filtration(cols,4)['lower_columns'];epiv,_=basis(early);evec=[epiv[k][0]for k in sorted(epiv)];einverse,_=basis(evec)
 for _ in range(data['actual_checks']):
  z=rng.randrange(1<<H.n);values=H.evaluate(z);theta=word(values,selectors);old=G.evaluate(theta);assert [word(values,g)for g in ff]==evaluate_family(base,cols,theta);out['actual']['coefficient_checks']+=1;assert [word(values,r)for r in roots]==[word(old,r)for r in rr];assert [values[w]for w in nn[0]['wires'][0]]==[old[w]for w in oo[0]['wires'][0]]
  for i in range(len(meta)):
   if i not in consumed:assert values[nn[1]['wires'][0][i]]==old[oo[1]['wires'][0][i]]
  guards=not any(values[w]for row in nn[2:]for group in row['wires']for w in group);canonical=not any(values[w]for group in nn[-1]['wires']for w in group)
  if guards:assert not any(old[oo[1]['wires'][0][i]]for i in consumed);out['actual']['guard_pass_samples']+=1
  assert accepted(values,nn)==(accepted(old,oo)and canonical);out['actual']['root_unconsumed_checks']+=1;coords=preimage(theta,inv);assert word(values,states)==preimage(apply(early,theta),einverse);assert coords&((1<<ny)-1)==z&((1<<ny)-1);assert coords>>ny==word(values,zz);original=original_readback(chart,theta);independent=chart['particular']
  for i,col in enumerate(chart['columns']):
   if values[selectors[i]]:independent^=col
  assert original==independent and original>>76==0;out['actual']['inverse_readback_checks']+=1
 out['actual']['profile_after_oracles_seconds']=time.monotonic()-begin;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
finish(out,error,START,'Guarded delayed-block reconstruction with every singular compatibility/free-slot branch and whole actual profile','If admitted,freeze changed guarded-block native diagnostic with complete nonlinear readback/recovery;otherwise reduce table/pivot/cofactor overhead before any longer or identical execution.')
