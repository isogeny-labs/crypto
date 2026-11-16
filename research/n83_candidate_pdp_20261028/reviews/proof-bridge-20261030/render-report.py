from pathlib import Path
import json,hashlib,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst,fitz
p=Path('/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028/reviews/proof-bridge-20261030')
fig,ax=plt.subplots(figsize=(10,3.4));ax.set(xlim=(0,10),ylim=(0,3.4));ax.axis('off')
nodes=[(.1,2.15,2.8,.95,'Graded joint review\nNo top obstruction below 17\nExplicit degree-17 syzygy','#dcebf7'),(3.5,2.15,2.8,.95,'Filtered joint review\nBounded descent: V16 = W16\nSource functional separates g','#dcebf7'),(7,2.15,2.8,.95,'Matching prime template\nFirst strict fall = 17\nLFD ≥ 17','#d9eedf'),(.1,.35,2.8,.95,'Fresh Lean / exact Rust\n8 arithmetic declarations\n1 saved case replayed','#eeeeee'),(3.5,.35,2.8,.95,'Independent blind derivation\nRe-derives degree-17 onset\nCentered all-target certificate','#fff0cc'),(7,.35,2.8,.95,'Next construction check\nNative centered normal form\nExceptional-target controls','#fff0cc')]
for x,y,w,h,label,color in nodes:
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=.04',facecolor=color,edgecolor='#52616b'))
 ax.text(x+w/2,y+h/2,label,ha='center',va='center',fontsize=9)
for a,b in [((2.95,2.63),(3.4,2.63)),((6.35,2.63),(6.9,2.63)),((6.35,.82),(6.9,.82)),((4.9,1.35),(4.9,2.05))]:ax.annotate('',xy=b,xytext=a,arrowprops={'arrowstyle':'->','color':'#52616b','lw':1.4})
fig.tight_layout();fig.savefig(p/'dependencies.svg');plt.close(fig)
f=p/'dependencies.svg';f.write_text('\n'.join(x.rstrip() for x in f.read_text().splitlines())+'\n')
(p/'diagram-data.json').write_text(json.dumps({'kind':'proof-dependency diagram','nodes':nodes,'source':'claim-status.json; exact reports in reviewers/','measured_performance':False},indent=2)+'\n')
text='''#set page(paper: "a4", margin: 17mm)
#set text(size: 9.5pt)
#set heading(numbering: none)
= Prime n83: independent proof-bridge review
Technical packet · T-995 · 30 October 2026

Three independent reports support the degree-17 first-fall result for the stated prime-field selector/Semaev chain: 27 selectors per block, m = 3–9, characteristic above 27, and nonzero normalization weights. The graded reviewer verifies absence of non-Koszul highest-form syzygies through 16 and an explicit witness at 17. The filtered reviewer proves bounded representation descent and separates the saved remainder from V16 while deriving it in V17. The blind reviewer independently derives the onset.

#image("dependencies.svg", width: 100%)
#text(size: 8pt)[Dependency diagram, not a performance measurement. Sources: the three verbatim TASK-N83 reports, machine/verification.json and claim-status.json. Blue: reviewed mathematical joints; gray: executed checks; yellow: new construction and next check.]

== New construction: the centered target is covered
The blind derivation pairs centered selectors y_j = z_j − w_j/2. An alternating projection annihilates every bounded-product row through degree 16, but is nonzero on the new terminal remainder for every target. Its degree is 16 generically and 15 at t = sum(w_j)/2. At that target the surviving coefficient is (w_26/2)^2. The derivation is preserved in TASK-N83-BLIND/report.md; next, implement the centered projection in Rust and independently test the exceptional-target case.

== Exact checks completed
The independent Rust verifier checked one saved case, original-prime-m3-natural: exact primality certificate, 17 supplied file hashes, six protocol source hashes, full source reconstruction, polynomial codecs and factors, circuit, coefficient polynomial, full remainder and pairing. It recomputed degree 16 and the nonzero pairing 4087116827087420414291699. The existing producer panel has 42 records; this independent replay covers one.

Fresh Lean 4.34.1 compilation and declaration/axiom audit passed all eight arithmetic lemmas. Six controls correctly accepted the known counterexample and rejected a false bound, missing declaration, sorry, custom axiom and native bypass. The S3-restored source reproduced the Lean axiom log. The full polynomial formalization still requires the string decomposition, Koszul transport, filtered descent and centered projection; no independent kernel rechecker was available.

== Degree interpretation and retained scope
The reviewed template gives first strict fall 17 and LFD at least 17. No numerical LFD upper bound is supplied. The blind argument gives highest-form quotient dimension ceil((m−2)/2), so there is no first zero Hilbert-component cutoff for this m3–9 ideal. Castelnuovo–Mumford regularity and solving degree were not computed. These candidate x-domain equations are distinct from the expanded point-lookup ideal. The full fixed-base Koblitz/binary/prime m2–9 study is retained.

== Independence, deviations and custody
The precommitted independence plan passed all three attestations; the checker validates declared boundaries, not operating-system isolation. A separately committed supplement repaired the filtered snapshot's missing raw inputs. The initial handoff omitted the native-language policy: the Python attempt and failure are retained, and the accepted replay uses the independently implemented Rust verifier. Two projection roles are documented separately: the old syzygy-coordinate map kills U, while the functional retains U.

Producer revision: c64f02c176787dbf40d455f95b75bd2dd5bfe5bf. All reports, raw builds, controls and failures are retained in Git/S3 according to FORMAT.json and the custody records. Producer records remain immutable. This packet records a technical review without an official ledger transition.
'''
(p/'report.typ').write_text(text)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
doc=fitz.open(p/'report.pdf');print('pages',len(doc))
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save('/home/ec2-user/n83-candidate-recovery/proof-bridge-report-page-'+str(i+1)+'.png')
shutil.copy2(__file__,p/'render-report.py')
