from pathlib import Path
import json,typst,fitz
D=Path(__file__).resolve().parent;E=D.parent.parent
summary=json.loads((E/'runs/summary.json').read_text())
assert summary==json.loads((E/'expected-summary.json').read_text())
assert (E/'exit-code.txt').read_text().strip()=='0'
chosen=[]
for line in (E/'runs/cases.jsonl').open():
 z=json.loads(line);p=z['payload']['plan']
 if z['arm']=='candidate' and p['kind']=='source' and p['source_instance'] in ['18','27','36']:
  n=p['r']//2;test=next(t for t in z['payload']['tests'] if len(t['input'])==n and not t['transpose'])
  chosen.append(dict(instance_id=z['instance_id'],r=p['r'],n=n,query_counts=test['counts'],acquisition=z['payload']['acquisition']))
assert len(chosen)==3
(D/'chart-data.json').write_text(json.dumps([{k:v for k,v in z.items() if k!='acquisition'}|{'acquisition_counts':z['acquisition']['counts'],'cache_profile':z['acquisition']['profile']} for z in chosen],indent=2)+'\n')
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="230"><rect width="760" height="230" fill="white"/><g font-family="sans-serif" font-size="16"><text x="10" y="24">Half-size principal query: scalar field products</text>']
mx=max(z['query_counts']['field_products'] for z in chosen)
for i,z in enumerate(chosen):
 y=45+50*i;v=z['query_counts']['field_products'];w=430*v/mx
 svg += [f'<text x="10" y="{y+21}">r={z["r"]}, n={z["n"]}</text>',f'<rect x="160" y="{y}" width="{w}" height="30" fill="#286e89"/>',f'<text x="{170+w}" y="{y+21}">{v:,}</text>']
svg.append('<text x="10" y="217">Candidate source instances18,27,36; new wraps2,1; exact counts.</text></g></svg>');(D/'products.svg').write_text(''.join(svg))
(D/'diagram.json').write_text(json.dumps(dict(stages=['full public companion operator','counted boundary cache','principal generator: width s+4','general dimension/two-wrap products'],next_derived_stage='Schur generator: width s+2; not executed'),indent=2)+'\n')
(D/'diagram.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="760" height="160"><rect width="760" height="160" fill="white"/><g font-family="sans-serif" font-size="15" text-anchor="middle"><rect x="5" y="20" width="220" height="70" fill="#deedf0"/><text x="115" y="46">Full public operator</text><text x="115" y="72">Boundary queries charged</text><path d="M225 55H270" stroke="#333"/><rect x="270" y="20" width="220" height="70" fill="#deedf0"/><text x="380" y="46">Principal generators</text><text x="380" y="72">Four correction columns</text><path d="M490 55H535" stroke="#333"/><rect x="535" y="20" width="220" height="70" fill="#deedf0"/><text x="645" y="46">Dimension n, wraps c,d</text><text x="645" y="72">Primal and transpose</text><text x="380" y="130">Dense matrices and explicit inverse oracles occur only in the checker.</text></g></svg>''')
lines='\n'.join(f'- r={z["r"]}, n={z["n"]}: {z["query_counts"]["field_products"]:,} field products, {z["query_counts"]["field_additions"]:,} additions, one inversion, {z["query_counts"]["integer_multiplications"]} native products, {z["query_counts"]["packing_bytes"]:,} packing bytes.' for z in chosen)
acq='\n'.join(f'- r={z["r"]}: {z["acquisition"]["counts"]["field_products"]:,} field products, {z["acquisition"]["counts"]["integer_multiplications"]:,} native products, {z["acquisition"]["counts"]["packing_bytes"]:,} packing bytes, {z["acquisition"]["profile"]["cached_field_slots"]:,} retained field slots.' for z in chosen)
t=f'''#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= General companion block interfaces
Exact component panel · 2026-11-01 · companion-blocks-validation

*Implemented and checked:* arbitrary matrix dimensions over the fixed prime
coefficient field, distinct companion wraps including zero wraps, primal and
transpose application, four-column principal-block corrections, and conditional
inverse-generator transport. All 738 context rows and 32,328 product checks
passed. The independent dense references checked 3,072 principal identities
and 192 inverse-generator identities. All 384 singular-oracle failures,
2,376 equal-wrap inputs and 67,920 deliberately corrupted results/counts or
generators were rejected. Candidate and tuple-input A/A matched exactly.

#image("diagram.svg",width:100%)
Editable diagram data and renderer are retained here. Production API:
companion-blocks/operators.py. Derivation: companion-blocks/DERIVATION.md.

*General operator.* Let L=Z_(n,a), R=Z_(n,b), with a different from b in F_r.
L^n=aI and R^n=bI imply (a-b)A=sum_j L^(n-1-j)GKᵗR^j for j=0 through n-1.
No restriction n=r or n less than r is needed. Two ordinary packed polynomial
products per generator column implement either direction. The synthetic
panel includes n=r+1 and n=1 as well as dimensions below r.

*Principal generators.* The leading block uses the restricted old generators
and four explicit boundary outer products. A public operator cache acquires
the last full row, crossing column, last block row and first full column.
Each miss executes and charges a full source application. The checker compares
every cached vector to its independently constructed dense row or column.
For four cuts and four new wrap choices, all 64 requests, 10 misses and
54 hits per source context are checked; 10r field slots are retained.

*Inverse transport scope.* For an invertible A,
R A⁻¹ - A⁻¹ L = (-A⁻¹G)(A⁻ᵗK)ᵗ. The wrapper transforms generators through
supplied forward and transpose inverse oracles. This experiment supplies
explicit unit-upper-triangular reference inverses and verifies both inverse
identities before use. It establishes the swapped-wrap transport, not efficient
inverse synthesis. Singular reference-oracle exceptions must propagate.

*Frozen cohort.* Each arm retains all 81 earlier generator contexts, their
four cuts and four new wrap pairs. The 288 synthetic contexts cross primes
7,13,19,43; dimensions 1,2,3,r-1,r,r+1; wraps (2,1),(1,2),(0,1),(1,0);
and upper-triangular, rank-deficient diagonal and zero matrices. Synthetic
proper half-size blocks also test the correction formula. Full operators
use zero, ones, seeded and all small basis vectors (endpoints for larger
sizes). Restrictions use every basis vector at sizes at most19, otherwise
one seeded vector, in addition to entrywise displacement identities.

#pagebreak()
= Exact work and execution custody
#image("products.svg",width:100%)
Three recorded source instances illustrated from the full retained panel;
all raw subchecks remain in runs/cases.jsonl. Exact counts have no uncertainty
bars. Query, acquisition, native and storage units remain separate.

{lines}

*Boundary acquisition totals, once per source context.* These include all
10 full operator queries shared across the 16 principal restrictions:

{acq}

*Formulas checked.* At dimension n and width s, each direction uses
2s(n-1)+n field products, s(3n-2)+1 additions and one inversion;
2s native products and 4s input encodings. With byte digit width
d=ceil(bitlength(n(r-1)²)/8), packing is 4snd bytes, product serialization
and decoding are each 2s(2n-1)d bytes, and reductions number2s(2n-1).
The principal correction alone costs n products and n+1 additions; boundary
acquisition is separately charged. Inverse transport records s calls in each
oracle direction and ns negations. No oracle is assigned zero total cost;
its supplied dense reference implementation is outside a fast-solve claim.

*Predata custody.* Commit91aaadd; nine source locks and one public input
archive. Before any numerical trial, seed-preflight.py checked every one of
the738 protocol execution cells against the expanded plan. The earlier
companion-matvec export failure remains unchanged. This panel uses a frozen
row index binding offsets and row hashes; the analyzer verifies that index
before exporting, and the full raw archive is hashed in the export file list.

*Execution.* One timeout900 invocation of ../.audit-venv/bin/python -u
companion-blocks-validation/check.py, no restart, producer exit0. All738
planned contexts completed, none excluded or dropped. Exact timestamps
and shared-host metadata are in runs/. No valid wall-time, CPU or RSS
comparison was collected. Export validation status, lock verification and
PDF inspection are recorded in provenance.json and export-check.log.

*Evidence.* runs/summary.json; runs/cases.jsonl; runs/row-index.json;
runs/export.json; expected-summary.json; protocol.json; source-lock.json;
input-lock.json; seed-preflight.json; launch.json. Report source, chart data,
SVGs and renderer are retained beside this PDF.

#pagebreak()
= Derived next step: Schur generators of width s+2
This page records a derivation and unexecuted prototype. The prototype in
companion-schur/schur.py passed syntax checking only; no Schur numerical
experiment ran. Full proof and call schedule: companion-blocks/SCHUR_NEXT.md.

Partition A=[[E,B],[C,D]], with E invertible, leading dimension n and trailing
dimension m. Set S=D-CE⁻¹B, U=[-CE⁻¹,I], V=[-E⁻¹B;I]. Then UA=[0,S]
and AV=[0;S]. Multiplying the full displacement identity by U and V gives
(L22-CE⁻¹L12)S-S(R22-R21E⁻¹B)=(UG)(VᵗK)ᵗ.

The companion crossing terms are L12=a u_n v_mᵗ and R21=u_m v_nᵗ,
where u is the first and v the last basis vector. For distinct new wraps c,d,
grouping the wrap and crossing corrections yields generators

GS = [ UG | aCE⁻¹u_n+c u_m | -S u_m ],

KS = [ VᵗK | Sᵗv_m | BᵗE⁻ᵗv_n+d v_m ].

Thus Z_(m,c)S-SZ_(m,d)=GS KSᵗ with constructed width s+2. This does
not assert minimal displacement rank or require S itself to be invertible.

*Counted implementation path.* A Schur application uses two full A
applications and one E solve: apply A to [0;x], solve Ez=Bx, apply A to
[z;0], then return Dx-Cz. The transpose path uses two Aᵗ applications and
one E⁻ᵗ solve. Columnwise generator construction, including both Schur
endpoint vectors, has an explicit uncached schedule of 2s+6 source
applications and s+2 leading-block solves in each direction, plus scalar
updates and generator storage. These are nested calls, not unit-cost solves.
Batching and the recursive pivot policy are still required.

*Required obstruction control.* A singular E does not establish that A is
singular. The invertible matrix [[0,1],[1,0]] has zero leading scalar block.
The next implementation must preserve that distinction and record a block
pivot obstruction or execute a separately charged pivot policy.

*Next frozen gate.* Validate the Schur prototype using the checked block
API with explicit reference E oracles. Check dense Schur
identities, both directions, complete nested call accounting, corrupted
corrections, zero/singular Schur cases and the invertible-A/singular-E
counterexample. Then replace reference oracles with a recursive backend.

*Literature context.* Bostan et al. (2017), Section6.2, also derives
low-width Schur generators under its displacement conventions and generic
rank-profile assumptions. The displayed formula specializes the project's
conventions. No base-field randomized success guarantee is imported.
#link("https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf")[Primary source, SIAM J. Matrix Anal. Appl. 38(3),733–775.]
'''
for a,b in [('at most19','at most 19'),('number2s','number 2s'),('Commit91aaadd','Commit 91aaadd'),('the738','the 738'),('timeout900','timeout 900'),('exit0','exit 0'),('All738','All 738'),('Section6.2','Section 6.2')]:t=t.replace(a,b)
t=t.replace('_',chr(92)+'_');(D/'report.typ').write_text(t)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc))
