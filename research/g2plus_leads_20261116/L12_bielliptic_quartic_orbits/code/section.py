"""Counted line sections, verifier, row builder (SPEC.md section 7)."""
from fp import pmul, trim, divroot, u_inv, u_pmul, Ops, Fp
from curve import eadd, emul


def _residual(F, S):
    """Return (status, roots) with roots a list of (value, multiplicity)."""
    S = trim(S)
    d = len(S) - 1
    if d == 2:
        e0, e1, e2 = S
        D = F.sub(F.mul(e1, e1), F.times4(F.mul(e2, e0)))
        if D == 0:
            t = F.inv(F.dbl(e2))
            return "split", [(F.mul(F.neg(e1), t), 2)]
        if F.sqrt(D) > 0:
            s = F.sqrt(D)
            t = F.inv(F.dbl(e2))
            r1 = F.mul(F.add(F.neg(e1), s), t)
            r2 = F.mul(F.sub(F.neg(e1), s), t)
            return "split", [(r1, 1), (r2, 1)]
        return "nonsplit", []
    if d == 1:
        e0, e1 = S
        return "split", [(F.mul(F.neg(e0), F.inv(e1)), 1)]
    raise RuntimeError("unexpected residual degree %d" % d)


def section(F, curve, points, index, i1, i2):
    """Counted section of the line through points[i1], points[i2]."""
    a1, a3, a2, a4, a6 = curve.a1, curve.a3, curve.a2, curve.a4, curve.a6
    x1, y1 = points[i1][0], points[i1][1]
    x2, y2 = points[i2][0], points[i2][1]
    terms = {}
    if x1 == x2:
        hx = F.add(F.mul(a1, x1), a3)
        fx = F.add(F.mul(F.add(F.mul(F.add(x1, a2), x1), a4), x1), a6)
        S = [F.neg(fx), 0, hx, 0, 1]
        S, _ = divroot(F, S, y1)
        S, _ = divroot(F, S, y2)
        st, roots = _residual(F, S)
        if st == "nonsplit":
            return st, {}
        terms[i1] = terms.get(i1, 0) + 1
        terms[i2] = terms.get(i2, 0) + 1
        for Y, mult in roots:
            k = index[(x1, Y)]
            terms[k] = terms.get(k, 0) + mult
        return st, terms
    m = F.mul(F.sub(y2, y1), F.inv(F.sub(x2, x1)))
    c = F.sub(y1, F.mul(m, x1))
    L = [c, m]
    L2 = pmul(F, L, L)
    L4 = pmul(F, L2, L2)
    HL2 = pmul(F, [a3, a1], L2)
    base = [a6, a4, a2, 1, 0]
    S = []
    for k in range(5):
        v = (L4[k] if k < len(L4) else 0)
        v = F.add(v, HL2[k] if k < len(HL2) else 0)
        v = F.sub(v, base[k])
        S.append(v)
    S = trim(S)
    S, _ = divroot(F, S, x1)
    S, _ = divroot(F, S, x2)
    st, roots = _residual(F, S)
    if st == "nonsplit":
        return st, {}
    terms[i1] = terms.get(i1, 0) + 1
    terms[i2] = terms.get(i2, 0) + 1
    for X, mult in roots:
        Y = F.add(F.mul(m, X), c)
        k = index[(X, Y)]
        terms[k] = terms.get(k, 0) + mult
    return st, terms


def verify_relation(curve, points, i1, i2, terms):
    """Uncounted verifier. points: list of (x,y,w,hx). terms: index -> multiplicity."""
    p = curve.p
    a1, a3 = curve.a1, curve.a3
    x1, y1 = points[i1][0], points[i1][1]
    x2, y2 = points[i2][0], points[i2][1]
    ok_a = True
    for k in terms:
        x, y = points[k][0], points[k][1]
        if (pow(y, 4, p) + curve.h(x) * y * y - curve.f(x)) % p != 0:
            ok_a = False
    caseV = (x1 == x2)
    ok_b = True
    if caseV:
        for k in terms:
            if points[k][0] != x1:
                ok_b = False
        hx = curve.h(x1)
        fx = curve.f(x1)
        S = [(-fx) % p, 0, hx, 0, 1]
        zs = {k: points[k][1] for k in terms}
        m0 = False
    else:
        m = (y2 - y1) * u_inv((x2 - x1) % p, p) % p
        c = (y1 - m * x1) % p
        for k in terms:
            x, y = points[k][0], points[k][1]
            if (y - m * x - c) % p != 0:
                ok_b = False
        L = [c, m]
        L2 = u_pmul(L, L, p)
        L4 = u_pmul(L2, L2, p)
        HL2 = u_pmul([a3, a1], L2, p)
        base = [curve.a6, curve.a4, curve.a2, 1, 0]
        S = [((L4[k] if k < len(L4) else 0) + (HL2[k] if k < len(HL2) else 0) - base[k]) % p
             for k in range(5)]
        zs = {k: points[k][0] for k in terms}
        m0 = (m == 0)
    total = sum(terms.values()) + (1 if (not caseV and m0) else 0)
    ok_c = (total == 4)
    # strip trailing zeros of S
    while len(S) > 1 and S[-1] == 0:
        S.pop()
    lead = S[-1]
    prod = [lead % p]
    for k, mult in terms.items():
        for _ in range(mult):
            prod = u_pmul(prod, [(-zs[k]) % p, 1], p)
    ok_d = (prod == S)
    # push-forward
    from curve import free_field
    FF = free_field(p)
    acc = None
    for k, mult in terms.items():
        Pk = (points[k][0], points[k][2])
        acc = eadd(FF, curve, acc, emul(FF, curve, Pk, mult))
    g3 = (acc is None)
    g2 = ok_a and ok_b and ok_c and ok_d
    return {"G2": bool(g2), "G3": bool(g3)}


def make_row(terms, col, sign):
    acc = {}
    for i, mult in terms.items():
        j = col[i]
        acc[j] = acc.get(j, 0) + sign[i] * mult
    return tuple(sorted((j, v) for j, v in acc.items() if v != 0))
