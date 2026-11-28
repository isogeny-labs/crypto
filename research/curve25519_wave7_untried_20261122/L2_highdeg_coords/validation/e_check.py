"""Validator: for EVERY recorded Macaulay target (all f, T = 4, 5, 6, k = 16, 18, 20)
recompute with own code: base coverage of [0, 2^t), census coverage of the target, the
reduced degree e, terms, Boolean roots, true decompositions, census agreement and the
M1 multiplier degree.  S^f is the validator's own e-basis interpolation."""
import json, sys, time
import numpy as np
from vcurve import VCurve, npow
from desk import ebasis, eval_ebasis, echelon, kernel_vector, sample_zeros
from mac_check import moebius, popc

LEAD = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L2_highdeg_coords'
DESK = json.load(open('desk_16_18_20_x-ye-xe-v-q-xeye-jac6-uv.json'))


def own_poly(C, f):
    p = C.p
    S = DESK[str(C.k)][f]['S_own']
    D, par = S['pervar'], S['parity']
    val, ok = C.fvals(f)
    rng = np.random.default_rng(55500 + C.k)
    cols = ebasis(D, par)
    a_, b_, c_ = sample_zeros(C, val, ok, len(cols) + 60, rng)
    M = eval_ebasis(cols, a_, b_, c_, p, D)
    E, piv = echelon(M, p)
    assert len(cols) - len(piv) == 1
    fc = [c for c in range(len(cols)) if c not in set(piv)][0]
    x = kernel_vector(E, piv, len(cols), fc, p)
    nz = np.flatnonzero(x)
    sel = [cols[i] for i in nz]; co = x[nz]

    def Sfun(a, b, c):
        out = np.zeros(a.size, dtype=np.int64)
        step = 512
        for s in range(0, a.size, step):
            Mm = eval_ebasis(sel, a[s:s + step] % p, b[s:s + step] % p, c[s:s + step] % p, p, D)
            acc = np.zeros(Mm.shape[0], dtype=np.int64)
            for j in range(0, Mm.shape[1], 1024):
                acc = (acc + (Mm[:, j:j + 1024] * co[None, j:j + 1024] % p).sum(axis=1)) % p
            out[s:s + step] = acc
        return out
    return Sfun


def main():
    meta = json.load(open(f'{LEAD}/data/census_meta.json'))
    res = []
    t0 = time.time()
    for k in (16, 18, 20):
        C = VCurve(k); p, n, r = C.p, C.n, C.r
        G = C.to_w(tuple(meta[str(k)]['facts']['generator']))
        recs = json.load(open(f'{LEAD}/data/macaulay_k{k}.json'))
        for f in ('x', 'ye', 'xe', 'v', 'q', 'xeye', 'jac6', 'uv'):
            Sf = own_poly(C, f)
            val, ok = C.fvals(f)
            odd = DESK[str(k)][f]['S_own']['parity'] is not None
            for T in (4, 5, 6):
                t = T - 1 if odd else T
                idx = np.arange(1 << T)
                if odd:
                    m = idx & ((1 << t) - 1); sg = idx >> t
                    av = np.where(sg == 0, m, (p - m) % p)
                else:
                    av = idx % p
                # base [0, 2^t) and its exact signed-pair coverage, own logs
                base = np.flatnonzero(ok & (val >= 0) & (val < (1 << t)))
                cls = np.unique(np.minimum(base, n - base))
                Ssig = np.concatenate([cls, (n - cls) % n])
                sm = (Ssig[:, None] + Ssig[None, :]) % n
                covset = set(np.unique(sm[(sm % 8 == 0) & (sm != 0)]).tolist())
                bcov = len(covset) / r
                # value -> logs over all points (full fibres)
                for f_r in [r_ for r_ in recs if r_['f'] == f and r_['T'] == T]:
                    lR = f_r['target_log']
                    Rm = C.to_m(C.wmul(lR, G))
                    myl = C.log_of(Rm)
                    neg = (n - myl) % n
                    out = dict(k=k, f=f, T=T, kind=f_r['kind'], target_log=lR, base_cov=bcov,
                               base_cov_rec=f_r['base_cov'], covered=myl in covset)
                    if not ok[neg]:
                        out['skip'] = True; out['rec_skip'] = 'skip' in f_r; res.append(out); continue
                    cR = int(val[neg])
                    A_, B_ = np.meshgrid(av, av, indexing='ij')
                    vals = Sf(A_.ravel(), B_.ravel(), np.full(A_.size, cR, dtype=np.int64)).reshape(A_.shape).T.reshape(-1)
                    h = moebius(vals, 2 * T, p)
                    nzm = np.flatnonzero(h)
                    e = max(popc(x) for x in nzm)
                    Z = np.flatnonzero(vals == 0)
                    true = 0
                    for z in Z.tolist():
                        a = int(av[z & ((1 << T) - 1)]); b = int(av[z >> T])
                        La = np.flatnonzero(ok & (val == a)); Lb = np.flatnonzero(ok & (val == b))
                        for l1 in La.tolist():
                            for l2 in Lb.tolist():
                                if (l1 + l2) % n == myl:
                                    true += 1
                    out.update(e=e, terms=int(nzm.size), roots=int(Z.size), true=true,
                               agrees=(true > 0) == out['covered'])
                    if Z.size == 0:
                        hi = moebius(npow(vals, p - 2, p), 2 * T, p)
                        out['M1'] = int(max(popc(x) for x in np.flatnonzero(hi)))
                    out['rec'] = dict(e=f_r.get('e'), terms=f_r.get('terms'), roots=f_r.get('roots'), true=f_r.get('true_decomp'),
                                      covered=f_r.get('census_covered'), M1=f_r.get('M1', {}).get('d_mult'))
                    out['match'] = (e == f_r['e'] and out['terms'] == f_r['terms'] and out['roots'] == f_r['roots']
                                    and out['covered'] == f_r['census_covered'] and out.get('M1') == f_r.get('M1', {}).get('d_mult')
                                    and abs(bcov - f_r['base_cov']) < 1e-12)
                    out['true_match'] = true == f_r['true_decomp']
                    res.append(out)
            print(f'k={k} f={f} done {time.time()-t0:.0f}s mismatches so far {sum(1 for x in res if not x.get("match", True))}', file=sys.stderr, flush=True)
    json.dump(res, open('e_check.json', 'w'), indent=1)


if __name__ == '__main__':
    main()
