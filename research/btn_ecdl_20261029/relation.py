"""Relation network: A1 + A2 = R, A_i on curve, x(A_i) in hashed set X_i."""
import numpy as np
from btn import Net


def add_gadget(net, F, P1, P2, R):
    """Constrain P1 + P2 = R (generic case x1 != x2) with affine formulas.
    P*, R are (xword, yword).  lam*(x1+x2) = y1+y2, x3 = lam^2+lam+x1+x2,
    y3 = lam*(x1+x3)+x3+y1."""
    n = F.n
    (x1, y1), (x2, y2), (x3, y3) = P1, P2, R
    s, t, lam, u = (net.word(n) for _ in range(4))
    net.xorw(s, x1, x2)
    net.xorw(t, y1, y2)
    m = net.mul(F, lam, s)
    for i in range(n): net.xorw([m[i]], [t[i]]) if False else net.add([m[i], t[i]], lambda a, b: a == b)
    l2 = net.sq(F, lam)
    for i in range(n): net.xor(x3[i], [l2[i], lam[i], s[i]])
    net.xorw(u, x1, x3)
    m2 = net.mul(F, lam, u)
    for i in range(n): net.xor(y3[i], [m2[i], x3[i], y1[i]])
    # x1 != x2: s != 0
    # x1 != x2: OR chain over s (parsimonious zero test, Lemma 2)
    o = net.word(n)
    net.add([o[0], s[0]], lambda a, b: a == b)
    for i in range(1, n):
        net.add([o[i], o[i - 1], s[i]], lambda a, b, c: a == (b | c))
    net.add([o[n - 1]], lambda a: a == 1)
    return lam


def on_curve(net, F, P):
    n = F.n
    x, y = P
    y2 = net.sq(F, y); xy = net.mul(F, x, y); x2 = net.sq(F, x); x3 = net.mul(F, x2, x)
    for i in range(n):
        net.add([y2[i], xy[i], x3[i]], lambda a, b, c, i=i: (a ^ b ^ c) == (1 if i == 0 else 0))


def member(net, xw, table):
    n = len(xw)
    net.add(xw, lambda *b: table[sum(v << i for i, v in enumerate(b))])


def build(F, R, tabs, full_check=True):
    """Interface = bits of (x1,y1,x2,y2) in this order."""
    net = Net()
    n = F.n
    P1 = (net.word(n), net.word(n)); P2 = (net.word(n), net.word(n))
    Rw = (net.word(n), net.word(n))
    net.fix(Rw[0], R[0]); net.fix(Rw[1], R[1])
    on_curve(net, F, P1)
    if full_check: on_curve(net, F, P2)
    lam = add_gadget(net, F, P1, P2, Rw)
    member(net, P1[0], tabs[0]); member(net, P2[0], tabs[1])
    iface = P1[0] + P1[1] + P2[0] + P2[1]
    return net, iface, lam, (P1, P2, Rw)
