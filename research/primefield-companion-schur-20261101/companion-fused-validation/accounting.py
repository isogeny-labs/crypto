"""Independent ledger reconstruction from public inputs and packed trace.

Digit arithmetic uses quotient/remainder, not the kernel's shifts/masks.
No Meter methods or production helpers are imported.
"""
from collections import defaultdict
from reference import expected_fixed_counts

def reconstruct(G,K,a,b,x,r,tr,trace):
 n=len(x);s=len(G[0]);R=r-1;B=s*n*n*R**4*(a+1)*(b+1)
 d=max(1,(B.bit_length()+7)//8);beta=256**d;w=8*d
 assert len(trace)==s+2 and trace[0]==dict(stage='setup',bound=B,digit_bytes=d,inverse=pow((a-b)%r,-1,r))
 fixed=expected_fixed_counts(r,n,s,a,b,tr);vol=defaultdict(int);products=[]
 def event(op,u,v,z):
  vol[op+'_input_bitlength_sum']+=u.bit_length()
  if op in ('left_shift','right_shift'):vol[op+'_distance_sum']+=v
  else:vol[op+'_input_bitlength_sum']+=v.bit_length()
  vol[op+'_result_bitlength_sum']+=z.bit_length()
 def product(u,v,z,kind):
  assert u*v==z
  products.append(dict(kind=kind,a_bits=u.bit_length(),b_bits=v.bit_length(),product_bits=z.bit_length()))
 def pack(v):return int.from_bytes(b''.join(z.to_bytes(d,'little') for z in v),'little')
 event('subtract',r,1,R)
 product(R,R,R**2,'bound');product(R**2,R**2,R**4,'bound');product(n,n,n*n,'bound')
 z=s*n*n;product(s,n*n,z,'bound');product(z,R**4,z*R**4,'bound');z*=R**4
 event('add',a,1,a+1);product(z,a+1,z*(a+1),'bound');z*=a+1
 event('add',b,1,b+1);product(z,b+1,B,'bound')
 lo=beta**(n-1);hi=beta**n
 event('left_shift',1,(n-1)*w,lo);event('subtract',lo,1,lo-1)
 event('left_shift',1,n*w,hi);event('subtract',hi,1,hi-1)
 shared=pack(list(x) if tr else list(x)[::-1]);u,v=(a,b) if tr else (b,a);acc=0
 for j,row in enumerate(trace[1:-1]):
  assert row['stage']=='column' and row['column']==j
  g=[z[j] for z in G];k=[z[j] for z in K]
  first=g[::-1] if tr else k;second=k[::-1] if tr else g
  P,T,Q,Y=row['P'],row['T'],row['Q'],row['Y']
  product(pack(first),shared,P,'convolution')
  low=P%lo;upper=P//lo;fold=u*low
  event('mask',P,lo-1,low);event('right_shift',P,(n-1)*w,upper)
  product(u,low,fold,'fold');event('left_shift',fold,w,fold*beta)
  assert T==upper+fold*beta;event('add',upper,fold*beta,T)
  raw=T.to_bytes(n*d,'little')
  digits=[int.from_bytes(raw[i*d:(i+1)*d],'little') for i in range(n)]
  product(pack(second),pack(digits[::-1]),Q,'convolution')
  low=Q%hi;upper=Q//hi;fold=v*upper
  event('mask',Q,hi-1,low);event('right_shift',Q,n*w,upper);product(v,upper,fold,'fold')
  assert Y==low+fold;event('add',low,fold,Y)
  event('add',acc,Y,acc+Y);acc+=Y;assert row['acc']==acc
 inv=trace[0]['inverse'];product(inv,acc,inv*acc,'scale')
 assert trace[-1]==dict(stage='final',packed=inv*acc)
 fixed.update(vol)
 fixed['operand_bitlength_product_sum']=sum(z['a_bits']*z['b_bits'] for z in products)
 fixed['product_result_bitlength_sum']=sum(z['product_bits'] for z in products)
 return fixed,products
