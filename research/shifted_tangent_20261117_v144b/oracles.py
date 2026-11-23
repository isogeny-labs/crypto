import time
START=time.monotonic()
import json,random,itertools
from .common import P,finish
from .kernel import packed
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys
from research.field_correction_basis_20261117_v128c.kernel import audited_lift
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.delayed_block_20261117_v137b.kernel import solve,solutions,row_action
out={'random_cases':0,'finite_difference_sets':0,'full_jet_lanes':0,'matrix_rhs_oracles':0,'boundary_checks':0,'all_registered_contexts_complete':False,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);f=Arithmetic(2,[0,1])
 for sample in range(data['small_random_cases']):
  d=2+sample%3;m=d-1;ell=2+sample%min(3,3*m-1);n=min(ell+2,2*ell-1,3*m);initial=[0,0,1,1,1,1];F=[initial[j]^(rng.getrandbits((5-j)*m)<<1)for j in range(6)];Ds=[[rng.getrandbits(max(0,(5-j)*m-ell+1))<<ell for j in range(6)]for _ in range(3)];Ds[0]=[0]*6 if sample%5==0 else Ds[0];q=packed(F,Ds,d,ell,n)['correction_masks'];base=audited_lift(bitpolys(F),d,f)['audit']
  for lane,D in enumerate(Ds):
   ref=jet(bitpolys(F),bitpolys(D),d,n,f)
   for k in range(ell,n+1):assert [(a>>lane)&1 for a in q[k-ell]]==ref[k-1]['tangent']
   out['full_jet_lanes']+=1
  for mask in range(8):
   G=[a for a in F]
   for lane,D in enumerate(Ds):
    if mask>>lane&1:G=[a^b for a,b in zip(G,D)]
   lifted=audited_lift(bitpolys(G),d,f)['audit']
   for k in range(ell,n+1):
    expected=[(a&mask).bit_count()%2 for a in q[k-ell]];actual=[a^b for a,b in zip(lifted[k-1]['correction'],base[k-1]['correction'])];assert actual==expected
   out['finite_difference_sets']+=1
  out['random_cases']+=1
 F=[0,0,1,1,1,1];D=[0,2,0,0,0,0];q=packed(F,[D],2,1,2)['correction_masks'];ref=jet(bitpolys(F),bitpolys(D),2,2,f);assert q[1][1]==ref[1]['tangent'][1]==0;G=[a^b for a,b in zip(F,D)];old=audited_lift(bitpolys(F),2,f)['audit'];new=audited_lift(bitpolys(G),2,f)['audit'];assert old[1]['correction'][1]^new[1]['correction'][1]==1;out['boundary_checks']+=1
 for sample in range(data['matrix_samples']):
  rows=[0]*6 if sample==0 else [1]*6 if sample==1 else [rng.randrange(128)for _ in range(6)]
  for rhs in range(64):assert solutions(solve(rows,rhs,7))=={x for x in range(128)if row_action(rows,x)==rhs};out['matrix_rhs_oracles']+=1
 out['all_registered_contexts_complete']=True;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Shifted packed tangent versus full jets,independent product-lift finite differences and singular rectangular matrix oracles','If passed,run the separately frozen actualK16profile;otherwise preserve counterexample and repair a changed recurrence/clipping assumption before further execution.')
