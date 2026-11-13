#!/usr/bin/env python3
"""CT73 finite control: consistent vs independent second-prime orientations.
No general cross-characteristic encoded-input algorithm is constructed.
"""
import json

M=0x17b

def field_mul(a,b):
    c=0
    while b:
        if b&1: c^=a
        a<<=1
        if a&0x100: a^=M
        b>>=1
    return c

def field_pow(a,n):
    r=1
    while n:
        if n&1:r=field_mul(r,a)
        a=field_mul(a,a)
        n>>=1
    return r

def frob(z):
    return field_mul(z,z)

def point_on_curve(z):
    x,y=z
    return field_mul(y,y)^y == field_mul(field_mul(x,x),x)^field_mul(x,x)

def fixed_witness():
    base=(19,2)
    orbit=[]
    z=base
    for _ in range(8):
        if not point_on_curve(z):raise ValueError('not a binary curve point')
        orbit.append(z)
        z=(frob(z[0]),frob(z[1]))
    if z!=base or len(set(orbit))!=8:raise ValueError('incorrect orbit')
    if not all(orbit[(i+4)%8] == (p[0],p[1]^1) for i,p in enumerate(orbit)):
        raise ValueError('half Frobenius must act as inversion on this subgroup')
    counts={'independent_correct':0,'independent_wrong_scalar':0,'independent_outside_subgroup':0}
    for i,pi in enumerate(orbit):
        for j,pj in enumerate(orbit):
            q=(pj[0],pj[1]^1) # Q=-P transported using independently selected j-th embedding
            if q==(pi[0],pi[1]^1):
                counts['independent_correct']+=1
            elif q==pi:
                counts['independent_wrong_scalar']+=1
            else:
                counts['independent_outside_subgroup']+=1
    if counts!={'independent_correct':8,'independent_wrong_scalar':8,'independent_outside_subgroup':48}:
        raise ValueError(str(counts))
    return {'case':'ct69_fixed_order3','target_field':256,'base_point':list(base),'orientations':8,
            'paired_choices':64,'same_place_valid':8,
            'frobenius_halfturn_equals_negation':True,**counts}

def matvec(M,v,q):
    return ((M[0]*v[0]+M[1]*v[1])%q,(M[2]*v[0]+M[3]*v[1])%q)

def full_image(q):
    # Enumerate every GL2 element, not merely the orbit formula.
    mats=[]
    for a in range(q):
      for b in range(q):
       for c in range(q):
        for d in range(q):
         if (a*d-b*c)%q:
          mats.append((a,b,c,d))
    n=q*q-1
    if len(mats)!=(q*q-1)*(q*q-q):raise ValueError('GL2 count')
    P=(1,0)
    images=set()
    for g in mats:
        images.add(matvec(g,P,q))
        for x in range(q):
            gx=matvec(g,((x*P[0])%q,(x*P[1])%q),q)
            gp=matvec(g,P,q)
            if gx != ((x*gp[0])%q,(x*gp[1])%q):
                raise ValueError('one map must preserve every scalar')
    if len(images)!=n or (0,0) in images:raise ValueError('image orbit not transitive')
    matches=sum(1 for u in images for v in images if v==((2*u[0])%q,(2*u[1])%q))
    if matches!=n:raise ValueError('independent orientations count wrong')
    return {'case':'abstract_full_image','ell':q,'group_size':len(mats),
            'nonzero_vectors':n,'shared_maps_checked':len(mats),
            'independent_pairs':n*n,'independent_correct':matches,
            'probability_denominator':n}

def main():
    for row in [fixed_witness()]+[full_image(q) for q in [3,5,7,11]]:
        print(json.dumps(row,sort_keys=True,separators=(',',':')))
if __name__=='__main__': main()
