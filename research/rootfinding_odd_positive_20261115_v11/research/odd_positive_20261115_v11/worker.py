"""General-cubic odd positive controls; coordinator execution only."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource
START=time.monotonic();CHECKS=0;RAW=0;NORM=0;P=Path(__file__).resolve().parent
def check(v,msg):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=500000 and time.monotonic()-START<80,'work cap'
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
def squarefree(f,C):
 u,v=C,deriv(f,C)
 while v:u,v=v,divmodp(f,u,v)[1]
 return len(u)==1
def product(f,roots):
 out=[1]
 for x in roots:out=mul(f,out,[f.neg(x),1])
 return out
def controls(public):
 global RAW
 records=[];summaries=[]
 for inp in public['fixtures']:
  f=F(inp['p'],2,inp['quadratic_constant']);f.audit();C=inp['C'];z,y=inp['target'];a,ya,h=inp['a'],inp['ya'],inp['h'];domain=inp['domain']
  check(f.q==inp['q'] and len(C)==4 and C[-1]==1 and squarefree(f,C),'general cubic nonsingular')
  check(len(set(domain))==2 and a in domain and z not in domain and a!=z,'arbitrary two-point domain and disjoint target')
  check(all(0<=v<f.q for v in C+domain+[z,y,a,ya,h]),'canonical coordinates')
  check(f.mul(ya,ya)==ev(f,C,a) and f.mul(y,y)==ev(f,C,z) and ya!=0,'rational nonramified point/target')
  rr,ss=affine(f,C,z,y,h,a,ya);qcoeff,delta=coeffs(f,C,z,y,h,a,ya,(rr,ss));expected=[];derived=[];raw_states=0;common=0;raw_survivors=[]
  # Full dense raw grid. No affine parameter formulas are used here.
  for k in range(f.q):
   ga=f.add(f.mul(h,a),k)
   for r in range(f.q):
    for s in range(f.q):
     RAW+=1;raw_states+=1;check(RAW<=20000,'raw grid cap')
     if not ga:common+=1;continue
     const=f.neg(f.add(f.add(f.add(f.pow(z,3),f.mul(r,f.mul(z,z))),f.mul(s,z)),f.mul(f.add(f.mul(h,z),k),y)))
     pa=f.add(f.add(f.add(f.pow(a,3),f.mul(r,f.mul(a,a))),f.mul(s,a)),const)
     if pa!=f.neg(f.mul(ga,ya)):continue
     pd=f.add(f.add(f.mul(f.c(3),f.mul(a,a)),f.mul(f.mul(f.c(2),r),a)),s)
     nd=f.sub(f.sub(f.mul(f.mul(f.c(2),pa),pd),f.mul(ev(f,deriv(f,C),a),f.mul(ga,ga))),f.mul(f.mul(f.mul(f.c(2),ev(f,C,a)),ga),h))
     if nd:continue
     poly,norm,H=literal(f,C,z,y,h,k,r,s);Q,rem=divmodp(f,H,product(f,[a,a]));check(not rem and len(Q)==4,'raw survivor literal quotient')
     expected.append([k,r,s,const]);raw_survivors.append(dict(parameters=[k,r,s,const],Q=Q))
  bags={tuple(product(f,bag)):list(bag)for bag in itertools.combinations_with_replacement(domain,3)};check(len(bags)==4,'all independent cubic multisets')
  Ad=product(f,domain);candidates=[];positive=0;order_count=0
  for k in range(f.q):
   ga=f.add(f.mul(h,a),k)
   if not ga:continue
   r=f.add(rr[0],f.mul(rr[1],k));s=f.add(ss[0],f.mul(ss[1],k));poly,norm,H=literal(f,C,z,y,h,k,r,s);Q,rem=divmodp(f,H,product(f,[a,a]));check(not rem and len(Q)==4,'derived quotient')
   parameters=[k,r,s,poly[0]];derived.append(parameters)
   pred=[f.add(f.add(qcoeff[0][j],f.mul(qcoeff[1][j],k)),f.mul(qcoeff[2][j],f.mul(k,k)))for j in range(4)];check(Q==pred,'formal quadratic coefficient replay')
   # R^3 and independent complete four-bag support, including repeated roots.
   rem=divmodp(f,Ad,Q)[1];cube=divmodp(f,mul(f,mul(f,rem,rem),rem),Q)[1];supported=not cube;check(supported==(tuple(Q)in bags),'exact support versus all cubic multisets')
   sources=[list(v)for v in sorted(set(itertools.permutations(bags[tuple(Q)]+[a,a])))]if supported else []
   oracle_orders=[list(v)for v in itertools.product(sorted(domain),repeat=5)if product(f,v)==H]
   check(sources==oracle_orders,'all canonical ordered x roots versus complete D^5 oracle')
   if supported:positive+=1;order_count+=len(sources)
   candidates.append(dict(parameters=parameters,P=poly,G=[k,h],N=norm,H=H,Q=Q,domain_polynomial=Ad,remainder=rem,cube_residue=cube,supported=supported,source_orders=sources))
  check(sorted(expected)==sorted(derived),'complete raw/derived parameter sets')
  check(positive>0,'positive disjoint extension norm control mandatory')
  summary=dict(q=f.q,raw_states=raw_states,regular_parameter_count=len(derived),supported_parameter_count=positive,canonical_x_orders_across_parameter_fibers=order_count,common_factor_raw_states=common,C2_nonzero=bool(C[2]),nonprime_domain=any(v>=f.p for v in domain),h_zero=h==0,target_disjoint=True)
  summaries.append(summary);records.append(dict(public=inp,affine_r=rr,affine_s=ss,quotient_coefficients=qcoeff,delta=delta,dense_parameters=expected,dense_survivors=raw_survivors,derived_parameters=derived,candidates=candidates,summary=summary))
 check(RAW==16354,'all fixed raw assignments completed')
 return dict(records=records,summaries=summaries,raw_states=RAW,literal_norm_evaluations=NORM)
def expected_checks(out,expected):
 receipts=[]
 for fixture in expected['fixtures']:
  rec=next(v for v in out['records']if v['public']['q']==fixture['q']);want=fixture['parameters'];row=next(v for v in rec['candidates']if v['parameters']==want)
  check(row['supported'] and len(row['source_orders'])==fixture['source_order_count'],'known constructive positive independently recovered')
  check(row['H']==product(F(rec['public']['p'],2,rec['public']['quadratic_constant']),fixture['bag']),'known seed complete source polynomial')
  receipts.append(dict(q=fixture['q'],parameters=want,source_order_count=len(row['source_orders'])))
 return receipts
def main():
 ident=sys.argv[1];status='passed';out={};err=None
 try:
  public=json.loads((P/'PUBLIC_INPUTS.json').read_text());out=controls(public)
  # Expected constructive witnesses are read only AFTER all parameter solving.
  out['expected_control_receipts']=expected_checks(out,json.loads((P/'EXPECTED_CONTROLS.json').read_text()))
 except Exception as exc:status='inconclusive';err=type(exc).__name__+': '+str(exc)
 raw=dict(details=out,error=err,predicates=CHECKS,raw_states=RAW,literal_norm_evaluations=NORM);payload=json.dumps(raw,sort_keys=True,separators=(',',':')).encode();retained=True
 if len(payload)>131072:
  retained=False;status='inconclusive';err='raw payload cap';payload=json.dumps(dict(error=err,omitted_payload_sha256=hashlib.sha256(payload).hexdigest(),raw_states=RAW,complete=False)).encode()
 blob=gzip.compress(payload,mtime=0)
 if len(blob)>16384:
  retained=False;status='inconclusive';err='compressed artifact cap';payload=json.dumps(dict(error=err,omitted_payload_sha256=hashlib.sha256(payload).hexdigest(),raw_states=RAW,complete=False)).encode();blob=gzip.compress(payload,mtime=0)
 with(P/'raw.json.gz').open('xb')as f:f.write(blob)
 summary=dict(fields=out.get('summaries',[]),raw_states=RAW,literal_norm_evaluations=NORM,expected_control_receipts=out.get('expected_control_receipts',[]))
 evidence=dict(summary=summary,error=err,predicates=CHECKS,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_payload_bytes=len(payload),raw_path='raw.json.gz',all_chart_candidate_certificates_retained=retained and status=='passed',complete_conditional_controls=status=='passed',scope='all regular selected-ordinate norm parameter fibers and canonical x source orders on two exposed odd-extension fixtures',common_factor_branch='UNKNOWN',ramified_branch='not exercised; UNKNOWN outside admitted fixtures',original_auxiliary_roots='UNKNOWN',original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,novelty_claim=False,peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='General-cubic GF9/GF25 disjoint positive norm controls; no original point/auxiliary/global root count.',evidence=evidence,next_actions=['Independently replay every stored literal norm, support residue and canonical x ordering; retain general cubic and arbitrary-domain guards before fresh matched conditional profiling.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['error']='late serialized report';evidence['complete_conditional_controls']=False;evidence['all_chart_candidate_certificates_retained']=False
  for row in summary['fields']:row['regular_parameter_count']=None;row['supported_parameter_count']=None;row['canonical_x_orders_across_parameter_fibers']=None
  text=json.dumps(result,separators=(',',':'))
 assert len(text.encode())<=16384,'stdout storage cap'
 assert time.monotonic()-START<88,'final reporting deadline';print(text)
if __name__=='__main__':main()
