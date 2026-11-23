"""Draw m=2 Kummer-tower systems (and nulls). Prints JSON lines: kind,p,t,seed,coeffs(3x3),grid_zeros,..."""
import sys, json, random
import numpy as np
def S3coeffs(a,b,xR,g,p):
    # S3 = (x1-x2)^2 xR^2 - 2((x1+x2)(x1x2+a)+2b) xR + (x1x2-a)^2 - 4b(x1+x2); x_i = g z_i
    c=[[0]*3 for _ in range(3)]
    def add(e1,e2,v): c[e1][e2]=(c[e1][e2]+v)%p
    add(2,0,xR*xR); add(0,2,xR*xR); add(1,1,-2*xR*xR)
    add(2,1,-2*xR); add(1,2,-2*xR); add(1,0,-2*a*xR); add(0,1,-2*a*xR); add(0,0,-4*b*xR)
    add(2,2,1); add(1,1,-2*a); add(0,0,a*a); add(1,0,-4*b); add(0,1,-4*b)
    return [[c[e1][e2]*pow(g,e1+e2,p)%p for e2 in range(3)] for e1 in range(3)]
def grid_zeros(c,p,t):
    K=1<<t; h=pow(3,(p-1)//K,p)
    # find generator of mu_K: need element of exact order K
    for base in range(2,500):
        h=pow(base,(p-1)//K,p)
        if K==1 or pow(h,K//2,p)!=1: break
    z=np.array([pow(h,k,p) for k in range(K)],dtype=object)
    zi=[np.ones(K,dtype=object),z,(z*z)%p]
    # f(z1,z2)=sum_e1 zi[e1](z1) * (sum_e2 c zi[e2](z2))
    tot=0
    inner=[ (sum(c[e1][e2]*zi[e2] for e2 in range(3)))%p for e1 in range(3)]
    for k1 in range(K):
        row=(inner[0]+zi[1][k1]*inner[1]+zi[2][k1]*inner[2])%p
        tot+=int(np.count_nonzero(row==0))
    return tot
def draw(kind,p,t,seed):
    rng=random.Random(seed)
    a,b,g,xR=[rng.randrange(1,p) for _ in range(4)]
    if kind=='kummer':
        c=S3coeffs(a,b,xR,g,p)
    elif kind=='null':
        while True:
            c=[[rng.randrange(1,p) for _ in range(3)] for _ in range(3)]
            M=[[c[2][2],c[2][1]],[c[1][2],c[1][1]]]   # bilinear top part in (y11,y10)x(y21,y20)
            if (M[0][0]*M[1][1]-M[0][1]*M[1][0])%p: break
    elif kind=='summand':
        c=[[rng.randrange(1,p),0,0],[rng.randrange(1,p),0,0],[rng.randrange(1,p),0,0]]
    elif kind=='planted':
        K=1<<t; h=pow(rng.randrange(2,p),(p-1)//K,p)
        x1=g*pow(h,rng.randrange(K),p)%p; x2=g*pow(h,rng.randrange(K),p)%p
        # quadratic in xR: A xR^2 + B xR + C
        A=(x1-x2)**2%p; B=-2*((x1+x2)*(x1*x2+a)+2*b)%p; C=((x1*x2-a)**2-4*b*(x1+x2))%p
        import sympy
        r=sympy.polys.galoistools.gf_zassenhaus([A%p,B%p,C%p],p,sympy.polys.domains.ZZ) if A else []
        roots=[(-f[1]*pow(int(f[0]),-1,p))%p for f in r if len(f)==2]
        if not roots: return None
        c=S3coeffs(a,b,roots[0],g,p); xR=roots[0]
    nondeg=(3*xR*xR+a)%p!=0
    return dict(kind=kind,p=p,t=t,seed=seed,coeffs=c,grid_zeros=grid_zeros(c,p,t),nondeg=nondeg)
if __name__=='__main__':
    kind,p,t,s0,n=sys.argv[1],int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]),int(sys.argv[5])
    for s in range(s0,s0+n):
        d=draw(kind,p,t,s)
        if d: print(json.dumps(d))
