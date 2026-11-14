"""Exact source-coupled bounds; no scientific work executes on import."""
from itertools import product, combinations_with_replacement
from math import comb, prod
from research.curved_blocks_20261109_v6.worker import curve
from research.source_coupled_bernstein_20261112_v1.worker import bernstein_interval

def generic27(params,M):
 # General tensor power-to-Bernstein conversion, independent of the closed10 formula.
 a,B,C,g,d=params
 terms={(0,0,0):C,(1,1,1):-d*M**3}
 for axis in range(3):
  q=[0,0,0];q[axis]=1;terms[tuple(q)]=B*M
  q[axis]=2;terms[tuple(q)]=a*M*M
 for i,j in [(0,1),(0,2),(1,2)]:
  q=[0,0,0];q[i]=q[j]=1;terms[tuple(q)]=(2*a-g)*M*M
 values=[]
 for index in product(range(3),repeat=3):
  total=0
  for degree,c in terms.items():
   if any(k>i for i,k in zip(index,degree)):continue
   numerator=8*prod(comb(i,k)for i,k in zip(index,degree));denominator=prod(comb(2,k)for k in degree)
   assert numerator%denominator==0
   total+=c*(numerator//denominator)
  values.append(total)
 return -((-min(values))//8),max(values)//8

def excludes(L,R,p):return L>R or -((-L)//p)>R//p

def literal(params,xyz):
 a,B,C,g,d=params;x,y,z=xyz;S=x+y+z;T=x*y+x*z+y*z;U=x*y*z
 return a*S*S+B*S+C-g*T-d*U

def coupled_row(c,row,tiny=False):
 p,M,H=c['p'],c['M'],c['H'];old=row['curved_mask'];assert len(old)==H+1
 new=list(old);records=[];checks=source_checks=0;narrower=0
 # An already excluded u has a checked old certificate: skip extra work explicitly.
 for u in range(H+1):
  if old[u]=='1':continue
  for ri,r in enumerate(row['candidates']):
   v=list(map(int,r['vector']));cc=int(r['constant'])
   params=(v[0],-2*v[0]*u*u-v[1]*u+v[3],v[0]*u**4+v[1]*u**3+v[2]*u*u+v[4]*u+cc,v[5],v[6])
   old_interval=curve(v,cc,M,u)
   bound=bernstein_interval(params,M);independent=generic27(params,M)
   assert bound==independent,'independent27 Bernstein mismatch'
   L,R=max(old_interval[0],bound[0]),min(old_interval[1],bound[1]);excluded=excludes(L,R,p)
   if tiny:
    for xyz in product(range(M+1),repeat=3):
     value=literal(params,xyz);assert old_interval[0]<=value<=old_interval[1] and bound[0]<=value<=bound[1] and L<=value<=R
     assert not(excluded and value%p==0),'true source excluded'
     source_checks+=1
   if excluded:new[u]='1'
   checks+=1;narrower+=R-L<old_interval[1]-old_interval[0]
   records.append(dict(u=u,candidate=ri,old_interval=list(map(str,old_interval)),bernstein_interval=list(map(str,bound)),intersection=[str(L),str(R)],excludes=excluded))
 assert checks<=25*(H+1) and source_checks<=40000
 new=''.join(new);assert all(a=='0'or b=='1'for a,b in zip(old,new))
 return dict(old_mask=old,new_mask=new,old_unknown_u=old.count('0'),skipped_old_excluded_u=old.count('1'),candidate_u_checks=checks,literal_source_checks=source_checks,narrower_candidate_ranges=narrower,extra_exclusions=new.count('1')-old.count('1'),ranges=records)

def integration_controls():
 # Fresh integration probes, not a repetition of the full prior972-case test.
 checked=0
 for M in range(3):
  for params in [(1,0,1,2,0),(-1,2,-3,-2,1),(0,-1,-4,1,-1),(1,-3,-7,0,1)]:
   first=bernstein_interval(params,M);assert first==generic27(params,M)
   for xyz in product(range(M+1),repeat=3):assert first[0]<=literal(params,xyz)<=first[1];checked+=1
   for k in [-3,-1,1,4]:
    shifted=(params[0],params[1],params[2]+7*k,params[3],params[4]);second=bernstein_interval(shifted,M)
    assert second==(first[0]+7*k,first[1]+7*k) and second==generic27(shifted,M)
    assert excludes(*first,7)==excludes(*second,7);checked+=1
 assert bernstein_interval((1,0,1,2,0),1)==(1,4)
 assert excludes(1,4,7) and not excludes(-5,10,7)
 return dict(fresh_literal_and_phase_checks=checked,generic27_matches=True,abstract_p7_witness=True,actual_elliptic_realization_proved=False,legacy172057_predicates_reused_not_rerun=True)
