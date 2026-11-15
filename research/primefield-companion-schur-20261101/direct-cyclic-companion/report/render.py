from pathlib import Path
import json,typst,fitz
D=Path(__file__).resolve().parent
x=dict(status='derived reduction; constructor syntax checked only',identities=['B - gamma T B T^t = V diag(c) V^t','gamma T B - B T = -H T','A = D^-1 J B J','Z_(r,gamma) A - A Z_(r,1) = G K^t'],generator_width='3+q(q-1)',source='https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf',experiment_run=False)
(D/'diagram.json').write_text(json.dumps(x,indent=2)+'\n')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="720" height="170"><rect width="720" height="170" fill="white"/><g font-family="sans-serif" font-size="15" text-anchor="middle"><rect x="5" y="20" width="210" height="75" fill="#dcecef"/><text x="110" y="48">Public character boundaries</text><text x="110" y="75">V, c: s = 3+q(q-1)</text><path d="M215 60H250" stroke="#333"/><rect x="250" y="20" width="220" height="75" fill="#dcecef"/><text x="360" y="48">Reverse + diagonal scaling</text><text x="360" y="75">Explicit G, K, RHS</text><path d="M470 60H505" stroke="#333"/><rect x="505" y="20" width="210" height="75" fill="#eee7d7"/><text x="610" y="48">Companion structured solve</text><text x="610" y="75">Backend not implemented</text><text x="360" y="135">No Pascal transform in the proposed generator constructor</text></g></svg>'
(D/'diagram.svg').write_text(svg)
t="""#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Direct cyclic companion reduction
Derived identities and unexecuted constructor · 2026-11-01

The existing weighted character Gram matrix has a cyclic displacement before
its inverse-Pascal transform. An explicit reversal and diagonal scaling convert
it to a standard companion displacement. The new constructor produces only
public generators and RHS; no fast solver or finite validation has run.

#image("diagram.svg",width:100%)
Diagram of derived/proposed stages. Source equations: ../DERIVATION.md;
prototype: ../generators.py. No measured-cost graph is applicable.

*Boundary identity.* Let (Tv)_k=v_(k+1 mod r), and let V collect the constant
vector and both endpoint character vectors of every translated pattern. The
same geometric weights as the current compiler telescope to
B - gamma T B Tᵗ = H = V diag(c) Vᵗ, with s=3+q(q-1).
Multiplying on the right by T gives gamma T B - B T = -H T.

*Companion form.* Let J reverse coordinates and D=diag(1,gamma,...,gamma^(r-1)).
Put A=D⁻¹ J B J, G=-D⁻¹ J V diag(c), K=J Tᵗ V. Then
Z_(r,gamma) A - A Z_(r,1) = G Kᵗ, where Z_(r,a) has subdiagonal ones and
top-right a. The key prime-field identity is gamma^r=gamma, so
D⁻¹(gamma Tᵗ)D has exactly that wrap entry. The characteristic polynomials
X^r-gamma and X^r-1 are coprime because gamma differs from1.

*No lost boundary data.* If gamma T B=B T, iterate r times using T^r=I:
(gamma-1)B=0, hence B=0. Thus the displacement is injective even though its
characteristic polynomials have repeated roots in characteristic r.

*Primal and dual transport.* To solve Bx=b, solve Ay=D⁻¹ Jb and set x=Jy.
If vA=0 and vD⁻¹ Jb=1, set w=vD⁻¹ J; then wB=0 and wb=1. The separate
public feature-exclusion test remains required. Here b_k=k is public.

*Implementation scope.* Constructor inputs are r,q,gamma,weights. It meters
character powers, boundary vectors, diagonal powers, row scalings and RHS.
It builds neither B nor a Pascal matrix, inverse or solution. O(rs) boundary
data remain. Only syntax has been checked; coefficient identities and counter
formulas still require the next frozen finite panel.

*Primary-source connection.* Bostan, Jeannerod, Mouilleron and Schost (2017)
study companion displacement and fast structured systems. Their Theorem6.7
has sufficient sampling condition |S| greater than2n(n+1). For n=r, no
S contained in F_r meets it. This limits use of that stated success guarantee;
it does not prove the sampler fails. No extension-field source is substituted.
#link("https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf")[Primary paper, SIAM J. Matrix Anal. Appl. 38(3),733–775.]

*Next gate.* Freeze complete entrywise boundary/companion and primal/dual
transport checks on the retained small and larger fixtures, with altered-wrap,
scaling, generator and witness controls. Then implement the structured solve
backend with explicit singular-system and base-field preconditioning behavior.
The current full recovery instrument remains unchanged.
"""
for a,b in [('from1','from 1'),('Theorem6.7','Theorem 6.7'),('than2n','than 2n')]:t=t.replace(a,b)
t=t.replace('_',chr(92)+'_')
(D/'report.typ').write_text(t);typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc))
