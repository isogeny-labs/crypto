"""Exact modular certificates for U*x + b*k + c = 0; no native graph dependency."""
def domain(p,n,rows):
 if type(p) is not int or not 2<=p<=1000000 or any(p%d==0 for d in range(2,int(p**.5)+1)):raise ValueError('supported prime required')
 if type(n) is not int or not 0<=n<=2048 or len(rows)>512:raise ValueError('matrix bounds')
 if any(len(r)!=n+2 or any(type(x)is not int or not 0<=x<p for x in r) for r in rows):raise ValueError('canonical matrix')
def solve(p,n,rows):
 domain(p,n,rows);m=len(rows);width=n+2
 a=[r[:]+[int(i==j) for j in range(m)] for i,r in enumerate(rows)];pivot={};rank=0
 for col in range(n+1):
  selected=next((i for i in range(rank,m) if a[i][col]),None)
  if selected is None:continue
  a[rank],a[selected]=a[selected],a[rank];inv=pow(a[rank][col],-1,p);a[rank]=[(x*inv)%p for x in a[rank]]
  for i in range(m):
   if i!=rank and a[i][col]:
    c=a[i][col];a[i]=[(x-c*y)%p for x,y in zip(a[i],a[rank])]
  pivot[col]=rank;rank+=1
 for row in a[rank:]:
  if row[n+1]:
   inv=pow(row[n+1],-1,p);return {'kind':'inconsistent','weights':[(x*inv)%p for x in row[width:]]}
 point=[0]*(n+1)
 for col,i in pivot.items():point[col]=(-a[i][n+1])%p
 if n in pivot:
  row=a[pivot[n]];return {'kind':'determined','weights':row[width:],'scalar':(-row[n+1])%p,'point':point}
 direction=[0]*n+[1]
 for col,i in pivot.items():direction[col]=(-a[i][n])%p
 return {'kind':'undetermined','point':point,'direction':direction}
