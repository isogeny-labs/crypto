#!/usr/bin/env python3
"""Render exact receipt observations only; no mathematical replay."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
root=Path(__file__).resolve().parent.parent
for rid in (37,38):
 h=root/str(rid);r=json.loads((h/'results.json').read_text());formal=r['formal']
 if rid==37:
  assert r['signed_class']['exact_order_under_retained_bridge']==671
  c=r['cofactor_class_tests'];labels=['E','11E','5E','3721E','11·3721E','671E'];values=[int(r['signed_class']['principal']),int(r['signed_class']['eleven_annihilates']),int(r['signed_class']['five_annihilates']),int(c['cofactor_class_principal']),int(c['eleven_cofactor_principal']),int(r['source_order_test']['six_seventy_one_principal'])]
  fig,ax=plt.subplots(figsize=(8,3));ax.scatter(values,range(len(labels)),s=80,color='#367e73');ax.set_yticks(range(len(labels)),labels);ax.invert_yaxis();ax.set_xticks([0,1]);ax.set_xlim(-.15,1.15);ax.set_xlabel('Exact equality-witness dimension against zero');ax.set_title('R37 actual scalar classes: 0 = nonprincipal, 1 = principal');ax.grid(axis='x',alpha=.25);fig.tight_layout();fig.savefig(h/'round37.svg');plt.close(fig)
  (h/'plot-data.json').write_text(json.dumps({'runs':['signed37-01','torsion37-01','isolate37-01','classorder37-01'],'labels':labels,'values':values,'unit':'F2 dimensions','uncertainty':'not applicable: exact computation'},indent=2)+'\n')
  text=f'''#set page(paper:"a4",margin:16mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz transfers — R37
*Exact source classes and finite cofactor; no IC performance claim*

The actual deck order is bound through all four point coordinates, complete
83-power Frobenius orbits and680 saved literal function values. The signed
class E is evaluated in two independent expressions, with equal results.

#image("round37.svg",width:100%)

Both E and11E are nonprincipal. The official LMFDB coefficient data for the
complementary form77.2.l.b give P_b(-1)=3721=61². All16 Hecke-basis inverse
rows, the character/a4 recurrence and both independent matrix norms pass;
the complete Euler polynomial of degree32 is retained.

The classes3721E and61(61E) agree and are nonprincipal, while11·3721E is
principal. The predicted full mixed-carrier annihilator450241E is principal.
The final671E test is principal. With the preceding nonzero multiples this
gives exact source order671=11·61 and least positive cofactor61 to11-torsion,
under the retained proper-curve/local/complete-RR/pole bridge. The actual saved
class61E has order11. The finite projector122=2·61 is idempotent on this cyclic
group, with value one modulo11 and zero modulo61.

All subspace divisions use every original generator, with residual/kernel,
rank-nullity, recombination and final membership certificates. Every product
is checked by two different arithmetic implementations. These internal checks
are not an independent external scientific replay.

Lean: {formal['audited_declarations']} audited declarations in {formal['accepted_runs']} accepted runs;
{formal['rejected_runs']} rejected compile attempts are retained. The proofs check abstract
weight, torsion, source-order and finite-projector implications. Native arrays,
Picard implementation, normalization and factor maps are not formalized.

The intended CM-sign image remains unknown. Full quotient saturation, proper
polarized embedding, theta, node187, arbitrary collectors and larger supports
remain open. Native4GiB/1200s and pinned Lean-j1-M4096/1200s caps unchanged.
Global metric integration is pending exclusive path ownership; scoped patch
included. No timing comparison, IC relation rank or scalar recovery claim.

Sources: Khuri-Makdisi math/0409209v3; official LMFDB77.2.l / mf_hecke_nf;
retained R15/R16 CM Euler/sign obstruction and R27 mixed-Prym bridge.
'''
 else:
  old=json.loads((root/'35/results.json').read_text());labels=['R35 accepted algebra','R38 accepted algebra'];values=[old['construction_counts']['closed_point_algebra_dimension'],r['construction_counts']['closed_point_algebra_dimension']]
  fig,ax=plt.subplots(figsize=(8,2.3));bars=ax.barh(labels,values,color=['#44779b','#398675']);ax.invert_yaxis();ax.set_xlim(0,3400);ax.bar_label(bars,padding=4);ax.set_xlabel('Exact algebra dimension over F₂; no timing comparison');ax.set_title('Verified construction dimensions at local precision154 and615');fig.tight_layout();fig.savefig(h/'round38.svg');plt.close(fig)
  (h/'plot-data.json').write_text(json.dumps({'runs':['R35/algebra35-01','R38/algebra38-01'],'labels':labels,'values':values,'unit':'F2 dimensions','uncertainty':'not applicable: exact computation'},indent=2)+'\n')
  text='''#set page(paper:"a4",margin:17mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz transfers — R38
*Larger exact evaluation algebra; no IC performance claim*

At the certified degree-five point Q, construct F2[t]/(t⁵+t³+1)⁶¹⁵, dimension
3075. Every one of the3075 inverse columns and all pivots are recombined.
The actual coefficient-field embedding, ring identities and corrupt inverse/
field controls pass under the unchanged native4GiBvirtual/1200s caps.

#image("round38.svg",width:100%)

A pinned Lean lemma proves coefficient extraction at precision615 under
explicit characteristic-two, residue-period32 and nilpotence hypotheses.
It compiles and passes the transitive axiom audit with zero failing and zero
trusted declarations. Native arrays and proper geometry are not formalized.

For the proposed baseD1=4D0 of degree512, evaluation at615Q has degree3075,
exceeding the L(6D1) pole bound3072 under the retained proper/pole bridge.
This can support the next larger construction, but615 source jets and the
complete section spaces of prospective dimensions452,1476,3012 are not built.

The degree367 gap can use29Q. The degree460 gap52 needs additional certified
support or a local line-bundle trivialization; degree-five copies alone do
not fill it. Actual larger support evaluation and factor images remain open.

No larger-n Koblitz transfer, IC relation rank, speed comparison, scalar
recovery or independent external replay is claimed. Lean4.34.1/mathlibd13f23b,
-j1-M4096/1200s. Global metric integration awaits exclusive scope release.
See R37/METRICS_PENDING.md, NEXT.md and retained exact inputs/certificates.
'''
 (h/'report.typ').write_text(text);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'));print('Rendered',rid)
