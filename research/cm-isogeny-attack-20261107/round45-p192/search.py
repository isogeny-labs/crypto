from pathlib import Path
from math import isqrt
import importlib.util,json
root=Path(__file__).resolve().parent
def load(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
lat=load('lat38',root.parent/'round38-p192/lattice.py');ec=load('ec18',root.parent/'round18-p192/cached_orbit.py');r,mu=ec.r,ec.mu
def region(lam,H,rectangle):
 scale=H if rectangle else 1;B=2*H*H if rectangle else H
 u,v=lat.reduce((scale,lam),(0,r));w=(0,(-lam*lam)%r)
 pts,cert=lat.box(u,v,w,B)
 assert all(x%scale==0 and (y-lam*(x//scale)+lam*lam)%r==0 for x,y,_,_ in pts)
 return {'H':H,'rectangle':rectangle,'scale':scale,'B':B,'basis':[u,v],'offset':w,'points':pts,'box':cert}
def minimum(lam,rectangle):
 lo,hi=0,1;calls=0
 while not region(lam,hi,rectangle)['points']:
  lo=hi;hi*=2;calls+=1;assert hi<=2**100 if not rectangle else hi<=2**70
 while lo+1<hi:
  mid=(lo+hi)//2;yes=region(lam,mid,rectangle)['points'];calls+=1
  if yes:hi=mid
  else:lo=mid
 return {'minimum':hi,'infeasible':region(lam,hi-1,rectangle),'feasible':region(lam,hi,rectangle),'search_calls':calls}
def balanced(trace,det,H):
 out=[]
 for ell in [2,3,5,7,11,13,17,19,23,29,31]:
  for a0 in range(ell):
   if (a0*a0-trace*a0+det)%ell or (2*a0-trace)%ell==0:continue
   a,b=a0,ell
   while b<=4*H:
    if b>=max(1,H//4):
     k=lat.nearest(trace-2*a,2*b)
     for shift in [k-1,k,k+1]:
      aa=a+shift*b;dd=trace-aa;n=aa*dd-det;assert n%b==0
      cc=n//b;A=[[aa,b],[cc,dd]]
      out.append({'matrix':A,'height':max(abs(x) for row in A for x in row),'prime':ell,'modulus':b,'root':a,'shift':shift})
    a+=(-(a*a-trace*a+det)//b*pow(2*a-trace,-1,ell)%ell)*b;b*=ell
 return out
def schedules(A):
 rows=[]
 for w in range(2,7):
  digits=[[lat.naf(n,w) for n in row] for row in A];m=2**(w-2)
  D=2*int(m>1);adds=2*(m-1);neg=0
  for row in digits:
   L=max(map(len,row));weight=sum(sum(x!=0 for x in d) for d in row)
   D+=max(0,L-1);adds+=max(0,weight-1);neg+=sum(sum(x<0 for x in d) for d in row)
  rows.append({'width':w,'D':D,'A':adds,'negative_lookups':neg,'digits':digits})
 return rows
rows=[]
for j in range(1,5):
 lam=pow(mu,j,r);mm=minimum(lam,False);rect=minimum(lam,True)
 tr,de,_,_=min(mm['feasible']['points']);companion=[[0,1],[-de,tr]]
 rectpts=rect['feasible']['points'];candidates=[]
 for x,y,_,_ in rectpts:
  trace=x//rect['minimum'];candidates.extend(balanced(trace,y,rect['minimum']))
 assert candidates
 best=min(candidates,key=lambda z:(z['height'],z['matrix']))
 mats=[]
 for kind,A in [('companion',companion),('balanced',best['matrix'])]:
  a,b=A[0];c,d=A[1];trace=a+d;det=a*d-b*c;assert (lam*lam-trace*lam+det)%r==0 and b%r
  ss=schedules(A)
  for z in ss:
   z['canonical_D']=2*z['D']-int(z['width']>2) if kind=='companion' else 3*z['D']
   z['canonical_A']=2*z['A']+3-(2**(z['width']-2)-1) if kind=='companion' else 3*z['A']+6
  sel=min(ss,key=lambda z:(z['canonical_D']+z['canonical_A'],z['canonical_D'],z['canonical_A'],z['width']))
  mats.append({'kind':kind,'matrix':A,'trace':trace,'determinant':det,'height':max(abs(x) for row in A for x in row),'eigenvector':[b%r,(lam-a)%r],'schedules':ss,'selected':sel,'canonicalization_D':sel['canonical_D'],'canonicalization_A':sel['canonical_A']})
 rows.append({'power':j,'eigenvalue':lam,'companion_minimax':mm,'entry_rectangle_bound':rect,'balanced_candidates':len(candidates),'balanced_witness':best,'matrices':mats})
 print('POWER',j,'companion_bits',mm['minimum'].bit_length(),'entry_bound_bits',rect['minimum'].bit_length(),'balanced_bits',best['height'].bit_length(),'canonical_DA',[(m['kind'],m['canonicalization_D'],m['canonicalization_A']) for m in mats],flush=True)
Path('certificate.json').write_text(json.dumps({'all_checks_pass':True,'r':r,'mu':mu,'rows':rows},indent=2)+'\n')
