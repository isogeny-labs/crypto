#!/usr/bin/env python3
"""Independent exact-bigint, all-case replay of original-Rust x-only Horner gate.
No native modules, no shared EC helpers, no plaintext witness supplied to recovery.
Usage: python3 verify_prime31_horner_xonly_sign_lift.py native-result.json
"""
import json
import sys

P=2147483647
A=847514846
B=2087444724
N=2147446831
G=(625037715,2100627251)
SEED=0x584f4e4c595f4348
WINS=[list(range(8)),list(range(8,16)),list(range(16,24)),list(range(23,31))]
MASK=(1<<64)-1

def rhs(x):
    return (x**3+A*x+B)%P
def neg(point):
    return None if point is None else (point[0],-point[1]%P)
def plus(p,q):
    if p is None: return q
    if q is None: return p
    x,y=p
    z,w=q
    if x==z and (y+w)%P==0:return None
    if x==z:
        slope=(3*x*x+A)*pow(2*y,-1,P)%P
    else:
        slope=(w-y)*pow((z-x)%P,-1,P)%P
    xx=(slope*slope-x-z)%P
    return (xx,(slope*(x-xx)-y)%P)
def times(k,p):
    q=None
    while k:
        if k&1:q=plus(q,p)
        p=plus(p,p)
        k>>=1
    return q
def splitmix(s):
    s=(s+0x9e3779b97f4a7c15)&MASK
    z=s
    z=((z^(z>>30))*0xbf58476d1ce4e5b9)&MASK
    z=((z^(z>>27))*0x94d049bb133111eb)&MASK
    return s,z^(z>>31)
def base31():
    out=[]
    for x in range(10000):
        z=rhs(x)
        y=pow(z,(P+1)//4,P)
        if y*y%P==z and y:
            out.append((x,min(y,P-y)))
            if len(out)==31:return out
    raise AssertionError("factor base not found")
def poly_N(x):
    return (x**4-2*A*x*x-8*B*x+A*A)%P
def poly_D(x):
    return 4*rhs(x)%P
def f3(u,v,r):
    return ((r-u)**2*v*v-2*((r+u)*(r*u+A)+2*B)*v+
            (r*u-A)**2-4*B*(r+u))%P
def eliminated_f3(u,z,r):
    # A*N^2+B*N*D+C*D^2: independently derive the rational elimination.
    n=poly_N(z); d=poly_D(z)
    aa=(r-u)**2
    bb=-2*((r+u)*(r*u+A)+2*B)
    cc=(r*u-A)**2-4*B*(r+u)
    return (aa*n*n+bb*n*d+cc*d*d)%P

def verify_elimination(u,z,r):
    d=poly_D(z)
    assert d!=0
    v=poly_N(z)*pow(d,P-2,P)%P
    fv=f3(u,v,r)
    e=eliminated_f3(u,z,r)
    assert e==(fv*d*d)%P
    assert (e==0)==(fv==0)

def lift_signed(xleaf,x_suffix,Q,base,window):
    """Only PUBLIC x-chart, PUBLIC Q and public canonical factor-base points."""
    if x_suffix[0]!=Q[0] or xleaf[7]!=x_suffix[7]:
        return None
    chosen=[None]*8
    tail=next(pt for j in window if (pt:=base[j])[0]==xleaf[7])
    chosen[7]=tail
    for i in range(6,-1,-1):
        pivot=next(pt for j in window if (pt:=base[j])[0]==xleaf[i])
        doubled=plus(tail,tail)
        compatible=[]
        for leaf in (pivot,neg(pivot)):
            candidate=plus(leaf,doubled)
            if candidate and candidate[0]==x_suffix[i]:
                compatible.append((leaf,candidate))
        if not compatible: return None
        chosen[i],tail=compatible[0]
    if tail!=Q:
        if neg(tail)!=Q:return None
        chosen=[neg(pt) for pt in chosen]
    verify=None
    for i,pt in enumerate(chosen):
        verify=plus(verify,times(1<<i,pt))
    assert verify==Q
    return chosen

def main(path):
    with open(path,encoding="utf-8") as f:
        raw=json.load(f)
    assert raw["schema"]=="prime31.xonly_horner_sign_lift.v1"
    assert raw["curve"]["p"]==P and raw["curve"]["n"]==N
    assert raw["curve"]["a"]==A and raw["curve"]["b"]==B
    assert tuple(raw["curve"]["g"])==G
    assert raw["seed"]==f"0x{SEED:016x}"
    assert raw["variables_after_public_x_target"]==21
    assert raw["equations_after_public_x_target"]==22
    assert raw["eliminated_variables_after_public_target"]==14
    assert raw["eliminated_equations_after_public_target"]==15
    assert raw["eliminated_total_degree"]==10
    assert raw["eliminated_z_degree"]==8
    assert raw["eliminated_u_and_r_degree"]==2
    assert raw["uniform_field_triples_equivalence_checked"]==8192
    assert raw["cleared_denominator_rational_roots_only"]
    assert raw["generic_sum_table_calls"]==0
    assert not raw["public_only_unknown_index_extraction"]
    assert times(N,G) is None
    base=base31()
    assert [x for x,y in base[:8]]==[0,1,2,3,5,6,8,9]
    s=SEED
    ordinary=exceptional=0
    eliminated_checked=0
    certs=[]
    counts=[]
    ncases=raw["cases_per_window"]
    assert 1<=ncases<=256
    for wi,window in enumerate(WINS):
        yes=no=0
        for ci in range(ncases):
            planted=[]
            for j in range(8):
                s,r=splitmix(s)
                x,y=base[window[r&7]]
                planted.append((x,y if not r&8 else -y%P))
            tail=[None]*8
            tail[7]=planted[7]
            for i in range(6,-1,-1):
                tail[i]=plus(planted[i],plus(tail[i+1],tail[i+1]))
            if any(t is None for t in tail):
                no+=1
                exceptional+=1
                continue
            xleaf=[pt[0] for pt in planted]
            tx=[pt[0] for pt in tail]
            dz=[plus(tail[i+1],tail[i+1])[0] for i in range(7)]
            assert all(poly_D(tx[i+1])*dz[i]%P==poly_N(tx[i+1]) for i in range(7))
            assert all(f3(xleaf[i],dz[i],tx[i])==0 for i in range(7))
            assert all(eliminated_f3(xleaf[i],tx[i+1],tx[i])==0 for i in range(7))
            for i in range(7):
                verify_elimination(xleaf[i],tx[i+1],tx[i])
                eliminated_checked+=1
            assert all(any(base[j][0]==x for j in window) for x in xleaf)
            assert poly_D(tx[1])*(dz[0]+1)%P!=poly_N(tx[1])
            recovered=lift_signed(xleaf,tx,tail[0],base,window)
            assert recovered is not None
            yes+=1
            ordinary+=1
            if len(certs)<16:
                certs.append({"window":wi,"case":ci,"x_leaves":xleaf,
                    "x_suffix":tx,"x_doubled":dz,"target":tail[0],
                    "recovered_signed_leaves":recovered})
        counts.append({"window":wi,"ordinary":yes,"exceptional":no})
    assert ordinary==raw["ordinary"] and exceptional==raw["exceptional"]
    assert raw["tested"]==4*ncases
    assert raw["eliminated_steps_verified"]==eliminated_checked==ordinary*7
    state=0x454c494d5f443134
    for _ in range(8192):
        state,bits=splitmix(state);u=bits%P
        state,bits=splitmix(state);z=bits%P
        state,bits=splitmix(state);r=bits%P
        verify_elimination(u,z,r)
    assert raw["poisoned_double_rejected"]==ordinary
    assert raw["poisoned_target_rejected"]==ordinary
    assert raw["per_window"]==counts
    assert raw["certificates"]==json.loads(json.dumps(certs))
    print(json.dumps({"schema":"prime31.xonly_horner_independent_replay.v1",
       "all_case_verified":raw["tested"],"ordinary":ordinary,"exceptional":exceptional,
       "certificate_exact_matches":len(certs),"prime31_order_check":True,
       "sign_lifting_independently_replayed":True,
       "eliminated_14var_steps_replayed":eliminated_checked,
       "uniform_field_triples_replayed":8192,
       "pcc_signed":False},
       sort_keys=True))

if __name__=="__main__":
    if len(sys.argv)!=2:raise SystemExit("pass original native JSON")
    main(sys.argv[1])
