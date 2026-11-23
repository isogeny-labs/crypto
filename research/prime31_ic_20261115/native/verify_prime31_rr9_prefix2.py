#!/usr/bin/env python3
"""Independent original-curve BigInt, dual-number and witness replay for Rust gate.
No imports from candidate crate; all signed leaves are GRADER/TRUTH ONLY.
Usage: python3 verify_prime31_rr9_prefix2.py rust-out.json
"""
import json,sys,math
P=2147483647;A=847514846;B=2087444724;N=2147446831
G=(625037715,2100627251)
OLD=[1,3,5,7,11,13,17,19]
TERN=[1,3,9,27,81,243,729,2187]
def ec_add(Q,T):
    if Q is None:return T
    if T is None:return Q
    x,y=Q;z,t=T
    if x==z and (y+t)%P==0:return None
    slope=((3*x*x+A)*pow(2*y,-1,P) if x==z else (t-y)*pow(z-x,-1,P))%P
    u=(slope*slope-x-z)%P
    return (u,(slope*(x-u)-y)%P)
def ec_mul(k,Q):
    ret=None
    while k:
        if k&1:ret=ec_add(ret,Q)
        Q=ec_add(Q,Q);k>>=1
    return ret
def basis():
    out=[]
    for x in range(100):
        rhs=(x**3+A*x+B)%P
        y=pow(rhs,(P+1)//4,P)
        if y and y*y%P==rhs:out.append((x,min(y,P-y)))
        if len(out)==8:break
    assert [x for x,y in out]==[0,1,2,3,5,6,8,9]
    return out
class D:
    def __init__(self,v,d=0):self.v=v%P;self.d=d%P
    def __add__(self,o):
        if not isinstance(o,D):o=D(o)
        return D(self.v+o.v,self.d+o.d)
    __radd__=__add__
    def __neg__(self):return D(-self.v,-self.d)
    def __sub__(self,o):return self+-o if isinstance(o,D) else self+D(-o)
    def __rsub__(self,o):return D(o)+-self
    def __mul__(self,o):
        if not isinstance(o,D):o=D(o)
        return D(self.v*o.v,self.v*o.d+self.d*o.v)
    __rmul__=__mul__
def rank5(a):
    m=[list(row) for row in a]
    r=0
    for k in range(5):
        j=next((i for i in range(r,6) if m[i][k]),None)
        if j is None:continue
        m[r],m[j]=m[j],m[r]
        z=pow(m[r][k],-1,P)
        m[r]=[c*z%P for c in m[r]]
        for i in range(r+1,6):
            f=m[i][k];m[i]=[(v-f*u)%P for v,u in zip(m[i],m[r])]
        r+=1
    return r
def algebraic(R,q,full):
    x,y=q;r,s=R
    a=sum(full[i]*pow(x,i,P) for i in range(4))%P
    v=(x*x*x+sum(full[i+4]*pow(x,i,P) for i in range(3)))%P
    return ((x-r)*a+(y+s)*v)%P
def six_eval(R,q0,q1,free,support,axis):
    """Fresh dual-number program for H_j and dH_j/d free[axis]; no analytic Jacobian used."""
    r,s=R
    x0,y0=q0;x1,y1=q1
    assert x0!=x1 and x0!=r and x1!=r
    al0=(y0+s)*pow(x0-r,-1,P)%P
    al1=(y1+s)*pow(x1-r,-1,P)%P
    v=[D(k,int(i==axis)) for i,k in enumerate(free)]
    a2,a3,v0,v1,v2=v
    def C(x,al):return -al*(x**3+v0+v1*x+v2*x*x)-a2*x*x-a3*x**3
    c0=C(x0,al0);c1=C(x1,al1)
    a1=(c1-c0)*pow(x1-x0,-1,P)
    a0=c0-x0*a1
    full=[a0,a1,a2,a3,v0,v1,v2]
    output=[]
    for slot in support[2:]:
        prod=D(1)
        for x,y in slot:
            xx=x%P
            aa=sum((full[i]*pow(xx,i,P) for i in range(4)),D(0))
            vv=xx**3+sum((full[i+4]*pow(xx,i,P) for i in range(3)),D(0))
            dd=(xx*xx+r*xx+r*r+A)%P
            mm=dd*vv*vv-(xx-r)*aa*aa-2*s*aa*vv
            prod=prod*mm
        output.append((prod.v,prod.d))
    return output
def check(data):
    assert data['schema']=='prime31.rr9.two-prefix.v1'
    assert data['candidate_has_two_supplied_signed_leaves']
    assert not data['public_unknown_prefix_extractor']
    assert not data['generic_group_sum_tables_used']
    assert not data['complete_nongeneric_ecdlp_solver']
    assert data['curve']=={'p':P,'a':A,'b':B,'n':N,'cofactor':1}
    assert ec_mul(N,G) is None
    bases=basis()
    table=data['algebraic_branch_cost_ceiling_table']
    for t,c in enumerate(table):
        nvar=7-t;deg=16
        assert c['known_prefixes']==t
        assert c['remaining_variables']==nvar
        assert c['remaining_resultants']==8-t
        assert c['prefix_branches']==16**t
        assert c['dense_degree16_monomial_ceiling']==math.comb(deg+nvar,nvar)
        assert c['branch_times_monomial_ceiling']==16**t*math.comb(deg+nvar,nvar)
    total=0
    for arm,ww in zip(data['arms'],[OLD,TERN]):
        assert arm['weights']==ww and arm['base8']==[list(q) for q in bases]
        support=[[ec_mul(w,q) for q in bases] for w in ww]
        assert len({q[0] for s in support for q in s})==64
        fixture_count=len(arm['fixtures'])
        assert fixture_count==arm['full_rank_fixtures'] and fixture_count>0
        for f in arm['fixtures']:
            t=f['case'];qs=[tuple(q) for q in f['signed_points']]
            R=tuple(f['target'])
            assert len(qs)==8
            assert ec_add(None,G)==G
            truth=None
            for j,q in enumerate(qs):
                i=(3*t+7*j+j*j)%8
                sgn=1 if (t+3*j)%4<2 else -1
                assert f['signed_indices'][j]==i and f['signs'][j]==sgn
                e=support[j][i]
                if sgn<0:e=(e[0],(-e[1])%P)
                assert q==e
                truth=ec_add(truth,q)
            assert truth==R
            full=f['grader_full']
            free=f['free']
            assert list(free)==full[2:]
            assert f['candidate_full']==full
            assert all(algebraic(R,q,full)==0 for q in qs)
            H=f['six_resultants'];jac=f['six_by_five_jacobian']
            assert H==[0]*6 and len(jac)==6
            assert rank5(jac)==5==f['jacobian_rank']
            for axis in range(5):
                dual=six_eval(R,qs[0],qs[1],free,support,axis)
                for j in range(6):
                    assert dual[j]==(H[j],jac[j][axis]),(arm['arm'],t,j,axis,dual[j],H[j],jac[j][axis])
            mutated=free.copy();mutated[0]=(mutated[0]+1)%P
            expected=six_eval(R,qs[0],qs[1],mutated,support,0)
            assert [v for v,d in expected]==f['mutation_a2_plus_one_resultants']
            assert any(v for v,d in expected)
            total+=1
    print('Independent BigInt replay PASS:',total,'original-curve fixtures;',
          6*5*total,'dual Jacobian cells; both weight schedules; no public index extraction')
if __name__=='__main__':
    with open(sys.argv[1],encoding='utf-8') as f:check(json.load(f))
