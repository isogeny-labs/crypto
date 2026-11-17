from research.image_cells_20261115_v67.algebra import rem

def echelon(values):
 piv={}
 for v in values:
  while v:
   j=v.bit_length()-1
   if j in piv:v^=piv[j]
   else:piv[j]=v;break
 return piv

def quotient(value,piv):
 for j in sorted(piv,reverse=True):
  if value>>j&1:value^=piv[j]
 return value

def second_image(H,q,f):
 m=len(H)-1;cols=[(rem([0]*j+list(q),H,f)+[0]*m)[:m]for j in range(m)];out=0
 for i in range(m):
  for j in range(i+1,m):out^=f.mul(cols[i][i],cols[j][j])^f.mul(cols[j][i],cols[i][j])
 return out
