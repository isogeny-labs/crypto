import sys,json; sys.path.insert(0,sys.argv[1]); from lp import *
n=6; g1=S3(0,1,4,n); g2=S3(2,3,5,n); g3=S3(4,5,None,n,zc=5)
for l in (1,2):
  for ub in ('delta','0.5*delta... skip'):
    pass
  for name,bf in (('uv_bound=delta',lambda de:[de]*6),('uv_bound=0.25',lambda de:[de]*4+[0.25,0.25]),('uv_bound=0.5',lambda de:[de]*4+[0.5,0.5]),('uv_bound=1',lambda de:[de]*4+[1,1])):
    r=solve([g1,g2,g3],bf,[2*l]*6,l,[1,1,1,1,1,1],mult_s=2*l,bigvars=(4,5))
    print(json.dumps(dict(arm='sanity_joint',l=l,bound=name,**r)),flush=True)
for d in (2,3,5):
  for l in (1,2,4):
    f={(i,):i+1 for i in range(d+1)}
    r=solve([f],lambda de:[de],[d*l+d-1],l,[1]); print(json.dumps(dict(arm='A0_univ_dense',d=d,l=l,**r)))
