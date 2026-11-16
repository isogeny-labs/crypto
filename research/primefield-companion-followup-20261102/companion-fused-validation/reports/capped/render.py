from pathlib import Path
import json,typst,fitz
D=Path(__file__).resolve().parent;V=D.parent.parent
r=json.loads((V/'terminal-reconciliation.json').read_text());arms=r['by_arm']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="200"><rect width="800" height="200" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="18">Completed context rows (declared 849 per arm)</text>'
for i,a in enumerate(['candidate','aa']):
 y=55+i*65;n=arms[a]['rows'];w=550*n/849
 svg+=f'<text x="20" y="{y+24}" font-family="sans-serif" font-size="17">{a}</text><rect x="125" y="{y}" width="550" height="34" fill="#eeeeee" stroke="#999"/><rect x="125" y="{y}" width="{w}" height="34" fill="#286e89"/><text x="690" y="{y+24}" font-family="sans-serif" font-size="17">{n}</text>'
svg+='</svg>';(D/'coverage.svg').write_text(svg)
t='''#set page(paper:"a4",margin:17mm)
#set text(size:10.5pt)
= Fused operator: original cap reconciled
2026-11-01 · Recorded candidate completion and partial A/A coverage

The original 3,600-second invocation ended at its preserved deadline.
Producer and follower both returned 124; both process handles are absent.
No restart, budget extension or replacement invocation occurred.

#image("coverage.svg",width:100%)

*Recorded completed work.* Candidate: 849 contexts and 17,124 products,
including 16,164 archived products and 960 carry products. Recorded controls:
68,496 rejected mutations, 51,372 equal-wrap rejections and 1,920 radix-guard
rejections. A/A: 28 contexts, 2,750 archived products, 11,000 rejected mutations,
8,250 equal-wrap rejections. All 28 corresponding payload hashes match.
These figures are recomputed from the complete raw rows, not forecasts.

*Coverage.* The declared two-arm panel has 1,698 contexts and 34,248 products.
Completed: 877 contexts. The remaining 821 lack completion receipts.
The unfinished attempted-row count is unknown because no per-row start receipt
was emitted. No A/A carry row completed. Full A/A and full-panel export are
absent; the integration execution gate remains closed.

*Custody and inspection.* All 431,414,955 raw bytes are retained, with no
partial trailing row. Reconciliation checked row order and declared seeds,
payload hashes, matched A/A payloads, fixed recorded kernel counts and native
multiplication descriptor sums. Ten frozen source locks and two input locks
were reverified. This audit reads recorded data; it does not replay kernels
or replace the producer's independent-reference checks with new trials.

```text
Raw SHA-256:
db199b944109851a17bbd77b3e0d547310d62b89bcaf538804288d06357ce4a2
Protocol SHA-256:
21e18b0471cccf698b0ffc2f82e02fe3e393f86004813af3d41a145aa1a0c843
```

*Provenance.* Predata commit 3e77b2c. Original producer 2245648 and follower
2245741. Deadline epoch 1793576987.8451536. Native exit-code.txt,
postprocessor-exit.txt and postprocess-state.json establish terminal status.
terminal-reconciliation.json contains all row offsets and hashes;
reconcile_capped.py is the separate custody script. Historical launch reports
retain their original observation dates.

*Next.* Preserve this invocation and all failures. The candidate arm provides
complete recorded component coverage, while the missing A/A portion stays
explicit. Identify the cost of preparing and serializing independent receipts
before designing any fresh full-cohort validation; a new run would require
its own approved finite budget and committed contract. Recursive inverse and
pivot constructions remain syntax-checked, unexecuted follow-up work.

Counts retain their own units; no elapsed-time or peak-memory comparison is
inferred. The displayed graph reports completed rows only, with no statistical
uncertainty model or extrapolation.
'''
(D/'report.typ').write_text(t);typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc))
