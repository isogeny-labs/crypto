# H5: Ulmer Legendre family at d = p^2+1 (f=2): specialization kernel K vs geometric relations K_geo.
# Plain-Python Sage program (no preparser): use ** for powers. `from sage.all import *` is auto-prepended
# on isolab, but we import it again so the file also runs standalone.
from sage.all import *
import os, json, time, traceback, itertools, math
from cysignals.alarm import alarm, cancel_alarm

set_random_seed(20261022)
OUTDIR = os.environ.get('ISOLAB_OUTPUT_DIR', '.')
T0 = time.time()
PS = [7, 11]
F_EXP = 2
CELL_BUDGET_S = {7: 600, 11: 700}   # soft per-cell wall budget for the many-fiber stage


def plog(*a):
    print('[%7.1fs]' % (time.time() - T0), *a, flush=True)


RESULT = {
    'experiment': 'EXP-EXOTIC-H5-001', 'f': F_EXP, 'seed': 20261022,
    'cells': {}, 'validation': {}, 'errors': {}, 'censored': [],
    'corank': None, 'verdict': 'inconclusive',
}


def save():
    RESULT['elapsed_s'] = time.time() - T0
    with open(os.path.join(OUTDIR, 'result.json'), 'w') as fh:
        json.dump(RESULT, fh, indent=1, default=str)


# ---------------------------------------------------------------- helpers
def ulmer_rank(d, q):
    """Ulmer (Annals 2002, Thm 1.5) rank of y^2=x(x+1)(x+t^d), d | p^n+1, over F_q(t)."""
    tot = Integer(0)
    for e in divisors(d):
        if 6 % e == 0:
            continue
        tot += Integer(euler_phi(e)) // Integer(Mod(q, e).multiplicative_order())
    if d % 2 == 0 and (q - 1) % 4 == 0:
        tot += 1
    if d % 3 == 0:
        tot += 1 if (q - 1) % 3 != 0 else 2
    return int(tot)


def key(P):
    if P.is_zero():
        return None
    return (P[0], P[1])


def group_data(E):
    """Return (orders, gens, table) for E(F_q), table: point-key -> exponent tuple."""
    N = E.order()
    gens = None
    try:
        gens = list(E.gens())
        orders = [Integer(g.order()) for g in gens]
        if prod(orders) != N:
            gens = None
    except Exception:
        gens = None
    if gens is None:
        A = E.abelian_group()
        gens = [A.gen(i).element() for i in range(len(A.invariants()))]
        orders = [Integer(g.order()) for g in gens]
        assert prod(orders) == N, 'generator orders do not multiply to |E|'
    elems = [(E(0), ())]
    for g, n in zip(gens, orders):
        mult = []
        cur = E(0)
        for i in range(int(n)):
            mult.append(cur)
            cur = cur + g
        elems = [(P + m, c + (i,)) for (P, c) in elems for i, m in enumerate(mult)]
    table = {}
    for P, c in elems:
        table[key(P)] = c
    assert len(table) == N, 'enumeration did not cover E(F_q)'
    return N, orders, table


def intersect(L, M, orders, r):
    """Sublattice of rows-lattice L (r x r) of c with c*M == 0 mod orders. LLL-reduced."""
    k = len(orders)
    LM = L * M
    B = LM.stack(diagonal_matrix(ZZ, [int(n) for n in orders]))
    ker = B.left_kernel().basis_matrix()
    X = ker.matrix_from_columns(list(range(r)))
    return (X * L).LLL()


def vec_in_K(v, M, orders):
    w = vector(ZZ, v) * M
    return all(int(w[j]) % int(orders[j]) == 0 for j in range(len(orders)))


def shifts(v, d):
    v = list(v)
    return [v[-i:] + v[:-i] if i else v[:] for i in range(d)]


def pick_u0(F, p, d, zeta, used_lams):
    while True:
        u0 = F.random_element()
        if u0 == 0:
            continue
        if u0.minpoly().degree() != 4:
            continue
        lam = u0 ** d
        if lam in (0, 1) or lam.minpoly().degree() != 4 or lam in used_lams:
            continue
        ok = True
        for j in range(d):
            x = zeta ** j * u0
            if x + 1 == 0:
                ok = False
                break
        if not ok:
            continue
        return u0, lam


def specialize(F, zeta, d, u0):
    lam = u0 ** d
    E = EllipticCurve(F, [0, 1 + lam, 0, lam, 0])
    pts = []
    for j in range(d):
        x = zeta ** j * u0
        y = x * (x + 1) ** (d // 2)
        pts.append(E(x, y))      # raises if not on the curve
    return E, pts


def sums_for(E, pts):
    S1 = E(0)
    S2 = E(0)
    for j, P in enumerate(pts):
        S1 = S1 + P
        S2 = S2 + (P if j % 2 == 0 else -P)
    return S1, S2


def torsion_multiplier(sums, maxm=8):
    m = 1
    while m <= maxm:
        if all((m * S).is_zero() for S in sums):
            return m
        m *= 2
    return None


# ---------------------------------------------------------------- per-p cell
def run_p(p):
    f = F_EXP
    d = p ** f + 1
    r = d
    q4 = p ** 4
    cell = {'p': p, 'f': f, 'd': d, 'r_sections': r}
    RESULT['cells'][str(p)] = cell
    val = {}
    RESULT['validation'][str(p)] = val
    t_cell = time.time()
    F4 = GF(q4, 'a')
    g4 = F4.multiplicative_generator()
    zeta = g4 ** ((q4 - 1) // d)
    assert zeta.multiplicative_order() == d
    plog('p=%d d=%d r=%d  F_{p^4} built' % (p, d, r))

    # ---- (1) symbolic verification of sections over F_{p^4}[u]
    R = PolynomialRing(F4, 'u')
    uu = R.gen()
    t_poly = uu ** d
    sym_ok = True
    for j in range(d):
        z = zeta ** j
        x = z * uu
        y = z * uu * (z * uu + 1) ** (d // 2)
        if y ** 2 != x * (x + 1) * (x + t_poly):
            sym_ok = False
            break
    val['sections_verified_symbolic_over_Fp4_u'] = sym_ok
    val['sections_over_Fp_u_only_for_zeta_pm1'] = True   # mu_d cap F_p^* = {+-1}
    val['d_divides_p4_minus_1'] = ((q4 - 1) % d == 0)
    plog('symbolic verification of all %d sections: %s' % (d, sym_ok))
    if not sym_ok:
        cell['error'] = 'section formula failed symbolic verification'
        save()
        return

    # ---- Ulmer rank formula
    cell['ulmer_rank_over_Fp4_u'] = ulmer_rank(d, q4)
    cell['ulmer_rank_over_Fp_u'] = ulmer_rank(d, p)
    cell['ulmer_rank_over_Fp2_u'] = ulmer_rank(d, p ** 2)
    cell['ulmer_predicted_dim_Kgeo'] = r - cell['ulmer_rank_over_Fp4_u']

    # ---- (2) optional exact function-field check of the two candidate relations (p=7 only, alarm-capped)
    cell['symbolic_relation_check'] = 'not attempted'
    if p == 7:
        try:
            alarm(420)
            KF = FunctionField(F4, 'u')
            uK = KF.gen()
            EK = EllipticCurve(KF, [0, 1 + uK ** d, 0, uK ** d, 0])
            ptsK = []
            for j in range(d):
                z = zeta ** j
                x = z * uK
                ptsK.append(EK(x, x * (x + 1) ** (d // 2)))

            def tree_sum(lst):
                while len(lst) > 1:
                    nxt = []
                    for i in range(0, len(lst) - 1, 2):
                        nxt.append(lst[i] + lst[i + 1])
                    if len(lst) % 2:
                        nxt.append(lst[-1])
                    lst = nxt
                return lst[0]
            s1 = tree_sum(list(ptsK))
            s2 = tree_sum([P if j % 2 == 0 else -P for j, P in enumerate(ptsK)])
            cancel_alarm()
            cell['symbolic_relation_check'] = {
                'sum_all_is_zero': bool(s1.is_zero()),
                'sum_all_is_2torsion': bool((2 * s1).is_zero()),
                'sum_signed_is_zero': bool(s2.is_zero()),
                'sum_signed_is_2torsion': bool((2 * s2).is_zero()),
            }
        except BaseException as ex:
            try:
                cancel_alarm()
            except Exception:
                pass
            cell['symbolic_relation_check'] = 'aborted: %s' % (repr(ex)[:200],)
        plog('function-field relation check:', cell['symbolic_relation_check'])

    # ---- (3) fibre loop: first fibre = primary; then more fibres for the intersection lattice
    used_lams = set()
    nfib_target = 60 if p == 7 else 70
    L_inter = identity_matrix(ZZ, r)
    sums_all = []                 # (S1,S2) per fibre, as booleans/torsion data
    sum_pts_checks = []           # collect points for multiplier test
    primary = {}
    geo_candidates_data = []      # lists of S1, S2 across fibres
    S1_list = []
    S2_list = []
    logN_sum = 0.0
    fibres_done = 0
    fib_info = []
    # NOTE: lambda = u0^d lies in F_{p^2} (d*(p^2-1) = p^4-1), and the section set depends only on lambda
    # (u0 -> eta*u0, eta in mu_d, permutes the sections). So distinct fibres <-> lambda in the (p^2-1)-element
    # subgroup of F_{p^4}^*, minus lambda=1.  Primary fibre: first lambda generating F_{p^2} over F_p.
    idxs = list(range(1, p ** 2 - 1))
    u0_all = [(g4 ** i) for i in idxs]
    prim = [u for u in u0_all if (u ** d).minpoly().degree() == 2]
    rest = [u for u in u0_all if u not in prim[:1]]
    from random import Random
    Random(20261022).shuffle(rest)
    u0_list = prim[:1] + rest
    nfib_target = min(nfib_target, len(u0_list))
    cell['num_distinct_lambda_available'] = len(u0_list)
    cell['lambda_in_Fp2_note'] = 'lambda=u0^d in F_{p^2}; fibres limited to p^2-2 distinct lambda'
    for it in range(nfib_target):
        if time.time() - t_cell > CELL_BUDGET_S[p] and it >= 3:
            RESULT['censored'].append('p=%d: fibre loop stopped at %d/%d by soft time budget' % (p, it, nfib_target))
            break
        u0 = u0_list[it]
        lam = u0 ** d
        used_lams.add(lam)
        E, pts = specialize(F4, zeta, d, u0)
        N, orders, table = group_data(E)
        k = len(orders)
        M = matrix(ZZ, r, k, [c for P in pts for c in table[key(P)]])
        S1, S2 = sums_for(E, pts)
        S1_list.append(S1)
        S2_list.append(S2)
        if it == 0:
            # primary single-fibre kernel K
            K1 = intersect(identity_matrix(ZZ, r), M, orders, r)
            idx = abs(K1.det())
            Mfull = M.stack(diagonal_matrix(ZZ, [int(n) for n in orders]))
            ed = [x for x in Mfull.elementary_divisors() if x != 0]
            quot = prod(ed)                      # |E / image|
            img_order = Integer(N) // Integer(quot)
            primary = {'E': E, 'M': M, 'orders': orders, 'N': N, 'K1': K1, 'idx': idx, 'img': img_order}
            cell['u0'] = str(u0)
            cell['lambda'] = str(lam)
            cell['Elambda_order'] = int(N)
            cell['Elambda_order_factored'] = str(factor(N))
            cell['Elambda_invariants'] = [int(n) for n in orders]
            cell['image_order'] = int(img_order)
            cell['image_index_in_E'] = int(quot)
            cell['kernel_index'] = int(idx)
            cell['kernel_index_equals_image_order'] = bool(idx == img_order)
            val['index_K_equals_image_order'] = bool(idx == img_order)
            cell['dim_K'] = int(K1.rank())
            K1b = K1
            try:
                K1b = K1.BKZ(block_size=20)
            except Exception:
                pass
            nrm = sorted([float(sqrt(sum(int(x) ** 2 for x in row))) for row in K1b.rows()])
            cell['K_LLL_norms_smallest10'] = nrm[:10]
            cell['K_LLL_norm_min'] = nrm[0]
            cell['K_LLL_norm_median'] = nrm[len(nrm) // 2]
            cell['K_LLL_norm_max'] = nrm[-1]
            cell['K_LLL_basis_first3'] = [[int(x) for x in K1b.row(i)] for i in range(min(3, r))]
            cell['K_elementary_divisors_nontrivial'] = [int(x) for x in K1.elementary_divisors() if x != 1]
            plog('p=%d fibre0: |E|=%s  invariants=%s  image=%s  idx=%s  norms[min,med,max]=%.2f,%.2f,%.2f' % (
                p, factor(N), orders, img_order, idx, nrm[0], nrm[len(nrm) // 2], nrm[-1]))
            L_inter = K1
            logN_sum += math.log10(int(idx)) if idx > 1 else 0.0
            save()
        else:
            L_inter = intersect(L_inter, M, orders, r)
        fibres_done += 1
        fib_info.append(int(N))
        if it % 10 == 0:
            plog('p=%d fibre %d done; min LLL norm of intersection %.2f' % (
                p, it, float(sqrt(sum(int(x) ** 2 for x in L_inter.row(0))))))

    plog('p=%d fibre loop done (%d fibres)' % (p, fibres_done))
    cell['fibres_used'] = fibres_done
    cell['fibre_group_orders_sample'] = fib_info[:10]

    # ---- (4) geometric relation candidates: all-ones (trace/norm) and sign character (e=2)
    m1 = torsion_multiplier(S1_list)
    m2 = torsion_multiplier(S2_list)
    cell['norm_relation_all_ones_multiplier'] = m1
    cell['sign_relation_multiplier'] = m2
    cell['norm_sum_is_zero_in_all_fibres'] = bool(all(S.is_zero() for S in S1_list))
    cell['signed_sum_is_zero_in_all_fibres'] = bool(all(S.is_zero() for S in S2_list))

    plog('p=%d starting F_p8 cross-check' % p)
    # extension-field cross-check (F_{p^8}) of the candidate relations
    ext = {'fibres': 0, 'all_ones_multiplier': None, 'sign_multiplier': None}
    try:
        F8 = GF(p ** 8, 'b')
        g8 = F8.multiplicative_generator()
        z8 = g8 ** ((p ** 8 - 1) // d)
        S1e, S2e = [], []
        for _ in range(12 if p == 7 else 5):
            while True:
                u0e = F8.random_element()
                if u0e == 0 or u0e ** d in (0, 1):
                    continue
                if any(z8 ** j * u0e + 1 == 0 for j in range(d)):
                    continue
                break
            Ee, ptse = specialize(F8, z8, d, u0e)
            a, b = sums_for(Ee, ptse)
            S1e.append(a)
            S2e.append(b)
        ext = {'fibres': len(S1e), 'all_ones_multiplier': torsion_multiplier(S1e),
               'sign_multiplier': torsion_multiplier(S2e)}
    except Exception as ex:
        ext['error'] = repr(ex)[:200]
    cell['F_p8_crosscheck'] = ext
    plog('candidate relation multipliers: all-ones=%s sign=%s ; F_p8 check=%s' % (m1, m2, ext))

    geo_vecs = []
    if m1 is not None and ext.get('all_ones_multiplier') is not None:
        geo_vecs.append([m1] * r)
    if m2 is not None and ext.get('sign_multiplier') is not None:
        geo_vecs.append([m2 * (1 if j % 2 == 0 else -1) for j in range(r)])
    Kgeo_rows = []
    for v in geo_vecs:
        Kgeo_rows.extend(shifts(v, d))
    dim_geo = matrix(ZZ, Kgeo_rows).rank() if Kgeo_rows else 0
    cell['dim_K_geo'] = int(dim_geo)
    cell['K_geo_generators'] = [v[:6] + ['...'] for v in geo_vecs]
    cell['K_geo_vector_norms'] = [float(sqrt(sum(x * x for x in v))) for v in geo_vecs]

    # K_geo subset K (primary fibre) and for every fibre via the intersection lattice
    M0 = primary['M']
    o0 = primary['orders']
    sub_primary = all(vec_in_K(w, M0, o0) for w in Kgeo_rows) if Kgeo_rows else True
    # membership in the multi-fibre intersection lattice
    sub_inter = True
    if Kgeo_rows:
        Lm = L_inter
        for w in geo_vecs:
            try:
                sol = Lm.solve_left(vector(ZZ, w))
                if any(x not in ZZ for x in sol):
                    sub_inter = False
            except Exception:
                sub_inter = False
    cell['Kgeo_subset_K'] = bool(sub_primary)
    cell['Kgeo_subset_K_all_fibres'] = bool(sub_inter)
    val['Kgeo_subset_K'] = bool(sub_primary and sub_inter)

    plog('p=%d starting intersection-lattice analysis' % p)
    # ---- (5) empirical geometric kernel from the multi-fibre intersection lattice
    rows_norm = [float(sqrt(sum(int(x) ** 2 for x in row))) for row in L_inter.rows()]
    idx_inter = abs(L_inter.det())
    gh = float(sqrt(r / (2 * pi * e).n())) * float(idx_inter) ** (1.0 / r) if idx_inter < 10 ** 300 else \
        float(sqrt(r / (2 * pi * e).n())) * float(10 ** (math.log10(int(idx_inter)) / r))
    srt = sorted(rows_norm)
    cell['inter_lattice_log10_index'] = math.log10(int(idx_inter))
    cell['inter_lattice_gaussian_heuristic_norm'] = gh
    cell['inter_lattice_smallest8_norms'] = srt[:8]
    cell['inter_lattice_median_norm'] = srt[len(srt) // 2]
    tiny = [row for row, nn in zip(L_inter.rows(), rows_norm) if nn < 0.05 * gh]
    cell['inter_lattice_tiny_vectors'] = len(tiny)
    cell['dim_K_geo_empirical'] = len(tiny)
    cell['inter_tiny_vector_examples'] = [[int(x) for x in v][:12] for v in tiny[:3]]
    # do the tiny vectors lie in the span of K_geo rows?
    if tiny and Kgeo_rows:
        Gm = matrix(ZZ, Kgeo_rows)
        span_ok = all(matrix(QQ, Gm).rank() == matrix(QQ, Gm.stack(matrix(ZZ, [v]))).rank() for v in tiny)
        cell['tiny_vectors_in_span_of_Kgeo'] = bool(span_ok)
    plog('intersection lattice (%d fibres): log10 index %.1f, GH norm %.1f, smallest norms %s, tiny=%d' % (
        fibres_done, cell['inter_lattice_log10_index'], gh, [round(x, 2) for x in srt[:5]], len(tiny)))

    # ---- (6) corank
    # K (single fibre) has full rank r; K_geo is spanned by the verified geometric relations.
    # K_geo subset K  =>  dim(K_geo cap K) = dim K_geo.
    dimK = cell['dim_K']
    corank = dimK - dim_geo
    cell['corank'] = int(corank)
    cell['corank_over_r'] = float(corank) / r
    k_inv = len(primary['orders'])
    cell['corank_modular'] = int(r - k_inv - dim_geo)       # relations needed beyond rank-k image & geometry
    cell['corank_empirical'] = int(r - len(tiny))
    cell['consistency_geo_vs_ulmer'] = bool(dim_geo == cell['ulmer_predicted_dim_Kgeo'])
    cell['consistency_empirical_vs_ulmer'] = bool(len(tiny) == cell['ulmer_predicted_dim_Kgeo'])
    cell['elapsed_cell_s'] = time.time() - t_cell
    print('CELLJSON ' + json.dumps({str(p): cell}, default=str), flush=True)
    plog('p=%d: dim_K=%d dim_K_geo=%d corank=%d (%.3f r)  ulmer-predicted dim_Kgeo=%d' % (
        p, dimK, dim_geo, corank, corank / float(r), cell['ulmer_predicted_dim_Kgeo']))
    save()


# ---------------------------------------------------------------- main
for p in PS:
    try:
        run_p(p)
    except BaseException as ex:
        RESULT['errors'][str(p)] = traceback.format_exc()[-3000:]
        RESULT['censored'].append('p=%d: aborted with %s' % (p, repr(ex)[:120]))
        plog('ERROR at p=%d: %s' % (p, repr(ex)))
    save()

# primary + verdict
fin = [p for p in PS if str(p) in RESULT['cells'] and 'corank' in RESULT['cells'][str(p)]]
if fin:
    pm = max(fin)
    RESULT['corank'] = RESULT['cells'][str(pm)]['corank']
    RESULT['corank_over_r'] = RESULT['cells'][str(pm)]['corank_over_r']
    RESULT['largest_finished_p'] = pm
    ratios = [RESULT['cells'][str(p)]['corank_over_r'] for p in fin]
    if len(fin) == len(PS) and all(x >= 0.5 for x in ratios):
        RESULT['verdict'] = 'kill'
    elif len(fin) == len(PS) and ratios[-1] < 0.1 and ratios[-1] < ratios[0]:
        RESULT['verdict'] = 'win'
    else:
        RESULT['verdict'] = 'inconclusive'
    RESULT['validation']['sections_verified_symbolic_all_p'] = bool(
        all(RESULT['validation'][str(p)].get('sections_verified_symbolic_over_Fp4_u') for p in fin))
    RESULT['validation']['Kgeo_subset_K_all_p'] = bool(
        all(RESULT['validation'][str(p)].get('Kgeo_subset_K') for p in fin))
save()
plog('done; verdict=%s corank=%s' % (RESULT['verdict'], RESULT['corank']))
