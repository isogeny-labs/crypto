"""Reproducible algebraic checks for research Figures 7--10.

The order-557 relations are constructed examples, not collected attack data.
The fiber census exhausts E(F_571). The uniform-Jacobian numerator is obtained
by enumerating distinct-x degree-two divisors and invoking the genus-two
Abel-map uniqueness away from the hyperelliptic pencil; it does not enumerate
every Jacobian class or solve the Certicom challenge.
"""

import csv
import itertools
import json
from math import gcd
from pathlib import Path


class Curve:
    def __init__(self,p,a,b):
        self.p,self.a,self.b=p,a%p,b%p

    def rhs(self,x):
        return (x**3+self.a*x+self.b)%self.p

    def contains(self,P):
        return P is None or P[1]*P[1]%self.p==self.rhs(P[0])

    def neg(self,P):
        return None if P is None else (P[0],-P[1]%self.p)

    def add(self,P,Q):
        if P is None:return Q
        if Q is None:return P
        p=self.p
        x,y=P;u,v=Q
        if x==u:
            if (y+v)%p==0:return None
            slope=(3*x*x+self.a)*pow(2*y,-1,p)%p
        else:slope=(v-y)*pow((u-x)%p,-1,p)%p
        z=(slope*slope-x-u)%p
        return z,(slope*(x-z)-y)%p

    def scale(self,k,P):
        if k<0:return self.scale(-k,self.neg(P))
        out=None
        while k:
            if k&1:out=self.add(out,P)
            k>>=1
            if k:P=self.add(P,P)
        return out


def chi(value,p):
    value%=p
    return 0 if value==0 else 1 if pow(value,(p-1)//2,p)==1 else -1


def sqrt(value,p):
    assert p%4==3
    if chi(value,p)==-1:return None
    root=pow(value%p,(p+1)//4,p)
    return min(root,-root%p)


def rank_mod(matrix,n):
    a=[[x%n for x in row] for row in matrix]
    rank=0
    for j in range(len(a[0])):
        pivot=next((i for i in range(rank,len(a)) if a[i][j]),None)
        if pivot is None:continue
        a[rank],a[pivot]=a[pivot],a[rank]
        inv=pow(a[rank][j],-1,n)
        a[rank]=[x*inv%n for x in a[rank]]
        for i in range(len(a)):
            if i!=rank:
                coefficient=a[i][j]
                a[i]=[(x-coefficient*y)%n for x,y in zip(a[i],a[rank])]
        rank+=1
        if rank==len(a):break
    return rank


def build_data(directory=None):
    directory=Path(directory or Path(__file__).resolve().parent)
    parameters=json.loads((directory/'parameters.json').read_text())
    p,a,b,n=571,508,137,557
    assert all(p%d for d in range(2,int(p**.5)+1))
    assert all(n%d for d in range(2,int(n**.5)+1))
    E=Curve(p,a,b)
    assert a==int(parameters['a'],16)%p and b==int(parameters['b'],16)%p
    assert (4*a**3+27*b*b)%p!=0
    P=(517,168)
    assert E.contains(P) and E.scale(n,P) is None
    points=[]
    for x in range(p):
        y=sqrt(E.rhs(x),p)
        if y is not None:
            points.append((x,y))
            if y:points.append((x,-y%p))
    assert len(points)+1==n
    k=37;Q=E.scale(k,P)
    logs=[11,23]
    factor_base=[E.scale(z,P) for z in logs]
    matrix=[[1,0],[0,1],[1,1]]
    w=[-1,-1,1]
    scenarios={}
    for name,beta in [('usable',[2,3,7]),('dependent',[2,3,5])]:
        alpha=[(sum(e*l for e,l in zip(row,logs))-bb*k)%n for row,bb in zip(matrix,beta)]
        for row,aa,bb in zip(matrix,alpha,beta):
            left=E.add(E.scale(aa,P),E.scale(bb,Q))
            right=None
            for e,F in zip(row,factor_base):right=E.add(right,E.scale(e,F))
            assert left==right
        wa=sum(x*y for x,y in zip(w,alpha))%n
        wb=sum(x*y for x,y in zip(w,beta))%n
        augmented=[row+[-bb%n] for row,bb in zip(matrix,beta)]
        recovered=(-wa*pow(wb,-1,n))%n if wb else None
        assert recovered==k if name=='usable' else recovered is None and wa==0
        scenarios[name]={'a':alpha,'b':beta,'wTa':wa,'wTb':wb,'rank_M':rank_mod(matrix,n),'rank_augmented':rank_mod(augmented,n),'recovered_k':recovered}
    assert scenarios['usable']['rank_augmented']==3 and scenarios['dependent']['rank_augmented']==2

    # An oriented large-prime triangle; verify both coefficient cancellation
    # and its numerical realization in the cyclic order-557 component.
    fb_logs=[11,23,31]
    large_logs=[5,9,14]
    large_matrix=[[1,-1,0],[0,1,-1],[-1,0,1]]
    beta=[1,4,2]
    rlogs=[(f+sum(e*l for e,l in zip(row,large_logs)))%n for f,row in zip(fb_logs,large_matrix)]
    alpha=[(r-bb*k)%n for r,bb in zip(rlogs,beta)]
    for i in range(3):
        assert E.add(E.scale(alpha[i],P),E.scale(beta[i],Q))==E.scale(rlogs[i],P)
    assert all(sum(row[j] for row in large_matrix)==0 for j in range(3))
    assert sum(rlogs)%n==sum(fb_logs)%n

    # C: Y^2=X^6+aX^2+b; companion elliptic quotient has this cubic.
    auxiliary_order=p+1+sum(chi(b*u**3+a*u*u+1,p) for u in range(p))
    jacobian_order=n*auxiliary_order
    assert auxiliary_order==609 and jacobian_order==339213
    assert gcd(n,auxiliary_order)==1
    census=[]
    for m in [4,8,16,32,64,128]:
        fb=set()
        for x in range(1,m+1):
            y=sqrt((x**6+a*x*x+b)%p,p)
            if y is not None and y:
                for xx in [x,-x%p]:
                    for yy in [y,-y%p]:fb.add((xx,yy))
        B=len(fb)
        assert B%4==0
        covered=0
        for x,y in points:
            root=sqrt(x,p)
            if root and (root,y) in fb and (-root%p,y) in fb:covered+=1
        assert covered==B//2
        divisor_encodings=set()
        for A,D in itertools.combinations(sorted(fb),2):
            if A[0]==D[0]:continue
            x1,y1=A;x2,y2=D
            u1=-(x1+x2)%p;u0=x1*x2%p
            v1=(y2-y1)*pow((x2-x1)%p,-1,p)%p
            v0=(y1-v1*x1)%p
            assert (y1*y1-(x1**6+a*x1*x1+b))%p==0
            assert (y2*y2-(x2**6+a*x2*x2+b))%p==0
            divisor_encodings.add((u1,u0,v1,v0))
        count=len(divisor_encodings)
        assert count==B*B//2-B
        census.append({'positive_X_cutoff':m,'B':B,'covered_E_targets':covered,'nonidentity_E_targets':n-1,'distinct_x_degree_two_divisors':count,'Jacobian_order':jacobian_order,'E_probability':covered/(n-1),'J_probability':count/jacobian_order})

    # Classify the actual published P,Q fibers without computing their log.
    challenge_p=int(parameters['p'],16)
    fibers={}
    for name in ['P','Q']:
        xp,yp=[int(value,16) for value in parameters[name]]
        sign=chi(xp,challenge_p)
        if sign==1:
            root=sqrt(xp,challenge_p)
            fibers[name]={'chi_x':sign,'split':True,'root_X':hex(root),'y':hex(yp)}
        else:
            assert sign==-1
            imaginary_scale=sqrt(-xp,challenge_p)
            assert imaginary_scale*imaginary_scale%challenge_p==-xp%challenge_p
            fibers[name]={'chi_x':sign,'split':False,'X_in_quadratic_extension':'i times imaginary_scale, with i^2=-1','imaginary_scale':hex(imaginary_scale),'y':hex(yp)}
    assert fibers['P']['split'] and not fibers['Q']['split']
    data={
        'scope':__doc__,
        'toy_curve':{'p':p,'a':a,'b':b,'n':n,'P':P,'Q':Q,'k':k,'factor_base_logs_for_construction_only':logs,'factor_base_points':factor_base},
        'kernel':{'M':matrix,'w':w,'scenarios':scenarios},
        'partial_triangle':{'A_large':large_matrix,'factor_base_logs':fb_logs,'large_class_logs_for_construction_only':large_logs,'a':alpha,'b':beta,'combined_a':sum(alpha)%n,'combined_b':sum(beta)%n,'combined_log':sum(rlogs)%n},
        'fiber_census':{'auxiliary_elliptic_order':auxiliary_order,'Jacobian_order':jacobian_order,'rows':census,'counting_scope':'S_B consists of degree-two divisors supported on F_B with distinct X, modulo D_infinity. Abel-map injectivity away from the hyperelliptic pencil gives distinct classes; full J is not enumerated.'},
        'published_ECCp131_fibers':fibers,
        'collection_model':{'baseline':{'trial_cost':1,'p_valid':.02,'eta':.5,'cost_per_effective_row':100},'alternative':{'trial_cost':4,'p_valid':.04,'eta':.5,'cost_per_effective_row':200},'scope':'Illustrative phase-average model, not measured challenge attack costs.'},
    }
    (directory/'worked-example-results.json').write_text(json.dumps(data,indent=2)+'\n')
    with (directory/'fiber-census.csv').open('w',newline='') as output:
        writer=csv.DictWriter(output,fieldnames=list(census[0]),lineterminator='\n');writer.writeheader();writer.writerows(census)
    return data


if __name__=='__main__':
    data=build_data()
    print(json.dumps({'kernel':data['kernel'],'partial_triangle':data['partial_triangle'],'fiber_census':data['fiber_census'],'published_fiber_characters':{k:v['chi_x'] for k,v in data['published_ECCp131_fibers'].items()}},indent=2))
