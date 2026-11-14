"""Summarise pass-2 hypothesis-test outputs and check pass-1 reproduction.  [pcc:unverified]"""
import json, glob, os, collections, sys
os.chdir(sys.argv[1])
rows_out = []
tot = collections.Counter()
for f in sorted(glob.glob('G_pass2/G_*.json')) + sorted(glob.glob('G_bits/G_*.json')):
    try: r = json.load(open(f))
    except Exception as e:
        print('skip', f, e); continue
    s = r['summary']; R = r['rows']
    nd = [x for x in R if x['target']=='nondec']; de = [x for x in R if x['target']=='dec']
    dn = [x for x in de if x['delta'] >= 0]
    ndw = [x['delta_weak'] for x in nd]; dew = [x['delta_weak'] for x in de if x['delta_weak'] is not None]
    rec = dict(kind=s['kind'], m=s['m'], size=s['size'], grid=s['grid'], max=s['max'],
               nondec=len(nd), nondec_eq_max=sum(x['delta']==x['max'] for x in nd),
               dec=len(de), dec_degenerate=len(de)-len(dn),
               dec_ge_generic=sum(x['delta']>=x['generic_pred'] for x in dn),
               weak_min=min(ndw+dew), weak_min_over_max=round(min(ndw+dew)/s['max'],3),
               weak_dev_min=min([x['delta_weak']-x['generic_weak'] for x in nd]+[x['delta_weak']-x['generic_weak'] for x in de if x['delta_weak'] is not None]),
               Hfrac_weak_min=round(min([x['H_frac_weak'] for x in nd]+[x['H_frac_weak'] for x in de if x['H_frac_weak'] is not None]),4),
               ctrl_rand=f"{s['ctrl_random']['eq_generic_pred']}/{s['ctrl_random']['n']}",
               ctrl_plant=f"{s['ctrl_planted']['eq_generic_pred']}/{s['ctrl_planted']['n']}", sec=s['sec'])
    rows_out.append(rec)
    for k_ in ('nondec','nondec_eq_max','dec','dec_degenerate','dec_ge_generic'):
        tot[k_] += rec[k_]
rows_out.sort(key=lambda r:(r['kind'], r['m'], r['size']))
for r in rows_out: print(r)
print(dict(tot))
mism = 0; comp = 0
for f in glob.glob('G_pass1/G_*.json'):
    g = f.replace('G_pass1','G_pass2')
    try: a = json.load(open(f)); b = json.load(open(g))
    except Exception: continue
    ra = [(x['target'], x.get('xR'), x['delta']) for x in a['rows'] if x['target'] in ('nondec','dec')]
    rb = [(x['target'], x.get('xR'), x['delta']) for x in b['rows'] if x['target'] in ('nondec','dec')]
    comp += len(ra); mism += sum(1 for u,v in zip(ra, rb) if u != v) + abs(len(ra)-len(rb))
print('pass1 vs pass2 strong delta rows compared', comp, 'mismatches', mism)
lines = ["| family | $m$ | size | grid | $\\rho$ | non-dec: $\\delta=\\rho$ | dec (degenerate) | non-degenerate dec: $\\delta\\ge$ generic | min $\\delta_{\\rm weak}/\\rho$ | min (weak − generic) | min $H(\\delta_{\\rm weak})/|W|^m$ | controls rand / planted |", "|---|---|---|---|---|---|---|---|---|---|---|---|"]
for r in rows_out:
    lines.append(f"| {r['kind']} | {r['m']} | {r['size']} | {r['grid']} | {r['max']} | {r['nondec_eq_max']}/{r['nondec']} | {r['dec']} ({r['dec_degenerate']}) | {r['dec_ge_generic']}/{r['dec']-r['dec_degenerate']} | {r['weak_min_over_max']} | {r['weak_dev_min']} | {r['Hfrac_weak_min']} | {r['ctrl_rand']} / {r['ctrl_plant']} |")
open('G_table.md','w').write("\n".join(lines)+"\n")
print("\n".join(lines))
json.dump({"instances": rows_out, "totals": dict(tot), "pass_reproduction": {"rows": comp, "mismatches": mism}}, open('G_summary.json','w'), indent=1)
