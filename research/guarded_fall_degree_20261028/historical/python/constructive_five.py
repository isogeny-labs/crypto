"""Construct a degree-five refutation of an unsatisfiable guarded B=3 model.

This producer implements the quadratic-fibre proof directly. It performs no
Macaulay closure, Gaussian elimination, or search for a refutation. Its output
uses proof_trace.py's elementary DAG schema and is checked by that independent
sparse-polynomial replay. All degree limits are ordinary polynomial degrees.
"""

import argparse
import itertools
import json
from pathlib import Path

from instance import normalized_problem, problem_digest
from prime_chain import build_problem
from proof_trace import verify_refutation


def add(p,*polys):
    out={}
    for poly in polys:
        for e,c in poly.items():out[e]=(out.get(e,0)+c)%p
    return {e:c for e,c in out.items() if c}


def scale(p,c,poly):
    return {e:c*a%p for e,a in poly.items() if c*a%p}


def mul(p,a,b):
    out={}
    for e,c in a.items():
        for f,d in b.items():
            g=tuple(x+y for x,y in zip(e,f))
            out[g]=(out.get(g,0)+c*d)%p
    return {e:c for e,c in out.items() if c}


def const(p,c,n):return {(0,)*n:c%p} if c%p else {}
def var(i,n):
    e=[0]*n;e[i]=1;return {tuple(e):1}
def deg(poly):return max((sum(e) for e in poly),default=-1)


def divide(p,poly,divisors):
    """Lex division by monic, total-degree-preserving divisors."""
    work=dict(poly);quot=[{} for _ in divisors];rem={}
    leads=[max(g) for g in divisors]
    assert all(g[e]==1 and sum(e)==deg(g) for g,e in zip(divisors,leads))
    while work:
        e=max(work);c=work[e]
        for i,(g,lm) in enumerate(zip(divisors,leads)):
            if all(x>=y for x,y in zip(e,lm)):
                shift=tuple(x-y for x,y in zip(e,lm))
                term={shift:c};quot[i]=add(p,quot[i],term)
                work=add(p,work,scale(p,-1,mul(p,term,g)))
                break
        else:
            rem[e]=c;del work[e]
    return quot,rem


def evaluate(poly,values,p):
    total=0
    for e,c in poly.items():
        for x,k in zip(values,e):c=c*pow(x,k,p)%p
        total=(total+c)%p
    return total


def lagrange(p,base,s,axis,n=3):
    out=const(p,1,n);den=1;x=var(axis,n)
    for t in base:
        if t==s:continue
        out=mul(p,out,add(p,x,const(p,-t,n)));den=den*(s-t)%p
    return scale(p,pow(den,-1,p),out)


class DAG:
    def __init__(self,problem):
        self.problem=normalized_problem(problem);self.p=problem['p']
        self.nodes=[];self.values=[];self.maximum_degree=-1
        self.inputs=[self.emit({'op':'input','index':i},{tuple(e):c for c,e in f})
                     for i,f in enumerate(self.problem['polynomials'])]
        self.zero=self.scaled(self.inputs[0],0)

    def emit(self,node,poly):
        assert deg(poly)<=5,('degree budget exceeded',node,deg(poly))
        self.maximum_degree=max(self.maximum_degree,deg(poly))
        self.nodes.append(node);self.values.append(poly)
        return len(self.nodes)-1

    def scaled(self,j,c):
        c%=self.p
        if c==1:return j
        return self.emit({'op':'scale','parent':j,'coefficient':c},scale(self.p,c,self.values[j]))

    def plus(self,j,k,c=1):
        c%=self.p
        if not c:return j
        return self.emit({'op':'add','left':j,'right':k,'coefficient':c},
                         add(self.p,self.values[j],scale(self.p,c,self.values[k])))

    def times_variable(self,j,i):
        return self.emit({'op':'multiply','parent':j,'variable':i},
                         mul(self.p,self.values[j],var(i,5)))

    def times_polynomial(self,j,factor):
        result=self.zero
        for exp,c in factor.items():
            term=j
            for i,k in enumerate(exp):
                for _ in range(k):term=self.times_variable(term,i)
            result=self.plus(result,term,c)
        return result

    def certificate(self,goal):
        assert self.values[goal]==const(self.p,1,5)
        keep=set();todo=[goal]
        while todo:
            j=todo.pop()
            if j in keep:continue
            keep.add(j)
            todo.extend(self.nodes[j][k] for k in ('parent','left','right') if k in self.nodes[j])
        ordered=sorted(keep);mapping={j:i for i,j in enumerate(ordered)};nodes=[]
        for j in ordered:
            item=dict(self.nodes[j])
            for key in ('parent','left','right'):
                if key in item:item[key]=mapping[item[key]]
            nodes.append(item)
        return {'schema':'bounded-degree-refutation-v1','degree':5,
                'problem_sha256':problem_digest(self.problem),'nodes':nodes,'goal':mapping[goal]}


class Construction:
    def __init__(self,problem):
        self.original=problem;self.p=p=problem['p'];self.base=sorted(problem['factor_base_abscissas'])
        assert len(self.base)==3 and len(set(self.base))==3
        self.a=problem['curve']['a'];self.b=problem['curve']['b'];self.r=problem['target'][0]
        assert self.r not in self.base,'an unsatisfiable guarded model has target abscissa outside S'
        expected,_=build_problem(p,self.a,self.b,self.base,problem['target'])
        assert normalized_problem(problem)==normalized_problem(expected),'unexpected input presentation'
        self.trace=DAG(problem);self.stages={}
        self.W=add(p,const(p,sum(self.base),5),scale(p,-1,var(0,5)),scale(p,-1,var(1,5)))
        self.w=var(0,3);self.z=var(1,3);self.u=var(2,3)
        self.w_powers=[const(p,1,5)]
        for _ in range(6):self.w_powers.append(mul(p,self.w_powers[-1],self.W))

    def mapped(self,formal):
        p=self.p;out={}
        for (i,j,k),c in formal.items():
            for exp,d in self.w_powers[i].items():
                e=list(exp);e[2]+=j;e[3]+=k;e=tuple(e)
                out[e]=(out.get(e,0)+c*d)%p
        return {e:c for e,c in out.items() if c}

    def pair(self,formal,j):
        assert self.trace.values[j]==self.mapped(formal)
        return formal,j

    def linear(self,*terms):
        formal={};j=self.trace.zero
        for c,(poly,k) in terms:
            formal=add(self.p,formal,scale(self.p,c,poly));j=self.trace.plus(j,k,c)
        return self.pair(formal,j)

    def times(self,item,factor):
        poly,j=item
        return self.pair(mul(self.p,poly,factor),self.trace.times_polynomial(j,self.mapped(factor)))

    def reduce(self,item,axes=(0,1)):
        poly,j=item;divisors=[self.gs[i][0] for i in axes]
        quot,rem=divide(self.p,poly,divisors)
        for q,i in zip(quot,axes):
            j=self.trace.plus(j,self.trace.times_polynomial(self.gs[i][1],self.mapped(q)),-1)
        return self.pair(rem,j)

    def derive_from_pair_ideal(self,target,start=None):
        """Derive a mapped target using the already available Q and g(x2)."""
        p=self.p;t=self.trace;target=self.mapped(target)
        if start is None:
            quotient,remainder=divide(p,target,[t.values[self.Q],t.values[t.inputs[1]]])
            assert not remainder
            j=t.zero
            for q,k in zip(quotient,[self.Q,t.inputs[1]]):j=t.plus(j,t.times_polynomial(k,q))
        else:
            diff=add(p,t.values[start],scale(p,-1,target))
            quotient,remainder=divide(p,diff,[t.values[self.Q],t.values[t.inputs[1]]])
            assert not remainder
            j=start
            for q,k in zip(quotient,[self.Q,t.inputs[1]]):j=t.plus(j,t.times_polynomial(k,q),-1)
        assert t.values[j]==target
        return j

    def initialize(self):
        p=self.p;t=self.trace;x1,x2=var(0,5),var(1,5)
        s1=sum(self.base)%p;s2=sum(a*b for a,b in itertools.combinations(self.base,2))%p
        q=add(p,mul(p,x1,x1),mul(p,x1,x2),mul(p,x2,x2),
              scale(p,-s1,add(p,x1,x2)),const(p,s2,5))
        diff=t.plus(t.inputs[0],t.inputs[1],-1)
        self.Q=t.plus(t.times_variable(diff,4),t.times_polynomial(t.inputs[5],q),-1)
        assert t.values[self.Q]==q
        gformal=[]
        for axis in (0,1):
            g=const(p,1,3)
            for s in self.base:g=mul(p,g,add(p,var(axis,3),const(p,-s,3)))
            gformal.append(g)
        self.gs=[self.pair(gformal[0],self.derive_from_pair_ideal(gformal[0])),
                 self.pair(gformal[1],t.inputs[2])]
        q1={}
        for missing in self.base:
            a,b=[s for s in self.base if s!=missing]
            coefficients=[((a*b-self.a)**2-4*self.b*(a+b))%p,
                          -2*((a+b)*(a*b+self.a)+2*self.b)%p,(a-b)**2%p]
            quad={(0,0,k):c for k,c in enumerate(coefficients) if c}
            q1=add(p,q1,mul(p,lagrange(p,self.base,missing,0),quad))
        q1=self.pair(q1,self.derive_from_pair_ideal(q1,t.inputs[3]))
        q2={}
        for e,c in t.values[t.inputs[4]].items():
            assert not e[0] and not e[1] and not e[4]
            q2[(0,e[2],e[3])]=c
        q2=self.pair(q2,t.inputs[4])
        self.stages['pair_quotient_derived']=True
        return q1,q2

    def normalize_quadratic(self,item,axis):
        """Combine q, NF(t*q), NF(t*NF(t*q)); never form t^2*q at degree six."""
        p=self.p;poly,j=item
        leading={tuple(list(e[:2])+[0]):c for e,c in poly.items() if e[2]==2}
        inverse={}
        for s in self.base:
            point=[0,0,0];point[axis]=s
            value=evaluate(leading,point,p)
            assert value,'quadratic leading coefficient is not a unit on the grid'
            inverse=add(p,inverse,scale(p,pow(value,-1,p),lagrange(p,self.base,s,axis)))
        rows=[item]
        for _ in range(2):rows.append(self.reduce(self.times(rows[-1],var(axis,3)),(axis,)))
        result=self.linear(*[(c,rows[e[axis]]) for e,c in inverse.items()])
        result=self.reduce(result,(axis,))
        assert result[0].get((0,0,2))==1
        assert all(e[2]<2 or e==(0,0,2) for e in result[0])
        assert deg(result[0])<=3
        return result

    def coefficient(self,item,power):
        return {(i,j,0):c for (i,j,k),c in item[0].items() if k==power}

    def run(self):
        p=self.p;q1,q2=self.initialize()
        h1=self.normalize_quadratic(q1,0);h2=self.normalize_quadratic(q2,1)
        A=self.coefficient(h1,1);B=self.coefficient(h1,0)
        C=self.coefficient(h2,1);D=self.coefficient(h2,0)
        E=add(p,A,scale(p,-1,C));J=add(p,B,scale(p,-1,D))
        L=self.linear((1,h1),(-1,h2))
        assert L[0]==add(p,mul(p,E,self.u),J)
        hs=self.linear((1,h1),(1,h2))
        # Quadratic cancellation leaves N=F*u+G of degree at most four;
        # crucially F is separable: a polynomial of w plus a polynomial of z.
        N=self.reduce(self.linear((2,self.times(L,self.u)),(-1,self.times(hs,E))))
        F=self.coefficient(N,1);G=self.coefficient(N,0)
        assert N[0]==add(p,mul(p,F,self.u),G)
        assert all(not (i and j) for i,j,k in F),'the linear coefficient must be separable'
        assert deg(F)<=2 and deg(N[0])<=4
        rows={(0,0):N}
        # E*N would formally cost six. Build its pure w^i and z^j pieces
        # separately, reducing after each multiplication. First powers reset
        # to degree <=4; second powers cost at most five.
        for axis in (0,1):
            current=N
            for i in (1,2):
                current=self.reduce(self.times(current,var(axis,3)))
                rows[(i,0) if axis==0 else (0,i)]=current
                if i==1:assert deg(current[0])<=4
        EN=self.linear(*[(c,rows[(i,j)]) for (i,j,k),c in E.items()])
        FL=self.reduce(self.times(L,F))
        # EN-FL is scalar, and equals -2 times the monic-quadratic resultant.
        scalar_identity=self.linear((1,EN),(-1,FL))
        resultant=self.linear((-pow(2,-1,p),scalar_identity))
        assert deg(resultant[0])<=4 and all(e[2]==0 for e in resultant[0])
        inv={}
        for s in self.base:
            for z in self.base:
                value=evaluate(resultant[0],(s,z,0),p)
                if not value:raise ValueError('the fibre resultant vanishes: this is not an unsatisfiable guarded instance')
                projector=mul(p,lagrange(p,self.base,s,0),lagrange(p,self.base,z,1))
                inv=add(p,inv,scale(p,pow(value,-1,p),projector))
        rrows={(0,0):resultant}
        for i in range(3):
            if i:rrows[(i,0)]=self.reduce(self.times(rrows[(i-1,0)],self.w))
            for j in (1,2):rrows[(i,j)]=self.reduce(self.times(rrows[(i,j-1)],self.z))
        one=self.linear(*[(c,rrows[(i,j)]) for (i,j,k),c in inv.items()])
        assert one[0]==const(p,1,3)
        cert=self.trace.certificate(one[1])
        assert verify_refutation(self.original,cert)
        return cert,{'algorithm':'constructive quadratic-fibre degree-five proof',
                     'macaulay_or_rref_search':False,'maximum_node_degree':self.trace.maximum_degree,
                     'certificate_nodes':len(cert['nodes']),'replay_verified':True,
                     'formal_stage_degrees':{'H1':deg(h1[0]),'H2':deg(h2[0]),
                         'L':deg(L[0]),'N':deg(N[0]),'EN':deg(EN[0]),
                         'FL':deg(FL[0]),'resultant':deg(resultant[0]),'goal':deg(one[0])},
                     'resultant_grid_values_nonzero':True}


def constructive_refutation(problem):return Construction(problem).run()


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('problem',type=Path)
    parser.add_argument('--out',type=Path)
    args=parser.parse_args()
    cert,report=constructive_refutation(json.loads(args.problem.read_text()))
    if args.out:args.out.write_text(json.dumps(cert,indent=2)+'\n')
    print(json.dumps(report,indent=2))
