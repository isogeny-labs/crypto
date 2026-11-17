"""Exact nonzero five-source reciprocal cell join; no original-root claim."""
def coefficients(f,roots):
 p=[1]
 for x in roots:
  q=[0]*(len(p)+1)
  for i,v in enumerate(p):q[i]^=f.mul(v,x);q[i+1]^=v
  p=q
 return p

def solve(f,domains,z,cap=65536,work_cap=1<<24):
 assert len(domains)==5 and z
 q=f.order;domains=[tuple(d)for d in domains]
 assert all(len(set(d))==len(d)and all(0<x<q for x in d)for d in domains)
 assert len(domains[0])*len(domains[1])+len(domains[2])*len(domains[3])*len(domains[4])<=work_cap
 inv={x:f.inv(x)for d in domains for x in d};assert all(f.mul(x,v)==1 for x,v in inv.items());iz=f.inv(z);assert f.mul(iz,z)==1
 pairs={}
 for a in domains[0]:
  for b in domains[1]:pairs.setdefault(inv[a]^inv[b],[]).append((a,b))
 roots=[];work=len(domains[0])*len(domains[1]);truncated=False
 for a in domains[2]:
  ia=inv[a]^iz
  for b in domains[3]:
   ib=ia^inv[b]
   for c in domains[4]:
    work+=1
    for x,y in pairs.get(ib^inv[c],()):
     if len(roots)>=cap:truncated=True;break
     roots.append((x,y,a,b,c))
    if truncated:break
   if truncated:break
  if truncated:break
 if truncated:return dict(status='inconclusive',complete=False,tuples=roots,lower_bound=len(roots),exact_count=None,work=work)
 assert len(roots)==len(set(roots))
 for row in roots:
  H=coefficients(f,row);assert H[0]==f.mul(z,H[1])
 return dict(status='complete',complete=True,tuples=roots,exact_count=len(roots),lower_bound=len(roots),work=work)
