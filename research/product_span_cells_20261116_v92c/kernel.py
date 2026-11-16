from research.disjoint_separator_20261116_v92.kernel import Prefix,certificate as original

def columns(S,y,u0,k,B):
 assert S.d+k-1<=S.n and k>=1;a,b,c=S.coefficients(y,u0,B);cols=S.columns(a,b,True);E=Prefix(S.n,S.f.mod,S.d+k-1);cols.extend(E.columns(1,y,True));v=S.f.sq(y)
 for _ in range(k):cols.append(v);v=S.timesT(S.timesT(v))
 return cols,c

def certificate(S,y,u0,k,B,method):
 if method=='full':return original(S,y,u0,k,B)
 assert method=='compressed';cols,c=columns(S,y,u0,k,B);rows=S.prepare(cols)[0]
 if S.particular(c,rows)is not None:return {'empty':False,'rank':len(rows),'dual':None}
 def reduce(v):
  for p in sorted(rows,reverse=True):
   if v>>p&1:v^=rows[p][0]
  return v
 r=reduce(c);bit=(r&-r).bit_length()-1;w=sum(((reduce(1<<j)>>bit)&1)<<j for j in range(S.n));assert all((w&v).bit_count()%2==0 for v in cols)and(w&c).bit_count()%2==1
 return {'empty':True,'rank':len(rows),'dual':w}
