"""l=1 D4-invariant lattice, fplll LLL, vanishing check."""
import math, subprocess, os, tempfile, time
from build_h import MONOS, WDEG, inv_vals
LOG6=math.log2(6)
C0=sum(2*a+LOG6*b+2*c for (a,b,c,d) in MONOS)   # log2 of C(4,k) constants over support
def xstar_bits(L, n=105):
    """log2 X* from det^(1/n)*sqrt(n) = p, full support, constants included."""
    return (L - C0 - 0.5*n*math.log2(n))/sum(WDEG)
def bounds(X, model='Y'):
    return (4*X**2, 6*X**4, 4*X**6, X**4)
def build(coef, p, Bd, mu0=0):
    inv=pow(coef[mu0],-1,p); h=[x*inv%p for x in coef]
    sc=[Bd[0]**a*Bd[1]**b*Bd[2]**c*Bd[3]**d for (a,b,c,d) in MONOS]
    rows=[[h[j]*sc[j] for j in range(105)]]
    for i in range(105):
        if i==mu0: continue
        r=[0]*105; r[i]=p*sc[i]; rows.append(r)
    return rows, sc
def fplll(rows, tag=''):
    fd,fn=tempfile.mkstemp(suffix='.lat',dir=os.environ.get('TMPDIR','/tmp')); 
    with os.fdopen(fd,'w') as f: f.write('['+'\n'.join('['+' '.join(map(str,r))+']' for r in rows)+']\n')
    t=time.time(); out=subprocess.run(['/opt/homebrew/bin/fplll','-a','lll',fn],capture_output=True,text=True,check=True).stdout
    dt=time.time()-t; os.unlink(fn)
    vecs=[list(map(int,l.strip().strip('[]').split())) for l in out.strip().strip('[]').split('\n') if l.strip().strip('[]')]
    return vecs, dt
def check(vecs, sc, ivals, k=3):
    """unscale first k nonzero vectors; evaluate over Z at invariant integers."""
    res=[]
    nz=[v for v in vecs if any(v)]
    for v in nz[:k]:
        assert all(x%s==0 for x,s in zip(v,sc))
        h=[x//s for x,s in zip(v,sc)]
        val=sum(hj*ivals[0]**a*ivals[1]**b*ivals[2]**c*ivals[3]**d for hj,(a,b,c,d) in zip(h,MONOS))
        res.append({'vanish':val==0,'log2norm':round(sum(x*x for x in v).bit_length()/2,1)})
    return res
