#set page(paper:"a4",margin:18mm)
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

*Boundary identity.* Let (Tv)\_k=v\_(k+1 mod r), and let V collect the constant
vector and both endpoint character vectors of every translated pattern. The
same geometric weights as the current compiler telescope to
B - gamma T B Tᵗ = H = V diag(c) Vᵗ, with s=3+q(q-1).
Multiplying on the right by T gives gamma T B - B T = -H T.

*Companion form.* Let J reverse coordinates and D=diag(1,gamma,...,gamma^(r-1)).
Put A=D⁻¹ J B J, G=-D⁻¹ J V diag(c), K=J Tᵗ V. Then
Z\_(r,gamma) A - A Z\_(r,1) = G Kᵗ, where Z\_(r,a) has subdiagonal ones and
top-right a. The key prime-field identity is gamma^r=gamma, so
D⁻¹(gamma Tᵗ)D has exactly that wrap entry. The characteristic polynomials
X^r-gamma and X^r-1 are coprime because gamma differs from 1.

*No lost boundary data.* If gamma T B=B T, iterate r times using T^r=I:
(gamma-1)B=0, hence B=0. Thus the displacement is injective even though its
characteristic polynomials have repeated roots in characteristic r.

*Primal and dual transport.* To solve Bx=b, solve Ay=D⁻¹ Jb and set x=Jy.
If vA=0 and vD⁻¹ Jb=1, set w=vD⁻¹ J; then wB=0 and wb=1. The separate
public feature-exclusion test remains required. Here b\_k=k is public.

*Implementation scope.* Constructor inputs are r,q,gamma,weights. It meters
character powers, boundary vectors, diagonal powers, row scalings and RHS.
It builds neither B nor a Pascal matrix, inverse or solution. O(rs) boundary
data remain. Only syntax has been checked; coefficient identities and counter
formulas still require the next frozen finite panel.

*Primary-source connection.* Bostan, Jeannerod, Mouilleron and Schost (2017)
study companion displacement and fast structured systems. Their Theorem 6.7
has sufficient sampling condition |S| greater than 2n(n+1). For n=r, no
S contained in F\_r meets it. This limits use of that stated success guarantee;
it does not prove the sampler fails. No extension-field source is substituted.
#link("https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf")[Primary paper, SIAM J. Matrix Anal. Appl. 38(3),733–775.]

*Next gate.* Freeze complete entrywise boundary/companion and primal/dual
transport checks on the retained small and larger fixtures, with altered-wrap,
scaling, generator and witness controls. Then implement the structured solve
backend with explicit singular-system and base-field preconditioning behavior.
The current full recovery instrument remains unchanged.
