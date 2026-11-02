from pathlib import Path
import json,hashlib,typst,fitz
D=Path(__file__).resolve().parent;E=D.parent.parent;R=E.parent
rows=[json.loads(x) for x in (E/'runs/cases.jsonl').read_text().splitlines()]
s=json.loads((E/'runs/summary.json').read_text())
assert s['verified'] and len(rows)==162 and (E/'exit-code.txt').read_text().strip()=='0'
a=[x for x in rows if x['arm']=='candidate' and x['r'] in [359,479,683] and x['q']>1 and int(x['instance_id'])%3==0]
assert len(a)==9
(D/'chart-data.json').write_text(json.dumps([dict(r=x['r'],q=x['q'],instance_id=x['instance_id'],counts=x['counts']) for x in a],indent=2)+'\n')
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="370"><rect width="760" height="370" fill="white"/><g font-family="sans-serif" font-size="15"><text x="10" y="22">Constructor field products: gamma = 2, candidate arm</text>']
mx=max(x['counts']['field_products'] for x in a)
for i,x in enumerate(a):
 y=42+i*33;v=x['counts']['field_products'];w=440*v/mx
 svg.extend([f'<text x="8" y="{y+17}">r={x["r"]}, q={x["q"]}</text>',f'<rect x="150" y="{y}" width="{w}" height="24" fill="#276b87"/>',f'<text x="{160+w}" y="{y+17}">{v:,}</text>'])
svg.append('<text x="10" y="360">Exact integer counts; 9 cases shown from 162. No elapsed-time inference.</text></g></svg>')
(D/'products.svg').write_text(''.join(svg))
started=min(x['started_at'] for x in rows);finished=max(x['finished_at'] for x in rows)
t=f'''#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Direct companion generators: complete finite validation
2026-11-01 · Exact component computation · direct-companion-validation

*All 162 cases passed.* The public generator constructor agrees with an
independent dense feature construction across 27 coefficient-field contexts,
three gamma labels, and two arms. The normalized dual-witness panel
passed all 66 cases. All 1,038 corruption controls and 324 invalid-gamma
rejections passed. Candidate and tuple-weight A/A outputs and counts agree
exactly. The single producer invocation returned 0; no restart was used.

*What was checked.* The checker forms quadratic characters using square sets,
then independently constructs the full weighted feature Gram matrix B and
boundary matrix H. It checks B - gamma T B Tᵗ = H and the companion identity
Z_(r,gamma) A - A Z_(r,1) = G Kᵗ entrywise. For orders at most 19 it also
uses explicit companion matrices. A=D⁻¹ J B J, where J reverses coordinates
and D=diag(gamma^i). Coprimality follows from the nonzero difference of
X^r-gamma and X^r-1; gamma^r=gamma is checked in every case.

*Transport.* Every case checks a random known primal vector and a general
residual-dual vector. For all width-one contexts and the retained public
r13q4 and r479q15 exclusion witnesses, normalization and nullspace identities
are checked on both sides. These are tests of transport, using known public
witnesses; no structured solve backend is involved. Old-compiler labels,
weights and inverse-Pascal boundary coordinates are checked independently.

*Controls.* Six deliberate changes per case alter G, K, the left wrap,
inverse diagonal, RHS or field-product count. Each is rejected by full
constructor-output/count equality against the independent reference. The
66 normalized cases also reject a zero witness. Gamma 0 and 1 are rejected
in each case. These controls test the named validators, not every possible
corruption of a future solver.

*Panel.* Original contexts: (7,6),(13,12),(19,15),(43,15),(73,15),(349,15).
Larger contexts: r359 with q13–15; r479 with q15–17; r683 with q16–18.
Width one is checked for all nine orders. Retained preflight contexts:
(7,3),(13,4),(19,4). Gamma labels are 2, r-1, and a frozen seeded draw;
coincident values retain their separate labels. Weights and random vectors
use seed 20261101+100000r+100q+gamma-index. Gamma seed: 20261101+100r+q.

*Custody and execution.* Predata commit 8a4876f; nine source locks and the
empty input lock are retained. Command: timeout 900 ../.audit-venv/bin/python
-u direct-companion-validation/check.py. Exact row timestamps span
{started} to {finished}. Shared local host; no valid timing, CPU or peak-RAM
measurement was collected. All 162 planned cases completed; zero failed,
interrupted or excluded cases. Protocol and export are checked separately.

#pagebreak()
= Constructor accounting and next backend
#image("products.svg",width:100%)
Recorded field-product counts from runs/cases.jsonl, candidate instances
18,21,24,27,30,33,36,39,42. Data and SVG renderer are retained beside this PDF.
Each bar is one exact case; uncertainty bars are not applicable. A/A covers
all 81 matched instances, not just these nine illustrated rows.

*Exact ledger.* Let K be the number of offset patterns, N the feature count,
s=2K+1=3+q(q-1), and pc(e)=popcount(e)+bitlength(e)-1. The checked constructor
has r pc((r-1)/2) + 2r sum_P |P| + sum_P pc(q-max(P)) + N-1 + 2r + 2rs
field products; 1+K+rs field additions; and one field inversion. The
constructor retains O(rs) generator data. Dense-checker and old-reference
work are excluded from this constructor ledger. No total compiler cost,
runtime improvement or memory reduction is inferred from these counts.

*Concrete next dependency.* The companion representation and transport now
have a complete finite certificate panel. The next component is a structured
matrix multiplication and singular-system backend that consumes G,K and
returns separately checkable primal solutions or normalized dual witnesses.
The base-field preconditioning guarantee must be treated explicitly; the
existing full prime-field recovery instrument remains a separate artifact.

*Evidence paths.* runs/summary.json; runs/cases.jsonl; runs/export.json;
source-lock.json; protocol.json; producer.log; exit-code.txt; launch.json;
export-check.log. The derivation and constructor are in
../direct-cyclic-companion/DERIVATION.md and generators.py. This report is a
fresh result record and does not rewrite the earlier unexecuted-construction
report. Provenance and file hashes are retained at the experiment root.
'''
t=t.replace('_',chr(92)+'_')
(D/'report.typ').write_text(t);typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc))
