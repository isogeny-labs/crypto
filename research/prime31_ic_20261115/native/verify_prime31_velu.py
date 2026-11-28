#!/usr/bin/env python3
"""Independent producer-side prime31 Vélu certificate check.

This verifier uses Python integers, not the native Rust EC or mapped source.
It does not solve ECDLP or certify any performance claim. [pcc:unverified]
"""
import json
import sys

P = 2147483647
N = 2147446831
A = 847514846
B = 2087444724
G = (625037715, 2100627251)
KERNEL = 533591203
DUAL_X = 546710038
EXPECTED_A_B = (323891735, 1985248359)
EXPECTED_G = (1767020261, 634125456)
X_ROOTS = {1890447397, 1697269373, 546710038, 160540486}
SCALARS = [1, 2, 3, 5, 7, 11, 13, 17, 19, 31, 37, 101, 257, 65537, 1234567, N - 1]


def inv(x):
    if x % P == 0:
        raise ZeroDivisionError("kernel denominator")
    return pow(x, P-2, P)


def on_curve(q, a, b):
    if q is None:
        return True
    x, y = q
    return (y*y-x*x*x-a*x-b) % P == 0


def add_points(u, v, a):
    if u is None: return v
    if v is None: return u
    x, y = u
    z, w = v
    if x == z and (y+w) % P == 0: return None
    if x == z:
        m = (3*x*x+a)*inv(2*y) % P
    else:
        m = (w-y)*inv(z-x) % P
    r = (m*m-x-z) % P
    return (r, (m*(x-r)-y) % P)


def mul_point(k, q, a):
    out = None
    while k:
        if k & 1: out = add_points(out, q, a)
        q = add_points(q, q, a)
        k >>= 1
    return out


def coeff(a,b,x):
    t = (3*x*x+a) % P
    w = (5*x*x*x+3*a*x+2*b) % P
    return ((a-10*t) % P, (b-14*w) % P)


def psi3(a,b,x):
    return (3*pow(x,4,P)+6*a*x*x+12*b*x-a*a) % P


def morphism(q,a,b,u):
    if q is None: return None
    x,y=q
    if x == u: raise ZeroDivisionError("geometric kernel")
    h = inv(x-u)
    t = (3*u*u+a) % P
    f = (u*u*u+a*u+b) % P
    x2 = (x+2*t*h+4*f*h*h) % P
    y2 = (y*(1-2*t*h*h-8*f*h*h*h)) % P
    return (x2,y2)


def poly3_roots_match(a,b,roots):
    # Coefficients of 3*product(X-root_i), ascending.
    c = [3]
    for r in roots:
        n = [0]*(len(c)+1)
        for i,v in enumerate(c):
            n[i] = (n[i]-v*r) % P
            n[i+1] = (n[i+1]+v) % P
        c = n
    return c == [(-a*a)%P,(12*b)%P,(6*a)%P,0,3]


def verify(data):
    assert data["standing"] == "[pcc:unverified]"
    assert data["p"] == P and data["n"] == N
    assert data["source_curve"] == [A,B]
    assert data["source_generator"] == list(G)
    assert data["kernel_x"] == KERNEL
    assert psi3(A,B,KERNEL) == 0
    assert pow((KERNEL**3+A*KERNEL+B)%P,(P-1)//2,P) == P-1
    assert coeff(A,B,KERNEL) == EXPECTED_A_B
    aa,bb = EXPECTED_A_B
    assert data["target_curve"] == [aa,bb]
    assert on_curve(G,A,B) and on_curve(EXPECTED_G,aa,bb)
    assert morphism(G,A,B,KERNEL) == EXPECTED_G
    assert data["target_generator"] == list(EXPECTED_G)
    assert set(data["target_three_division_x_roots"]) == X_ROOTS
    assert poly3_roots_match(aa,bb,X_ROOTS)
    assert all(psi3(aa,bb,r)==0 for r in X_ROOTS)
    assert data["dual_kernel_x"] == DUAL_X
    assert coeff(aa,bb,DUAL_X)==((81*A)%P,(729*B)%P)
    assert data["dual_codomain_coefficients"] == [(81*A)%P,(729*B)%P]
    assert mul_point(N,G,A) is None
    assert mul_point(N,EXPECTED_G,aa) is None
    assert data["endomorphism_conductors"] == {"source":3,"target":1}
    assert data["full_ic_target_logs_recovered"] == 0
    assert data["full_ic_timing_performed"] is False and data["speedup_claim"] is False
    assert data["public_scalar_controls"] == len(SCALARS)
    for k in SCALARS:
        q=mul_point(k,G,A)
        image=morphism(q,A,B,KERNEL)
        assert image==mul_point(k,EXPECTED_G,aa)
        assert on_curve(image,aa,bb)
        back=morphism(image,aa,bb,DUAL_X)
        three=mul_point(3,q,A)
        assert back==((9*three[0])%P,(27*three[1])%P)
    # Distinct tampering checks and original-vs-new generator inequality.
    assert psi3(A,B,KERNEL+1)!=0
    assert EXPECTED_G!=G
    return {"standing":"[pcc:unverified]","verifier":"independently authored Python big-integer arithmetic","scalar_controls":len(SCALARS),"dual_compositions_checked":len(SCALARS),"target_four_rational_3_division_x":4,"status":"producer-side exact replay passed","full_solver_timing":False}


if __name__=="__main__":
    if len(sys.argv)!=2: raise SystemExit("usage: verify_prime31_velu.py <Rust JSONL>")
    lines=[s for s in open(sys.argv[1],encoding="utf8") if s.strip()]
    assert len(lines)==1, "expected one exact-source JSON readout"
    print(json.dumps(verify(json.loads(lines[0])),sort_keys=True))
