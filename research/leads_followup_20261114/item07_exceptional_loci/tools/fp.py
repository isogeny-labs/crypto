import sys, json
sys.path.insert(0,'.')
from census import *
p=int(sys.argv[1]); tt=int(sys.argv[2]); ks=[int(x) for x in sys.argv[3].split(',')]; outf=sys.argv[4]
out=open(outf,'w')
for al in range(p):
    for be in range(p):
        a,b,t=al*tt*tt%p,be*pow(tt,3,p)%p,tt
        if (4*a**3+27*b*b)%p==0 or len(s3_coeffs(p,a,b,t))!=9 or (t**3+a*t+b)%p==0: continue
        rec=dict(alpha=al,beta=be)
        for k in ks:
            d,one=closure_dim(p,a,b,t,k); rec[f'd{k}']=d; rec[f'o{k}']=one
        out.write(json.dumps(rec)+'\n'); out.flush()
