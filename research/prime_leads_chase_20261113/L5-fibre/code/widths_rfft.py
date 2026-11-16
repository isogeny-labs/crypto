"""Summation-message widths |jF|, pinned widths, and Fourier concentration for factor bases
in the order-r subgroup G of a toy curve. Bases are mapped to Z_r by exact discrete logs
(BSGS), then all sumset work is exact integer/bitmap arithmetic in Z_r.
Usage: widths.py OUT.jsonl bits [bits ...]"""
import json, math, os, random, sys, time
import numpy as np
sys.path.insert(0, os.path.dirname(__file__))
from ec import find_curve, fibre_points, bsgs_log, in_G, INF

OUT = sys.argv[1]
N_TARGETS = 200


def sumset_bitmaps(logs, r, jmax):
    """Exact bitmaps of jA for j=1..jmax, A given by logs. Uses exact integer convolution counts
    via FFT with verification on small cases; the bitmap is exact if counts rounded > 0.5."""
    a = np.zeros(r, dtype=np.float64)
    a[np.array(sorted(set(logs)), dtype=np.int64)] = 1.0
    fa = np.fft.rfft(a)
    maps, counts = {1: a > 0.5}, {1: a.copy()}
    fj = fa.copy()
    for j in range(2, jmax + 1):
        fj = fj * fa
        c = np.fft.irfft(fj, n=r)
        counts[j] = np.rint(c)
        assert np.max(np.abs(c - counts[j])) < 0.25, 'fft precision'
        maps[j] = counts[j] > 0.5
    return maps, counts, fa


def sumset_exact(logs, r, j):
    """Pure exact sumset (python sets) for verification of the FFT path at small sizes."""
    A = set(logs)
    S = {0}
    for _ in range(j):
        S = {(s + x) % r for s in S for x in A}
    return S


def spectrum_stats(fa, B, r):
    pw = np.abs(fa[1:]) ** 2  # rfft half spectrum (r odd: conjugate pairs), drop trivial
    tot = pw.sum()
    srt = np.sort(pw)[::-1]
    out = {}
    for name, k in (('top_B', B), ('top_sqrt_r', int(math.isqrt(r)))):
        k = max(1, min(k // 2, len(srt)))
        out[name] = float(srt[:k].sum() / tot)
    out['max_over_B'] = float(np.sqrt(srt[0]) / B)  # largest nontrivial |hat 1_A| / B
    return out


def pinned_widths(maps, r, m, rng):
    """For m-tuples: max over cuts j=1..m-1 of |jA ∩ (R-(m-j)A)|, averaged over random targets
    R drawn uniformly from Z_r, and also over targets with >=1 decomposition."""
    res = {}
    for j in range(1, m):
        left = maps[j]
        right = maps[m - j]
        # R - (m-j)A as bitmap: index s in it iff R-s in right  <=> s = R - y, y in right
        rev = np.roll(right[::-1], 1)  # rev[s] = right[-s mod r]
        ws = []
        for _ in range(N_TARGETS):
            R = rng.randrange(r)
            ws.append(int(np.count_nonzero(left & np.roll(rev, R))))
        res[j] = float(np.mean(ws))
    return res


def run(bits, rows_out):
    t0 = time.time()
    C = find_curve(bits, 1000 + bits)
    E, p, r, g, t = C['E'], C['p'], C['r'], C['g'], C['t']
    log = bsgs_log(E, g, r)
    rng = random.Random(bits)
    # verify log on a few random points
    for _ in range(3):
        k = rng.randrange(1, r)
        assert log(E.mul(k, g)) == k
    m_list = (2, 3)
    for expo_name, expo in (('p^1/3', 1 / 3), ('p^1/4', 1 / 4), ('p^1/5', 1 / 5)):
        Bt = max(8, int(round(p ** expo)))
        bases = {}
        # A1 fibre base, random image set S
        cand = list(range(p))
        rng.shuffle(cand)
        F, S = [], []
        for xp in cand:
            pts = fibre_points(C, xp)
            if pts:
                F += pts; S.append(xp)
            if len(F) >= Bt:
                break
        bases['A1_fibre_randS'] = [log(P) for P in F]
        # A2 fibre base, interval image set S = {0,1,2,...}
        F, xp = [], 0
        while len(F) < Bt:
            F += fibre_points(C, xp); xp += 1
        bases['A2_fibre_intS'] = [log(P) for P in F]
        # I interval-x base on E
        F, x = [], 0
        while len(F) < Bt:
            F += [P for P in E.lift(x) if x != t and in_G(E, r, P)]; x += 1
        bases['I_interval_x'] = [log(P) for P in F]
        B = len(bases['A1_fibre_randS'])
        # P positive control: AP of logs, symmetric {±1..±B/2}
        half = max(1, B // 2)
        bases['P_AP_logs'] = [k for k in range(1, half + 1)] + [r - k for k in range(1, half + 1)]
        # N negative controls: random symmetric sets of the same size as each arm (3 seeds)
        for s in range(3):
            rs = random.Random(bits * 100 + s + int(expo * 1000))
            ks = set()
            while len(ks) < half:
                ks.add(rs.randrange(1, (r + 1) // 2))
            bases[f'N_random_s{s}'] = list(ks) + [r - k for k in ks]
        for name, logs in bases.items():
            logs = sorted(set(l % r for l in logs))
            Bn = len(logs)
            maps, counts, fa = sumset_bitmaps(logs, r, 3)
            W = {j: int(np.count_nonzero(maps[j])) for j in (1, 2, 3)}
            if Bn ** 2 <= 2e5:
                assert len(sumset_exact(logs, r, 2)) == W[2]
            sym = all(((r - l) % r) in set(logs) for l in logs)
            row = dict(bits=bits, p=p, r=r, h=C['h'], B_rule=expo_name, arm=name, B=Bn, symmetric=sym,
                       W1=W[1], W2=W[2], W3=W[3],
                       gen2=min(Bn * (Bn + 1) // 2 if not sym else Bn * Bn // 2 + 1, r),
                       spectrum=spectrum_stats(fa, Bn, r),
                       pinned_m2=pinned_widths(maps, r, 2, random.Random(1)),
                       pinned_m3=pinned_widths(maps, r, 3, random.Random(2)),
                       mean_count_m2=float(counts[2].mean()), mean_count_m3=float(counts[3].mean()),
                       frac_targets_decomposable_m3=float(np.count_nonzero(maps[3]) / r))
            rows_out.write(json.dumps(row) + '\n'); rows_out.flush()
            print(bits, expo_name, name, Bn, W, round(time.time() - t0, 1), flush=True)


with open(OUT, 'a') as fo:
    for b in sys.argv[2:]:
        run(int(b), fo)
