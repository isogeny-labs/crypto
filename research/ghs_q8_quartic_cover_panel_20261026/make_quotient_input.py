#!/usr/bin/env python3
"""Freeze the order-554 quartic base-change quotient computation."""

from pathlib import Path

root = Path(__file__).resolve().parent
source = (root / '554.magma').read_text()
prefix = source.split('B, beta :=', 1)[0]
body = '''CK := BaseChange(C, 3);
print "extension_order", #BaseField(CK);
print "extension_points", #RationalPoints(CK);
A := AutomorphismGroup(CK);
print "curve_automorphism_order", #A;
print "curve_automorphism_generators", [Order(A.i) : i in [1..Ngens(A)]];
for i in [1..Ngens(A)] do
    H := AutomorphismGroup(CK, [A.i]);
    print "quotient_index", i;
    print "quotient_order", #H;
    try
        D, projection := CurveQuotient(H);
        print "quotient_genus", Genus(D);
        print "quotient_curve", D;
        print "quotient_projection", projection;
    catch e
        print "quotient_error", e;
    end try;
end for;
H := AutomorphismGroup(CK, [A.1*A.2]);
print "quotient_index", 3;
try
    D, projection := CurveQuotient(H);
    print "quotient_genus", Genus(D);
    print "quotient_curve", D;
    print "quotient_projection", projection;
catch e
    print "quotient_error", e;
end try;
'''
(root / '554_quotients.magma').write_text(prefix + body)
