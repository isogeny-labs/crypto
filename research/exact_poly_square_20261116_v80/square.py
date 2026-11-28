from research.image_cells_20261115_v67.algebra import rem

def square_mod(a,H,f):
 assert len(H)>=2 and H[-1]==1
 if not a:return []
 out=[0]*(2*len(a)-1)
 for i,x in enumerate(a):
  if x:out[2*i]=f.add(out[2*i],f.sq(x))
 if f.p!=2:
  for i,x in enumerate(a):
   if not x:continue
   for j in range(i+1,len(a)):
    y=a[j]
    if y:
     v=f.mul(x,y);out[i+j]=f.add(out[i+j],f.add(v,v))
 return rem(out,H,f)
