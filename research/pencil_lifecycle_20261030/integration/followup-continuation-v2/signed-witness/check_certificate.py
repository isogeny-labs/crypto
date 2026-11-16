"""Direct certificate checks only: no elimination, inverses or native graph code."""
def verify(p,n,rows,cert):
 if type(p)is not int or not 2<=p<=1000000 or any(p%d==0 for d in range(2,int(p**.5)+1)):return False
 if type(n)is not int or not 0<=n<=2048 or len(rows)>512:return False
 canonical=lambda xs:all(type(x)is int and 0<=x<p for x in xs)
 if any(len(r)!=n+2 or not canonical(r) for r in rows):return False
 kind=cert.get('kind')
 if kind in ('determined','inconsistent'):
  if set(cert)!=({'kind','weights','scalar','point'} if kind=='determined' else {'kind','weights'}):return False
  w=cert['weights']
  if len(w)!=len(rows) or not canonical(w):return False
  combined=[sum(c*r[j] for c,r in zip(w,rows))%p for j in range(n+2)]
  if kind=='inconsistent':return combined==[0]*(n+1)+[1]
  k=cert['scalar']
  x=cert['point']
  return canonical([k]) and len(x)==n+1 and canonical(x) and x[-1]==k and combined==[0]*n+[1,(-k)%p] and all((sum(v*y for v,y in zip(row[:-1],x))+row[-1])%p==0 for row in rows)
 if kind=='undetermined':
  if set(cert)!={'kind','point','direction'}:return False
  x=cert['point'];d=cert['direction']
  if len(x)!=n+1 or len(d)!=n+1 or not canonical(x+d) or x[-1]!=0 or d[-1]!=1:return False
  return all((sum(v*y for v,y in zip(row[:-1],x))+row[-1])%p==0 and sum(v*y for v,y in zip(row[:-1],d))%p==0 for row in rows)
 return False
