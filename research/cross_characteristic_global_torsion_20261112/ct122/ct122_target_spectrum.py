"""CT122 ordinary-target trace and local CRT controls; independent math oracle."""
import json
P = 11

def characteristic(a):
    a1,a2,a3,a4,a6=a
    b2=a1*a1+4*a2
    b4=2*a4+a1*a3
    b6=a3*a3+4*a6
    b8=a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    return -(b2*b2*b8+8*b4**3+27*b6*b6-9*b2*b4*b6)

def num_f2(a):
    a1,a2,a3,a4,a6=(v%2 for v in a)
    return 1 + sum((y*y+a1*x*y+a3*y-x*x*x-a2*x*x-a4*x-a6)%2==0 for x in range(2) for y in range(2))

def group_order(t,n):
    s0,s1=2,t
    if n==1: return 3-t
    for i in range(2,n+1): s0,s1=s1,t*s1-2*s0
    return 2**n+1-s1

def mv(v,t):
    x,y=v
    return (-2*y)%P,(x+t*y)%P

def orbit(v,t,sign):
    x,y=v; w=v
    for k in range(1,201):
        w=mv(w,t)
        if w==v or (sign and w==((-x)%P,(-y)%P)): return k
    raise AssertionError((v,t,sign))

def eigen(t):
    return sorted(r for r in range(1,P) if (r*r-t*r+2)%P==0)

def order(a):
    r=1
    for i in range(1,11):
        r=r*a%P
        if r==1:return i
    raise AssertionError(a)

original=(0,-1,1,-10,-20)
models={
    'supersingular_existing':original,
    'ordinary_trace_plus_one':(59,-1,60,-10,39),
    'ordinary_trace_minus_one':(59,58,60,-10,39)
}
rs={}
for name,a in models.items():
    assert tuple(v%59 for v in a)==tuple(v%59 for v in original)
    delta=characteristic(a)
    assert delta%59 ==characteristic(original)%59 !=0
    assert delta%2 !=0
    t=3-num_f2(a)
    classes={}
    for x in range(11):
        for y in range(11):
            if (x,y)==(0,0):continue
            d=orbit((x,y),t,False)
            k=orbit((x,y),t,True)
            classes[(d,k)]=classes.get((d,k),0)+1
    assert sum(classes.values())==120
    es=eigen(t)
    for a0 in es:
        assert ((a0*a0-t*a0+2)%11)==0
    rs[name]={"curve_coeffs":list(a),"discriminant_mod59":delta%59,"discriminant_mod2":delta%2,"f2_points":num_f2(a),"trace":t,"frobenius_eigenvalues_mod11":es,"eigenvalue_orders":[order(r) for r in es],"f32_group_order":group_order(t,5),"torsion_degree_distribution":[{"point":d,"x":k,"count":c} for (d,k),c in sorted(classes.items())]}
assert rs['supersingular_existing']['torsion_degree_distribution']==[{'point':40,'x':20,'count':120}]
for name in ('ordinary_trace_plus_one','ordinary_trace_minus_one'):
    assert rs[name]['torsion_degree_distribution']==[{'point':5,'x':5,'count':10},{'point':10,'x':5,'count':10},{'point':10,'x':10,'count':100}]
    assert rs[name]['f32_group_order']%11 ==0
print(json.dumps(rs,sort_keys=True,separators=(',',':')))
