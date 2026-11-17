"""Trace-zero subgroups and an executable, exhaustive relation-collection model.

This is an index-calculus *mechanism* experiment. Its enumerative decomposition
table is intentionally not advertised as the Groebner-basis algorithm from the
paper, or as evidence of the paper's asymptotic speedup.
"""
import itertools
import random
from time import perf_counter
from sage.all import GF, EllipticCurve, Integer, matrix, vector, factor
from common import (ec_key, field_data, point_data, frobenius, trace_point,
                    min_field_degree, bsgs_ec, project_prime_order)


def choose_trace_instance(p, n):
    k = GF(p)
    K = GF(p**n, name='z')
    for a in range(p):
        for b in range(1, p):
            if (4*a**3+27*b*b) % p == 0:
                continue
            E0 = EllipticCurve(k, [a, b])
            # Avoid anomalous/supersingular base curves in this experiment.
            if E0.is_supersingular() or E0.cardinality() == p:
                continue
            E = E0.change_ring(K)
            N = Integer(E.cardinality())
            torder = N // E0.cardinality()
            choices = [int(r) for r,e in factor(torder)
                       if r > 7 and n % r and E0.cardinality() % r]
            if not choices:
                continue
            r = max(choices)
            for _ in range(30):
                # Im(pi - 1) = ker(trace) on these finite rational point sets.
                # Use the SAME source point in the subtraction.
                R = E.random_point()
                T = frobenius(R, p) - R
                P = project_prime_order(T,torder,r)
                if not P.is_zero() and (r*P).is_zero():
                    return E0, E, P, r, int(torder)
    raise ValueError(f'No eligible trace-zero instance at p={p}, n={n}')


def trace_structure(p, n, enumerate_kernel=False):
    start = perf_counter()
    E0,E,P,r,torder = choose_trace_instance(p,n)
    K = E.base_field()
    secret = random.randrange(1,r)
    Q = secret * P
    assert trace_point(P,p,n).is_zero()
    assert trace_point(Q,p,n).is_zero()
    assert trace_point(P+Q,p,n) == trace_point(P,p,n)+trace_point(Q,p,n)
    lam = bsgs_ec(P, frobenius(P,p), r)['scalar']
    assert pow(lam,n,r) == 1
    assert sum(pow(lam,i,r) for i in range(n)) % r == 0
    recovered = bsgs_ec(P,Q,r)
    assert recovered['scalar'] == secret
    counted = None
    if enumerate_kernel:
        counted = sum(1 for R in E.points() if trace_point(R,p,n).is_zero())
        assert counted == torder
    return {'family':'trace_zero_structure', 'p':p, 'n':n, 'status':'passed',
            'field':field_data(K), 'curve':str(E), 'base_curve_order':int(E0.cardinality()),
            'ambient_curve_order':int(E.cardinality()), 'trace_zero_order':torder,
            'trace_zero_factorization':str(factor(torder)), 'subgroup_order':r,
            'point':point_data(P), 'target':point_data(Q), 'secret':secret,
            'minimum_point_field_degree':min_field_degree(P,p,n),
            'frobenius_eigenvalue_mod_r':lam, 'enumerated_kernel_size':counted,
            'bsgs':recovered, 'seconds':perf_counter()-start,
            'claim':'subgroup structure and scalar recovery only'}


def factor_base(E, P, torder, r, p, n, limit=10):
    """Project a geometrically restricted set to the target prime subgroup.

    Search x in a two-dimensional F_p-linear subspace of F_(p^n). This is a
    variant of a small-factor-base construction, not the specific symmetric
    coordinate restriction used by Gorla--Massierer.
    """
    K=E.base_field(); z=K.gen()
    seen={}; tested=0
    for shift in range(min(5,p)):
        v1=z**(n-2) + K(shift)
        v2=z**(n-1)
        for a,b in itertools.product(range(p), repeat=2):
            x=K(a)*v1+K(b)*v2
            rhs=x**3+E.a4()*x+E.a6()
            tested+=1
            for y in rhs.sqrt(all=True):
                R=E(x,y)
                if not trace_point(R,p,n).is_zero():
                    continue
                R=project_prime_order(R,torder,r)
                if R.is_zero():
                    continue
                # A factor and its negative use a single logarithm unknown.
                if str(ec_key(-R)) < str(ec_key(R)):
                    R=-R
                seen.setdefault(ec_key(R),R)
        if len(seen) >= min(limit,max(3,(n-1)+1)):
            break
    return list(seen.values())[:limit],tested


def decomposition_table(base, length, max_tuples=150000):
    """Exhaustive signed decomposition lookup, separately measured and bounded."""
    zero=base[0].curve()(0); table={ec_key(zero): [0]*len(base)}
    signed=[(P,i,1) for i,P in enumerate(base)]+[(-P,i,-1) for i,P in enumerate(base)]
    tuples=0
    for ell in range(1,length+1):
        for indices in itertools.combinations_with_replacement(range(len(signed)),ell):
            tuples+=1
            if tuples>max_tuples:
                return table,tuples-1,True
            R=zero; coeff=[0]*len(base)
            for j in indices:
                S,i,sign=signed[j]; R+=S; coeff[i]+=sign
            table.setdefault(ec_key(R),coeff)
    return table,tuples,False


def trace_relations(p,n,limit=8):
    start=perf_counter()
    E0,E,P,r,torder=choose_trace_instance(p,n)
    secret=random.randrange(1,r); Q=secret*P
    before=perf_counter()
    base,xtested=factor_base(E,P,torder,r,p,n,limit)
    if len(base)<2:
        raise ValueError('Factor base too small; no attack claimed')
    factor_seconds=perf_counter()-before
    before=perf_counter()
    table,tuples,capped=decomposition_table(base,n-1)
    decomposition_seconds=perf_counter()-before
    rows=[]; rhs=[]; attempts=0; unknowns=len(base)+1
    Fr=GF(r)
    before=perf_counter()
    while attempts<max(1000,40*unknowns):
        attempts+=1
        a=random.randrange(r); b=random.randrange(r)
        R=a*P+b*Q
        v=table.get(ec_key(R))
        if v is None:
            continue
        rows.append(v+[-b]); rhs.append(a)
        if len(rows)>=unknowns and matrix(Fr,rows).rank()==unknowns:
            break
    relation_seconds=perf_counter()-before
    A=matrix(Fr,rows); c=vector(Fr,rhs)
    if A.rank()<unknowns:
        raise ValueError('Insufficient independent relations; no scalar recovered')
    before=perf_counter()
    solution=A.solve_right(c)
    recovered=int(solution[-1]); linear_seconds=perf_counter()-before
    assert A*solution==c
    assert recovered*P==Q and recovered==secret
    for F,logarithm in zip(base,solution[:-1]):
        assert int(logarithm)*P==F
    baseline=bsgs_ec(P,Q,r)
    return {'family':'trace_zero_relations','p':p,'n':n,'status':'passed',
            'field':field_data(E.base_field()),'curve':str(E), 'subgroup_order':r,
            'trace_zero_order':torder,'point':point_data(P),'target':point_data(Q),
            'secret':secret,'recovered_scalar':recovered,'factor_base_size':len(base),
            'factor_base':[point_data(F) for F in base],
            'factor_base_x_candidates':xtested,'decomposition_length':n-1,
            'decomposition_tuples':tuples,'decomposition_table_entries':len(table),
            'decomposition_capped':capped,'relation_attempts':attempts,
            'relations_collected':len(rows),'matrix_rank':int(A.rank()),
            'factor_base_seconds':factor_seconds,'decomposition_seconds':decomposition_seconds,
            'relation_seconds':relation_seconds,'linear_algebra_seconds':linear_seconds,
            'bsgs':baseline,'seconds':perf_counter()-start,
            'claim':'enumerative relation mechanism; not a Groebner-basis speed benchmark'}
