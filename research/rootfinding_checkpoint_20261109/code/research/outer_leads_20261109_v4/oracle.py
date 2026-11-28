"""Unexecuted portable whole-h-certificate control. Standard library only."""
import itertools, json, sys, time
FEATURES=((2,0),(1,2),(1,1),(1,0),(0,4),(0,3),(0,2),(0,1))
def add(p,*rows):
 d={}
 for row in rows:
  for k,v in row.items():d[k]=(d.get(k,0)+v)%p
 return {k:v for k,v in d.items() if v}
def scale(p,row,c):return {k:v*c%p for k,v in row.items() if v*c%p}
def mul(p,a,b):
 d={}
 for (s,h),v in a.items():
  for (t,j),w in b.items():d[s+t,h+j]=(d.get((s+t,h+j),0)+v*w)%p
 return {k:v for k,v in d.items() if v}
def con(p,c):return {(0,0):c%p} if c%p else {}
def evaluate(p,row,S,h):return sum(v*pow(S,s,p)*pow(h,j,p) for (s,j),v in row.items())%p
def pmul(p,a,b):
 c=[{} for _ in range(len(a)+len(b)-1)]
 for i,x in enumerate(a):
  for j,y in enumerate(b):c[i+j]=add(p,c[i+j],mul(p,x,y))
 return c
def divide(p,a,x):
 q=[{} for _ in range(len(a)-1)];q[-1]=a[-1]
 for j in range(len(q)-2,-1,-1):q[j]=add(p,a[j+1],scale(p,q[j+1],x))
 assert not add(p,a[0],scale(p,q[0],x))
 return q
def chart(p,a,b,points,h0=0):
 S={(1,0):1};h=add(p,{(0,1):1},con(p,h0));L=[]
 for i,(x,y) in enumerate(points):
  other=[v[0] for j,v in enumerate(points) if j!=i];u,v=other
  inv=pow((x-u)*(x-v)%p,-1,p);L.append([u*v*inv%p,-(u+v)*inv%p,inv])
 A=sum(-y*l[2] for (x,y),l in zip(points,L))%p
 B={}
 for (x,y),l in zip(points,L):B=add(p,B,scale(p,add(p,con(p,-x**3),scale(p,h,-x*y)),l[2]))
 kap=2*A%p
 if kap==0:return None
 k=scale(p,add(p,mul(p,h,h),scale(p,B,-2),con(p,-sum(x for x,y in points)),scale(p,S,-1)),pow(kap,-1,p))
 P=[{}, {}, {}, con(p,1)]
 for (x,y),l in zip(points,L):
  value=add(p,con(p,-x**3),scale(p,h,-x*y),scale(p,k,-y))
  for j in range(3):P[j]=add(p,P[j],scale(p,value,l[j]))
 Q=[k,h];N=pmul(p,P,P);FQ=pmul(p,[con(p,b),con(p,a),{},con(p,1)],pmul(p,Q,Q))
 for j in range(len(FQ)):N[j]=add(p,N[j],scale(p,FQ[j],-1))
 for x,y in points:N=divide(p,N,x)
 assert N[3]==con(p,1) and N[2]==scale(p,S,-1)
 T=N[1];U=scale(p,N[0],-1)
 assert all(s*2+j<=4 and ((s,j)==(0,0) or (s,j) in FEATURES) for q in (T,U) for s,j in q)
 return T,U

def solve(p,A,b):
 A=[list(row)+[z%p] for row,z in zip(A,b)]
 for i in range(len(A)):
  pivot=next(j for j in range(i,len(A)) if A[j][i]%p);A[i],A[pivot]=A[pivot],A[i]
  A[i]=[v*pow(A[i][i]%p,-1,p)%p for v in A[i]]
  for j in range(len(A)):
   if j!=i:
    z=A[j][i];A[j]=[(x-z*y)%p for x,y in zip(A[j],A[i])]
 return [row[-1] for row in A]
def literal(p,a,b,points,S,h):
 # Independent 4x4 solve; no Lagrange or symbolic-polynomial helpers.
 A=[[x*x%p,x,1,y] for x,y in points]+[[2,0,0,0]]
 B=[-x**3-h*x*y for x,y in points]+[h*h-sum(x for x,y in points)-S]
 r,s,t,k=solve(p,A,B);P=[t,s,r,1];Q=[k,h]
 def conv(x,y):
  out=[0]*(len(x)+len(y)-1)
  for i,v in enumerate(x):
   for j,w in enumerate(y):out[i+j]+=v*w
  return out
 N=conv(P,P);FQ=conv([b,a,0,1],conv(Q,Q))
 for i,v in enumerate(FQ):N[i]-=v
 for x,y in points:
  out=[0]*(len(N)-1);out[-1]=N[-1]%p
  for j in range(len(out)-2,-1,-1):out[j]=(N[j+1]+x*out[j+1])%p
  assert (N[0]+x*out[0])%p==0;N=out
 assert N[3]%p==1 and N[2]%p==(-S)%p
 return N[1]%p,-N[0]%p

def certificate(p,T,U,M,H,gamma,delta):
 def lift(v):
  v%=p
  return v if 2*v<=p else v-p
 lambdas=[lift(gamma*T.get(k,0)+delta*U.get(k,0)) for k in FEATURES]
 c=lift(gamma*T.get((0,0),0)+delta*U.get((0,0),0));lo=hi=c
 for v,(s,h) in zip(lambdas,FEATURES):
  z=v*(3*M)**s*H**h;lo+=min(0,z);hi+=max(0,z)
 for v,upper in [(-gamma,3*M*M),(-delta,M**3)]:
  z=v*upper;lo+=min(0,z);hi+=max(0,z)
 return ((-((-lo)//p))>hi//p),dict(lambdas=lambdas,c=c,L=lo,R=hi)

def main():
 start=time.monotonic();charts=identities=triples=closed=positiveblocks=exceptional=0;closed_blocks=set()
 for p in (5,7,11):
  for a,b in ((0,1),(1,1),(1,2)):
   if (4*a**3+27*b*b)%p==0:continue
   pts=[]
   for x in range(p):
    ys=[y for y in range(p) if (y*y-x**3-a*x-b)%p==0]
    if ys:pts.append((x,ys[0]))
   for points in list(itertools.combinations(pts,3))[:3]:
    base=chart(p,a,b,points)
    if base is None:exceptional+=1;continue
    charts+=1;M=2
    for S in range(3*M+1):
     for h in range(p):
      assert tuple(evaluate(p,q,S,h) for q in base)==literal(p,a,b,points,S,h);identities+=1
    for width in (1,2,3):
     for h0 in range(0,p,width):
      H=min(width,p-h0)-1;T,U=chart(p,a,b,points,h0);hits=[]
      for xyz in itertools.product(range(M+1),repeat=3):
       S=sum(xyz);e2=sum(xyz[i]*xyz[j] for i in range(3) for j in range(i));e3=xyz[0]*xyz[1]*xyz[2]
       for u in range(H+1):
        actual=literal(p,a,b,points,S,h0+u)
        hit=(e2%p,e3%p)==actual;hits.append(hit);triples+=1
      if any(hits):positiveblocks+=1
      for gamma,delta in itertools.product(range(-1,2),repeat=2):
       yes,cert=certificate(p,T,U,M,H,gamma,delta)
       if yes:
        closed+=1;closed_blocks.add((p,a,b,points,width,h0));assert not any(hits)
    assert time.monotonic()-start<75,'component budget'
 # Explicit characteristic-three and exceptional-chart controls.
 assert chart(5,-1,1,((4,1),(0,1),(1,1))) is None
 tri=((0,1),(1,1),(2,2));ct=chart(3,2,1,tri);assert ct is not None
 for S in range(7):
  for h in range(3):
   assert tuple(evaluate(3,q,S,h) for q in ct)==literal(3,2,1,tri,S,h);identities+=1
 for xyz in itertools.product(range(3),repeat=3):
  S=sum(xyz);e2=sum(xyz[i]*xyz[j] for i in range(3) for j in range(i));e3=xyz[0]*xyz[1]*xyz[2]
  for h in range(3):
   assert ((e2%3,e3%3)==literal(3,2,1,tri,S,h))==((e2%3,e3%3)==tuple(evaluate(3,q,S,h) for q in ct));triples+=1
 # Nonvacuous rejection and a whole-h positive counterexample.
 assert certificate(19,con(19,9),con(19,9),1,18,1,0)[0]
 assert not certificate(19,{}, {},1,18,1,0)[0]
 evidence=dict(controls_complete=True,charts=charts,independent_literal_identities=identities,ordered_triple_h_checks=triples,positive_blocks=positiveblocks,rejected_small_actual_blocks=len(closed_blocks),rejection_certificates=closed,characteristic_three_literal_checks=21,explicit_kappa_zero_guard=True,exceptional_kappa_charts=exceptional,nonvacuous_abstract_rejection=True,whole_h_zero_chart_counterexample=True,source_execution_seconds=time.monotonic()-start,original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1] if len(sys.argv)>1 else 'unregistered-whole-h-oracle',status='passed',summary='Whole-h elimination agrees with independent literal norms and retains every small supported triple; no full-width search or performance claim.',evidence=evidence,next_actions=['Freeze a complete actual h-block panel and independently replay all certificates; measure total cost and global block count.'])
 text=json.dumps(result)
 if time.monotonic()-start>=80:
  result.update(status='inconclusive',summary='Whole-h control report exceeded acceptance budget.');result['evidence']['controls_complete']=False;text=json.dumps(result)
 assert time.monotonic()-start<88,'final serialized source deadline'
 print(text)
if __name__=='__main__':main()
