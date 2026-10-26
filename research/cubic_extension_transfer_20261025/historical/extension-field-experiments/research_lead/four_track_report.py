"""Build the evidence summary and standalone report for the four-track stage."""
import collections,hashlib,html,json,math
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
ROOT=Path(__file__).resolve().parent


def read(name):return json.loads((ROOT/'results'/name).read_text())


def component_figure(component,structure):
    rows={tuple(row['orbit_key']):row for row in component['curves']}
    levels={tuple(row['orbit_key']):row['distance_to_prepared_boundary'] for row in structure['node_labels']}
    top=sorted(k for k,v in levels.items() if v==2);assert len(top)==2
    pos={};edges=set()
    for i,key in enumerate(top):
        pos[key]=(i*10,2)
        children=sorted(tuple(e['orbit_key']) for e in rows[key]['edges']
                        if e['three_lines']==4 and levels[tuple(e['orbit_key'])]==1)
        assert len(children)==3
        for j,child in enumerate(children):
            pos[child]=(i*10+(j-1)*3,1)
            grandchildren=sorted(tuple(e['orbit_key']) for e in rows[child]['edges']
                                 if e['three_lines']==4 and levels[tuple(e['orbit_key'])]==0)
            assert len(grandchildren)==3
            for l,key2 in enumerate(grandchildren):pos[key2]=(pos[child][0]+(l-1)*.8,0)
    assert len(pos)==26
    for key,row in rows.items():
        for e in row['edges']:
            if e['three_lines']==4:
                other=tuple(e['orbit_key']);edges.add(tuple(sorted([key,other])))
    fig,ax=plt.subplots(figsize=(10.5,3.8),layout='constrained')
    for a,b in edges:ax.plot([pos[a][0],pos[b][0]],[pos[a][1],pos[b][1]],color='#bac3cd',lw=1.3,zorder=1)
    for key,(x,y) in pos.items():
        good=sum(s['unobstructed'] for s in rows[key]['kernels_tested'])
        ax.scatter([x],[y],s=310 if y else 250,color='#b13d35' if good==0 else '#23628c',zorder=2)
        ax.text(x,y,str(good),ha='center',va='center',color='white',fontsize=10,zorder=3)
    ax.set_title('One closed prepared 3-isogeny component over F$_{109^3}$',loc='left',fontsize=13,pad=16)
    ax.set_yticks([0,1,2],['Boundary','Distance 1','Distance 2']);ax.tick_params(axis='y',length=0)
    ax.set_xticks([]);ax.set_ylim(-.45,2.4)
    for spine in ax.spines.values():spine.set_visible(False)
    fig.text(.15,.01,'26 Frobenius orbits; labels count usable kernels among 12. Red: all obstructed. Nonprepared neighbors omitted.',fontsize=8,color='#444444')
    fig.savefig(ROOT/'results/four_track_component.svg',bbox_inches='tight')
    fig.savefig(ROOT/'results/four_track_component.png',dpi=160,bbox_inches='tight');plt.close(fig)


def main():
    forced=read('forced_two_step_audit.json');assert forced['status']=='passed'
    component=read('neighborhood_component.json');structure=read('structure_audit.json')
    endo=read('endomorphism_audit.json');cm=read('cm_coverage.json');edges=read('transfer_edge_cases.json')
    broad=read('neighborhood_random.json');nine=read('neighborhood_random_nine.json')
    assert broad['status']==nine['status']=='complete'
    benchmarks=[]
    for filename in ['four_track_benchmarks.json','four_track_large-prime_benchmarks.json',
                     'repaired_pipeline.json','forced_source_pipeline.json']:
        d=read(filename)
        for row in d.get('experiments',[d]):
            assert row['status']=='passed'
            s=row['target_solver'];rho=row['source_pollard_rho']['median_seconds']
            benchmarks.append({'file':filename,'p':row['p'],'r':row['r'],'scalar':row['scalar'],
                               'isogeny_steps':row['isogeny_steps'],'algorithm':s['algorithm'],
                               'target_seconds':s['seconds'],'pipeline_seconds':row['seconds'],
                               'rho_median_seconds':rho,'target_matrix_rank':s['linear_system_rank'],
                               'chart_retries':len(row['chart_failures']),
                               'basic_target_seconds':row.get('basic_target_comparison',{}).get('seconds'),
                               'speedup_over_source_rho':rho/row['seconds']})
    assert all(row['speedup_over_source_rho']<1 for row in benchmarks)
    validation=read('four_track_validation.json');assert validation['status']=='passed'
    paths=[p for p in ROOT.glob('*.py') if p.name in {
        'obstruction_neighborhoods.py','structure_audit.py','endomorphism_audit.py','cm_coverage.py',
        'forced_two_step_audit.py','inverse_map.py','polynomial_inverse.py','transfer_pipeline.py',
        'line_index_calculus.py','large_prime_index_calculus.py','pollard_rho.py',
        'transfer_edge_cases.py','validate_four_track.py','forced_source_pipeline.py','repaired_pipeline.py',
        'explicit_counterexample_witnesses.py','four_track_report.py'}]
    summary={'status':'passed','coverage':{'prepared_component_orbits':len(component['curves']),
              'closed':component['component_closed'],'broad_cases':len(broad['curves']),
              'broad_draws':broad['draws'],'broad_screened_zero_centers':sum(row['usable_witness'] is None for row in broad['curves']),
              'targeted_nine_cases':len(nine['curves']),
              'targeted_screened_zero_centers':sum(row['usable_witness'] is None for row in nine['curves']),
              'one_extra_step_counterexample_found':broad['one_step_counterexample_found'] or nine['one_step_counterexample_found'],
              'certified_forced_total_steps':forced['minimum_total_degree3_steps_from_sources'],
              'certified_forced_source_count':len(forced['forced_sources']),
              'forced_p':forced['p'],'forced_r':forced['r']},
              'structure':{'endomorphism_discriminant':-21195,'different_coverage_same_endomorphism_order':True,
                           'cm_frobenius_orbits':len(cm['frobenius_orbits']),
                           'cm_zero_coverage_orbits':sum(row['usable_count']==0 for row in cm['frobenius_orbits']),
                           'exceptional_cm_j_polynomials':[row['j_minimal_polynomial'] for row in cm['frobenius_orbits'] if row['usable_count']==0],
                           'coprime_three_repair_degrees':[r['isogeny']['degree'] for r in cm['one_step_coprime_three_repairs']],
                           'coprime_repairs_leave_prepared_3_component':[tuple(r['isogeny']['codomain_orbit_key']) not in {tuple(a['orbit_key']) for a in component['curves']} for r in cm['one_step_coprime_three_repairs']]},
              'transfer_edge_checks':edges['checks'],'benchmarks':benchmarks,
              'validation':validation,'source_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},
              'limits':['No proof or disproof of the universal one-extra-step claim after torsion preparation.',
                        'Generic tangent charts and bounded retries; universal exceptional-input coverage remains open.',
                        'Dense experimental solvers; no new complexity exponent or practical speedup.',
                        'Wall times include concurrent load; no controlled scaling or density estimate.',
                        'No novelty priority claim or cryptographic-size DLP computation.']}
    (ROOT/'results/four_track_summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    component_figure(component,structure)
    c=summary['coverage'];forced_source=forced['forced_sources'][0]
    table='\n'.join(f"| {row['p']} | {row['r']} | {'large-prime' if 'large-prime' in row['algorithm'] else 'line'} | {row['isogeny_steps']} | {row['target_seconds']:.2f} s | {row['pipeline_seconds']:.2f} s | {row['rho_median_seconds']:.4f} s |" for row in benchmarks)
    text=f'''# Results from all four research tracks

The new work produces a certified forced two-step example, rules out several
simple arithmetic predictors of coverage, strengthens image recovery, and adds
line and double-large-prime target solvers. All recorded source answers pass
independent verification. The tested complete attacks remain slower than
source-group Pollard rho.

## 1. Coverage: distinguish a total step from an extra step

Over F_({forced['p']}³), with {forced['field']['modulus'].replace('x','z')}=0, the prepared curve

    {forced['bad_prepared_curve'].split('defined by ',1)[-1].split(' over ')[0]}

has prime order **r={forced['r']}**, is ordinary and non-subfield, and satisfies
the torsion gate. All twelve target Jacobians independently have the wrong
order **{forced['obstructed_jacobian_order']}**. Exact N1/N2 counts, actual
divisor orders, and unique multiples in a Weil interval certify those orders.

There are **{len(forced['forced_sources'])} nonprepared neighboring source curves**, each with a unique
rational degree-three isogeny leading to an isomorphic all-obstructed prepared
model. Therefore at least **two degree-three steps in total** are needed from
these sources within this construction. A usable two-step target is exhibited.
This refutes the total-one-step sufficient condition for arbitrary gated inputs.

One of the forced sources is

    {forced_source['curve'].split('defined by ',1)[-1].split(' over ')[0]}

Its unique degree-three kernel polynomial is
`{forced_source['unique_degree3_kernel']}`. Its codomain is isomorphic over
the cubic field to the all-obstructed model above.

It does **not** refute one additional step after torsion preparation. Every
zero-coverage center found in the new searches has an eligible usable neighbor.
The universal extra-step assertion remains open.

The broad unrestricted-coefficient search tested **{c['broad_cases']} distinct prepared
Frobenius orbits** within its time budget and screened {c['broad_screened_zero_centers']} zero-coverage centers.
The deliberately biased scalar-nine-torsion stratum tested **{c['targeted_nine_cases']} orbits** and screened
{c['targeted_screened_zero_centers']} zero-coverage centers. The known p=109 component closes after **26** eligible
orbits. The datasets can overlap and are not density estimates.

Evidence: [forced two-step certificate](results/forced_two_step_audit.json),
[explicit divisor witnesses](results/explicit_forced_two_step_witnesses.json),
[broad search](results/neighborhood_random.json),
[nine-torsion stratum](results/neighborhood_random_nine.json), and
[closed component](results/neighborhood_component.json).

## 2. Structure: the endomorphism ring does not decide coverage

Two curves in the p=109, r=1293169 isogeny class have the **same geometric
endomorphism discriminant -21195**, scalar -I Frobenius on E[9], and the same
distance from the prepared boundary. One has zero usable kernels and the
other has usable kernels with an independently certified target order r.

The proof uses exact division-polynomial root counts: forty for E[9] and 121
for E[27]. It is cross-checked with exact Hilbert class polynomials. Thus a
criterion using only these arithmetic data cannot classify coverage.

All **36** roots of H_(-21195) lie in the cubic field and form **12 Frobenius
orbits**. Exactly **one orbit** in this complete finite screen has zero coverage.
A **degree-five isogeny** repairs it while preserving both the three-torsion
type and the endomorphism order; the repaired target order is independently
certified. This motivates obstruction-guided walks using small degrees coprime
to three as well as degree three. General isogeny preprocessing is established
prior art, so this specialization is not asserted novel.

Within this fixed CM-order audit, the exceptional orbit is exactly
**j³+55j²+74j+107=0 over F_109**. The degree-five repair leaves the closed
prepared degree-three component pictured below. This provides a concrete way
to change the obstruction while keeping the endomorphism order and torsion
type. The cubic identifies a finite exceptional orbit, not a general criterion
over varying fields.

![The closed prepared component](results/four_track_component.svg)

Evidence: [structural comparison](results/structure_audit.json),
[endomorphism proof data](results/endomorphism_audit.json), and
[complete CM screen and degree-five repair](results/cm_coverage.json).

## 3. Transfer: actual fallback and exceptional-case handling

The reusable solver now composes a bounded isogeny walk, screens obstructions,
inverts source images through base-field polynomial systems, recovers actual
closed-place divisors, and aligns signs by addition in the target Jacobian.

It handles trivial targets, evaluation poles, empty affine inversion fibers,
coincident x-coordinates, and closed places whose y-coordinate is not generated
by x. Seven targeted edge checks pass. A real p=19 benchmark encounters an
empty inversion fiber and succeeds after a common-source-multiplier retry.

Two larger complete examples exercise the fallback: the p=109 prepared
counterexample is repaired automatically in one step, and a p=157 unprepared
source is transferred through two actual degree-three isogenies. The recovered
source scalars are **1729** and **271828**, respectively. Neither is an input
to the attack API.

This is still a generic-tangent-chart implementation with bounded retries.
Universal handling of all exceptional geometric inputs remains unfinished.
See [the edge-case record](results/transfer_edge_cases.json),
[p=109 complete run](results/repaired_pipeline.json), and
[p=157 complete run](results/forced_source_pipeline.json).

## 4. Target solvers and complete timing

Split line sections supply exact relations with the canonical class retained
as an unknown. The double-large-prime implementation builds a graph on points
outside a small factor base and substitutes rooted-tree expressions before
linear algebra. A translate walk supplies anchoring relations efficiently.
These are implementations of known geometric index-calculus ideas.

| p | r | Target solver | 3-isogeny steps | Target log | Complete transfer and log | Source rho median |
|---|---:|---|---:|---:|---:|---:|
{table}

The p=157 large-prime run uses a matrix of rank **39**, covering **157 of 158**
rational curve points, instead of assigning a separate unknown to every point.
Its final answer is checked both in the target Jacobian and on the source
elliptic curve. The basic smooth-divisor solver is separately measured on the
same target instances where a comparison is archived.

Every row is a single complete pipeline observation; the rho column is the
median of five seeded full-subgroup runs. Pipeline time includes construction,
section setup, inversion, and target DLP, and excludes independent controls.
Some jobs ran concurrently on four cores. These timings show a bottleneck
reduction, not a controlled scaling estimate. **No complete attack beats source
Pollard rho here. No new exponent or cryptographic-sized DLP is claimed.**

Raw results: [line solver](results/four_track_benchmarks.json),
[large-prime solver](results/four_track_large-prime_benchmarks.json),
[independent certificate checks](results/four_track_validation.json), and
[machine-readable summary and code hashes](results/four_track_summary.json).

## Reproduce and remaining work

With SageMath 10.6, from the suite root:

```sh
sage -python research_lead/obstruction_neighborhoods.py --mode component
sage -python research_lead/obstruction_neighborhoods.py --mode random --count 3500 --seconds 1200
sage -python research_lead/obstruction_neighborhoods.py --mode random --target-nine --count 250 --seconds 300 --seed 20261116
sage -python research_lead/forced_two_step_audit.py
sage -python research_lead/explicit_counterexample_witnesses.py --audit forced_two_step_audit.json --output explicit_forced_two_step_witnesses.json
sage -python research_lead/structure_audit.py
sage -python research_lead/endomorphism_audit.py
sage -python research_lead/cm_coverage.py
sage -python research_lead/transfer_edge_cases.py
sage -python research_lead/transfer_pipeline.py --primes 7,13,19,31 --compare-basic
sage -python research_lead/transfer_pipeline.py --primes 7,13,19,31 --solver large-prime --compare-basic
sage -python research_lead/repaired_pipeline.py
sage -python research_lead/forced_source_pipeline.py
sage -python research_lead/validate_four_track.py
python3 research_lead/four_track_report.py
```

Wall-clock-bounded searches will not reproduce identical case counts on every
machine. Seeds and actual accepted inputs are saved. Benchmark files resume
completed cases; rename an existing output file to force fresh measurements.

The main mathematical question is now the **extra-step** bound, with total-step
coverage already refuted. A useful theoretical investigation is to express the
simultaneous obstruction signs as square classes of modular functions and look
for dependencies that forbid or permit an all-negative neighborhood. This is a
proposed direction, not an established calculation or theorem.

Comprehensive chart coverage, sparse large-scale implementations, isolated
repeated performance measurements, and novelty verification also remain open.
The derivations and limitations are in [FOUR_TRACK_METHOD.md](FOUR_TRACK_METHOD.md).
'''
    (ROOT/'FOUR_TRACK_RESULTS.md').write_text(text)
    svg=(ROOT/'results/four_track_component.svg').read_text()
    timingrows=''.join(f'<tr><td>{row["p"]}</td><td>{row["r"]}</td><td>{"Large-prime" if "large-prime" in row["algorithm"] else "Line"}</td><td>{row["isogeny_steps"]}</td><td>{row["target_seconds"]:.2f}</td><td>{row["pipeline_seconds"]:.2f}</td><td>{row["rho_median_seconds"]:.4f}</td></tr>' for row in benchmarks)
    page=f'''<!doctype html><html lang="en"><meta charset="utf-8"><title>Extension-field ECDLP: four research tracks</title>
<style>body{{font:16px/1.55 system-ui;max-width:1100px;margin:40px auto;padding:0 24px;color:#172938}}svg{{width:100%;height:auto}}a{{color:#23628c}}table{{width:100%;border-collapse:collapse;font-size:14px}}th,td{{padding:8px;border-bottom:1px solid #dde3e9;text-align:right}}th{{background:#f2f5f8}}h2{{margin-top:36px}}.callout{{padding:18px;border-left:4px solid #b13d35;background:#fff4f2}}.note{{color:#4a5c6a}}</style>
<h1>Extension-field ECDLP: four research tracks</h1>
<p><a href="../FOUR_TRACK_RESULTS.md">Full report</a> · <a href="../FOUR_TRACK_METHOD.md">Methods and limits</a> · <a href="four_track_summary.json">Summary and code hashes</a></p>
<p class="callout"><strong>A total-one-step guarantee fails.</strong> Over F<sub>157³</sub>, two nonprepared sources each have a unique rational 3-isogeny into a model whose twelve candidate Jacobians all have the wrong order. A two-step transfer succeeds. One extra step after preparation remains an open universal question.</p>
<h2>Coverage and arithmetic structure</h2><p>The broad search tested {c['broad_cases']} prepared Frobenius orbits; a separate biased nine-torsion stratum tested {c['targeted_nine_cases']}. No counterexample to the extra-step bound was found. Finite searches do not prove it.</p>
<p>At p=109, models with the same geometric endomorphism ring (discriminant −21195) have different coverage. In the complete twelve-orbit CM screen, one orbit has all twelve kernels obstructed. A degree-five isogeny repairs it while preserving the torsion type and endomorphism ring.</p>
{svg}
<h2>Transfer and target solvers</h2><p>The reusable transfer includes a bounded obstruction walk, polynomial image inversion, closed-place recovery for coincident x-coordinates, and explicit exceptional-chart retries. The complete p=157 run recovers scalar 271828 through two actual 3-isogenies. Seven targeted edge checks pass.</p>
<p>Line relations and double-large-prime tree substitution reduce the target-solving bottleneck. The p=157 graph covers 157 of 158 rational points and yields a matrix of rank 39. This implements established geometric index-calculus ideas.</p>
<table><thead><tr><th>p</th><th>r</th><th>Target solver</th><th>3-isogeny steps</th><th>Target (s)</th><th>Pipeline (s)</th><th>Source rho (s)</th></tr></thead><tbody>{timingrows}</tbody></table>
<p class="note">Each pipeline value is one observation, including setup and inversion. Rho is the median of five seeded full-subgroup runs performed afterward. Some jobs ran concurrently on four cores. No complete attack beats source rho in these experiments.</p>
<h2>Evidence</h2><p><a href="forced_two_step_audit.json">Twelve-order certificate</a> · <a href="explicit_forced_two_step_witnesses.json">Explicit divisor witnesses</a> · <a href="endomorphism_audit.json">Endomorphism audit</a> · <a href="cm_coverage.json">CM coverage</a> · <a href="forced_source_pipeline.json">Two-step DLP</a> · <a href="four_track_validation.json">Independent validation</a></p>
<p class="note">Universal extra-step coverage, universal exceptional-chart handling, sparse large-scale implementation, and novelty remain open. No new exponent or cryptographic-size DLP computation is claimed.</p></html>'''
    (ROOT/'results/four_track_report.html').write_text(page)
    print('FOUR TRACK REPORT COMPLETE',len(benchmarks),'benchmarks',flush=True)


if __name__=='__main__':main()
