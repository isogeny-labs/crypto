#!/usr/bin/env python3
"""Emit auditable 2-isogeny paths from every order-554 source to b=105."""

from pathlib import Path

root=Path(__file__).resolve().parent
prefix=(root/'554.magma').read_text().split('B, beta :=',1)[0]
code='''CK := BaseChange(C,3);
L := BaseField(CK);
w := L.1;
P1<X> := PolynomialRing(L);
models := [14,84,105,222,235,408,437,454,493];
edges := [<14,493>,<84,14>,<105,454>,<222,235>,<235,105>,
          <408,84>,<437,408>,<454,437>,<493,222>];
function SourceCurve(n,L,w)
    bn := &+[(Integers()!((n div 2^i) mod 2))*w^i : i in [0..8]];
    return EllipticCurve([L|1,1,0,0,bn]);
end function;
for start in models do
    E := SourceCurve(start,L,w);
    G := [g : g in RationalPoints(E) | Order(g) eq 277][1];
    Q := 7*G;
    assert Order(Q) eq 277;
    print "source",start,"generator",G,"target",Q;
    current := start;
    path := [start];
    while current ne 105 do
        following := [pair[2] : pair in edges | pair[1] eq current][1];
        F, two_map := IsogenyFromKernel(E,X);
        assert Degree(two_map) eq 2;
        E_next := SourceCurve(following,L,w);
        assert IsIsomorphic(F,E_next);
        normalize := Isomorphism(F,E_next);
        G := normalize(two_map(G));
        Q := normalize(two_map(Q));
        assert Order(G) eq 277 and Order(Q) eq 277 and Q eq 7*G;
        print "edge_checked",current,following,"degree",Degree(two_map);
        E := E_next;
        current := following;
        Append(~path,current);
    end while;
    print "path",path,"terminal_generator",G,"terminal_target",Q;
end for;
'''
(root/'554_isogeny_paths.magma').write_text(prefix+code)
