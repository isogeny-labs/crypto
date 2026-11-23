import sys,json,time,platform,scipy,numpy,sympy
sys.path.insert(0,sys.argv[1]); from lp import *
out={'versions':dict(python=platform.python_version(),scipy=scipy.__version__,numpy=numpy.__version__,sympy=sympy.__version__)}
res=[]; t0=time.time()
def rec(**d): d['delta_needed']='0.169-0.2'; res.append(d); print(json.dumps(d),flush=True)
# A0 controls
for d in (2,3,5):
    f={(d,):1,(1,):3,(0,):5}
    for l in (1,2,4):
        r=solve([f],lambda de:[de],[d*l+d-1],l,[1]); rec(arm='A0_univ',d=d,l=l,**r)
bd={(0,0):1,(1,0):13,(1,1):1}
for l in (1,2,3,4):
    r=solve([bd],lambda de:[de,0.5],[2*l,l],l,[1,1]); rec(arm='A0_BD',l=l,**r)
s3=S3(0,1,None,2,zc=5)
for l in (1,2,3,4):
    best=None
    for o in ([1,1],[1,2],[2,1]):
        r=solve([s3],lambda de:[de,de],[2*l,2*l],l,o)
        if best is None or r['delta']>best['delta']: best=dict(r,order=o)
    rec(arm='A0_S3block',l=l,**best)
# joint
n=6; g1=S3(0,1,4,n); g2=S3(2,3,5,n); g3=S3(4,5,None,n,zc=5)
orders=[[1,1,1,1,1,1],[1,1,1,1,0,0],[2,2,2,2,1,1],[1,1,1,1,3,3],[0,0,0,0,1,1],[3,3,3,3,1,1]]
for l in [1,2]:
    for arm,s in (('A1',0),('A2',1),('A3',2*l)):
        best=None
        for o in orders:
            t=time.time()
            r=solve([g1,g2,g3],lambda de:[de]*4+[1,1],[2*l]*4+[2*l]*2,l,o,mult_s=s,bigvars=(4,5))
            r['order']=o; r['sec']=round(time.time()-t,1)
            print(' ',arm,l,o,r['delta'],r['dim'],r['sec'],flush=True)
            if best is None or r['delta']>best['delta']: best=r
        rec(arm=arm,l=l,s=s,**best)
out['results']=res; out['runtime_s']=time.time()-t0
json.dump(out,open(sys.argv[1]+'/RESULTS.json','w'),indent=1)
