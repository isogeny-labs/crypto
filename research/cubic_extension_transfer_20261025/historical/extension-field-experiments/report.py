"""Generate a standalone HTML report, flattened CSV, and exportable SVG plots."""
import csv
import html
import json
from pathlib import Path


def generate_report(directory):
    directory=Path(directory)
    data=json.loads((directory/'results.json').read_text())
    rows=data['experiments']; meta=data['metadata']
    columns=['case_id','family','status','expectation_met','p','n','d','subgroup_order',
             'target_genus','construction_seconds','order_count_seconds',
             'source_bsgs_seconds','target_bsgs_seconds','transfer_seconds',
             'measured_speedup_including_transfer','worker_wall_seconds','seed']
    with (directory/'summary.csv').open('w',newline='') as out:
        writer=csv.DictWriter(out,fieldnames=columns); writer.writeheader()
        for r in rows:
            flat={c:r.get(c,'') for c in columns}
            flat['source_bsgs_seconds']=r.get('source_bsgs',r.get('bsgs',{})).get('seconds','')
            flat['target_bsgs_seconds']=r.get('target_bsgs',{}).get('seconds','')
            flat['transfer_seconds']=r.get('transfer_seconds_two_points_plus_search',r.get('two_point_transfer_seconds',''))
            writer.writerow(flat)
    plots=[]
    try:
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
        import numpy as np
        matched=[r for r in rows if r.get('source_bsgs') and r.get('target_bsgs') and r['status']=='passed']
        if matched:
            labels=[f"p={r['p']}, n={r['n']}→{r['d']}, g={r['target_genus']}" for r in matched]
            fig,ax=plt.subplots(figsize=(max(9,len(matched)*.72),4.7))
            x=np.arange(len(matched)); width=.25
            ax.bar(x-width,[r['source_bsgs']['seconds'] for r in matched],width,label='Source BSGS',color='#225ea8')
            ax.bar(x,[r['target_bsgs']['seconds'] for r in matched],width,label='Target BSGS',color='#8263a8')
            ax.bar(x+width,[r['transfer_seconds_two_points_plus_search'] for r in matched],width,label='Transfer + generator search',color='#edaa45')
            ax.set_yscale('log'); ax.set_ylabel('Measured seconds (log scale)')
            ax.set_xticks(x,labels,rotation=40,ha='right')
            ax.set_title('Matched subgroup orders; generic BSGS on both sides')
            ax.legend(); ax.grid(axis='y',alpha=.2); fig.tight_layout()
            fig.savefig(directory/'transfer_timings.svg'); plt.close(fig)
            plots.append(('transfer_timings.svg','Measured toy-instance timings. No index-calculus implementation is timed here.'))
        fig,ax=plt.subplots(figsize=(7,3.8))
        degrees=np.arange(2,9)
        ax.plot(degrees,degrees/2,'o-',label='Pollard rho when r ≈ pⁿ',color='#225ea8')
        ax.plot(degrees,2-2/degrees,'o-',label='Fixed-degree reference bound',color='#8263a8')
        ax.set_xlabel('Fixed extension degree n'); ax.set_ylabel('Exponent of p')
        ax.set_title('Theoretical reference only: hidden n-dependent costs omitted')
        ax.legend(); ax.grid(alpha=.2); fig.tight_layout()
        fig.savefig(directory/'degree_bounds.svg'); plt.close(fig)
        plots.append(('degree_bounds.svg','These are asymptotic reference exponents, not experimental measurements.'))
    except ImportError:
        pass
    escape=html.escape
    entries=[]
    for r in rows:
        params=r.get('parameters',{})
        label=json.dumps(params,sort_keys=True)
        details=escape(json.dumps(r,indent=2))
        entries.append(f"<tr data-status='{escape(r['status'])}'><td>{escape(r['family'])}</td>"
                       f"<td>{escape(label)}</td><td>{escape(r['status'])}</td>"
                       f"<td>{r.get('subgroup_order','—')}</td><td>{r.get('target_genus','—')}</td>"
                       f"<td>{r.get('worker_wall_seconds',0):.3f}</td>"
                       f"<td><details><summary>Raw record</summary><pre>{details}</pre></details></td></tr>")
    figures=''.join(f'<figure><img src="{src}" alt="{escape(caption)}"><figcaption>{escape(caption)}</figcaption></figure>' for src,caption in plots)
    unexpected=[r for r in rows if not r.get('expectation_met',False)]
    text=f'''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Elliptic curves over extension fields — experiment results</title>
<style>
:root{{font-family:system-ui,sans-serif;color:#17263e;background:#f4f6fa}}body{{max-width:1280px;margin:auto;padding:32px}}
h1{{font-size:2rem;margin-bottom:8px}}h2{{margin-top:34px}}p,li{{line-height:1.55}}a{{color:#225ea8}}
.cards{{display:flex;gap:16px;flex-wrap:wrap;margin:24px 0}}.card{{background:white;border:1px solid #dce2ed;border-radius:10px;padding:16px 22px;min-width:150px}}
.card strong{{display:block;font-size:1.8rem}}.note{{background:#fff6df;border-left:4px solid #edaa45;padding:12px 18px}}
table{{width:100%;border-collapse:collapse;background:white;font-size:.88rem}}td,th{{padding:10px;border-bottom:1px solid #e3e7ef;text-align:left;vertical-align:top}}
th{{background:#e8edf5}}pre{{max-width:600px;max-height:450px;overflow:auto;white-space:pre-wrap;font-size:.76rem}}input,select{{padding:9px;margin:0 10px 14px 0;border:1px solid #bbc7db;border-radius:5px}}
input{{width:min(420px,70vw)}}figure{{margin:22px 0;background:white;padding:14px;border-radius:10px}}img{{max-width:100%}}figcaption,small{{color:#50627d}}
.scroll{{overflow:auto}}code{{font-size:.9em}}
</style>
<h1>Elliptic curves over extension fields</h1>
<p>Reproducible experiments on low-genus transfers, trace-zero subgroups, and intermediate fields. All input scalars are synthetic.</p>
<div class="cards"><div class="card"><strong>{len(rows)}</strong>recorded cases</div>
<div class="card"><strong>{sum(r['status']=='passed' for r in rows)}</strong>passed</div>
<div class="card"><strong>{sum(r['status']=='ineligible' for r in rows)}</strong>ineligible parameter sets</div>
<div class="card"><strong>{len(unexpected)}</strong>unexpected outcomes</div></div>
<p><a href="results.json">Full JSON</a> · <a href="results.jsonl">Streaming JSONL</a> · <a href="summary.csv">CSV</a> · <a href="plan.json">Executed plan</a></p>
<div class="note"><b>A successful transfer is not a measured DLP speedup.</b> Toy cover experiments use the same generic BSGS algorithm and the same subgroup order on both sides. Transfer timings include the measured point-transfer and generator-search work; separate construction and validation work is included in total runtime.</div>
<h2>What each experiment establishes</h2>
<ul>
<li><b>Tian kernel:</b> published prime-order cubic curve, rational maximal isotropic 3-torsion kernel, Frobenius stability, coordinate-factor rejection, and full source ECDLP recovery.</li>
<li><b>Tian rational map:</b> evaluate all six published functions, trace the divisor classes, match both published images, and verify the subgroup order and homomorphism relations. This evaluates one published isogeny, not a constructor for arbitrary new curves.</li>
<li><b>Tian Jacobian:</b> replay the actual non-hyperelliptic genus-three divisor classes. The target BSGS searches a 4096-element scalar interval; it is not comparable to the full-order source search and is excluded from the timing-ratio plot.</li>
<li><b>Constructed covers:</b> explicit genus-two and genus-three pullback-and-trace maps, including genuine curves over F_(p⁴) and F_(p⁶) mapping to Jacobians over proper intermediate fields. This is not a reproduction of the full Joux–Vitse record computation.</li>
<li><b>Trace zero:</b> exact orders, sampled prime subgroups, Frobenius action, small-field kernel enumeration, and scalar recovery. Relation experiments use exhaustive signed decompositions and linear algebra, not the paper’s Gröbner-basis implementation.</li>
<li><b>Intermediate-field controls:</b> trace transitivity, a surviving base-field subgroup, and a new extension subgroup annihilated by ordinary trace.</li>
<li><b>Binary GHS:</b> original characteristic-two Artin–Schreier rank and candidate-genus screening. It does not construct a divisor map or time an attack.</li>
</ul>
<h2>Measurements and theoretical reference</h2>{figures}
<h2>Recorded cases</h2><input id="search" placeholder="Filter by family, p, n, or result"><select id="status"><option value="">All statuses</option><option>passed</option><option>ineligible</option><option>error</option><option>timeout</option></select><small id="count"></small>
<div class="scroll"><table><thead><tr><th>Family</th><th>Parameters</th><th>Status</th><th>r</th><th>Genus</th><th>Total seconds</th><th>Evidence</th></tr></thead><tbody>{''.join(entries)}</tbody></table></div>
<h2>Reproducibility</h2><pre>{escape(json.dumps(meta,indent=2))}</pre>
<h2>Sources</h2><ul>
<li><a href="https://arxiv.org/abs/2012.07173">Tian, Cover attacks for elliptic curves with prime order (2020 preprint; 2023 publication)</a></li>
<li><a href="https://arxiv.org/abs/1405.1059">Gorla–Massierer, Index Calculus in the Trace Zero Variety (2015)</a></li>
<li><a href="https://eprint.iacr.org/2011/020">Joux–Vitse, Cover and Decomposition Index Calculus (EUROCRYPT 2012)</a></li>
<li><a href="https://www.kantmath.de/publications/papers/weil_descent.pdf">Gaudry–Hess–Smart, Constructive and Destructive Facets of Weil Descent (2002)</a></li>
<li><a href="https://doi.org/10.1090/S0025-5718-2010-02281-1">Diem, On the discrete logarithm problem in class groups of curves (2011)</a></li></ul>
<script>
const q=document.querySelector('#search'),s=document.querySelector('#status'),rs=[...document.querySelectorAll('tbody tr')];
function filter(){{let n=0;for(const r of rs){{let show=(!s.value||r.dataset.status===s.value)&&r.textContent.toLowerCase().includes(q.value.toLowerCase());r.hidden=!show;if(show)n++;}}document.querySelector('#count').textContent=n+' cases shown';}}
q.addEventListener('input',filter);s.addEventListener('change',filter);filter();
</script></html>'''
    (directory/'report.html').write_text(text)


if __name__=='__main__':
    import sys
    generate_report(sys.argv[1])
