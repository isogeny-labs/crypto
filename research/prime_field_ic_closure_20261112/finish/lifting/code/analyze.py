"""Aggregate toy-xedni JSONL runs into tables (markdown + JSON).  [pcc:unverified]"""
import sys, os, json, glob, math, statistics
sys.path.insert(0, os.path.dirname(__file__))
from bounds import log2_C_sharp, log2_C_lean, log2_eps

OUT = os.path.join(os.path.dirname(__file__), '..', 'out')


def binom_cdf(k, n, p):
    if p <= 0:
        return 1.0
    if p >= 1:
        return 0.0 if k < n else 1.0
    s = 0.0
    lp, lq = math.log(p), math.log1p(-p)
    for i in range(0, k + 1):
        s += math.exp(math.lgamma(n + 1) - math.lgamma(i + 1) - math.lgamma(n - i + 1) + i * lp + (n - i) * lq)
    return min(1.0, s)


def cp_upper(k, n, alpha=0.05):
    if k >= n:
        return 1.0
    lo, hi = k / n, 1.0
    for _ in range(80):
        mid = (lo + hi) / 2
        if binom_cdf(k, n, mid) > alpha / 2:
            lo = mid
        else:
            hi = mid
    return hi


def cp_lower(k, n, alpha=0.05):
    if k == 0:
        return 0.0
    lo, hi = 0.0, k / n
    for _ in range(80):
        mid = (lo + hi) / 2
        # P[X >= k] = 1 - cdf(k-1)
        if 1 - binom_cdf(k - 1, n, mid) < alpha / 2:
            lo = mid
        else:
            hi = mid
    return lo


def load(path):
    L = [json.loads(l) for l in open(path)]
    return L[0], L[1:]


def summarize(meta, recs):
    q_mean = statistics.mean(t['q'] for t in meta['targets'])
    p_mean = statistics.mean(t['p'] for t in meta['targets'])
    r = meta['r']
    used = [x for x in recs if x.get('status') in ('ok', 'identical-lifts')]
    skipped = {}
    for x in recs:
        st = x.get('status')
        if st not in ('ok', 'identical-lifts'):
            skipped[st] = skipped.get(st, 0) + 1
    n = len(used)
    rel = [x for x in used if x.get('rel')]
    triv = [x for x in rel if x.get('rel_trivial')]
    nontriv = [x for x in rel if not x.get('rel_trivial')]
    succ = [x for x in used if x.get('success')]
    succ_nt = [x for x in succ if not x.get('rel_trivial')]
    vfail = [x for x in used if x.get('verified') in ('exact-FAIL', 'mod-FAIL')]
    e1 = sum(1 for x in used if x.get('e1'))
    e2 = sum(1 for x in used if x.get('e2'))
    coll = sum(1 for x in used if x.get('collision'))
    noncoll = [x for x in used if not x.get('collision')]
    rel_noncoll = [x for x in noncoll if x.get('rel')]
    e1_nc = sum(1 for x in used if x.get('e1') and not x.get('collision'))
    patt = {}
    for x in nontriv:
        v = [a for a in x['rel'] if a]
        key = f"supp{len(v)}_max{max(abs(a) for a in v)}"
        patt[key] = patt.get(key, 0) + 1
    xc = [x['xcert_log2'] for x in used if x.get('status') == 'ok' and not x.get('rel') and x.get('xcert_log2') is not None]
    # height statistics only on trials whose lifted points are independent (no relation found and
    # Gram matrix not numerically singular): for dependent points lambda_G is 0 by definition.
    hs_all = [x for x in used if x.get('heights')]
    hs = [x['heights'] for x in hs_all if not x.get('rel') and x['heights'].get('lambda_vec') not in ('singular',)
          and x['heights'].get('lambda_G', 0) > 0]
    h_dep = sum(1 for x in hs_all if x.get('rel') or x['heights'].get('lambda_vec') == 'singular')
    # Weyl: lambda_min(G_true) >= lambda_min(G_est) - 1.5 r eps  (entrywise truncation error <= 1.5 eps)
    h_cert = sum(1 for x in hs_all if x['heights']['min_eig'] > 1.5 * r * x['heights']['eps'])
    h_cert_norel = sum(1 for x in hs_all if not x.get('rel') and x['heights']['min_eig'] > 1.5 * r * x['heights']['eps'])
    h_norel = sum(1 for x in hs_all if not x.get('rel'))
    s = dict(arm=meta['arm'], bits=meta['bits'], r=r, p_mean=p_mean, q_mean=q_mean, n=n, skipped=skipped,
             relations=len(rel), rel_trivial=len(triv), rel_nontrivial=len(nontriv),
             nontrivial_vectors=[x['rel'] for x in nontriv][:10],
             successes=len(succ), successes_nontrivial=len(succ_nt), verify_failures=len(vfail),
             e1_noncollision=e1_nc, nontrivial_patterns=patt,
             realised_given_e1nc=(len(nontriv) / e1_nc) if e1_nc else None,
             collision=coll, collision_pred=r * (r - 1) / q_mean * n,
             noncollision_trials=len(noncoll), noncollision_relations=len(rel_noncoll),
             e1=e1, e1_pred=(3 ** r - 1) / 2 / q_mean * n, e2=e2, e2_pred=min(1.0, (5 ** r - 1) / 2 / q_mean) * n,
             succ_rate=len(succ) / n if n else None,
             succ_ub95=cp_upper(len(succ), n) if n else None,
             succ_nt_ub95=cp_upper(len(succ_nt), n) if n else None,
             xcert_log2_min=min(xc) if xc else None, xcert_log2_median=statistics.median(xc) if xc else None)
    if meta['arm'] in ('PC', 'PS', 'PD', 'POS'):
        pl = [x for x in used if x.get('status') == 'ok' and 'planted_realized' in x]
        k = sum(1 for x in pl if x['planted_realized'])
        s.update(planted_n=len(pl), planted_realized=k, planted_rate=k / len(pl) if pl else None,
                 planted_ci=(cp_lower(k, len(pl)), cp_upper(k, len(pl))) if pl else None)
        if meta['arm'] in ('PC', 'POS'):
            agree = sum(1 for x in pl if x.get('int_collinear') == x['planted_realized'])
            s.update(collinear_agree=agree)
            # planted relation must be detected by the sieve whenever realized
            det = sum(1 for x in pl if x['planted_realized'] and x.get('rel'))
            s.update(planted_detected=det)
    if hs:
        def med(key):
            v = [h[key] for h in hs if h.get(key) is not None]
            return statistics.median(v) if v else None
        rhos = [h['rho_G'] for h in hs if h.get('rho_G')]
        s.update(h_n=len(hs), h_dependent_excluded=h_dep, h_trials=len(hs_all), h_rank_certified=h_cert,
                 h_norel=h_norel, h_rank_certified_norel=h_cert_norel, rho_G_median=med('rho_G'), rho_G_max=max(rhos) if rhos else None,
                 H_median=med('H'), hmin_median=med('hmin'), logDW_median=med('log_absDW'),
                 lang_hmin_median=med('lang_hmin'), lang_lamG_median=med('lang_lambdaG'),
                 lang_lamG_min=min(h['lang_lambdaG'] for h in hs),
                 H_over_logDW_median=med('H_over_logDW'), eps_max=max(h['eps'] for h in hs),
                 min_eig_min=min(h['min_eig'] for h in hs))
        for rho_key in ('rho_G_median', 'rho_G_max'):
            rho = s[rho_key]
            if rho:
                ls = log2_C_sharp(r, rho)
                s[f'log2_eps_sharp_{rho_key}'] = log2_eps(r, ls, math.log2(q_mean))
    return s


def main():
    rows = []
    for f in sorted(glob.glob(os.path.join(OUT, '*_b*_r*.jsonl'))):
        meta, recs = load(f)
        if not recs:
            continue
        rows.append(summarize(meta, recs))
    json.dump(rows, open(os.path.join(OUT, 'summary.json'), 'w'), indent=1)
    # markdown tables
    md = []
    md.append('| arm | bits | r | q~ | n | collisions (pred) | e1 (pred) | rel triv/nontriv | succ (nontriv) | succ 95% UB | log2 X_cert min/med | rho_G med/max | log2 eps(rho_G med) |')
    md.append('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    for s in rows:
        if s['arm'] != 'X':
            continue
        md.append(f"| X | {s['bits']} | {s['r']} | {s['q_mean']:.0f} | {s['n']} | {s['collision']} ({s['collision_pred']:.1f}) | "
                  f"{s['e1']} ({s['e1_pred']:.1f}) | {s['rel_trivial']}/{s['rel_nontrivial']} | {s['successes']} ({s['successes_nontrivial']}) | "
                  f"{s['succ_ub95']:.2e} | {s['xcert_log2_min'] if s['xcert_log2_min'] is None else round(s['xcert_log2_min'],1)}/"
                  f"{s['xcert_log2_median'] if s['xcert_log2_median'] is None else round(s['xcert_log2_median'],1)} | "
                  f"{(s.get('rho_G_median') or 0):.3f}/{(s.get('rho_G_max') or 0):.3f} | {(s.get('log2_eps_sharp_rho_G_median') or float('nan')):.1f} |")
    md.append('')
    md.append('Honest arm, decomposition of the non-trivial relations (support >= 3 or a coefficient of size >= 2):')
    md.append('')
    md.append('| bits | r | trials with a box-1 mod-q relation but no collision | non-trivial char-0 relations | ratio | patterns | false sieve candidates rejected |')
    md.append('|---|---|---|---|---|---|---|')
    for s in rows:
        if s['arm'] != 'X':
            continue
        rat = s['realised_given_e1nc']
        md.append(f"| {s['bits']} | {s['r']} | {s['e1_noncollision']} | {s['rel_nontrivial']} | {('%.3f' % rat) if rat is not None else '-'} | "
                  f"{', '.join(f'{k}:{v}' for k, v in sorted(s['nontrivial_patterns'].items())) or '-'} | {s['verify_failures']} |")
    md.append('')
    md.append('| arm | bits | r | p~ | planted n | realized | rate | 95% CI | rate*p | int-collinear agree | detected |')
    md.append('|---|---|---|---|---|---|---|---|---|---|---|')
    for s in rows:
        if s['arm'] == 'X':
            continue
        ci = s.get('planted_ci') or (float('nan'), float('nan'))
        md.append(f"| {s['arm']} | {s['bits']} | {s['r']} | {s['p_mean']:.0f} | {s.get('planted_n')} | {s.get('planted_realized')} | "
                  f"{(s.get('planted_rate') or 0):.2e} | [{ci[0]:.1e}, {ci[1]:.1e}] | {(s.get('planted_rate') or 0) * s['p_mean']:.3f} | "
                  f"{s.get('collinear_agree', '-')} | {s.get('planted_detected', '-')} |")
    md.append('')
    md.append('| bits | r | heights n | H med | hmin med | log|Delta_W| med (nats) | H/log|Delta_W| | lambda_G/log|Delta_W| med (min) | rho_G med (max) | trunc eps max | min eigenvalue min |')
    md.append('|---|---|---|---|---|---|---|---|---|---|---|')
    for s in rows:
        if s['arm'] != 'X' or not s.get('h_n'):
            continue
        md.append(f"| {s['bits']} | {s['r']} | {s['h_n']} | {s['H_median']:.2f} | {s['hmin_median']:.2f} | {s['logDW_median']:.0f} | "
                  f"{s['H_over_logDW_median']:.4f} | {s['lang_lamG_median']:.4f} ({s['lang_lamG_min']:.4f}) | {s['rho_G_median']:.3f} ({s['rho_G_max']:.3f}) | "
                  f"{s['eps_max']:.3f} | {s['min_eig_min']:.3f} |")
    open(os.path.join(OUT, 'tables.md'), 'w').write('\n'.join(md) + '\n')
    print('\n'.join(md))


if __name__ == '__main__':
    main()
