#!/usr/bin/env python3
"""Presentation only; all statistics come from native analysis/manifest."""
import json
import os
from pathlib import Path
os.environ.setdefault('MPLCONFIGDIR','/tmp/conductor-twist-mpl')
os.environ.setdefault('XDG_CACHE_HOME','/tmp/conductor-twist-fonts')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

HERE=Path(__file__).resolve().parent
v=json.loads((HERE/'summary.json').read_text());out=HERE/'reports';out.mkdir(exist_ok=True)
pilot=json.loads((HERE/'pilot-spec.json').read_text())
slugs=list(dict.fromkeys(json.loads(line)['workload']['curve']['slug'] for line in (HERE/'results/pilot-l0/records.jsonl').read_text().splitlines()))
groups={r['curve_slug']:r for r in v['ECDLP']['groups'] if r['arm']=='rho-counted'}
raw=[json.loads(line) for line in (HERE/'results/pilot-l0/records.jsonl').read_text().splitlines()]
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':11,'axes.spines.top':False,'axes.spines.right':False})
fig,ax=plt.subplots(figsize=(8.7,3.1),constrained_layout=True)
for i,(slug,color) in enumerate(zip(slugs,['#467aa6','#d69a50'])):
    ax.bar(i,groups[slug]['mean_solve_seconds'],color=color,width=.55)
    points=[r['time']['solve_wall_ns']/1e9 for r in raw if r['arm']=='rho-counted' and r['workload']['curve']['slug']==slug]
    ax.scatter([i-.04,i+.04],points,color='#243548',zorder=3,s=25)
ax.set_xticks([0,1],['source: conductor 1','quotient: conductor 33555419'])
ax.set_ylabel('solve seconds');ax.set_title('Rho: one matched target, two rounds; descriptive L0')
ax.grid(axis='y',alpha=.15);fig.savefig(out/'rho.svg');fig.savefig(out/'rho.png',dpi=145);plt.close(fig)
scalar={r['curve_slug']:r for r in v['scalar']['descriptive_by_curve']}
scalarplan=json.loads((HERE/'results/scalar-l0/plan.json').read_text())
scalar_slugs=list(dict.fromkeys(w['curve']['slug'] for w in scalarplan['workloads']))
fig,ax=plt.subplots(figsize=(8.7,3.1),constrained_layout=True)
for i,(slug,color) in enumerate(zip(scalar_slugs,['#467aa6','#d69a50','#79a17d'])):
    ax.bar(i,scalar[slug]['mean_ns_per_mul']/1000,color=color,width=.55)
    ax.scatter(i,scalar[slug]['median_ns_per_mul']/1000,color='#243548',zorder=3,s=25)
ax.set_xticks([0,1,2],['source','quotient','source u=2 control']);ax.set_ylabel('microseconds per multiplication')
ax.set_title('32 bases/model ×6 measured rounds; batch64; median dots');ax.grid(axis='y',alpha=.15)
fig.savefig(out/'scalar.svg');fig.savefig(out/'scalar.png',dpi=145);plt.close(fig)
diagram='''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="350" viewBox="0 0 1100 350">
<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6" fill="none" stroke="#38688c"/></marker></defs>
<rect width="1100" height="350" fill="#f3f7fb"/>
<g font-family="DejaVu Sans" fill="#244459" text-anchor="middle">
<rect x="35" y="30" width="290" height="70" rx="8" fill="#dcebf5"/><text x="180" y="60" font-size="21">T: auxiliary source</text><text x="180" y="83" font-size="14">rational ell-order kernel K</text>
<rect x="775" y="30" width="290" height="70" rx="8" fill="#dcebf5"/><text x="920" y="60" font-size="21">T′: auxiliary quotient</text><text x="920" y="83" font-size="14">independent torsion probe evaluated</text>
<rect x="35" y="240" width="290" height="80" rx="8" fill="#e1efe5"/><text x="180" y="271" font-size="21">E: conductor 1</text><text x="180" y="296" font-size="14">original 53-bit prime subgroup</text>
<rect x="775" y="240" width="290" height="80" rx="8" fill="#e1efe5"/><text x="920" y="271" font-size="21">E′: conductor 33555419</text><text x="920" y="296" font-size="14">transported original subgroup</text>
<text x="550" y="46" font-size="16">normalized Φ over Fp</text><text x="550" y="265" font-size="16">twisted Φd over Fp:33 point images checked</text>
<text x="345" y="166" font-size="15">ud over Fp²</text><text x="750" y="166" font-size="15">ud over Fp²</text>
</g><g stroke="#38688c" stroke-width="2" marker-end="url(#arrow)"><path d="M340,70 H755"/><path d="M340,288 H755"/><path d="M180,113 V226"/><path d="M920,113 V226"/></g></svg>'''
(out/'map.svg').write_text(diagram)
rho_rows='\n'.join(f'[{label}], [{groups[slug]["mean_solve_seconds"]:.3f}], [2 verified],'
                    for label,slug in zip(['source','quotient'],slugs))
scalar_rows='\n'.join(f'[{label}], [{scalar[slug]["mean_ns_per_mul"]/1000:.1f}], [{scalar[slug]["median_ns_per_mul"]/1000:.1f}], [192],'
                       for label,slug in zip(['source','quotient','u=2 control'],scalar_slugs))
text=r'''#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10.4pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
= Explicit prime-conductor crossing, evaluated locally
19 November 2026. Original 80-bit objective remains open.

The actual ordinary endomorphism conductors are 1 and 33555419. This ratio is a 26-bit prime (log2 ratio 25.0000424361), over the 53-bit field prime 5629830687772387. It is the evaluated retained case in this report; no world-record statement is made. Arithmetic elliptic-curve conductors over global fields are not the quantity measured here. [pcc:unverified]

#table(columns: (1fr,2fr), inset: 5pt, align: left,
 [Parameter], [Exact retained value],
 [source equation a,b], [5620419248410339, 675862186879016],
 [quotient equation a,b], [2539833775737119, 1737207970637906],
 [prime map degree / conductor ratio], [33555419],
 [original prime subgroup order], [5629830654216971 (cofactor 1)],
 [source generator], [(2, 1401162638053187)],
 [transported generator], [(2975092980436849, 5333622322807887)],
)

#image("map.svg", width: 100%)

For normalized auxiliary map $(x,y) mapsto (f(x), y f'(x))$, the twist-d map is
$(x,y) mapsto (d f(x/d), y f'(x/d))$, here d=2 nonsquare. Its coefficient scaling is d²/d³. The auxiliary kernel is explicitly typed; it is not asserted to lie on E. Both horizontal maps descend to Fp, although the vertical isomorphisms use Fp². [pcc:unverified]

Fresh PARI counts give ordinary trace t=33555417 and discriminant t²−4p=−19 ell². The Frobenius/endomorphism order sandwich allows only conductors 1 or ell. PARI computes H−19(X)=X+884736 and class number 1. The source j is its root; the quotient j is different, excluding conductor 1 there. See CM_ARGUMENT.md and executable check-conductors.gp. This is a standard CM argument with arithmetic certificates, not a compiled formal proof. [pcc:unverified]

#pagebreak()
= Full construction, point transport and recovery
The native square-root Vélu schedule covers all 16777709 half-kernel representatives, with baby/giant counts 2896/2896. It emits the exact retained quotient only after its complete-coverage gate. 33 point inputs are original G plus 32 PARI-seeded known-answer targets; every image independently passes membership and the original prime-subgroup scalar identity. This is distinct from the later scalar timer's SHA-derived 32-base panel. [pcc:unverified]

#table(columns: (1.3fr,1fr,1.9fr), inset: 5pt,
 [Actual call], [Elapsed seconds], [Interval],
 [main-b], [8.661], [quotient construction plus 33 images],
 [follow-up repeat2], [8.664], [same exact input/output bytes],
 [follow-up repeat3], [8.457], [same exact input/output bytes],
 [auxiliary probe], [0.578], [quotient plus 1 independent torsion point],
 [normalized dual], [8.594], [dual construction plus 33 images],
)

These are Bash elapsed times under a watchdog of 120 seconds, address-space cap of 12 GiB and shared lock. The follow-up repeats were declared after the first call. Construction and per-point evaluation are not separately priced; RSS was not observed. No other build/audit/benchmark overlapped measured calls. Local L0 timing only; no PMU or reserved-core admission. [pcc:unverified]

The independent auxiliary torsion probe has nontrivial Weil pairing with the first kernel. Its nonzero image supplies the dual kernel, whose exact prime order is independently verified. The double quotient equation equals ell⁴/ell⁶-scaled E. All 33 compositions equal ell²/ell³-scaled [ell]P. With gcd(ell,r)=1, inverse scaling and multiplication by ell⁻¹ mod r recover the original subgroup. The general injectivity also follows because degree ell is coprime to r. Finite controls test implementation; they do not replace the isogeny theorem. [pcc:unverified]

Bounded qualification: 18 direct PARI kernel-polynomial maps,470 point-input evaluations over fields 71/11, square and nonsquare twists, three byte-identical legacy outputs and eight CLI/source rejection controls. Four further wrong-output/truncated-coverage controls reject. Immutable original evaluator and source/binary receipts remain preserved. [pcc:unverified]

Failed attempts are retained: initial build rejected external header macro/warnings; the include boundary was corrected. First control generator lacked its output directory. Initial map launcher lacked /usr/bin/time and exited127 before native execution; a separately named Bash-time launcher completed. No failed launch is counted as a constructed map. [pcc:unverified]

#pagebreak()
= Matched endpoint ECDLP and actual IC attempts
The existing word-field ecbench solver uses original prime r = 5629830654216971, one shared target and two measured rounds per endpoint. The quotient generator is the actual transported G. Four rho recoveries pass parent and independent PARI checks; all eight rho/IC deterministic attempts replay identically. [pcc:unverified]

#table(columns: (1fr,1fr,1fr), inset: 5pt,
 [Endpoint], [Rho mean seconds], [Outcome],
 @RHO@
)
#image("rho.svg", width: 100%)

Rho's coordinate hashing gives different walks/counts on the two models despite paired seeds. Canonicalization work is unpriced. Means are descriptive L0 observations; no endpoint ratio, conductor-related cause or speed ranking is admitted. This is four solve attempts of one target, not four independent targets or the full 32-target protocol. [pcc:unverified]

Index calculus actually ran on both word-field endpoints: ic.pipeline, prime-abscissa base size 8, rootfind-subtract m=2,10000 trials and 5 second solver budget. All four attempts exhaust 10000 trials, find zero relations and recover no logarithm. They are retained outcomes, not successful solve times. The four attempts take approximately11–12ms each; incomplete field costs and unpriced root/lookup/guard work remain in raw ledgers. This does not prove impossibility or a general complexity statement. [pcc:unverified]

The initial pilot plan attached IC parameters to a copied rho arm and was rejected before solving. The corrected spec selects by method ID. Inputs, order, seeds, caps, raw outcomes and replay receipts remain committed or in the complete raw bundle. Independent PARI checks confirm all eight target laws and all four recovered scalars. [pcc:unverified]

#pagebreak()
= 32-point scalar panel and requirement status
Existing scalar-timing CLI, counted four-limb library:32 bases per model, six measured rounds plus one warmup, batch64. All 672 batch outputs match independent PARI arithmetic. Native means/medians below exclude warmups. Bars/dots present native statistics; Python performs rendering and unit conversion only. [pcc:unverified]

#image("scalar.svg", width: 100%)
#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Model], [Mean µs], [Median µs], [Measured batches],
 @SCALAR@
)

Timer includes multiplication, loop/black_box and ledgers; setup/expected-result construction/verification/output are excluded. Repeated batch throughput is not cold single-call latency. This adapter differs from the word-field ECDLP backend, so these microseconds are not rho-step costs. The initial scalar launch used a nonexistent copied binary path and exited127 before producing samples; corrected named launcher preserves binary/spec. [pcc:unverified]

Requirement status: the retained 26-bit-prime crossing has an explicit quotient,33 images, standard CM conductor argument, normalized dual controls and matched pilot ECDLP measurements.32-base scalar measurements are checked. Full32-target ECDLP and admitted performance remain open. The original 80-bit attempt still has no quotient/images after its earlier 60second cap; this milestone does not discharge it. The next larger local-family proposal is NEXT_LOCAL.md, unexecuted here. [pcc:unverified]

Complete raw results, qualification and negative-control directories form one 61700-byte deterministic archive with per-file extraction and byte-identical repack checks. Its SHA256 is e33b1259ac0710fb7a1b8a87bb5091a5c0b8477356276cbd489b485430eec259. It naturally fits the configured 64 KiB data threshold. S3 remains unauthenticated; no remote upload/readback is claimed. Benchmark records remain open for publication; older oversized archives remain separate obligations. [pcc:unverified]

Sources: actual native/ecbench contracts, retained CM kernel/catalog/CONSTRUCTOR.md, PARI/FLINT APIs; Galbraith ANTS introduction/section2 and Galbraith–Gilchrist–Robert section2.1 (links in CM_ARGUMENT.md). KB unavailable. Producer evidence PCC-20261119-3b9f6e/21ae00/1da80c/a266fd/3731f3 remains unsigned/not_covered. No formal proof, world-record or performance-leaderboard promotion is asserted. [pcc:unverified]
'''
(out/'report.typ').write_text(text.replace('@RHO@',rho_rows).replace('@SCALAR@',scalar_rows))
md=f'''# Evaluated prime33555419 endomorphism-conductor crossing

Actual source/target conductors **1 →33555419**, a 26-bit prime ratio (log2 ratio 25.0000424361), over field 5629830687772387. Full quotient and 33 non-kernel original-subgroup images evaluated;32 matched map scalars and 33 normalized dual compositions independently checked. Original subgroup order 5629830654216971 retained. Standard CM order argument and executable arithmetic certificates are in [CM_ARGUMENT.md](CM_ARGUMENT.md). This is below the original 80-bit minimum. [pcc:unverified]

| Endpoint | Rho mean seconds | Verified attempts |
|---|---:|---:|
| Source | {groups[slugs[0]]['mean_solve_seconds']:.3f} |2 |
| Quotient | {groups[slugs[1]]['mean_solve_seconds']:.3f} |2 |

One matched target/two rounds, L0. Four rho recoveries and all eight rho/IC replays pass, with independent PARI checks. All four IC attempts exhaust 10000 trials with zero relations and no logarithm. No conductor-related speedup is inferred from different rho walks. [pcc:unverified]

Full construction plus 33 point evaluations took8.661,8.664,8.457seconds across three calls with byte-identical outputs. Scalar panel covers32 bases/model ×6 measured rounds plus warmup on source/image/u2; all 672 batch outputs independently checked. Separate construction/online transfer prices and performance admission remain open. [pcc:unverified]

[Four-page report](reports/report.pdf), [exact native evidence](summary.json), [map protocol](PROTOCOL.md), [ECDLP/IC protocol](PILOT_PROTOCOL.md), [scalar protocol](SCALAR_PROTOCOL.md), [complete raw bundle](raw-results.tar.xz) and [per-file archive inventory](archive-info.json). Extract the raw bundle here to restore all native raw directories. Exact build/launch/amendment/checker sources and failed attempts are preserved. [pcc:unverified]

The whole raw bundle is61700bytes, SHA256 e33b1259ac0710fb7a1b8a87bb5091a5c0b8477356276cbd489b485430eec259; no splitting or storage-policy exception. No S3 upload/readback succeeded (publication/sts.stderr); records stay open. All producer evidence is unsigned/not_covered. Native checks establish their stated scopes, not formal proof. [pcc:unverified]

Original 80-bit crossing and full 32-target ECDLP protocol remain unfinished. [Next larger-family proposal](NEXT_LOCAL.md) is unexecuted. No job is active after these completed runs. [pcc:unverified]
'''
(HERE/'README.md').write_text(md)
print('rendered figures and report source from native summaries')
