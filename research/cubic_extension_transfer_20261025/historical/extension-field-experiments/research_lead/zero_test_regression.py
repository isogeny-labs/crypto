"""Independent Sage 10.6 Hess zero-test regression and order certificate.

The curve is a new quotient from the sweep at p=13. Direct point counts,
divisor-class equality, and the production certificate must agree. The generic
is_zero/inequality discrepancy is recorded, not required on future Sage versions.
"""
import json
from pathlib import Path
import sys
from sage.all import GF,PolynomialRing,FunctionField,version
from quotient_theta import curve_points,jacobian_order_certificate
ROOT=Path(__file__).resolve().parent; sys.path.insert(0,str(ROOT.parent))
from tian import certified_plane_orders


def main():
    p=13;k=GF(p);R=PolynomialRing(k,['X','Y','Z']);X,Y,Z=R.gens()
    f=(X**4-5*X**3*Y+2*X**2*Y**2+4*X*Y**3-6*Y**4+6*X**3*Z
       -6*X**2*Y*Z+5*X*Y**2*Z-5*Y**3*Z+3*X**2*Z**2+X*Y*Z**2
       -Y**2*Z**2+3*X*Z**3-Y*Z**3-6*Z**4)
    aff,_=curve_points(f,k)
    F0=FunctionField(k,'x');x=F0.gen();Ry=PolynomialRing(F0,'y');y=Ry.gen()
    F=certified_plane_orders(F0.extension(Ry(f(x,y,1)).monic(),'y'))
    x=F(x);y=F.gen();O=F.maximal_order()
    div=[O.ideal(x-a,y-b).divisor() for a,b,_ in aff]
    B=div[0];G=F.jacobian(base_div=3*B).group();V=G.point(div[1]-B)
    W=2259*V; assert W==G.zero()
    Ns=[]
    for n in [1,2,3]:
        a,b=curve_points(f,GF(p**n,name='b'));Ns.append(len(a)+len(b))
    assert Ns==[14,170,2381]
    actual=p**3+1+(Ns[2]-p**3-1)//3;assert actual==2259
    certificate=jacobian_order_certificate(f,p,2137)
    assert certificate['jacobian_order']==actual and certificate['serre_twist_obstruction']
    out={'status':'passed','sage':version(),'quartic':str(f),'point_counts':Ns,
         'independent_jacobian_order':actual,'class_equality_says_zero':bool(W==G.zero()),
         'generic_is_zero_says_zero':bool(W.is_zero()),
         'generic_inequality_says_nonzero':bool(W!=G.zero()),'certificate':certificate}
    (ROOT/'results/zero_test_regression.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps(out),flush=True)


if __name__=='__main__':main()
