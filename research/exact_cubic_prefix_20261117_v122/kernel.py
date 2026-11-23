from research.residue_quotient_20261116_v99b.kernel import pmul
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.triangular_prefix_quadratic_20261116_v121.kernel import solve as quadratic

def evaluate(H,x):
    value=0
    for a in reversed(H):value=pmul(value,x)^a
    return value

def translate(H,c):
    out=[H[-1]]
    for a in reversed(H[:-1]):
        new=[0]*(len(out)+1)
        for j,value in enumerate(out):new[j]^=pmul(value,c);new[j+1]^=value
        new[0]^=a;out=new
    return out

def support(H,m):
    assert m>=1 and len(H)==4 and H[3]==1
    failure={'supported':False,'roots':None,'all_domain_roots_claim':False}
    if any(a.bit_length()>(3-j)*(m-1)+1 for j,a in enumerate(H)):return failure
    if m==1:
        work=list(H);roots=[]
        for root in [0,1]:
            multiplicity=0
            while len(work)>1 and evaluate(work,root)==0:
                Q=[0]*(len(work)-1);Q[-1]=1
                for j in range(len(work)-2,0,-1):Q[j-1]=work[j]^pmul(root,Q[j])
                assert work[0]^pmul(root,Q[0])==0;work=Q;multiplicity+=1
            if multiplicity:roots.append((root,multiplicity))
        return {'supported':True,'roots':roots,'all_domain_roots_claim':True}if len(work)==1 else failure
    G=[reverse(a,(3-j)*(m-1))for j,a in enumerate(H)];signature=(G[2]&1,G[1]&1,G[0]&1);kind={(0,0,0):0,(1,0,0):1,(0,1,0):2,(1,1,1):3}.get(signature)
    if kind is None:return failure
    if kind==0:return support(H,m-1)
    if kind==3:
        top=1<<(m-1);result=support(translate(H,top),m-1)
        if result['supported']:return {'supported':True,'roots':sorted((r^top,mult)for r,mult in result['roots']),'all_domain_roots_claim':True}
        return failure
    series=int(kind==1);assert((G[1]&1)^series)==1
    for n in range(1,m):series^=((evaluate(G,series)>>n)&1)<<n
    root=reverse(series,m-1)
    if evaluate(H,root):return failure
    a=H[2]^root;b=H[1]^pmul(root,a);assert H[0]^pmul(root,b)==0;Q=quadratic(a,b,m)
    if not Q['roots']:return failure
    counts={root:1}
    for r,mult in Q['roots']:counts[r]=counts.get(r,0)+mult
    assert sum(counts.values())==3
    return {'supported':True,'roots':sorted(counts.items()),'all_domain_roots_claim':True}
