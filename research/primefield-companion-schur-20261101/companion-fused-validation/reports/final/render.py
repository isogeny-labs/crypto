from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,typst,fitz
D=Path(__file__).resolve().parent;E=D.parent.parent;R=E.parent
assert all((E/n).read_text().strip()=='0' for n in ['exit-code.txt','analysis-exit.txt','export-check-exit.txt','postprocessor-exit.txt'])
summary=json.loads((E/'runs/recomputed-summary.json').read_text());assert summary['cases']==1698
samples=[];totals={'candidate':{},'aa':{}};oldtotals={};rows=0
for line in (E/'runs/cases.jsonl').open():
 z=json.loads(line);rows+=1
 for t in z['payload']['tests']:
  for k,v in t['counts'].items():totals[z['arm']][k]=totals[z['arm']].get(k,0)+v
  if z['arm']=='candidate' and 'archived_counts' in t:
   for k,v in t['archived_counts'].items():oldtotals[k]=oldtotals.get(k,0)+v
  if z['arm']=='candidate' and z['instance_id'] in ['18','27','36'] and t['stage']=='full' and t['label']=='ones' and not t['transpose']:
   samples.append(dict(instance=z['instance_id'],r=z['payload']['plan']['r'],n=t['n'],s=t['s'],baseline=t['archived_counts'],candidate=t['counts']))
assert rows==1698 and totals['candidate']==totals['aa']
for lock in ['source-lock.json','input-lock.json']:
 for p,want in json.loads((E/lock).read_text()).items():
  h=hashlib.sha256()
  with (R/p).open('rb') as f:
   for block in iter(lambda:f.read(1048576),b''):h.update(block)
  assert h.hexdigest()==want,p
(D/'chart-data.json').write_text(json.dumps(dict(summary=summary,samples=samples,candidate_totals=totals['candidate'],archived_baseline_totals=oldtotals),indent=2)+'\n')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="760" height="240"><rect width="760" height="240" fill="white"/><g font-family="sans-serif" font-size="15"><text x="10" y="24">Final coefficient reductions per full-source query (count)</text>'
maxv=max(z['baseline']['coefficient_reductions'] for z in samples)
for i,z in enumerate(samples):
 y=54+60*i;old=z['baseline']['coefficient_reductions'];new=z['candidate']['coefficient_reductions']
 svg+=f'<text x="10" y="{y+12}">r={z["r"]}</text><rect x="100" y="{y}" width="{500*old/maxv}" height="16" fill="#a9b6bf"/><text x="{108+500*old/maxv}" y="{y+13}">{old}</text><rect x="100" y="{y+21}" width="{max(1,500*new/maxv)}" height="16" fill="#286e89"/><text x="{108+max(1,500*new/maxv)}" y="{y+34}">{new}</text>'
svg+='</g></svg>';(D/'reductions.svg').write_text(svg)
lines=[]
for z in samples:
 a=z['baseline'];b=z['candidate']
 lines.append(f'- r={z["r"]}, n={z["n"]}, width={z["s"]}: native products {a["integer_multiplications"]} → {b["integer_multiplications"]}; packing bytes {a["packing_bytes"]} → {b["packing_bytes"]}; operand-bit-length product sum {a["operand_bitlength_product_sum"]} → {b["operand_bitlength_product_sum"]}.')
t='''#set page(paper:"a4",margin:18mm)
#set text(size:10.5pt)
= Fused companion application: full validation
Terminal exact finite-panel report · '''+datetime.now(timezone.utc).isoformat()+'''

All 1698 contexts and 34248 products completed under the frozen contract.
Every archived product was retained: 32328 across both arms. Carry fixtures
added 1920 products. Candidate/list and AA/tuple payloads agree exactly.
Producer, analyzer, export checker and follower all exited zero.

Checks include archived dense-reference equality, independently reconstructed
native arithmetic/bit volumes, every carry-fixture intermediate against
coefficient-list convolution, and fixed field/native/byte formulas. Controls:
136992 rejected output/count/trace/sign mutations; 102744 equal-wrap rejections;
3840 proof-bound radix rejections. The separate explicit overflow certificate
uses radix256 and coefficients [256,1], decoding to [0,2].

#image("reductions.svg",width:100%)
Gray is the archived per-product operator; blue is the fused operator.
The three full-source ones-vector queries use identical public generators.
These are counted reductions, not elapsed times or peak-memory measurements.

*Construction.* Positive packed coefficients survive convolution, extraction,
folding, column accumulation and inverse scaling without carry when radix
exceeds s n² (r−1)⁴ (1+a)(1+b). Only n final coefficients are reduced. The
query performs one field subtraction and inversion. Native work includes
4s+8 products; its wider radix and other wide operations remain charged.

*Native tradeoff on those same queries.*
'''+ '\n'.join(lines)+'''

#pagebreak()
= Scope, provenance and continuation

*Protocol.* Predata commit 3e77b2c; hash
21e18b0471cccf698b0ffc2f82e02fe3e393f86004813af3d41a145aa1a0c843.
The predata seed audit checked all1698executioncells. Ten source locks and two
input archive locks reverified. One invocation, 3600-second cap, no restart.
All rows and native events retained in runs/cases.jsonl; row-index.json binds
individual rows. Full export and recomputed summary are retained.

*Reference roles.* The 369 archived contexts per arm supply full, inverse and
principal-block generators and every original test vector. Their saved
outputs were independently dense-verified by the earlier block panel. No
prior producer was rerun. The 480 carry contexts per arm cover primes
7,13,19,43; dimensions1,2,r−1,r,r+1; widths1,3,7; four wrap pairs including
zero wraps; maximum and seeded digits; both directions. This is exhaustive
for the declared finite panel, not for arbitrary inputs.

*Units.* The graph counts modular coefficient reductions. Native product
counts, operand-bit-length products, add/subtract/shift/mask bit volumes,
packing, intermediate serialization, reversal and decoding bytes are separate
ledger fields. Checker/trace overhead, Python loop/index work and allocation
are outside the kernel arithmetic ledger. Aggregated candidate totals include
carry fixtures; archived baseline totals do not, and are not compared as
matched workloads. Per-query examples above are matched.

*Next construction.* Use the validated operator in a separately frozen Schur
construction comparison, preserving every original context and obstruction
control. Leading-block solves still use explicitly charged reference oracles.
Validate primal/dual lifting and pivot handling before replacing them with a
recursive backend. The capped original Schur run remains immutable.

*Files.* CONTRACT.md, protocol.json, source-lock.json, input-lock.json;
runs/cases.jsonl, row-index.json, summary.json, recomputed-summary.json,
carry-certificate.json and export.json; producer.log, analysis.log,
export-check.log and terminal exit receipts. Chart data, editable Typst source
and renderer accompany this report.
'''
(D/'report.typ').write_text(t)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'));doc=fitz.open(D/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc));print(json.dumps(samples))
