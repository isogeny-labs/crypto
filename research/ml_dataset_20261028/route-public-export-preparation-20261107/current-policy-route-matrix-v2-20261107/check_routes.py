"""Source-bound STATIC routing inventory; no solver execution or launch authority."""
from pathlib import Path
import json,hashlib,math,copy,argparse
B=Path(__file__).parent
T=B/'templates'
R=Path.cwd()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def route(p,f,n,m,s):
 if p.get('work_limit',100000)<=0:return ('pending_configuration','positive_work_required','mod.rs:430')
 if p.get('channels'):
  expected='f4' if p['channels']['schema'].endswith('/v5') else 'sat'
  return ('conditional_route','explicit_owned_profile','sat_original_owner.rs:108; owned_boolean_search_worker.rs:8; sat_search_worker.rs:6') if f in ['binary','koblitz'] and s==expected and (23<=n<=31 or 47<=n<=61) and 2<=m<=15 else ('pending_capability','owned_profile_excluded','same')
 if s in ['enumerate','bsgs','double-large-prime']:return ('conditional_route','bounded_group_or_relation_route','solvers.rs:1093-1102')
 if s=='f4-wide-dynamic-v1':
  if f!='prime' or not 33<=n<=61 or not 4<=m<=15:return ('pending_capability','dynamic_family_size_arity_excluded','solvers.rs:4209')
  return ('conditional_route','wide_dynamic','solvers.rs:4209') if p['degree_bound']==12 else ('pending_configuration','dynamic_requires_degree12','solvers.rs:4209')
 if s=='sat':return ('conditional_route','bounded_chain_sat','solvers.rs:1731') if f!='prime' else ('pending_capability','prime_sat_missing','solvers.rs:1738')
 if s=='weil':return ('encoding_only','selector_weil_not_solver','solvers.rs:1956; capabilities:1096') if f!='prime' and m==2 else ('pending_capability','weil_family_or_arity_excluded','solvers.rs:1956')
 if s=='root':return ('conditional_route','m2_root_and_lift','solvers.rs:3325') if m==2 else ('pending_capability','higher_arity_root_missing','solvers.rs:3325')
 if s=='pkm':return ('conditional_route','prime_kummer_m2_m3','solvers.rs:2310') if f=='prime' and n<=31 and m<=3 else ('pending_capability','pkm_family_modulus_or_arity_excluded','solvers.rs:2310')
 if s=='f6':return ('conditional_route','geometric_four_sum_not_groebner','solvers.rs:3251') if f!='prime' and m==4 else ('pending_capability','f6_family_or_arity_excluded','solvers.rs:3251')
 if s in ['f4','f5'] and f!='prime':
  if m==2:return ('conditional_route','boolean_matrix_m2','solvers.rs:1956')
  if m==3 and (not(23<=n<=31 or 47<=n<=61) or not 1<=p['degree_bound']<=6):return ('pending_configuration','m3_requires_supported_size_and_degree','solvers.rs:568')
  if m==3:return ('conditional_route','four_signed_two_x_m3','solvers.rs:568') if p['factor_base_size']==4 else ('pending_configuration','m3_requires_factor_base_size4','solvers.rs:568')
  return ('pending_capability','legacy_boolean_high_arity_missing','solvers.rs:1967')
 if s=='f5':return ('pending_capability','prime_f5_missing','solvers.rs:1956')
 if s=='f4':return ('conditional_route','prime_affine_or_supervised_f4','solvers.rs:3049') if n<=31 or m<=3 else ('pending_capability','ordinary_f4_wide_high_arity_missing_use_separate_dynamic_method','solvers.rs:1097,3049,4209')
 raise AssertionError((f,n,m,s))
def count(p):
 assert p['twists'] is True, 'only paired-twist frozen templates admitted'
 return math.prod([len(p[k]) for k in ['families','field_sizes','arities','solvers','factor_base_policies']]+[p[k] for k in ['curve_seeds','generators','keys']]+[2])
def main():
 global R
 parser=argparse.ArgumentParser(description='Verify only the hash-pinned campaign templates; no general capability proof.')
 parser.add_argument('--repo',type=Path,default=Path.cwd())
 parser.add_argument('--generate',action='store_true',help='Pre-freeze generation only; requires an external output directory.')
 parser.add_argument('--output-dir',type=Path)
 args=parser.parse_args();R=args.repo.resolve()
 if args.generate and args.output_dir is None:parser.error('--generate requires --output-dir')
 if args.output_dir and args.output_dir.resolve()==B.resolve():parser.error('refusing to overwrite frozen packet')
 if args.output_dir:args.output_dir.mkdir(parents=True,exist_ok=True)
 def emit(name,value):
  data=json.dumps(value,indent=2)+'\n'
  if args.generate:(args.output_dir/name).write_text(data)
  else:
   assert (B/name).read_text()==data,'frozen output differs: '+name
   if args.output_dir:(args.output_dir/name).write_text(data)
 # Regression assertions on configuration gates, not solver executions.
 probe={'degree_bound':3,'factor_base_size':8}
 assert route(probe,'prime',61,4,'f4-wide-dynamic-v1')[0]=='pending_configuration'
 assert route(dict(probe,degree_bound=12),'prime',61,4,'f4-wide-dynamic-v1')[0]=='conditional_route'
 assert route(probe,'binary',23,3,'f4')[0]=='pending_configuration'
 assert route(dict(probe,factor_base_size=4),'binary',23,3,'f4')[0]=='conditional_route'
 assert route(probe,'prime',61,2,'sat')[0]=='pending_capability'
 assert route(probe,'binary',23,2,'weil')[0]=='encoding_only'
 bindings=json.loads((B/'SOURCE_BINDINGS.json').read_text())
 for name,h in bindings['source_hashes'].items():assert digest(R/name)==h, 'source drift: '+name
 matrix=[];derived=[]
 for path in sorted(T.glob('*.json')):
  p=json.loads(path.read_text());assert digest(path)==bindings['template_hashes'][path.name]
  # Groups are single family/method/size-band with homogeneous arity admission.
  for f in p['families']:
   for s in p['solvers']:
    for ns in [[n for n in p['field_sizes'] if n<=31],[n for n in p['field_sizes'] if n>=47]]:
     if not ns:continue
     groups={}
     for m in p['arities']:
      rule=route(p,f,ns[0],m,s);assert all(route(p,f,n,m,s)==rule for n in ns);groups.setdefault(rule,[]).append(m)
     for (status,reason,source),ms in groups.items():
      q=copy.deepcopy(p);q.update(families=[f],solvers=[s],field_sizes=ns,arities=ms)
      row={'template':path.name,'family':f,'solver':s,'field_sizes':ns,'arities':ms,'status':status,'reason':reason,'source':source,'nominal_cells':count(q),'authorized_cells':0};matrix.append(row)
      if status in ['conditional_route','encoding_only']:derived.append((row,q))
 # Explicit corrected configuration blocks, preserving original templates.
 for path in sorted(T.glob('*methods*.json')):
  p=json.loads(path.read_text())
  for f in p['families']:
   if f=='prime':
    q=copy.deepcopy(p);q.update(families=[f],solvers=['f4-wide-dynamic-v1'],field_sizes=[n for n in p['field_sizes'] if n>=47],arities=list(range(4,16)),degree_bound=12)
    derived.append(({'template':path.name,'status':'corrected_configuration','reason':'explicit_wide_dynamic_degree12','authorized_cells':0},q))
   else:
    q=copy.deepcopy(p);q.update(families=[f],solvers=['f4','f5'],arities=[3],factor_base_size=4)
    derived.append(({'template':path.name,'status':'corrected_configuration','reason':'four_signed_two_x_m3_profile','authorized_cells':0},q))
 totals={}
 for row in matrix:totals[row['status']]=totals.get(row['status'],0)+row['nominal_cells']
 assert sum(totals.values())==sum(count(json.loads(p.read_text())) for p in T.glob('*.json'))
 output={'schema':'dataset-static-route-matrix/v1','source_commit':bindings['source_commit'],'statuses_do_not_prove_execution':True,'totals':totals,'rows':matrix}
 emit('MATRIX.json',output)
 plans=[]
 for i,(row,q) in enumerate(derived):
  assert q['protocol_sha256'] is None
  statuses={route(q,f,n,m,s)[0] for f in q['families'] for n in q['field_sizes'] for m in q['arities'] for s in q['solvers']}
  assert statuses<= {'conditional_route','encoding_only'},(row,statuses)
  plans.append({'id':f'eligible-block-{i:03}','derivation':row,'plan':q,'nominal_cells':count(q),'authorized_cells':0,'native_validation':'pending'})
 emit('ELIGIBLE_DRAFT_PLANS.json',{'schema':'dataset-static-derived-plans/v1','not_frozen_not_funded':True,'plans':plans})
 emit('PENDING_REQUIREMENTS.json',{'schema':'dataset-pending-route-requirements/v1','original_requested_scope_preserved':True,'rows':[r for r in matrix if r['status'].startswith('pending')],'universal_remaining_gates':['current assembled CLI validation','authentic original-cell validation and controlled execution','conditional instance/domain/capture/memory/work gates','matched controls and finite approved protocol','scientific corpus and complete trace/degree/cost evidence']})
 print(json.dumps({'source_bindings_verified':True,'template_bindings_verified':True,'rows':len(matrix),'derived_blocks':len(plans),'nominal_original_counts':totals,'derived_routes_statically_eligible':True}))
if __name__=='__main__':main()
