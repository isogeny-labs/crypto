"""Exact initial syzygy check before any Koszul syzygies can occur.

For unequal-degree inputs use the homogeneous top map
  direct_sum R_(D-deg f_i) -> R_D.
This routine only returns a first fall when it occurs strictly below the
smallest degree deg(f_i)+deg(f_j), i != j. In that range the Koszul quotient
is trivial. It deliberately does not implement the general quotient case.
"""

from closure import monomials, rank_mod
from instance import normalized_problem


def early_first_fall(problem):
    problem = normalized_problem(problem)
    p,n = problem['p'],problem['n']
    polynomials = [f for f in problem['polynomials'] if f]
    degrees = [max(sum(e) for c,e in f) for f in polynomials]
    boundary = min((a+b for i,a in enumerate(degrees)
                    for j,b in enumerate(degrees) if i < j), default=0)
    records = []
    for d in range(min(degrees),boundary):
        columns = [e for e in monomials(n,d) if sum(e) == d]
        index = {e:j for j,e in enumerate(columns)}
        rows = []
        for f,top_degree in zip(polynomials,degrees):
            if top_degree > d:
                continue
            top = [(c,e) for c,e in f if sum(e) == top_degree]
            multipliers = [e for e in monomials(n,d-top_degree)
                           if sum(e) == d-top_degree]
            for multiplier in multipliers:
                row = [0]*len(columns)
                for c,e in top:
                    exp = tuple(a+b for a,b in zip(e,multiplier))
                    row[index[exp]] = (row[index[exp]]+c) % p
                rows.append(row)
        rank = rank_mod(rows,p)
        records.append({'degree':d,'domain_dimension':len(rows),'rank':rank,
                        'kernel_dimension':len(rows)-rank})
        if rank < len(rows):
            return {'first_fall_degree':d,'first_possible_koszul_degree':boundary,
                    'convention':'homogeneous top syzygies modulo Koszul syzygies',
                    'checks':records}
    return {'first_fall_degree':None,'first_possible_koszul_degree':boundary,
            'checks':records,'scope':'general Koszul quotient not implemented'}
