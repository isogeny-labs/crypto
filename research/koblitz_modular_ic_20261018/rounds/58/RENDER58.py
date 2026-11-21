"""Render recorded exact counts and conditional geometric implications only."""
from pathlib import Path
import json,re
import matplotlib
matplotlib.use('Agg');matplotlib.rcParams['svg.hashsalt']='koblitz-r58-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());accepted={x['attempt']:x for x in r['runs']if x['status']=='accepted'}
def line(attempt,prefix):return next(x for x in accepted[attempt]['native_results'] if x.startswith(prefix))
def count(text,key):return int(re.search(r'\b'+key+r'=(\d+)',text).group(1))
s=line('space58-01','SPACE58_RESULT ');inc=line('include58-01','INCLUDE58_RESULT ');fiber=line('fiber58-01','FIBER58_RESULT ')
old=json.loads((h.parent/'57/results.json').read_text());oldline=next(t for row in old['runs']if row['attempt']=='trace57-02' for t in row['native_results']if t.startswith('TRACE57_RESULT '))
n=count(s,'complete_dimension');rank=count(s,'sigma_nilpotent_rank')
data={'R57':{'complete_space':count(oldline,'space_dimension'),'fixed':count(oldline,'fixed_dimension'),'trace':count(oldline,'trace_dimension'),'fixed_mod_trace':count(oldline,'fixed_mod_trace_dimension')},'R58':{'complete_space':n,'fixed':n-rank,'trace':rank,'fixed_mod_trace':n-2*rank},'ambient':count(s,'ambient'),'conditions':count(s,'local_conditions'),'source_results':[s,inc,fiber],'derived':'R58 fixed=dimension-rank; fixed/trace=dimension-2*rank, using N²=0 and independently checked contraction','uncertainty':'fixed exact algebra, no statistical intervals'}
(h/'plot-data58.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(9.4,3),layout='constrained');labels=['Complete space','Fixed','Trace image','Fixed / trace'];keys=['complete_space','fixed','trace','fixed_mod_trace']
for off,label,color in [(-.18,'R57','#7197b8'),(.18,'R58','#2c7756')]:
 vals=[data[label][k]for k in keys];bars=ax.bar([i+off for i in range(4)],vals,width=.35,label=label,color=color)
 for b in bars:ax.annotate(str(int(b.get_height())),(b.get_x()+b.get_width()/2,b.get_height()),xytext=(0,4),textcoords='offset points',ha='center')
ax.set(xticks=range(4),xticklabels=labels,ylabel='Binary dimension',title='L(P) and L(2P) on the same genus-61 source',ylim=(0,116));ax.grid(axis='y',alpha=.15);ax.legend();fig.savefig(h/'spaces58.svg',metadata={'Date':None});plt.close(fig)
p=h/'spaces58.svg';p.write_text('\n'.join(s.rstrip()for s in p.read_text().splitlines())+'\n')
rows='\n'.join('['+x['attempt']+'],['+x['status']+'],['+(str(int(x['native_wall_seconds']))+' s'if x['kind']=='Singular'else'not recorded')+'],'for x in r['runs'])
assert len(accepted['pole-lean58-03']['audited_declarations'])==7
source='''#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R58: complete 100-section doubled-pole space
The actual degree-160 prescribed-pole system now has all 100 source-bound
sections. The 196 quadratic-product candidates pass complete ambient bounds
and independence. All 96 local conditions have rank 96. The actual trace
complex admits an explicit contraction. All four old fixed classes modulo
traces acquire actual preimages when the pole bound is doubled.

#image("spaces58.svg",width:100%)
Recorded counts: R57 trace57-02 and R58 space58-01/include58-01. R58 fixed
and quotient dimensions are derived by rank-nullity and checked contraction.
No sampling, timing comparison or change of source genus is represented.

== Complete ambient and local restriction
R56 gives div(A)=P-D, with four reduced degree-20 points in P. The ambient
D0 has degree 128, and D has degree 80. On the proper genus-61 source,
2D0 has degree 256 and RR dimension 196; 2D has degree 160 and RR
 dimension 100. No normal-generation assertion is assumed.

The native calculation evaluates ACTUAL products of the descended sections
in F4 charts; linear descent of an original product is a different expression.
All 3,920 product pole bounds and 1,360 coefficient-conjugation checks pass.
257 F4 jets give 514 binary rows and rank 196. The 42 ramified and 6
unramified removed coefficients, each over F4, give the full 96 rows.

C K=0, R K=I100 and K R+H C=I196 certify the entire kernel. Every
exported matrix entry is binary. Controls 1,A squared belong; actual b
squared is excluded. The actual involution and its restriction square to I.

== Whole-source functions
All 196 products, all 100 kernel functions, all 100 divisions by A squared
and all 100 source involution substitutions are checked in the full degree-32
field over F2(u). Division identifies complete L(2D) with complete L(2P).
An independent checker replays every source reconstruction and division.

#pagebreak()
= Trace contraction, finite support and map constraints

== Explicit recovery for every fixed function
N=sigma+I has rank 50 on the 100-space and N squared is zero. A complete
Jordan basis with two-sided inverse produces Q with
#raw("NQ+QN=I100, Q^2=0",block:true)
Thus ker(N)=image(N), and Q gives a trace preimage of every fixed vector.
The retained generic R57 complete-kernel Lean implication applies directly.

The old 24-space inclusion multiplies numerator f by A. It has rank 24,
commutes with the actual involution, and satisfies all 24 normalized source
identities. Applying Q to the four old fixed-quotient representatives gives
four actual functions whose traces equal the embedded old functions.
Every identity is checked in the whole source field. A changed preimage
is rejected. The four becoming traces is a function-space result.

== All twenty finite support points over F32
The source-bound norm of degree 80 factors into four degree-20 F2 fields
and twenty simple degree-4 points over F32. All five coefficient conjugates
per component, every product, and actual coordinate Frobenius actions pass.
At every point Frob32 squared equals source sigma: w goes to w+1,
lambda and base coordinates are fixed, and nu goes to nu+b.

== Conditional degree and closed-point trace tests
These are constraints on a future map, not maps constructed. For a
nonconstant F32 map to the ordinary Koblitz model, with both x and y/x
in L(2P), double-pole bounds force ramification one over O and T=(0,1).
Two disjoint fibers supported in twenty degree-4 points give degree 4k,
1≤k≤10. The allowed degrees 4,8,12,16,20,24,28,32,36,40 all have
explicit integer inverses modulo 11. Intended-subgroup action remains a
separate candidate check.

For a sigma-anti-equivariant F32 map, Frob32 squared sends every image
to its inverse, so each four-cycle closed-point trace cancels. Degree-zero
divisors supported entirely on these twenty points have zero pushforward.
Use this set as pole geometry and a reverse-evaluator negative control;
collectors need other ordinary supports. This reuses and checks the R54
finite-fiber identity on the complete labelled support.

== Next controlled-pole map construction
First test all 18 F2-stable O/T fiber patterns: 12 ordered degree-20 pairs
and 6 complementary degree-40 partitions. Normal B modulo A to the fourth
power, dimension 320, supplies the complete vanishing jets. A principal
x-divisor gives a unique x up to scalar; test all 31 nonzero F32 scalars
and both signs. The equation for t is then F2-linear, with Nt=1. Keep the
broader F32-support search below as a separate lane.

Use the actual 50-dimensional fixed space and a trace-one affine 50-space
over F32. With t=y/x, enforce both ordinary signs in the whole source:
#raw("x^2*(t^2+t+x+a)=1",block:true)
Five binary coordinates per parameter give 500 bits. Coefficient squaring
is binary-linear, so the resulting Boolean coefficient equations are
quadratic. Compile all five conjugates and retain complete divisor and
intended order-11 action checks. This compiler is the next experiment.

#pagebreak()
= Verification and retained execution record

== Every attempt
#table(columns:3,inset:4pt,[Attempt],[Terminal status],[Native UTC diagnostic],
RUNROWS
)
UTC differences have one-second resolution; zero means same-second
endpoints. They are custody diagnostics. CPU and Lean elapsed times were
not recorded as benchmark measurements. Every failed attempt is retained.

== Seven new Lean implications
Seven authored statements compile and pass a separate axiom audit, zero
failing and zero trusted: local-unit double-zero order, simple-pole exclusion,
imaginary and real coefficient extraction, target four-cycle trace
cancellation, unramified double-bound implication and the degree-unit bound.
No sorry, native_decide or new axiom is used. Native arrays and all actual
field/source identities remain separate evidence. Properness, genus,
complete RR interpretation, local parameter/unit facts, morphism existence,
F32 definition and target equivariance are explicit premises as applicable.
Read PROOF_MAP.md.

The first Lean attempt lacked a CharP import; the second left ten closed
Coprime goals unsolved. Both failed sources/logs are retained. The repaired
finite cases use kernel-checked decide. All native attempts passed. Historical
executed-source byte pins are checked against their frozen Git revisions.

Native: seed 58, one Singular/FLINT thread, 4 GiB virtual and 1,200-second
cap per invocation. Lean: 4.34.1, mathlib d13f23b, one worker, 4 GiB managed
memory, separate 1,200-second compile/audit caps and no virtual limit.
Accepted producers are never repeated for publication or report generation.

== Sources and preserved construction lanes
Reviewed R27 complete RR/cusps, R43/R50 product packet, R54 finite action,
R55/R56 normal quotient and complete poles, R57 full restriction and trace
functions, and Stacks RR tag 0B5B. No novelty claim for these standard tools.

NEXT.md retains the actual F32 relation compiler, possible 180-section
third-pole bound, integral level-77 map, F2 modular-factor descent, intended
order-11 action, proper polarization, separate incidence saturation,
node187 coupled divisors, arbitrary reverse classes and larger 112/991
sources. All original authorization, ownership and resource limits persist.
'''.replace('RUNROWS',rows)
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'));print(json.dumps({'pdf_bytes':(h/'report.pdf').stat().st_size,'workflows':len(r['runs'])}))
