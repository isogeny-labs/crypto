from itertools import product
from research.image_cells_20261115_v67.algebra import BField
class PF:
 def __init__(self,p):self.p=p
 def add(self,a,b):return(a+b)%self.p
 def neg(self,a):return(-a)%self.p
 def mul(self,a,b):return a*b%self.p
 def sq(self,a):return a*a%self.p
class F9:
 p=3
 def add(self,a,b):return(a%3+b%3)%3+3*((a//3+b//3)%3)
 def neg(self,a):return(-a%3)+3*((- (a//3))%3)
 def mul(self,a,b):return(a%3*(b%3)+2*(a//3)*(b//3))%3+3*((a%3*(b//3)+(a//3)*(b%3))%3)
 def sq(self,a):return self.mul(a,a)
def s3(x,y,z,A,B,f):
 add=f.add;mul=f.mul;sq=f.sq;sub=lambda a,b:add(a,f.neg(b));scale=lambda a,n:mul(a,n%f.p)
 if f.p==2:return sq(mul(x,y))^sq(mul(x,z))^sq(mul(y,z))^mul(mul(x,y),z)^B
 xy=mul(x,y);s=add(x,y)
 return sub(add(sub(mul(sq(sub(x,y)),sq(z)),mul(scale(add(mul(s,add(xy,A)),scale(B,2)),2),z)),sq(sub(xy,A))),scale(mul(B,s),4))
def points_and_tags(f,q,A,B):
 pts={};tags={}
 for x in range(q):
  rhs=f.add(f.mul(f.sq(x),x),f.add(f.mul(A,f.sq(x)if f.p==2 else x),B));ys=[y for y in range(q)if f.add(f.sq(y),f.mul(x,y)if f.p==2 else 0)==rhs];pts[x]=ys;tags[x]='T'if(x==0 if f.p==2 else rhs==0)else('R'if ys else'A')
 return pts,tags
def components(values,tags):
 parent=list(range(9))
 def root(i):
  while parent[i]!=i:i=parent[i]
  return i
 for edge in [(0,1,5),(5,2,6),(6,3,7),(7,4,8)]:
  active=[i for i in edge if tags[values[i]]!='T']
  for i in active[1:]:parent[root(i)]=root(active[0])
 out={}
 for i,x in enumerate(values):
  if tags[x]!='T':out.setdefault(root(i),[]).append(i)
 return list(out.values())
def local_oracles():
 rows=[];witness=None
 cases=[(BField(n,m),1<<n,A,1,'GF2^'+str(n))for n,m in [(2,7),(3,11),(4,19)]for A in [0,1]]+[(PF(p),p,A,B,'F'+str(p))for p in [3,5,7,11,17]for A,B in [(1,1),(1,0)]]+[(F9(),9,1,B,'F9')for B in [0,1]]
 for f,q,A,B,name in cases:
  pts,tags=points_and_tags(f,q,A,B);valid=0
  for x,y,z in product(range(q),repeat=3):
   if s3(x,y,z,A,B,f):continue
   labels={tags[t]for t in [x,y,z]if tags[t]!='T'};assert len(labels)<=1;valid+=1
   if 'A'in labels and 'T'in [tags[x],tags[y],tags[z]]and witness is None:witness={'field':name,'A':A,'B':B,'triple':[x,y,z],'tags':[tags[t]for t in [x,y,z]]}
  rows.append({'field':name,'A':A,'B':B,'valid_triples':valid,'torsion_x':[x for x in tags if tags[x]=='T']})
 return {'cases':rows,'valid_triples':sum(r['valid_triples']for r in rows),'torsion_cut_counterexample':witness}
