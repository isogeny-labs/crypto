#!/usr/bin/env sage -python
"""Nonsplit quadratic-pole V4 descent over F16, with an exact q16 source."""
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, is_prime

HERE=Path(__file__).resolve().parent
R=PolynomialRing(GF(2),"z")
z=R.gen()
L=GF(2**24,modulus=z**24+z**16+z**15+z**14+z**13+z**10+z**9+z**7+z**5+z**3+1,name="z")
u=L.gen()
q=16

def dec(n):
    return sum(((n>>i)&1)*u**i for i in range(24))

def code(x):
    return sum(int(c)<<i for i,c in enumerate(x.polynomial().list()))

def subfield(n):
    h=L.multiplicative_generator()**((q**6-1)//(q**n-1))
    return [L(0)]+[h**i for i in range(q**n-1)]

def absolute_trace_K(x):
    return sum((x**(2**i) for i in range(12)),L(0))

K=subfield(3)
k2=subfield(2)
k=[x for x in K if x**q==x]
assert len(K)==q**3 and len(k2)==q**2 and len(k)==q

def run():
    t=dec(16191718)
    A=dec(2832837)
    a=dec(5522909)
    b=dec(9159130)
    s=A*A**(q**3)
    assert t+t**q==1 and t**(q*q)==t and t not in k
    assert A+A**(q*q)+A**(q**4)==0
    assert A**(q**3)!=A and s*s==b and b**(q**3)==b
    assert a**q==a and absolute_trace_K(a)==1
    d=pow(2*(q+1),-1,q**3-1)
    assert absolute_trace_K(b**d)==0

    RX=PolynomialRing(L,"x")
    x=RX.gen()
    f0=A/(x+t)+A**(q**3)/(x+t**q)
    def frob_func(f):
        num=sum((c**q*x**i for i,c in enumerate(f.numerator())),L(0))
        den=sum((c**q*x**i for i,c in enumerate(f.denominator())),L(0))
        return num/den
    f1=frob_func(f0)
    f2=frob_func(f1)
    assert frob_func(f2)==f0 and f0+f1+f2==0
    assert all(c**(q**3)==c for c in list(f0.numerator())+list(f0.denominator()))
    N_C0=2+sum(2 for xx in K if absolute_trace_K(f0(xx))==0)
    N_E=2+sum(2 for xx in K if xx and
              absolute_trace_K(xx+a+b/(xx*xx))==0)
    assert N_C0==N_E==4054 and is_prime(2027)
    E=EllipticCurve(L,[L(1),a,L(0),L(0),b])
    roots={vv*vv+vv:vv for vv in K}
    G=None
    for xx in sorted(K,key=code):
        if not xx:continue
        rhs=xx+a+b/(xx*xx)
        if rhs not in roots:continue
        P=E(xx,xx*roots[rhs])
        H=2*P
        if H!=E(0) and 2027*H==E(0):
            G=H
            break
    assert G is not None
    Q=7*G
    assert Q!=E(0) and 2027*Q==E(0)

    alpha=next(x for x in sorted(K,key=code) if x and x+x**q+x**(q*q)==0)
    p=alpha+alpha**(q*q)
    v=alpha**q+alpha**(q*q)
    assert len({L(0),p,v,p+v})==4
    RU=PolynomialRing(L,"U")
    U=RU.gen()
    Lpoly=U*(U+p)*(U+v)*(U+p+v)
    l1,l2=Lpoly[1],Lpoly[2]
    F=p**4*(f0+f0**2)+v**4*(f1+f1**2)+l2*(p**2*f0+v**2*f1)
    num,den=F.numerator(),F.denominator()
    assert all(c**q==c for c in list(num)+list(den)+[l1,l2])

    # Exhaustive q16 distinct-norm and triangle-overlap counts.
    W0=L.gen()+L.gen()**(q*q)
    line={c*W0 for c in k2}
    W1=next((z+z**(q*q) for z in
             (L.multiplicative_generator()**i for i in range(1,50))
             if z+z**(q*q) not in line))
    norm_values={w*w**(q**3) for c in k2 for h in k2
                 if (w:=W0*c+W1*h)}
    outside=sum(1 for ss in norm_values if absolute_trace_K((ss*ss)**d)==0)
    assert len(norm_values)==2055 and outside==975
    assert 4095+1088+outside==6158

    result={"schema":"ghs-nonsplit-quadratic-poles-q16-v1",
            "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            "field_modulus":str(L.modulus()),"base_q":q,
            "source_a":code(a),"source_b":code(b),
            "source_order":N_E,"subgroup_prime":2027,
            "source_G":[code(G[0]),code(G[1])],
            "source_Q":[code(Q[0]),code(Q[1])],
            "pole_t":code(t),"pole_tq":code(t**q),
            "residue_A":code(A),"residue_Aq3":code(A**(q**3)),
            "sqrt_b":code(s),"f0":str(f0),"f1":str(f1),
            "descent_alpha":code(alpha),"descent_p":code(p),
            "descent_v":code(v),
            "additive_quartic_coefficients_low_to_high":[code(Lpoly[i]) for i in range(5)],
            "rhs_numerator_coefficients_low_to_high":list(map(code,num.list())),
            "rhs_denominator_coefficients_low_to_high":list(map(code,den.list())),
            "quotient_order":N_C0,
            "population_q16":{"distinct_b":len(norm_values),
                             "outside_triangle_models":outside,
                             "three_gate_union_models":6158,
                             "all_normalized_ordinary_models":8190}}
    (HERE/"nonsplit_4054.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps({key:result[key] for key in
                      ("source_a","source_b","source_order","subgroup_prime",
                       "source_G","source_Q","pole_t","residue_A",
                       "population_q16")}))

if __name__=="__main__":
    run()
