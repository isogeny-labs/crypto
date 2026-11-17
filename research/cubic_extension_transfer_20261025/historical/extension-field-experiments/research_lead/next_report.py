"""Validate and summarize the counterexample, repair, and polynomial transfers."""
from datetime import datetime,timezone
from pathlib import Path
import hashlib,html,json,os
ROOT=Path(__file__).resolve().parent;OUT=ROOT/'results'


def main():
    audit=json.loads((OUT/'counterexample_audit.json').read_text())
    witnesses=json.loads((OUT/'explicit_counterexample_witnesses.json').read_text())
    tests=[json.loads((OUT/name).read_text()) for name in ['polynomial_inverse.json','polynomial_inverse_p13.json']]
    walk=json.loads((OUT/'coverage_walk.json').read_text())
    broad=json.loads((OUT/'coverage_search2.json').read_text())
    fast=json.loads((OUT/'fast_counterexample.json').read_text())
    assert audit['status']==witnesses['status']=='passed' and len(witnesses['certificates'])==12
    assert all(c['verified'] and c['unique_multiple']==1296891 for c in witnesses['certificates'])
    assert all(k['jacobian_order']==1296891 for k in audit['all_12_quotient_certificates']['kernels'])
    assert audit['one_additional_isogeny_repair']['target_order_certificate']['jacobian_order']==1293169
    assert not audit['has_nonscalar_three_neighbor']
    for test in tests:
        assert test['status']=='passed' and test['all_image_coordinates_in_base_field']
        assert not test['target_group_enumeration_used']
        assert test['section_certificate']['extension_field_point_enumeration_used'] is False
        assert test['section_certificate']['exact_diagonal_identities_verified']==8
        assert test['source_bsgs']['scalar']==test['recovered_scalar']
    summary={'created_utc':datetime.now(timezone.utc).isoformat(),
             'prepared_curve_coverage_counterexample':{'p':109,'r':1293169,'all_twelve_jacobian_orders':1296891,
                 'explicit_divisor_witnesses':12,'one_more_3_isogeny_repairs':True,
                 'weaker_existential_one_step_conjecture_refuted':False},
             'polynomial_transfers':[{'p':t['p'],'r':t['r'],'scalar':t['recovered_scalar'],
                 'total_seconds':t['seconds'],'three_inversions_seconds':sum(i['seconds'] for i in t['inversion_runs']),
                 'target_index_calculus_seconds':t['target_index_calculus']['seconds'],
                 'source_bsgs_seconds':t['source_bsgs']['seconds'],
                 'extension_field_point_enumeration':False,'target_group_enumeration':False} for t in tests],
             'searches':{'closed_walk_classes':len(walk['curves']),'walk_frontier':walk['queued'],
                         'walk_truncation':walk['depth_truncated_edges'],'broad_orbits':len(broad['curves']),
                         'hesse_accepted_before_hit':len(fast['curves'])},
             'source_hashes':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(ROOT.glob('*.py'))},
             'limits':['The weaker existential one-step coverage claim remains open.',
                       'Only two small-field complete ECDLP tests are reported.',
                       'The target index-calculus solver is basic and much slower than source BSGS in these tests.',
                       'Comprehensive nongeneric chart handling and cryptographic-size DLP benchmarking remain unfinished.',
                       'No new attack exponent or literature-priority claim is established.']}
    (OUT/'next_summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    os.environ.setdefault('MPLCONFIGDIR','/tmp/extension-field-next-mpl')
    os.environ.setdefault('XDG_CACHE_HOME','/tmp/extension-field-next-cache')
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    values=[0]+[n['unobstructed_quotients'] for n in audit['neighbors']]
    labels=['Prepared E','Neighbor 1','Neighbor 2','Neighbor 3','Neighbor 4']
    fig,ax=plt.subplots(figsize=(9,4.5))
    ax.bar(labels,values,color='#257668',label='Passes chi18')
    ax.bar(labels,[12-v for v in values],bottom=values,color='#cf945b',label='Fails chi18')
    ax.set_ylim(0,13);ax.set_ylabel('Candidates out of 12')
    ax.set_title('Torsion readiness can fail; one extra 3-isogeny repairs this example')
    ax.legend(loc='upper left',bbox_to_anchor=(0,1.02));ax.grid(axis='y',alpha=.2);ax.set_axisbelow(True)
    fig.tight_layout();fig.savefig(OUT/'counterexample_repair.svg');fig.savefig(OUT/'counterexample_repair.png',dpi=150);plt.close(fig)
    table='\n'.join(f"| {t['p']} | {t['r']} | {t['scalar']} | {t['three_inversions_seconds']:.2f} s | {t['target_index_calculus_seconds']:.2f} s | {t['total_seconds']:.2f} s | {t['source_bsgs_seconds']:.4f} s |" for t in summary['polynomial_transfers'])
    text=f'''# New results: prepared-curve coverage fails, polynomial transfer works

The stronger prepared-curve coverage claim has a counterexample. Separately,
the quotient-to-divisor stage is now implemented as a base-field polynomial
system, and actual target index calculus completes two small ECDLP reductions.

## A precise counterexample and its repair

Over F_(109³), with z³+z+103=0, the curve

    y² = x³ + (37z²+105z+29)x + (43z²+6z+66)

is ordinary, has j not in F_109, and has prime group order r=1293169. It meets
p=1 mod 3 and r=4 mod 9 and already has scalar -I action on E[3]. Nevertheless,
all 12 transverse quotient candidates produce curves whose base-field
Jacobians have order **1296891=3^7·593**. They cannot preserve an r-subgroup.

All twelve orders have independent exact certificates using N1=110, N2=11882,
explicit rational-point divisor orders, and a unique multiple in a Weil
interval. The source prime order also has an independent point-annihilator and
Hasse-interval certificate. This conclusion does not depend only on chi18.

Each of the four immediate 3-isogeny neighbors has usable candidates: 5,3,4,5
respectively. One repaired target Jacobian has independently certified order r.
See [the counterexample, equations, and scope](COUNTEREXAMPLE.md),
[the full audit](results/counterexample_audit.json), and
[explicit divisor witnesses](results/explicit_counterexample_witnesses.json).

**What is refuted:** every prepared curve immediately has a usable kernel, so
the procedure can stop once the torsion condition holds.

**What remains open:** one freely chosen additional isogeny always reaches
some usable prepared model. This example is repaired by such a step and does
not refute that weaker existential statement. No universal walk-length bound
is proved.

## Direct image inversion and actual target logarithms

The new driver uses a rational odd theta characteristic and rational tangent
chart. Eight symmetric sections are fitted from a fixed number of random curve
points, with rank 112 and eight exact symbolic diagonal identities checked.
The final seven-variable polynomial system is over F_p. Its solutions give
actual divisor classes; three images, for P,Q,P+Q, fix their relative signs.

The target logarithm is then recovered by a rational-point smooth-divisor
index-calculus solver. Every relation is checked in the actual Jacobian and
the answer is checked on the original elliptic curve. No target-group
enumeration, F_(p³) point scan, or known source logarithm is used by this driver.

| p | Source prime order | Recovered scalar | Three polynomial inversions | Target index calculus | Complete pipeline | Source BSGS |
|---|---:|---:|---:|---:|---:|---:|
{table}

The pipeline timings include setup and checks. The BSGS measurements use the
same complete source subgroup. These small tests show **no speedup**. The
target solver is basic smooth-divisor index calculus, not Diem's optimized
line/large-prime method. No cryptographic-size DLP was solved.

The [derivation](POLYNOMIAL_INVERSION.md) explains the theta translation,
normalization cancellation, section construction, and polynomial recovery.
Raw records: [p=7](results/polynomial_inverse.json),
[p=13](results/polynomial_inverse_p13.json).

## Search scope and novelty

The counterexample was found in a 4430-case early-rejection Hesse-family search.
A separate 1000-case search and 49 closed classes in three targeted isogeny
components found no counterexample. These are different, potentially
overlapping exploratory datasets, not independent density estimates.

The work builds on [Tian's quotient and symmetric-section method](https://arxiv.org/abs/2012.07173),
known isogeny preprocessing, and established algebraic Serre-obstruction
techniques, including [Kirschmer–Narbonne–Ritzenthaler–Robert](https://arxiv.org/abs/2004.08315).
The odd-theta base-field specialization and this coverage counterexample have
not been certified novel. The counterexample refutes our proposed sufficient
condition, not a claimed theorem of Tian.

The remaining research work is optimized target index calculus, comprehensive
chart handling, larger transfer benchmarks, and the weaker coverage question.

## Reproduce

From the suite root with SageMath 10.6:

```sh
sage -python research_lead/counterexample_audit.py
sage -python research_lead/explicit_counterexample_witnesses.py
sage -python research_lead/polynomial_inverse.py --solve
sage -python research_lead/polynomial_inverse.py --solve --p 13
sage -python research_lead/finalize_transfer_results.py
python3 research_lead/next_report.py
```

The older exhaustive `complete_toy.py` inverse oracle is development evidence
and is not used in the polynomial runs. Saved code hashes and validation scope
are in [next_summary.json](results/next_summary.json).
'''
    (ROOT/'NEXT_RESULTS.md').write_text(text)
    svg=(OUT/'counterexample_repair.svg').read_text();svg=svg[svg.index('<svg'):]
    timingrows=''.join(f"<tr><td>{t['p']}</td><td>{t['r']}</td><td>{t['scalar']}</td><td>{t['three_inversions_seconds']:.2f}s</td><td>{t['target_index_calculus_seconds']:.2f}s</td><td>{t['total_seconds']:.2f}s</td><td>{t['source_bsgs_seconds']:.4f}s</td></tr>" for t in summary['polynomial_transfers'])
    page=f'''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Cubic ECDLP research: counterexample and polynomial transfer</title>
<style>body{{font:16px/1.6 system-ui,sans-serif;color:#19332f;max-width:1050px;margin:35px auto;padding:0 20px}}h1{{line-height:1.2}}a{{color:#167765}}.note{{padding:14px 20px;background:#fff4df;border-left:4px solid #c79249}}svg{{max-width:100%;height:auto}}table{{border-collapse:collapse;width:100%}}td,th{{padding:8px;border-bottom:1px solid #d6dfdc;text-align:left}}pre{{white-space:pre-wrap;font-size:12px;background:#f2f6f4;padding:16px}}</style>
<h1>Prepared-curve coverage fails; polynomial transfer works</h1>
<p>An ordinary prime-order curve over F_(109³) passes the torsion screen, but all 12 immediate quotient candidates are obstructed. One additional 3-isogeny repairs it. A new polynomial driver also completes ECDLP transfers over p=7 and p=13 without target-group enumeration.</p>
<p><a href="../NEXT_RESULTS.md">Results and reproduction</a> · <a href="../COUNTEREXAMPLE.md">Counterexample proof and scope</a> · <a href="../POLYNOMIAL_INVERSION.md">Inversion derivation</a> · <a href="explicit_counterexample_witnesses.json">Explicit certificates</a></p>
<div class="note">The refuted claim is immediate coverage on every torsion-prepared model. The weaker claim allowing a freely chosen additional step remains open. No new attack exponent, cryptographic-size DLP, or priority claim is established.</div>
{svg}<p>The initial curve's twelve failed targets are independently order-certified. The neighbor counts are chi18 screens; one passing neighbor target is independently certified.</p>
<h2>The order obstruction</h2><p>Source r=1293169 is prime. Every reconstructed Jacobian has order 1296891=3^7·593, so none contains the source r-subgroup. Exact N1,N2 counts and explicit divisor-order witnesses prove the target orders.</p>
<h2>Actual polynomial transfer</h2><p>A rational odd-theta reference and tangent chart put the inversion equations over F_p. Eight exact symmetric-section identities are checked. Polynomial solutions yield divisors, three images align signs, and actual smooth-divisor index calculus recovers the logarithm.</p>
<table><tr><th>p</th><th>r</th><th>Scalar</th><th>3 inversions</th><th>Target IC</th><th>Pipeline</th><th>Source BSGS</th></tr>{timingrows}</table>
<p>The two small-field runs show no speedup. The target solver is a basic factor-base implementation, not optimized Diem index calculus. The sampled setup does not scan F_(p³).</p>
<h2>Provenance and limits</h2><p>The work specializes known methods of <a href="https://arxiv.org/abs/2012.07173">Tian</a> and established obstruction theory. Novelty is unverified. Optimized target solving, nongeneric charts, larger transfer benchmarks, and the weaker coverage claim remain open.</p>
<pre>{html.escape(json.dumps(summary,indent=2))}</pre></html>'''
    (OUT/'next_report.html').write_text(page)
    print(json.dumps({k:v for k,v in summary.items() if k!='source_hashes'},indent=2))


if __name__=='__main__':main()
