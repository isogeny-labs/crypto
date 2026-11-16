"""Scoped odd regular Hermite controls; coordinator execution only."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource
START=time.monotonic();CHECKS=0;RAW=0;NORM=0;P=Path(__file__).resolve().parent
def check(v,msg):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=2000000 and time.monotonic()-START<80,'work cap'
 assert v,msg
class F:
 def __init__(self,p,n=1,m=0):
  self.p=p;self.n=n;self.m=m;self.q=p**n
  self.adds=[[self.enc(((a%p+b%p)%p,(a//p+b//p)%p))for b in range(self.q)]for a in range(self.q)]
  self.negs=[self.enc((-a%p,-(a//p)%p))for a in range(self.q)]
  self.muls=[[self.enc(((a%p*(b%p)-m*(a//p)*(b//p))%p,((a%p)*(b//p)+(a//p)*(b%p))%p))for b in range(self.q)]for a in range(self.q)]
 def enc(self,c):return c[0]%self.p+(self.p*(c[1]%self.p)if self.n==2 else 0)
 def c(self,x):return x%self.p
 def add(self,a,b):return self.adds[a][b]
 def neg(self,a):return self.negs[a]
 def sub(self,a,b):return self.add(a,self.neg(b))
 def mul(self,a,b):return self.muls[a][b]
 def pow(self,a,e):
  z=1
  while e:
   if e&1:z=self.mul(z,a)
   a=self.mul(a,a);e//=2
  return z
 def inv(self,a):check(a!=0,'inverse guard');return self.pow(a,self.q-2)
 def div(self,a,b):return self.mul(a,self.inv(b))
 def audit(self):
  if self.n==2:check(all((x*x+self.m)%self.p for x in range(self.p)),'irreducible quadratic')
  for a in range(self.q):
   if a:check(self.mul(a,self.inv(a))==1,'inverse')
   for b in range(self.q):
    av=[a%self.p,a//self.p];bv=[b%self.p,b//self.p];cv=[0,0,0]
    for i in range(self.n):
     for j in range(self.n):cv[i+j]+=av[i]*bv[j]
    if self.n==2:cv[0]-=self.m*cv[2]
    check(self.mul(a,b)==self.enc(cv[:2]),'independent coefficient convolution')
    for d in range(self.q):
     check(self.mul(a,self.add(b,d))==self.add(self.mul(a,b),self.mul(a,d)),'distributivity')
     check(self.mul(self.mul(a,b),d)==self.mul(a,self.mul(b,d)),'associativity')
def trim(a):
 a=list(a)
 while a and not a[-1]:a.pop()
 return a
def add(f,a,b):return trim([f.add(a[i]if i<len(a)else 0,b[i]if i<len(b)else 0)for i in range(max(len(a),len(b)))])
def neg(f,a):return[f.neg(x)for x in a]
def sub(f,a,b):return add(f,a,neg(f,b))
def scale(f,a,c):return trim([f.mul(x,c)for x in a])
def mul(f,a,b):
 if not a or not b:return []
 out=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):out[i+j]=f.add(out[i+j],f.mul(x,y))
 return trim(out)
def divmodp(f,a,b):
 a=trim(a);b=trim(b);check(bool(b),'polynomial divisor');out=[0]*max(0,len(a)-len(b)+1);ib=f.inv(b[-1])
 while len(a)>=len(b):
  j=len(a)-len(b);v=f.mul(a[-1],ib);out[j]=v
  for i,x in enumerate(b):a[i+j]=f.sub(a[i+j],f.mul(v,x))
  a=trim(a)
 return trim(out),a
def ev(f,a,x):
 z=0
 for v in reversed(a):z=f.add(f.mul(z,x),v)
 return z
def deriv(f,a):return trim([f.mul(f.c(i),a[i])for i in range(1,len(a))])
def pointlist(f,C):return[(x,y)for x in range(f.q)for y in range(f.q)if f.mul(y,y)==ev(f,C,x)]
def affine(f,C,z,y,h,a,ya):
 d=f.sub(a,z);cp=ev(f,deriv(f,C),a);haya=f.mul(f.mul(h,a),ya);hzy=f.mul(f.mul(h,z),y)
 v=[f.add(f.sub(f.sub(f.pow(z,3),f.pow(a,3)),haya),hzy),f.sub(y,ya)]
 w=[f.sub(f.sub(f.neg(f.div(f.mul(cp,f.mul(h,a)),f.mul(f.c(2),ya))),f.mul(h,ya)),f.mul(f.c(3),f.mul(a,a))),f.neg(f.div(cp,f.mul(f.c(2),ya)))]
 rr=[f.div(f.sub(f.mul(d,wj),vj),f.mul(d,d))for wj,vj in zip(w,v)]
 ss=[f.sub(wj,f.mul(f.mul(f.c(2),a),rj))for wj,rj in zip(w,rr)]
 return rr,ss
def literal(f,C,z,y,h,k,r,s):
 global NORM
 NORM+=1
 t=f.neg(f.add(f.add(f.add(f.pow(z,3),f.mul(r,f.mul(z,z))),f.mul(s,z)),f.mul(f.add(f.mul(h,z),k),y)))
 poly=[t,s,r,1];g=[k,h];norm=sub(f,mul(f,poly,poly),mul(f,C,mul(f,g,g)));H,rem=divmodp(f,norm,[f.neg(z),1]);check(not rem and len(H)==6 and H[-1]==1,'literal target norm')
 return poly,norm,H
def coeffs(f,C,z,y,h,a,ya,rs):
 rr,ss=rs
 tt=[f.neg(f.add(f.add(f.add(f.pow(z,3),f.mul(rr[0],f.mul(z,z))),f.mul(ss[0],z)),f.mul(f.mul(h,z),y))),f.neg(f.add(f.add(f.mul(rr[1],f.mul(z,z)),f.mul(ss[1],z)),y))]
 p0=[tt[0],ss[0],rr[0],1];p1=[tt[1],ss[1],rr[1]];g0=[0,h]
 nn=[sub(f,mul(f,p0,p0),mul(f,C,mul(f,g0,g0))),scale(f,sub(f,mul(f,p0,p1),mul(f,C,g0)),f.c(2)),sub(f,mul(f,p1,p1),C)]
 D=mul(f,[f.neg(z),1],mul(f,[f.neg(a),1],[f.neg(a),1]));qq=[]
 for v in nn:
  q,r=divmodp(f,v,D);check(not r,'formal coefficient division');qq.append(q+[0]*(4-len(q)))
 at,bt=qq[2][1],qq[1][1];au,bu=f.neg(qq[2][0]),f.neg(qq[1][0]);delta=f.sub(f.mul(au,bt),f.mul(at,bu))
 return qq,delta
def choose(f):
 found={};first=[];seen=0
 cats=['ramified','a_zero','a_equals_z','r1_zero','r1_nonzero','delta_zero','delta_nonzero','regular']
 for c0 in [0,1]:
  C=[c0,1,0,1];pts=pointlist(f,C)
  # C' is1 in characteristic3; both declared cubics have nonzero discriminant in characteristic5.
  check(all(not(ev(f,C,x)==0 and ev(f,deriv(f,C),x)==0)for x in range(f.q)),'squarefree selected cubic')
  for z,y in pts:
   for a,ya in pts:
    for h in range(f.q):
     seen+=1;check(seen<=100000,'selection state cap');cell=[C,z,y,h,a,ya];tags=[]
     if a==z:tags=['a_equals_z']
     elif not ya:tags=['ramified']
     else:
      rs=affine(f,C,z,y,h,a,ya);_,delta=coeffs(f,C,z,y,h,a,ya,rs)
      tags=['regular','r1_zero'if not rs[0][1]else'r1_nonzero','delta_zero'if not delta else'delta_nonzero']
      if not a:tags.append('a_zero')
     for tag in tags:
      if tag not in found:found[tag]=cell
     if len(first)<16:first.append(cell)
     if len(found)==len(cats):break
    if len(found)==len(cats):break
   if len(found)==len(cats):break
  if len(found)==len(cats):break
 out=[];cap=8 if f.q==25 else 16
 for cell in [found[t]for t in cats if t in found]+first:
  if cell not in out:out.append(cell)
  if len(out)>=cap:break
 return out,dict(selection_candidates=seen,available_categories=sorted(found),missing_categories=[t for t in cats if t not in found],selected_cells=out)
def run():
 global RAW
 records=[];fields=[];positive=0
 for pp,nn,mm in [(3,1,0),(5,1,0),(3,2,1),(5,2,2)]:
  f=F(pp,nn,mm);f.audit();cells,selection=choose(f);fieldcounts=dict(order=f.q,raw_states=0,positive_parameters=0,regular_cells=0,ramified_cells=0,excluded_target_collision=0,common_factor_assignments=0)
  for C,z,y,h,a,ya in cells:
   if a==z:
    fieldcounts['excluded_target_collision']+=1;records.append(dict(q=f.q,cell=[C,z,y,h,a,ya],status='excluded_a_equals_z'));continue
   if ya:rs=affine(f,C,z,y,h,a,ya);qq,delta=coeffs(f,C,z,y,h,a,ya,rs);fieldcounts['regular_cells']+=1
   else:rs=None;qq=None;delta=None;fieldcounts['ramified_cells']+=1
   Dsrc=sorted({a,f.add(a,1)});bags={}
   for bag in itertools.combinations_with_replacement(Dsrc,3):
    q=[1]
    for x in bag:q=mul(f,q,[f.neg(x),1])
    bags[tuple(q)]=list(bag)
   expected=[];got=[];details=[];common=0
   # Independent dense grid: literal point and derivative constraints first,
   # without reuse of the affine parameter formula; norm only on survivors.
   for k in range(f.q):
    ga=f.add(f.mul(h,a),k)
    for r in range(f.q):
     for s in range(f.q):
      RAW+=1;fieldcounts['raw_states']+=1;check(RAW<=200000,'raw r/s/k cap')
      if not ga:common+=1;continue
      t=f.neg(f.add(f.add(f.add(f.pow(z,3),f.mul(r,f.mul(z,z))),f.mul(s,z)),f.mul(f.add(f.mul(h,z),k),y)))
      pa=f.add(f.add(f.add(f.pow(a,3),f.mul(r,f.mul(a,a))),f.mul(s,a)),t)
      if pa!=f.neg(f.mul(ga,ya)):continue
      pd=f.add(f.add(f.mul(f.c(3),f.mul(a,a)),f.mul(f.mul(f.c(2),r),a)),s)
      nv=f.sub(f.sub(f.mul(f.mul(f.c(2),pa),pd),f.mul(ev(f,deriv(f,C),a),f.mul(ga,ga))),f.mul(f.mul(f.mul(f.c(2),ev(f,C,a)),ga),h))
      if nv:continue
      poly,norm,H=literal(f,C,z,y,h,k,r,s);Q,rem=divmodp(f,H,mul(f,[f.neg(a),1],[f.neg(a),1]));check(not rem,'independent raw double quotient');expected.append([k,r,s])
   for k in range(f.q):
    ga=f.add(f.mul(h,a),k)
    if not ga or not ya:continue
    r=f.add(rs[0][0],f.mul(rs[0][1],k));s=f.add(rs[1][0],f.mul(rs[1][1],k));poly,norm,H=literal(f,C,z,y,h,k,r,s)
    Q,rem=divmodp(f,H,mul(f,[f.neg(a),1],[f.neg(a),1]));check(not rem and len(Q)==4,'derived double quotient');got.append([k,r,s])
    predicted=[f.add(f.add(qq[0][j],f.mul(qq[1][j],k)),f.mul(qq[2][j],f.mul(k,k)))for j in range(4)];check(Q==predicted,'quadratic coefficient replay')
    S=f.neg(Q[2]);check(S==f.sub(f.sub(f.sub(f.sub(f.mul(h,h),z),f.mul(f.c(2),a)),f.mul(f.c(2),rs[0][0])),f.mul(f.mul(f.c(2),rs[0][1]),k)),'odd varying sum')
    Ad=mul(f,[f.neg(Dsrc[0]),1],[f.neg(Dsrc[1]),1]);R=divmodp(f,Ad,Q)[1];R3=divmodp(f,mul(f,mul(f,R,R),R),Q)[1];supported=not R3;check(supported==(tuple(Q)in bags),'independent complete cubic source multiset oracle')
    sources=[]
    if supported:
     sources=[list(v)for v in sorted(set(itertools.permutations(bags[tuple(Q)]+[a,a])))];check(all(all(x in Dsrc for x in v)for v in sources),'source ordering support');positive+=1;fieldcounts['positive_parameters']+=1
    details.append(dict(k=k,r=r,s=s,t=poly[0],P=poly,norm=norm,H=H,Q=Q,domain=Dsrc,remainder=R,cube_residue=R3,supported=supported,sources=sources))
   check(sorted(expected)==sorted(got),'complete independent raw parameter set equality')
   if not ya:check(not expected,'ramified regular branch empty')
   fieldcounts['common_factor_assignments']+=common
   records.append(dict(q=f.q,cell=[C,z,y,h,a,ya],status='complete_regular_component',affine_parameters=rs,quadratic_coefficients=qq,delta=delta,expected=expected,derived=got,candidates=details,common_factor_raw_assignments=common,common_factor_status='UNKNOWN',target_outside_domain=z not in Dsrc))
  fields.append(dict(counts=fieldcounts,selection=selection))
 check(positive>0,'nonvacuous positive supported norm parameter required')
 return dict(fields=fields,records=records,positive_supported_parameters=positive,raw_r_s_k_states=RAW,literal_norm_evaluations=NORM,common_factor_branch='UNKNOWN',original_auxiliary_roots='UNKNOWN',complete_global_root_search=False)
def main():
 ident=sys.argv[1];status='passed';out={};err=None
 try:out=run()
 except Exception as exc:status='inconclusive';err=type(exc).__name__+': '+str(exc)
 retained=True
 raw=dict(out=out,predicates=CHECKS,raw_r_s_k_states=RAW,literal_norm_evaluations=NORM,error=err);payload=json.dumps(raw,sort_keys=True,separators=(',',':')).encode()
 if len(payload)>32*1024*1024:
  retained=False;status='inconclusive';err='raw storage cap';raw=dict(error=err,omitted_payload_sha256=hashlib.sha256(payload).hexdigest(),raw_r_s_k_states=RAW,literal_norm_evaluations=NORM,complete=False);payload=json.dumps(raw).encode()
 blob=gzip.compress(payload,mtime=0)
 with(P/'raw-controls.json.gz').open('xb')as f:f.write(blob)
 summary={k:v for k,v in out.items()if k not in ['records','fields']};summary['fields']=[dict(counts=v['counts'],selection_candidates=v['selection']['selection_candidates'],available_categories=v['selection']['available_categories'],missing_categories=v['selection']['missing_categories'],selected_cell_count=len(v['selection']['selected_cells']))for v in out.get('fields',[])]
 evidence=dict(details=summary,error=err,predicates=CHECKS,raw_r_s_k_states=RAW,literal_norm_evaluations=NORM,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_path='raw-controls.json.gz',raw_includes_all_chart_candidates_and_support_rejections=retained and status=='passed',individual_dense_grid_rejection_certificates=False,stdout_contract='compact summary only; complete chart candidate certificates in raw gzip when retained',peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_controls=status=='passed',complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,novelty_claim=False,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Odd regular Hermite norm controls over GF3/GF5/GF9/GF25; scoped component only.',evidence=evidence,next_actions=['Review missing guard categories and positive-field gaps; compare charged conditional construction only after complete independent residue/source replay. Preserve common-factor/original auxiliary obligations.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['complete_conditional_controls']=False;evidence['error']='late serialized report';summary['positive_supported_parameters']=None
  for row in summary.get('fields',[]):row['counts']['positive_parameters']=None
  text=json.dumps(result,separators=(',',':'))
 assert time.monotonic()-START<88,'final stdout deadline';print(text)
if __name__=='__main__':main()
