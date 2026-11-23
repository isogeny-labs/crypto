"""Validator: own Boolean reduction and own degree-D Macaulay closure on recorded targets.
x uses the explicit Weierstrass S3 (no interpolation); v uses the validator's own
e-basis interpolation.  Targets are the producer's recorded target logs, mapped to
points with the producer's recorded generator (census_meta.json), multiplied by the
validator's own Weierstrass arithmetic."""
import json, math, sys, time
import numpy as np
from vcurve import VCurve, npow
from desk import ebasis, eval_ebasis, echelon, kernel_vector, sample_zeros

LEAD = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L2_highdeg_coords'


def own_S(C, f):
    """returns a callable S(a, b, c) on int64 arrays for f in {x, v}."""
    p = C.p
    if f == 'x':
        a4, a6 = C.a4, C.a6
        def S(x1, x2, x3):
            d = (x1 - x2) % p
            t1 = d * d % p * x3 % p * x3 % p
            t2 = 2 * ((((x1 + x2) % p) * ((x1 * x2 + a4) % p) + 2 * a6) % p) % p * x3 % p
            m = (x1 * x2 - a4) % p
            t3 = m * m % p
            t4 = 4 * a6 % p * ((x1 + x2) % p) % p
            return (t1 - t2 + t3 - t4) % p
        return S
    val, ok = C.fvals(f)
    rng = np.random.default_rng(31337 + C.k)
    for par in (0, 1):
        cols = ebasis(9, par)
        a_, b_, c_ = sample_zeros(C, val, ok, len(cols) + 60, rng)
        M = eval_ebasis(cols, a_, b_, c_, p, 9)
        E, piv = echelon(M, p)
        if len(cols) - len(piv) == 1:
            fc = [c for c in range(len(cols)) if c not in set(piv)][0]
            x = kernel_vector(E, piv, len(cols), fc, p)
            nz = np.flatnonzero(x)
            sel = [cols[i] for i in nz]; co = x[nz]
            def S(a, b, c, sel=sel, co=co):
                a, b, c = np.broadcast_arrays(np.asarray(a, dtype=np.int64), np.asarray(b, dtype=np.int64), np.asarray(c, dtype=np.int64))
                sh = a.shape
                M = eval_ebasis(sel, a.ravel() % p, b.ravel() % p, c.ravel() % p, p, 9)
                return (M @ co % p).reshape(sh) if M.shape[1] * p * p < 2**62 else None
            return S
    raise RuntimeError('no S')


def moebius(vals, n, p):
    h = vals.astype(np.int64) % p
    for i in range(n):
        bit = 1 << i
        idx = np.arange(1 << n)
        hi = idx[(idx & bit) != 0]
        h[hi] = (h[hi] - h[hi ^ bit]) % p
    return h


def popc(m):
    return bin(int(m)).count('1')


class Closure:
    """degree-D Macaulay closure with degree falls in F_p[z]/(z_i^2 - z_i), own code.
    Columns ordered by (-degree, mask). Counting convention: reduction rows x pivots x
    active cols; per RREF pivot (#rows touched) x active cols; back-reduction basis rows x
    new pivots x active cols."""

    def __init__(self, n, p):
        self.n, self.p, self.N = n, p, 1 << n
        masks = sorted(range(self.N), key=lambda m: (-popc(m), m))
        self.col_of = np.empty(self.N, dtype=np.int64); self.col_of[masks] = np.arange(self.N)
        self.mask_of = np.array(masks, dtype=np.int64)
        self.deg = np.array([popc(m) for m in masks])
        self.ndeg = [sum(math.comb(n, i) for i in range(D + 1)) for D in range(n + 1)]
        self.zi = []
        for i in range(n):
            bit = 1 << i
            src_with = self.col_of[[m for m in range(self.N) if m & bit]]
            src_without = self.col_of[[m ^ bit for m in range(self.N) if m & bit]]
            self.zi.append((src_with, src_without))

    def mulz(self, H, i):
        G = np.zeros_like(H)
        a, b = self.zi[i]
        G[:, a] = (H[:, a] + H[:, b]) % self.p
        return G

    def run(self, shat_cols, e, mode, lin_target=None):
        n, p, N = self.n, self.p, self.N
        lin_target = n if lin_target is None else lin_target
        B = np.zeros((0, N), dtype=np.int64); piv = []
        cnt = 0
        hist = []; Dref = None; Droot = None
        # multipliers m * Shat grouped by |m|, generated without repetition
        layers = {0: (shat_cols[None, :] % p, [-1])}
        done_rows = set()

        def absorb(X, lo):
            nonlocal B, piv, cnt
            if X.shape[0] == 0:
                return
            Xa = X[:, lo:] % p
            if piv:
                P = [c - lo for c in piv]
                coef = Xa[:, P]
                Xa = (Xa - self._mm(coef, B[:, lo:])) % p
                cnt += Xa.shape[0] * len(piv) * Xa.shape[1]
            Xa = Xa[np.any(Xa != 0, axis=1)]
            if Xa.shape[0] == 0:
                return
            # own RREF of the residual
            M = Xa.copy(); r = 0; pn = []
            R_, C_ = M.shape
            for c in range(C_):
                if r == R_:
                    break
                nz = np.flatnonzero(M[r:, c])
                if nz.size == 0:
                    continue
                i = r + int(nz[0])
                if i != r:
                    M[[r, i]] = M[[i, r]]
                M[r] = M[r] * pow(int(M[r, c]), p - 2, p) % p
                col = M[:, c].copy(); col[r] = 0
                rows = np.flatnonzero(col)
                if rows.size:
                    M[rows] = (M[rows] - col[rows, None] * M[r][None, :]) % p
                    cnt += rows.size * C_
                pn.append(c + lo); r += 1
            M = M[:r]
            full = np.zeros((r, N), dtype=np.int64); full[:, lo:] = M
            if piv:
                B = (B - self._mm(B[:, pn], full)) % p
                cnt += B.shape[0] * len(pn) * (N - lo)
            B = np.vstack([B, full]); piv = piv + pn
            o = np.argsort(piv); B = B[o]; piv = [piv[i] for i in o]

        for D in range(e, n + 1):
            lo = N - self.ndeg[D]
            j = D - e
            if j > 0:
                prevX, prevL = layers[j - 1]
                rows, last = [], []
                for row, li in zip(prevX, prevL):
                    for i in range(li + 1, n):
                        rows.append(self.mulz(row[None, :], i)[0]); last.append(i)
                X = np.array(rows, dtype=np.int64).reshape(-1, N)
                layers[j] = (X, last)
            else:
                X = layers[0][0]
            absorb(X, lo)
            while True:
                low = [ii for ii, c in enumerate(piv) if (D == n or self.deg[c] <= D - 1) and c not in done_rows]
                if not low:
                    break
                for ii in low:
                    done_rows.add(piv[ii])
                Hh = B[low]
                absorb(np.vstack([self.mulz(Hh, i) for i in range(n)]), lo)
            refuted = (N - 1) in piv
            lin = int(sum(1 for c in piv if self.deg[c] <= 1))
            hist.append(dict(D=D, rank=len(piv), muladds=int(cnt), refuted=bool(refuted), lin=lin))
            if refuted and Dref is None:
                Dref = D
            if lin >= lin_target and Droot is None:
                Droot = D
            if (mode == 'ref' and refuted) or (mode == 'root' and lin >= lin_target):
                break
        return dict(D_ref=Dref, D_root=Droot, hist=hist)

    def _mm(self, X, Y):
        p = self.p
        out = np.zeros((X.shape[0], Y.shape[1]), dtype=np.int64)
        step = 1024
        for s in range(0, X.shape[1], step):
            Z = X[:, s:s + step].astype(np.float64) @ Y[s:s + step].astype(np.float64)
            out = (out + np.rint(Z).astype(np.int64) % p) % p
        return out


def main():
    ks = [int(x) for x in sys.argv[1].split(',')]
    plan = [tuple(x.split(':')) for x in sys.argv[2].split(',')]   # e.g. x:4,v:4,x:5
    ntarg = int(sys.argv[3]) if len(sys.argv) > 3 else 3
    meta = json.load(open(f'{LEAD}/data/census_meta.json'))
    out = []
    t0 = time.time()
    for k in ks:
        C = VCurve(k)
        p, n = C.p, C.n
        g = tuple(meta[str(k)]['facts']['generator'])
        G = C.to_w(g)
        gen_ok = (g[1] * g[1] - int(C.rhs[g[0]])) % p == 0 and C.order_is_n(G)
        recs = json.load(open(f'{LEAD}/data/macaulay_k{k}.json'))
        Sf = {}
        for f, T in plan:
            T = int(T)
            if f not in Sf:
                Sf[f] = own_S(C, f)
            S = Sf[f]
            nb = T; nn = 2 * T
            odd = f == 'v'
            idx = np.arange(1 << nb)
            if odd:
                t = T - 1
                m = idx & ((1 << t) - 1); sg = idx >> t
                av = np.where(sg == 0, m, (p - m) % p)
            else:
                av = idx % p
            mine = [r for r in recs if r['f'] == f and r['T'] == T and r['kind'] == 'common'][:ntarg]
            mine += [r for r in recs if r['f'] == f and r['T'] == T and r['kind'] == 'covered'][:1]
            cl = Closure(nn, p)
            for r in mine:
                lR = r['target_log']
                Rw = C.wmul(lR, G)
                Rm = C.to_m(Rw)
                if f == 'x':
                    cR = (Rm[0] + C.A3) % p          # x(-R) = x(R)
                else:
                    cR = (-Rm[1]) % p                # v(-R) = -v(R)
                A_, B_ = np.meshgrid(av, av, indexing='ij')   # A_[ia, ib]
                vals_grid = S(A_, B_, np.full(A_.shape, cR, dtype=np.int64))
                vals = vals_grid.T.reshape(-1)       # index = ia + (ib << nb)
                h = moebius(vals, nn, p)
                nzm = np.flatnonzero(h)
                e = max(popc(x) for x in nzm)
                Z = np.flatnonzero(vals == 0)
                rec = dict(k=k, f=f, T=T, kind=r['kind'], target_log=lR, gen_ok=bool(gen_ok), e=int(e), terms=int(nzm.size),
                           roots=int(Z.size))
                if Z.size == 0:
                    inv = npow(vals, p - 2, p)
                    hi = moebius(inv, nn, p)
                    rec['M1_d_mult'] = int(max(popc(x) for x in np.flatnonzero(hi)))
                shc = np.zeros(1 << nn, dtype=np.int64); shc[cl.col_of] = h
                lin_t = None
                if Z.size:
                    Mz = np.array([[1] + [(int(z) >> i) & 1 for i in range(nn)] for z in Z], dtype=np.int64)
                    E, pv = echelon(Mz, p)
                    lin_t = nn + 1 - len(pv)
                tt = time.time()
                res = cl.run(shc, e, 'ref' if Z.size == 0 else 'root', lin_t)
                rec.update(D_ref=res['D_ref'], D_root=res['D_root'], hist=res['hist'], secs=round(time.time() - tt, 1))
                # compare with the producer's record
                pm = r.get('M2', {})
                ph = pm.get('hist', [])
                rec['producer'] = dict(e=r['e'], terms=r['terms'], roots=r['roots'], M1=r.get('M1', {}).get('d_mult'),
                                       D_ref=pm.get('D_ref'), D_root=pm.get('D_root'),
                                       ranks=[x['rank'] for x in ph], lins=[x['lin'] for x in ph],
                                       muladds=ph[-1]['muladds'] if ph else None)
                rec['match'] = dict(e=rec['e'] == r['e'], terms=rec['terms'] == r['terms'], roots=rec['roots'] == r['roots'],
                                    M1=rec.get('M1_d_mult') == r.get('M1', {}).get('d_mult'),
                                    D_ref=rec['D_ref'] == pm.get('D_ref'), D_root=rec['D_root'] == pm.get('D_root'),
                                    ranks=[x['rank'] for x in rec['hist']] == [x['rank'] for x in ph],
                                    muladds_ratio=(rec['hist'][-1]['muladds'] / ph[-1]['muladds']) if ph and ph[-1]['muladds'] else None)
                out.append(rec)
                print(f"k={k} f={f} T={T} {r['kind']} lR={lR} e={e}/{r['e']} terms={rec['terms']}/{r['terms']} roots={rec['roots']}/{r['roots']} "
                      f"M1={rec.get('M1_d_mult')}/{r.get('M1',{}).get('d_mult')} Dref={rec['D_ref']}/{pm.get('D_ref')} Droot={rec['D_root']}/{pm.get('D_root')} "
                      f"muladds={rec['hist'][-1]['muladds']}/{rec['producer']['muladds']} {rec['secs']}s", file=sys.stderr, flush=True)
    json.dump(out, open('mac_check_' + '_'.join(map(str, ks)) + '_' + sys.argv[2].replace(':', '').replace(',', '-') + '.json', 'w'), indent=1)


if __name__ == '__main__':
    main()
