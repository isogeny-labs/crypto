class ZDDLimit(RuntimeError):
 def __init__(self,reason,stats):super().__init__(reason);self.stats=dict(stats)

class Manager:
 def __init__(self,n,max_nodes=100000,max_cache=500000,max_work=2000000):
  if any(type(x)is not int or x<0 for x in [n,max_nodes,max_cache,max_work])or max_nodes<2:raise ValueError('invalidZDDlimits')
  self.n=n;self.nodes=[(n,0,0),(n,1,1)];self.unique={};self.xcache={};self.mcache={};self.work=0;self.max_nodes=max_nodes;self.max_cache=max_cache;self.max_work=max_work
 def stats(self):return {'nodes':len(self.nodes),'cache_entries':len(self.xcache)+len(self.mcache),'work':self.work}
 def tick(self):
  self.work+=1
  if self.work>self.max_work:raise ZDDLimit('workcap',self.stats())
 def make(self,v,lo,hi):
  if hi==0:return lo
  assert 0<=v<self.n and self.nodes[lo][0]>v and self.nodes[hi][0]>v
  key=(v,lo,hi)
  if key not in self.unique:
   if len(self.nodes)>=self.max_nodes:raise ZDDLimit('nodecap',self.stats())
   self.unique[key]=len(self.nodes);self.nodes.append(key)
  return self.unique[key]
 def split(self,a,v):return self.nodes[a][1:]if self.nodes[a][0]==v else(a,0)
 def save(self,cache,key,value):
  if key not in cache and len(self.xcache)+len(self.mcache)>=self.max_cache:raise ZDDLimit('cachecap',self.stats())
  cache[key]=value;return value
 def xor(self,a,b):
  self.tick()
  if a==0:return b
  if b==0:return a
  if a==b:return 0
  key=(min(a,b),max(a,b))
  if key in self.xcache:return self.xcache[key]
  v=min(self.nodes[a][0],self.nodes[b][0]);a0,a1=self.split(a,v);b0,b1=self.split(b,v);value=self.make(v,self.xor(a0,b0),self.xor(a1,b1));return self.save(self.xcache,key,value)
 def mul(self,a,b):
  self.tick()
  if a==0 or b==0:return 0
  if a==1:return b
  if b==1 or a==b:return a
  key=(min(a,b),max(a,b))
  if key in self.mcache:return self.mcache[key]
  v=min(self.nodes[a][0],self.nodes[b][0]);a0,a1=self.split(a,v);b0,b1=self.split(b,v);lo=self.mul(a0,b0);hi=self.xor(self.xor(self.mul(a0,b1),self.mul(a1,b0)),self.mul(a1,b1));return self.save(self.mcache,key,self.make(v,lo,hi))
 def evaluate(self,roots,x):
  values=[0,1]
  for v,lo,hi in self.nodes[2:]:values.append(values[lo]^(((x>>v)&1)&values[hi]))
  return [values[r]for r in roots]
 def degrees_terms(self,roots):
  degrees=[-1,0];terms=[0,1]
  for v,lo,hi in self.nodes[2:]:degrees.append(max(degrees[lo],1+degrees[hi]));terms.append(terms[lo]+terms[hi])
  return [degrees[r]for r in roots],[terms[r]for r in roots]
 def monomials(self,root):
  cache={0:frozenset(),1:frozenset([0])}
  def visit(i):
   if i not in cache:
    v,lo,hi=self.nodes[i];cache[i]=visit(lo)|frozenset(m|(1<<v)for m in visit(hi))
   return cache[i]
  return visit(root)

def compile_cones(C,outputs,**caps):
 M=Manager(C.n,**caps);needed=set();stack=list(outputs)
 while stack:
  i=stack.pop()
  if i in needed:continue
  needed.add(i);node=C.nodes[i]
  if node[0]in['and','xor','not']:stack.extend(node[1:])
 forms={0:0,1:1}
 for i in sorted(needed):
  if i<2:continue
  kind,*args=C.nodes[i]
  if kind=='input':value=M.make(args[0],0,1)
  elif kind=='xor':value=M.xor(forms[args[0]],forms[args[1]])
  elif kind=='and':value=M.mul(forms[args[0]],forms[args[1]])
  else:assert kind=='not';value=M.xor(forms[args[0]],1)
  forms[i]=value
 return M,[forms[i]for i in outputs]
