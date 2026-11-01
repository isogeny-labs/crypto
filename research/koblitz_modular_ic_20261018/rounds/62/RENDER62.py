"""Render recorded rows and their counts only; no field algebra or new trials."""
from pathlib import Path
import json,csv,collections
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent
result=json.loads((h/'results.json').read_text())
rows=list(csv.DictReader((h/'runs/split62-02/split62.tsv').open(),delimiter='|'))
families=['basis','control','cubic','mixed']
totals=collections.Counter(r['family'] for r in rows)
zeros=collections.Counter(r['family'] for r in rows if r['zero_odd']=='1')
data={'source':'runs/split62-02/split62.tsv','rows':len(rows),'families':[{'family':f,'functions':totals[f],'zero_odd':zeros[f]} for f in families]}
(h/'plot-data62.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(8.3,3.1));xs=list(range(4))
ax.bar([x-.18 for x in xs],[totals[f] for f in families],width=.36,label='Complete cohort',color='#205984')
ax.bar([x+.18 for x in xs],[zeros[f] for f in families],width=.36,label='Zero odd output',color='#d38127')
for x,f in zip(xs,families):
 ax.text(x-.18,totals[f]+4,str(totals[f]),ha='center',fontsize=9)
 ax.text(x+.18,zeros[f]+4,str(zeros[f]),ha='center',fontsize=9)
ax.set_xticks(xs,families);ax.set_ylabel('Number of source functions');ax.set_ylim(0,365)
ax.legend(frameon=False);ax.set_title('All 435 source functions retained (split62-02)')
fig.tight_layout();fig.savefig(h/'cohort62.svg');plt.close(fig)
status=collections.Counter(r['status'] for r in result['runs'])
table=[]
for r in result['runs']:
 wall=str(r.get('native_wall_seconds','not recorded'))
 table.append('['+r['attempt'].replace('-','‑')+'], ['+r['status']+'], ['+wall+'],')
text='''#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= R62: complete square decomposition and generator route
Exact Koblitz modular-carrier field construction. Source field: the retained
R61 degree32 extension of F2(u), with generators lambda and v. These are
whole-source identities, with no scalar-fiber or series approximation.

== Recorded computation
The matrix construction binds all32 standard monomials and squared columns
and checks both inverse products. The repaired complete cohort checks435
whole-source decompositions and13920 rational coefficient parity identities.
The fresh source replay reloads every input and root, binds all435 functions
to R61 and rechecks the complete operator and negative controls.

#image("cohort62.svg", width: 100%)
Recorded counts from split62-02; all435 rows, seed62, one exact source instance.
Zero odd components are field-operator outputs. Their complete supported
divisors and global differential regularity are separate obligations.

== Terminal receipts
#table(columns: (2fr,1fr,1.4fr), inset: 4pt,
[Attempt], [Status], [Native wall seconds],
'''+ '\n'.join(table)+'''
)
Wall times are native receipt intervals, not controlled performance comparisons.
Lean compile and audit intervals were not recorded by the proof driver.

#pagebreak()
= Direct generator route and proof scope
For a grouped characteristic-two source polynomial,
$ P(t)=a^2+u b^2+t c^2+u t d^2 $, put $q=c^2+u d^2$.
With $P(t)=0$ and a checked nonzero derivative/inverse,
$ t=((a c+u b d)/q)^2+u((a d+b c)/q)^2. $
The direct route uses pair arithmetic rather than reading the squared-basis
matrix. It checks the actual source grouping, derivative inverse, both
source generators, all32 generated basis pairs and all435 functions, and
compares870 roots with the matrix output. The rational scalar parity helper
is shared; this is not a separately implemented scalar parity algorithm.

== Lean statements
The pinned Lean4.34.1/mathlib d13f23b toolchain checks twelve authored
claims: rational parity, uniqueness under nonsquare-u and characteristic-two
premises, the zero-odd square criterion, five cyclic5 norm/projector/control
claims, three pair arithmetic identities and the generator formula. Native
field/model and nonsquare-parameter inputs, normality, local valuations,
complete holomorphic regularity, actual deck/Cartier action and target-factor
action are not inferred from these abstract implications.

== Failures retained
The first complete435 attempt used unsupported ideal equality and is rejected
in full despite a late marker. The first Lean compile failed on a lemma name
and numeral-cast normalization; its artifact is rejected. All later failures,
if present, are listed in results.json with their actual errors. An early
raw-index mistake was corrected without deleting local bytes or rewriting
Git history. Accepted producers were not repeated for publication.

== Scaling and next construction
GENERATOR-LEAD62.md and SCALE112-DESIGN62.md bind the proposed larger rung
to the marked23 degree7 base followed by the degree16 lambda extension.
The larger field has degree112;991 denotes the selected factor action.
Next: apply the generator operator to this exact larger source and complete
the normal-order/infinity bridge needed for the full61-dimensional
holomorphic space. Unit divisors/saturation, actual primitive cyclic block,
Hecke/order11 action, proper polarization, quotient saturation and arbitrary
reverse divisors retain their separate obligations.

== Reproduction and provenance
PROTOCOL.md, protocol.json, donors.json and per-attempt source revisions
freeze the affected inputs before data. results.json verifies historical
source bytes. Native limits:1200seconds,4GiB virtual, one Singular/FLINT
thread, seed62, one math job. Lean:1200seconds separately per compile/audit,
CPU0, -j1 -M4096, unlimited virtual address space. No timing claim.

Sources: R61/SQUARE-DECOMPOSITION61.md; R52/CM.md;
#link("https://www.singular.uni-kl.de/index.php/singular.pdf")[Singular manual].
'''
(h/'report.typ').write_text(text.rstrip()+'\n')
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'attempts':len(result['runs']),'status':dict(status),'cohort':data}))
