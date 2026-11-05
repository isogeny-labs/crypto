from l08lib import *
from ecc2k import toy
import numpy as np, time, json
inst = toy(19); c = inst.curve; f = c.field; r = inst.order
t=time.time()
P = inst.P; pts=[None]; cur=None
idx={}
cur=None
enc=np.zeros(r,dtype=np.int64)
for i in range(r):
    if i==0: continue
    cur = c.add(cur, P) if cur is not None else P
    x,y = cur
    idx[cur]=i
    enc[i] = 2*x + (f.div(y,x) & 1)
assert c.add(cur,P) is None, "order mismatch"
x,y = P; tau = (f.square(x), f.square(y)); lam = idx[tau]
# orbit size
o=1; v=lam
while v!=1: v=v*lam%r; o+=1
print("r",r,"lambda",lam,"order of lambda",o,"secs",round(time.time()-t,1), "a,b", c.a, c.b)
ii=np.arange(r,dtype=np.int64); cm=enc.copy(); cur=ii.copy()
for _ in range(o-1):
    cur=cur*lam%r; cm=np.minimum(cm,enc[cur])
cm[0]=0; enc[0]=0
np.savez("toy19.npz",enc=enc,cenc=cm,lam=lam,r=r,o=o)
print("fixed pts under tau (orbit size 1):", int(sum(1 for i in range(1,r) if i*lam%r==i)))
