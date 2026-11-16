"""Record explicit point-difference order witnesses for every obstructed quartic."""
import argparse,ast,json
from pathlib import Path
from math import isqrt
from sage.all import GF,PolynomialRing,factor
from target_index_calculus import jacobian_context
from quotient_theta import curve_points
ROOT=Path(__file__).resolve().parent


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--audit',default='counterexample_audit.json')
    ap.add_argument('--output',default='explicit_counterexample_witnesses.json');args=ap.parse_args()
    audit=json.loads((ROOT/'results'/args.audit).read_text())
    p=audit['p'];N=audit['obstructed_jacobian_order'];k=GF(p);R=PolynomialRing(k,['X','Y','Z']);X,Y,Z=R.gens()
    rows=[];lo=p**3+1-isqrt(4*p**3);hi=p**3+1+isqrt(4*p**3)
    key='all_12_quotient_certificates' if 'all_12_quotient_certificates' in audit else 'all_12_independent_quotient_certificates'
    for row in audit[key]['kernels']:
        g=R({ast.literal_eval(e):c for e,c in row['quartic_coefficients'].items()})
        c=k.zero()
        if not g(0,1,0):
            c=next(c for c in k if g(c,1,0));g=g(X+c*Y,Y,Z)
        F,_,_,_,_=jacobian_context(g);x=F(F.base_field().gen());y=F.gen();O=F.maximal_order()
        aff,_=curve_points(g,k);aff=sorted(aff,key=lambda P:(int(P[0]),int(P[1])))
        Bpoint=aff[0];B=O.ideal(x-Bpoint[0],y-Bpoint[1]).divisor();G=F.jacobian(base_div=3*B).group()
        exponent=1;witnesses=[]
        for point in aff[1:]:
            P=O.ideal(x-point[0],y-point[1]).divisor();V=G.point(P-B)
            assert N*V==G.zero();d=N
            for ell,_ in factor(N):
                while d%ell==0 and (d//ell)*V==G.zero():d//=int(ell)
            from math import lcm
            exponent=lcm(exponent,d)
            witnesses.append({'point':[int(point[0]),int(point[1])],'base_point':[int(Bpoint[0]),int(Bpoint[1])],
                              'divisor_class_order':d,'annihilator_verified':True,
                              'proper_prime_divisions_nonzero':all(not (d//int(l))*V==G.zero() for l,_ in factor(d))})
            multiples=[j*exponent for j in range((lo+exponent-1)//exponent,hi//exponent+1)]
            if len(multiples)==1:break
        assert multiples==[N]
        assert row['N1']==p+1 and row['N2']==p*p+1
        rows.append({'kernel_index':row['kernel_index'],'coordinate_change_X_equals_Xprime_plus_cY':int(c),
                     'point_counts':[row['N1'],row['N2']],'weil_interval':[lo,hi],
                     'witnesses':witnesses,'unique_multiple':N,'verified':True})
        print('WITNESS',row['kernel_index'],exponent,N,flush=True)
    out={'status':'passed','p':p,'source_prime_order':audit['r'],'obstructed_jacobian_order':N,
         'certificates':rows,'scope':'Explicit rational-point divisor witnesses plus previously exact N1,N2 counts'}
    (ROOT/'results'/args.output).write_text(json.dumps(out,indent=2)+'\n')


if __name__=='__main__':main()
