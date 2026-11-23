from pathlib import Path
import importlib.util,json
root=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('o44',root.parent/'round44-p192/orbit.py');o=importlib.util.module_from_spec(s);s.loader.exec_module(o)
spec=json.loads((root/'scope.json').read_text());sel=json.loads((root.parent/'round46-p192/certificate.json').read_text())['rows'][1];lam=sel['lambda'];sel=sel['selected'];B=4
secret=20261107;Q=o.old.mul(secret,o.G)
def point(a,b):return o.old.add(o.old.mul(a%o.r,o.G),o.old.mul(b%o.r,Q))
def enc(P):return [o.old.mul(pow(lam,j,o.r),P) for j in range(3)]
def neg(P,F):return None if P is None else (P[0],F.sub(0,P[1]))
def canon(T,F):
 if T[0] is None:return [None]*3,0,1
 j=min(range(5),key=lambda i:T[i][0]);minus=F.sub(0,T[j][1]);sg=1 if T[j][1]<minus else -1
 out=[T[(j+k)%5] for k in range(3)]
 if sg<0:out=[neg(P,F) for P in out]
 return out,j,sg
def batch(pairs,F,layers):
 out,branches=o.add_many(pairs,F);layers.append(branches);return out

def represent(states,F,layers):
 # Tables have the same width four for each of three input points.
 assert sel['widths']==[4,4,4]
 inputs=[P for T in states for P in T];twice=batch([(P,P) for P in inputs],F,layers);tables=[[P] for P in inputs]
 for _ in range(3):
  vals=batch([(t[-1],d) for t,d in zip(tables,twice)],F,layers)
  for t,v in zip(tables,vals):t.append(v)
 ds=sel['digits'];X=[None]*B;initialized=False
 for bit in range(max(map(len,ds))-1,-1,-1):
  if initialized:X=batch([(P,P) for P in X],F,layers)
  for k in range(3):
   d=ds[k][bit] if bit<len(ds[k]) else 0
   if not d:continue
   vals=[tables[3*i+k][(abs(d)-1)//2] for i in range(B)]
   if d<0:vals=[neg(P,F) for P in vals]
   if not initialized:X=vals;initialized=True
   else:X=batch(list(zip(X,vals)),F,layers)
 total=[T[0] for T in states]
 for vals in [[T[1] for T in states],[T[2] for T in states],X]:total=batch(list(zip(total,vals)),F,layers)
 fifth=[neg(P,F) for P in total]
 return [canon(T+[x,z],F) for T,x,z in zip(states,X,fifth)]
coeffs=[(1+17*i,3+11*i) for i in range(16)];table=[enc(point(a,b)) for a,b in coeffs];states=[];labels=[]
for i in range(B):
 a,b=1+101*i,7+103*i;P=point(a,b);T,j,sg=canon([o.old.mul(pow(lam,k,o.r),P) for k in range(5)],o.Field());z=sg*pow(lam,j,o.r)%o.r;states.append(T);labels.append([z*a%o.r,z*b%o.r])
initial={'states':states,'labels':labels};steps=[];seen={};repeats=[]
for step in range(8):
 F=o.Field();layers=[];indices=[o.partition(T[0]) for T in states]
 sums=batch([(P,U) for T,idx in zip(states,indices) for P,U in zip(T,table[idx])],F,layers);translated=[sums[3*i:3*i+3] for i in range(B)];reps=represent(translated,F,layers)
 nextlabels=[];nextstates=[];orient=[]
 for i,(T,j,sg) in enumerate(reps):
  a,b=labels[i];c,d=coeffs[indices[i]];z=sg*pow(lam,j,o.r)%o.r;L=[z*(a+c)%o.r,z*(b+d)%o.r]
  assert T==enc(point(*L))
  key=o.key(T[0]);previous=seen.get(key)
  if previous is not None:repeats.append({'step':step,'walker':i,'labels':L,'previous':previous,'fruitless':L==previous['labels']})
  else:seen[key]={'step':step,'walker':i,'labels':L}
  nextstates.append(T);nextlabels.append(L);orient.append([j,sg])
 states,labels=nextstates,nextlabels
 steps.append({'states':states,'labels':labels,'orientations':orient,'indices':indices,'cost':F.cost,'layers':layers})
 print(step,F.cost,'layers',len(layers),flush=True)
(root/'certificate.json').write_text(json.dumps({'p':o.p,'r':o.r,'lambda':lam,'G':o.G,'Q':Q,'table':table,'coefficients':coeffs,'initial':initial,'steps':steps,'repeats':repeats,'setup':'stored initial triples and table excluded; scalar-label and hash operations separate'},indent=2)+'\n')
