"""Test one-step obstruction escape and collect exact 3-adic invariants.

Unlike the earlier Hesse-family search, random inputs here have unrestricted
short-Weierstrass coefficients. Outcomes are exploratory, not density estimates.
All-negative neighborhoods are independently certified before being reported.
"""
import argparse, json
from collections import deque
from pathlib import Path
from time import perf_counter
from sage.all import (GF, EllipticCurve, Integer, PolynomialRing, matrix,
                      set_random_seed)
from quotient_sweep import parse_curve
from quotient_theta import normal_three, action_matrices, invariant_tensor, kernels, construct
from obstruction_probe import chi_square_class

ROOT = Path(__file__).resolve().parent


def orbit_key(E, p):
    j = E.j_invariant()
    return min(tuple(int(c) for c in a.polynomial().padded_list(3))
               for a in [j, j**p, j**(p*p)])


def screen(E, p, complete=False):
    L = GF(p**18, name='v'); emb = E.base_field().embeddings(L)[0]
    EL = EllipticCurve(L, [emb(E.a4()), emb(E.a6())])
    basis, sec, _ = normal_three(EL)
    action = action_matrices(EL, basis, sec)
    actions = [{c: M.apply_map(lambda x: x**(p**j)) for c, M in action.items()}
               for j in range(3)]
    R = PolynomialRing(L, 'u'); u = R.gen()
    roots = (u**3 + EL.a4()*u + EL.a6()).roots(multiplicities=False)
    signs = []
    for rows in kernels():
        T, _ = invariant_tensor(actions, rows)
        value, good = chi_square_class(T, roots, EL, p, emb)
        signs.append({'value': value, 'unobstructed': good})
        if good and not complete: break
    return signs


def invariants(E, p, r, matrix9=False):
    trace = p**3 + 1 - r; disc = trace*trace - 4*p**3
    roots9 = len(E.division_polynomial(9).roots(multiplicities=False))
    out = {'trace': trace, 'frobenius_discriminant': disc,
           'discriminant_v3': int(Integer(disc).valuation(3)),
           'twist_order_v3': int(Integer(2*(p**3+1)-r).valuation(3)),
           'nine_torsion_x_roots': roots9}
    if matrix9:
        L = GF(p**18, name='v'); emb = E.base_field().embeddings(L)[0]
        EL = EllipticCurve(L, [emb(E.a4()), emb(E.a6())])
        pts = [EL(x, (x**3+EL.a4()*x+EL.a6()).sqrt())
               for x in EL.division_polynomial(9).roots(multiplicities=False)]
        P = next(P for P in pts if not (3*P).is_zero())
        Q = next(Q for Q in pts if Q.weil_pairing(P, 9)**3 != 1)
        def key(V): return () if V.is_zero() else (V[0], V[1])
        coords = {key(a*P+b*Q): (a,b) for a in range(9) for b in range(9)}
        assert len(coords) == 81
        images = [coords[key(EL(V[0]**(p**3), V[1]**(p**3)))] for V in [P,Q]]
        M = matrix(2,2,[images[j][i] for i in range(2) for j in range(2)])
        assert all((int(M[i,j]) + int(i==j)) % 3 == 0 for i in range(2) for j in range(2))
        assert int(M.det()) % 9 == p**3 % 9 and int(M.trace()) % 9 == trace % 9
        A = matrix(GF(3), 2, 2, [(int(M[i,j])+int(i==j))//3 for i in range(2) for j in range(2)])
        assert roots9 == (3**(4-int(A.rank()))-1)//2
        out.update(frobenius_mod9=[list(map(int,row)) for row in M.rows()],
                   minus_identity_lift_rank=int(A.rank()), minus_identity_lift_det=int(A.det()),
                   minus_identity_lift_trace=int(A.trace()))
    return out


def neighbors(E, p, r):
    out=[]
    for phi in E.isogenies_prime_degree(3):
        E1=phi.codomain(); E1.set_order(r, num_checks=0)
        n=len(E1.division_polynomial(3).roots(multiplicities=False))
        out.append((phi,E1,n))
    return out


def examine(E,p,r,complete=False):
    signs=screen(E,p,complete=complete)
    out={'p':p,'r':r,'curve':str(E),'orbit_key':orbit_key(E,p),
         'kernels_tested':signs,'usable_witness':next((i for i,s in enumerate(signs) if s['unobstructed']),None),
         'all_12_tested':len(signs)==12, **invariants(E,p,r)}
    if out['usable_witness'] is None:
        out.update(invariants(E,p,r,matrix9=True)); out['neighbors']=[]
        for phi,E1,n in neighbors(E,p,r):
            row={'curve':str(E1),'kernel':str(phi.kernel_polynomial()),'three_lines':n}
            if n==4:
                ss=screen(E1,p,complete=True)
                row.update(usable_count=sum(s['unobstructed'] for s in ss),signs=ss,
                           **invariants(E1,p,r,matrix9=True))
            out['neighbors'].append(row)
        out['one_step_escape_found']=any(n.get('usable_count',0)>0 for n in out['neighbors'])
        if not out['one_step_escape_found']:
            # A genuine candidate must certify all 12 on the center and each eligible neighbor.
            out['independent_center_certificate']=construct(E,p,certify=True)
            assert all(not k['quotient_is_base_field_jacobian'] for k in out['independent_center_certificate']['kernels'])
            for row in out['neighbors']:
                if row['three_lines']!=4:continue
                cert=construct(parse_curve(row['curve'],E.base_field()),p,certify=True)
                assert all(not k['quotient_is_base_field_jacobian'] for k in cert['kernels'])
                row['independent_certificate']=cert
            out['certified_one_step_counterexample']=True
    return out


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--mode',choices=['component','random'],default='component')
    ap.add_argument('--count',type=int,default=500);ap.add_argument('--seconds',type=int,default=1200)
    ap.add_argument('--target-nine',action='store_true',help='Stratify toward full scalar-minus-I nine-torsion using at most one extra isogeny')
    ap.add_argument('--seed',type=int,default=20261105)
    args=ap.parse_args();seed=args.seed;set_random_seed(seed);start=perf_counter()
    suffix='_nine' if args.target_nine else ''
    path=ROOT/f'results/neighborhood_{args.mode}{suffix}.json';rows=[];draws=0;seen=set();queue=deque()
    primes=[67,73,79,97,103,109,127,139,151,157,163,181,193,199]
    fields={p:GF(p**3,name='z') for p in primes}
    if args.mode=='component':
        old=json.loads((ROOT/'results/counterexample_audit.json').read_text())
        E=parse_curve(old['curve'],fields[old['p']]);queue.append((old['p'],old['r'],E,0))
        seen.add((old['p'],orbit_key(E,old['p'])))
    while len(rows)<args.count and perf_counter()-start<args.seconds:
        if args.mode=='component':
            if not queue:break
            p,r,E,depth=queue.popleft();E.set_order(r,num_checks=0)
        else:
            # Rotate attempted primes in the targeted stratum so exhausting a
            # small field's finite pool cannot stall every larger field.
            p=primes[(draws if args.target_nine else len(rows))%len(primes)];K=fields[p];draws+=1
            a,b=K.random_element(),K.random_element()
            if not 4*a**3+27*b*b:continue
            E=EllipticCurve(K,[a,b]);j=E.j_invariant()
            if j**p==j:continue
            R=PolynomialRing(K,'u');u=R.gen()
            if not (u**3+a*u+b).is_irreducible():continue
            xs=E.division_polynomial(3).roots(multiplicities=False)
            if len(xs) not in [1,4] or (xs[0]**3+a*xs[0]+b).is_square():continue
            r=int(E.cardinality())
            if r%9!=4 or (p**3+1-r)%p==0 or not Integer(r).is_prime():continue
            if args.target_nine and (2*(p**3+1)-r)%81:continue
            if len(xs)==1:E=E.isogenies_prime_degree(3)[0].codomain();E.set_order(r,num_checks=0)
            if args.target_nine and len(E.division_polynomial(9).roots(multiplicities=False))!=40:
                lifts=[E1 for phi,E1,n in neighbors(E,p,r)
                       if n==4 and len(E1.division_polynomial(9).roots(multiplicities=False))==40]
                if not lifts:continue
                E=lifts[0]
            key=(p,orbit_key(E,p))
            if key in seen:continue
            seen.add(key);depth=0
        row=examine(E,p,r,complete=args.mode=='component');row['depth']=depth
        if args.mode=='component':
            row['edges']=[]
            for phi,E1,n in neighbors(E,p,r):
                key=(p,orbit_key(E1,p));row['edges'].append({'orbit_key':key[1],'three_lines':n})
                if n==4 and key not in seen:
                    seen.add(key);queue.append((p,r,E1,depth+1))
        rows.append(row)
        out={'status':'running','seed':seed,'mode':args.mode,'target_nine':args.target_nine,'curves':rows,'draws':draws,
             'queued':len(queue),'seconds':perf_counter()-start,
             'one_step_counterexample_found':any(r.get('certified_one_step_counterexample',False) for r in rows),
             'scope':'Exploratory finite-field tests, not a density estimate or universal proof'}
        path.write_text(json.dumps(out,indent=2)+'\n')
        if len(rows)%25==0 or row['usable_witness'] is None:
            print(args.mode,len(rows),'p',p,'bad',row['usable_witness'] is None,'queue',len(queue),'s',int(out['seconds']),flush=True)
        if out['one_step_counterexample_found']:break
    out.update(status='complete',seconds=perf_counter()-start,
               component_closed=args.mode=='component' and not queue)
    path.write_text(json.dumps(out,indent=2)+'\n')
    print('NEIGHBORHOOD COMPLETE',args.mode,len(rows),'closed',out['component_closed'],flush=True)


if __name__=='__main__':main()
