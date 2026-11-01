import numpy as np, scipy.fft as sf
r=13903; rng=np.random.default_rng(1)
m1=np.zeros(r,bool); m1[rng.choice(r,300,replace=False)]=True
m2=np.zeros(r,bool); m2[rng.choice(r,300,replace=False)]=True
X=np.rint(sf.irfft(sf.rfft(m1*1.0)*sf.rfft(m2*1.0),n=r)).astype(int)
i1=np.nonzero(m1)[0]; bad=0
for R in rng.integers(0,r,50):
    x=int(m2[(R-i1)%r].sum()); bad+= (x!=X[R])
print('brute mismatches',bad,'of 50, max X',X.max())
