#!/usr/bin/env sage -python
"""Bounded, replayable two-summand relation measurement; see README.md."""
import argparse, hashlib, json, math, platform, random, resource, time
from pathlib import Path
from sage.all import GF, EllipticCurve, PolynomialRing, prod, version, is_prime


def pt(P):
    return None if P.is_zero() else [int(P[0]), int(P[1])]


def run(args):
    start = time.perf_counter_ns(); cpu = time.process_time_ns()
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
    timings = {}; stages = {}; counts = {'candidates': 0, 'identity': 0, 'filtered': 0, 'duplicate_x': 0}
    def phase(name, fn):
        t = time.perf_counter_ns(); c = time.process_time_ns()
        result = fn(); timings[name] = {'wall_ns': time.perf_counter_ns()-t, 'cpu_ns': time.process_time_ns()-c}; return result
    def acquire():
        # Target is a fresh known coefficient pair, not a planted decomposition.
        while True:
            ar, br = rand(), rand(); R = ar*P + br*Q
            if R != O: break
        rows=[]; seen=set(); trace=[]
        walk_start=None; da=db=None; aa=[]; bb=[]
        if args.mode == 'walk':
            walk_a, walk_b = rand(), rand(); walk_start=[walk_a,walk_b]; da, db = rand(), rand()
            walk = walk_a*P + walk_b*Q; delta = da*P + db*Q
        if args.mode == 'two-group':
            aa=[rand() for _ in range(args.group_size)]; bb=[rand() for _ in range(args.group_size)]
            ap=[a*P for a in aa]; bq=[b*Q for b in bb]
        for idx in range(args.max_candidates):
            if len(rows) == 2*args.block_size: break
            if args.mode == 'independent': a,b=rand(),rand(); point=a*P+b*Q
            elif args.mode == 'walk':
                a,b,point=walk_a,walk_b,walk
                walk_a=(walk_a+da)%n; walk_b=(walk_b+db)%n; walk=walk+delta
            else:
                i=rng.randrange(len(aa)); j=rng.randrange(len(bb)); a,b=aa[i],bb[j]; point=ap[i]+bq[j]
            counts['candidates']+=1
            record={'index':idx,'a':a,'b':b,'point':pt(point)}
            if point == O: reason='identity'
            elif int(point[0]) % args.modulus != args.residue: reason='filtered'
            elif int(point[0]) in seen: reason='duplicate_x'
            else:
                reason='accepted'; seen.add(int(point[0])); rows.append(record.copy())
            if reason != 'accepted': counts[reason]+=1
            record['status']=reason; trace.append(record)
        return ar,br,R,rows,trace,{'walk_start':walk_start,'walk_step':[da,db] if walk_start is not None else None, 'group_a':aa,'group_b':bb}
    ar,br,R,rows,trace,generator=phase('generation', acquire)
    output={'schema':'e2e-relation-measurement/v1','config':vars(args),'curve':{'p':args.p,'a':args.curve_a,'b':args.curve_b,'P':pt(P),'Q':pt(Q),'n':n},'target':{'a':ar,'b':br,'point':pt(R)},'generator':generator,'candidates':trace,'counts':counts,'timings':timings,'status':'incomplete', 'metrics':{'degree_of_regularity':None,'solving_degree':None,'solver_operations':None},'metric_notes':{'degree_of_regularity':'not exported by backend','solving_degree':'not exported by backend','solver_operations':'not exported by backend'}}
    if len(rows) < 2*args.block_size:
        output['status']='candidate_budget_exhausted'
    else:
        def construct():
            ring=PolynomialRing(F, names=('x','y'), order='degrevlex'); x,y=ring.gens(); r=F(R[0]); A=F(args.curve_a); B=F(args.curve_b)
            s3=(x-y)**2*r**2-2*((x+y)*(x*y+A)+2*B)*r+(x*y-A)**2-4*B*(x+y)
            f=prod(x-F(t['point'][0]) for t in rows[:args.block_size]); g=prod(y-F(t['point'][0]) for t in rows[args.block_size:])
            return ring,[s3,f,g]
        ring,polys=phase('polynomial_construction', construct)
        output['system']={'variables':['x','y'],'order':'degrevlex','input_polynomials':[str(x) for x in polys], 'input_degrees':[int(x.total_degree()) for x in polys]}
        basis=phase('groebner_basis', lambda:ring.ideal(polys).groebner_basis())
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
        output['status']='complete'; output['useful_relations']=sum(bool(w['verified']) for w in output['witnesses'])
    output['total']={'wall_ns':time.perf_counter_ns()-start,'cpu_ns':time.process_time_ns()-cpu,'peak_rss_native':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'peak_rss_note':'process lifetime; bytes on macOS, KiB on Linux'}
    output['environment']={'sage':version(),'python':platform.python_version(),'platform':platform.platform(),'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    return output


def main():
    p=argparse.ArgumentParser(); p.add_argument('--p',type=int,required=True); p.add_argument('--curve-a',type=int,required=True); p.add_argument('--curve-b',type=int,required=True)
    for name in ('px','py','qx','qy'): p.add_argument('--'+name,type=int,required=True)
    p.add_argument('--mode',choices=['independent','walk','two-group'],required=True); p.add_argument('--seed',type=int,required=True); p.add_argument('--block-size',type=int,default=4); p.add_argument('--group-size',type=int,default=8); p.add_argument('--max-candidates',type=int,default=1000); p.add_argument('--modulus',type=int,default=1); p.add_argument('--residue',type=int,default=0); p.add_argument('--output',required=True)
    a=p.parse_args()
    if min(a.block_size,a.group_size,a.max_candidates,a.modulus)<1 or not 0<=a.residue<a.modulus: p.error('invalid sizes or residue')
    Path(a.output).write_text(json.dumps(run(a),indent=2,sort_keys=True)+'\n')
if __name__=='__main__': main()
