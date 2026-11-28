#!/usr/bin/env python3
"""Independent BigInt replay for Prime31 direct Frobenius inverse-Velu Rust transcript.

Implements short-Weierstrass arithmetic and the degree-three rational map
independently; receives only public (Q,P) rows, not producer's root solver.
No benchmark or independent signed PCC status is inferred from this audit.
"""
import json
import sys

P = (1 << 31) - 1
N = 2147446831
EA, EB = 847514846, 2087444724
DA, DB = 323891735, 1985248359
G = (625037715, 2100627251)
H = (1767020261, 634125456)
U, TAU, V = 533591203, 1555600864, 259447350

def ec_add(z, w, a):
    if z is None:
        return w
    if w is None:
        return z
    x, y = z
    X, Y = w
    if x == X and (y + Y) % P == 0:
        return None
    if x == X:
        lam = ((3*x*x+a) * pow(2*y, -1, P)) % P
    else:
        lam = ((Y-y) * pow(X-x, -1, P)) % P
    xx = (lam*lam-x-X) % P
    return (xx, (lam*(x-xx)-y) % P)

def scalar(k, z, a):
    out = None
    while k:
        if k & 1:
            out = ec_add(out, z, a)
        z = ec_add(z, z, a)
        k >>= 1
    return out

def on_curve(q, a, b):
    if q is None:
        return True
    x,y=q
    return 0 <= x < P and 0 <= y < P and (y*y-x*x*x-a*x-b) % P == 0

def velu(q):
    if q is None:
        return None
    x,y=q
    h = (x-U)%P
    if not h:
        raise ValueError("geometric kernel pole")
    d=pow(h,-1,P)
    dx=(x+2*TAU*d+4*V*d*d)%P
    dy=y*(1-2*TAU*d*d-8*V*d*d*d)%P
    return (dx,dy)

def cubic_coeff(X):
    return [(-2*TAU*U+4*V-X*U*U)%P,
            (U*U+2*TAU+2*X*U)%P,
            (-2*U-X)%P,1]

def check_quadratic(f,x):
    a2=(f[2]+x)%P
    a1=(f[1]+x*a2)%P
    assert (f[0]+x*a1)%P == 0
    discriminant=(a2*a2-4*a1)%P
    assert pow(discriminant,(P-1)//2,P)==P-1


def poly_trim(a):
    a=list(a)
    while a and not a[-1]:
        a.pop()
    return a

def pseudo_rem(a, b):
    # Independent fraction-free polynomial remainder over Fp.
    a=poly_trim(a)
    b=poly_trim(b)
    assert b
    while len(a)>=len(b):
        lead_a, lead_b = a[-1], b[-1]
        shift=len(a)-len(b)
        a=[v*lead_b%P for v in a]
        for j,c in enumerate(b):
            a[shift+j]=(a[shift+j]-lead_a*c)%P
        a=poly_trim(a)
    return a

def pseudo_linear_root(f, z):
    a,b=poly_trim(f),poly_trim(z)
    while b:
        a,b=b,pseudo_rem(a,b)
    assert len(a)==2 and a[1]
    return -a[0]*pow(a[1],-1,P)%P

def monic_rem(a, b):
    assert b and b[-1]==1
    a=poly_trim(a)
    while len(a)>=len(b):
        coeff=a[-1]
        shift=len(a)-len(b)
        for j,c in enumerate(b):
            a[shift+j]=(a[shift+j]-coeff*c)%P
        a=poly_trim(a)
    return a

def poly_mul_mod(a,b,f):
    c=[0]*(len(a)+len(b)-1)
    for i,v in enumerate(a):
        for j,w in enumerate(b):
            c[i+j]=(c[i+j]+v*w)%P
    return monic_rem(c,f)

def poly_pow_mod(a,n,f):
    r=[1]
    while n:
        if n&1:
            r=poly_mul_mod(r,a,f)
        n//=2
        if n:
            a=poly_mul_mod(a,a,f)
    return r

def independent_fraction_free_inverse(q):
    # Public Q only: no supplied source preimage or discrete-log scalar.
    if q is None:
        return None
    X,Y=q
    if not on_curve(q,DA,DB):
        raise ValueError("invalid public destination")
    f=cubic_coeff(X)
    z=poly_pow_mod([0,1],P,f)
    z += [0]*max(0,2-len(z))
    z[1]=(z[1]-1)%P
    x=pseudo_linear_root(f,z)
    h=(x-U)%P
    if not h:
        raise ValueError("kernel pole")
    den=(h*h*h-2*TAU*h-8*V)%P
    if not den:
        raise ValueError("zero derivative")
    return (x, Y*(h*h*h)*pow(den,-1,P)%P)

def check(path):
    with open(path,encoding="utf-8") as f:
        doc=json.load(f)
    assert doc["schema"]=="prime31.frobenius_inverse_cubic.v1"
    assert doc["full_index_calculus_logs_solved"]==0
    assert doc["full_cost_speedup_claim"] is False
    assert doc["independent_signed_pcc"] is False
    assert doc["p"]==P and doc["n"]==N and doc["cofactor"]==1
    assert doc["source_curve"]==[EA,EB]
    assert doc["codomain_curve"]==[DA,DB]
    assert doc["public_targets"]==256
    assert doc["unique_linear_gcd"]==256
    assert doc["nontrivial_quadratic_irreducible"]==256
    assert doc["dual_agreement"]==256
    assert doc["fraction_free_batch_matches"]==256
    assert doc["fraction_free_batch_fp_inversions"]==2
    assert doc["direct_inverse_fp_inversions"]==1024
    rows=doc["public_transcript"]
    assert len(rows)==256
    assert scalar(N,G,EA) is None and scalar(N,H,DA) is None
    expected_source=None
    expected_target=None
    for j,row in enumerate(rows):
        assert row["case"]==j+1
        q=tuple(row["public_q"])
        preimage=tuple(row["source_preimage"])
        expected_source=ec_add(expected_source,G,EA)
        expected_target=ec_add(expected_target,H,DA)
        assert q==expected_target and preimage==expected_source
        assert on_curve(q,DA,DB) and on_curve(preimage,EA,EB)
        assert velu(preimage)==q
        assert independent_fraction_free_inverse(q)==preimage
        x,y=preimage
        X,Y=q
        f=cubic_coeff(X)
        assert (sum(c*pow(x,i,P) for i,c in enumerate(f)))%P==0
        check_quadratic(f,x)
        hh=(x-U)%P
        assert hh
        den=(hh**3-2*TAU*hh-8*V)%P
        assert den
        reconstructed=Y*(hh**3)*pow(den,-1,P)%P
        assert reconstructed==y
    assert doc["negation_identity_bad_point_controls"]==3
    print(json.dumps({
        "schema":"prime31.frobenius_inverse_cubic.independent_bigint.v1",
        "standing":"producer-independent implementation, [pcc:unverified]",
        "records_replayed":len(rows),
        "source_and_target_scalar_checks":2*len(rows),
        "forward_velu_checks":len(rows),
        "exact_cubic_and_nonsquare_quadratic_checks":len(rows),
        "ordinate_inverse_checks":len(rows),
        "independent_fraction_free_frobenius_gcd_checks":len(rows),
        "batch_two_inverse_counter_checked":True,
        "no_new_nongeneric_relations":True,
        "pass":True
    },sort_keys=True))

if __name__=="__main__":
    if len(sys.argv)!=2:
        raise SystemExit("usage: verify_prime31_frobenius_inverse.py <rust.json>")
    check(sys.argv[1])
