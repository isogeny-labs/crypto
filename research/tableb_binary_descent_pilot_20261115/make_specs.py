import json,sys
# usage: make_specs.py <bits> <targets> <dims comma> <solver> <out> [rounds]
bits,T,dims,solver,out=sys.argv[1:6]; rounds=int(sys.argv[6]) if len(sys.argv)>6 else 1
src=json.load(open(f'../ic_tableb_binary_ksum_20261115/specs/B{bits}-r1.json'))
inc={'21':'inc-n62','24':'inc-n67'}[bits]
keep=['rho-neg','rho-neg-aa',inc,'ksum5']
arms=[a for a in src['arms'] if a['name'] in keep]
for d in [x for x in dims.split(',') if x]:
    p={'factor_base':f'binary-subspace:dimension={d}','oracle':'descent-algebraic:m=3','solver':solver,'max_trials':'13528399621'}
    arms.append({'name':f'desc3-D{d}','role':'candidate','method':{'id':'ic.pipeline','params':p}})
src['arms']=arms
src['workloads']['targets_per_curve']=int(T)
src['label']=f'tableb_binary_descent_pilot_20261115 n{bits} (pilot, not data)'
src['description']='Pilot: descent-algebraic m=3 vs ksum5 vs tuned pair-table incumbent. Public synthetic curve; no security claim.'
src['measurement'].update({'rounds':rounds,'warmup':0})
src['measurement']['seed']=20261115900+int(bits)
src['workloads']['target_seed']=20261115800+int(bits)
json.dump(src,open(out,'w'),indent=1)
