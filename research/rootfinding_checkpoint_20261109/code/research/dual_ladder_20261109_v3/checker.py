"""Independent Python-integer certificate and input-semantics audit; charged by worker."""
def check(c):
 p=int(c['p']);M=int(c['M']);assert len(c['T'])==len(c['U'])==3 and len(c['vector'])==len(c['lo'])==len(c['hi'])==4;t=list(map(int,c['T']));u=list(map(int,c['U']));y=list(map(int,c['vector']));lo=list(map(int,c['lo']));hi=list(map(int,c['hi']));assert p>0 and M>=0
 assert lo==[0,0,0,0]and hi==[3*M,9*M*M,3*M*M,M**3]
 assert(y[0]+t[1]*y[2]+u[1]*y[3])%p==0 and(y[1]+t[0]*y[2]+u[0]*y[3])%p==0
 C=(y[2]*t[2]+y[3]*u[2])%p;low=sum(a*(l if a>=0 else h)for a,l,h in zip(y,lo,hi));high=sum(a*(h if a>=0 else l)for a,l,h in zip(y,lo,hi));ql=-((C-low)//p);qh=(high-C)//p
 assert C==int(c['constant'])and low==int(c['minimum'])and high==int(c['maximum'])and ql==int(c['quotient_low'])and qh==int(c['quotient_high'])and ql>qh
 return True

def verify_row(row,setup):
 c=row['certificate'];p=int(c['p']);record=setup[row['field']];assert p==record[0]and int(c['M'])==(1<<record[1])-1
 a,b,zx,zy=record[2:];assert a==int(row['curve_a'])and b==int(row['curve_b'])and[zx,zy]==list(map(int,row['target']))
 x,y=list(map(int,row['anchor_x'])),list(map(int,row['anchor_y']));h=int(row['h']);assert len(x)==len(y)==2 and 0<=x[0]<=int(c['M']) and 0<=y[0]<=int(c['M']) and h==[v%p for v in [0,1,-1,2,-2,3,-3,4]][row['h_index']];assert x[0]!=y[0]and x[0]!=zx and y[0]!=zx
 for xx,yy in [x,y,[zx,zy]]:assert(yy*yy-xx**3-a*xx-b)%p==0
 def ab(pt):
  xx,yy=pt;iz=pow((xx-zx)%p,-1,p);return((zy-yy)*iz%p,(-(xx*xx+xx*zx+zx*zx)+h*(zx*zy-xx*yy)*iz)%p)
 ax,bx=ab(x);ay,by=ab(y);iz=pow((x[0]-y[0])%p,-1,p);A=(ax-ay)*iz%p;B=(bx-by)*iz%p;CC=(ax-x[0]*A)%p;DD=(bx-x[0]*B)%p;kap=-2*A%p;assert kap;delta=(h*h-zx-2*B-x[0]-y[0])%p;ik=pow(kap,-1,p)
 def mul(v,w):
  out=[0]*(len(v)+len(w)-1)
  for i,vi in enumerate(v):
   for j,wj in enumerate(w):out[i+j]=(out[i+j]+vi*wj)%p
  return out
 def div(v,xx):
  out=[0]*(len(v)-1);out[-1]=v[-1]
  for j in range(len(out)-2,-1,-1):out[j]=(v[j+1]+xx*out[j+1])%p
  assert(v[0]+xx*out[0])%p==0;return out
 def at(S):
  k=(S-delta)*ik%p;r=(A*k+B)%p;s=(CC*k+DD-zx*r)%p;t=-(zx**3+r*zx*zx+s*zx+(h*zx+k)*zy)%p;P=[t,s,r,1];N=mul(P,P);fq=mul([b,a,0,1],mul([k,h],[k,h]));N=[(v-(fq[j]if j<len(fq)else 0))%p for j,v in enumerate(N)];C=div(div(div(N,zx),x[0]),y[0]);return C[1],-C[0]%p
 vals=[at(i)for i in range(3)]
 for j,key in enumerate(['T','U']):
  f0,f1,f2=[v[j]for v in vals];aa=(f2-2*f1+f0)*pow(2,-1,p)%p;bb=(f1-f0-aa)%p;assert [v%p for v in map(int,c[key])]==[aa,bb,f0]
 return check(c)


def verify_positive(case,setup):
 pub=case['public'];ex=case['expected'];name=pub['field'];p=int(pub['p']);record=setup[name];assert p==record[0]and int(pub['M'])==(1<<record[1])-1
 a,b=record[2:4];assert [a,b]==[int(pub['curve_a']),int(pub['curve_b'])];z=list(map(int,pub['target']));src=[list(map(int,v))for v in ex['source_points']];h=int(pub['h']);k=int(ex['k']);r=int(ex['r']);s=int(ex['s']);t=int(ex['t']);M=int(pub['M'])
 for x,y in src+[z]:assert(y*y-x**3-a*x-b)%p==0 and(x>M if [x,y]==z else 0<=x<=M)
 for x,y in src+[z]:assert(x**3+r*x*x+s*x+t+(h*x+k)*y)%p==0
 def mul(v,w):
  out=[0]*(len(v)+len(w)-1)
  for i,vi in enumerate(v):
   for j,wj in enumerate(w):out[i+j]=(out[i+j]+vi*wj)%p
  return out
 P=[t,s,r,1];N=mul(P,P);fq=mul([b,a,0,1],mul([k,h],[k,h]));N=[(v-(fq[j]if j<len(fq)else 0))%p for j,v in enumerate(N)];H=[0]*6;H[-1]=N[-1]
 for j in range(4,-1,-1):H[j]=(N[j+1]+z[0]*H[j+1])%p
 assert(N[0]+z[0]*H[0])%p==0;want=[1]
 for x,y in src:want=mul(want,[-x%p,1])
 assert H==want
 public=dict(pub,certificate={k:pub[k]for k in ['p','M','T','U']});line=verify_public_quadratics(public,{name:[p,int(pub['dimension']),a,b,*z]})
 remaining=[x for x,y in src]
 for key in ['anchor_x','anchor_y']:
  point=list(map(int,pub[key]));assert point in src;remaining.remove(point[0])
 roots=list(map(int,ex['residual_roots']));assert sorted(remaining)==sorted(roots);assert len(roots)==3 and all(0<=v<=M for v in roots)
 S=sum(roots);assert k==(S-line['delta'])*pow(line['kap'],-1,p)%p;T=sum(roots[i]*roots[j]for i in range(3)for j in range(i+1,3));U=roots[0]*roots[1]*roots[2]
 tq=list(map(int,pub['T']));uq=list(map(int,pub['U']));assert(tq[0]*S*S+tq[1]*S+tq[2]-T)%p==0 and(uq[0]*S*S+uq[1]*S+uq[2]-U)%p==0
 return True


def verify_public_quadratics(row,setup):
 c=row['certificate'];p=int(c['p']);record=setup[row['field']];assert p==record[0]and int(c['M'])==(1<<record[1])-1
 a,b,zx,zy=record[2:];assert a==int(row['curve_a'])and b==int(row['curve_b'])and[zx,zy]==list(map(int,row['target']))
 x,y=list(map(int,row['anchor_x'])),list(map(int,row['anchor_y']));h=int(row['h']);assert len(x)==len(y)==2 and 0<=x[0]<=int(c['M']) and 0<=y[0]<=int(c['M']) and 0<=h<p;assert x[0]!=y[0]and x[0]!=zx and y[0]!=zx
 for xx,yy in [x,y,[zx,zy]]:assert(yy*yy-xx**3-a*xx-b)%p==0
 def ab(pt):
  xx,yy=pt;iz=pow((xx-zx)%p,-1,p);return((zy-yy)*iz%p,(-(xx*xx+xx*zx+zx*zx)+h*(zx*zy-xx*yy)*iz)%p)
 ax,bx=ab(x);ay,by=ab(y);iz=pow((x[0]-y[0])%p,-1,p);A=(ax-ay)*iz%p;B=(bx-by)*iz%p;CC=(ax-x[0]*A)%p;DD=(bx-x[0]*B)%p;kap=-2*A%p;assert kap;delta=(h*h-zx-2*B-x[0]-y[0])%p;ik=pow(kap,-1,p)
 def mul(v,w):
  out=[0]*(len(v)+len(w)-1)
  for i,vi in enumerate(v):
   for j,wj in enumerate(w):out[i+j]=(out[i+j]+vi*wj)%p
  return out
 def div(v,xx):
  out=[0]*(len(v)-1);out[-1]=v[-1]
  for j in range(len(out)-2,-1,-1):out[j]=(v[j+1]+xx*out[j+1])%p
  assert(v[0]+xx*out[0])%p==0;return out
 def at(S):
  k=(S-delta)*ik%p;r=(A*k+B)%p;s=(CC*k+DD-zx*r)%p;t=-(zx**3+r*zx*zx+s*zx+(h*zx+k)*zy)%p;P=[t,s,r,1];N=mul(P,P);fq=mul([b,a,0,1],mul([k,h],[k,h]));N=[(v-(fq[j]if j<len(fq)else 0))%p for j,v in enumerate(N)];C=div(div(div(N,zx),x[0]),y[0]);return C[1],-C[0]%p
 vals=[at(i)for i in range(3)]
 for j,key in enumerate(['T','U']):
  f0,f1,f2=[v[j]for v in vals];aa=(f2-2*f1+f0)*pow(2,-1,p)%p;bb=(f1-f0-aa)%p;assert [v%p for v in map(int,c[key])]==[aa,bb,f0]
 return dict(A=A,B=B,C=CC,D=DD,kap=kap,delta=delta)
