"""Fast interpolation degree on product grids Z^m (Z = d distinct abscissas), for the hypothesis test.
[pcc:unverified]

Reduced interpolant: h = sum_a C[a] prod_l x_l^{a_l}, a in [0,d)^m, C = h x_1 Vinv x_2 Vinv ... (mode products).
Degree of the monomial x^a in the presentation:
   uni   (base f = prod (x - r), no aux):  w(a_l) = a_l            max = m(d-1)
   tower (Z = g mu_{2^k}, digit chain):     w(a_l) = popcount(a_l) max = m k
(For the tower the standard monomial x^{a0} t1^{a1} ... restricts to x^{sum a_j 2^j}; its degree is the
 digit sum.  Both presentations are Groebner bases for any degree order, so delta = max weight of the
 support of C; with freedom on a set V of grid points, delta(h;V) = least t such that the weight->t part
 of C(h) lies in the span of the weight->t parts of C(e_v), v in V.)
"""
import numpy as np
import flint

def mat_inv_mod(Vm, p):
    d = len(Vm)
    A = flint.nmod_mat(d, d, p)
    for i in range(d):
        for j in range(d):
            A[i, j] = int(Vm[i][j])
    Ai = A.inv()
    return np.array([[int(Ai[i, j]) for j in range(d)] for i in range(d)], dtype=np.int64)

def vandermonde(roots, p, ncols=None):
    d = len(roots); ncols = ncols or d
    return np.array([[pow(int(r), j, p) for j in range(ncols)] for r in roots], dtype=np.int64)

def mode_product(T, Mx, axis, p):
    """T[..., i_axis, ...] -> sum_i Mx[j, i] T[..., i, ...] (mod p)."""
    T = np.moveaxis(T, axis, 0)
    sh = T.shape
    R = (Mx @ T.reshape(sh[0], -1)) % p
    R = R.reshape((Mx.shape[0],) + sh[1:])
    return np.moveaxis(R, 0, axis)

def coeffs(hgrid, Vinv, p):
    C = hgrid % p
    for ax in range(C.ndim):
        C = mode_product(C, Vinv, ax, p)
    return C

def weight_grid(d, m, kind):
    if kind == "uni":
        w1 = np.arange(d)
    elif kind == "tower":
        w1 = np.array([bin(i).count("1") for i in range(d)])
    else:
        raise ValueError(kind)
    W = np.zeros((d,) * m, dtype=np.int64)
    for ax in range(m):
        sh = [1] * m; sh[ax] = d
        W = W + w1.reshape(sh)
    return W

def delta_of(C, Wt):
    nz = C != 0
    return int(Wt[nz].max()) if nz.any() else -1

def delta_free(C, Vinv, Vidx, Wt, p):
    """least t such that C|_{wt>t} in span{C(e_v)|_{wt>t} : v in Vidx}; Vidx = list of grid index tuples."""
    if not Vidx:
        return delta_of(C, Wt)
    m = C.ndim
    # C(e_v)[a] = prod_l Vinv[a_l, v_l]
    cols = []
    for v in Vidx:
        T = np.ones((1,) * m, dtype=np.int64)
        for ax in range(m):
            sh = [1] * m; sh[ax] = Vinv.shape[0]
            T = (T * Vinv[:, v[ax]].reshape(sh)) % p
        cols.append(T)
    tmax = int(Wt.max())
    best = None
    for t in range(tmax, -2, -1):
        mask = Wt > t
        if not mask.any():
            best = t; continue
        vecs = [c[mask] for c in cols]
        hv = C[mask]
        nrow = int(mask.sum())
        A = flint.nmod_mat(nrow, len(vecs), p)
        Ah = flint.nmod_mat(nrow, len(vecs) + 1, p)
        for j, v in enumerate(vecs):
            for i in np.nonzero(v)[0]:
                A[int(i), j] = int(v[i]); Ah[int(i), j] = int(v[i])
        for i in np.nonzero(hv)[0]:
            Ah[int(i), len(vecs)] = int(hv[i])
        if A.rank() == Ah.rank():
            best = t
        else:
            break
    return best

def semaev_grid(SR, m, roots, p):
    """values of S_R(r_{i1},...,r_{im}) on the grid via mode products of the coefficient tensor."""
    e_max = max(max(e) for e in SR)
    T = np.zeros((e_max + 1,) * m, dtype=np.int64)
    for e, c in SR.items():
        T[e] = c % p
    P = vandermonde(roots, p, ncols=e_max + 1)          # d x (e_max+1)
    G = T
    for ax in range(m):
        G = mode_product(G, P, ax, p)
    return G

def inv_mod_array(A, p):
    """elementwise inverse mod p of nonzero entries (zeros stay 0), vectorised Fermat (p < 2^31)."""
    base = A % p
    res = np.ones_like(base)
    e = p - 2
    while e:
        if e & 1: res = (res * base) % p
        base = (base * base) % p
        e >>= 1
    res[A % p == 0] = 0
    return res

def _rank(rows_np, p):
    """rank over F_p of an int64 numpy 2-D array."""
    r, c = rows_np.shape
    if r == 0 or c == 0: return 0
    A = flint.nmod_mat(r, c, p)
    nz = np.nonzero(rows_np % p)
    for i, j in zip(nz[0], nz[1]):
        A[int(i), int(j)] = int(rows_np[i, j] % p)
    return A.rank()

def mobius_mats(s, p):
    """Moebius (values -> multilinear coefficients) and zeta (coefficients -> values) on {0,1}^s,
    indices are integers 0..2^s-1 read as bit sets."""
    n = 2 ** s
    Z = np.zeros((n, n), dtype=np.int64)
    Mb = np.zeros((n, n), dtype=np.int64)
    for i in range(n):
        for j in range(n):
            if j & ~i == 0:          # j subset of i
                Z[i, j] = 1
                Mb[i, j] = (-1) ** (bin(i ^ j).count("1")) % p
    return Mb, Z

def delta_weak(invS, roots, Vinv, Vidx, Wt, p, dS, kind, Vand=None):
    """least t such that some u(x_1) (a function of block 1, nonzero on the base, vanishing on pi_1(V)),
    of presentation degree <= dS + t, and some phi supported on V give C(u*invS + phi)|_{wt>t} = 0.
    invS must be 0 on V.  Returns (t, scanned_down_to)."""
    m = invS.ndim; d = len(roots)
    Hp = invS % p
    for ax in range(1, m):
        Hp = mode_product(Hp, Vinv, ax, p)
    if Vand is None:
        Vand = vandermonde(roots, p)                  # Vand[i, j] = r_i^j  (value of basis fn j at point i)
    w1 = np.arange(d) if kind == "uni" else np.array([bin(j).count("1") for j in range(d)])
    piV = sorted({v[0] for v in Vidx})
    free = []
    for v in Vidx:
        T = np.ones((1,) * m, dtype=np.int64)
        for ax in range(m):
            sh = [1] * m; sh[ax] = d
            T = (T * Vinv[:, v[ax]].reshape(sh)) % p
        free.append(T)
    tmax = int(Wt.max())
    best = None
    for t in range(tmax, -2, -1):
        jcols = np.nonzero(w1 <= dS + t)[0]
        mask = Wt > t
        idx = np.nonzero(mask)
        R = len(idx[0])
        if R:
            a0 = idx[0]
            rest = idx[1:]
            Hsel = Hp[(slice(None),) + tuple(rest)] if m > 1 else Hp[:, None]   # d x R
            G = (Vinv[a0, :] * Hsel.T) % p                                      # R x d
            Mu = (G @ Vand[:, jcols]) % p                                       # R x |jcols|
        else:
            Mu = np.zeros((0, len(jcols)), dtype=np.int64)
        Cons = np.array([[pow(int(roots[i0]), int(j), p) for j in jcols] for i0 in piV], dtype=np.int64).reshape(len(piV), len(jcols))
        if free:
            Fv = np.stack([f[mask] for f in free], axis=1) if R else np.zeros((0, len(free)), dtype=np.int64)
            top = np.concatenate([Mu, Fv], axis=1)
            bot = np.concatenate([Cons, np.zeros((len(piV), len(free)), dtype=np.int64)], axis=1)
            A = np.concatenate([top, bot], axis=0)
            rF = _rank(Fv, p)
        else:
            A = np.concatenate([Mu, Cons], axis=0)
            rF = 0
        # u-part must be nonzero on the base: u in span of x^j (j in jcols, j < d) is nonzero as a function
        feasible = _rank(A, p) < rF + len(jcols)
        if feasible:
            best = t
        else:
            break
    return best
