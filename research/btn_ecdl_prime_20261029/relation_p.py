"""Relation network over F_p: A1 + A2 = R, A_i on curve, x(A_i) in hashed set X_i.
Interface (for Lemma-1 sampling) = bits of (x1, y1, x2, y2)."""
from btn import Net
from prime_field import on_curve_p, add_relation, _fix_const


def build_p(C, R, tabs):
    F = C.F; d = F.d
    net = Net()
    P1 = (net.word(d), net.word(d)); P2 = (net.word(d), net.word(d))
    Rw = (net.word(d), net.word(d)); _fix_const(net, Rw[0], R[0]); _fix_const(net, Rw[1], R[1])
    on_curve_p(net, C, P1); on_curve_p(net, C, P2)
    add_relation(net, C, P1, P2, Rw)
    for P, t in ((P1, tabs[0]), (P2, tabs[1])):
        net.add(P[0], lambda *b, t=t: t[sum(v << i for i, v in enumerate(b))] if sum(v << i for i, v in enumerate(b)) < len(t) else 0)
    return net, P1[0] + P1[1] + P2[0] + P2[1]
