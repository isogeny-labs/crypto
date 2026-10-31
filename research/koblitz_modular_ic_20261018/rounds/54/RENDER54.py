"""Figures/PDF from terminal native receipts; no mathematical producer replay."""
from pathlib import Path
import json,re,html
import matplotlib
matplotlib.use('Agg')
matplotlib.rcParams['svg.hashsalt']='koblitz-r54-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
accepted={x['attempt']:x for x in r['runs'] if x['status']=='accepted'}
def fields(attempt):return {k:int(v) for k,v in re.findall(r'(\w+)=(-?\d+)',accepted[attempt]['native_results'][0])}
f=fields('fiber54-01');p=fields('frobfiber54-02');flat=fields('flatpoints54-01')
assert [f[x] for x in ['rank_E','rank_K','image','relations','inverse']]==[16,16,1,1,1]
assert p['RR_products']==2112 and p['anti_trace_zero']==1 and flat['binary_value_conversions']==128
rho=[int(re.search(r'rho_degree=(\d+)',s).group(1)) for s in accepted['flatpoints54-01']['point_results']]
norm=[int(re.search(r'norm_degree=(\d+)',s).group(1)) for s in accepted['frobfiber54-02']['point_results']]
data={'fiber_source':accepted['fiber54-01']['native_results'],'prime_source':accepted['frobfiber54-02']['native_results'],'flat_source':accepted['flatpoints54-01']['native_results'],'projector_rank':f['rank_E'],'basis_rank':f['rank_K'],'trace_rank':p['trace_rank'],'anti_rank':p['anti_rank'],'anti_trace_zero':True,'norm_degrees':norm,'base_recovery_degrees':rho}
(h/'plot-data54.json').write_text(json.dumps(data,indent=2)+'\n')
fig,axes=plt.subplots(1,3,figsize=(9,3),layout='constrained')
axes[0].bar(['E','K'],[f['rank_E'],f['rank_K']],color=['#28664d','#609b80']);axes[0].set(title='Actual RR component',ylabel='Rank over F32',ylim=(0,18))
axes[1].bar(['orbit trace','1 - sign'],[p['trace_rank'],p['anti_rank']],color=['#28664d','#b46b27']);axes[1].set(title='Restricted fiber operators',ylabel='Rank over F32',ylim=(0,10))
axes[2].plot([1,2,3,4],norm,'o-',label='binary modulus');axes[2].plot([1,2,3,4],rho,'s-',label='base recovery');axes[2].set(title='Four closed source points',xlabel='Point index',ylabel='Polynomial degree over F2',xticks=[1,2,3,4],ylim=(0,22));axes[2].legend(fontsize=8)
for ax in axes[:2]:
 for bar in ax.patches:ax.annotate(str(int(bar.get_height())),(bar.get_x()+bar.get_width()/2,bar.get_height()),xytext=(0,5),textcoords='offset points',ha='center')
for ax in axes:ax.grid(axis='y',alpha=.15)
fig.savefig(h/'fiber54.svg',metadata={'Date':None});plt.close(fig)
p_svg=h/'fiber54.svg';p_svg.write_text('\n'.join(s.rstrip() for s in p_svg.read_text().splitlines())+'\n')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="220" viewBox="0 0 1120 220"><rect width="1120" height="220" fill="white"/><g font-family="DejaVu Sans,sans-serif" fill="#15364a">'
boxes=[('Actual normal RR algebra',['32 source generators','528 multiplication columns','Original nu/source bindings']),('Split fiber component',['rank E = rank K = 16','K F = E; F K = I','Full multiplication products']),('Primitive coordinate',['w = nu / b','Degree 16 over F32','Four quartic prime factors']),('Binary point evaluators',['Four degree-20 moduli','a = rho_i(w), all 32 values','128 full conversions'])]
for x,(title,lines) in zip([10,290,570,850],boxes):
 svg+=f'<rect x="{x}" y="15" width="260" height="185" rx="8" fill="#d7eee3"/><text x="{x+10}" y="49" font-size="20">{html.escape(title)}</text>'
 for j,line in enumerate(lines):svg+=f'<text x="{x+10}" y="{91+36*j}" font-size="17">{html.escape(line)}</text>'
svg+='</g></svg>'; (h/'construction54.svg').write_text(svg+'\n')
def cell(s):return '[#text('+json.dumps(s)+')]'
table=[]
for row in r['runs']:
 detail=('exit '+str(row['native_exit'])+'; '+str(row['error_count'])+' error lines') if row['kind']=='Singular' else 'compile 0; audit 0; 6 declarations'
 table += [row['attempt'],row['status'],detail]
source='''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R54: actual RR fiber and four binary degree-20 evaluators
The complete normal-RR fiber is now identified directly by source matrices.
Four closed source points have binary residue fields and complete function
evaluators. Their orbit traces give a source-bound sign-minus null result.

== Full source and inverse products
All 528 retained multiplication columns bind the unit,lambda,V and actual
descended nu actions. At a^5+a^2+1=0 the idempotent selects the actual
supersingular base point. The 16 monomials lambda^i*nu^j, i,j=0,1,2,3,
form a complete basis of that component. Both ranks are16 and
#raw("K F = E; F K = I16; Mlambda K = K L; Mnu K = K N",block:true)
Both descended equations vanish as full matrices;all border products pass.
A changed basis entry is detected. Source: fiber54-01.
#image("construction54.svg",width:100%)

== Explicit equations and larger residue fields
The actual border equations are
#raw("lambda^4+b*lambda+b^2=0; nu^4+b^3*nu+b^4*(lambda+1)=0",block:true)
With w=nu/b,full ideal products give
#raw("lambda=w^4+w+1; P=w^16+(1+b)*w^4+b*w+1+b+b^2",block:true)
P has four irreducible quartic factors over F32. Their five coefficient
conjugates give four distinct irreducible degree-20 norms over F2. Every
factor is checked independently by Frobenius/gcd tests. Each coupled point
has all 32 RR basis values and all 528 multiplication products with full
quotient witnesses:2,112 products total.
#image("fiber54.svg",width:100%)
All plotted values are exact native data. Sources:fiber54-01,frobfiber54-02,
flatpoints54-01. Operator ranks are on this F32 fiber algebra,not global
collector or Jacobian ranks. The complete anti-times-trace product is zero.
#pagebreak()
= Complete binary evaluators and restricted obstruction
For each point the coupled binary ideal (a^5+a^2+1,g_i(a,w)) is identified
by both full ideal products with (N_i(w),a+rho_i(w)). All four fields have
degree20 over F2. The rho degrees are16,16,18,18. All128 RR values become
binary polynomials of degree below20,with full source conversion products.
The fresh checker independently replays all2,112 RR products and checks
unit and actual source-coordinate labels. Altered values are rejected.

== Frobenius and the failed anti-projected direction
On the complete16-dimensional F32 algebra the native matrices give
#raw("T=Frob32; T^2=S; T^4=I; S(w)=w+1",block:true)
The operator U=I+T+T^2+T^3 has rank4,and I-S has rank8. Their full product
is zero. Thus these closed-point orbit traces are sign-invariant;they add
no directions through the sign-minus projection. A changed trace is detected.
No selected order-11 factor sign or global Jacobian action is inferred.

== Lean and complete workflow ledger
Five authored implications cover basis injectivity,component recovery,exact
range identification,order-four trace invariance and conditional anti-trace
annihilation. Compilation and separate audit exit0. Lean also generated
orbitTrace.eq_1:all six declarations audit with zero failing and zero trusted.
Actual arrays,field arithmetic and normal-RR geometry are external premises.
'''
source+='#table(columns:(1.15fr,.7fr,1.8fr),inset:5pt,'+','.join(cell(s) for s in ['Workflow','Status','Receipt']+table)+')\n'
source+='''
Four workflows accepted,one rejected. The first primitive run used the
reserved identifier degree;point checks were skipped and its late marker is
rejected. The corrected attempt changes only local names. Immutable copied
drivers and all29 historical source/protocol byte pins are verified.

Native limits:4GiB virtual,1200s,one Singular/FLINT thread,seed54. Lean4.34.1,
mathlibd13f23b,-j1/-M4096,1200s separately for compile/audit,no virtual cap.
Only native wall times are recorded;complete CPU times were not recorded.
No accepted producer is repeated for publication. No comparative speed claim.

== Next construction
Finish compressed global target checking through Frobenius powers and the
QR/G,N syzygies,then the actual level-11/degree-128 source map,F2 descent
and intended11 action. Test the larger A=0 quotient of dimension80 and its
possible CRT coordinate model before claiming that global quotient. Determine
actual cusp/infinity poles before declaring a complete principal divisor.
Keep proper polarization,full saturation,larger112/991 and reverse-class lanes.

Sources:R50/R51 complete RR algebra,R52 actual nu action,R53 descended fiber;
native linalg.lib;Stacks0BJF;standard finite-field Frobenius/gcd criterion.
See CM.md,PROOF_MAP.md,protocol.json and results.json for premises and receipts.
'''
source='\n'.join(line if line.startswith('#') else line.replace(',',', ').replace(';','; ').replace(':',': ').replace(',  ',', ').replace(';  ','; ').replace(':  ',': ') for line in source.splitlines())+'\n'
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'pdf_bytes':(h/'report.pdf').stat().st_size,'workflows':len(r['runs'])}))
