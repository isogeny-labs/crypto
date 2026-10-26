"""Complete a new toy ECDLP transfer using a deliberately exhaustive inverse oracle.

The quotient/Jacobian identification is validated by calibrated Weil functions.
The inverse dictionary enumerates the target group, so it is O(r) work and is
only a correctness oracle, not a practical general image-inversion algorithm.
The DLP solver itself is actual smooth-divisor index calculus and receives no
source scalar or dictionary enumeration labels.
"""
import json
from pathlib import Path
from time import perf_counter
from sage.all import set_random_seed,matrix
from inverse_map import setup,half_divisor
from target_index_calculus import jacobian_context,solve
from common import bsgs_ec
ROOT=Path(__file__).resolve().parent


def main():
    set_random_seed(20261028);start=perf_counter();ctx=setup()
    g=ctx['g'];F,Gold,Bold,baseold,places=jacobian_context(g);F9=ctx['F'];x=ctx['x'];y=ctx['y']
    xx=F(F.base_field().gen());yy=F.gen();k=g.base_ring()
    ell=[k(c) for c in ctx['odd_line']]
    KC=xx.divisor().denominator();odd=half_divisor((ell[0]*xx+ell[1]*yy+ell[2]).divisor()+KC)
    Opoint=F.maximal_order().ideal(xx-k(ctx['a']),yy-k(ctx['b'])).divisor()
    base=odd+Opoint;assert base.degree()==3
    G=F.jacobian(base_div=base).group();B=Opoint.support()[0]
    places=[B]+[pl for pl in places if pl!=B]
    Opt=ctx['Opt'];L=ctx['L'];inc9=ctx['inc9'];section9=ctx['sec9']
    generator=G.point(places[1].divisor()-B.divisor());assert 373*generator==G.zero()
    def evaluate_at(ff,a,b):
        def polynomial(poly):
            value=L.zero()
            for c in reversed(poly.list()):value=value*a+inc9(c)
            return value
        value=L.zero()
        for c in reversed(ff.list()):
            den=polynomial(c.denominator())
            if not den:raise ZeroDivisionError('Evaluation chart denominator')
            value=value*b+polynomial(c.numerator())/den
        return value
    def direct_signature(D):
        if any(e!=1 or pl.is_infinite_place() for pl,e in D.list()):return None
        points=[]
        for pl,e in D.list():
            KP,_,to=pl._residue_field();embedding=KP.embeddings(L)[0]
            a,b=embedding(to(xx)),embedding(to(yy))
            points.extend((a**(7**j),b**(7**j)) for j in range(pl.degree()))
        assert len(points)==3
        denominator=matrix(L,[[1,a,b] for a,b in points]).det()**2
        if not denominator:return None
        values=[]
        try:
            for fn in ctx['functions']:
                num=matrix(L,[[evaluate_at(ff,a,b) for a,b in points] for ff in fn['basis']]).det()**2
                for a,b in points:num*=evaluate_at(fn['principal'],a,b)
                value=num/denominator/inc9(fn['at_zero']);assert value**(7**9)==value
                values.append(section9(value))
        except ZeroDivisionError:return None
        return tuple(values)
    lookup={};missing=0;V=G.zero();before=perf_counter()
    for i in range(373):
        div=V.divisor()+base
        sig=direct_signature(div)
        if sig is None:missing+=1
        elif sig in lookup:
            W=lookup[sig];assert V==W or V==-W
        else:lookup[sig]=V
        V+=generator
        if i%25==0:print('INVERSE ORACLE',i,'keys',len(lookup),'seconds',perf_counter()-before,flush=True)
    oracle_seconds=perf_counter()-before;assert V==G.zero()
    print('ORACLE COMPLETE',len(lookup),'missing',missing,flush=True)
    E=ctx['source'];z=E.base_field().gen()
    P=E(2*z*z+5*z+5,6*z*z+6*z+1);Q=E(4*z*z+2*z,z+2)
    for multiplier in range(1,373):
        PP=multiplier*P;QQ=multiplier*Q
        signatures=[ctx['source_signature'](R,odd_reference=True) for R in [PP,QQ,PP+QQ]]
        if all(sig in lookup for sig in signatures):break
    else:raise AssertionError('No source triple matches the independently evaluated target functions')
    S,T0,U=[lookup[sig] for sig in signatures]
    if S+T0==U or S+T0==-U:T=T0
    elif S-T0==U or S-T0==-U:T=-T0
    else:raise AssertionError('The three inverse images do not admit consistent signs')
    assert not S==G.zero() and 373*S==G.zero()
    S=Gold.point(S.divisor());T=Gold.point(T.divisor())
    recovered=solve(S,T,373,baseold,places if places[0]==Bold else [Bold]+[pl for pl in places if pl!=Bold])
    scalar=recovered['scalar'];assert scalar*P==Q
    baseline=bsgs_ec(P,Q,373)
    assert baseline['scalar']==scalar
    out={'status':'passed','p':7,'r':373,'quartic':str(g),'recovered_scalar':scalar,
         'quotient_calibration_functions':6,'inverse_oracle_projective_images':len(lookup),
         'inverse_oracle_missing_points':missing,'inverse_oracle_seconds':oracle_seconds,
         'inverse_oracle_complexity':'O(r) exhaustive target enumeration; correctness oracle only',
         'three_image_sign_recovery':True,'target_generator':str(S),'target_target':str(T),
         'source_randomization_multiplier':multiplier,
         'target_index_calculus':recovered,'source_bsgs':baseline,'seconds':perf_counter()-start,
         'limits':['The inverse oracle is not an efficient general divisor inversion.',
                   'No DLP speedup or optimized O(p) target solver is claimed.']}
    (ROOT/'results/complete_toy.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps(out),flush=True)


if __name__=='__main__':main()
