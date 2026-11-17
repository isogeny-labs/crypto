"""Summarize saved research probes without rerunning arithmetic."""
from datetime import datetime,timezone
from pathlib import Path
import hashlib
import html
import json
import os

ROOT=Path(__file__).resolve().parent; OUT=ROOT/'results'
probe=json.loads((OUT/'curve_probe.json').read_text())
models=json.loads((OUT/'kernel_classification.json').read_text())
fixture=json.loads((OUT/'composed_fixture.json').read_text())
rows=probe['curves']
assert len(rows)==282 and all(r['status']=='passed' for r in rows)
assert all(r['cheap_preprocessing_gate']==r['preprocessed_transverse_kernel_eligible'] for r in rows)
assert fixture['status']=='passed'
direct=sum(r['direct_transverse_kernel_eligible'] for r in rows)
after=sum(r['preprocessed_transverse_kernel_eligible'] for r in rows)
assert (direct,after)==(12,35)
summary={'created_utc':datetime.now(timezone.utc).isoformat(),
         'sampled_curves':len(rows),'sampled_isogeny_classes':len({(r['p'],r['prime_group_order']) for r in rows}),
         'direct_candidate_curves':direct,'candidate_curves_after_at_most_one_step':after,
         'newly_eligible_curves':after-direct,
         'direct_candidate_isogeny_classes':len({(r['p'],r['prime_group_order']) for r in rows if r['direct_transverse_kernel_eligible']}),
         'candidate_isogeny_classes_after_preprocessing':len({(r['p'],r['prime_group_order']) for r in rows if r['preprocessed_transverse_kernel_eligible']}),
         'composed_fixture_verified':True,'sage':probe['sage'],'seed':probe['seed'],
         'source_sha256':{str(p.relative_to(ROOT.parent)):hashlib.sha256(p.read_bytes()).hexdigest()
                          for p in sorted(list(ROOT.glob('*.py'))+[ROOT.parent/'common.py',ROOT.parent/'tian.py'])},
         'limits':['Kernel eligibility is not a Jacobian certificate.',
                   'Samples are exploratory and not independent across isogeny classes.',
                   'The composed target DLP uses a bounded scalar interval.',
                   'No new quotient constructor, cryptographic-scale speedup, or priority claim.']}
(OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
os.environ.setdefault('MPLCONFIGDIR','/tmp/extension-field-research-mpl')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np
strata=[r for r in probe['strata'] if r['p_mod_3']==1]
x=np.arange(len(strata)); w=.36
fig,ax=plt.subplots(figsize=(8.5,4.5))
ax.bar(x-w/2,[r['direct_eligible'] for r in strata],w,label='Direct torsion eligibility',color='#225ea8')
ax.bar(x+w/2,[r['after_one_step_eligible'] for r in strata],w,label='After at most one 3-isogeny',color='#8263a8')
ax.set_xticks(x,[str(r['p']) for r in strata]); ax.set_xlabel('Prime p, for E over F_(p^3)')
ax.set_ylabel('Candidate curves out of 24 sampled')
ax.set_title('Torsion-kernel candidates, not completed Jacobian attacks')
ax.set_ylim(0,12); ax.grid(axis='y',alpha=.2); ax.set_axisbelow(True); ax.legend()
fig.tight_layout(); fig.savefig(OUT/'candidate_counts.svg'); fig.savefig(OUT/'candidate_counts.png',dpi=140); plt.close(fig)
table=''.join(f"<tr><td>{r['q_mod_3']}</td><td>{r['trace_mod_3']}</td><td>{r['source_frobenius_type']}</td><td>{r['frobenius_stable_lagrangians']}</td><td>{r['transverse_stable_lagrangians']}</td><td>{r['prime_order_compatible']}</td></tr>" for r in models['models'])
svg=(OUT/'candidate_counts.svg').read_text(); svg=svg[svg.index('<svg'):]
page=f'''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Research lead: a cheap isogeny step before cubic descent</title>
<style>body{{font:16px/1.6 system-ui,sans-serif;color:#17263e;max-width:1000px;margin:35px auto;padding:0 20px}}a{{color:#225ea8}}h1{{line-height:1.2}}.note{{padding:14px 20px;background:#fff5db;border-left:4px solid #e2a233}}table{{border-collapse:collapse;width:100%}}td,th{{padding:8px;border-bottom:1px solid #d8e0ed;text-align:left}}svg{{max-width:100%;height:auto}}pre{{white-space:pre-wrap;background:#f1f4f9;padding:14px}}</style>
<h1>A cheap isogeny step before Tian's cubic descent</h1>
<p><b>Validated lead; novelty and a general new attack remain unestablished.</b></p>
<p>For an ordinary prime-order E/F_(p³), p ≥ 5, check p = 1 mod 3 and r = 4 mod 9. This is equivalent to the quadratic twist order being divisible by nine in the favorable characteristic class. If needed, one rational 3-isogeny reaches scalar -I Frobenius on E[3].</p>
<div class="note">The reached torsion module has 12 transverse rational (3,3,3)-kernel candidates. Each quotient still needs polarized indecomposability, finite-field Jacobian descent, and target-curve checks. Eligibility alone does not certify an attack.</div>
<p><a href="../README.md">Derivation, proof sketch, prior work, next milestone</a> · <a href="curve_probe.json">282 curve records</a> · <a href="kernel_classification.json">Exact module enumeration and all candidate bases</a> · <a href="composed_fixture.json">Composed reduction</a></p>
<h2>Experiments</h2><p>12 direct candidate curves became 35 after at most one isogeny. At the level of the sampled isogeny classes, the counts increased from 10 to 20, out of 203 sampled classes. The exploratory sample is not a density estimate. No counterexample to the torsion screen occurred.</p>
{svg}<h2>Finite linear-algebra check</h2>
<p>33,880 three-dimensional subspaces and 1,120 Lagrangians were enumerated for each of eight source Frobenius models over F_3.</p>
<table><tr><th>p mod 3</th><th>t mod 3</th><th>Source A</th><th>Stable</th><th>Transverse</th><th>Prime-order compatible</th></tr>{table}</table>
<h2>Actual composed reduction</h2><p>A different input curve of prime order 143069629, with no direct transverse auxiliary-3 kernel, was reduced through dual 3-isogenies to Tian's published curve. Evaluation of the six rational functions produced nine times the published image classes. Scalar 1987 was recovered on the actual target Jacobian within [0,4095]. This reuses a known quotient and is not a full-order target performance benchmark.</p>
<h2>Novelty boundary</h2><p>Isogeny preprocessing already appears in <a href="https://eprint.iacr.org/2001/054">Galbraith–Hess–Smart (2002)</a>. <a href="https://doi.org/10.1007/s00145-023-09474-2">Tian (2023)</a> supplies the relevant gluing framework. The prospective contribution is an automatic construction and coverage study for the 12 candidates, supported by this cheap arithmetic screen. Priority is not certified.</p>
<h2>Next falsifiable milestone</h2><p>Construct the new Jacobian quotients for the 23 newly eligible curves and determine how many are usable non-hyperelliptic genus-three targets over the base field. Then measure a real target index-calculus solver and all preprocessing costs. Decomposable or incorrectly descended quotients must be recorded as failures.</p>
<h2>Reproducibility</h2><pre>{html.escape(json.dumps(summary,indent=2))}</pre></html>'''
(OUT/'report.html').write_text(page)
print(json.dumps({k:v for k,v in summary.items() if k!='source_sha256'},indent=2))
