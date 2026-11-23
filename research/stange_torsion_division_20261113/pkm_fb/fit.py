"""Fit per-target msolve cost ~ |V|^alpha for each grid and encoding; bootstrap over targets.
Reads runs/*.json (pkm_tower.py) and runs/e2e.jsonl (pkm_e2e.py); writes fits.json, fit_table.md,
pkm_fb.{png,svg,pdf}. Noise floor: a cell is excluded from the fit if its median is < 3 ms
(same rule as ../growth_fit). Bootstrap: 4000 replicates, resample targets within each |V|,
recompute medians and the OLS slope; seed 20261115."""
import json, glob, os, statistics as st, random, math, re
import numpy as np
import matplotlib; matplotlib.use('Agg'); import matplotlib.pyplot as plt
os.chdir(os.path.dirname(os.path.abspath(__file__)))
FLOOR = 3e-3; NB = 4000; rng = random.Random(20261115)
grids = {'m2_pkm': 'runs/m2_k*.json', 'm3_pkm': 'runs/m3_k*.json', 'm2_fixp': 'runs/fixp_k*.json',
         'm2_isog': 'runs/iso_m2_k*.json', 'm3_isog': 'runs/iso_m3_k*.json'}
cells = {}   # (grid, arm) -> {n: {'times': [...], 'p':, 'deg': set, 'dims': [...]}}
for gname, pat in grids.items():
    for f in sorted(glob.glob(pat)):
        try:
            d = json.load(open(f))
        except ValueError:
            print('# skipped (empty or running):', f); continue
        for arm in d['arms']:
            ts = [r['arms'][arm]['time_s'] for r in d['records'] if 'time_s' in r['arms'].get(arm, {})]
            fails = [r['arms'][arm] for r in d['records'] if 'fail' in r['arms'].get(arm, {})]
            dg = sorted({r['arms'][arm]['max_step_deg'] for r in d['records'] if r['arms'].get(arm, {}).get('max_step_deg') is not None})
            if not ts and not fails:
                continue
            cells.setdefault((gname, arm), {})[d['n']] = {'times': ts, 'p': d['p'], 'deg': dg, 'fails': len(fails), 'reps': d.get('reps', 3),
                                                          'nvars': next((r['arms'][arm]['nvars'] for r in d['records'] if 'nvars' in r['arms'].get(arm, {})), None)}
def ols(x, y):
    x = np.log(x); y = np.log(y); A_ = np.vstack([x, np.ones_like(x)]).T
    sl, ic = np.linalg.lstsq(A_, y, rcond=None)[0]; return sl, ic
out = {'floor_s': FLOOR, 'bootstrap': NB, 'fits': {}, 'cells': {}}
rows = []
for (g, arm), cs in sorted(cells.items()):
    ns = sorted(cs)
    out['cells'][f'{g}/{arm}'] = {n: {'median_s': st.median(c['times']) if c['times'] else None, 'targets_ok': len(c['times']), 'fails': c['fails'],
                                      'p': c['p'], 'max_step_deg': c['deg'], 'nvars': c['nvars'], 'reps': c['reps']} for n, c in cs.items()}
    use = [n for n in ns if cs[n]['times'] and st.median(cs[n]['times']) >= FLOOR]
    excl = [n for n in ns if n not in use]
    if len(use) < 3:
        out['fits'][f'{g}/{arm}'] = {'used_n': use, 'excluded_n': excl, 'alpha': None}; continue
    meds = [st.median(cs[n]['times']) for n in use]
    a0, _ = ols(use, meds)
    bs = []
    for _ in range(NB):
        mm = [st.median([rng.choice(cs[n]['times']) for _ in cs[n]['times']]) for n in use]
        bs.append(ols(use, mm)[0])
    bs.sort(); lo, hi = bs[int(0.025 * NB)], bs[int(0.975 * NB) - 1]
    last2 = math.log(meds[-1] / meds[-2]) / math.log(use[-1] / use[-2])
    out['fits'][f'{g}/{arm}'] = {'used_n': use, 'excluded_n': excl, 'alpha': a0, 'ci95': [lo, hi], 'last_two_slope': last2,
                                 'targets_per_n': [len(cs[n]['times']) for n in use]}
    rows.append(f"| {g} | {arm} | {use[0]}-{use[-1]} ({len(use)} sizes) | {a0:.2f} | [{lo:.2f}, {hi:.2f}] | {last2:.2f} | {', '.join(str(x) for x in excl) or '-'} |")
# ratio table T2/D and T4/D at matched (grid, n)
ratio_rows = []
for g in grids:
    if (g, 'D') not in cells: continue
    for n in sorted(cells[(g, 'D')]):
        cD = cells[(g, 'D')][n]
        if not cD['times']: continue
        r = [f"{st.median(cD['times'])*1e3:.2f} ms (d{','.join(map(str, cD['deg']))})"]
        for arm in ('T4', 'T2', 'I2'):
            c = cells.get((g, arm), {}).get(n)
            if c and c['times']:
                r.append(f"{st.median(c['times'])*1e3:.2f} ms (d{','.join(map(str, c['deg']))}), x{st.median(c['times'])/st.median(cD['times']):.1f}")
            elif c and c['fails']:
                r.append('fail')
            else:
                r.append('-')
        ratio_rows.append(f"| {g} | {n} | {cD['p']} | {len(cD['times'])} | " + ' | '.join(r) + ' |')
# e2e
e2e = [json.loads(l) for l in open('runs/e2e.jsonl')] if os.path.exists('runs/e2e.jsonl') else []
e2e_rows = []
for kk in sorted({e['k'] for e in e2e}):
    for arm in ('D', 'T2'):
        es = [e for e in e2e if e['k'] == kk and e['arm'] == arm]
        if not es: continue
        tg = st.mean(e['ic']['targets'] for e in es); ad = st.mean(e['ic']['walk_and_sign_adds'] for e in es)
        mss = st.mean(e['ic']['msolve_wall_s_total'] for e in es); rho = st.mean(e['rho_store_neg']['ops_mean'] for e in es)
        rhos = st.mean(e['rho_store_neg']['python_wall_s_mean'] for e in es)
        e2e_rows.append(f"| {kk} | {es[0]['n']} | {', '.join(str(e['p']) for e in es)} | {arm} | {len(es)} | {st.mean(e['fb_size'] for e in es):.1f} | {tg:.0f} | {ad:.0f} | {rho:.1f} | {ad/rho:.1f} | {mss:.2f} | {rhos*1e3:.2f} ms | {mss/rhos:.0f} |")
        assert all(e['ic']['log_ok'] for e in es)
out['e2e_runs'] = len(e2e)
json.dump(out, open('fits.json', 'w'), indent=1, default=str)
with open('fit_table.md', 'w') as f:
    f.write("Per-target msolve time ~ |V|^alpha (wall s minus baseline; median over targets). [pcc:unverified]\n\n")
    f.write("| grid | arm | |V| used | alpha | 95% bootstrap | slope of last two | excluded (<3 ms or failed) |\n|---|---|---|---|---|---|---|\n" + "\n".join(rows) + "\n\n")
    f.write("Median per-target time, (max F4 step degree), ratio to D. [pcc:unverified]\n\n| grid | |V| | p | targets | D | T4 | T2 | I2 |\n|---|---|---|---|---|---|---|---|\n" + "\n".join(ratio_rows) + "\n\n")
    if e2e_rows:
        f.write("End-to-end toy IC (m=2, PKM subgroup base) vs rho_store_neg, means over seeds. [pcc:unverified]\n\n| k | |V| | p | arm | seeds | |F| | targets | IC adds | rho adds | adds ratio | IC msolve s | rho wall | wall ratio |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|\n" + "\n".join(e2e_rows) + "\n")
print(open('fit_table.md').read())
# plot
fig, axs = plt.subplots(1, 3, figsize=(15, 4.6))
col = {'D': '#1f77b4', 'T2': '#d62728', 'T4': '#ff7f0e', 'I2': '#2ca02c'}; mk = {'m2_pkm': 'o', 'm3_pkm': 's', 'm2_fixp': '^', 'm2_isog': 'D', 'm3_isog': 'D'}
for ax, gs, title in ((axs[0], ['m2_pkm', 'm2_fixp', 'm2_isog'], 'm = 2 (S3)'), (axs[1], ['m3_pkm', 'm3_isog'], 'm = 3 (S4)')):
    for g in gs:
        for arm in ('D', 'T4', 'T2', 'I2'):
            cs = cells.get((g, arm))
            if not cs: continue
            ns = sorted(n for n in cs if cs[n]['times']); md = [st.median(cs[n]['times']) for n in ns]
            fl = [st.median(cs[n]['times']) < FLOOR for n in ns]
            ax.scatter([n for n, f_ in zip(ns, fl) if not f_], [m for m, f_ in zip(md, fl) if not f_], color=col[arm], marker=mk[g])
            ax.scatter([n for n, f_ in zip(ns, fl) if f_], [max(m, 1e-5) for m, f_ in zip(md, fl) if f_], facecolors='none', edgecolors=col[arm], marker=mk[g])
            fit = out['fits'].get(f'{g}/{arm}', {})
            if fit.get('alpha') is not None:
                u = fit['used_n']; m_ = [st.median(cs[n]['times']) for n in u]; sl, ic = ols(u, m_)
                xx = np.array([u[0], u[-1]]); ax.plot(xx, np.exp(ic) * xx ** sl, color=col[arm], ls={'m2_fixp': ':', 'm2_isog': '--', 'm3_isog': '--'}.get(g, '-'),
                                                     label=f"{arm} {g}: alpha {sl:.2f} [{fit['ci95'][0]:.2f},{fit['ci95'][1]:.2f}]")
    ax.axhline(FLOOR, color='grey', lw=0.8, ls='--'); ax.set_xscale('log', base=2); ax.set_yscale('log')
    ax.set_xlabel('|V| (factor-base subgroup size)'); ax.set_ylabel('msolve per-target wall s (minus baseline)'); ax.set_title(title); ax.legend(fontsize=7)
ax = axs[2]
for g in ('m2_pkm', 'm3_pkm', 'm2_isog', 'm3_isog'):
    for arm in ('D', 'T4', 'T2', 'I2'):
        cs = cells.get((g, arm))
        if not cs: continue
        ns = sorted(n for n in cs if cs[n]['deg']); ax.plot(ns, [max(cs[n]['deg']) for n in ns], marker=mk[g], color=col[arm], label=f'{arm} {g}')
ax.set_xscale('log', base=2); ax.set_yscale('log'); ax.set_xlabel('|V|'); ax.set_ylabel('max F4 step degree (msolve -v 2)'); ax.set_title('step degree'); ax.legend(fontsize=7)
fig.suptitle('PKM composed-map encodings (T2, T4: p-1 subgroup towers; I2: 2-isogeny tower) vs dense univariate D on the same factor base; exploratory, [pcc:unverified]', fontsize=9)
fig.tight_layout()
for ext in ('png', 'svg', 'pdf'):
    fig.savefig(f'pkm_fb.{ext}', dpi=120)
