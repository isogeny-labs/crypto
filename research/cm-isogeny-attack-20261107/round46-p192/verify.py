from pathlib import Path
from fractions import Fraction as F
from itertools import product
import json,hashlib
c=json.loads(Path('certificate.json').read_text());r=c['r']
def inv(A):
 M=[[F(x) for x in A[i]]+[F(i==j) for j in range(3)] for i in range(3)];det=F(1)
 for i in range(3):
  k=next(k for k in range(i,3) if M[k][i])
  if k!=i:M[k],M[i]=M[i],M[k];det=-det
  z=M[i][i];det*=z;M[i]=[x/z for x in M[i]]
  for k in range(3):
   if k==i:continue
   z=M[k][i];M[k]=[x-z*y for x,y in zip(M[k],M[i])]
 return [row[3:] for row in M],det
def ceil(x):return -((-x.numerator)//x.denominator)
def points(A,w,H):
 A=[[A[0][i]+A[1][i] for i in range(3)],A[1],A[2]];V,d=inv(A)
 corners=list(product([-H,H],repeat=3));ranges=[]
 for j in range(3):
  vals=[sum((X[i]-w[i])*V[i][j] for i in range(3)) for X in corners]
  ranges.append(range(ceil(min(vals)),max(vals).__floor__()+1))
 out=set()
 for z in product(*ranges):
  X=tuple(w[i]+sum(z[j]*A[j][i] for j in range(3)) for i in range(3))
  if max(map(abs,X))<=H:out.add(X)
 return sorted(out)
def digits(n,w):
 ds=[]
 while n:
  z=n%(2**w) if n%2 else 0
  if z>=2**(w-1):z-=2**w
  ds.append(z);n=(n-z)//2
 return ds
total=0
for row in c['rows']:
 lam=row['lambda'];A=row['basis'];w=row['offset'];V,d=inv(A);assert abs(d)==r
 assert w==[pow(lam,3,r),0,0]
 assert all(sum(v[i]*pow(lam,i,r) for i in range(3))%r==0 for v in A)
 H=row['minimax_height'];assert not points(A,w,H-1)
 pp=points(A,w,H);assert tuple(row['minimax_coefficients']) in pp
 pp=points(A,w,row['schedule_box']['H']);assert pp==list(map(tuple,row['box_points']))
 best=None;digest=hashlib.sha256();count=0
 for coeff in pp:
  for widths in product(range(2,7),repeat=3):
   ds=[digits(n,z) for n,z in zip(coeff,widths)]
   assert all(sum(a*2**i for i,a in enumerate(d))==n for d,n in zip(ds,coeff))
   D=max(map(len,ds))-1+sum(z>2 for z in widths)
   AA=sum(sum(a!=0 for a in d) for d in ds)-1+sum(2**(z-2)-1 for z in widths)+3
   s={'coefficients':coeff,'widths':widths,'D':D,'A':AA,'negative_lookups':sum(sum(a<0 for a in d) for d in ds),'digits':ds}
   digest.update(json.dumps(s,sort_keys=True).encode());count+=1
   key=(D+AA,D,AA,coeff,widths)
   if best is None or key<best[0]:best=(key,s)
 assert json.dumps(best[1],sort_keys=True)==json.dumps(row['selected'],sort_keys=True)
 assert count==row['schedules_checked'] and digest.hexdigest()==row['schedule_digest'];total+=count
out={'all_checks_pass':True,'global_minimax_certificates':4,'independent_box_points':sum(len(z['box_points']) for z in c['rows']),'independent_signed_digit_schedules':total,'basis':'alternate unimodular basis and Gauss-Jordan inverse, no producer imports'}
Path('verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
