#!/usr/bin/env python3
"""CT91: independent target-field cubic splitting and full E[7] partition."""
import json
MOD=0x1225
def mul12(a,b):
    out=0
    while b:
        if b&1:out^=a
        b>>=1;a<<=1
        if a&4096:a^=MOD
    return out&4095
def pow12(a,n):
    out=1
    while n:
        if n&1:out=mul12(out,a)
        a=mul12(a,a);n>>=1
    return out
def tr12(a):
    r=0
    for _ in range(12):r^=a;a=mul12(a,a)
    return r
one=(1,0)
g=mul12(2,mul12(2,2))^mul12(2,2)
assert g and tr12(g)==1
def add(a,b):return(a[0]^b[0],a[1]^b[1])
def prod(a,b):
    u,v=a;x,y=b
    w=mul12(v,y)
    return(mul12(u,x)^mul12(w,g),mul12(u,y)^mul12(v,x)^w)
def inv(a):
    u,v=a;t=mul12(u,u)^mul12(u,v)^mul12(mul12(v,v),g)
    assert t
    w=pow12(t,4094)
    return(mul12(u^v,w),mul12(v,w))
def fcurve(P):
    if P is None:return True
    x,y=P
    return add(prod(y,y),y)==add(prod(prod(x,x),x),prod(x,x))
def plus(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and add(y,v)==one:return None
    if P==Q:
        lam=prod(x,x);nu=add(y,prod(lam,x))
    else:
        lam=prod(add(y,v),inv(add(x,u)));nu=add(y,prod(lam,x))
    xx=add(add(add(prod(lam,lam),one),x),u)
    yy=add(add(prod(lam,xx),nu),one)
    assert fcurve((xx,yy))
    return(xx,yy)
def negate(P):
    if P is None:return None
    return(P[0],add(P[1],one))
def frob(P,steps=1):
    if P is None:return None
    x,y=P
    for _ in range(steps):x=prod(x,x);y=prod(y,y)
    return(x,y)
def line(P):
    assert fcurve(P)
    a=None;out=set()
    for i in range(1,8):
        a=plus(a,P)
        if i<7:
            assert a is not None
            out.add(a)
    assert a is None and len(out)==6
    return frozenset(out)
as_solutions={}
for a in range(4096):as_solutions.setdefault(mul12(a,a)^a,[]).append(a)
assert len(as_solutions)==2048 and all(len(v)==2 for v in as_solutions.values())
def cubic(x,c):
    return mul12(x,mul12(x,x))^mul12(1545,mul12(x,x))^mul12(1906,x)^c
results=[];seedlines=[]
for e,c in ((240,1261),(241,1260)):
    roots=[x for x in range(4096) if cubic(x,c)==0]
    assert len(roots)==3
    assert roots[0]^roots[1]^roots[2]==1545
    assert mul12(roots[0],roots[1])^mul12(roots[0],roots[2])^mul12(roots[1],roots[2])==1906
    assert mul12(mul12(roots[0],roots[1]),roots[2])==c
    for x in roots:assert tr12(mul12(x,mul12(x,x))^mul12(x,x))==1
    points=frozenset((((x,0),(y,1)) for x in roots for y in as_solutions[(mul12(x,mul12(x,x))^mul12(x,x))^g]))
    assert len(points)==6
    P=min((P for P in points if P[0][0]==roots[0]),key=lambda P:P[1][0])
    assert points==line(P)
    seedlines.append(P)
    results.append({"idempotent":e,"cubic_constant":c,"x_roots":roots,"ordinate_trace":1,"line_order":7})
assert results[0]["x_roots"]==[2,17,1562]
assert results[1]["x_roots"]==[122,383,1804]
alllines=[];lineorbits=[]
for P in seedlines:
    Ls=[line(frob(P,i)) for i in range(4)]
    assert len(set(Ls))==4
    lineorbits.append(Ls);alllines+=Ls
assert len(set(alllines))==8
assert all(not alllines[i].intersection(alllines[j]) for i in range(8) for j in range(i+1,8))
allpoints=set().union(*alllines)
assert len(allpoints)==48
assert all(frob(P,24)==P and frob(P,12)==negate(P) for P in allpoints)
frob_mult=[]
for P in seedlines:
    Q=frob(P,4);vals=[];v=None
    for k in range(7):
        if v==Q:vals.append(k)
        v=plus(v,P)
    assert len(vals)==1 and vals[0] in (3,4)
    frob_mult.append(vals[0])
def source_add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and (y+v+1)%37==0:return None
    if P==Q:
        lam=(3*x*x-2*x-10)*pow(2*y+1,-1,37)%37
    else:
        lam=(v-y)*pow(u-x,-1,37)%37
    nu=(y-lam*x)%37
    X=(lam*lam+1-x-u)%37
    Y=(-lam*X-nu-1)%37
    assert (Y*Y+Y-X**3+X*X+10*X+20)%37==0
    return(X,Y)
source=[None];target=[None]
for i in range(1,7):
    source.append(source_add(source[-1],(9,4)))
    target.append(plus(target[-1],seedlines[0]))
assert source==[None,(9,4),(27,9),(28,0),(28,36),(27,27),(9,32)]
assert [[list(v) for v in p] if p else None for p in target]==[None,
    [[2,0],[0,1]],[[17,0],[77,1]],[[1562,0],[2879,1]],
    [[1562,0],[2878,1]],[[17,0],[76,1]],[[2,0],[1,1]]]
assert len(target)==len(set(target))==7 and all(target.index(P)==i for i,P in enumerate(target))
assert source_add(source[-1],(9,4)) is None and plus(target[-1],seedlines[0]) is None
assert [len(x) for x in lineorbits]==[4,4]
assert frob_mult==[3,3]
assert all(frob(seedlines[0],i)!=seedlines[0] for i in (1,2,3,4,6,8,12))
res={"experiment":"ct91-binary-7-torsion-partition","selected":240,"pairs":results,"frobenius_line_orbit_sizes":[4,4],"frob4_scalar_per_seed":frob_mult,"disjoint_lines":8,"nonzero_torsion_points":48,"toy_public_log_cases":7,"target_scan_used_for_verification":True,"field_degree":24,"selected_target_x":2,"unselected_prime_compatible_with_curve":True,"scalar_free_source_recognition":False}
print(json.dumps(res,sort_keys=True,separators=(',',':')))
