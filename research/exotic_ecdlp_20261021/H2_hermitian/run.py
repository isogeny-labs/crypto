# H2: Hermitian-surface line relations pushed to a Legendre fiber: rank + spectrum (C2).
# Plain-Python Sage program (from sage.all import * is auto-prepended by the harness).
# Use ** for power. Builtins sum/max/min/abs/all/any are accessed through `B` to avoid
# Sage's shadowing of them.
import os, json, time, traceback, collections
import builtins as B
import numpy as np
import scipy.sparse as sp
import scipy.sparse.linalg as spla

try:
    set_random_seed(20261022)
    HAVE_SAGE = True
except NameError:  # local dry-run without Sage
    HAVE_SAGE = False
RNG = np.random.RandomState(20261022)

OUT = os.environ.get("ISOLAB_OUTPUT_DIR", ".")
PS = [int(x) for x in os.environ.get("H2_PS","7,11,13,31").split(",")]
HEAVY = {31}  # line enumeration (~9.5e5 lines x 962 pts) infeasible in the memory/time budget
T0 = time.time()
BIGP = 2147483659  # prime > 2^31, generic modulus standing in for "rank over Q"


def log(*a):
    print("[%7.1fs]" % (time.time() - T0), *a, flush=True)


def jsonable(o):
    if isinstance(o, dict):
        return {str(k): jsonable(v) for k, v in o.items()}
    if isinstance(o, (list, tuple)):
        return [jsonable(v) for v in o]
    if isinstance(o, (np.integer,)):
        return int(o)
    if isinstance(o, (np.floating,)):
        return float(o)
    if isinstance(o, (np.bool_,)):
        return bool(o)
    if isinstance(o, np.ndarray):
        return jsonable(o.tolist())
    if o is None or isinstance(o, (bool, int, float, str)):
        return o
    nm = type(o).__name__
    try:
        if "Integer" in nm:
            return int(o)
        return float(o)
    except Exception:
        return str(o)


def dump_json(obj, path):
    tmp = path + ".tmp"
    with open(tmp, "w") as f:
        json.dump(jsonable(obj), f, indent=1)
    os.replace(tmp, path)


# ----------------------------------------------------------------- rank helpers
def rank_gram_Q(Mat):
    """rank over Q of integer matrix Mat (rows x c), via exact Gram matrix Mat^T Mat (c x c)."""
    Mat = np.asarray(Mat, dtype=np.int64)
    G = Mat.T @ Mat
    if HAVE_SAGE:
        return int(matrix(ZZ, G.tolist()).rank())
    return int(np.linalg.matrix_rank(G.astype(float)))


def rank_mod(Mat, modulus):
    Mat = np.asarray(Mat, dtype=np.int64)
    if Mat.shape[0] == 0:
        return 0
    Mat = np.unique(Mat % modulus, axis=0)
    if HAVE_SAGE:
        return int(matrix(GF(modulus), Mat.tolist()).rank())
    # naive fallback (small only)
    A = Mat.copy() % modulus
    r = 0
    for c in range(A.shape[1]):
        piv = None
        for i in range(r, A.shape[0]):
            if A[i, c] % modulus:
                piv = i
                break
        if piv is None:
            continue
        A[[r, piv]] = A[[piv, r]]
        A[r] = (A[r] * B.pow(int(A[r, c]), -1, modulus)) % modulus
        for i in range(A.shape[0]):
            if i != r and A[i, c]:
                A[i] = (A[i] - A[i, c] * A[r]) % modulus
        r += 1
    return r


def clusters(vals, rel=1e-6, floor=1e-7):
    vals = sorted([float(v) for v in vals if v > floor], reverse=True)
    out = []
    for v in vals:
        if out and B.abs(out[-1][0] - v) <= rel * B.max(1.0, B.abs(out[-1][0])):
            out[-1][1] += 1
        else:
            out.append([v, 1])
    return out


# ----------------------------------------------------------------- field F_{p^2}
class Fq2:
    """F_{p^2} = F_p[t]/(t^2 - n); element a + b t has index a + p*b."""

    def __init__(self, p):
        self.p = p
        q = p * p
        self.q = q
        n = next(c for c in range(2, p) if B.pow(c, (p - 1) // 2, p) == p - 1)
        self.n = n
        idx = np.arange(q)
        a = idx % p
        b = idx // p
        self.add = (((a[:, None] + a[None, :]) % p) + p * ((b[:, None] + b[None, :]) % p)).astype(np.int32)
        mA = (a[:, None] * a[None, :] + n * b[:, None] * b[None, :]) % p
        mB = (a[:, None] * b[None, :] + b[:, None] * a[None, :]) % p
        self.mul = (mA + p * mB).astype(np.int32)
        self.neg = (((-a) % p) + p * ((-b) % p)).astype(np.int32)
        self.frob = (a + p * ((-b) % p)).astype(np.int32)  # x -> x^p
        self.norm = ((a * a - n * b * b) % p).astype(np.int32)  # N(x) in F_p (as int; F_p subset of F_q has same index)
        inv = np.zeros(q, dtype=np.int32)
        for x in range(1, q):
            Nx = int(self.norm[x])
            Ni = B.pow(Nx, -1, p)
            inv[x] = (a[x] * Ni % p) + p * ((-b[x] * Ni) % p)
        self.inv = inv
        self.sub = self.add[:, self.neg]  # sub[x, y] = x - y
        sq = np.zeros(q, dtype=bool)
        sq[self.mul[idx, idx]] = True
        self.is_sq = sq  # squares incl 0
        # primitive element
        self.g = self._primitive()
        # python-list copies for scalar loops
        self.mulL = self.mul.tolist()
        self.addL = self.add.tolist()
        self.invL = self.inv.tolist()
        self.negL = self.neg.tolist()
        self.frobL = self.frob.tolist()

    def _primitive(self):
        q = self.q
        fac = [r for r in range(2, q) if (q - 1) % r == 0 and B.all(r % s for s in range(2, int(r ** 0.5) + 1))]
        for g in range(2, q):
            ok = True
            for r in fac:
                e = (q - 1) // r
                x, base, ee = 1, g, e
                while ee:
                    if ee & 1:
                        x = int(self.mul[x, base])
                    base = int(self.mul[base, base])
                    ee >>= 1
                if x == 1:
                    ok = False
                    break
            if ok:
                return g
        raise RuntimeError("no primitive element")

    def power(self, x, e):
        r, base = 1, x
        while e:
            if e & 1:
                r = int(self.mul[r, base])
            base = int(self.mul[base, base])
            e >>= 1
        return r


# ----------------------------------------------------------------- H geometry
def count_H_points_bruteforce(F):
    """count projective points of P^3(F_q) with sum N(x_i) = 0 (mod p); returns count."""
    p, q = F.p, F.q
    nm = F.norm.astype(np.int64)
    tot = 0
    allx = np.arange(q)
    for lead in range(4):
        free = 3 - lead
        if free == 0:
            ns = np.array([0])
            cnt = 1 if (1 % p) == 0 else 0
            tot += cnt
            continue
        grids = np.meshgrid(*([allx] * free), indexing="ij")
        s = np.ones(grids[0].shape, dtype=np.int64)  # leading coordinate 1, norm 1
        for gr in grids:
            s = s + nm[gr]
        tot += int(np.count_nonzero(s % p == 0))
    return tot


def all_H_points_array(F):
    p, q = F.p, F.q
    nm = F.norm.astype(np.int64)
    allx = np.arange(q)
    pts = []
    for lead in range(4):
        free = 3 - lead
        if free == 0:
            continue
        grids = np.meshgrid(*([allx] * free), indexing="ij")
        grids = [gg.ravel() for gg in grids]
        s = np.ones(grids[0].shape, dtype=np.int64)
        for gg in grids:
            s = s + nm[gg]
        keep = (s % p) == 0
        K = int(np.count_nonzero(keep))
        arr = np.zeros((K, 4), dtype=np.int32)
        arr[:, lead] = 1
        for j, gg in enumerate(grids):
            arr[:, lead + 1 + j] = gg[keep]
        pts.append(arr)
    return np.vstack(pts)


def rref2(F, v1, v2):
    mulL, addL, invL, negL = F.mulL, F.addL, F.invL, F.negL
    r0, r1 = list(v1), list(v2)
    # first pivot
    c1 = None
    for c in range(4):
        if r0[c] or r1[c]:
            c1 = c
            break
    if c1 is None:
        return None
    if not r0[c1]:
        r0, r1 = r1, r0
    iv = invL[r0[c1]]
    r0 = [mulL[iv][x] for x in r0]
    f = r1[c1]
    if f:
        nf = negL[f]
        r1 = [addL[r1[j]][mulL[nf][r0[j]]] for j in range(4)]
    c2 = None
    for c in range(4):
        if r1[c]:
            c2 = c
            break
    if c2 is None:
        return None
    iv = invL[r1[c2]]
    r1 = [mulL[iv][x] for x in r1]
    f = r0[c2]
    if f:
        nf = negL[f]
        r0 = [addL[r0[j]][mulL[nf][r1[j]]] for j in range(4)]
    return tuple(r0 + r1)


def apply_g(F, g, v):
    mulL, addL = F.mulL, F.addL
    out = []
    for i in range(4):
        s = 0
        for j in range(4):
            s = addL[s][mulL[g[i][j]][v[j]]]
        out.append(s)
    return out


def unitary_generators(F, nrot=3):
    p, q = F.p, F.q
    mul, add, frob, norm = F.mul, F.add, F.frob, F.norm
    gens = []

    def perm(i, j):
        m = [[1 if r == c else 0 for c in range(4)] for r in range(4)]
        m[i][i] = m[j][j] = 0
        m[i][j] = m[j][i] = 1
        return m

    gens += [perm(0, 1), perm(1, 2), perm(2, 3)]
    omega = F.power(F.g, p - 1)  # norm 1 element of order p+1
    m = [[1 if r == c else 0 for c in range(4)] for r in range(4)]
    m[0][0] = omega
    gens.append(m)
    # rotations [[a, -b^p],[b, a^p]] with N(a)+N(b)=1, a,b != 0
    cand = [(a, b) for a in range(1, q) for b in range(1, q) if (int(norm[a]) + int(norm[b])) % p == 1]
    for k in RNG.choice(len(cand), size=nrot, replace=False):
        a, b = cand[int(k)]
        m = [[1 if r == c else 0 for c in range(4)] for r in range(4)]
        m[0][0] = a
        m[0][1] = int(F.neg[frob[b]])
        m[1][0] = b
        m[1][1] = int(frob[a])
        gens.append(m)
    return gens


def check_unitary(F, g):
    # g^* g = I with g^*_{ij} = frob(g_{ji})
    p = F.p
    for i in range(4):
        for j in range(4):
            s = 0
            for k in range(4):
                s = int(F.add[s, F.mul[F.frob[g[k][i]], g[k][j]]])
            if s != (1 if i == j else 0):
                return False
    return True


def enumerate_lines(F):
    p, q = F.p, F.q
    # initial totally isotropic line: u=(1,alpha,0,0), w=(0,0,1,beta), N(alpha)=N(beta)=-1
    alpha = next(x for x in range(1, q) if (int(F.norm[x]) + 1) % p == 0)
    u = [1, alpha, 0, 0]
    w = [0, 0, 1, alpha]
    start = rref2(F, u, w)
    gens = unitary_generators(F)
    assert B.all(check_unitary(F, g) for g in gens), "non-unitary generator"
    seen = {start}
    dq = collections.deque([start])
    while dq:
        key = dq.popleft()
        v1, v2 = list(key[:4]), list(key[4:])
        for g in gens:
            k2 = rref2(F, apply_g(F, g, v1), apply_g(F, g, v2))
            if k2 not in seen:
                seen.add(k2)
                dq.append(k2)
    arr = np.array(sorted(seen), dtype=np.int32)
    return arr[:, :4].copy(), arr[:, 4:].copy()


def line_points_idx(F, U, W):
    """for every line, the q+1 normalized point keys; also verifies each lies on H."""
    p, q = F.p, F.q
    L = U.shape[0]
    keys = np.empty((L, q + 1), dtype=np.int64)
    mul, add, inv = F.mul, F.add, F.inv
    nm = F.norm.astype(np.int64)
    ok = True
    for t in range(q + 1):
        if t < q:
            # point u + t*w
            tw = mul[t, W]
            P = add[U, tw]
        else:
            P = W
        s = nm[P].sum(axis=1) % p
        if np.any(s != 0):
            ok = False
        # normalize first nonzero coord to 1
        nz = P != 0
        first = np.argmax(nz, axis=1)
        lead = P[np.arange(L), first]
        iv = inv[lead]
        Pn = mul[iv[:, None], P]
        keys[:, t] = ((Pn[:, 0].astype(np.int64) * q + Pn[:, 1]) * q + Pn[:, 2]) * q + Pn[:, 3]
    return keys, ok


def legendre_columns(F, lam):
    """columns = <+-1> classes of E_lam(F_q): x with g(x) square (incl 0) + the point O at infinity."""
    q = F.q
    mul, sub = F.mul, F.sub
    x = np.arange(q)
    xm1 = sub[x, 1]
    xml = sub[x, lam]
    g = mul[mul[x, xm1], xml]
    valid = F.is_sq[g]
    colmap = -np.ones(q + 1, dtype=np.int64)  # index q = infinity
    xs = np.nonzero(valid)[0]
    colmap[xs] = np.arange(len(xs))
    colmap[q] = len(xs)
    # #E(F_q) = q + 1 + sum chi(g(x))
    chi = np.where(g == 0, 0, np.where(F.is_sq[g], 1, -1))
    nE = int(q + 1 + chi.sum())
    return colmap, len(xs) + 1, nE


def fibre_rows(F, U, W, lam, zetas, surrogate):
    """M (dense int32): lines x columns; also dropped statistics."""
    p, q = F.p, F.q
    mul, add, sub, neg, frob, inv = F.mul, F.add, F.sub, F.neg, F.frob, F.inv
    colmap, ncols, nE = legendre_columns(F, lam)
    L = U.shape[0]
    rows, cols = [], []
    drop_axis = 0
    drop_nonsq = 0
    for z in zetas:
        c = sub[U[:, 0], mul[z, U[:, 3]]]  # u0 - z u3
        d = sub[W[:, 0], mul[z, W[:, 3]]]  # w0 - z w3
        assert not np.any((c == 0) & (d == 0)), "line contained in fibre plane (lambda=-1?)"
        s = neg[d]
        t = c
        # point = s*u + t*w
        P = add[mul[s[:, None], U], mul[t[:, None], W]]
        x1, x2, x3 = P[:, 1], P[:, 2], P[:, 3]
        axis = x3 == 0
        drop_axis += int(axis.sum())
        x3s = np.where(axis, 1, x3)
        if surrogate == "A":  # X = x1/x2 (Moebius / projection from the line x1=x2=0), inf if x2=0
            X = np.where(x2 == 0, q, mul[x1, inv[x2]])
        else:  # "B": X = x1 * x2^p / x3^(p+1)
            n3 = mul[x3s, frob[x3s]]
            X = mul[mul[x1, frob[x2]], inv[n3]]
        col = colmap[X]
        good = (~axis) & (col >= 0)
        drop_nonsq += int(((~axis) & (col < 0)).sum())
        rows.append(np.nonzero(good)[0])
        cols.append(col[good])
    rows = np.concatenate(rows)
    cols = np.concatenate(cols)
    Mcsr = sp.coo_matrix((np.ones(len(rows), dtype=np.int32), (rows, cols)), shape=(L, ncols)).tocsr()
    return Mcsr, ncols, nE, drop_axis, drop_nonsq


def spectrum_of_gram(G):
    ev = np.linalg.eigvalsh(G.astype(np.float64))
    return np.sort(ev)[::-1]


def srg_check_incidence(F, keys_idx, L, npts):
    """exact check that NN^T has spectrum {(p+1)(p^2+1), p^2+p, 0} (GQ(p^2,p) incidence)."""
    p = F.p
    q = p * p
    rows = np.repeat(np.arange(L), q + 1)
    N = sp.csr_matrix((np.ones(L * (q + 1), dtype=np.int64), (rows, keys_idx.ravel())), shape=(L, npts))
    A = (N @ N.T).tocsr()
    th1 = (p + 1) * (p * p + 1)
    th2 = p * p + p
    diag_ok = bool(np.all(A.diagonal() == p * p + 1))
    off = A - sp.diags(A.diagonal())
    off_vals = np.unique(off.data)
    off_ok = bool(set(off_vals.tolist()) <= {1})
    deg_ok = bool(np.all(np.asarray(off.sum(axis=1)).ravel() == p * (p * p + 1)))
    # polynomial annihilator A (A - th2)(A - th1) on random vectors
    resid = 0.0
    for _ in range(4):
        v = RNG.randint(-1, 2, size=L).astype(np.int64)
        y = A @ v - th1 * v
        y = A @ y - th2 * y
        y = A @ y
        resid = B.max(resid, float(np.abs(y).max()) / B.max(1.0, float(np.abs(v).max())))
    trA = int(A.diagonal().sum())
    m2 = (trA - th1) // th2
    m2_int = ((trA - th1) % th2) == 0
    trA2 = int(A.multiply(A).sum())
    tr2_pred = th1 ** 2 + m2 * th2 ** 2
    # numeric top eigenvalues (Lanczos)
    try:
        ev = spla.eigsh(A.astype(np.float64), k=B.min(6, L - 2), which="LA", return_eigenvectors=False)
        top = sorted([float(x) for x in ev], reverse=True)
    except Exception as e:
        top = ["eigsh failed: %r" % (e,)]
    full = None
    if L <= 3500:
        evf = np.sort(np.linalg.eigvalsh(A.toarray().astype(np.float64)))[::-1]
        full = [[round(v, 4), m] for v, m in clusters(evf, rel=1e-8, floor=-1e18)]
        # clusters() floors at floor; include zeros
    return {
        "diag_all_p2_plus_1": diag_ok, "offdiag_in_{0,1}": off_ok, "line_degree_p(p^2+1)": deg_ok,
        "annihilator_residual_inf_norm": resid,
        "trace_A": trA, "mult_p2_plus_p_from_trace": int(m2), "mult_integral": bool(m2_int),
        "trace_A2": trA2, "trace_A2_predicted": int(tr2_pred),
        "trace_A2_matches": bool(trA2 == tr2_pred),
        "rank_NNt_predicted": int(1 + m2),
        "top_eigenvalues_lanczos": top, "full_spectrum_clusters_dense": full,
        "predicted_nonzero_eigenvalues": [th1, th2],
    }, N


def run_p(p):
    cell = {"p": p}
    t0 = time.time()
    q = p * p
    npts_expected = (p ** 3 + 1) * (p ** 2 + 1)
    nlines_expected = (p + 1) * (p ** 3 + 1)
    cell["lines_count_expected"] = (p ** 3 + 1) * (p ** 2 + 1)  # as written in the protocol
    cell["lines_count_expected_correct"] = nlines_expected  # (p+1)(p^3+1): true #lines of H(3,p^2)
    cell["points_count_expected"] = npts_expected
    cell["protocol_line_formula_is_actually_point_count"] = True
    cell["srg_predicted_eigenvalues"] = [(p * p + 1) * (p + 1), p * p - p - 1, -(p + 1)]  # as stated in the task
    cell["gq_NNt_predicted_eigenvalues"] = {"nonzero": [(p + 1) * (p * p + 1), p * p + p], "zero": 0}
    cell["gq_collinearity_adjacency_predicted"] = [p * p * (p + 1), p * p - 1, -(p + 1)]
    cell["gq_line_graph_adjacency_predicted"] = [p * (p * p + 1), p - 1, -(p * p + 1)]
    F = Fq2(p)
    # lambda = N(u0), u0 = primitive element of F_q ; lambda generates F_p^*
    u0 = F.g
    lam = int(F.norm[u0])
    assert lam not in (0, 1, p - 1)
    cell["lambda"] = lam
    cell["u0"] = int(u0)
    colmap, ncols, nE = legendre_columns(F, lam)
    cell["ncols"] = int(ncols)
    cell["E_order_over_Fq"] = nE
    cell["E_lambda"] = "y^2 = x(x-1)(x-%d) over F_{%d}" % (lam, q)
    cell["trace_over_Fq"] = q + 1 - nE
    # cross-check order with Sage when available
    ellfac = None
    if HAVE_SAGE:
        try:
            Fs = GF(q, "a")
            Es = EllipticCurve(Fs, [0, -(1 + lam), 0, lam, 0])
            nEs = int(Es.cardinality())
            cell["E_order_sage"] = nEs
            ellfac = factor(nEs)
            cell["E_order_factorization"] = str(ellfac)
            ell = int(max([f[0] for f in ellfac]))
            cell["ell_largest_prime_factor"] = ell
        except Exception as e:
            cell["E_order_sage_error"] = repr(e)
            ell = 3
    else:
        ell = 3
    ell = int(cell.get("ell_largest_prime_factor", ell))
    val = {}
    val["ncols_equals_E_order_over_2_plus_2"] = bool(ncols == nE // 2 + 2)
    val["E_order_matches_sage"] = bool(cell.get("E_order_sage", nE) == nE)
    # --- independent point count of H by brute force P^3
    if p not in HEAVY:
        hp = count_H_points_bruteforce(F)
        cell["points_count_bruteforce"] = hp
        val["point_count_matches_formula"] = bool(hp == npts_expected)

    if p in HEAVY:
        cell["censored"] = True
        cell["reason"] = "line enumeration infeasible in budget (%d lines x %d pts; M would have ~%.1e nonzeros)" % (
            nlines_expected, q + 1, nlines_expected * (q + 1))
        cell["validation"] = val
        return cell

    # --- lines by orbit of PGU(4,p) on one totally isotropic line
    U, W = enumerate_lines(F)
    L = U.shape[0]
    cell["line_count"] = int(L)
    val["line_count_matches_correct_formula"] = bool(L == nlines_expected)
    val["line_count_matches_protocol_formula"] = bool(L == cell["lines_count_expected"])
    log("p=%d lines=%d expected=%d" % (p, L, nlines_expected))

    # brute-force collinearity check of the line count (p = 7 only)
    if p == 7:
        pts = all_H_points_array(F)
        nP = len(pts)
        mul, add, frob = F.mul, F.add, F.frob
        ordered = 0
        chunk = 400
        pf = frob[pts]
        for s0 in range(0, nP, chunk):
            Pc = pts[s0:s0 + chunk]
            acc = np.zeros((len(Pc), nP), dtype=np.int32)
            for i in range(4):
                acc = add[acc, mul[Pc[:, i][:, None], pf[:, i][None, :]]]
            ordered += int(np.count_nonzero(acc == 0))
        ordered -= nP  # P == Q pairs (isotropic: form(P,P)=0)
        lines_bf = ordered // (q * (q + 1))
        cell["lines_count_bruteforce_pairs"] = int(lines_bf)
        val["line_count_bruteforce_matches"] = bool(lines_bf == L)

    keys, onH = line_points_idx(F, U, W)
    val["all_line_points_on_H"] = bool(onH)
    ukeys, inv = np.unique(keys.ravel(), return_inverse=True)
    npts = len(ukeys)
    pidx = inv.reshape(L, q + 1)
    cell["points_on_lines_union"] = int(npts)
    val["points_union_equals_H_points"] = bool(npts == npts_expected)
    pdeg = np.bincount(pidx.ravel(), minlength=npts)
    val["each_point_on_p_plus_1_lines"] = bool(np.all(pdeg == p + 1))
    del keys
    log("p=%d points=%d" % (p, npts))

    # --- incidence-level SRG / GQ(p^2,p) check
    srg, Ninc = srg_check_incidence(F, pidx, L, npts)
    cell["incidence_gq_check"] = srg
    res_N = float(srg["annihilator_residual_inf_norm"])
    cell["spectrum_residual_vs_srg"] = res_N  # incidence NN^T vs derived GQ(p^2,p) spectrum
    obs_nonzero = srg["predicted_nonzero_eigenvalues"]
    stated = cell["srg_predicted_eigenvalues"]
    cell["spectrum_residual_vs_srg_as_stated_in_protocol"] = float(
        B.max(B.min(B.abs(v - s) for s in stated) for v in obs_nonzero))
    val["incidence_is_GQ_p2_p_spectrum"] = bool(
        res_N < 1e-6 and srg["trace_A2_matches"] and srg["diag_all_p2_plus_1"] and srg["offdiag_in_{0,1}"])
    log("p=%d incidence check resid=%g" % (p, res_N))

    # --- fibre restriction
    zetas = [z for z in range(1, q) if int(F.norm[z]) == lam]
    assert len(zetas) == p + 1
    cell["fibre_planes_zeta_count"] = len(zetas)
    results = {}
    for sur in ("A", "B"):
        Mcsr, ncols_s, nE_s, d_axis, d_nsq = fibre_rows(F, U, W, lam, zetas, sur)
        Mfull = Mcsr.toarray().astype(np.int64)
        hit = np.nonzero(Mfull.sum(axis=0) > 0)[0]
        Mh = Mfull[:, hit]
        nh = Mh.shape[1]
        r = {}
        r["ncols"] = int(ncols_s)
        r["ncols_hit"] = int(nh)
        r["ncols_never_hit"] = int(ncols_s - nh)
        r["entries_dropped_on_axis_x3_eq_0"] = d_axis
        r["entries_dropped_nonsquare_rhs"] = d_nsq
        r["nnz"] = int(np.count_nonzero(Mh))
        r["density"] = float(np.count_nonzero(Mh)) / (L * nh)
        r["row_weight_mean"] = float(Mh.sum(axis=1).mean())
        r["distinct_rows"] = int(len(np.unique(Mh, axis=0)))
        rQ = rank_gram_Q(Mh)
        r["rank_Q"] = rQ
        r["rank_FBIG"] = rank_mod(Mh, BIGP)
        r["rank_Fell"] = rank_mod(Mh, ell)
        r["ell"] = ell
        # Lambda = differences of rows (homogeneous relations)
        r["rank_Q_row_differences"] = rank_gram_Q(Mh - Mh[0:1, :])
        # tautological pencil identities: S_P = sum_{l through P} r_l ; D_P = S_P - S_P0
        S = (Ninc.T.tocsr() @ sp.csr_matrix(Mh)).toarray() if False else (Ninc.T.tocsr() @ Mh)
        S = np.asarray(S, dtype=np.int64)
        D = S - S[0:1, :]
        taut = rank_gram_Q(D)
        r["tautological_dim"] = taut
        r["rank_S_pencil_sums"] = rank_gram_Q(S)
        # D lies in the row-difference lattice by construction; verify
        GG = Mh.T @ Mh + D.T @ D
        rst = int(matrix(ZZ, GG.tolist()).rank()) if HAVE_SAGE else int(np.linalg.matrix_rank(GG.astype(float)))
        r["rank_Q_rows_plus_pencil_differences"] = rst
        r["pencil_differences_inside_rowspace"] = bool(rst == rQ)
        gen_rank = B.min(L, nh)
        r["generic_rank_min_rows_cols"] = gen_rank
        r["nontaut_corank"] = int(gen_rank - rQ)
        r["nontaut_corank_over_ncols_hit"] = float(gen_rank - rQ) / nh
        r["nontaut_rank_beyond_taut"] = int(r["rank_Q_row_differences"] - taut)
        r["nontaut_rank_beyond_taut_generic"] = int(B.max(0, B.min(L - 1, nh - 1) - taut))
        r["corank_all_columns_incl_never_hit"] = int(ncols_s - rQ)
        # spectrum of M M^T (nonzero part) = spectrum of Gram
        G = (Mh.T @ Mh)
        ev = spectrum_of_gram(G)
        r["top_eigenvalues"] = [float(x) for x in ev[:6]]
        r["smallest_eigenvalue_of_gram"] = float(ev[-1])
        cl = clusters(ev)
        r["eigenvalue_cluster_count"] = len(cl)
        r["eigenvalue_clusters_top"] = [[round(v, 6), m] for v, m in cl[:8]]
        # M's gram eigenvalues vs stated/derived GQ sets: expected NOT to match (M = N*Phi, a push-forward)
        ref = [float(cell["gq_NNt_predicted_eigenvalues"]["nonzero"][0]), float(cell["gq_NNt_predicted_eigenvalues"]["nonzero"][1])]
        r["M_gram_eig_mismatch_vs_GQ_NNt_nonzero_set_max"] = float(
            B.max(B.min(B.abs(v - s) for s in ref) for v in ev[:6]))
        # random baselines (10 each): R1 Bernoulli density-matched 0/1 ; R2 rows with p+1 random column draws
        ranks1, ranks2, top1, top2 = [], [], [], []
        dens = r["density"]
        for rep in range(10):
            R1 = (RNG.random_sample((L, nh)) < dens).astype(np.int64)
            ranks1.append(rank_gram_Q(R1))
            top1.append(spectrum_of_gram(R1.T @ R1)[:3].tolist())
            R2 = np.zeros((L, nh), dtype=np.int64)
            draws = RNG.randint(0, nh, size=(L, p + 1))
            for k in range(p + 1):
                np.add.at(R2, (np.arange(L), draws[:, k]), 1)
            ranks2.append(rank_gram_Q(R2))
            top2.append(spectrum_of_gram(R2.T @ R2)[:3].tolist())
        r["random_matrix_rank_mean"] = float(np.mean(ranks1))
        r["random_matrix_rank_sd"] = float(np.std(ranks1))
        r["random_matrix_top_eigs_mean"] = np.mean(np.array(top1), axis=0).tolist()
        r["random_rows_p1_draws_rank_mean"] = float(np.mean(ranks2))
        r["random_rows_p1_draws_rank_sd"] = float(np.std(ranks2))
        r["random_rows_p1_draws_top_eigs_mean"] = np.mean(np.array(top2), axis=0).tolist()
        results[sur] = r
        log("p=%d surrogate %s: ncols_hit=%d rank_Q=%d taut=%d corank=%d rand=%.1f" % (
            p, sur, nh, rQ, taut, r["nontaut_corank"], r["random_matrix_rank_mean"]))

    A = results["A"]
    cell["surrogate_A"] = A
    cell["surrogate_B"] = results["B"]
    # promote primary-surrogate (A) numbers to the cell's top level (spec keys)
    for k in ("ncols", "rank_Q", "tautological_dim", "nontaut_corank", "top_eigenvalues",
              "random_matrix_rank_mean", "random_matrix_rank_sd"):
        cell[k] = A[k]
    cell["nontaut_corank_B"] = results["B"]["nontaut_corank"]
    val["random_baseline_computed"] = True
    cell["validation"] = val
    cell["seconds"] = time.time() - t0
    return cell


def main():
    out = {
        "experiment": "EXP-EXOTIC-H2-001",
        "seed": 20261022,
        "p_values": PS,
        "cells": {},
        "model_note": (
            "SURROGATE. No faithful Hermitian-surface -> Legendre-curve map exists (genus/Kodaira mismatch), so the "
            "norm-circle fibre of PROTOCOL.md is used: for lambda=N(u0) the fibre F_lambda = {N(x0)=lambda N(x3)} on H is the "
            "union of the p+1 planes x0=zeta*x3, N(zeta)=lambda (the norm circle). Each of the (p+1)(p^3+1) lines of H "
            "(GQ(p^2,p)) meets F_lambda in exactly p+1 points (one per plane; coincident when the line meets the axis "
            "x0=x3=0). Surrogate A pushes a fibre point (zeta x3:x1:x2:x3) to the Legendre x-coordinate X=x1/x2 ; "
            "surrogate B to X=x1*x2^p/x3^(p+1). Columns = <+-1>-classes (x-coordinates) of E_lambda(F_{p^2}), "
            "E_lambda: y^2=x(x-1)(x-lambda), plus O; a point is dropped if X is not an x-coordinate of an F_{p^2}-point. "
            "Entries are UNSIGNED multiplicities (the sign of y is not determined by the surrogate). M = (lines x columns). "
            "Tautological relations = span of pencil differences S_P - S_P0, S_P = sum of rows of the p+1 lines through P "
            "(= hyperplane section T_P cap F pushed down; all such have the same class). "
            "nontaut_corank = min(#lines, #hit columns) - rank_Q(M). Columns never hit by the surrogate image are reported "
            "separately (ncols_never_hit) and are not counted as corank. rank_Q = exact rank of Gram M^T M. "
            "Protocol formula (p^3+1)(p^2+1) is the number of POINTS of H; the number of lines is (p+1)(p^3+1). "
            "The stated SRG eigenvalue list is not the spectrum of N N^T; the derived GQ(p^2,p) incidence spectrum "
            "{(p+1)(p^2+1), p^2+p, 0} is used for spectrum_residual_vs_srg, and the as-stated residual is also reported."
        ),
        "validation": {},
    }
    finished = []
    for p in PS:
        try:
            cell = run_p(p)
        except Exception as e:
            cell = {"p": p, "error": repr(e), "traceback": traceback.format_exc()}
            log("p=%d ERROR %r" % (p, e))
        out["cells"][str(p)] = cell
        if "error" not in cell and not cell.get("censored"):
            finished.append(p)
        dump_json(out, os.path.join(OUT, "result.json"))
    # ---------------- aggregate, validation and verdict
    out["finished_p"] = finished
    out["censored_p"] = [p for p in PS if out["cells"][str(p)].get("censored")]
    out["errored_p"] = [p for p in PS if "error" in out["cells"][str(p)]]
    allval = {}
    for p in finished:
        for k, v in out["cells"][str(p)]["validation"].items():
            allval["p%d_%s" % (p, k)] = v
    out["validation"] = allval
    out["validation"]["line_count_matches_correct_formula_all"] = bool(
        finished and B.all(out["cells"][str(p)]["validation"].get("line_count_matches_correct_formula") for p in finished))
    out["validation"]["line_count_matches_protocol_formula_all"] = bool(
        finished and B.all(out["cells"][str(p)]["validation"].get("line_count_matches_protocol_formula") for p in finished))
    out["validation"]["protocol_formula_mismatch_note"] = (
        "(p^3+1)(p^2+1) is #points(H); #lines = (p+1)(p^3+1). Count checked against the correct formula.")
    out["validation"]["random_matrix_baseline_computed"] = bool(
        finished and B.all(out["cells"][str(p)]["validation"].get("random_baseline_computed") for p in finished))
    out["validation"]["incidence_GQ_spectrum_verified_all"] = bool(
        finished and B.all(out["cells"][str(p)]["validation"].get("incidence_is_GQ_p2_p_spectrum") for p in finished))
    inst_ok = bool(finished and B.all(
        v for p in finished for k, v in out["cells"][str(p)]["validation"].items()
        if k in ("line_count_matches_correct_formula", "point_count_matches_formula", "points_union_equals_H_points",
                 "each_point_on_p_plus_1_lines", "all_line_points_on_H", "incidence_is_GQ_p2_p_spectrum",
                 "ncols_equals_E_order_over_2_plus_2", "random_baseline_computed")))
    out["validation"]["instance_validation_all_pass"] = inst_ok

    if finished:
        pmax = B.max(finished)
        cm = out["cells"][str(pmax)]
        out["nontaut_corank"] = cm["nontaut_corank"]
        out["primary_p"] = pmax
        out["spectrum_residual_vs_srg_by_p"] = {str(p): out["cells"][str(p)]["spectrum_residual_vs_srg"] for p in finished}
        coranks = [out["cells"][str(p)]["nontaut_corank"] for p in finished]
        coranksB = [out["cells"][str(p)]["nontaut_corank_B"] for p in finished]
        ratios = [out["cells"][str(p)]["surrogate_A"]["nontaut_corank_over_ncols_hit"] for p in finished]
        resid_ok = B.all(out["cells"][str(p)]["spectrum_residual_vs_srg"] < 1e-6 for p in finished)
        kill = (B.all(c == 0 for c in coranks) and B.all(c == 0 for c in coranksB) and resid_ok and inst_ok)
        grows = (len(ratios) >= 2 and B.all(ratios[i + 1] > ratios[i] for i in range(len(ratios) - 1)) and ratios[-1] > 0)
        win = bool(grows or (not resid_ok and inst_ok))
        out["verdict"] = "kill" if kill else ("win" if win else "inconclusive")
        out["verdict_inputs"] = {
            "coranks_A": coranks, "coranks_B": coranksB, "corank_over_ncols_A": ratios,
            "incidence_spectrum_residual_lt_1e-6_all_p": bool(resid_ok), "instance_validation_all_pass": inst_ok,
            "censored_p": out["censored_p"],
        }
    else:
        out["nontaut_corank"] = None
        out["verdict"] = "inconclusive"
    out["total_seconds"] = time.time() - T0
    dump_json(out, os.path.join(OUT, "result.json"))
    log("done verdict=%s" % out["verdict"])


try:
    main()
except Exception as _e:  # always leave a result.json
    traceback.print_exc()
    with open(os.path.join(OUT, "result.json"), "w") as f:
        json.dump({"verdict": "inconclusive", "nontaut_corank": None, "fatal_error": repr(_e),
                   "traceback": traceback.format_exc()}, f, indent=1)
