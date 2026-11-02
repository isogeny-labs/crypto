import sys,json,itertools; from lp2 import *
n=6; g1=S3(0,1,4,n); g2=S3(2,3,5,n); g3=S3(4,5,None,n,zc=5)
orders=[[1,1,1,1,1,1],[2,2,2,2,1,1],[3,3,3,3,1,1],[1,1,1,1,3,3]]
l=int(sys.argv[1]); ub=int(sys.argv[2]) if len(sys.argv)>2 else 2*l
for allowed in [(0,),(2,),(0,1),(0,2),(0,1,2)]:
  best=0
  for o in orders:
    r=solve([g1,g2,g3],lambda de:[de]*4+[1,1],[2*l]*4+[ub]*2,l,o,mult_s=None,bigvars=(4,5),allowed=allowed)
    best=max(best,r['delta'])
  print(json.dumps(dict(l=l,ubox=ub,allowed=allowed,best_delta=best)),flush=True)
