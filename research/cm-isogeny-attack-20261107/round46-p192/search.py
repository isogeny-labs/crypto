from pathlib import Path
from fractions import Fraction
from itertools import product
from flint import fmpz_mat
import json,hashlib
r=int('FFFFFFFFFFFFFFFFFFFFFFFF99DEF836146BC9B1B4D22831',16);mu=4072180873419819102403428347769737935154921989474241554654
def det(A):return sum((-1)**j*A[0][j]*(A[1][(j+1)%3]*A[2][(j+2)%3]-A[1][(j+2)%3]*A[2][(j+1)%3])*(-1)**j for j in range(3))
def inverse(A):
 d=det(A);out=[]
 for i in range(3):
  row=[]
  for j in range(3):
   rows=[k for k in range(3) if k!=j];cols=[k for k in range(3) if k!=i]
   cof=(-1)**(i+j)*(A[rows[0]][cols[0]]*A[rows[1]][cols[1]]-A[rows[0]][cols[1]]*A[rows[1]][cols[0]])
   row.append(Fraction(cof,d))
  out.append(row)
 return out
def ceil(x):return -((-x.numerator)//x.denominator)
def enum(A,w,H):
 inv=inverse(A);ranges=[]
 for j in range(3):
  center=-sum(w[i]*inv[i][j] for i in range(3));radius=H*sum(abs(inv[i][j]) for i in range(3))
  ranges.append([ceil(center-radius),(center+radius).__floor__()])
 rows=[]
 for z in product(*(range(a,b+1) for a,b in ranges)):
  x=tuple(w[i]+sum(z[j]*A[j][i] for j in range(3)) for i in range(3))
  if max(map(abs,x))<=H:rows.append((x,z))
 return rows,{'H':H,'ranges':ranges,'coefficient_boxes':__import__('math').prod(max(0,b-a+1) for a,b in ranges)}
def naf(n,w):
 sign=1 if n>=0 else -1;n=abs(n);out=[]
 while n:
  d=0
  if n%2:d=n%(2**w);d=d-2**w if d>=2**(w-1) else d
  out.append(sign*d);n=(n-d)//2
 return out
def schedules(coeff):
 digits={(i,w):naf(coeff[i],w) for i in range(3) for w in range(2,7)}
 for widths in product(range(2,7),repeat=3):
  ds=[digits[i,w] for i,w in enumerate(widths)]
  D=max(map(len,ds))-1+sum(w>2 for w in widths)
  A=sum(sum(d!=0 for d in z) for z in ds)-1+sum(2**(w-2)-1 for w in widths)+3
  yield {'coefficients':coeff,'widths':widths,'D':D,'A':A,'negative_lookups':sum(sum(d<0 for d in z) for z in ds),'digits':ds}
rows=[];H=2**65-1
for power in range(1,5):
 lam=pow(mu,power,r);A=fmpz_mat([[r,0,0],[-lam,1,0],[-pow(lam,2,r),0,1]]).lll();A=[[int(A[i,j]) for j in range(3)] for i in range(3)]
 assert abs(det(A))==r
 w=[pow(lam,3,r),0,0];iv=inverse(A)
 z=[int((-sum(w[i]*iv[i][j] for i in range(3))+Fraction(1,2)).__floor__()) for j in range(3)]
 incumbent=[w[i]+sum(z[j]*A[j][i] for j in range(3)) for i in range(3)];K=max(map(abs,incumbent))
 small,box=enum(A,w,K);best=min(small,key=lambda q:(max(map(abs,q[0])),q[0]));minimum=max(map(abs,best[0]))
 assert not enum(A,w,minimum-1)[0]
 pts,cert=enum(A,w,H);assert pts
 choices=[];digest=hashlib.sha256();count=0
 for coeff,z in sorted(pts):
  assert (sum(coeff[i]*pow(lam,i,r) for i in range(3))-pow(lam,3,r))%r==0
  for s in schedules(coeff):
   digest.update(json.dumps(s,sort_keys=True).encode());count+=1
   if not choices or (s['D']+s['A'],s['D'],s['A'],s['coefficients'],s['widths'])<(choices[0]['D']+choices[0]['A'],choices[0]['D'],choices[0]['A'],choices[0]['coefficients'],choices[0]['widths']):choices=[s]
 sel=choices[0]
 rows.append({'power':power,'lambda':lam,'basis':A,'offset':w,'minimax_height':minimum,'minimax_coefficients':best[0],'incumbent_height':K,'incumbent_box':box,'minimax_infeasible_box':enum(A,w,minimum-1)[1],'schedule_box':cert,'box_points':[x for x,z in sorted(pts)],'schedules_checked':count,'schedule_digest':digest.hexdigest(),'selected':sel})
 print('POWER',power,'minimax_bits',minimum.bit_length(),'box_points',len(pts),'selected',sel['D'],sel['A'],sel['coefficients'],sel['widths'],flush=True)
Path('certificate.json').write_text(json.dumps({'all_checks_pass':True,'r':r,'mu':mu,'rows':rows},indent=2)+'\n')
