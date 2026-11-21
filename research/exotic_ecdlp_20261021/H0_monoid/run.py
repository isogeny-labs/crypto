# H0 monoid probe (EXP-EXOTIC-H0-001). Plain-Python Sage program.  `from sage.all import *` is auto-prepended.
import builtins, os, json, math, time, traceback
import numpy as np
sum = builtins.sum; min = builtins.min; max = builtins.max; abs = builtins.abs
range = builtins.range; sorted = builtins.sorted; round = builtins.round; len = builtins.len

SEED = 20261022
set_random_seed(SEED)
OUT = os.environ['ISOLAB_OUTPUT_DIR']
os.makedirs(OUT, exist_ok=True)
PRIMES = [1009, 65537, 1048583]
T = 2000          # targets
K = 100000        # random signed m-subsets for sumset coverage
MMAX = 6
NKS = 200         # replicate KS tests
KS_BATCH = 500    # pairs per replicate KS test
STEPS = 10        # greedy steps
CELL_BUDGET_S = 600
T0 = time.time()

def log(*a):
    print('[%7.1fs]' % (time.time() - T0), *a, flush=True)

# ---------------- vectorised affine arithmetic on y^2 = x^3 + a x + b over F_p ----------------
def vinv(a, p):
    # modular inverse via Fermat, vectorised (p < 2^21 so products fit int64)
    r = np.ones_like(a); base = a % p; e = p - 2
    while e:
        if e & 1:
            r = (r * base) % p
        base = (base * base) % p
        e >>= 1
    return r

def vadd(P, Q, a, p):
    x1, y1, i1 = P; x2, y2, i2 = Q
    x1, y1, i1, x2, y2, i2 = np.broadcast_arrays(x1, y1, i1, x2, y2, i2)
    same_x = (x1 == x2)
    dbl = same_x & (y1 == y2) & (y1 != 0)
    neg = same_x & ~dbl            # y1 == -y2 (incl. y=0 doubling) -> infinity
    num = np.where(dbl, (3 * x1 * x1 + a) % p, (y2 - y1) % p)
    den = np.where(dbl, (2 * y1) % p, (x2 - x1) % p)
    den = np.where(den == 0, 1, den)
    lam = (num * vinv(den, p)) % p
    x3 = (lam * lam - x1 - x2) % p
    y3 = (lam * (x1 - x3) - y1) % p
    inf = neg
    # identity handling
    x3 = np.where(i1, x2, np.where(i2, x1, x3))
    y3 = np.where(i1, y2, np.where(i2, y1, y3))
    inf = np.where(i1, i2, np.where(i2, i1, inf))
    return x3.astype(np.int64), y3.astype(np.int64), inf.astype(bool)

# ---------------- statistics ----------------
def ks_uniform(u):
    u = np.sort(np.asarray(u, dtype=float)); n = len(u)
    i = np.arange(1, n + 1)
    D = max(float(np.max(i / n - u)), float(np.max(u - (i - 1) / n)))
    lam = (math.sqrt(n) + 0.12 + 0.11 / math.sqrt(n)) * D
    s = 0.0
    for k in range(1, 101):
        s += 2 * (-1) ** (k - 1) * math.exp(-2 * k * k * lam * lam)
    return D, min(1.0, max(0.0, s))

def summ(v):
    v = np.asarray(v, dtype=float)
    return {'mean': float(v.mean()), 'median': float(np.median(v)), 'min': float(v.min()), 'n': int(len(v))}

def ols_slope(Y, s):
    # Y: W x len(s) matrix; per-row OLS slope vs s
    s = np.asarray(s, dtype=float); sc = s - s.mean()
    return (Y * sc).sum(axis=1) / (sc * sc).sum()

# ---------------- one cell ----------------
def run_cell(p):
    t_cell = time.time()
    rng = np.random.default_rng(SEED + p)
    cell = {'p': p}
    # --- build curve with large prime subgroup ---
    E = None
    for attempt in range(500):
        a = int(ZZ.random_element(0, p)); b = int(ZZ.random_element(0, p))
        if (4 * a ** 3 + 27 * b ** 2) % p == 0:
            continue
        Ec = EllipticCurve(GF(p), [a, b])
        Nn = int(Ec.cardinality())
        fac = factor(Nn)
        ell = int(max(f for f, _ in fac))
        if ell >= Nn // 8 and ell > 100 and ZZ(ell).is_prime():
            E = Ec; break
    if E is None:
        return {'censored': True, 'reason': 'no curve with large prime subgroup found'}
    G = None
    while G is None:
        Pt = E.random_point()
        cand = (Nn // ell) * Pt
        if cand != E(0):
            G = cand
    gen_ok = bool(ZZ(ell).is_prime() and (ell * G == E(0)) and G != E(0) and ell >= Nn // 8)
    cell.update({'a': a, 'b': b, 'N': Nn, 'ell': ell, 'cofactor': Nn // ell, 'G': [int(G[0]), int(G[1])]})
    # --- factor base ---
    X = int(math.isqrt(p - 1) + 1)   # ceil(sqrt p)
    fx, fy = [], []
    for x in range(X):
        rhs = (x ** 3 + a * x + b) % p
        if rhs == 0:
            fx.append(x); fy.append(0)
        elif pow(rhs, (p - 1) // 2, p) == 1:
            y = int(GF(p)(rhs).sqrt())
            fx += [x, x]; fy += [y, (p - y) % p]
    Fx = np.array(fx, dtype=np.int64); Fy = np.array(fy, dtype=np.int64); n = len(Fx)
    cell['F_size'] = int(n); cell['sqrt_p'] = float(math.sqrt(p)); cell['ceil_sqrt_p'] = X
    cell['F_over_sqrt_p'] = float(n / math.sqrt(p))
    Fz = np.zeros(n, dtype=bool)
    log('p=%d N=%d ell=%d cof=%d |F|=%d' % (p, Nn, ell, Nn // ell, n))
    # --- self-check of vector add against Sage ---
    ok = True
    idx = rng.integers(0, n, 60); idy = rng.integers(0, n, 60)
    Px = Fx[idx]; Py = Fy[idx]; Qx = Fx[idy]; Qy = Fy[idy]
    rx, ry, ri = vadd((Px, Py, np.zeros(60, bool)), (Qx, Qy, np.zeros(60, bool)), a, p)
    for k in range(60):
        S = E(int(Px[k]), int(Py[k])) + E(int(Qx[k]), int(Qy[k]))
        if S == E(0):
            ok &= bool(ri[k])
        else:
            ok &= (not ri[k]) and int(S[0]) == int(rx[k]) and int(S[1]) == int(ry[k])
    cell['vadd_matches_sage'] = bool(ok)
    # --- random-point sampler (uniform over E(F_p)\{O}) ---
    ys = np.arange(p, dtype=np.int64)
    root = -np.ones(p, dtype=np.int64)
    root[(ys * ys) % p] = ys
    def rand_points(m):
        xs = np.empty(0, dtype=np.int64)
        while len(xs) < m:
            c = rng.integers(0, p, 2 * m + 16).astype(np.int64)
            rhs = (c * c % p * c + a * c + b) % p
            xs = np.concatenate([xs, c[root[rhs] >= 0]])
        xs = xs[:m]
        rhs = (xs * xs % p * xs + a * xs + b) % p
        y = root[rhs]
        sgn = rng.integers(0, 2, m).astype(bool)
        y = np.where(sgn, (p - y) % p, y)
        return xs, y
    # --- (b) sumset coverage rho_m ---
    Tx, Ty = rand_points(T)
    tkeys = Tx * p + Ty
    idxs = rng.integers(0, n, (K, MMAX))
    for _ in range(200):
        s = np.sort(idxs, axis=1)
        bad = (s[:, 1:] == s[:, :-1]).any(axis=1)
        if not bad.any():
            break
        idxs[bad] = rng.integers(0, n, (int(bad.sum()), MMAX))
    cur = (np.zeros(K, np.int64), np.zeros(K, np.int64), np.ones(K, bool))
    cum = np.empty(0, dtype=np.int64)
    rho = {}
    null = {}
    bound = {}
    cumdraws = 0
    from math import comb
    for m in range(1, MMAX + 1):
        pt = (Fx[idxs[:, m - 1]], Fy[idxs[:, m - 1]], np.zeros(K, bool))
        cur = vadd(cur, pt, a, p)
        keys = (cur[0] * p + cur[1])[~cur[2]]
        cum = np.unique(np.concatenate([cum, keys]))
        rho[str(m)] = float(np.isin(tkeys, cum).mean())
        cumdraws += K
        null[str(m)] = float(1 - math.exp(-cumdraws / Nn))
        bound[str(m)] = float(min(1.0, sum(comb(n, j) for j in range(1, m + 1)) / Nn))
    cell['rho_m'] = rho
    cell['rho_m_null_random_draws'] = null
    cell['rho_m_combinatorial_bound'] = bound
    cell['rho_m_distinct_sums_cum'] = int(len(cum))
    log('rho_m', rho, 'null', null)
    if time.time() - t_cell > CELL_BUDGET_S:
        cell['censored'] = True; cell['reason'] = 'time budget exceeded after sumset'; return cell
    # --- (c) rank defect: normalized x(R-Q); F is closed under negation so R-Q ranges over R+F ---
    def ranks_all(Rx, Ry):
        # returns (normalized rank array over all Q in F, with inf -> 0)
        x3, y3, inf = vadd((np.full(n, Rx, np.int64), np.full(n, Ry, np.int64), Fz.copy()), (Fx, Fy, Fz), a, p)
        r = np.where(inf, 0.0, (x3 + 0.5) / p)
        return r, x3, y3, inf
    Rx0, Ry0 = rand_points(T)
    best = np.empty(T); pooled = np.empty((T, n)); onerand = np.empty(T)
    qpick = rng.integers(0, n, T)
    for i in range(T):
        r, _, _, _ = ranks_all(int(Rx0[i]), int(Ry0[i]))
        pooled[i] = r; best[i] = r.min(); onerand[i] = r[qpick[i]]
    D1, p1 = ks_uniform(onerand)
    Dp, pp = ks_uniform(pooled.ravel())
    ub = 1 - (1 - best) ** n     # null CDF of min of n iid uniforms
    Dm, pm = ks_uniform(ub)
    cell['rank_one_random_Q'] = summ(onerand); cell['ks_one_random_Q'] = {'D': D1, 'p_value': p1}
    cell['rank_all_pairs'] = summ(pooled.ravel()); cell['ks_all_pairs_dependent'] = {'D': Dp, 'p_value': pp}
    cell['rank_min_over_Q'] = summ(best); cell['rank_min_over_Q_null_mean'] = float(1.0 / (n + 1))
    cell['rank_min_over_Q_null_median'] = float(1 - 0.5 ** (1.0 / n))
    cell['ks_min_over_Q_vs_beta1n'] = {'D': Dm, 'p_value': pm}
    # --- replicate KS tests (NKS batches of KS_BATCH independent (R, random Q) pairs) ---
    Rrx, Rry = rand_points(NKS * KS_BATCH)
    qi = rng.integers(0, n, NKS * KS_BATCH)
    z = np.zeros(NKS * KS_BATCH, bool)
    x3, y3, inf = vadd((Rrx, Rry, z), (Fx[qi], Fy[qi], z), a, p)
    rr = np.where(inf, 0.0, (x3 + 0.5) / p).reshape(NKS, KS_BATCH)
    pvals = np.array([ks_uniform(rr[i])[1] for i in range(NKS)])
    cell['ks_replicates'] = {'n_tests': NKS, 'batch': KS_BATCH, 'frac_p_lt_0.01': float((pvals < 0.01).mean()),
                             'frac_p_lt_0.05': float((pvals < 0.05).mean()), 'min_p': float(pvals.min()),
                             'ks_of_pvalues_vs_uniform_p': ks_uniform(pvals)[1]}
    log('KS one-random-Q p=%.4g, min-stat p=%.4g, replicates frac<0.01=%.3f' % (p1, pm, (pvals < 0.01).mean()))
    if time.time() - t_cell > CELL_BUDGET_S:
        cell['censored'] = True; cell['reason'] = 'time budget exceeded after rank defect'; return cell
    # --- greedy recursion: R <- R - Q*, STEPS steps, T walks; real label x vs hashed-label control ---
    Wx, Wy = rand_points(T)
    perm = rng.permutation(p).astype(np.int64)    # fixed random permutation of [0,p): structure-free label, same support
    def greedy(lab):
        traj = np.zeros((T, STEPS + 1)); traj[:, 0] = (lab[Wx] + 0.5) / p
        hit = np.zeros(T, dtype=bool)             # walk landed back in F (R - Q = O for some Q)
        hstep = np.zeros(T, dtype=np.int64)       # step index at which R-Q=O found (= decomposition length), 0 if none
        for w in range(T):
            x, y = int(Wx[w]), int(Wy[w]); done = False
            for s_ in range(1, STEPS + 1):
                if done:
                    traj[w, s_] = 0.0; continue
                x3, y3, inf = vadd((np.full(n, x, np.int64), np.full(n, y, np.int64), Fz.copy()), (Fx, Fy, Fz), a, p)
                r = np.where(inf, 0.0, (lab[x3] + 0.5) / p)
                j = int(np.argmin(r))
                traj[w, s_] = r[j]
                if inf[j]:
                    done = True; hit[w] = True; hstep[w] = s_
                else:
                    x, y = int(x3[j]), int(y3[j])
        return traj, hit, hstep
    def cost_stats(hit, hstep):
        # group-op accounting: each greedy step enumerates |F| additions R+Q (n ops). A walk that finds R-Q=O at step s
        # used s*n ops and yields a decomposition of length s (s-1 chosen terms + the final base element).
        steps_used = np.where(hit, hstep, STEPS)
        total_ops = int(steps_used.sum()) * n
        nh = int(hit.sum())
        out_ = {'ops_per_step': int(n), 'walks': T, 'successes': nh, 'total_ops_all_walks': total_ops}
        if nh > 0:
            ops_succ = hstep[hit] * n
            out_['median_ops_successful_walk'] = float(np.median(ops_succ))
            out_['mean_ops_successful_walk'] = float(ops_succ.mean())
            out_['amortized_ops_per_decomposition'] = float(total_ops / nh)   # includes failed walks
            lens = hstep[hit]
            out_['decomposition_length_hist'] = {str(k): int((lens == k).sum()) for k in range(1, STEPS + 1)}
            out_['decomposition_length_median'] = float(np.median(lens)); out_['decomposition_length_max'] = int(lens.max())
        else:
            out_['median_ops_successful_walk'] = None; out_['amortized_ops_per_decomposition'] = None
        return out_
    def arm(lab):
        traj, hit, hstep = greedy(lab)
        steps = np.arange(1, STEPS + 1)
        sl = ols_slope(traj[:, 1:], steps); sl0 = ols_slope(traj, np.arange(0, STEPS + 1))
        m_ = float(sl.mean()); se_ = float(sl.std(ddof=1) / math.sqrt(T))
        dec = (traj[:, 2:] < traj[:, 1:-1]).mean(axis=0)
        return {'walks': T, 'steps': STEPS,
                'mean_rank_by_step': [float(v) for v in traj.mean(axis=0)],
                'median_rank_by_step': [float(v) for v in np.median(traj, axis=0)],
                'null_mean_rank_after_greedy_step': float(1.0 / (n + 1)),
                'slope_steps_1_to_10': m_, 'slope_se': se_, 'slope_z': float(m_ / se_) if se_ > 0 else 0.0,
                'slope_steps_0_to_10_incl_start': float(sl0.mean()),
                'slope_relative_to_null_mean': float(m_ / (1.0 / (n + 1))),
                'frac_strict_decrease_by_step_1to9': [float(v) for v in dec],
                'hit_rate_lands_in_F': float(hit.mean()),
                'cost': cost_stats(hit, hstep),
                'convention': 'slope of normalized label-rank vs step; negative = decreases'}
    cell['recursion'] = arm(np.arange(p, dtype=np.int64))
    cell['recursion_hashed'] = arm(perm)
    rr_, rh_ = cell['recursion'], cell['recursion_hashed']
    mean_sl, se_sl = rr_['slope_steps_1_to_10'], rr_['slope_se']
    sl0 = np.array([rr_['slope_steps_0_to_10_incl_start']])
    cell['slope_diff_real_minus_hashed'] = mean_sl - rh_['slope_steps_1_to_10']
    cell['slope_diff_z'] = float((mean_sl - rh_['slope_steps_1_to_10']) / math.sqrt(se_sl ** 2 + rh_['slope_se'] ** 2))
    cell['recursion_slope'] = mean_sl
    cell['recursion_slope_z'] = cell['recursion']['slope_z']
    cell['ks_p_primary'] = p1
    cell['validation'] = {'generator_order_prime_factor_ok': gen_ok, 'vadd_matches_sage': bool(ok)}
    cell['elapsed_s'] = time.time() - t_cell
    log('recursion slope(1..10)=%.3g se=%.3g z=%.2f ; start-incl slope=%.3g ; elapsed %.1fs' %
        (mean_sl, se_sl, cell['recursion_slope_z'], float(sl0.mean()), cell['elapsed_s']))
    return cell

def finalize(cells, built):
    ok = [c for c in cells.values() if not c.get('censored') and 'recursion_slope' in c]
    out = {'experiment': 'EXP-EXOTIC-H0-001', 'seed': SEED, 'primes': PRIMES, 'T': T, 'K': K, 'm_max': MMAX,
           'ks_replicates': NKS, 'cells': cells,
           'recursion_slope_definition': 'slope of mean normalized x(R-Q*) rank vs greedy step (steps 1..10); negative = decreases. Compared against a hashed-label control (fixed random permutation of [0,p)); decision uses real-minus-hashed slope z. Top-level value = most negative real slope over p.',
           'validation': {'fields_built': bool(built),
                          'generator_order_prime_factor_ok': bool(len(ok) > 0 and all(c['validation']['generator_order_prime_factor_ok'] for c in ok)),
                          'vadd_matches_sage': bool(len(ok) > 0 and all(c['validation']['vadd_matches_sage'] for c in ok)),
                          'all_cells_completed': bool(len(ok) == len(PRIMES))},
           'total_elapsed_s': time.time() - T0}
    if ok:
        out['recursion_slope'] = float(min(c['recursion_slope'] for c in ok))
        out['recursion_slope_by_p'] = {str(c['p']): c['recursion_slope'] for c in ok}
        out['ks_p_by_p'] = {str(c['p']): c['ks_p_primary'] for c in ok}
    else:
        out['recursion_slope'] = None
    # decision rule
    verdict = 'inconclusive'
    if len(ok) == len(PRIMES):
        uniform_all = all(c['ks_p_primary'] > 0.01 for c in ok)
        diff_neg_all = all(c['slope_diff_z'] < -2.58 for c in ok)
        ps = np.log([float(c['p']) for c in ok])
        def fit(key, cells_=None):
            cs = cells_ if cells_ is not None else ok
            v = [c['recursion']['cost'][key] for c in cs]
            if len(cs) < 2 or any(x is None or x <= 0 for x in v):
                return None
            return float(np.polyfit(np.log([float(c['p']) for c in cs]), np.log(v), 1)[0])
        big = [c for c in ok if c['p'] >= 65537]   # |F|/sqrt(p) ~ 1.03-1.04 and |F| >= 264; p=1009 (|F|=18) is finite-size
        c_amort3 = fit('amortized_ops_per_decomposition'); c_med3 = fit('median_ops_successful_walk')
        c_amort = fit('amortized_ops_per_decomposition', big); c_med = fit('median_ops_successful_walk', big)
        med_len = [c['recursion']['cost'].get('decomposition_length_median') for c in ok]
        out['cost_exponent'] = {'c_amortized_ops_per_decomposition': c_amort, 'c_median_ops_successful_walk': c_med, 'fit_primary': 'p in {65537,1048583} (2 points; p=1009 excluded as finite-size: |F|=18, |F|/sqrt p=0.57)', 'c_amortized_3pt_all_p': c_amort3, 'c_median_3pt_all_p': c_med3,
                                'reference_sqrtN_exponent': 0.5,
                                'amortized_ops_by_p': {str(c['p']): c['recursion']['cost']['amortized_ops_per_decomposition'] for c in ok},
                                'median_ops_by_p': {str(c['p']): c['recursion']['cost']['median_ops_successful_walk'] for c in ok},
                                'ops_over_sqrt_p_amortized': {str(c['p']): (c['recursion']['cost']['amortized_ops_per_decomposition'] or 0) / math.sqrt(c['p']) for c in ok},
                                'decomposition_length_median_by_p': {str(c['p']): m for c, m in zip(ok, med_len)},
                                'note': 'few-point log-log fits; indicative only'}
        out['decision_inputs'] = {'ks_uniform_at_all_p(p>0.01)': uniform_all, 'c_amortized': c_amort, 'c_median': c_med, 'c_amortized_3pt': c_amort3, 'c_median_3pt': c_med3,
                                  'slope_real_sig_more_negative_than_hashed_all_p (demoted, confounded)': diff_neg_all}
        out['control_summary'] = {str(c['p']): {'real_slope': c['recursion']['slope_steps_1_to_10'], 'real_z': c['recursion']['slope_z'],
                                  'hashed_slope': c['recursion_hashed']['slope_steps_1_to_10'], 'hashed_z': c['recursion_hashed']['slope_z'],
                                  'diff_z': c['slope_diff_z'], 'hit_real': c['recursion']['hit_rate_lands_in_F'],
                                  'hit_hashed': c['recursion_hashed']['hit_rate_lands_in_F']} for c in ok}
        out['verdict_rule'] = 'cost rule: KILL if c>=0.45 (ops/decomposition ~ sqrt(p) or worse); WIN only if c<0.45 and decomposition length does not grow with p; else inconclusive. Slope comparison demoted (confounded: F is x-defined).'
        lit = 'inconclusive' if c_amort3 is None else ('kill' if c_amort3 >= 0.45 else 'win')
        out['verdict_literal_3pt_amortized_fit'] = lit
        if c_amort is None:
            verdict = 'inconclusive'
        elif c_amort >= 0.45:
            verdict = 'kill'
        elif c_amort < 0.45 and all(m is not None for m in med_len) and med_len[-1] <= med_len[0]:
            verdict = 'win'
    out['verdict'] = verdict
    with open(os.path.join(OUT, 'result.json'), 'w') as f:
        json.dump(out, f, indent=1)
    return out

cells = {}
built = False
for p in PRIMES:
    assert ZZ(p).is_prime()
    try:
        cells[str(p)] = run_cell(p)
        built = True
    except Exception as ex:
        traceback.print_exc()
        cells[str(p)] = {'p': p, 'censored': True, 'reason': 'exception: %r' % (ex,)}
    finalize(cells, built)   # incremental write
res = finalize(cells, built)
log('VERDICT', res['verdict'], 'recursion_slope', res.get('recursion_slope'))
print(json.dumps(res, indent=1))
