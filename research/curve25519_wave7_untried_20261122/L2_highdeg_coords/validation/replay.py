"""Validator: independent reduced census replay at k = 16 (own point table with a
different generator, own base construction, own signed-pair coverage count).
Only the frame-draw protocol (seed rule from PROTOCOL section 3 and the order
L, L', L'' of the three uniform draws per frame) is shared, so rows can be compared
one by one with data/census_k16_<arm>_t<t>.npz."""
import hashlib, json, sys, time
import numpy as np
from vcurve import VCurve

LEAD = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L2_highdeg_coords'


def dseed(*parts):
    return int(hashlib.sha256(('wave7-L2|' + '|'.join(str(x) for x in parts)).encode()).hexdigest()[:12], 16)


class Sorted:
    def __init__(self, C, f):
        val, ok = C.fvals(f)
        logs = np.flatnonzero(ok)
        v = val[ok]
        o = np.lexsort((logs, v))
        self.v = v[o]; self.l = logs[o]; self.p = C.p
        # distinct values and one representative log per value
        first = np.concatenate([[True], self.v[1:] != self.v[:-1]])
        self.dv = self.v[first]; self.dl = self.l[first]

    def window(self, L, M):
        p = self.p
        L %= p
        hi = L + M
        if hi <= p:
            a = np.searchsorted(self.v, L, 'left'); b = np.searchsorted(self.v, hi, 'left')
            return self.l[a:b]
        a = np.searchsorted(self.v, L, 'left'); b = np.searchsorted(self.v, hi - p, 'left')
        return np.concatenate([self.l[a:], self.l[:b]])

    def first_c(self, L, c):
        i = np.searchsorted(self.dv, L % self.p, 'left')
        idx = (i + np.arange(c)) % self.dv.size
        return self.dl[idx]


def classes(logs, n):
    s = set()
    for l in np.asarray(logs).tolist():
        l %= n
        s.add(min(l, n - l))
    return sorted(s)


def cover(cls, n):
    if not cls:
        return 0, 0, 0
    c = np.array(cls, dtype=np.int64)
    S = np.concatenate([c, (n - c) % n])
    i, j = np.triu_indices(S.size)
    sm = (S[i] + S[j]) % n
    good = (sm % 8 == 0) & (sm != 0)
    return int(np.unique(sm[good]).size), len(cls), int(good.sum())


def main():
    arms = sys.argv[1].split(',') if len(sys.argv) > 1 else ['xAA', 'v', 'xe', 'xeye']
    ts = [int(x) for x in sys.argv[2].split(',')] if len(sys.argv) > 2 else [4, 5, 6, 7]
    Kmax = int(sys.argv[3]) if len(sys.argv) > 3 else 400
    C = VCurve(16)
    n, p = C.n, C.p
    X = Sorted(C, 'x')
    tabs = {f: Sorted(C, f) for f in ('v', 'xe', 'xeye')}
    res = {}
    t0 = time.time()
    for arm in arms:
        for t in ts:
            rng = np.random.default_rng(dseed(16, arm, t))
            data = np.load(f'{LEAD}/data/census_k16_{arm}_t{t}.npz')['rows'].astype(np.int64)
            K = min(Kmax, len(data))
            mine = np.zeros((K, 10), dtype=np.int64)
            for fr in range(K):
                L = int(rng.integers(p)); Lm = int(rng.integers(p)); Lv = int(rng.integers(p))
                if arm == 'xAA':
                    lg = X.window(L, 2 ** (t + 1)); vals = 2 ** (t + 1)
                else:
                    lg = tabs[arm].window(L, 2 ** t); vals = 2 ** t
                cl = classes(lg, n)
                cov, cols, J = cover(cl, n)
                xm = classes(X.first_c(Lm, cols), n) if cols else []
                xmc, _, xmJ = cover(xm, n)
                xv = classes(X.window(Lv, 2 ** t), n)
                xvc, xvcols, xvJ = cover(xv, n)
                mine[fr] = (vals, len(lg), cols, cov, J, xmc, xmJ, xvcols, xvc, xvJ)
            eq = mine == data[:K]
            res[f'{arm}/t{t}'] = dict(K=K, rows_identical=int(eq.all(axis=1).sum()),
                                      col_mismatch=[int(x) for x in (~eq).sum(axis=0)],
                                      my_R_col=float(mine[:, 3].mean() / mine[:, 5].mean()),
                                      data_R_col_same_rows=float(data[:K, 3].mean() / data[:K, 5].mean()),
                                      my_R_val=float(mine[:, 3].mean() / mine[:, 8].mean()))
            # translation-collapse check: class set of the f-base closed under +h for h in H
            print(f'{arm} t={t} K={K} identical={res[f"{arm}/t{t}"]["rows_identical"]} '
                  f'R_col={res[f"{arm}/t{t}"]["my_R_col"]:.4f} {time.time()-t0:.0f}s', file=sys.stderr, flush=True)
    # mechanism: invariance of xe / xeye class sets under translation by H
    r = C.r
    mech = {}
    rng = np.random.default_rng(4242)
    for f, Hs in (('xe', [4 * r]), ('xeye', [2 * r, 4 * r, 6 * r])):
        okc = 0
        for _ in range(200):
            L = int(rng.integers(p))
            cl = set(classes(tabs[f].window(L, 64), n))
            inv = all(set(min((c + h) % n, (n - c - h) % n) for c in cl) <= cl for h in Hs)
            okc += inv
        mech[f] = f'{okc}/200 bases with class set closed under +h, h in H'
    res['mechanism_k16'] = mech
    res['generator_uv_validator'] = list(C.gen_uv)
    json.dump(res, open('replay_' + '-'.join(arms) + '_' + '-'.join(map(str, ts)) + f'_K{Kmax}.json', 'w'), indent=1)
    print(json.dumps(res, indent=1))


if __name__ == '__main__':
    main()
