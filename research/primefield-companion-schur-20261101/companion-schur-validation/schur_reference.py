import numpy as np

def inverse_or_null(A,r):
 n=len(A);W=np.concatenate((A.copy(),np.eye(n,dtype=np.int64)),axis=1);piv=[]
 for j in range(n):
  candidates=[i for i in range(len(piv),n) if W[i,j]%r]
  if not candidates:continue
  row=len(piv);i=candidates[0];W[[row,i]]=W[[i,row]]
  W[row]=W[row]*pow(int(W[row,j]),-1,r)%r
  for k in range(n):
   if k!=row and W[k,j]:W[k]=(W[k]-W[k,j]*W[row])%r
  piv.append(j)
 if len(piv)==n:
  Inv=W[:,n:];I=np.eye(n,dtype=np.int64)
  assert np.array_equal(A@Inv%r,I) and np.array_equal(Inv@A%r,I)
  return Inv,None
 free=next(j for j in range(n) if j not in piv);w=np.zeros(n,dtype=np.int64);w[free]=1
 for i,j in enumerate(piv):w[j]=-W[i,free]%r
 assert np.any(w) and not np.any(A@w%r)
 return None,w.tolist()

def oracles(Inv,r):
 stats=dict(forward_calls=0,transpose_calls=0,dense_terms=0,coefficient_reductions=0)
 def call(x,tr):
  stats['transpose_calls' if tr else 'forward_calls']+=1
  if Inv is None:raise ZeroDivisionError('reference leading block is singular')
  k=len(x);stats['dense_terms']+=k*k;stats['coefficient_reductions']+=k
  return ((Inv.T if tr else Inv)@np.array(x,dtype=np.int64)%r).tolist()
 return lambda x:call(x,False),lambda x:call(x,True),stats
