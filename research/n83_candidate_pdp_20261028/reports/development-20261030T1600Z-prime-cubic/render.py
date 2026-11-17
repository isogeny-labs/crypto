from pathlib import Path
import json,hashlib,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');r=Path('/home/ec2-user/n83-candidate-recovery')
d=json.loads((s/'validation-prime-cubic-development/summary.json').read_text())
custody=json.loads((s/'evidence/prime-cubic-development-storage.json').read_text())
out=s/'reports/development-20261030T1600Z-prime-cubic';out.mkdir()
rows=d['dense_controls'];fig,ax=plt.subplots(figsize=(8,2.7),layout='constrained')
labels=[f"ell={x['ell']}, p={x['p']}" for x in rows]
ax.bar(range(6),[x['columns'] for x in rows],color='#d1d5db',label='Columns')
ax.scatter(range(6),[x['dense_rank'] for x in rows],color='#126b8b',label='Dense rank = sparse rank',zorder=3)
ax.set(xticks=range(6),xticklabels=labels,ylabel='Dimension',title='Small-prime development controls; m=3')
ax.legend();fig.savefig(out/'controls.svg');plt.close(fig)
# Canonicalize renderer whitespace before hashing, upload or inspection.
p=out/'controls.svg';p.write_text('\n'.join(x.rstrip() for x in p.read_text().splitlines())+'\n')
cells='\n'.join(f"[{x['m']}],[{x['columns']}],[unmeasured]," for x in d['production_layout_only'])
text=f'''#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Serif",size:9pt)
= Cubic coefficient-map development
*Development controls only. No n83 cubic production or new FFD/LFD result.*
The implementation and raw controls prepare the degree-seven coefficient map.
Current source-bound conditional FFD intervals7–17 remain unchanged.
#image("controls.svg",width:100%)
*Recorded checks.* Six small-prime maps have matching sparse and independent dense ranks:2/16,34/84,181/260 for ell1,2,3 at each of primes11 and101. The independent terminal-square control at ell8 has rank56/56 at both primes. A seven-selector terminal cubic annihilator is retained explicitly; it demonstrates why the previous projection size cannot simply be reused. It is not an original27-selector kernel.

*Coverage and correctness.* All ordered triples of local indices0–26 are covered by nine-selector subsets. A pair-complete but triple-incomplete family is rejected. Cubic monomials exclude repeated selectors but retain intermediate cubes. Independent quartic expansion, four deterministic cap reasons, replay and corrupted-journal controls pass.28 native invocations comprise12 new and16 duplicated donor invocations. The S3-restored test executable passes all28 again; its raw control artifacts are retained separately. The development CLI rejects an83-bit prime without creating output.

*Prospective domain sizes: exact combinatorial counts, not measured ranks.*
#table(columns:3,inset:3pt,[m],[Columns],[Production rank],{cells})
*Interpretation.* Full injectivity, triple covering and the source-normalization argument could provide conditional FFD≥8. Deficiency,caps and interruption cannot provide that bound. Production requires its own frozen resource envelope and all42 source identities. No exactFFD,newLFD,regularity,solving-degree,performance or elite-base improvement is established. The full three-family m2–9 objective remains open.

*Retention.* PRIME_CUBIC_FORMAT.json describes sparse monomials,rows,pivots,journal events and control files. The archive contains source,executables,logs and fixtures; it was restored from S3 and hash-verified before and after replay. Source:{custody['source_revision']}. Archive SHA256:{custody['archive']['sha256']}.

Definitions:#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla,Definitions1.3–1.5 and Section4]. Study-specific argument:PRIME_CUBIC_DESIGN.md. Independent scientific review pending.
'''
(out/'report.typ').write_text(text);shutil.copy2(__file__,out/'render.py');shutil.copy2(s/'validation-prime-cubic-development/summary.json',out/'plot-data.json')
typst.compile(str(out/'report.typ'),output=str(out/'report.pdf'))
doc=fitz.open(out/'report.pdf');f=r/'prime-cubic-development-final';f.mkdir(exist_ok=True)
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.4,1.4)).save(f/f'report-page-{i+1}.png')
identity={'directory':str(out),'pages':len(doc),'sha256':hashlib.sha256((out/'report.pdf').read_bytes()).hexdigest()}
(f/'report-identity.json').write_text(json.dumps(identity,indent=2)+'\n');print(json.dumps(identity))
