from pathlib import Path
import json
from orbit import *
root=Path(__file__).resolve().parent;s=json.loads((root/'scope.json').read_text());secret=s['public_control_secret'];Q=old.mul(secret,G)
coeffs=[(1+17*i,3+11*i) for i in range(16)]
def point(a,b):return old.add(old.mul(a%r,G),old.mul(b%r,Q))
table5=[encode(point(a,b),5) for a,b in coeffs]
rows=[]
for B in s['batch_sizes']:
 for m,count,mode in [(5,B,'cached'),(1,B,'same_walk_count'),(1,3*B,'same_raw_state_bytes')]:
  table=table5 if m==5 else [[T[0]] for T in table5];states=[];labels=[]
  for i in range(count):
   a,b=1+101*i,7+103*i;T,j,sign=canonical(encode(point(a,b),m),Field());states.append(T);labels.append(label(a,b,j,sign))
  initial={'states':states,'labels':labels};steps=[];seen=[{key(T[0]):0} for T in states];repeats=[]
  for step in range(s['steps_per_walk']):
   F=Field();indices=[partition(T[0]) for T in states];pairs=[(P,U) for T,idx in zip(states,indices) for P,U in zip(T,table[idx])]
   sums,branches=add_many(pairs,F);addition_cost=F.cost.copy();nextstates=[];nextlabels=[];orientations=[]
   for i,idx in enumerate(indices):
    T,j,sign=canonical(sums[i*m:(i+1)*m],F);a,b=labels[i];c,d=coeffs[idx];L=label((a+c)%r,(b+d)%r,j,sign)
    assert T[0]==point(*L)
    k=key(T[0])
    if k in seen[i]:repeats.append({'walker':i,'step':step+1,'previous':seen[i][k],'length':step+1-seen[i][k]})
    else:seen[i][k]=step+1
    nextstates.append(T);nextlabels.append(L);orientations.append([j,sign])
   states,labels=nextstates,nextlabels
   steps.append({'indices':indices,'states':states,'labels':labels,'orientations':orientations,'branches':branches,'addition_cost':addition_cost,'total_cost':F.cost})
  rows.append({'B':B,'arity':m,'walkers':count,'mode':mode,'initial':initial,'steps':steps,'repeats':repeats})
  print('WALK',B,mode,'repeat_records',len(repeats),'first_add_cost',steps[0]['addition_cost'],flush=True)
controls=[]
for j in range(5):
 for sign in [1,-1]:
  z=sign*pow(mu,j,r)%r;a,b=17,29;d=31;c=(z*(a+b*secret)-d*secret)%r
  L=[];canon=[]
  for aa,bb in [(a,b),(c,d)]:
   T,rot,sg=canonical(encode(point(aa,bb),5),Field());canon.append(T);L.append(label(aa,bb,rot,sg))
  assert canon[0]==canon[1];recovered=recover(*L);assert recovered==secret and old.mul(recovered,G)==Q
  controls.append({'j':j,'sign':sign,'input_labels':[[a,b],[c,d]],'canonical_labels':L,'canonical_point':canon[0][0],'recovered':recovered})
assert recover([1,2],[1,2]) is None
branchpoints=[(None,G),(G,None),(G,old.neg(G)),(G,G),(G,old.mul(2,G))];F=Field();outputs,branches=add_many(branchpoints,F)
assert outputs==[old.add(P,Q) for P,Q in branchpoints]
out={'all_checks_pass':True,'p':str(p),'r':str(r),'mu':str(mu),'G':G,'Q':Q,'table_coefficients':coeffs,'table5':table5,'walks':rows,'planted_collision_controls':controls,'zero_denominator_control':True,'branch_control':{'pairs':branchpoints,'outputs':outputs,'branches':branches,'cost':F.cost},'claim_scope':'Deterministic labelled mechanics and planted recovery; no unknown-key search or mixing experiment'}
(root/'certificate.json').write_text(json.dumps(out,indent=2)+'\n')
