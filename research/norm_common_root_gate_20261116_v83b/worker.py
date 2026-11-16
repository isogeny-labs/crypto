import json,time,hashlib
from .kernel import norm,gate,power
from .common import P,finish
from research.image_cells_20261115_v67.algebra import BField,pmul,evaluate
from research.torsion_cut_20261116_v74.core import PF,F9,points_and_tags
START=time.monotonic()
class F27:
 p=3
 def add(self,a,b):return sum(((a//(3**i)+b//(3**i))%3)*3**i for i in range(3))
 def neg(self,a):return sum((-(a//(3**i))%3)*3**i for i in range(3))
 def mul(self,a,b):
  c=[0]*5
  for i in range(3):
   for j in range(3):c[i+j]=(c[i+j]+(a//3**i%3)*(b//3**j%3))%3
  for i in [4,3]:c[i-3]=(c[i-3]+2*c[i])%3;c[i-2]=(c[i-2]+c[i])%3
  return sum(c[i]*3**i for i in range(3))
 def sq(self,a):return self.mul(a,a)

def main():
 out={'cases':0,'primitive':0,'constant_b':0,'common_rational':0,'common_anti':0,'unresolved_b_zero':0,'field_cases':[],'witnesses':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());fs=[(BField(n,M),1<<n,A,'GF2^'+str(n))for n,M in d['binary_fields']for A in [0,1]]+[(PF(p),p,1,'F'+str(p))for p in d['prime_fields']]+[(F9(),9,1,'F9'),(F27(),27,1,'F27')]
  for f,q,A,name in fs:
   assert all(f.mul(x,power(x,q-2,f))==1 for x in range(1,q));pts,tags=points_and_tags(f,q,A,1);out['field_cases'].append({'field':name,'q':q,'A':A,'B':1})
   for i in range(d['cases_per_configuration']):
    rand=lambda j:int.from_bytes(hashlib.sha256((str(d['seed'])+name+str(A)+':'+str(i)+':'+str(j)).encode()).digest(),'big')%q
    r=rand(0);h=1+rand(1)%(q-1);b=[f.mul(h,f.neg(r)),h];kind=i%4
    if kind==0:
     a=[rand(2),rand(3),rand(4),1]
     if not evaluate(a,r,f):a[0]=f.add(a[0],1)
     out['primitive']+=1
    elif kind in [1,2]:a=pmul([f.neg(r),1],[rand(2),rand(3),1],f)
    else:a=[rand(2),rand(3),rand(4),1];b=[h];out['constant_b']+=1
    N=norm(a,b,A,1,f);v=gate(a,b,A,1,f,q);assert v['status']=='certified';actual=[x for x in range(q)if evaluate(N,x,f)==0 and tags[x]=='A'];assert actual==v['anti_x'];out['cases']+=1
    if kind in [1,2]:
     key='common_anti'if actual else'common_rational';out[key]+=1
     if actual:
      work=N;mult=0
      while len(work)>1 and evaluate(work,r,f)==0:
       z=[0]*(len(work)-1);z[-1]=work[-1]
       for j in range(len(z)-2,-1,-1):z[j]=f.add(work[j+1],f.mul(r,z[j+1]))
       work=z;mult+=1
      assert mult==2
      if len(out['witnesses'])<8:out['witnesses'].append({'field':name,'A':A,'a':a,'b':b,'norm':N,'anti_x':r,'multiplicity':mult})
   v=gate([0,1],[],A,1,f,q);assert v['status']=='unresolved';out['unresolved_b_zero']+=1
  assert out['common_anti']and out['common_rational'];out['complete']=True
 except Exception as e:error=repr(e)
 finish(out,error,START,'Linear-section common-root gate compared with literal finite-field point and repeated-factor oracles across odd/binary fields.','Recover or retain section coefficients for actual norm cells; compare paid scalar exceptional-root gate against spectral trace only when exact norm certificates are available; keep zero-b branch unresolved until covered.')
if __name__=='__main__':main()
