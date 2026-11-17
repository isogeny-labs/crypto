"""Probe the coverage conjecture near cases with only one usable kernel.

Breadth-first rational 3-isogeny walks preserve prime group order. Only nodes
with scalar -I action on E[3] are screened. Frobenius-conjugate j-invariants
are identified to avoid counting the same test three times.
"""
import argparse,json
from collections import deque
from pathlib import Path
from time import perf_counter
from sage.all import GF,set_random_seed
from quotient_sweep import parse_curve
from obstruction_probe import one_curve
from quotient_theta import construct
ROOT=Path(__file__).resolve().parent


def key(E,p):
    j=E.j_invariant()
    return min(tuple(int(x) for x in a.polynomial().list()) for a in [j,j**p,j**(p*p)])


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--limit',type=int,default=150)
    ap.add_argument('--depth',type=int,default=12);args=ap.parse_args()
    set_random_seed(20261027);start=perf_counter()
    old=json.loads((ROOT/'results/coverage_hunt.json').read_text())
    seeds=[r for r in old['curves'] if r['usable_quotient_count']==1]
    queue=deque();seen=set();rows=[];truncated_edges=0
    for row in seeds:
        p=row['p'];K=GF(p**3,name='z');E=parse_curve(row['prepared_curve'],K)
        k=(p,key(E,p))
        if k not in seen:queue.append((p,row['r'],E,0,None));seen.add(k)
    while queue and len(rows)<args.limit:
        p,r,E,depth,parent=queue.popleft();E.set_order(r,num_checks=0)
        assert len(E.division_polynomial(3).roots(multiplicities=False))==4
        ks=one_curve(E,p);good=sum(k['predicts_base_field_jacobian'] for k in ks)
        neighbors=E.isogenies_prime_degree(3);types=[]
        for phi in neighbors:
            E1=phi.codomain();nr=len(E1.division_polynomial(3).roots(multiplicities=False))
            types.append(nr)
            k=(p,key(E1,p))
            if nr==4 and k not in seen and depth<args.depth:
                seen.add(k);queue.append((p,r,E1,depth+1,len(rows)))
            elif nr==4 and k not in seen:
                truncated_edges+=1
        row={'p':p,'r':r,'curve':str(E),'j':str(E.j_invariant()),'depth':depth,
             'parent_index':parent,'usable_quotient_count':good,'neighbor_kernel_line_counts':types,'kernels':ks}
        if good==0:
            row['independent_certificate']=construct(E,p,certify=True)
            assert all(not k['quotient_is_base_field_jacobian'] for k in row['independent_certificate']['kernels'])
            floors=[phi.codomain() for phi,n in zip(neighbors,types) if n==1]
            if floors:
                floor=floors[0];up=floor.isogenies_prime_degree(3)
                assert len(up)==1 and up[0].codomain().j_invariant()==E.j_invariant()
                row['one_step_counterexample_source']=str(floor)
        rows.append(row)
        out={'seed':20261027,'curves':rows,'queued':len(queue),'depth_truncated_edges':truncated_edges,
             'seconds':perf_counter()-start,
             'all_obstructed_found':good==0,'scope':'Targeted isogeny walk; no random-density estimate'}
        (ROOT/'results/coverage_walk.json').write_text(json.dumps(out,indent=2)+'\n')
        print('WALK',len(rows),p,'depth',depth,'good',good,'neighbors',types,'queue',len(queue),flush=True)
        if good==0:break


if __name__=='__main__':main()
