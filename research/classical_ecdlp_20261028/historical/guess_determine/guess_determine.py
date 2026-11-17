"""Independent classical implementation of arXiv:2607.09814v2, Algorithms 1-2.

The attack takes only curve parameters, P, Q, and the prime group order.
No secret scalar or discrete-log oracle is passed to it. Benchmark fixture
generation knows the secret solely to check the returned result.

Uses exact arithmetic and only Python's standard library. Column maps are
explicit; small projective kernels use cross products. General-position
failures are counted and skipped. Every returned answer has a checked
zero minor, a checked elliptic-curve point-sum relation, and w*P == Q.
"""
import argparse
from collections import Counter
from itertools import combinations
import json
from math import ceil
from pathlib import Path
from random import Random
from statistics import median
from time import perf_counter

SOURCE = 'https://arxiv.org/html/2607.09814v2'


class Curve:
    def __init__(self, p, a, b):
        self.p,self.a,self.b = p,a,b
        self.additions = 0

    def contains(self, point):
        return point is None or (point[1]**2-point[0]**3-
                                  self.a*point[0]-self.b) % self.p == 0

    def neg(self, point):
        return None if point is None else (point[0],-point[1] % self.p)

    def add(self, U, V):
        self.additions += 1
        if U is None: return V
        if V is None: return U
        x,y = U
        z,v = V
        p = self.p
        if x == z and (y+v) % p == 0: return None
        slope = ((v-y)*pow(z-x,-1,p) if x != z else
                 (3*x*x+self.a)*pow(2*y,-1,p)) % p
        X = (slope*slope-x-z) % p
        return X,(slope*(x-X)-y) % p

    def mul(self, n, U):
        if n < 0: return self.mul(-n,self.neg(U))
        result = None
        while n:
            if n & 1: result = self.add(result,U)
            U = self.add(U,U)
            n >>= 1
        return result


def transpose(A, ncols=None):
    if not A:
        return [[] for _ in range(ncols or 0)]
    return [list(c) for c in zip(*A)]


def rref(A,p,ncols=None):
    width = len(A[0]) if A else ncols
    assert width is not None
    R = [[x % p for x in row] for row in A]
    pivots = []
    for col in range(width):
        row = len(pivots)
        pivot = next((j for j in range(row,len(R)) if R[j][col]),None)
        if pivot is None: continue
        R[row],R[pivot] = R[pivot],R[row]
        inv = pow(R[row][col],-1,p)
        R[row] = [v*inv % p for v in R[row]]
        for j in range(len(R)):
            if j != row and R[j][col]:
                c = R[j][col]
                R[j] = [(v-c*w) % p for v,w in zip(R[j],R[row])]
        pivots.append(col)
        if len(pivots) == len(R): break
    return R,pivots


def nullspace(A,p,ncols=None):
    width = len(A[0]) if A else ncols
    R,pivots = rref(A,p,width)
    free = [j for j in range(width) if j not in pivots]
    result = []
    for j in free:
        v = [0]*width
        v[j] = 1
        for row,c in enumerate(pivots): v[c] = -R[row][j] % p
        result.append(v)
    return result


def normalize(v,p):
    first = next((x % p for x in v if x % p),None)
    if first is None: return None
    inv = pow(first,-1,p)
    return tuple(x*inv % p for x in v)


def projective_kernel(rows,p,d):
    if d == 2:
        a,b = rows[0]
        return normalize((b,-a),p)
    if d == 3:
        a,b,c = rows[0]
        x,y,z = rows[1]
        return normalize((b*z-c*y,c*x-a*z,a*y-b*x),p)
    N = nullspace(rows,p,d)
    return normalize(N[0],p) if len(N) == 1 else None


def matmul(A,B,p):
    BT = transpose(B)
    return [[sum(x*y for x,y in zip(row,col)) % p for col in BT]
            for row in A]


def columns(A,indices):
    return [[row[i] for i in indices] for row in A]


def anti_diagonal(A,p):
    """Row operations only; preserves every column's original index."""
    h = len(A)
    right = [row[-h:] for row in A]
    augmented = [row+[int(i==j) for j in range(h)]
                 for i,row in enumerate(right)]
    R,pivots = rref(augmented,p)
    if pivots[:h] != list(range(h)): return None
    inverse = [row[h:] for row in R]
    result = matmul(inverse[::-1],A,p)
    assert all(row[-h:] == [int(i+j==h-1) for j in range(h)]
               for i,row in enumerate(result))
    return result


def evaluation_matrix(points,n,p):
    """All homogeneous degree-n monomials, evaluated at affine (x,y,1)."""
    powers = [(a,b) for a in range(n+1) for b in range(n+1-a)]
    return [[pow(x,a,p)*pow(y,b,p) % p for a,b in powers]
            for x,y in points]


def sample_kernel(E,P,Q,r,n,rng,stats):
    ell = 3*n
    records,seen = [],set()
    for side,base in [('P',P),('Q',E.neg(Q))]:
        used_scalars = set()
        while len(used_scalars) < ell:
            scalar = rng.randrange(1,r)
            if scalar in used_scalars: continue
            point = E.mul(scalar,base)
            if point is None or point in seen:
                stats['sampling_rejections'] += 1
                continue
            used_scalars.add(scalar)
            seen.add(point)
            records.append({'side':side,'scalar':scalar,'point':point})
    M = evaluation_matrix([d['point'] for d in records],n,E.p)
    K = nullspace(transpose(M),E.p)
    if len(K) != ell:
        stats['wrong_dimension_matrices'] += 1
        return None
    K = anti_diagonal(K,E.p)
    if K is None:
        stats['singular_systematic_blocks'] += 1
        return None
    assert not any(any(row) for row in matmul(K,M,E.p))
    stats['input_dense_zero_entries'] += sum(
        x == 0 for row in K for x in row[:ell])
    stats['matrices_built'] += 1
    return K,records


def collision_candidates(U,d,p,stats,deadline):
    h,width = len(U),len(U[0])
    for b in combinations(range(h),h-d):
        if perf_counter() > deadline: raise TimeoutError
        stats['b_choices'] += 1
        T = nullspace(transpose(columns(U,b)),p,h)
        if len(T) != d:
            stats['rank_defect_b'] += 1
            continue
        # The first d signatures are remaining dense columns. Sparse-column
        # signatures follow in actual column order, with explicit maps.
        remaining = [j for j in range(width) if j not in b]
        S = [normalize([sum(v[k]*U[k][j] for k in range(h)) % p
                        for v in T],p) for j in remaining]
        if any(s is None for s in S):
            stats['zero_signature_b'] += 1
            continue
        table = {}
        for X in combinations(range(len(S)),d-1):
            stats['small_kernel_computations'] += 1
            v = projective_kernel([S[i] for i in X],p,d)
            if v is None:
                stats['rank_defect_intersections'] += 1
                continue
            if v in table:
                Y = table[v]
                union = sorted(set(X)|set(Y))
                if len(union) != d:
                    stats['collision_union_mismatches'] += 1
                    continue
                stats['signature_collisions'] += 1
                I = sorted(set(b)|{remaining[i] for i in union})
                assert len(I) == h
                assert len(rref(columns(U,I),p)[1]) < h
                yield I,{'b':b,'remaining_columns':remaining,'X':X,'Y':Y,
                         'normalized_kernel':v,'reduced_minor_columns':I}
            else:
                table[v] = X
            stats['peak_hash_entries'] = max(stats['peak_hash_entries'],len(table))


def extract_log(E,P,Q,r,K,records,a,I,stats):
    ell = len(K)
    chosen = sorted(a+[ell+(ell-1-j) for j in range(ell) if j not in I])
    assert len(chosen) == ell and len(set(chosen)) == ell
    assert len(rref(columns(K,chosen),E.p)[1]) < ell
    support = [j for j in range(2*ell) if j not in chosen]
    total = None
    for j in support: total = E.add(total,records[j]['point'])
    assert total is None, 'Zero-minor-to-group-relation mapping failed'
    alpha = sum(records[j]['scalar'] for j in support if j < ell) % r
    beta = sum(records[j]['scalar'] for j in support if j >= ell) % r
    if beta == 0:
        assert alpha == 0
        stats['uninformative_relations'] += 1
        return None
    recovered = alpha*pow(beta,-1,r) % r
    assert E.mul(recovered,P) == Q
    return recovered,{'zero_minor_columns':chosen,'point_sum_indices':support,
                       'alpha':alpha,'beta':beta}


def guess_determine(E,P,Q,r,n,d,rng,seconds=15,max_matrices=4,
                    guesses_per_matrix=32,save_witness=False):
    assert n >= 2 and n % 2 == 0
    ell,h = 3*n,3*n//2
    assert 2 <= d <= h and 2*ell < r
    assert E.contains(P) and E.contains(Q) and E.mul(r,P) is None
    stats = Counter()
    E.additions = 0
    started = perf_counter()
    deadline = started+seconds
    try:
        for unused in range(max_matrices):
            setup = sample_kernel(E,P,Q,r,n,rng,stats)
            if setup is None: continue
            K,records = setup
            visited = set()
            for unused_guess in range(guesses_per_matrix):
                if perf_counter() > deadline: raise TimeoutError
                a = sorted(rng.sample(range(ell),h))
                if tuple(a) in visited: continue
                visited.add(tuple(a))
                stats['a_guesses'] += 1
                # This known P-side sum must be nonzero for an informative
                # relation. No unknown logarithm is needed for this check.
                if sum(records[i]['scalar'] for i in range(ell)
                       if i not in a) % r == 0:
                    stats['zero_P_sum_guesses'] += 1
                    continue
                U = anti_diagonal(transpose(columns(K,a)),E.p)
                if U is None:
                    stats['singular_reduction_blocks'] += 1
                    continue
                for I,collision in collision_candidates(U,d,E.p,stats,deadline):
                    answer = extract_log(E,P,Q,r,K,records,a,I,stats)
                    if answer is None: continue
                    recovered,mapping = answer
                    result = {'status':'solved','log':recovered,
                              'seconds':perf_counter()-started,
                              'group_additions':E.additions,'stats':dict(stats)}
                    if save_witness:
                        result['witness'] = dict(n=n,defect=d,K=K,
                            records=records,a=a,**collision,**mapping)
                    return result
        status = 'search_limit'
    except TimeoutError:
        status = 'timeout'
    return {'status':status,'seconds':perf_counter()-started,
            'group_additions':E.additions,'stats':dict(stats)}


def pollard_rho(E,P,Q,r,rng,seconds=15):
    E.additions = 0
    started = perf_counter()
    deadline = started+seconds
    def step(state):
        R,a,b = state
        bucket = 0 if R is None else R[0] % 3
        if bucket == 0: return E.add(R,P),(a+1)%r,b
        if bucket == 1: return E.add(R,Q),a,(b+1)%r
        return E.add(R,R),2*a%r,2*b%r
    iterations = 0
    for restart in range(32):
        a,b = rng.randrange(r),rng.randrange(r)
        initial = E.add(E.mul(a,P),E.mul(b,Q)),a,b
        slow,fast = step(initial),step(step(initial))
        while slow[0] != fast[0]:
            if perf_counter() > deadline:
                return {'status':'timeout','seconds':perf_counter()-started}
            slow,fast = step(slow),step(step(fast))
            iterations += 1
        denominator = (fast[2]-slow[2]) % r
        if denominator:
            recovered = (slow[1]-fast[1])*pow(denominator,-1,r) % r
            assert E.mul(recovered,P) == Q
            return {'status':'solved','log':recovered,
                    'seconds':perf_counter()-started,
                    'group_additions':E.additions,'iterations':iterations,
                    'restarts':restart}
    return {'status':'restart_limit','seconds':perf_counter()-started}


FIXTURES = [
    (251,1,4,271,(0,2)),
    (1021,1,6,991,(2,4)),
    (4093,1,22,4049,(4,1121)),
    (16381,2,16,16567,(0,4)),
    (65521,1,35,65761,(28357,64561)),
    (1048573,1,14,1048193,(992029,718933)),
]


def run_experiment(args):
    rows = []
    for p,a,b,r,P in FIXTURES:
        if p > args.max_prime: continue
        E = Curve(p,a,b)
        assert (4*a**3+27*b*b) % p != 0 and p != r
        assert E.contains(P) and E.mul(r,P) is None
        n = args.degree or 2*ceil(r.bit_length()/6)
        d = args.defect or (2 if r < 1000 else 3)
        for trial in range(args.trials):
            fixture_rng = Random(args.seed+100000*p+trial)
            secret = fixture_rng.randrange(1,r)
            Q = E.mul(secret,P)
            result = guess_determine(E,P,Q,r,n,d,
                Random(args.seed+200000*p+trial),seconds=args.seconds,
                max_matrices=args.matrices,guesses_per_matrix=args.guesses,
                save_witness=trial == 0)
            rho = pollard_rho(E,P,Q,r,Random(args.seed+300000*p+trial),
                              seconds=args.seconds)
            for method in [result,rho]:
                if method['status'] == 'solved': assert method['log'] == secret
            row = dict(p=p,a=a,b=b,r=r,P=P,Q=Q,trial=trial,
                       expected_log=secret,n=n,ell=3*n,defect=d,
                       guess_determine=result,pollard_rho=rho)
            rows.append(row)
            print(json.dumps({key:row[key] for key in ['p','r','trial','n','defect']}
                | {'guess':{k:v for k,v in result.items() if k != 'witness'},
                   'rho':rho}),flush=True)
    summary = []
    for p in sorted({row['p'] for row in rows}):
        cases = [row for row in rows if row['p'] == p]
        solvers = {}
        for name in ['guess_determine','pollard_rho']:
            solved = [row[name] for row in cases if row[name]['status']=='solved']
            solvers[name] = {'solved':len(solved),'total':len(cases),
                'median_seconds_solved':median(x['seconds'] for x in solved)
                    if solved else None}
        summary.append(dict(p=p,r=cases[0]['r'],n=cases[0]['n'],
                            defect=cases[0]['defect'],methods=solvers))
    report = {'source':SOURCE,'implementation':'Independent Python reference',
              'seed':args.seed,'timing_scope':'End-to-end from public instance, '
              'including point sampling, matrix construction, failed searches, '
              'and verification; excludes fixture generation.',
              'limits':dict(seconds_per_method=args.seconds,
                            max_matrices=args.matrices,
                            guesses_per_matrix=args.guesses),
              'cases':rows,'summary':summary}
    Path(args.output).write_text(json.dumps(report,indent=2)+'\n')
    print('SUMMARY',json.dumps(summary),flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--trials',type=int,default=5)
    parser.add_argument('--max-prime',type=int,default=1048573)
    parser.add_argument('--seconds',type=float,default=15)
    parser.add_argument('--matrices',type=int,default=4)
    parser.add_argument('--guesses',type=int,default=32)
    parser.add_argument('--degree',type=int)
    parser.add_argument('--defect',type=int)
    parser.add_argument('--seed',type=int,default=20261027)
    parser.add_argument('--output',default='guess-results.json')
    run_experiment(parser.parse_args())
