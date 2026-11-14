import json, sys, statistics as st, glob
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from bounds import modelX_cramer
rows = []
for f in sorted(glob.glob(sys.argv[1] + '/mestre_*.jsonl')) + sorted(glob.glob(sys.argv[1] + '/lift_*.jsonl')):
    data = [json.loads(l) for l in open(f)]
    if not data: continue
    tag = f.rsplit('/', 1)[1].replace('.jsonl', '')
    if tag.startswith('mestre'):
        ctrl = max(max(d['ctrl']['dbl_rel'], d['ctrl']['par_rel']) for d in data)
        for s in (7, 8, 9):
            cs = [c for d in data for c in d['cases'] if c['s'] == s]
            good = [c for c in cs if 'rho_G' in c]
            deg = len(cs) - len(good)
            if not good:
                print(tag, s, 'no case (no extra points searched or all degenerate)', deg); continue
            r = sorted(c['rho_G'] for c in good)
            relerr = max(c['maxerr'] / c['lambda_G'] for c in good)
            med, worst = st.median(r), r[-1]
            k = s + 1
            mm, mw = 128 - modelX_cramer(k, med), 128 - modelX_cramer(k, worst)
            # propagate height error into rho (worst case): rho*(1+relerr)/(1-relerr)
            wcorr = worst * (1 + relerr) / max(1e-9, 1 - relerr)
            mwc = 128 - modelX_cramer(k, wcorr)
            row = dict(src=tag, s=s, k=k, n=len(good), degenerate_dropped=deg, rho_min=r[0], rho_med=med, rho_max=worst,
                       max_rel_height_err=relerr, rho_max_errcorr=wcorr, margin_med=mm, margin_worst=mw, margin_worst_errcorr=mwc,
                       ctrl_max_rel=ctrl, H_med=st.median(c['H'] for c in good), lam_med=st.median(c['lambda_G'] for c in good))
            rows.append(row)
            print(f"{tag:14s} s={s} k={k} n={len(good):3d} drop={deg:2d} rho min/med/max {r[0]:.3f}/{med:.3f}/{worst:.3f} relerr {relerr:.1e} margin(med) {mm:+.1f} margin(worst) {mw:+.1f} (err-corrected {mwc:+.1f}) ctrl {ctrl:.1e} H~{row['H_med']:.1f}")
    else:
        good = [d for d in data if 'rho_G' in d]
        if not good: continue
        r = sorted(d['rho_G'] for d in good)
        relerr = max(d['maxerr'] / d['lambda_G'] for d in good)
        print(f"{tag:14s} (7-coef xedni lifts, k=6 independent) n={len(good)} pbits={good[0]['pbits']} rho min/med/max {r[0]:.4f}/{st.median(r):.4f}/{r[-1]:.4f} relerr {relerr:.1e} H~{st.median(d['H'] for d in good):.1f} red_ok={all(d['red_ok'] for d in good)}")
        rows.append(dict(src=tag, pbits=good[0]['pbits'], n=len(good), rho_min=r[0], rho_med=st.median(r), rho_max=r[-1], max_rel_height_err=relerr))
json.dump(rows, open(sys.argv[1] + '/rho_summary.json', 'w'), indent=1)
