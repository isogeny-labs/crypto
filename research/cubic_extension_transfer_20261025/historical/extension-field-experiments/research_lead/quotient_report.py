"""Validate saved quotient experiments and build the current research report."""
from collections import Counter,defaultdict
from datetime import datetime,timezone
from pathlib import Path
import hashlib,html,json,os,statistics

ROOT=Path(__file__).resolve().parent;OUT=ROOT/'results'


def main():
    sweep=json.loads((OUT/'quotient_sweep.json').read_text())
    obstruction=json.loads((OUT/'obstruction_probe.json').read_text())
    points=json.loads((OUT/'theta_point_probe.json').read_text())
    regression=json.loads((OUT/'zero_test_regression.json').read_text())
    large_verification=json.loads((OUT/'large_source_verification.json').read_text())
    assert large_verification['status']=='passed'
    hunt=json.loads((OUT/'coverage_hunt.json').read_text())
    larger=[json.loads(p.read_text()) for p in sorted(OUT.glob('large_instance_*.json'))]
    curves=sweep['curves'];assert len(curves)==23 and all(r['status']=='passed' for r in curves)
    assert sweep['constructor_sha256']==hashlib.sha256((ROOT/'quotient_theta.py').read_bytes()).hexdigest()
    assert len(obstruction['curves'])==23
    matched=0;counts=defaultdict(list)
    for row,obs in zip(curves,obstruction['curves']):
        assert row['input_index']==obs['input_index'] and row['p']==obs['p']
        assert len(row['kernels'])==12
        lookup={tuple(sorted(k['quartic_coefficients'].items())):k for k in row['kernels']}
        for k in obs['kernels']:
            target=lookup[tuple(sorted(k['quartic_coefficients'].items()))]
            assert k['predicts_base_field_jacobian']==target['quotient_is_base_field_jacobian']
            assert target['smooth'] and target['bitangents_verified']==28 and target['base_field_descent']
            assert target['N1']==row['p']+1 and target['N2']==row['p']**2+1
            matched+=1
        obs['independent_order_comparison_passed']=True
        n=sum(k['quotient_is_base_field_jacobian'] for k in row['kernels']);assert n>0
        counts[row['p']].append(n)
    # Finalize comparisons that were pending when the faster chi18 run finished.
    (OUT/'obstruction_probe.json').write_text(json.dumps(obstruction,indent=2)+'\n')
    assert points['distinct_projective_images']==points['expected_distinct_images']==187
    assert points['associated_jacobian_order_certificate']['jacobian_order']==373
    assert regression['status']=='passed'
    good=sum(map(sum,counts.values()));total=12*len(curves)
    assert (good,total)==(152,276)
    hcounts=Counter(r['usable_quotient_count'] for r in hunt['curves'])
    strata=[{'p':p,'inputs':len(ns),'quartics':12*len(ns),'usable':sum(ns),
             'obstructed':12*len(ns)-sum(ns),'minimum_usable_per_input':min(ns),
             'maximum_usable_per_input':max(ns)} for p,ns in sorted(counts.items())]
    summary={'created_utc':datetime.now(timezone.utc).isoformat(),'new_inputs':23,
             'smooth_base_field_quartics':total,'usable_jacobian_quotients':good,
             'obstructed_quotients':total-good,'inputs_with_usable_quotients':23,
             'independent_chi18_matches':matched,'strata':strata,
             'median_chi18_evaluation_seconds':statistics.median(k['obstruction_evaluation_seconds'] for r in obstruction['curves'] for k in r['kernels']),
             'large_source_certificates_reverified':True,
             'coverage_hunt_curves':len(hunt['curves']),'coverage_hunt_histogram':dict(sorted(hcounts.items())),
             'coverage_hunt_all_obstructed':hcounts.get(0,0),
             'larger_instances':[{'p':r['p'],'r':r['r'],'r_bits':r['r_bits'],
                                  'predicted_usable':sum(k['predicts_base_field_jacobian'] for k in r['kernels']),
                                  'seconds':r['seconds']} for r in larger],
             'source_hashes':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(ROOT.glob('*.py'))},
             'limits':['No general conversion of quotient Weil coordinates to target divisors.',
                       'No new target DLP solved or end-to-end speedup measured.',
                       'No universal usable-quotient theorem or priority claim.',
                       'Larger instances use chi18; independent Jacobian orders were certified only in the small-field sweep.']}
    (OUT/'quotient_summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    os.environ.setdefault('MPLCONFIGDIR','/tmp/extension-field-quotient-mpl')
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    fig,ax=plt.subplots(figsize=(8.5,4.7))
    labels=[f"{r['p']}\n({r['inputs']} inputs)" for r in strata]
    ax.bar(labels,[r['usable'] for r in strata],label='Jacobian over F_p',color='#207467')
    ax.bar(labels,[r['obstructed'] for r in strata],bottom=[r['usable'] for r in strata],
           label='Quadratic twist obstruction',color='#ce9052')
    ax.set_xlabel('Base prime p');ax.set_ylabel('Constructed quotients')
    ax.set_title('276 new quartics: 152 usable quotients, 124 obstructed')
    ax.grid(axis='y',alpha=.2);ax.set_axisbelow(True);ax.legend();fig.tight_layout()
    fig.savefig(OUT/'quotient_counts.svg');fig.savefig(OUT/'quotient_counts.png',dpi=150);plt.close(fig)
    table='\n'.join(f"| {s['p']} | {s['inputs']} | {s['quartics']} | {s['usable']} | {s['obstructed']} | {s['minimum_usable_per_input']}–{s['maximum_usable_per_input']} |" for s in strata)
    largetext='\n'.join(f"- p={r['p']}, prime r={r['r']} ({r['r_bits']} bits): {sum(k['predicts_base_field_jacobian'] for k in r['kernels'])}/12 predicted usable; one smooth quartic reconstructed and all 28 bitangents checked. Total observed time {r['seconds']:.2f} s, including source generation." for r in larger)
    first=curves[0]['kernels'][0]
    text=f'''# Explicit quotients after one 3-isogeny: results

The construction stage succeeded on all **23 newly eligible inputs** from the
earlier screen. Their 276 kernels produced **276 smooth plane quartics over the
base field**. Of the corresponding polarized quotients, **152 are base-field
Jacobians and 124 have the quadratic twist obstruction**. Every input has at
least one usable quotient; the observed range is 2–11 out of 12.

This is a working construction/coverage result, not a new end-to-end ECDLP
solver. The remaining implementation step is to invert general quotient Weil
coordinates into target divisors, recover signs consistently, and run target
index calculus with all costs included.

## Exact small-field certificates

| p | Newly eligible inputs | Smooth quartics | Usable Jacobians | Obstructed | Usable per input |
|---|---:|---:|---:|---:|---:|
{table}
| Total | 23 | 276 | 152 | 124 | 2–11 |

For every quartic, all 28 odd-gradient bitangents and global smoothness were
checked. Every coefficient was verified fixed by p-Frobenius. Independent
Jacobian-order certificates combine exact counts over F_p and F_(p²), exact
divisor-class orders, and a Weil interval containing just one possible multiple.
The [raw records](results/quotient_sweep.json) contain the equations and certificates.

These are exploratory inputs from the earlier sample; Frobenius conjugates and
curves in the same isogeny class occur. These counts are not independent samples
or a population-density estimate. No claim is made about all cubic curves.

## The more useful implementation lead

The same 27-coefficient tangent-line tensor supplies odd theta gradients. They
give both a quartic in the Weil-restriction differential coordinates and a direct
evaluation of the known chi_18 obstruction criterion. This avoids reconstructing
from Dixmier-Ohno invariants and avoids Jacobian arithmetic during kernel
selection. The chi_18 predictions match **all 276 independently certified
quartics**, identified by equation rather than arbitrary kernel labels.

The [method and derivation](QUOTIENT_METHOD.md) distinguish this specialization
from established isogeny, theta-gradient, and Serre-obstruction techniques.
The latter have substantial prior art, including
[Kirschmer–Narbonne–Ritzenthaler–Robert](https://arxiv.org/abs/2004.08315).
Novelty of the complete specialization is unverified; no new attack exponent is
claimed.

## A point map was evaluated on a newly constructed quotient

On the p=7, r=373 example, the new quotient is evaluated in 64 projective
level-two Weil coordinates. All 27 kernel translations leave the image unchanged.
Enumerating the 373-point subgroup yields exactly 187 images, as required when
opposite points are identified. The associated quartic's Jacobian order is
independently certified as 373. The new source/target test pair has scalar 218.

This validates quotient-point evaluation modulo sign. It does not convert those
coordinates into divisors or solve a DLP on this new target. See
[the exact image audit](results/theta_point_probe.json).

## Larger construction tests

{largetext}

CM discriminant -59 supplies candidate orders, but the source prime orders are
independently certified: a nonzero point is killed by the prime r, and its Hasse
interval contains only one multiple of r. One descending 3-isogeny supplies an
input that fails the direct torsion screen, and its unique ascending step reaches
the prepared curve. Large-p smoothness uses exact Jacobian-ideal Gröbner bases
equal to [1] in all three affine charts.

The larger tests use chi_18 for the obstruction. Their Jacobian orders were not
independently point-counted. Their timings include different checks from the
small-field certification sweep and are not DLP performance comparisons.

## Bounded search for an obstruction counterexample

The additional search examined {len(hunt['curves'])} distinct prepared (j,r)
pairs across p=13,19,31,37,43,61. It found {hcounts.get(0,0)} inputs with all 12
quotients obstructed. The minimum observed usable count was {min(hcounts)}.
The [records](results/coverage_hunt.json) retain all chi_18 predictions. Only an
all-obstructed hit triggers independent full quotient-order certification in
this extra search; the other records are obstruction predictions.

Whether at least one usable quotient always exists under the stated torsion
conditions remains open here. A proof or an independently certified
counterexample would be the next mathematical contribution.

## Reproduce

From the suite root with SageMath 10.6:

```sh
sage -python research_lead/quotient_sweep.py
sage -python research_lead/obstruction_probe.py
sage -python research_lead/theta_point_probe.py
sage -python research_lead/zero_test_regression.py
sage -python research_lead/large_instance.py --bits 32
sage -python research_lead/large_instance.py --bits 86
sage -python research_lead/coverage_hunt.py --count 72 --max-draws-per-prime 100000
sage -python research_lead/coverage_hunt.py --count 200 --resume --max-draws-per-prime 3000
python3 research_lead/quotient_report.py
```

The original eligibility outputs remain in this folder. The current arithmetic
results are in [quotient_summary.json](results/quotient_summary.json). The recovered
Sage zero-test issue is documented by an independent three-extension point-count
[regression](results/zero_test_regression.json); production certificates use class
equality with G.zero(). Development failures are excluded from the current report.
'''
    (ROOT/'QUOTIENT_RESULTS.md').write_text(text)
    svg=(OUT/'quotient_counts.svg').read_text();svg=svg[svg.index('<svg'):]
    tr=''.join(f"<tr><td>{s['p']}</td><td>{s['inputs']}</td><td>{s['quartics']}</td><td>{s['usable']}</td><td>{s['obstructed']}</td><td>{s['minimum_usable_per_input']}–{s['maximum_usable_per_input']}</td></tr>" for s in strata)
    cards=''.join(f"<li>{r['r_bits']}-bit prime order; p={r['p']}; {sum(k['predicts_base_field_jacobian'] for k in r['kernels'])}/12 predicted usable; one quartic and 28 bitangents verified; {r['seconds']:.2f} s including source generation.</li>" for r in larger)
    page=f'''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Cubic elliptic quotients: construction results</title>
<style>body{{font:16px/1.6 system-ui,sans-serif;color:#19332f;max-width:1000px;margin:35px auto;padding:0 20px}}h1{{line-height:1.2}}a{{color:#167765}}.note{{padding:14px 20px;background:#fff4df;border-left:4px solid #c79249}}svg{{max-width:100%;height:auto}}table{{border-collapse:collapse;width:100%}}td,th{{padding:8px;border-bottom:1px solid #d6dfdc;text-align:left}}pre{{white-space:pre-wrap;font-size:12px;background:#f2f6f4;padding:16px}}</style>
<h1>The quotient construction works on all 23 new inputs</h1>
<p><b>276 smooth base-field quartics; 152 usable Jacobians; 124 twist-obstructed quotients.</b> Each input has 2–11 usable choices among its 12 kernels.</p>
<div class="note">This is a construction and coverage result. General target-divisor inversion, sign recovery, and target index-calculus benchmarking remain unfinished. No new end-to-end DLP speedup or certified novelty is claimed.</div>
<p><a href="../QUOTIENT_RESULTS.md">Results and reproduction</a> · <a href="../QUOTIENT_METHOD.md">Derivation and prior work</a> · <a href="quotient_sweep.json">Equations and exact certificates</a> · <a href="obstruction_probe.json">chi18 comparisons</a></p>
{svg}<table><tr><th>p</th><th>Inputs</th><th>Quartics</th><th>Usable</th><th>Obstructed</th><th>Usable/input</th></tr>{tr}</table>
<h2>What strengthens the lead</h2><p>A 27-coefficient tangent-line tensor yields both odd theta gradients and an evaluation of the known chi18 criterion. The gradients reconstruct the quartic directly in base-field differential coordinates; chi18 selects unobstructed kernels without target Jacobian arithmetic. All 276 predictions agree with independent exact order certificates.</p>
<h2>New point map</h2><p>The p=7 quotient is evaluated in projective level-two Weil coordinates. All 27 kernel translations agree, and the entire 373-point subgroup gives exactly 187 images modulo sign. The associated quartic has certified Jacobian order 373. Conversion to target divisors remains the next implementation step.</p>
<h2>Larger synthetic inputs</h2><ul>{cards}</ul><p>Source orders are independently certified. Large-instance obstruction checks use chi18; independent target point counts were performed only in the small-field sweep. Timings are construction observations, not DLP benchmarks.</p>
<h2>Counterexample search</h2><p>{len(hunt['curves'])} additional prepared (j,r) pairs; {hcounts.get(0,0)} all-obstructed cases; minimum {min(hcounts)} usable choices. This exploratory search does not prove a universal usable-quotient theorem.</p>
<h2>Novelty boundary</h2><p>The pipeline builds on <a href="https://arxiv.org/abs/2012.07173">Tian</a>, <a href="https://eprint.iacr.org/2001/054">GHS isogeny preprocessing</a>, classical odd-theta gradients, and known Serre-obstruction methods, including <a href="https://arxiv.org/abs/2004.08315">Kirschmer–Narbonne–Ritzenthaler–Robert</a>. The prospective contribution is this automatic specialization and measured coverage, with a sharp arithmetic gate and 12 candidates. Novelty remains unverified.</p>
<h2>Saved validation metadata</h2><pre>{html.escape(json.dumps(summary,indent=2))}</pre></html>'''
    (OUT/'quotient_report.html').write_text(page)
    print(json.dumps({k:v for k,v in summary.items() if k!='source_hashes'},indent=2))


if __name__=='__main__':main()
