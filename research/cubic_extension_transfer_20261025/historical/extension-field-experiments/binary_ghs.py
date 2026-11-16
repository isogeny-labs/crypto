"""Characteristic-two GHS geometry screening, with no target DLP claim.

For E: y^2+xy=x^3+b and b_i=(sqrt(b))^(q^i), the associated
Artin--Schreier space is spanned over F_2 by (1,b_i). Its rank m determines
the candidate cover genus 2^(m-1) or 2^(m-1)-1. Count the contributing
characters directly as an independent genus calculation.
"""
from time import perf_counter
from sage.all import GF, EllipticCurve, matrix, vector, factor
from common import field_data


def binary_screen(s,n,samples=12):
    start=perf_counter(); q=2**s; K=GF(q**n,name='z')
    cases=[]; seen=set()
    for b in K:
        if not b or b in seen:
            continue
        orbit=[b**(q**i) for i in range(n)]
        if len(set(orbit))!=n:
            continue
        seen.update(orbit)
        sqrtb=b.sqrt(); cs=[sqrtb**(q**i) for i in range(n)]
        rows=[[1]+[int(c.polynomial()[i]) for i in range(K.degree())] for c in cs]
        M=matrix(GF(2),rows)
        magic=int(M.rank())
        vectors=list(M.row_space())
        # Each nonzero character A*x+B/x contributes genus one exactly when
        # both simple poles are present; otherwise its genus is zero.
        genus=sum(int(v[0]==1 and any(v[1:])) for v in vectors)
        axis=vector(GF(2),[1]+[0]*K.degree())
        subtract=int(axis in M.row_space())
        assert genus==2**(magic-1)-subtract
        E=EllipticCurve(K,[1,0,0,0,b]); N=int(E.cardinality())
        r=int(max(factor(N))[0])
        cases.append({'b':str(b),'curve_order':N,'largest_prime_factor':r,
                      'magic_number':magic,'candidate_genus':genus,
                      'x_only_character_present':bool(subtract),
                      'generic_baseline_sqrt_r':r**0.5,
                      'fixed_genus_index_calculus_power_only':
                        q**(2-2/genus) if genus>=2 else None})
        if len(cases)>=samples:
            break
    assert cases
    return {'family':'binary_ghs_geometry_screen','p':2,'s':s,'n':s*n,
            'relative_extension_degree':n,'q':q,'status':'passed',
            'field':field_data(K),'samples':cases,
            'candidate_genera':sorted(set(c['candidate_genus'] for c in cases)),
            'seconds':perf_counter()-start,
            'claim':'GHS rank/genus eligibility screen only; no divisor map or attack timing'}
