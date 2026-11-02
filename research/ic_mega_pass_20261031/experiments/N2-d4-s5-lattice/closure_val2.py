import sys,json; sys.path.insert(0,'../MP-lattices-3')
from lp import solve, S3
out={}
s3=S3(0,1,None,2,zc=5)
for l,L in ((1,4),(2,8),(3,12)):
    r=max((solve([s3],lambda d:[d,d],[L,L],l,o) for o in ([1,1],[-1,-1],[1,-1])),key=lambda r:r['delta']); out[f'S3_l{l}_L{L}']=r; print(l,L,r)
json.dump(out,open('raw/closure_validate_S3.json','w'),indent=1)
