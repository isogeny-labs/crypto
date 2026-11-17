#!/usr/bin/env python3
"""Create a Magma independent genus-one model from Sage's exact invariants."""

import json
from pathlib import Path

root=Path(__file__).resolve().parent
data=json.loads((root/'explicit_invariants.json').read_text())
row=data['results'][0]
assert row['quotient_points']==554
prefix=(root/'554.magma').read_text().split('B, beta :=',1)[0]

def v(n):
    return '0' if n<0 else ('1' if n==0 else f'w^{n}')

T=row['coordinate_transform']
matrix_rows=[', '.join(v(n) for n in line) for line in T]
b=[v(n) for n in row['B2_exponents']]
d=[v(n) for n in row['C4_exponents']]
body=f'''CK := BaseChange(C, 3);
L := BaseField(CK);
w := L.1;
assert DefiningPolynomial(L) eq Polynomial([L|1,0,0,0,1,0,0,0,0,1]);
T := Matrix(L,3,3,[{', '.join(v(n) for line in T for n in line)}]);
P3<t0,t1,t2,t3> := PolynomialRing(L,4);
Q1 := t1^2 + t0*t2;
Q2 := {v(row['c_exponent'])}*t3^2 + ({b[0]}*t0 + {b[1]}*t1 + {b[2]}*t2)*t3
    + {d[0]}*t0^2 + {d[1]}*t0*t1 + {d[2]}*t1^2
    + {d[3]}*t1*t2 + {d[4]}*t2^2;
D := Curve(ProjectiveSpace(P3),[Q1,Q2]);
print "quotient_smooth", IsNonsingular(D);
print "quotient_genus", Genus(D);
print "quotient_points", #RationalPoints(D);
P2 := CoordinateRing(AmbientSpace(CK));
vx := P2.1; vy := P2.2; vz := P2.3;
Xf := T[1,1]*vx+T[1,2]*vy+T[1,3]*vz;
Yf := T[2,1]*vx+T[2,2]*vy+T[2,3]*vz;
Zf := T[3,1]*vx+T[3,2]*vy+T[3,3]*vz;
Uf := Xf^2+Xf*Yf;
projection := map<CK->D|[Yf^2,Yf*Zf,Zf^2,Uf]>;
print "projection_regular", IsRegular(projection);
for p in RationalPoints(CK) do
    xyz := Vector(L,Eltseq(p))*Transpose(T);
    X := xyz[1]; Y := xyz[2]; Z := xyz[3];
    U := X^2 + X*Y;
    q := D![Y^2,Y*Z,Z^2,U];
    assert projection(p) eq q;
    print "source_point", p;
    print "quotient_point", q;
    E, psi := EllipticCurve(D,q);
    print "elliptic_model", E;
    print "elliptic_points", #E;
    for n in { [14,84,105,222,235,408,437,454,493] } do
        bn := &+[(Integers()!((n div 2^i) mod 2))*w^i : i in [0..8]];
        Es := EllipticCurve([L|1,1,0,0,bn]);
        print "source_b_order_isomorphic", n, #Es, IsIsomorphic(E,Es);
    end for;
    bn := &+[(Integers()!((105 div 2^i) mod 2))*w^i : i in [0..8]];
    Es := EllipticCurve([L|1,1,0,0,bn]);
    source_to_quotient := Isomorphism(Es,E,[w^424,w^228,w^167,w^209]);
    print "source_105_isomorphism", source_to_quotient;
    print "source_105_isomorphism_data", IsomorphismData(source_to_quotient);
    cpoints := RationalPoints(CK);
    images := [psi(projection(p4)) : p4 in cpoints];
    for i in [1..#cpoints] do
        ep0 := images[i];
        j := Index(images,7*ep0);
        if Order(ep0) eq 277 and j ne 0 then
            print "paired_source_generator", ep0 @@ source_to_quotient;
            print "paired_source_target", (7*ep0) @@ source_to_quotient;
            print "paired_generator_fiber", [cpoints[t] : t in [1..#cpoints] | images[t] eq ep0];
            print "paired_target_fiber", [cpoints[t] : t in [1..#cpoints] | images[t] eq 7*ep0];
            print "paired_origin_fiber", [cpoints[t] : t in [1..#cpoints] | images[t] eq E!0];
            break;
        end if;
    end for;
    for g in RationalPoints(Es) do
        if Order(g) eq 277 then
            print "source_generator", g;
            print "source_target_7G", 7*g;
            print "quotient_generator", source_to_quotient(g);
            print "quotient_target", source_to_quotient(7*g);
            for p2 in RationalPoints(CK) do
                ep := psi(projection(p2));
                if Order(ep) eq 277 then
                    print "fiber_target_elliptic", ep;
                    print "fiber_target_projection", projection(p2);
                    print "fiber_target_points", [p3 : p3 in RationalPoints(CK) |
                        projection(p3) eq projection(p2)];
                    print "fiber_origin_points", [p3 : p3 in RationalPoints(CK) |
                        projection(p3) eq q];
                    print "fiber_scalar_in_source_generator",
                        Index([i*source_to_quotient(g) : i in [1..276]],ep);
                    break;
                end if;
            end for;
            break;
        end if;
    end for;
    break;
end for;
'''
(root/'554_elliptic_quotient.magma').write_text(prefix+body)
