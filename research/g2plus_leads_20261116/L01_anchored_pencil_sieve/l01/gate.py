import math
from .rng import stream
from .instance import (random_f, squarefree, chitab_plain, point_counts, jac_order,
                       split_order, Tables)
from .curve import Curve, IDENT
from .pencil import p_full, GateFailure
from .baseline import b_labelled
from .linalg import solve_mod_r


def _roots_bruteforce(p, u, v):
    pts = []
    for x in range(p):
        val = 0
        for c in reversed(u):
            val = (val * x + c) % p
        if val == 0:
            y = 0
            for c in reversed(v):
                y = (y * x + c) % p
            pts.append((x, y))
    return pts


def run_gate(g, p, budget_check=None):
    chi = chitab_plain(p)
    rec = {"schema": "L01.gate/v1", "g": g, "p": p, "status": "ok", "pass": False}
    sel = None
    for s in range(1, 401):
        f = random_f(p, g, s)
        if not squarefree(f, p):
            continue
        N = point_counts(f, p, g, chi)
        J = jac_order(N, p, g)
        so = split_order(J)
        if so is None:
            continue
        sel = (s, f, N, J, so[0], so[1])
        break
    if sel is None:
        rec["status"] = "no_curve"
        return rec
    cs, f, N, J, h, r = sel
    rec.update({"curve_seed": cs, "seeds_tried": cs, "N": N, "J": J, "h": h, "r": r})
    T = Tables(p, g, f)
    curve = Curve(p, g, f, T)
    rng = stream(0x4754, g, p, cs, 0)
    ok = True
    for _ in range(16):
        D = curve.random_divisor(rng)
        if curve.mul_ltr(D, J) != IDENT:
            ok = False
    rec["order_check_ok"] = ok
    while True:
        D0 = curve.random_divisor(rng)
        G = curve.mul_ltr(D0, h)
        if G != IDENT:
            break
    assert curve.mul_ltr(G, r) == IDENT
    t = 1 + rng.below(r - 1)
    Tt = curve.mul_ltr(G, t)
    rec["t_planted"] = t
    n = T.n
    rec["n_cols"] = n
    # BSGS logs of all base points
    m = math.isqrt(r) + 1
    table = {}
    cur = IDENT
    for j in range(m):
        table.setdefault(cur, j)
        cur = curve.cantor(cur, G)
    step = curve.negate(cur)
    Ltrue = []
    for x in T.B:
        P = curve.point(x, T.sq[T.fx[x]])
        Q = curve.mul_ltr(P, h) if h > 1 else P
        found = None
        for i in range(m + 1):
            if Q in table:
                found = (i * m + table[Q]) % r
                break
            Q = curve.cantor(Q, step)
        if found is None:
            rec["status"] = "bsgs_fail"
            return rec
        Ltrue.append(found)
        if budget_check:
            budget_check()
    # pencil rows
    vc = Curve(p, g, f, T)
    pf = p_full(T, g, cs, 1, 3 * n, vc, budget_check=budget_check, max_perms=50)
    rows = pf["acc"].rows
    rec["pencil_rows"] = len(rows)
    rec["pf_status"] = pf["status"]
    bad = sum(1 for row in rows if sum(c * Ltrue[j] for j, c in row) % r != 0)
    rec["pencil_rows_bad"] = bad
    nrel, lbad = b_labelled(T, g, vc, G, Tt, t, r, h, Ltrue, rng, 50, budget_check)
    rec["labelled_rels"] = nrel
    rec["labelled_rels_bad"] = lbad
    # descents
    drows = []
    tries = 0

    def accept(D):
        u, v = list(D[0]), list(D[1])
        if len(u) - 1 != g:
            return None
        pts = _roots_bruteforce(p, u, v)
        if len(pts) != g or any(y == 0 for _, y in pts):
            return None
        row = {}
        half = (p - 1) // 2
        for x, y in pts:
            row[T.col[x]] = row.get(T.col[x], 0) + (1 if 1 <= y <= half else -1)
        return row

    while len(drows) < 3:
        tries += 1
        assert tries < 200000
        a = 1 + rng.below(r - 1)
        row = accept(curve.mul_ltr(G, a))
        if row is not None:
            drows.append((row, 0, h * a % r))
    trows = []
    while len(trows) < 3:
        tries += 1
        assert tries < 400000
        a = 1 + rng.below(r - 1)
        b = 1 + rng.below(r - 1)
        D = curve.cantor(curve.mul_ltr(G, a), curve.mul_ltr(Tt, b))
        row = accept(D)
        if row is not None:
            trows.append((row, (-h * b) % r, h * a % r))
    rec["descent_rows"] = len(drows) + len(trows)
    nun = n + 1
    sys_rows = []
    for row in rows:
        c = [0] * nun
        for j, v in row:
            c[j] = v % r
        sys_rows.append((c, 0))
    # hom rank
    _, pv0, _ = solve_mod_r(sys_rows, nun, r)
    rec["hom_rank"] = len(pv0)
    for row, tc, rhs in drows + trows:
        c = [0] * nun
        for j, v in row.items():
            c[j] = v % r
        c[n] = tc
        sys_rows.append((c, rhs))
    cons, piv, M = solve_mod_r(sys_rows, nun, r)
    det = {}
    for col, ri in piv:
        if all(M[ri][c] % r == 0 for c in range(nun) if c != col):
            det[col] = M[ri][nun] % r
    rec["L_determined"] = sum(1 for c in det if c < n)
    rec["L_mismatch"] = sum(1 for c, v in det.items() if c < n and v != Ltrue[c])
    rec["t_recovered"] = det.get(n)
    passed = (cons and ok and bad == 0 and lbad == 0 and det.get(n) == t
              and rec["L_mismatch"] == 0 and nrel == 50)
    rec["consistent"] = cons
    rec["pass"] = bool(passed)
    return rec
