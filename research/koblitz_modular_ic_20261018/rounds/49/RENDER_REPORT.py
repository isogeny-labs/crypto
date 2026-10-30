"""Render recorded terminal receipts only; no mathematical computations."""
from pathlib import Path
import json,html
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());rows=r['runs'];c=r['construction_counts']
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="'+str(100+len(rows)*42)+'" viewBox="0 0 1000 '+str(100+len(rows)*42)+'"><rect width="100%" height="100%" fill="#f7fafc"/><g font-family="sans-serif"><text x="24" y="30" font-size="21">R49 recorded execution flow</text>']
for i,x in enumerate(rows):
 y=65+i*42;label=x['attempt']+' — '+x['status']
 if x.get('timeout'):label+=' (1200 s cap)'
 if x.get('compiler_started') is False:label+=' (checkpoint gate; compiler not started)'
 color='#145c43' if x['status']=='accepted' else '#9a3412'
 svg+=['<rect x="24" y="'+str(y-21)+'" width="952" height="33" rx="5" fill="#e8eef5"/><text x="38" y="'+str(y)+'" font-size="16" fill="'+color+'">'+html.escape(label)+'</text>']
svg+=['<text x="24" y="'+str(83+len(rows)*42)+'" font-size="14">Receipts only. No speed, IC-rank, proper-map or larger-n transfer claim.</text></g></svg>'];(h/'round49.svg').write_text(''.join(svg))
intro='These are exact-computation observations and execution receipts, not an independent scientific verdict or a performance comparison.'
body='''The smooth and cusp controls exercise positive-characteristic normalP and full norTest. The source preparation reconstructs the twisted source from the original Tate/QR equations, checks both generic ideal inclusions, eight registered units, and degree 32 over F2(u). A compact chart retains b,c, descends all five equations to F2, and agrees with the accepted plane generic ideal at degree 32. These are input and function-field checks; neither preparation is a normalization result.

Both affine normalization attempts timed out in minimal-prime decomposition before exporting a selected component or reaching normalP. Their frozen programs require all minimal primes, a unique component mapping to the degree-32 source field, and normalP without an assumed-prime shortcut. A computed output would still need the separately frozen verifier: source identities, corruption control, prime equality, full norTest, and normalized generic degree. Unfinished decomposition is an incomplete calculation. It is not a geometric impossibility or a mathematical null result.

The conditional Lean lemmas give relative degree one and bijectivity for a field tower with equal positive degrees. Actual fraction fields, compatible maps, native ideals and generic-degree values are hypotheses to instantiate, not formalized CAS data. The first compile failed because a narrow import omitted the standard vector-space freeness instance; the repaired source imports it. A subsequent launch failed at a checkpoint upload HTTP 503 before compiler start; no compile or audit receipt exists for that launch. All failures remain indexed.

Independent exact CAS stages overlapped on a shared host. A live observation found each of the two active presentation processes receiving about half a CPU. Wall-cap outcomes are censored under that co-tenancy; no isolated runtime comparison or feasibility bound is inferred. Subsequent normalization is admitted after these processes terminate. Native runs keep the 4 GiB virtual-memory and 1200 s per-invocation limits. Lean uses 4.34.1, mathlib d13f23b723b8a846827a245b89c10fc7d3f11612, one thread, 4096 MiB and 1200 s per compile/audit. No new EKS job, Diem restart, freeze restart, Conductor claim or merge occurred. Independent scientific replay and semantic review remain pending.

Proper projective charts/gluing, complete pole divisors and prescribed RR spaces, the chosen level-77 factor map and nonzero order-11 action, full integral four-map recovery, arbitrary reverse principal divisors, quotient saturation/polarization, theta/node187 and genuine larger-n transfer remain open. Existing n=3/n=5 controls remain retained. No faster-IC assessment is inferred.
'''
table='| Attempt | Outcome | Receipt |\n|---|---|---|\n'
for x in rows:
 receipt='checkpoint HTTP503; no compiler' if x.get('compiler_started') is False else ('native exit '+str(x['native_exit']) if x['kind']=='Singular' else 'compile '+str(x['compile_exit'])+', audit '+str(x['audit_exit']))
 table+='| '+x['attempt']+' | '+x['status']+(' (timeout)' if x.get('timeout') else '')+' | '+receipt+' |\n'
body+='\nA partial primitive shape line is a candidate only. It is not counted as a field presentation unless the entire producer passes its original-equation and reverse-identity checks. Missing lambda completion data stays missing; an empty log does not identify the step consuming the cap.\n'
body+='\nGeneric source receipt: '+'; '.join(r.get('generic_source_result_lines',[]))+'\n\nPrimitive presentation receipt: '+'; '.join(r.get('primitive_result_lines',[]))+'\n\nLambda construction receipt: '+('; '.join(r.get('lambda_result_lines',[])) or 'No completed result line; wall cap reached.')+'\n'
counts=json.dumps(c,indent=2)
(h/'REPORT.md').write_text('# R49: explicit twisted-source charts and normalization attempts\n\n'+intro+'\n\n'+table+'\nRecorded construction counters (not performance):\n\n```json\n'+counts+'\n```\n\n'+body+'\nSee PROTOCOL.md, COMPACT-NORMAL-PROTOCOL.md, LEAN-PROTOCOL.md, PROOF_MAP.md, results.json and ARTIFACTS.md. The SVG displays recorded statuses because no comparable timing measurements were taken.\n')
quote=lambda s:json.dumps(s,ensure_ascii=False)
typ='#set page(paper: "a4", margin: 17mm)\n#set text(size: 10pt)\n= R49: source charts and normalization attempts\n'+intro+'\n#image("round49.svg", width: 100%)\n\n== Recorded counters\n'
for k,v in c.items():typ+='#text('+quote(k.replace('_',' ')+': '+str(v))+')\n#linebreak()\n'
typ+='\n== Method and scope\n'+body
(h/'report.typ').write_text(typ)
print('Rendered report source and recorded-outcome SVG for',len(rows),'terminal attempts')
