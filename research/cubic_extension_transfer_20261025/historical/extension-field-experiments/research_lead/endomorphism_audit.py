"""Same endomorphism order and isogeny class, different quotient coverage."""
import hashlib,json
from pathlib import Path
from time import perf_counter
from sage.all import GF,hilbert_class_polynomial
from quotient_sweep import parse_curve
ROOT=Path(__file__).resolve().parent


def main():
    start=perf_counter();old=json.loads((ROOT/'results/structure_audit.json').read_text())
    p=old['p'];K=GF(p**3,name='z');discriminants=[-2355,-21195]
    polynomials={d:hilbert_class_polynomial(d) for d in discriminants}
    curves=[]
    for label in ['bad_curve','good_curve']:
        E=parse_curve(old[label],K);j=E.j_invariant()
        n9=len(E.division_polynomial(9).roots(multiplicities=False))
        n27=len(E.division_polynomial(27).roots(multiplicities=False))
        evaluations={d:polynomials[d](j)==0 for d in discriminants}
        assert n9==40 and n27==121 and evaluations=={-2355:False,-21195:True}
        curves.append({'role':label,'curve':str(E),'nine_x_roots':n9,'twentyseven_x_roots':n27,
                       'class_polynomial_vanishes':evaluations,'endomorphism_discriminant':-21195})
    out={'status':'passed','p':p,'r':old['r'],'curves':curves,'seconds':perf_counter()-start,
         'class_polynomials':[{'discriminant':d,'degree':int(H.degree()),
                               'integer_coefficients_sha256':hashlib.sha256(str(list(H)).encode()).hexdigest(),
                               'coefficients_mod_p':list(map(int,H.change_ring(GF(p)).list()))}
                              for d,H in polynomials.items()],
         'frobenius_discriminant':-1716795,'fundamental_discriminant':-2355,
         'frobenius_conductor':27,'common_endomorphism_conductor':3,
         'conclusion':'Coverage differs within one isogeny class at the same geometric endomorphism order.',
         'reason':'Scalar -I on E[9] forces the endomorphism order to contain (pi+1)/9. Failure of scalar -I on E[27] excludes the maximal order; the only remaining order has conductor 3.',
         'limits':'The specific twelve-kernel coverage property is compared; no novelty priority claim.'}
    (ROOT/'results/endomorphism_audit.json').write_text(json.dumps(out,indent=2)+'\n')
    print('ENDOMORPHISM AUDIT COMPLETE',out['seconds'],flush=True)


if __name__=='__main__':main()
