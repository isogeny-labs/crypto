from research.image_cells_20261115_v67.algebra import rem,image,binary_tower,tower_mod,trim
from research.vieta_rationality_20261116_v75.trace import inv,power,stripped
from research.shared_semilinear_trace_20261116_v82b.kernel import trace_jump,Frobenius
from research.exact_poly_square_20261116_v80.square import square_mod
from research.rationality_image_20261116_v76.worker import label_image as reference

def label(H,A,B,f,F):
 H,removed=stripped(H)
 if len(H)==1:return[1],[],removed
 iv=inv(H[0],f);b=power(B,1<<(f.n-1),f);t=[f.mul(b,f.mul(v,iv))for v in H[1:]]
 if len(t)<2:t +=[0]*(2-len(t))
 t[0]^=A;t[1]^=1;t=rem(t,H,f);L=trace_jump(t,f.n,H,f,F);return image(H,L,f),L,removed

def support(H,scalars,f):
 if not 2<=len(H)<=6 or H[-1]!=1:raise ValueError('monic degree1..5 required')
 r=tower_mod(H,scalars,f)
 for _ in range(3):r=square_mod(r,H,f)
 return not r

def reject(C):
 m=len(C)-1;return any(C[m-j]if m>=j else 0 for j in [1,2,4])

def bundle(records,f,method,d):
 scalars=binary_tower(f,d);F=Frobenius(f)if method=='shared'else None;rows=[]
 for r in records:
  supported=support(r['H'],scalars,f);row={'name':r['name'],'support':supported,'scope':r['scope']}
  if method!='support':
   C,L,removed=label(r['H'],r['A'],r['B'],f,F)if method=='shared'else reference(r['H'],r['A'],r['B'],f)
   row.update({'image_polynomial':C,'lambda_remainder':trim(L),'removed_zero_roots':removed,'rationality_reject':reject(C)})
  rows.append(row)
 return rows
