#!/usr/bin/env python3
"""CT68: exhaustive Frobenius centralizers and simultaneous two-prime matrices.
Public mathematical controls, not an encoded-input cross-characteristic transfer.
"""
import json
from itertools import product


def mm(a, b, p):
    return ((a[0]*b[0]+a[1]*b[2])%p,
            (a[0]*b[1]+a[1]*b[3])%p,
            (a[2]*b[0]+a[3]*b[2])%p,
            (a[2]*b[1]+a[3]*b[3])%p)


def mv(a, v, p):
    return ((a[0]*v[0]+a[1]*v[1])%p,
            (a[2]*v[0]+a[3]*v[1])%p)


def det(a, p):
    return (a[0]*a[3]-a[1]*a[2])%p


def audit(p):
    G=[a for a in product(range(p),repeat=4) if det(a,p)]
    if len(G)!=(p*p-1)*(p*p-p):raise ValueError('GL2 size')
    nonsquare=next(d for d in range(2,p) if d not in {x*x%p for x in range(p)})
    Fs=(1,0,0,2)
    Fi=(0,nonsquare,1,0)
    Fj=(1,1,0,1)
    Fz=(1,0,0,1)
    def central(f):return [a for a in G if mm(a,f,p)==mm(f,a,p)]
    def orbit(g,v):return len({mv(a,v,p) for a in g})
    cs,ci,cj,cz=(central(f) for f in (Fs,Fi,Fj,Fz))
    both=[a for a in cs if mm(a,Fi,p)==mm(Fi,a,p)]
    result={
      'ell':p,
      'scalar':{'centralizer':len(cz),'orbit_eigen':orbit(cz,(1,0))},
      'split':{'centralizer':len(cs),'orbit_eigen':orbit(cs,(1,0)),'orbit_generic':orbit(cs,(1,1))},
      'jordan':{'centralizer':len(cj),'orbit_eigen':orbit(cj,(1,0)),'orbit_generic':orbit(cj,(0,1))},
      'irreducible':{'centralizer':len(ci),'orbit_eigen':orbit(ci,(1,0))},
      'joint_split_irreducible':{'centralizer':len(both),'orbit_line':orbit(both,(1,0)),
          'fixed_oriented_point':sum(mv(a,(1,0),p)==(1,0) for a in both)},
    }
    expected={
      'ell':p,
      'scalar':{'centralizer':len(G),'orbit_eigen':p*p-1},
      'split':{'centralizer':(p-1)**2,'orbit_eigen':p-1,'orbit_generic':(p-1)**2},
      'jordan':{'centralizer':p*(p-1),'orbit_eigen':p-1,'orbit_generic':p*(p-1)},
      'irreducible':{'centralizer':p*p-1,'orbit_eigen':p*p-1},
      'joint_split_irreducible':{'centralizer':p-1,'orbit_line':p-1,'fixed_oriented_point':1},
    }
    if result!=expected:raise ValueError(f'counterexample at {p}: {result} expected {expected}')
    if p==3:
        Fct=(0,1,1,1)  # CT67's actual characteristic-two Frobenius matrix
        actual=[a for a in cs if mm(a,Fct,p)==mm(Fct,a,p)]
        actual_target=central(Fct)
        item={'target_centralizer':len(actual_target),'joint_centralizer':len(actual),
              'joint_line_orbit':orbit(actual,(1,0))}
        if item!={'target_centralizer':8,'joint_centralizer':2,'joint_line_orbit':2}:
            raise ValueError('CT67 actual matrix failed')
        result['joint_11a1_ell3_matrix_model']=item
    if p==3 and (len(ci),len(both))!=(8,2):raise ValueError('CT67 refinement')
    return result


def main():
    result={'classification':'fixed_frobenius_centralizer','field':'F_ell',
            'cases':[audit(p) for p in (3,5,7,11)],
            'scope':'abstract_GL2_relabelings_only_no_global_input_lift'}
    print(json.dumps(result,sort_keys=True,separators=(',',':')))

if __name__=='__main__':main()
