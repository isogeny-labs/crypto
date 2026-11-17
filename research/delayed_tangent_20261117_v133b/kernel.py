from research.odd_coprime_factor_20261116_v119c.kernel import initial,inverse_block,matvec
from research.residue_quotient_20261116_v99b.kernel import basis,apply

def at(p,k):return p[k]if 0<=k<len(p)else 0

def coefficient(A,B,k,f):
 out=[0]*5
 for i,a in enumerate(A):
  for j,b in enumerate(B):
   if i+j>=5:continue
   for t in range(k+1):
    x,y=at(a,t),at(b,k-t)
    if x and y:out[i+j]=f.add(out[i+j],f.mul(x,y))
 return out

def jet(F,D,d,n,f):
 assert all(not any(v[:1])for v in D)
 S,I=inverse_block(f);A=[[0],[0],[1]];B=[[v]for v in initial(f)];DA=[[0],[0],[0]];DB=[[0]for _ in range(4)];audit=[];m=d-1
 for k in range(1,n+1):
  old=coefficient(A,B,k,f);left=coefficient(DA,B,k,f);right=coefficient(A,DB,k,f)
  err=[f.sub(at(F[j],k),old[j])for j in range(5)];de=[f.sub(at(D[j],k),f.add(left[j],right[j]))for j in range(5)];c=matvec(I,err,f);dc=matvec(I,de,f)
  assert matvec(S,c,f)==err and matvec(S,dc,f)==de
  for V,DV,degree,lo in [(A,DA,2,0),(B,DB,3,2)]:
   for j in range(degree):
    if k<=(degree-j)*m:V[j].append(c[lo+j]);DV[j].append(dc[lo+j])
  audit.append({'correction':c,'tangent':dc})
 return audit

def bitpolys(xs):return [[(a>>k)&1 for k in range(a.bit_length())]for a in xs]

def image_representatives(columns,cap):
 piv,_=basis(columns);assert len(piv)<=cap,'earlyrankcap';vectors=[piv[k][1]for k in sorted(piv)];return [apply(vectors,x)for x in range(1<<len(vectors))]

def row_solutions(rows,width):return [v for v in range(1,1<<width)if all((r&v).bit_count()%2 for r in rows)]
