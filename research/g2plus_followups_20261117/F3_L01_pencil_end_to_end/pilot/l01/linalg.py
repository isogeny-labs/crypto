from .field import Counter

Q61 = (1 << 61) - 1


def sparse_eliminate(rows, ncols, q=Q61, cap=50_000_000):
    """Counted sparse elimination (SPEC 9.3). rows: list of tuples of (col, coef)."""
    ctr = Counter()
    R = {}
    colrows = {}
    for i, row in enumerate(rows):
        d = {}
        for c, v in row:
            v %= q
            if v:
                d[c] = v
        if d:
            R[i] = d
            for c in d:
                colrows.setdefault(c, set()).add(i)
    uncovered = ncols - len(colrows)
    nnz = sum(len(d) for d in R.values())
    initial = nnz
    peak = nnz
    pivots = 0
    status = "ok"
    while True:
        best = None
        for c, rs in colrows.items():
            if rs:
                key = (len(rs), c)
                if best is None or key < best:
                    best = key
        if best is None:
            break
        c = best[1]
        prow_id = min(colrows[c], key=lambda i: (len(R[i]), i))
        prow = R[prow_id]
        pc = prow[c]
        ctr.inv += 1
        ip = pow(pc, q - 2, q)
        others = sorted(i for i in colrows[c] if i != prow_id)
        items = [(j, val) for j, val in prow.items() if j != c]
        for k in others:
            rk = R[k]
            ctr.mul += 1
            fac = rk[c] * ip % q
            for j, val in items:
                ctr.mul += 1
                t = fac * val % q
                nv = (rk.get(j, 0) - t) % q
                if nv:
                    if j not in rk:
                        colrows[j].add(k)
                    rk[j] = nv
                else:
                    if j in rk:
                        del rk[j]
                        colrows[j].discard(k)
            del rk[c]
            colrows[c].discard(k)
            nnz += 0
        # remove pivot row and column
        for j in prow:
            colrows[j].discard(prow_id)
        del R[prow_id]
        del colrows[c]
        for k in others:
            if not R[k]:
                del R[k]
        nnz = sum(len(d) for d in R.values())
        peak = max(peak, nnz)
        pivots += 1
        colrows = {cc: rs for cc, rs in colrows.items() if rs}
        if ctr.mul > cap:
            status = "capped"
            break
    return {"pivots": pivots, "mul": ctr.mul, "inv": ctr.inv, "initial_nnz": initial,
            "peak_nnz": peak, "uncovered_cols": uncovered, "status": status}


def solve_mod_r(rows, nun, r):
    """Dense Gauss-Jordan mod r. rows: list of (coef list len nun, rhs). Returns
    (consistent, pivots dict col->row index in reduced matrix, reduced rows)."""
    M = [list(c) + [b] for c, b in rows]
    piv = []
    ri = 0
    for col in range(nun):
        sel = None
        for i in range(ri, len(M)):
            if M[i][col] % r:
                sel = i
                break
        if sel is None:
            continue
        M[ri], M[sel] = M[sel], M[ri]
        iv = pow(M[ri][col], r - 2, r)
        M[ri] = [x * iv % r for x in M[ri]]
        for i in range(len(M)):
            if i != ri and M[i][col] % r:
                fct = M[i][col]
                M[i] = [(x - fct * y) % r for x, y in zip(M[i], M[ri])]
        piv.append((col, ri))
        ri += 1
    for i in range(ri, len(M)):
        if M[i][nun] % r:
            return False, piv, M
    return True, piv, M
