#!/usr/bin/env sage -python
"""Bounded, replayable two-summand relation measurement; see README.md."""
import argparse, ctypes, hashlib, json, math, os, platform, random, resource, sys, time
from pathlib import Path
from sage.all import GF, EllipticCurve, PolynomialRing, prod, version, is_prime
from accounting import count_witnesses


def pt(P):
    return None if P.is_zero() else [int(P[0]), int(P[1])]


def top_form_regularity(input_terms, block_size):
    """First zero degree of the quotient by these three input top forms."""
    observed=[]
    for terms in input_terms:
        degree=max((sum(t['exponents']) for t in terms),default=-1)
        observed.append(sorted((tuple(t['exponents']),t['coefficient']) for t in terms if sum(t['exponents'])==degree))
    expected=[[((2,2),1)], [((block_size,0),1)], [((0,block_size),1)]]
    if observed != expected:
        return {'status':'unsupported_shape','degree':None,'top_forms':observed}
    return {'status':'exact_monomial_ideal', 'degree':1 if block_size==1 else block_size+1,
            'top_forms':observed, 'definition':'first d with <input highest-degree forms>_d equal to the degree-d homogeneous component',
            'last_surviving_monomial':[0,0] if block_size==1 else [block_size-1,1],
            'scope':'input-generator top-form ideal; not a backend solving degree or field-operation count'}


def run(args):
    start = time.perf_counter_ns(); cpu = time.process_time_ns()
    timings = {}
    output = {'schema':'e2e-relation-measurement/v1', 'config':vars(args), 'status':'running', 'phase':'setup', 'timings':timings, 'source_sha256_at_start':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), 'accounting_sha256_at_start':hashlib.sha256(Path(__file__).with_name('accounting.py').read_bytes()).hexdigest()}
    def checkpoint():
        if args.checkpoint:
            path = Path(args.checkpoint)
            tmp = path.with_name(path.name + '.tmp')
            tmp.write_text(json.dumps(output, indent=2, sort_keys=True) + '\n')
            os.replace(tmp, path)
    checkpoint()
    if args.p <= 3 or not is_prime(args.p):
        raise ValueError('p must be a prime greater than 3')
    F = GF(args.p); E = EllipticCurve(F, [args.curve_a, args.curve_b]); O = E(0)
    P = E(args.px, args.py); n = int(P.order())
    if not is_prime(n):
        raise ValueError('P must have prime order')
    Q = E(args.qx, args.qy)
    if int(E.order()) % n != 0 or math.gcd(int(E.order()) // n, n) != 1:
        raise ValueError('curve must have a unique subgroup of order n')
    if Q == O or n * Q != O:
        raise ValueError('Q must be a nonidentity point in the P subgroup')
    rng = random.Random(args.seed)
    rand = lambda: rng.randrange(n)
    counts = {'candidates': 0, 'identity': 0, 'filtered': 0, 'duplicate_x': 0}
    def phase(name, fn):
        output['phase'] = name; checkpoint()
        t = time.perf_counter_ns(); c = time.process_time_ns()
        result = fn(); timings[name] = {'wall_ns': time.perf_counter_ns()-t, 'cpu_ns': time.process_time_ns()-c}; return result
    def acquire():
        # These counters cover explicit calls in this phase, not the internals of scalar multiplication.
        ops={'target_draws':0, 'scalar_multiplication_calls':0, 'point_addition_calls':0}
        def gmul(k, point):
            ops['scalar_multiplication_calls']+=1
            return k*point
        def gadd(left, right):
            ops['point_addition_calls']+=1
            return left+right
        # Target is a fresh known coefficient pair, not a planted decomposition.
        while True:
            ops['target_draws']+=1
            ar, br = rand(), rand(); R = gadd(gmul(ar,P),gmul(br,Q))
            if R != O: break
        rows=[]; seen=set(); trace=[]
        walks=[]; aa=[]; bb=[]
        if args.mode == 'two-group':
            aa=[rand() for _ in range(args.group_size)]; bb=[rand() for _ in range(args.group_size)]
            ap=[gmul(a,P) for a in aa]; bq=[gmul(b,Q) for b in bb]
        for idx in range(args.max_candidates):
            if len(rows) == 2*args.block_size: break
            if args.mode == 'independent': a,b=rand(),rand(); point=gadd(gmul(a,P),gmul(b,Q))
            elif args.mode == 'walk':
                if idx % args.walk_length == 0:
                    walk_a,walk_b=rand(),rand(); da,db=rand(),rand()
                    walks.append({'start':[walk_a,walk_b], 'step':[da,db], 'first_candidate':idx})
                    walk=gadd(gmul(walk_a,P),gmul(walk_b,Q)); delta=gadd(gmul(da,P),gmul(db,Q))
                a,b,point=walk_a,walk_b,walk
                walk_a=(walk_a+da)%n; walk_b=(walk_b+db)%n; walk=gadd(walk,delta)
            else:
                i=rng.randrange(len(aa)); j=rng.randrange(len(bb)); a,b=aa[i],bb[j]; point=gadd(ap[i],bq[j])
            counts['candidates']+=1
            record={'index':idx,'a':a,'b':b,'point':pt(point)}
            if point == O: reason='identity'
            elif not args.x_min <= int(point[0]) <= args.x_max or int(point[0]) % args.modulus != args.residue: reason='filtered'
            elif int(point[0]) in seen: reason='duplicate_x'
            else:
                reason='accepted'; seen.add(int(point[0])); rows.append(record.copy())
            if reason != 'accepted': counts[reason]+=1
            record['status']=reason; trace.append(record)
        return ar,br,R,rows,trace,{'walks':walks, 'group_a':aa,'group_b':bb},ops
    ar,br,R,rows,trace,generator,generation_operations=phase('generation', acquire)
    output.update({'curve':{'p':args.p,'a':args.curve_a,'b':args.curve_b,'P':pt(P),'Q':pt(Q),'n':n},'target':{'a':ar,'b':br,'point':pt(R)},'generator':generator,'generation_operations':generation_operations,'candidates':trace,'counts':counts,'timings':timings,'status':'running', 'metrics':{'degree_of_regularity':None,'solving_degree':None,'solver_operations':None},'metric_notes':{'degree_of_regularity':'not exported by backend','solving_degree':'not exported by backend','solver_operations':'not exported by backend'}})
    checkpoint()
    if len(rows) < 2*args.block_size:
        output['status']='candidate_budget_exhausted'
    else:
        def construct():
            ring=PolynomialRing(F, names=('x','y'), order='degrevlex'); x,y=ring.gens(); r=F(R[0]); A=F(args.curve_a); B=F(args.curve_b)
            s3=(x-y)**2*r**2-2*((x+y)*(x*y+A)+2*B)*r+(x*y-A)**2-4*B*(x+y)
            f=prod(x-F(t['point'][0]) for t in rows[:args.block_size]); g=prod(y-F(t['point'][0]) for t in rows[args.block_size:])
            return ring,[s3,f,g]
        ring,polys=phase('polynomial_construction', construct)
        input_terms = [sorted(({'exponents':[int(e) for e in exp], 'coefficient':int(coef)} for exp,coef in poly.dict().items()), key=lambda item: item['exponents']) for poly in polys]
        output['solver']={'backend':'sage/libsingular:std','protocol_requested':args.solver_protocol,'protocol_stream':'stdout' if args.solver_protocol else None,'root_extraction':'sage ideal variety; backend selected by Sage'}
        output['system']={'variables':['x','y'],'order':'degrevlex','coefficient_modulus':args.p,'input_polynomials':[str(x) for x in polys], 'input_degrees':[int(x.total_degree()) for x in polys], 'input_terms':input_terms, 'input_term_counts':[len(terms) for terms in input_terms]}
        regularity=phase('top_form_regularity',lambda:top_form_regularity(input_terms,args.block_size))
        output['metrics']['degree_of_regularity']=regularity['degree']
        output['metric_notes']['degree_of_regularity']=('exact first-zero degree for this input top-form ideal; not solver solving degree' if regularity['degree'] is not None else 'unsupported top-form shape; no value reported')
        output['top_form_regularity']=regularity
        def compute_basis():
            # libSingular writes via C stdio, which must be flushed on both
            # sides of our markers for the saved stream to preserve ordering.
            libc = ctypes.CDLL(None) if args.solver_protocol else None
            if libc:
                sys.stdout.flush(); libc.fflush(None)
                print('=== BEGIN GROEBNER BASIS PROTOCOL ===', flush=True)
            try:
                return ring.ideal(polys).groebner_basis(algorithm='libsingular:std', prot=args.solver_protocol)
            finally:
                if libc:
                    sys.stdout.flush(); libc.fflush(None)
                    print('=== END GROEBNER BASIS PROTOCOL ===', flush=True)
        basis=phase('groebner_basis', compute_basis)
        output['system']['groebner_basis']=[str(x) for x in basis]
        output['metrics']['maximum_output_basis_degree']=max((int(x.total_degree()) for x in basis),default=0)
        solutions=phase('root_extraction',lambda:ring.ideal(basis).variety())
        output['roots']=[[int(v[ring.gen(0)]),int(v[ring.gen(1)])] for v in solutions]
        def verify():
            witnesses=[]
            for x,y in output['roots']:
                for i,u in enumerate(rows[:args.block_size]):
                    if u['point'][0]!=x: continue
                    for j,v in enumerate(rows[args.block_size:]):
                        if v['point'][0]!=y: continue
                        U=E(*u['point']); V=E(*v['point'])
                        for su in (-1,1):
                            for sv in (-1,1):
                                if su*U+sv*V != R: continue
                                a=(su*u['a']+sv*v['a']-ar)%n; b=(su*u['b']+sv*v['b']-br)%n
                                w=None if b==0 else (-a*pow(b,-1,n))%n
                                witnesses.append({'indices':[i,j+args.block_size],'signs':[su,sv],'A':a,'B':b,'scalar':w,'verified':w is not None and w*P==Q})
            return witnesses
        output['witnesses']=phase('relation_verification',verify)
        output['relation_counts']=count_witnesses(output['witnesses'])
        output['useful_relations']=output['relation_counts']['nonzero_b_verified']
        output['status']='verification_failed' if output['relation_counts']['nonzero_b_verification_failed'] else 'complete'
    output['total']={'wall_ns':time.perf_counter_ns()-start,'cpu_ns':time.process_time_ns()-cpu,'peak_rss_native':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'peak_rss_note':'process lifetime; bytes on macOS, KiB on Linux'}
    output['environment']={'sage':version(),'python':platform.python_version(),'python_implementation':platform.python_implementation(),'platform':platform.platform(),'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    output['phase']='finished'
    checkpoint()
    return output


def main():
    p=argparse.ArgumentParser(); p.add_argument('--p',type=int,required=True); p.add_argument('--curve-a',type=int,required=True); p.add_argument('--curve-b',type=int,required=True)
    for name in ('px','py','qx','qy'): p.add_argument('--'+name,type=int,required=True)
    p.add_argument('--walk-length',type=int,default=8); p.add_argument('--x-min',type=int,default=0); p.add_argument('--x-max',type=int,default=None)
    p.add_argument('--checkpoint', default=None); p.add_argument('--solver-protocol', action='store_true')
    p.add_argument('--mode',choices=['independent','walk','two-group'],required=True); p.add_argument('--seed',type=int,required=True); p.add_argument('--block-size',type=int,default=4); p.add_argument('--group-size',type=int,default=8); p.add_argument('--max-candidates',type=int,default=1000); p.add_argument('--modulus',type=int,default=1); p.add_argument('--residue',type=int,default=0); p.add_argument('--output',required=True)
    a=p.parse_args()
    if a.checkpoint and Path(a.checkpoint).resolve() == Path(a.output).resolve(): p.error('checkpoint and output paths must differ')
    if a.x_max is None: a.x_max=a.p-1
    if not 0 <= a.x_min <= a.x_max < a.p: p.error('invalid x interval')
    if min(a.walk_length,a.block_size,a.group_size,a.max_candidates,a.modulus)<1 or not 0<=a.residue<a.modulus: p.error('invalid sizes or residue')
    Path(a.output).write_text(json.dumps(run(a),indent=2,sort_keys=True)+'\n')
if __name__=='__main__': main()
