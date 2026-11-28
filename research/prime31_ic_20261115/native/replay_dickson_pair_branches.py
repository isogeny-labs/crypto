#!/usr/bin/env python3
"""Independent producer-local prime31 Dickson/S3 branch replay.

This is NOT the index-calculus oracle: EC group operations are used only
to validate the polynomial roots. No target logarithm or rho comparison.
"""
import json

P = 2**31 - 1
A, B, N = 847514846, 2087444724, 2147446831
D = 6

def f(x):
    return (x*x - 2) % P

def sqrt(x):
    x %= P
    r = pow(x, (P + 1)//4, P)
    return r if r*r % P == x else None

def prime31_base():
    for seed in range(2,10000):
        c = seed
        for _ in range(D):
            c = f(c)
        if c in (2,P-2):
            continue
        vals = [c]
        for _ in range(D):
            out = []
            for y in vals:
                r = sqrt(y + 2)
                if r is not None:
                    out.extend((r, -r % P))
            vals = sorted(set(out))
        if len(vals) != 2**D:
            continue
        points = []
        for x in vals:
            y = sqrt(x**3 + A*x + B)
            if y not in (None,0):
                points.append((x, min(y,P-y)))
        if len(points)>=2:
            return seed, c, vals, points
    raise AssertionError("no split Dickson base")

def collision(x,y):
    assert x!=y
    for j in range(D):
        if x!=y and (x+y)%P==0:
            return j
        x,y=f(x),f(y)
    raise AssertionError("first-collision classification failed")

def s3_roots(x,y):
    aa=(x-y)**2%P
    bb=(-2*((x+y)*(x*y+A)+2*B))%P
    cc=((x*y-A)**2-4*B*(x+y))%P
    if not aa:
        assert bb!=0
        return [(-cc*pow(bb,-1,P))%P]
    disc=(bb**2-4*aa*cc)%P
    assert disc==16*((x**3+A*x+B)%P)*((y**3+A*y+B)%P)%P
    root=sqrt(disc)
    assert root is not None
    inv=pow(2*aa,-1,P)
    return sorted(set(((-bb+root)*inv%P,(-bb-root)*inv%P)))

def ecadd(u,v):
    if u is None: return v
    if v is None: return u
    x,y=u; xx,yy=v
    if x==xx and (y+yy)%P==0: return None
    z=((3*x*x+A)*pow(2*y,-1,P) if x==xx else (yy-y)*pow((xx-x)%P,-1,P))%P
    rx=(z*z-x-xx)%P
    return (rx,(z*(x-rx)-y)%P)

def main():
    seed,c,allx,pts=prime31_base()
    assert (seed,c,len(allx),len(pts)) == (3,933677148,64,31)
    full=[0]*D
    for i,x in enumerate(allx):
        for y in allx[i+1:]:
            full[collision(x,y)]+=1
    filt=[0]*D
    roots_by_branch=[0]*(D+1)
    roots=set()
    for i,(x,yy) in enumerate(pts):
        for y,zz in pts[i:]:
            j=D if x==y else collision(x,y)
            if j<D: filt[j]+=1
            pair=s3_roots(x,y)
            assert len(pair)==(1 if j==D else 2)
            for w in pair:
                assert w not in roots
                roots.add(w);roots_by_branch[j]+=1
            p=ecadd((x,yy),(y,zz))
            assert p is not None and p[0] in pair
            if x!=y:
                q=ecadd((x,yy),(y,-zz%P))
                assert q is not None and q[0] in pair and p[0]!=q[0]
    assert full==[32,64,128,256,512,1024]
    assert filt==[8,10,31,62,120,234]
    assert roots_by_branch==[16,20,62,124,240,468,31]
    assert len(roots)==961
    print(json.dumps({
        "standing":"producer-local independently authored Python arithmetic, not PCC",
        "p":P,"n":N,"seed":seed,"terminal":c,"depth":D,
        "full_fibre":len(allx),"liftable_base":len(pts),
        "full_fibre_first_collision":full,"liftable_first_collision":filt,
        "s3_root_count_per_branch_and_diagonal":roots_by_branch,
        "distinct_s3_abscissa_roots":len(roots),
        "ec_additions":"validation only; no complete ECDLP or index-calculus oracle"
    },sort_keys=True))

if __name__=="__main__":
    main()
