#!/usr/bin/env python3
"""Generate the five frozen exhaustive Magma c-sweep inputs."""

from pathlib import Path

root = Path(__file__).resolve().parent
sources = ((482, 1, 43, 241), (524, 0, 9, 131),
           (548, 0, 155, 137), (554, 1, 105, 277),
           (556, 0, 32, 139))
for order, a2, b, prime in sources:
    script = f'''K<w> := GF(512);
k<a> := GF(8);
Embed(k,K);
assert DefiningPolynomial(K) eq Polynomial([K|1,0,0,0,1,0,0,0,0,1]);
assert DefiningPolynomial(k) eq Polynomial([k|1,1,0,1]);
ell := &+[(Integers()!(({b} div 2^i) mod 2))*w^i : i in [0..8]];
E := EllipticCurve([K|1,{a2},0,0,ell]);
assert #E eq {order};
print "source", {order}, {a2}, {b}, #E, {prime};
for j in [0..510] do
    try
        genus := WeilDescentGenus(E,k,w^j);
        print "row", {order}, {b}, j, genus;
    catch err
        print "error", {order}, {b}, j, err;
    end try;
end for;
'''
    (root / f'all_{order}.magma').write_text(script)
    print(order, len(script))
