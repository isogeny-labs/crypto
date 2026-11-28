"""PROTOCOL.md section 4: lowest geometric-mean la_add+la_mul at 32771 among settings with zero
la_fallbacks (all dev curves), ties to the smaller EXCESS. Development data only."""
import glob, json, math, re, sys
out = {}
for f in sorted(glob.glob('dev/dev_W*_F*_E*.jsonl')):
    W, F, E = re.match(r'dev/dev_W(\d+)_F([\d.]+)_E(\d+)\.jsonl', f).groups()
    rows = [json.loads(l) for l in open(f)]
    runs = [r for r in rows if r.get('kind') == 'run']
    sge = [r for r in runs if r['arm'] == 'Q_diem_2lp_sfb_sge']
    sfb = [r for r in runs if r['arm'] == 'Q_diem_2lp_sfb']
    ok = all(r['certified'] and r['recovered'] == r['t_secret'] for r in runs)
    fb = sum(r.get('la_fallbacks') or 0 for r in sge)
    big = [r for r in sge if r['p'] == 32771]
    gm = math.exp(sum(math.log(r['la_add'] + r['la_mul']) for r in big) / max(len(big), 1)) if big else None
    gm_sfb = math.exp(sum(math.log(r['la_add'] + r['la_mul']) for r in sfb if r['p'] == 32771) / max(1, sum(r['p'] == 32771 for r in sfb)))
    out[f'{W},{F},{E}'] = dict(W=int(W), FILL=float(F), EXCESS=int(E), n_sge=len(sge), n_32771=len(big),
                               all_certified=ok, la_fallbacks=fb, gm_la_32771=gm, gm_la_32771_sfb=gm_sfb)
elig = [v for v in out.values() if v['la_fallbacks'] == 0 and v['all_certified'] and v['n_32771'] == 10]
pick = min(elig, key=lambda v: (v['gm_la_32771'], v['EXCESS'])) if elig else None
json.dump(dict(settings=out, pick=pick), open('dev/choice.json', 'w'), indent=1)
for k, v in out.items(): print(k, v['n_sge'], v['la_fallbacks'], v['all_certified'], v['gm_la_32771'] and round(v['gm_la_32771']), round(v['gm_la_32771_sfb']))
print('PICK', pick)
